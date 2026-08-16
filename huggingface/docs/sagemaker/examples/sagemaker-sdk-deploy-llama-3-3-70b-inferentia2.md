# Deploy Llama 3.3 70B on AWS Inferentia2

Last updated 2026-08-05

In this tutorial you will learn how to deploy [/meta-llama/Llama-3.3-70B-Instruct](https://huggingface.co/meta-llama/Llama-3.3-70B-Instruct) model on AWS Inferentia2 with Hugging Face Optimum on Amazon SageMaker. We are going to use the Hugging Face TGI Neuron Container, a purpose-built Inference Container to easily deploy LLMs on AWS Inferentia2 powered by[ Text Generation Inference](https://huggingface.co/docs/text-generation-inference/index) and [Optimum Neuron](https://huggingface.co/docs/optimum-neuron/index).

We will cover how to:
1. [Setup development environment](#1-setup-development-environment)
2. [Retrieve the new Hugging Face TGI Neuron DLC](#2-retrieve-the-new-hugging-face-tgi-neuron-dlc)
3. [Deploy Llama 3.3 70B to inferentia2](#3-deploy-llama-33-70b-to-inferentia2)
4. [Clean up](#4-clean-up)

Lets get started! 🚀

[AWS inferentia (Inf2)](https://aws.amazon.com/ec2/instance-types/inf2/) are purpose-built EC2 for deep learning (DL) inference workloads. Here are the different instances of the Inferentia2 family.

| instance size | accelerators | Neuron Cores | accelerator memory | vCPU | CPU Memory | on-demand price ($/h) |
| ------------- | ------------ | ------------ | ------------------ | ---- | ---------- | --------------------- |
| inf2.xlarge   | 1            | 2            | 32                 | 4    | 16         | 0.76                  |
| inf2.8xlarge  | 1            | 2            | 32                 | 32   | 128        | 1.97                  |
| inf2.24xlarge | 6            | 12           | 192                | 96   | 384        | 6.49                  |
| inf2.48xlarge | 12           | 24           | 384                | 192  | 768        | 12.98                 |

## 1. Setup development environment

For this tutorial, we are going to use a Notebook Instance in Amazon SageMaker with the Python 3 (ipykernel) and the `sagemaker` python SDK to deploy Llama 3.3 70B to a SageMaker inference endpoint.

Make sur you have the latest version of the SageMaker SDK installed.

```python
!pip install 'sagemaker>=3.0.0' --upgrade --quiet
```

This example uses the [SageMaker Python SDK v3](https://github.com/aws/sagemaker-python-sdk). v3 introduces a new, framework-agnostic API built around `ModelBuilder` (inference) and `ModelTrainer` (training), which replaces the v2 `HuggingFaceModel` and `HuggingFace` classes.

Then, instantiate the sagemaker role and session.

```python
import boto3
from sagemaker.core.helper.session_helper import Session, get_execution_role

sess = Session()
# sagemaker session bucket -> used for uploading data, models and logs
# sagemaker will automatically create this bucket if it does not exist
sagemaker_session_bucket = sess.default_bucket()

try:
    role = get_execution_role()
except Exception:
    iam = boto3.client('iam')
    role = iam.get_role(RoleName='sagemaker_execution_role')['Role']['Arn']

print(f"sagemaker role arn: {role}")
print(f"sagemaker session region: {sess.boto_region_name}")
```

## 2. Retrieve the latest Hugging Face Inference NeuronX DLC

The Hugging Face PyTorch Inference NeuronX DLC is used to run LLM inference on AWS Inferentia2. In the SageMaker Python SDK v3 you can retrieve its URI with `image_uris.retrieve` from `sagemaker.core`, passing `inference_tool="neuronx"` together with the `transformers` version, the bundled PyTorch version (`base_framework_version`) and `py_version`. You can find the available images [here](https://huggingface.co/docs/optimum-neuron/containers).

```python
from sagemaker.core import image_uris

region = sess.boto_region_name
instance_type = "ml.inf2.48xlarge"

# Retrieve the Hugging Face PyTorch Inference NeuronX DLC for inference on Inferentia2.
# `inference_tool="neuronx"` selects the Neuron inference container; `version` is the
# transformers version and `base_framework_version` the bundled PyTorch version.
llm_image = image_uris.retrieve(
    framework="huggingface",
    inference_tool="neuronx",
    region=region,
    version="4.55.4",
    base_framework_version="pytorch2.8.0",
    py_version="py310",
    image_scope="inference",
    instance_type=instance_type,
)

print(f"llm image uri: {llm_image}")
```

## 3. Deploy Llama 3.3 70B to Inferentia2

At the time of writing, [AWS Inferentia2 does not support dynamic shapes for inference](https://awsdocs-neuron.readthedocs-hosted.com/en/v2.6.0/general/arch/neuron-features/dynamic-shapes.html#neuron-dynamic-shapes), which means that we need to specify our sequence length and batch size ahead of time.
To make it easier for customers to utilize the full power of Inferentia2, we created a [neuron model cache](https://huggingface.co/docs/optimum-neuron/guides/cache_system), which contains pre-compiled configurations for the most popular LLMs, including Llama 3.3 70B. 

This means we don't need to compile the model ourselves, but we can use the pre-compiled model from the cache. You can find compiled/cached configurations on the [Hugging Face Hub](https://huggingface.co/aws-neuron/optimum-neuron-cache/tree/main/inference-cache-config). If your desired configuration is not yet cached, you can compile it yourself using the [Optimum CLI](https://huggingface.co/docs/optimum-neuron/guides/export_model) or open a request at the [Cache repository](https://huggingface.co/aws-neuron/optimum-neuron-cache/discussions).

**Deploying Llama 3.3 70B to a SageMaker Endpoint**  

Before deploying the model to Amazon SageMaker, we must define the TGI Neuron endpoint configuration. We need to make sure the following additional parameters are defined: 

- `HF_NUM_CORES`: Number of Neuron Cores used for the compilation.
- `HF_BATCH_SIZE`: The batch size that was used to compile the model.
- `HF_SEQUENCE_LENGTH`: The sequence length that was used to compile the model.
- `HF_AUTO_CAST_TYPE`: The auto cast type that was used to compile the model.

We still need to define traditional TGI parameters with:

- `HF_MODEL_ID`: The Hugging Face model ID.
- `HF_TOKEN`: The Hugging Face API token to access gated models.
- `MAX_BATCH_SIZE`: The maximum batch size that the model can handle, equal to the batch size used for compilation.
- `MAX_INPUT_TOKEN`: The maximum input length that the model can handle. 
- `MAX_TOTAL_TOKENS`: The maximum total tokens the model can generate, equal to the sequence length used for compilation.

Optionnaly, you can configure the endpoint to support chat templates:
- `MESSAGES_API_ENABLED`: Enable Messages API 

**Select the right instance type**

Llama 3.3 70B is a large model and requires a lot of memory. We are going to use the `inf2.48xlarge` instance type, which has 192 vCPUs and 384 GB of accelerator memory. The `inf2.48xlarge` instance comes with 12 Inferentia2 accelerators that include 24 Neuron Cores. If you want to find the cached configurations for Llama 3.3 70B, you can find them [here](https://huggingface.co/aws-neuron/optimum-neuron-cache/blob/main/inference-cache-config/llama3-70b.json#L16). In our case we will use a batch size of 4 and a sequence length of 4096. 

Before we can deploy Llama 3.3 70B to Inferentia2, we need to make sure we have the necessary permissions to access the model. You can request access to the model [here](https://huggingface.co/meta-llama/Llama-3.3-70B-Instruct) and create a User access token following this [guide](https://huggingface.co/docs/hub/en/security-tokens).

After that we can create our endpoint configuration and deploy the model to Amazon SageMaker. We will deploy the endpoint with the Messages API enabled, so that it is fully compatible with the OpenAI Chat Completion API.

```python
import uuid
from sagemaker.core.resources import Model
from sagemaker.core.shapes import ContainerDefinition

model_id = "meta-llama/Llama-3.3-70B-Instruct"  # model_id from hf.co/models

health_check_timeout = 3600  # additional time to load the model
volume_size = 512            # size in GB of the EBS volume

# TGI Neuron endpoint configuration (passed to the container as env vars)
config = {
    "HF_MODEL_ID": model_id,
    "HF_NUM_CORES": "24",            # number of neuron cores used for compilation
    "HF_AUTO_CAST_TYPE": "bf16",     # dtype used to compile the model
    "MAX_BATCH_SIZE": "4",           # max batch size (== batch size used for compilation)
    "MAX_INPUT_TOKENS": "4000",      # max length of input text
    "MAX_TOTAL_TOKENS": "4096",      # max generated length (== sequence length used for compilation)
    "MESSAGES_API_ENABLED": "true",  # enable the OpenAI-compatible Messages API
    "HF_TOKEN": "<REPLACE WITH YOUR TOKEN>",  # needed to access gated models like Llama
}

assert config["HF_TOKEN"] != "<REPLACE WITH YOUR TOKEN>", "Please replace '<REPLACE WITH YOUR TOKEN>' with your Hugging Face Hub API token"

# Create the SageMaker Model directly from the container image + env vars (no model data):
# the TGI Neuron container pulls HF_MODEL_ID (and its pre-compiled Neuron cache) from the Hub
# at runtime. We use the sagemaker.core resource API rather than ModelBuilder here because
# ModelBuilder's TGI builder is GPU-only and does not support Inferentia/Neuron instances.
model_name = f"llama-3-3-70b-{uuid.uuid4().hex[:8]}"
llm_model = Model.create(
    model_name=model_name,
    primary_container=ContainerDefinition(
        image=llm_image,
        environment=config,
    ),
    execution_role_arn=role,
    region=region,
)

print(f"Created model: {model_name}")
```

Now we create a SageMaker endpoint configuration and endpoint from the model. We deploy on a single `ml.inf2.48xlarge` instance; TGI automatically shards the model across all Inferentia devices.

```python
from sagemaker.core.resources import EndpointConfig, Endpoint
from sagemaker.core.shapes import ProductionVariant

endpoint_name = f"llama-3-3-70b-{uuid.uuid4().hex[:8]}"

endpoint_config = EndpointConfig.create(
    endpoint_config_name=endpoint_name,
    production_variants=[
        ProductionVariant(
            variant_name="AllTraffic",
            model_name=model_name,
            initial_instance_count=1,
            instance_type=instance_type,
            container_startup_health_check_timeout_in_seconds=health_check_timeout,
            volume_size_in_gb=volume_size,
            initial_variant_weight=1,
        )
    ],
    region=region,
)

llm = Endpoint.create(
    endpoint_name=endpoint_name,
    endpoint_config_name=endpoint_name,
    region=region,
)

# Wait until the endpoint is in service (deployment takes around 30 minutes).
llm.wait_for_status(target_status="InService")
print(f"Endpoint in service: {endpoint_name}")
```

SageMaker will now create our endpoint and deploy the model to it. It takes around 30 minutes for deployment.

After our endpoint is deployed we can run inference on it. We will use the `invoke` method of the `Endpoint` to run inference. The request and response bodies are JSON.

The endpoint supports the Messages API, which is fully compatible with the OpenAI Chat Completion API. The Messages API allows us to interact with the model in a conversational way. We can define the role of the message and the content. The role can be either `system`,`assistant` or `user`. The `system` role is used to provide context to the model and the `user` role is used to ask questions or provide input to the model.

Parameters can be defined as in the `parameters` attribute of the payload. Check out the chat completion [documentation](https://platform.openai.com/docs/api-reference/chat/create) to find supported parameters.

```json
{
  "messages": [
    { "role": "system", "content": "You are a helpful assistant." },
    { "role": "user", "content": "What is deep learning?" }
  ]
}
```

```python
# Prompt to generate
messages=[
    { "role": "system", "content": "You are a helpful assistant." },
    { "role": "user", "content": "What is deep learning in one sentence?" }
]

# Generation arguments https://platform.openai.com/docs/api-reference/chat/create
parameters = {
    "max_tokens":100,
}
```

Okay lets test it.

```python
import json

# Build the request payload (Messages API)
payload = {"messages": messages, **parameters}

res = llm.invoke(body=json.dumps(payload), content_type="application/json")
chat = json.loads(res.body.read())

print(chat["choices"][0]["message"]["content"].strip())
```

## 4. Clean up

To clean up, we can delete the model and endpoint.

```python
llm.delete()
endpoint_config.delete()
llm_model.delete()
```

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/deploy-llama-3-3-70b-inferentia2/sagemaker-notebook.ipynb)!
