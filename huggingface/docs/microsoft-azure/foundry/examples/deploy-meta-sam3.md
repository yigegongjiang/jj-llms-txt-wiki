# Deploy Meta SAM 3 on Microsoft Foundry

Written by Alvaro BartolomeLast updated 2026-07-01

At the moment gated models can only be deployed programmatically, since Microsoft Foundry hasn't incorporated those yet; meaning that when deploying from Microsoft Foundry or Azure Machine Learning, even if the `HuggingFaceTokenConnection` is set, you might stumble upon the following error:

```md
Failed to retrieve and inject workspace connection secrets. Please verify that the endpoint identity has been granted the Workspace Connection Secrets Reader role or any custom roles with actions Microsoft.MachineLearningServices/workspaces/connections/listsecrets/action & Microsoft.MachineLearningServices/workspaces/metadata/secrets/read and ensure that the secret reference schema in the environment variables is accurate.
```

This example shows how to deploy Segment Anything Model 3 (SAM 3) from the Hugging Face collection on Microsoft Foundry (formerly Azure AI Foundry) as an Azure Machine Learning Managed Online Endpoint. SAM 3 is state-of-the-art across all text and visual segmentation tasks in both images and videos whilst maintaining all the performance and functionality of it predecessor, SAM 2.

![SAM 3 on Microsoft Foundry](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-meta-sam3/sam3-microsoft-foundry.png)

