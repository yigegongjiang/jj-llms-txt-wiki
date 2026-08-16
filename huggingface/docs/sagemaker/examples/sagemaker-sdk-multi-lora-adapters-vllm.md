# Serve many LoRA adapters from one endpoint with Hugging Face vLLM

Written by Dario SalvatiLast updated 2026-08-05

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/sagemaker/notebooks/sagemaker-sdk/multi-lora-adapters-vllm/cover.png)

In this notebook, we'll deploy [`mistralai/Mistral-7B-v0.1`](https://huggingface.co/mistralai/Mistral-7B-v0.1) with the Hugging Face **vLLM** Deep Learning Container (DLC) on Amazon SageMaker AI, and then serve several **LoRA adapters** from that single endpoint. Each adapter specializes the same base model for a different task, and every request picks the adapter it needs.

We'll walk through the following steps:

- Understand how multi-LoRA serving works, and what SageMaker inference components add to it
- Package LoRA adapters from the Hugging Face Hub and stage them in Amazon S3
- Deploy the base model as a **base inference component** on a GPU endpoint
- Attach each adapter as an **adapter inference component** on top of that base
- Route requests to the base model or to one adapter, and compare the answers
- Add and remove adapters on a running endpoint, without a redeployment
- Clean up the endpoint resources to avoid ongoing charges

For this example, you'll need AWS credentials and a SageMaker execution role. The base model and the adapters are public, so a Hugging Face token is optional here. Authenticated requests get more generous rate limits and faster downloads, and a token is required if you switch to a gated model.

## Why serve adapters instead of full models

