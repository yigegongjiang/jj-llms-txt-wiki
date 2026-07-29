# Deploy Llama 3.3 70B on AWS Inferentia2

In this tutorial you will learn how to deploy [/meta-llama/Llama-3.3-70B-Instruct](https://huggingface.co/meta-llama/Llama-3.3-70B-Instruct) model on AWS Inferentia2 with Hugging Face Optimum on Amazon SageMaker. We are going to use the Hugging Face vLLM Neuron Container, a purpose-built Inference Container to easily deploy LLMs on AWS Inferentia2 powered by [vLLM](https://github.com/vllm-project/vllm.git) and [Optimum Neuron](https://huggingface.co/docs/optimum-neuron/index).

We will cover how to:
1. [Setup development environment](#1-setup-development-environment)
2. [Retrieve the new Hugging Face vLLM Neuron DLC](#2-retrieve-the-new-hugging-face-vllm-neuron-dlc)
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
!pip install sagemaker --upgrade --quiet
```

Then, instantiate the sagemaker role and session.

```python
import boto3
from sagemaker.core.helper.session_helper import get_execution_role

try:
    role = get_execution_role()
except ValueError:
    iam = boto3.client('iam')
    role = iam.get_role(RoleName='sagemaker_execution_role')['Role']['Arn']

print(f"sagemaker role arn: {role}")
```

## 2. Retrieve the latest Hugging Face vLLM Neuron DLC

The latest Hugging Face vLLM Neuron DLCs can be used to run inference on AWS Inferentia2. To retrieve it you can use the method `image_uris.retrieve` of the Sagemaker SDK. However, if you have the Optimum Neuron package installed, you can use the `ecr.image_uri` function to retrieve the appropriate Hugging Face vLLM Neuron DLC URI based on your desired `region` and `version`. Default values can be deduced by your AWS credentials. For more details see the [containers](https://huggingface.co/docs/optimum-neuron/containers) documentation.

```python
!pip install optimum-neuron[neuronx]
from optimum.neuron.utils import ecr

REGION = "us-east-1"
llm_image = ecr.image_uri("vllm", region=REGION)
# print image uri
print(f"llm image uri: {llm_image}")
```

## 3. Deploy Llama 3.3 70B to Inferentia2

At the time of writing, [AWS Inferentia2 does not support dynamic shapes for inference](https://awsdocs-neuron.readthedocs-hosted.com/en/v2.6.0/general/arch/neuron-features/dynamic-shapes.html#neuron-dynamic-shapes), which means that we need to specify our sequence length and batch size ahead of time.
To make it easier for customers to utilize the full power of Inferentia2, we created a [neuron model cache](https://huggingface.co/docs/optimum-neuron/guides/cache_system), which contains pre-compiled configurations for the most popular LLMs, including Llama 3.3 70B. 

This means we don't need to compile the model ourselves, but we can use the pre-compiled model from the cache. You can find compiled/cached configurations on the [Hugging Face Hub](https://huggingface.co/aws-neuron/optimum-neuron-cache/tree/main/inference-cache-config). If your desired configuration is not yet cached, you can compile it yourself using the [Optimum CLI](https://huggingface.co/docs/optimum-neuron/guides/export_model) or open a request at the [Cache repository](https://huggingface.co/aws-neuron/optimum-neuron-cache/discussions).

**Deploying Llama 3.3 70B to a SageMaker Endpoint**  

All we need when deploying the model to Amazon SageMaker, is to set the Hugging Face model id and token.

- `SM_ON_MODEL`: The Hugging Face model ID.
- `HF_TOKEN`: The Hugging Face API token to access gated models.

Note: even if you model is not gated, we recommend setting your Hugging Face token to avoid rate limitations when fetching weights or pre-compiled neuron artifacts.

Optionally, you can specify some deployment parameters to select a specific cached configuration (otherwise a default one will be selected).

- `SM_ON_TENSOR_PARALLEL_SIZE`: Number of Neuron Cores used for the compilation.
- `SM_ON_BATCH_SIZE`: The batch size that was used to compile the model.
- `SM_ON_SEQUENCE_LENGTH`: The sequence length that was used to compile the model.

**Select the right instance type**

Llama 3.3 70B is a large model and requires a lot of memory. We are going to use the `inf2.48xlarge` instance type, which has 192 vCPUs and 384 GB of accelerator memory. The `inf2.48xlarge` instance comes with 12 Inferentia2 accelerators that include 24 Neuron Cores. If you want to find the cached configurations for Llama 3.3 70B, you can find them [here](https://huggingface.co/aws-neuron/optimum-neuron-cache/blob/main/inference-cache-config/llama3-70b.json). Here we will let the framework select one of them automatically, so we won't specify any specific deployment parameter.

Before we can deploy Llama 3.3 70B to Inferentia2, we need to make sure we have the necessary permissions to access the model. You can request access to the model [here](https://huggingface.co/meta-llama/Llama-3.3-70B-Instruct) and create a User access token following this [guide](https://huggingface.co/docs/hub/en/security-tokens).

After that we can create our endpoint configuration and deploy the model to Amazon SageMaker.

```python
from sagemaker.core.resources import Model, ContainerDefinition

# Define Model and Endpoint configuration parameter
environment = {
    "SM_ON_MODEL": "meta-llama/Llama-3.3-70B-Instruct",
    "HF_TOKEN": "",
}

assert environment["HF_TOKEN"] != "", "Please replace '' with your Hugging Face Hub API token"

container = ContainerDefinition(image=llm_image, environment=environment)

# create Model with the container definition
model = Model.create(
    model_name="llama-3-3-70b-neuronx-model",
    primary_container=container,
    execution_role_arn=role,
    region=REGION
)
```

After we have created the `Model` we need to define a deployment configuration.  We will deploy the model with the `ml.inf2.48xlarge` instance type. vLLM will automatically distribute and shard the model across all Inferentia devices.

```python
from sagemaker.core.resources import EndpointConfig, ProductionVariant

# sagemaker config
instance_type = "ml.inf2.48xlarge"
health_check_timeout=3600 # additional time to load the model
volume_size=512 # size in GB of the EBS volume

endpoint_config = EndpointConfig.create(
    endpoint_config_name="llama-3-3-70b-neuronx-endpoint-config",
    production_variants=[
        ProductionVariant(
            variant_name="AllTraffic",
            model_name=model.model_name,
            initial_instance_count=1,
            instance_type=instance_type,
            container_startup_health_check_timeout_in_seconds=health_check_timeout,
            volume_size_in_gb=volume_size,
            inference_ami_version = "al2-ami-sagemaker-inference-neuron-2"
        )
    ],
)
```

We can now deploy the `Model` to an `Endpoint`.

```python
from sagemaker.core.resources import Endpoint

endpoint = Endpoint.create(
    endpoint_name="llama-3-3-70b-neuronx-endpoint",
    endpoint_config_name=endpoint_config.endpoint_config_name,
)

endpoint.wait_for_status(target_status='InService')
```

SageMaker will now create our endpoint and deploy the model to it. It takes around 30 minutes for deployment.

After our endpoint is deployed we can run inference on it. We will use the `invoke` method to run inference on our endpoint. 

The endpoint supports the Messages API, which is fully compatible with the OpenAI Chat Completion API. The Messages API allows us to interact with the model in a conversational way. We can define the role of the message and the content. The role can be either `system`,`assistant` or `user`. The `system` role is used to provide context to the model and the `user` role is used to ask questions or provide input to the model.

Parameters can be defined as separate attributes of the payload. Check out the chat completion [documentation](https://platform.openai.com/docs/api-reference/chat/create) to find supported parameters.

```python
# Prompt to generate
messages=[
    { "role": "system", "content": "You are a helpful assistant." },
    { "role": "user", "content": "What is deep learning in one sentence?" }
]
```

Okay lets test it.

```python
import json

# Generation arguments https://platform.openai.com/docs/api-reference/chat/create
result = endpoint.invoke(
    body=json.dumps({
        "messages": messages,
        "max_tokens": 50,
        "top_k": 50,
        "top_p": 0.9,
        "temperature": 0.7,
    }),
    content_type="application/json"
)
output = json.loads(result.body.read().decode('utf-8'))
message = output["choices"][0]["message"]
assert message["role"] == "assistant"
print("Generated response:", message["content"])
```

## 4. Clean up

To clean up, we can delete the model and endpoint.

```python
model.delete()
endpoint_config.delete()
endpoint.delete()
```