[SAM 3](https://huggingface.co/facebook/sam3) is a unified foundation model for promptable segmentation in images and videos, state-of-the-art in [`mask-generation`](https://huggingface.co/tasks/mask-generation). SAM 3 can detect, segment, and track objects using text or visual prompts such as points, boxes, and masks. Compared to its predecessor SAM 2, SAM 3 introduces the ability to exhaustively segment all instances of an open-vocabulary concept specified by a short text phrase or exemplars. Unlike prior work, SAM 3 can handle a vastly larger set of open-vocabulary prompts. It achieves 75-80% of human performance on our new SA-CO benchmark which contains 270K unique concepts, over 50 times more than existing benchmarks.

![Meta SAM 3 Benchmarks](https://lookaside.fbsbx.com/elementpath/media/?media_id=1132635808951855&version=1763568008&transcode_extension=webp)

[Mask generation](https://huggingface.co/tasks/mask-generation) is the task of generating masks that identify a specific object or region of interest in a given image. Masks are often used in segmentation tasks, where they provide a precise way to isolate the object of interest for further processing or analysis.

For more information, make sure to check [Meta SAM 3 on the Hugging Face Hub](https://huggingface.co/facebook/sam3).

## Pre-requisites

To run the following example, you will need to comply with the following pre-requisites, alternatively, you can also read more about those in the [Azure Machine Learning Tutorial: Create resources you need to get started](https://learn.microsoft.com/en-us/azure/machine-learning/quickstart-create-resources?view=azureml-api-2).

- An Azure account with an active subscription.
- The Azure CLI installed and logged in.
- The Azure Machine Learning extension for the Azure CLI.
- An Azure Resource Group.
- A Hub-based project on Microsoft Foundry (classic, i.e., Azure AI Foundry Hub-based project).

For more information, please go through the steps in [Configure Azure Machine Learning and Microsoft Foundry](https://huggingface.co/docs/microsoft-azure/guides/configure-azure-ml-microsoft-foundry).

Additionally, given that [`facebook/sam3`](https://huggingface.co/facebook/sam3) is a gated model, you need to agree to the gating on the Hugging Face Hub and wait for the authors to approve it so that you can download the weights. For more information, please check our guide on how to deploy [Hugging Face gated models on Microsoft Foundry](https://huggingface.co/docs/microsoft-azure/guides/access-gated-models). 

![Meta SAM 3 gating on the Hub](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-meta-sam3/sam3-gating.png)

## Setup and installation

In this example, the [Azure Machine Learning SDK for Python](https://github.com/Azure/azure-sdk-for-python/tree/main/sdk/ml/azure-ai-ml) will be used to create the endpoint and the deployment, as well as to invoke the deployed API. Along with it, you will also need to install `azure-identity` to authenticate with your Azure credentials via Python.

```python
%pip install azure-ai-ml azure-identity --upgrade --quiet
```

More information at [Azure Machine Learning SDK for Python](https://learn.microsoft.com/en-us/python/api/overview/azure/ai-ml-readme?view=azure-python).

Then, setting the following environment variables is recommended as those will be used along the example for the Azure ML Client, so make sure to update and set those values accordingly as per your Microsoft Azure account and resources.

```python
%env LOCATION eastus
%env SUBSCRIPTION_ID <YOUR_SUBSCRIPTION_ID>
%env RESOURCE_GROUP <YOUR_RESOURCE_GROUP>
%env WORKSPACE_NAME <YOUR_WORKSPACE_NAME>
```

You also need to define both the endpoint and deployment names, as those will be used throughout the example too:

Note that endpoint names must to be globally unique per region i.e., even if you don't have any endpoint named that way running under your subscription, if the name is reserved by another Azure customer, then you won't be able to use the same name. Adding a timestamp or a custom identifier is recommended to prevent running into HTTP 400 validation issues when trying to deploy an endpoint with an already locked / reserved name. Also the endpoint name must be between 3 and 32 characters long.

```python
import os
from uuid import uuid4

os.environ["ENDPOINT_NAME"] = f"endpoint-{str(uuid4())[:8]}"
os.environ["DEPLOYMENT_NAME"] = f"deployment-{str(uuid4())[:8]}"
```

Finally, as [`facebook/sam3`](https://huggingface.co/facebook/sam3) is a gated model, you need to create an Azure Machine Learning Connection with Custom keys named `HuggingFaceTokenConnection` with a key `HF_TOKEN` with [your Hugging Face read or fine-grained token](https://huggingface.co/settings/tokens/new?canReadGatedRepos=true&tokenType=fineGrained) flagged as secret to make sure that the model weights can be downloaded from the Hugging Face Hub on runtime.

![Azure Machine Learning Connection for 'HF_TOKEN'](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-meta-sam3/azureml-connection.png)

Alternatively, you can also create the Azure Machine Learning Connection programmatically as follows:

```bash
az ml connection create \
    --name HuggingFaceTokenConnection \
    --type "Generic" \
    --resource-group $RESOURCE_GROUP \
    --workspace-name $WORKSPACE_NAME \
    --secret "HF_TOKEN=<YOUR_HF_TOKEN_HERE>"
```

## Authenticate to Azure Machine Learning

First you need to authenticate into the Microsoft Foundry via Azure Machine Learning with the Python SDK:

On standard Azure Machine Learning deployments you'd need to create the `MLClient` using the Azure Machine Learning Workspace as the `workspace_name` whereas for Microsoft Foundry, you need to provide the Azure AI Foundry Hub-based project name as the `workspace_name` instead, and that will deploy the endpoint under Microsoft Foundry too.

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

To create the Azure Machine Learning Managed Online Endpoint you don't need to provide the model ID on the Hugging Face Hub but rather the model URI on Azure Machine Learning formatted as follows:

```python
model_id = "facebook/sam3"

model_uri = f"azureml://registries/HuggingFace/models/{model_id.replace('/', '-').replace('_', '-').lower()}/labels/latest"
model_uri
```

To check if a model from the Hugging Face Hub is available in Azure, you should read about it in [Supported Models](https://huggingface.co/docs/microsoft-azure/azure-ai/models). If not, you can always [Request a model addition in the Hugging Face collection on Azure](https://huggingface.co/docs/microsoft-azure/guides/request-model-addition)).

Then you need to create the [ManagedOnlineEndpoint via the Azure ML Python SDK](https://learn.microsoft.com/en-us/python/api/azure-ai-ml/azure.ai.ml.entities.managedonlineendpoint?view=azure-python) making sure to include the `enfoce_access_to_default_secret_stores: enabled` property so that the Azure Machine Learning Connection secret can be read to later pull the weights from the Hugging Face Hub.

```python
from azure.ai.ml.entities import ManagedOnlineEndpoint

endpoint = ManagedOnlineEndpoint(
    name=os.getenv("ENDPOINT_NAME"),
    properties={"enforce_access_to_default_secret_stores": "enabled"},
)

client.begin_create_or_update(endpoint).wait()
```

After creating the endpoint, you need to create the [ManagedOnlineDeployment via the Azure ML Python SDK](https://learn.microsoft.com/en-us/python/api/azure-ai-ml/azure.ai.ml.entities.managedonlinedeployment?view=azure-python) i.e., create a deployment linked to the given endpoint.

All Hugging Face models on Azure run on an optimized inference backend that supports multiple hardware SKUs, as listed in the [Supported Hardware page](https://huggingface.co/docs/microsoft-azure/azure-ai/supported-hardware). Some models require GPU-enabled instances, so you may need to request a quota increase using the [Azure Machine Learning quota management guide](https://learn.microsoft.com/en-us/azure/machine-learning/how-to-manage-quotas?view=azureml-api-2).

```python
from azure.ai.ml.entities import ManagedOnlineDeployment

deployment = ManagedOnlineDeployment(
    name=os.getenv("DEPLOYMENT_NAME"),
    endpoint_name=os.getenv("ENDPOINT_NAME"),
    model=model_uri,
    instance_type="Standard_NC40ads_H100_v5",
    instance_count=1,
)

client.online_deployments.begin_create_or_update(deployment).wait()
```

The deployment might take ~10-15 minutes, but it could as well take longer depending on the selected SKU availability in the region. Once deployed, you will be able to inspect the endpoint details, the real-time logs, how to consume the endpoint, and [monitoring (on preview)](https://learn.microsoft.com/en-us/azure/machine-learning/concept-model-monitoring?view=azureml-api-2).

Find more information about it at [Azure Machine Learning Managed Online Endpoints](https://learn.microsoft.com/en-us/azure/machine-learning/concept-endpoints-online?view=azureml-api-2#managed-online-endpoints)

## Send requests to the Foundry Endpoint

Once the Foundry Deployment is done, you can send requests to the Foundry Endpoint following [the API specification defined in the model card on Microsoft Foundry](https://ai.azure.com/catalog/models/facebook-sam3).

### Azure Machine Learning SDK

The Foundry Endpoint can be invoked via the Azure ML Client previously instantiated, but it requires an actual JSON file, which means that you first need to dump the payload into a JSON file then invoke the API.

```python
import json
import os
import tempfile

with tempfile.NamedTemporaryFile(mode="w+", delete=True, suffix=".json") as f:
    json.dump({
        "inputs": "http://images.cocodataset.org/val2017/000000077595.jpg",
        "parameters": {"points_per_batch":16},
    }, f)
    
    f.flush()

    response = client.online_endpoints.invoke(
        endpoint_name=os.getenv("ENDPOINT_NAME"),
        deployment_name=os.getenv("DEPLOYMENT_NAME"),
        request_file=f.name,
    )
    output = json.loads(response)
```

The `output` will contain a key `results` with a list of all the generated masks, meaning each item in the list is a dict with the keys `mask`, with the generated mask, and `score`, with the generated score for the given mask; note that the results are sorted by `score` from higher to lower (in the 0.0 to 1.0 range).

```python
output["results"][0]
```

### Python `requests`

Alternatively, you can also send standard HTTP requests via e.g. cURL or Python `requests`, but there's a few things to take into consideration:

- The API URL is available via `client.online_endpoints.get(os.getenv("ENDPOINT_NAME")).scoring_uri`, or rather via Microsoft Foundry or Azure Machine Learning.
- The API Key needs to be provided as a Bearer token in the authorization header and is available via `client.online_endpoints.get_keys(os.getenv("ENDPOINT_NAME")).primary_key`, or rather via Microsoft Foundry or Azure Machine Learning.
- The header `azureml-model-deployment` with `os.getenv("DEPLOYMENT_NAME")` needs to be provided given that an endpoint can have more than one deployment, hence its name needs to be provided as a header.

```python
import requests

url = client.online_endpoints.get(os.getenv("ENDPOINT_NAME")).scoring_uri
key = client.online_endpoints.get_keys(os.getenv('ENDPOINT_NAME')).primary_key

output = requests.post(
    url,
    headers={
        "Authorization": f"Bearer {key}",
        "azureml-model-deployment": os.getenv("DEPLOYMENT_NAME"),
    },
    data={
        "inputs":"http://images.cocodataset.org/val2017/000000077595.jpg",
        "parameters": { "points_per_batch":16 },
    }
)
```

## Print generated masks

Once the masks have been generated, you can convert those into `PIL` images given that those are generated encoded in base64; then you can print those (not the masks are "boolean" images hence black and white) with the following snippet:

```python
%pip install numpy Pillow matplotlib --upgrade --quiet
```

```python
import base64
import math
from io import BytesIO

import matplotlib.pyplot as plt
import numpy as np
from PIL import Image

mask_size = (128, 128)
spacing = 24

pil_masks = []
for result in output["results"]:
    mask_b64 = result["mask"]
    mask_bytes = base64.b64decode(mask_b64)
    mask_img = Image.open(BytesIO(mask_bytes)).convert("L").resize(mask_size)
    pil_masks.append(mask_img)

n = len(pil_masks)
cols = min(4, n)
rows = math.ceil(n / cols)

fig, axes = plt.subplots(rows, cols, figsize=(cols * 2.2, rows * 2.2))
axes = np.atleast_1d(axes).flatten()

for idx, ax in enumerate(axes):
    if idx < n:
        ax.imshow(pil_masks[idx], cmap="gray")
    else:
        ax.imshow(np.zeros(mask_size), cmap="gray")
    ax.set_xticks([])
    ax.set_yticks([])
    for spine in ax.spines.values():
        spine.set_visible(True)
        spine.set_color("lightgray")
        spine.set_linewidth(spacing // 8)

plt.subplots_adjust(wspace=spacing / mask_size[0], hspace=spacing / mask_size[1])
plt.show()
```

![Grid of black and white generated masks](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-meta-sam3/cat-masks-only.png)

As a recommendation, to better see the generated masks you can print those over the generated image with a different color with the following snippet:

```python
!wget http://images.cocodataset.org/val2017/000000077595.jpg -O image.jpg
```

```python
import base64
import math
from io import BytesIO

import matplotlib.pyplot as plt
import numpy as np
from PIL import Image

image = np.array(Image.open("image.jpg").convert("RGB"))

def mask_from_base64(mask_b64, target_shape):
    mask_bytes = base64.b64decode(mask_b64)
    mask_img = (
        Image.open(BytesIO(mask_bytes))
        .convert("L")
        .resize((target_shape[1], target_shape[0]), resample=Image.NEAREST)
    )
    return np.array(mask_img) > 0

mask_list = [
    mask_from_base64(res["mask"], image.shape[:2]) for res in output["results"]
]

n = len(mask_list) + 1
cols = min(4, n)
rows = math.ceil(n / cols)

fig, axes = plt.subplots(rows, cols, figsize=(cols * 5, rows * 5))
axes = np.atleast_1d(axes).flatten()

for idx, mask in enumerate(mask_list):
    overlay = image.copy()
    color = np.random.randint(0, 256, 3, dtype=np.uint8)
    alpha = 0.5
    overlay[mask] = (alpha * color + (1 - alpha) * overlay[mask]).astype(np.uint8)
    axes[idx].imshow(overlay)
    axes[idx].set_title(f"Mask {idx}")

for ax in axes[n:]:
    ax.axis("off")
for ax in axes[:n]:
    ax.axis("off")

plt.tight_layout()
plt.show()
```

![Grid of original image with generated masks overlay](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-meta-sam3/cat-with-masks.png)

## Release resources

Once you are done using the Foundry Endpoint, you can delete the resources (i.e., you will stop paying for the instance on which the model is running and all the attached costs) as follows:

```python
client.online_endpoints.begin_delete(name=os.getenv("ENDPOINT_NAME")).result()
```

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/Microsoft-Azure/tree/main/examples/foundry/deploy-meta-sam3/azure-notebook.ipynb)!
