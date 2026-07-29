# Deploy Large Language Models (LLMs) on Microsoft Foundry

Written by Alvaro BartolomeLast updated 2026-07-01

This example showcases how to deploy a Large Language Model (LLM) from the Hugging Face collection on Microsoft Foundry as an Azure Machine Learning Managed Online Endpoint. Additionally, this example also showcases how to run inference with both the Azure Machine Learning Python SDK, the OpenAI Python SDK, and even how to locally run a Gradio application for chat completion.

Note that this example will go through the Python SDK / Azure CLI programmatic deployment, if you'd rather prefer using the one-click deployment experience, please check [One-click deployments from the Hugging Face Hub on Microsoft Foundry](https://huggingface.co/docs/microsoft-azure/guides/one-click-deployment-azure-ai).

TL;DR Microsoft Foundry (formerly Azure AI Foundry) provides a unified platform for enterprise AI operations, model builders, and application development. Azure Machine Learning is a cloud service for accelerating and managing the machine learning (ML) project lifecycle.

---

This example will specifically deploy [`Qwen/Qwen2.5-32B-Instruct`](https://huggingface.co/Qwen/Qwen2.5-32B-Instruct) from the Hugging Face Hub (or see it on [AzureML](https://ml.azure.com/models/qwen-qwen2.5-32b-instruct/version/1/catalog/registry/HuggingFace) or on [Microsoft Foundry](https://ai.azure.com/explore/models/qwen-qwen2.5-32b-instruct/version/1/registry/HuggingFace)) as an Azure Machine Learning Managed Online Endpoint on Microsoft Foundry.

Qwen2.5 is one of the latest series of Qwen large language models, bringing the following improvements upon Qwen2 such as:

- Significantly more knowledge and has greatly improved capabilities in coding and mathematics, thanks to our specialized expert models in these domains.
- Significant improvements in instruction following, generating long texts (over 8K tokens), understanding structured data (e.g, tables), and generating structured outputs especially JSON. More resilient to the diversity of system prompts, enhancing role-play implementation and condition-setting for chatbots.
- Long-context Support up to 128K tokens and can generate up to 8K tokens.
- Multilingual support for over 29 languages, including Chinese, English, French, Spanish, Portuguese, German, Italian, Russian, Japanese, Korean, Vietnamese, Thai, Arabic, and more.

![Qwen2.5 32B Instruct on the Hugging Face Hub](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-large-language-models/qwen2.5-hub.png)

![Qwen2.5 32B Instruct on Azure AI Foundry](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-large-language-models/qwen2.5-azure-ai-foundry.png)

For more information, make sure to check [their model card on the Hugging Face Hub](https://huggingface.co/Qwen/Qwen2.5-32B-Instruct/blob/main/README.md).

Note that you can select any LLM available on the Hugging Face Hub with the "Deploy on Microsoft Foundry" option enabled, or directly select any of the LLMs available on either the Azure Machine Learning or Microsoft Foundry model catalog under the "HuggingFace" collection.

## Pre-requisites

To run the following example, you will need to comply with the following pre-requisites, alternatively, you can also read more about those in the [Azure Machine Learning Tutorial: Create resources you need to get started](https://learn.microsoft.com/en-us/azure/machine-learning/quickstart-create-resources?view=azureml-api-2).

- An Azure account with an active subscription.
- The Azure CLI installed and logged in.
- The Azure Machine Learning extension for the Azure CLI.
- An Azure Resource Group.
- A Hub-based project on Microsoft Foundry.

For more information, please go through the steps in the guide ["Configure Azure Machine Learning and Microsoft Foundry"](https://huggingface.co/docs/microsoft-azure/guides/configure-azure-ml-microsoft-foundry).

## Setup and installation

In this example, the [Azure Machine Learning SDK for Python](https://github.com/Azure/azure-sdk-for-python/tree/main/sdk/ml/azure-ai-ml) will be used to create the endpoint and the deployment, as well as to invoke the deployed API. Along with it, you will also need to install `azure-identity` to authenticate with your Azure credentials via Python.

```python
%pip install azure-ai-ml azure-identity --upgrade --quiet
```

More information at [Azure Machine Learning SDK for Python](https://learn.microsoft.com/en-us/python/api/overview/azure/ai-ml-readme?view=azure-python).

Then, for convenience setting the following environment variables is recommended as those will be used along the example for the Azure Machine Learning Client, so make sure to update and set those values accordingly as per your Microsoft Azure account and resources.

```python
%env LOCATION eastus
%env SUBSCRIPTION_ID <YOUR_SUBSCRIPTION_ID>
%env RESOURCE_GROUP <YOUR_RESOURCE_GROUP>
%env WORKSPACE_NAME <YOUR_WORKSPACE_NAME>
```

Finally, you also need to define both the endpoint and deployment names, as those will be used throughout the example too:

Note that endpoint names must to be globally unique per region i.e., even if you don't have any endpoint named that way running under your subscription, if the name is reserved by another Azure customer, then you won't be able to use the same name. Adding a timestamp or a custom identifier is recommended to prevent running into HTTP 400 validation issues when trying to deploy an endpoint with an already locked / reserved name. Also the endpoint name must be between 3 and 32 characters long.

```python
import os
from uuid import uuid4

os.environ["ENDPOINT_NAME"] = f"qwen-endpoint-{str(uuid4())[:8]}"
os.environ["DEPLOYMENT_NAME"] = f"qwen-deployment-{str(uuid4())[:8]}"
```

## Authenticate to Azure Machine Learning

Initially, you need to authenticate into Microsoft Foundry Hub via Azure Machine Learning with the Azure Machine Learning Python SDK, which will be later used to deploy `Qwen/Qwen2.5-32B-Instruct` as an Azure Machine Learning Managed Online Endpoint on Microsoft Foundry.

On standard Azure Machine Learning deployments you'd need to create the `MLClient` using the Azure Machine Learning Workspace as the `workspace_name` whereas for Microsoft Foundry, you need to provide Azure AI Foundry Hub-based project name as the `workspace_name` instead, and that will deploy the endpoint under Microsoft Foundry too.

```python
import os
from azure.ai.ml import MLClient
from azure.identity import DefaultAzureCredential

client = MLClient(
    credential=DefaultAzureCredential(),
    subscription_id=os.getenv("SUBSCRIPTION_ID"),
    resource_group_name=os.getenv("RESOURCE_GROUP"),
    workspace_name=os.getenv("WORKSPACE_NAME"),
)
```

## Create and Deploy Foundry Endpoint

Before creating the Managed Online Endpoint, you need to build the model URI, which is formatted as it follows `azureml://registries/HuggingFace/models/<MODEL_ID>/labels/latest` where the `MODEL_ID` won't be the Hugging Face Hub ID but rather its name on Azure, as follows:

```python
model_id = "Qwen/Qwen2.5-32B-Instruct"

model_uri = f"azureml://registries/HuggingFace/models/{model_id.replace('/', '-').replace('_', '-').lower()}/labels/latest"
model_uri
```

To check if a model from the Hugging Face Hub is available in Azure, you should read about it in [Supported Models](https://huggingface.co/docs/microsoft-azure/azure-ai/models). If not, you can always [Request a model addition in the Hugging Face collection on Azure](https://huggingface.co/docs/microsoft-azure/guides/request-model-addition)).

Then you need to create the [ManagedOnlineEndpoint via the Azure Machine Learning Python SDK](https://learn.microsoft.com/en-us/python/api/azure-ai-ml/azure.ai.ml.entities.managedonlineendpoint?view=azure-python) as follows.

Every model in the Hugging Face collection is powered by an efficient inference backend, and each of those can run on a wide variety of instance types (as listed in [Supported Hardware](https://huggingface.co/docs/microsoft-azure/azure-ai/supported-hardware)). Since for models and inference engines require a GPU-accelerated instance, you might need to request a quota increase as per [Manage and increase quotas and limits for resources with Azure Machine Learning](https://learn.microsoft.com/en-us/azure/machine-learning/how-to-manage-quotas?view=azureml-api-2).

```python
from azure.ai.ml.entities import ManagedOnlineEndpoint, ManagedOnlineDeployment

endpoint = ManagedOnlineEndpoint(name=os.getenv("ENDPOINT_NAME"))

deployment = ManagedOnlineDeployment(
    name=os.getenv("DEPLOYMENT_NAME"),
    endpoint_name=os.getenv("ENDPOINT_NAME"),
    model=model_uri,
    instance_type="Standard_NC40ads_H100_v5",
    instance_count=1,
)
```

```python
client.begin_create_or_update(endpoint).wait()
```

![Azure AI Endpoint from Azure AI Foundry](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-large-language-models/azure-ai-endpoint.png)

On Microsoft Foundry the endpoint will only be listed within the "My assets -> Models + endpoints" tab once the deployment is created, not before as in Azure Machine Learning where the endpoint is shown even if it doesn't contain any active or in-progress deployments.

```python
client.online_deployments.begin_create_or_update(deployment).wait()
```

![Azure AI Deployment from Azure AI Foundry](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-large-language-models/azure-ai-deployment.png)

The deployment might take ~10-15 minutes, but it could as well take longer depending on the selected SKU availability in the region. Once deployed, you will be able to inspect the endpoint details, the real-time logs, how to consume the endpoint, and [monitoring (on preview)](https://learn.microsoft.com/en-us/azure/machine-learning/concept-model-monitoring?view=azureml-api-2).

Find more information about it at [Azure Machine Learning Managed Online Endpoints](https://learn.microsoft.com/en-us/azure/machine-learning/concept-endpoints-online?view=azureml-api-2#managed-online-endpoints).

## Send requests to the Foundry Endpoint

Finally, now that the Foundry Endpoint is deployed, you can send requests to it. In this case, since the task of the model is `text-generation` (also known as `chat-completion`) you can either use the default scoring endpoint, being `/generate` which is the standard text generation endpoint without chat capabilities (as leveraging the chat template or having an OpenAI-compatible OpenAPI interface), or alternatively just benefit from the fact that the inference engine in which the model is running on top exposes OpenAI-compatible routes as `/v1/chat/completions`.

Note that below only some of the options are listed, but you can send requests to the deployed endpoint as long as you send the HTTP requests with the `azureml-model-deployment` header set to the name of the Foundry Deployment (not the Endpoint), and have the necessary authentication token / key to send requests to the given endpoint; then you can send HTTP request to all the routes that the backend engine is exposing, not only to the scoring route.

### Azure Python SDK

You can invoke the Foundry Endpoint on the scoring route, in this case `/generate` (more information about it in the `Qwen/Qwen2.5-32B-Instruct` page on either [AzureML](https://ml.azure.com/models/qwen-qwen2.5-32b-instruct/version/1/catalog/registry/HuggingFace) or [Microsoft Foundry](https://ai.azure.com/explore/models/qwen-qwen2.5-32b-instruct/version/1/registry/HuggingFace) catalogs), via the Azure Python SDK with the previously instantiated `azure.ai.ml.MLClient` (or instantiate a new one if working from a different session).

```python
import json
import os
import tempfile

with tempfile.NamedTemporaryFile(mode="w+", delete=True, suffix=".json") as tmp:
    json.dump({"inputs": "What is Deep Learning?", "parameters": {"max_new_tokens": 128}}, tmp)
    tmp.flush()

    response = client.online_endpoints.invoke(
        endpoint_name=os.getenv("ENDPOINT_NAME"),
        deployment_name=os.getenv("DEPLOYMENT_NAME"),
        request_file=tmp.name,
    )

print(json.loads(response))
```

Note that the Azure Machine Learning Python SDK requires a path to a JSON file when invoking the endpoints, meaning that whatever payload you want to send to the endpoint will need to be first converted into a JSON file, whilst that only applies to the requests sent via the Azure Machine Learning Python SDK.

### OpenAI Python SDK

Since the inference engine in which the model is running on top exposes OpenAI-compatible routes, you can also leverage the OpenAI Python SDK to send requests to the deployed Foundry Endpoint.

```python
%pip install openai --upgrade --quiet
```

To use the OpenAI Python SDK with Azure Machine Learning Managed Online Endpoints, you need to first retrieve:

- `api_url` with the `/v1` route (that contains the `v1/chat/completions` endpoint that the OpenAI Python SDK will send requests to)
- `api_key` which is the API Key on Microsoft Foundry or the primary key in Azure Machine Learning (unless a dedicated Azure Machine Learning Token is used instead)

```python
from urllib.parse import urlsplit

api_key = client.online_endpoints.get_keys(os.getenv("ENDPOINT_NAME")).primary_key

url_parts = urlsplit(client.online_endpoints.get(os.getenv("ENDPOINT_NAME")).scoring_uri)
api_url = f"{url_parts.scheme}://{url_parts.netloc}"
```

Alternatively, you can also build the API URL manually as it follows, since the URIs are globally unique per region, meaning that there will only be one endpoint named the same way within the same region:
```python
api_url = f"https://{os.getenv('ENDPOINT_NAME')}.{os.getenv('LOCATION')}.inference.ml.azure.com/v1"
```
Or just retrieve it from either Microsoft Foundry or the Azure Machine Learning Studio.

Then you can use the OpenAI Python SDK normally, making sure to include the extra header `azureml-model-deployment` header that contains the Microsoft Foundry or Azure Machine Learning Deployment.

Via the OpenAI Python SDK it can either be set within each call to `chat.completions.create` via the `extra_headers` parameter as commented below, or via the `default_headers` parameter when instantiating the `OpenAI` client (which is the recommended approach since the header needs to be present on each request, so setting it just once is preferred).

```python
import os
from openai import OpenAI

openai_client = OpenAI(
    base_url=f"{api_url}/v1",
    api_key=api_key,
    default_headers={"azureml-model-deployment": os.getenv("DEPLOYMENT_NAME")},
)

completion = openai_client.chat.completions.create(
    model="Qwen/Qwen2.5-32B-Instruct",
    messages=[
        {"role": "system", "content": "You are an assistant that responds like a pirate."},
        {
            "role": "user",
            "content": "What is Deep Learning?",
        },
    ],
    max_tokens=128,
    # extra_headers={"azureml-model-deployment": os.getenv("DEPLOYMENT_NAME")},
)
print(completion)
```

### cURL

Alternatively, you can also just use `cURL` to send requests to the deployed endpoint, with the `api_url` and `api_key` values programmatically retrieved in the OpenAI snippet and now set as environment variables so that `cURL` can use those, as it follows:

```python
os.environ["API_URL"] = api_url
os.environ["API_KEY"] = api_key
```

```python
!curl -sS $API_URL/v1/chat/completions \
    -H "Authorization: Bearer $API_KEY" \
    -H "Content-Type: application/json" \
    -H "azureml-model-deployment: $DEPLOYMENT_NAME" \
    -d '{ \
"messages":[ \
    {"role":"system","content":"You are an assistant that replies like a pirate."}, \
    {"role":"user","content":"What is Deep Learning?"} \
], \
"max_tokens":128 \
}' | jq
```

Alternatively, you can also just go to the Foundry Endpoint on either Microsoft Foundry under "My assets -> Models + endpoints" or in the Azure Machine Learning Studio via "Endpoints", and retrieve both the URL (note that it will default to the `/generate` endpoint, but to use the OpenAI-compatible layer you need to use the `/v1/chat/completions` endpoint instead) and the API Key values, as well as the Microsoft Foundry or Azure Machine Learning name for the given model.

### Gradio

[Gradio](https://www.gradio.app/) is the fastest way to demo your machine learning model with a friendly web interface so that anyone can use it. You can also leverage the OpenAI Python SDK to build a simple `ChatInterface` that you can use within the Jupyter Notebook cell where you are running it.

Ideally you could deploy the Gradio Chat Interface connected to your Azure Machine Learning Managed Online Endpoint as an Azure Container App as described in [Tutorial: Build and deploy from source code to Azure Container Apps](https://learn.microsoft.com/en-us/azure/container-apps/tutorial-deploy-from-code?tabs=python). If you'd like us to show you how to do it for Gradio in particular, feel free to [open an issue requesting it](https://github.com/huggingface/Microsoft-Azure/issues/new).

```python
%pip install gradio --upgrade --quiet
```

See below an example on how to leverage Gradio's `ChatInterface`, or find more information about it at [Gradio ChatInterface Docs](https://www.gradio.app/docs/gradio/chatinterface).

```python
import os
from typing import Dict, Iterator, List, Literal

import gradio as gr
from openai import OpenAI

openai_client = OpenAI(
    base_url=api_url,
    api_key=api_key,
    default_headers={"azureml-model-deployment": os.getenv("DEPLOYMENT_NAME")},
)

def predict(message: str, history: List[Dict[Literal["role", "content"], str]]) -> Iterator[str]:
    history.append({"role": "user", "content": message})
    
    stream = openai_client.chat.completions.create(
        model="Qwen/Qwen2.5-32B-Instruct",
        messages=history,
        stream=True,
    )
    chunks = []
    for chunk in stream:
        chunks.append(chunk.choices[0].delta.content or "")
        yield "".join(chunks)

demo = gr.ChatInterface(predict, type="messages")
demo.launch()
```

![Gradio Chat Interface with Azure AI Endpoint](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-large-language-models/azure-ml-gradio.png)

## Release resources

Once you are done using the Foundry Endpoint, you can delete the resources (i.e., you will stop paying for the instance on which the model is running and all the attached costs) as follows:

```python
client.online_endpoints.begin_delete(name=os.getenv("ENDPOINT_NAME")).result()
```

## Conclusion

Throughout this example you learnt how to create and configure your Azure account for Azure Machine Learning and Microsoft Foundry, how to then create a Managed Online Endpoint running an open model from the Hugging Face collection in the Azure Machine Learning and Microsoft Foundry model catalog, how to send inference requests to it afterwards with different alternatives, how to build a simple Gradio chat interface around it, and finally, how to stop and release the resources.

If you have any doubt, issue or question about this example, feel free to [open an issue](https://github.com/huggingface/Microsoft-Azure/issues/new) and we'll do our best to help!

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/Microsoft-Azure/tree/main/examples/foundry/deploy-large-language-models/azure-notebook.ipynb)!