[LoRA](https://huggingface.co/docs/peft/main/en/conceptual_guides/lora) fine-tuning keeps the base weights frozen and trains a small pair of low-rank matrices for the layers you target. What you ship at the end is an adapter of a few megabytes next to a base model of many gigabytes. The three adapters in this notebook weigh about 12 MB each; the base model weighs about 15 GB.

That difference is what makes multi-tenant serving practical. vLLM loads the base weights once and applies adapter weights per request, so a single GPU answers for many fine-tunes at the same time. One endpoint per fine-tune means one GPU per fine-tune, and one bill per fine-tune.

## Inference components: one base, many adapters

SageMaker AI expresses the same idea with [inference components](https://docs.aws.amazon.com/sagemaker/latest/dg/inference-components.html). An inference component is a deployable unit inside an endpoint, and it holds a model, a container, and the compute that the model needs.

Two kinds of component matter here:

- A **base inference component** owns the accelerator and a share of host memory. It runs the vLLM container with the base model.
- An **adapter inference component** points at a base component through `BaseInferenceComponentName`, and at an adapter archive in S3 through `Container.ArtifactUrl`. It has no compute of its own, because it borrows the compute of its base.

When you create an adapter component, SageMaker downloads the archive and registers it with the running container through the container's standard adapter routes, `POST /adapters` and `DELETE /adapters/{name}`. At invocation time, the `InferenceComponentName` request parameter selects who answers: the base component for the plain model, or an adapter component for one of the fine-tunes. Adding or deleting an adapter does not restart the endpoint.

## Setup

To run this example, we'll install the SageMaker Python SDK for the deployment and `huggingface_hub` to download the adapters from the Hub.

```python
%pip install -q "sagemaker>=3" huggingface_hub
```

We are going to need:

- An `HF_TOKEN`: used to download the model and the adapters from Hugging Face. Optional for the public repositories used here; authenticated requests get more generous rate limits and faster downloads (and a token is required for gated repositories).
- A SageMaker execution role: used to pull the DLC from ECR, read the adapter archives from S3, and deploy the model to SageMaker.

Let's start by setting up the token and the execution role.

```python
from huggingface_hub import get_token

HF_TOKEN = get_token()

if HF_TOKEN:
    print("HF_TOKEN loaded")
else:
    print(
        "No HF_TOKEN found. The public base model and adapters still download. "
        "Run huggingface_hub.notebook_login() to authenticate for higher rate "
        "limits or to access gated repositories."
    )
```

```python
import os

import boto3
from sagemaker.core.helper.session_helper import Session, get_execution_role

REGION = boto3.Session().region_name or os.environ.get("AWS_REGION", "us-east-1")
boto_sess = boto3.Session(region_name=REGION)
sess = Session(boto_session=boto_sess)

try:
    role = get_execution_role(sagemaker_session=sess)
    print(f"Role extracted from execution role: {role}")
except Exception:
    role_name = "sagemaker_execution_role"
    iam_client = boto_sess.client("iam")
    role = iam_client.get_role(RoleName=role_name)["Role"]["Arn"]
    print(f"Role extracted from iam client: {role}")
```

## Choosing a base model, adapters, and a DLC

Multi-LoRA serving has one hard requirement: every adapter must be trained on the **same base model**. An adapter trained on another base does not load.

We'll use [`mistralai/Mistral-7B-v0.1`](https://huggingface.co/mistralai/Mistral-7B-v0.1) as the base, together with three public adapters from Predibase's [LoRA Land](https://huggingface.co/predibase) collection. All three were trained on that base, each one for a different task:

| Adapter | Task |
| --- | --- |
| [`predibase/conllpp`](https://huggingface.co/predibase/conllpp) | Named entity recognition, answered as JSON |
| [`predibase/tldr_headline_gen`](https://huggingface.co/predibase/tldr_headline_gen) | Headline generation from a news passage |
| [`predibase/magicoder`](https://huggingface.co/predibase/magicoder) | Code generation |

We serve them with the Hugging Face **vLLM** DLC, and resolve its image URI for the chosen region and instance with the SageMaker SDK's `image_uris.retrieve` helper. You can also browse the available images on the [AWS Available Images](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-dg.pdf#available-images) page.

We'll target an `ml.g6.2xlarge` instance (a single NVIDIA L4 24GB GPU), which holds the 7B base model in bfloat16 and leaves room for the adapters and the KV cache. `ml.g5.2xlarge` and `ml.g6e.2xlarge` work as well. Two limits can stop the deployment: the account quota for that instance type, which fails endpoint creation with `ResourceLimitExceeded`, and the GPU capacity available in the region, which fails it with `InsufficientInstanceCapacity`. In either case, retry with another GPU instance type or in another region.

```python
from time import strftime

from sagemaker.core import image_uris

BASE_MODEL = "mistralai/Mistral-7B-v0.1"
ADAPTERS = {
    "ner": "predibase/conllpp",
    "headline": "predibase/tldr_headline_gen",
    "code": "predibase/magicoder",
}

INSTANCE_TYPE = "ml.g6.2xlarge"
IMAGE_URI = image_uris.retrieve(
    "huggingface-vllm",
    region=REGION,
    image_scope="inference",
    instance_type=INSTANCE_TYPE,
)

RESOURCE_SUFFIX = strftime("%Y%m%d-%H%M%S")
MODEL_NAME = f"multi-lora-model-{RESOURCE_SUFFIX}"
ENDPOINT_CONFIG_NAME = f"multi-lora-config-{RESOURCE_SUFFIX}"
ENDPOINT_NAME = f"multi-lora-endpoint-{RESOURCE_SUFFIX}"
BASE_COMPONENT_NAME = f"multi-lora-base-{RESOURCE_SUFFIX}"

print(IMAGE_URI)
```

## Packaging the adapters for SageMaker

An adapter component reads its weights from a `tar.gz` archive in S3, not from the Hub. For each adapter we download the two files that vLLM needs, `adapter_config.json` and `adapter_model.safetensors`, pack them at the root of an archive, and upload it to the session bucket.

Keep those files at the root of the archive. SageMaker extracts the archive into a directory and gives that directory to the container as the adapter source, so an extra folder level hides the weights.

```python
import tarfile
from pathlib import Path

from huggingface_hub import snapshot_download

ADAPTER_FILES = ["adapter_config.json", "adapter_model.safetensors"]
S3_PREFIX = f"multi-lora-adapters/{RESOURCE_SUFFIX}"

adapter_artifacts = {}

for alias, repo_id in ADAPTERS.items():
    local_dir = Path("adapters") / alias
    snapshot_download(repo_id=repo_id, local_dir=local_dir, allow_patterns=ADAPTER_FILES)

    archive = Path("adapters") / f"{alias}.tar.gz"
    with tarfile.open(archive, "w:gz") as tar:
        for file in sorted(local_dir.iterdir()):
            if file.is_file():
                tar.add(file, arcname=file.name)

    adapter_artifacts[alias] = sess.upload_data(path=str(archive), key_prefix=S3_PREFIX)
    print(f"{alias}: {adapter_artifacts[alias]}")
```

## Configuring vLLM for LoRA

The vLLM container is configured entirely through environment variables. Any vLLM server flag can be passed by uppercasing it, replacing dashes with underscores, and prefixing it with `SM_VLLM_`, so `--enable-lora` becomes `SM_VLLM_ENABLE_LORA`.

Three of these variables turn plain text generation into adapter serving:

- `SM_VLLM_ENABLE_LORA=true` starts the engine with LoRA support.
- `SM_VLLM_MAX_LORA_RANK=16` is the largest adapter rank the engine accepts. Our adapters are rank 8; read the `r` value in the `adapter_config.json` of each adapter and keep this setting at or above the largest one. A larger value reserves more GPU memory.
- `SM_VLLM_MAX_LORAS=4` is how many different adapters one batch can mix.

One more variable has no `SM_VLLM_` equivalent, and adapters do not answer without it:

- `VLLM_ALLOW_RUNTIME_LORA_UPDATING=true` is a plain vLLM environment variable rather than a server flag, so you set it under its own name. vLLM adds the adapter routes (`POST /adapters` and `DELETE /adapters/{name}`) only when this variable is true, and SageMaker uses those routes to register an adapter with the running container. Leave it out and nothing looks broken at first: the endpoint starts, the base model answers, and each adapter component still reports `InService`. Only the requests fail, with a `ModelError` that says `{"detail":"Not Found"}`, because the container never received the adapter.

We also set `SM_VLLM_HOST=0.0.0.0`, which is required so the container passes the SageMaker health check.

```python
env_vars = {
    "SM_VLLM_MODEL": BASE_MODEL,
    "SM_VLLM_HOST": "0.0.0.0",  # Bind to all interfaces so the health check passes
    "SM_VLLM_MAX_MODEL_LEN": "4096",
    "SM_VLLM_GPU_MEMORY_UTILIZATION": "0.85",
    "SM_VLLM_ENABLE_LORA": "true",
    "SM_VLLM_MAX_LORA_RANK": "16",  # At or above the rank of every adapter
    "SM_VLLM_MAX_LORAS": "4",  # Adapters that a single batch can mix
    "VLLM_ALLOW_RUNTIME_LORA_UPDATING": "true",  # Exposes the adapter routes
}

if HF_TOKEN:
    env_vars["HF_TOKEN"] = HF_TOKEN
```

## Deploying the base model

An endpoint that hosts inference components differs from a plain one in three ways:

- the production variant carries no model name, because each component brings its own model;
- the endpoint configuration carries an execution role, which SageMaker uses to fetch the component artifacts;
- the model, the container environment, and the compute requirements live in the component.

We build the endpoint with the SageMaker Python SDK v3 resource APIs: `Model`, `EndpointConfig`, `Endpoint`, and `InferenceComponent`.

**Cost note:** the next cells create a billable GPU-backed SageMaker real-time endpoint. Delete it in the cleanup section when you are done.

```python
from sagemaker.core.resources import Endpoint, EndpointConfig, InferenceComponent, Model
from sagemaker.core.shapes import (
    ContainerDefinition,
    InferenceComponentComputeResourceRequirements,
    InferenceComponentContainerSpecification,
    InferenceComponentRuntimeConfig,
    InferenceComponentSpecification,
    InferenceComponentStartupParameters,
    ProductionVariant,
    ProductionVariantRoutingConfig,
)

model = Model.create(
    model_name=MODEL_NAME,
    primary_container=ContainerDefinition(image=IMAGE_URI, environment=env_vars),
    execution_role_arn=role,
    region=REGION,
)

endpoint_config = EndpointConfig.create(
    endpoint_config_name=ENDPOINT_CONFIG_NAME,
    execution_role_arn=role,
    production_variants=[
        ProductionVariant(
            variant_name="AllTraffic",
            instance_type=INSTANCE_TYPE,
            initial_instance_count=1,
            inference_ami_version="al2-ami-sagemaker-inference-gpu-3-1",
            routing_config=ProductionVariantRoutingConfig(
                routing_strategy="LEAST_OUTSTANDING_REQUESTS"
            ),
        )
    ],
    region=REGION,
)

endpoint = Endpoint.create(
    endpoint_name=ENDPOINT_NAME,
    endpoint_config_name=ENDPOINT_CONFIG_NAME,
    region=REGION,
)
endpoint.wait_for_status("InService")
```

The endpoint is now a host with one GPU instance and no model on it. The base component adds the model: it claims the accelerator and a share of host memory, and SageMaker starts the vLLM container for it.

`min_memory_required_in_mb` is a reservation of host memory, not of GPU memory, and it has to fit in what the instance offers after SageMaker's own reservation. It is smaller than the model: the weights live on the GPU, and 8 GiB is enough for the container on an `ml.g6.2xlarge` (32 GiB of host memory). Ask for too much and the component fails with a message about memory or hardware resources, and a failed component keeps its reservation until you delete it. Raise the value only if you plan to place several base components on one instance.

The container downloads the base weights from the Hub on first start, so this step takes a few minutes.

```python
base_component = InferenceComponent.create(
    inference_component_name=BASE_COMPONENT_NAME,
    endpoint_name=ENDPOINT_NAME,
    variant_name="AllTraffic",
    specification=InferenceComponentSpecification(
        model_name=MODEL_NAME,
        compute_resource_requirements=InferenceComponentComputeResourceRequirements(
            number_of_accelerator_devices_required=1,
            min_memory_required_in_mb=8192,
        ),
        startup_parameters=InferenceComponentStartupParameters(
            model_data_download_timeout_in_seconds=1800,
            container_startup_health_check_timeout_in_seconds=1800,
        ),
    ),
    runtime_config=InferenceComponentRuntimeConfig(copy_count=1),
    region=REGION,
)
base_component.wait_for_status("InService")
```

## Talking to the base model

SageMaker requests go through the container's `/invocations` route, and `CustomAttributes` forwards them to a vLLM API route. The adapters we picked were trained on plain prompt-completion data rather than on a chat template, so we use `route=/v1/completions`.

`InferenceComponentName` selects the component that answers. With the base component name, we get Mistral 7B as published.

```python
import json

runtime = boto_sess.client("sagemaker-runtime")

def complete(prompt: str, component: str, max_tokens: int = 120) -> str:
    """Send one completion request to a component of the endpoint."""
    response = runtime.invoke_endpoint(
        EndpointName=ENDPOINT_NAME,
        InferenceComponentName=component,
        ContentType="application/json",
        Body=json.dumps({
            "model": BASE_MODEL,
            "prompt": prompt,
            "max_tokens": max_tokens,
            "temperature": 0.0,
        }),
        CustomAttributes="route=/v1/completions",
    )
    return json.loads(response["Body"].read())["choices"][0]["text"]
```

Each adapter expects the prompt format it was trained on, which its model card documents. We keep one prompt per task, so we can send the same prompt to the base model and to the adapter and compare.

```python
PROMPTS = {
    "ner": (
        "Your task is a Named Entity Recognition (NER) task. Predict the category "
        "of each entity, then place the entity into the list associated with the "
        "category in an output JSON payload. Below is an example:\n"
        'Input: EU rejects German call to boycott British lamb . Output: {"person": [], '
        '"organization": ["EU"], "location": [], "miscellaneous": ["German", "British"]}\n'
        "Now, complete the task.\n"
        "Input: Barack Obama visited Paris last week to meet engineers from Airbus . "
        "Output: "
    ),
    "headline": (
        "The following passage is content from a news report. Please summarize this "
        "passage in one sentence or less.\n\nPassage: Hugging Face and Amazon Web "
        "Services expanded their partnership so that developers can deploy open models "
        "on SageMaker AI with prebuilt Deep Learning Containers, including a vLLM "
        "container that serves LoRA adapters on a single endpoint.\n\nSummary: "
    ),
    "code": (
        "Below is a programming problem, paired with a language in which the solution "
        "should be written. Write a solution in the provided that appropriately solves "
        "the programming problem.\n\n### Problem: \n\ndef strlen(string: str) -> int:\n"
        '    """ Return length of given string\n    >>> strlen(\'\')\n    0\n    """\n\n'
        "### Language: python\n\n### Solution: "
    ),
}

print(complete(PROMPTS["ner"], BASE_COMPONENT_NAME))
```

The base model does a fair job on the entities, because the prompt shows it one example, but it does not stop: it keeps inventing new inputs until it runs out of tokens. That is what task fine-tuning fixes.

## Attaching the adapters

Now we add two adapter components on top of the base. Each one names the base component and the S3 archive, and asks for no compute of its own. Registration takes seconds, because the archives are a few megabytes and the base container is already running.

```python
adapter_components = {}

def attach_adapter(alias: str) -> InferenceComponent:
    """Register one adapter archive as an adapter inference component."""
    component = InferenceComponent.create(
        inference_component_name=f"multi-lora-{alias}-{RESOURCE_SUFFIX}",
        endpoint_name=ENDPOINT_NAME,
        specification=InferenceComponentSpecification(
            base_inference_component_name=BASE_COMPONENT_NAME,
            container=InferenceComponentContainerSpecification(
                artifact_url=adapter_artifacts[alias]
            ),
        ),
        region=REGION,
    )
    component.wait_for_status("InService")
    adapter_components[alias] = component
    print(f"{alias}: {component.inference_component_name}")
    return component

for alias in ("ner", "headline"):
    attach_adapter(alias)
```

## Comparing the base model with its adapters

The same endpoint, the same GPU, and the same base weights now answer in three different ways. Only the component name changes.

```python
for alias in ("ner", "headline"):
    component = adapter_components[alias].inference_component_name
    print(f"=== {alias} ({ADAPTERS[alias]})")
    print("base    :", complete(PROMPTS[alias], BASE_COMPONENT_NAME).strip())
    print("adapter :", complete(PROMPTS[alias], component).strip())
    print()
```

The adapters answer in the format they were trained on, and they stop when the answer is complete: the NER adapter returns one JSON object, and the headline adapter returns one headline.

## Adding an adapter to a running endpoint

Adapters are not fixed at deployment time. The third adapter joins the endpoint that is already serving traffic, and the components that were already there keep answering.

```python
attach_adapter("code")

print(complete(PROMPTS["code"], adapter_components["code"].inference_component_name))
```

## Removing an adapter

Deleting an adapter component unregisters that adapter and frees its memory. The base model and the other adapters are untouched. Requests that name a deleted component are rejected by SageMaker before they reach the container.

```python
from botocore.exceptions import ClientError

removed = adapter_components.pop("code")
removed.delete()
removed.wait_for_delete()

try:
    complete(PROMPTS["code"], removed.inference_component_name)
except ClientError as error:
    print(error.response["Error"]["Message"])

print("headline still served:", complete(PROMPTS["headline"], adapter_components["headline"].inference_component_name).strip())
```

## Notes for production

- **Scaling.** The base component owns the compute, so you scale it, not the adapters: raise `copy_count` for more copies, or attach an application autoscaling policy to the component. Adapters follow the base component they are attached to. Inference components also support [scaling an endpoint down to zero instances](https://docs.aws.amazon.com/sagemaker/latest/dg/realtime-endpoints-scale-to-zero.html) with managed instance scaling, which suits adapter fleets with idle periods.
- **Adapter capacity.** `SM_VLLM_MAX_LORAS` bounds how many adapters a single batch mixes, and `SM_VLLM_MAX_LORA_RANK` bounds the rank the engine reserves memory for. Keep the rank close to what your adapters actually use, because a higher rank costs GPU memory for every adapter slot.
- **Prompt formats.** An adapter answers well only for the prompt shape it was trained on. Ship the prompt template together with the adapter, or wrap each adapter in a small client-side helper as we did with `PROMPTS`.
- **Observability.** SageMaker publishes per-component CloudWatch metrics, so you can see invocation counts, latency, and errors for each adapter separately, on one endpoint.
- **Deletion order.** An adapter component depends on its base, so delete every adapter before the base component; SageMaker refuses to delete a base component that still has adapters attached.

## Cleanup

To stop the endpoint and avoid ongoing charges once you're done, delete the adapter components first, then the base component, the endpoint, the endpoint configuration, and the model:

```python
for alias, component in list(adapter_components.items()):
    component.delete()
    component.wait_for_delete()
    print(f"deleted adapter {alias}")

base_component.delete()
base_component.wait_for_delete()

# delete_endpoint is asynchronous; wait for it to finish before removing the
# config and model, which stay in use while the endpoint is still deleting.
endpoint.delete()
endpoint.wait_for_delete()

endpoint_config.delete()
model.delete()
print("endpoint resources deleted")
```

## Conclusion and references

We deployed [`mistralai/Mistral-7B-v0.1`](https://huggingface.co/mistralai/Mistral-7B-v0.1) on the Hugging Face vLLM DLC as a base inference component, attached LoRA adapters as adapter inference components, and selected the one that answers each request with `InferenceComponentName`. A single GPU served the base model and every adapter, and adapters were added and removed while the endpoint kept serving. The same pattern works for your own fine-tunes: train them on one base model, upload each adapter as a `tar.gz` to S3, and attach it to the base component.

References:

- [`mistralai/Mistral-7B-v0.1`](https://huggingface.co/mistralai/Mistral-7B-v0.1) and the [Predibase adapters](https://huggingface.co/predibase) used here
- [Hugging Face on Amazon SageMaker](https://huggingface.co/docs/sagemaker/en/index) and the [AWS Available Images](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-dg.pdf#available-images) list
- [SageMaker inference components](https://docs.aws.amazon.com/sagemaker/latest/dg/inference-components.html) and [multi-adapter inference](https://docs.aws.amazon.com/sagemaker/latest/dg/inference-components-adapters.html)
- [vLLM documentation](https://docs.vllm.ai/) and [LoRA adapters](https://docs.vllm.ai/en/latest/features/lora.html)
- [LoRA in PEFT](https://huggingface.co/docs/peft/main/en/conceptual_guides/lora)
- [Amazon SageMaker Python SDK](https://sagemaker.readthedocs.io/)

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/multi-lora-adapters-vllm/sagemaker-notebook.ipynb)!
