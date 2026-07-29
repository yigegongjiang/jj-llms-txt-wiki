# Build your own tool-calling agent with TRL on Azure Machine Learning

Written by Alvaro BartolomeLast updated 2026-07-01

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/machine-learning/tool-calling-agent-fine-tuning/thumbnail.png)

An agent is a model that can interact with its environment by choosing which tools (functions) to call and what arguments to pass, effectively turning a language model into a decision-maker that takes actions rather than just generating text. Teaching a model to make those tool-calling decisions is one of the most practical forms of fine-tuning today, because it lets you build agents that are tailored to your own APIs, internal tooling, or domain-specific workflows.

Recent work has shown that small, fine-tuned models can match or outperform much larger general-purpose models on targeted tasks. A 1-2B parameter model trained on high-quality, domain-specific tool-calling data will often be more reliable, faster, and cheaper to serve than a large proprietary model that has never seen your particular function signatures. Fine-tuning gives you control over exactly which tools the model knows about and how it invokes them, something that prompt engineering alone cannot always guarantee.

In this example you will fine-tune [Qwen/Qwen3.5-2B](https://huggingface.co/Qwen/Qwen3.5-2B) to become a tool-calling agent, training it on Azure Machine Learning with [TRL](https://huggingface.co/docs/trl/en/index) and a single NVIDIA H100 GPU. By the end you will have a model that can decide which function to call, and with what arguments, given a natural-language request.

What this notebook covers:

- **Setup**: provision the Azure resources (container registry, ML environment, compute, and key vault) needed to run the job.
- **Dataset**: download, filter, and format the [`NousResearch/hermes-function-calling-v1`](https://huggingface.co/datasets/NousResearch/hermes-function-calling-v1) dataset for supervised fine-tuning.
- **Training**: launch an SFT job on Azure Machine Learning with `accelerate` and TRL's `SFTTrainer`.
- **Inference**: load the fine-tuned agent locally and test it on a tool-calling prompt.

## Requirements

- You have a Microsoft Azure subscription and are logged in
- You have the `az` CLI installed
- You have the necessary permissions to:
    - Create an Azure Container Registry for Docker
        - If already created, only read+write permission is required
    - Create an Azure Machine Learning Environment (if not created already)
    - Create an Azure Machine Learning Compute Instance (if not created already)
    - Run Azure Machine Learning Jobs
    - Create an Azure Key Vault
        - If already created, only read+write permission is required
- You have Python 3.10+ installed locally and `pip`
- You have a Hugging Face Hub account

## Setup

The steps in this section only need to be run **once**. If you have already provisioned the Azure resources (container registry, ML workspace, compute, key vault), you can skip ahead to [Load & prepare the dataset](#Load-&-prepare-the-dataset).

### Set environment variables

For convenience, you can set the following environment variables to be used through the example:

```python
%env LOCATION eastus
%env SUBSCRIPTION_ID <YOUR_SUBSCRIPTION_ID>
%env RESOURCE_GROUP <YOUR_RESOURCE_GROUP> # e.g., huggingface-resource-group
%env WORKSPACE_NAME <YOUR_WORKSPACE_NAME>  # e.g., huggingface-workspace
%env REGISTRY_NAME <YOUR_REGISTRY_NAME>  # e.g., huggingface-registry
%env KEY_VAULT_NAME <YOUR_KEYVAULT_NAME>  # e.g., huggingface-key-vault
%env IMAGE_NAME hf-training
```

### Install Azure Python SDK + dependencies

You need to install some Azure Python SDK dependencies:

* `azure-identity` to use the `DefaultAzureCredential` authentication with your Managed Identity
* `azureml-core` and `azure-ai-ml` to create the Azure Machine Learning resources and run the job
* `azure-keyvault-secrets` to access Azure Key Vault secrets from Python

```python
%pip install azure-identity azureml-core azure-ai-ml azure-keyvault-secrets --upgrade --quiet
```

### Authenticate to Azure Machine Learning

Then you can already authenticate to Azure Machine Learning with your Managed Identity with Python as:

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

## Build and push Hugging Face container

Before submitting a training job you need a Docker image with all the Hugging Face dependencies (TRL, Transformers, Accelerate, etc.). The cells below create an Azure Container Registry (if you don't have one yet), build the image there, and register it as an Azure Machine Learning Environment.

If you already have a container registry and the image built, skip straight to the `Environment` creation cell.

Create an Azure Container Registry (skip if you already have one):

```python
!az acr create --resource-group $RESOURCE_GROUP --name $REGISTRY_NAME --sku Standard
```

Define the `Dockerfile` that installs all the required Hugging Face dependencies to run TRL on CUDA:

```python
%%writefile Dockerfile
FROM nvidia/cuda:12.8.1-devel-ubuntu24.04
LABEL maintainer="Hugging Face"

SHELL ["/bin/bash", "-c"]

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    build-essential \
    git \
    git-lfs \
    ffmpeg=7:* \
    libmagic-dev \
    && apt-get autoremove -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN if ! ldconfig -p | grep -q libnpp; then \
    apt-get update && apt-get install -y --no-install-recommends libnpp-dev; \
    fi && \
    if ! ldconfig -p | grep -q libnvrtc; then \
    apt-get update && apt-get install -y --no-install-recommends cuda-nvrtc-dev; \
    fi && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir -p /home/huggingface && \
    useradd -d /home/huggingface -s /bin/bash huggingface && \
    chown -R huggingface:huggingface /home/huggingface

RUN mkdir -p /home/huggingface/.triton/autotune && \
    chown -R huggingface:huggingface /home/huggingface/.triton

ENV HOME=/home/huggingface
USER huggingface
WORKDIR /home/huggingface

RUN curl -LsSf https://astral.sh/uv/install.sh | sh
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/huggingface/.cargo/bin:/home/huggingface/.local/bin/:$PATH"

RUN uv python install 3.11

RUN uv venv /home/huggingface/venv --python 3.11
ENV VIRTUAL_ENV=/home/huggingface/venv \
    PATH="/home/huggingface/venv/bin:$PATH"

ARG CUDA="cu128"
ARG PYTORCH="2.8.0"
ARG FLASH_ATTN="2.8.3"
ARG TRANSFORMERS="5.2.0"
ARG HUGGINGFACE_HUB="1.3.5"
# NOTE: Diffusers doesn't yet natively support Transformers v5.0.0
# https://github.com/huggingface/diffusers/pull/12976
ARG DIFFUSERS="0.36.0"
ARG TRL="0.27.1"
ARG PEFT="0.18.1"
ARG BITSANDBYTES="0.49.1"
ARG DATASETS="4.5.0"
ARG ACCELERATE="1.12.0"
ARG EVALUATE="0.4.6"
ARG SENTENCE_TRANSFORMERS="5.2.2"
ARG DEEPSPEED="0.18.5"
ARG KERNELS="0.12.1"

RUN uv pip install "torch==${PYTORCH}" torchvision torchaudio --torch-backend=${CUDA}

RUN uv pip install packaging ninja && \
    uv pip install "flash-attn==${FLASH_ATTN}" --no-build-isolation

RUN uv pip install --no-cache-dir \
    "transformers[sklearn,sentencepiece,vision]==${TRANSFORMERS}" \
    "huggingface_hub[cli,hf-xet]==${HUGGINGFACE_HUB}" \
    "kernels==${KERNELS}" \
    "diffusers==${DIFFUSERS}" \
    "datasets==${DATASETS}" \
    "accelerate==${ACCELERATE}" \
    "evaluate==${EVALUATE}" \
    "peft==${PEFT}" \
    "trl[liger,peft,vlm]==${TRL}" \
    "sentence-transformers==${SENTENCE_TRANSFORMERS}" \
    "deepspeed==${DEEPSPEED}" \
    "bitsandbytes==${BITSANDBYTES}"
```

Build the image on your Azure Container Registry:

Building the container image might take ~10-15 minutes.

```python
!az acr build -r $REGISTRY_NAME -g $RESOURCE_GROUP -t $IMAGE_NAME:latest -f Dockerfile .
```

Register the image as an Azure Machine Learning Environment (skip if already created):

```python
from azure.ai.ml.entities import Environment

environment = Environment(
    image=f"{os.getenv('REGISTRY_NAME')}.azurecr.io/{os.getenv('IMAGE_NAME')}:latest",
    name="hf-training",
    description="Environment w/ Hugging Face TRL for Training",
)
client.environments.create_or_update(environment)
```

## Provision Azure Machine Learning resources

Create the compute cluster and key vault needed by the training job. Skip any resource you have already provisioned.

Create an Azure Machine Learning Compute cluster with a single NVIDIA H100 (80 GB VRAM), which is more than enough for fine-tuning a 2B-parameter model:

```python
from azure.ai.ml.entities import AmlCompute

compute = AmlCompute(
    name="gpu-h100",
    size="Standard_NC40ads_H100_v5",
    tier="Dedicated",
    max_instances=1,
)
client.compute.begin_create_or_update(compute).wait()
```

Create an [Azure Key Vault](https://learn.microsoft.com/en-us/azure/key-vault/general/overview) to store secrets (e.g. your Hugging Face token) and assign yourself the *Key Vault Secrets Officer* role:

```python
!az keyvault create --name $KEY_VAULT_NAME --resource-group $RESOURCE_GROUP --location $LOCATION
```

```python
!az role assignment create \
  --role "Key Vault Secrets Officer" \
  --assignee "$(az ad signed-in-user show --query userPrincipalName -o tsv)" \
  --scope "$(az keyvault show --name $KEY_VAULT_NAME --resource-group $RESOURCE_GROUP --query id -o tsv)"
```

Store your [Hugging Face User Access Token](https://huggingface.co/settings/tokens) (with **read and write** permissions) as a Key Vault secret:

Azure Key Vault secret names do not allow underscores, so the secret is stored as `HF-TOKEN`. When referenced as an environment variable, Azure Machine Learning will map it to `HF_TOKEN` automatically.

```python
%pip install huggingface_hub --upgrade --quiet
```

```python
from huggingface_hub import get_token

os.environ["HF_TOKEN"] = get_token()
```

```python
!az keyvault secret set --vault-name $KEY_VAULT_NAME --name HF-TOKEN --value $HF_TOKEN
```

## Load & prepare the dataset

Now you are all set! So you can already start downloading and having a look at the data in [`NousResearch/hermes-function-calling-v1`](https://huggingface.co/datasets/NousResearch/hermes-function-calling-v1) which is a structured output dataset with function-calling conversations, json-mode, agentic json-mode and structured extraction samples, designed to train LLM models in performing function calls and returning structured output based on natural language instructions.

In this example, you will be using the `func_calling_singleturn` subset which contains single turn function calls, with ~1900 samples.

```python
%pip install datasets transformers --upgrade --quiet
```

```python
from datasets import load_dataset

dataset = load_dataset("NousResearch/hermes-function-calling-v1", "func_calling_singleturn", split="train")
# Dataset({
#     features: ['id', 'conversations', 'tools', 'category', 'subcategory', 'task'],
#     num_rows: 1893
# })
```

```python
dataset[0]

# {'id': '85f6c398-69c7-4df2-aed1-29d614a93a26',
#  'conversations': [{'from': 'system',
#    'value': 'You are a function calling AI model. You are provided with function signatures within <tools> </tools> XML tags. You may call one or more functions to assist with the user query. Don\'t make assumptions about what values to plug into functions.\n<tools>\n[{"type": "function", "function": {"name": "get_camera_live_feed", "description": "Retrieves the live feed from a specified security camera.", "parameters": {"type": "object", "properties": {"camera_id": {"type": "string", "description": "The unique identifier for the camera."}, "stream_quality": {"type": "string", "description": "The desired quality of the live stream.", "enum": ["720p", "1080p", "4k"]}}, "required": ["camera_id"]}}}, {"type": "function", "function": {"name": "list_all_cameras", "description": "Lists all the security cameras connected to the home network.", "parameters": {"type": "object", "properties": {"include_offline": {"type": "boolean", "description": "Whether to include cameras that are currently offline.", "default": false}}, "required": []}}}, {"type": "function", "function": {"name": "record_camera_feed", "description": "Starts recording the live feed from a specified security camera.", "parameters": {"type": "object", "properties": {"camera_id": {"type": "string", "description": "The unique identifier for the camera."}, "duration": {"type": "integer", "description": "The duration in minutes for which to record the feed.", "default": 60}}, "required": ["camera_id"]}}}, {"type": "function", "function": {"name": "get_recorded_feed", "description": "Retrieves a previously recorded feed from a specified security camera.", "parameters": {"type": "object", "properties": {"camera_id": {"type": "string", "description": "The unique identifier for the camera."}, "start_time": {"type": "string", "description": "The start time of the recording to retrieve, in ISO 8601 format."}, "end_time": {"type": "string", "description": "The end time of the recording to retrieve, in ISO 8601 format."}}, "required": ["camera_id", "start_time", "end_time"]}}}, {"type": "function", "function": {"name": "pan_tilt_camera", "description": "Controls the pan and tilt functions of a PTZ (Pan-Tilt-Zoom) security camera.", "parameters": {"type": "object", "properties": {"camera_id": {"type": "string", "description": "The unique identifier for the PTZ camera."}, "pan_angle": {"type": "integer", "description": "The angle in degrees to pan the camera. Positive values pan right, negative values pan left."}, "tilt_angle": {"type": "integer", "description": "The angle in degrees to tilt the camera. Positive values tilt up, negative values tilt down."}}, "required": ["camera_id", "pan_angle", "tilt_angle"]}}}]\n</tools>\nFor each function call return a json object with function name and arguments within <tool_call> </tool_call> tags with the following schema:\n<tool_call>\n{"name": <function-name>, "arguments": <args-dict>}\n</tool_call>\n'},
#   {'from': 'human',
#    'value': 'I\'ve recently installed a new security system at my home, and I want to ensure everything is functioning as it should. Specifically, I\'d like to start by checking the live feed from the camera located at the front door to monitor any activity. The camera has a unique identifier, which I\'ve already configured to be "front_door." I\'d prefer to view the live stream in high definition, so a 1080p quality would be ideal. Could you please call the appropriate function to retrieve the live feed from my front door camera in 1080p quality and provide me with the link to the stream?\n\nFollowing this, I would also like to record the live feed from this camera for the next 30 minutes. This is to test the recording feature and to keep an archived copy for security purposes. Please initiate the recording function for the "front_door" camera with a recording duration of 30 minutes.\n\nLastly, as part of my routine surveillance checks, I need to review footage from yesterday between 3 PM and 5 PM. The camera "front_garden" should have the recording for that period. I need to retrieve this specific recorded feed. The start time for the recording was at 15:00 on April 22, 2023, and it ended at 17:00 on the same day.\n\nTo summarize, I request the execution of the following functions with the provided details:\n1. Retrieve the live feed from the "front_door" camera at 1080p quality.\n2. Start a 30-minute recording of the live feed from the "front_door" camera.\n3. Retrieve the recorded feed from the "front_garden" camera, focusing on the time period between 15:00 and 17:00 on April 22, 2023.\n\nThank you for assisting with the management of my home security camera feeds.'},
#   {'from': 'gpt',
#    'value': '<tool_call>\n{"name": "get_camera_live_feed", "arguments": {"camera_id": "front_door", "stream_quality": "1080p"}}\n</tool_call>\n<tool_call>\n{"name": "record_camera_feed", "arguments": {"camera_id": "front_door", "duration": 30}}\n</tool_call>\n<tool_call>\n{"name": "get_recorded_feed", "arguments": {"camera_id": "front_garden", "start_time": "2023-04-22T15:00:00Z", "end_time": "2023-04-22T17:00:00Z"}}\n</tool_call>\n'}],
#  'tools': '[{"type": "function", "function": {"name": "get_camera_live_feed", "description": "Retrieves the live feed from a specified security camera.", "parameters": {"type": "object", "properties": {"camera_id": {"type": "string", "description": "The unique identifier for the camera."}, "stream_quality": {"type": "string", "description": "The desired quality of the live stream.", "enum": ["720p", "1080p", "4k"]}}, "required": ["camera_id"]}}}, {"type": "function", "function": {"name": "list_all_cameras", "description": "Lists all the security cameras connected to the home network.", "parameters": {"type": "object", "properties": {"include_offline": {"type": "boolean", "description": "Whether to include cameras that are currently offline.", "default": false}}, "required": []}}}, {"type": "function", "function": {"name": "record_camera_feed", "description": "Starts recording the live feed from a specified security camera.", "parameters": {"type": "object", "properties": {"camera_id": {"type": "string", "description": "The unique identifier for the camera."}, "duration": {"type": "integer", "description": "The duration in minutes for which to record the feed.", "default": 60}}, "required": ["camera_id"]}}}, {"type": "function", "function": {"name": "get_recorded_feed", "description": "Retrieves a previously recorded feed from a specified security camera.", "parameters": {"type": "object", "properties": {"camera_id": {"type": "string", "description": "The unique identifier for the camera."}, "start_time": {"type": "string", "description": "The start time of the recording to retrieve, in ISO 8601 format."}, "end_time": {"type": "string", "description": "The end time of the recording to retrieve, in ISO 8601 format."}}, "required": ["camera_id", "start_time", "end_time"]}}}, {"type": "function", "function": {"name": "pan_tilt_camera", "description": "Controls the pan and tilt functions of a PTZ (Pan-Tilt-Zoom) security camera.", "parameters": {"type": "object", "properties": {"camera_id": {"type": "string", "description": "The unique identifier for the PTZ camera."}, "pan_angle": {"type": "integer", "description": "The angle in degrees to pan the camera. Positive values pan right, negative values pan left."}, "tilt_angle": {"type": "integer", "description": "The angle in degrees to tilt the camera. Positive values tilt up, negative values tilt down."}}, "required": ["camera_id", "pan_angle", "tilt_angle"]}}}]',
#  'category': 'IoT and Home Automation',
#  'subcategory': 'Security Camera Management',
#  'task': 'View and Manage Security Camera Feeds'}
```

As you can see, the dataset has the columns: "id", "conversations", "tools", "category", "subcategory", and "task"; but the content you will be using for fine-tuning are the "conversations". Now given that the dataset has ~1900 rows, we'll shrink it to 500 rows stratified by "category".

```python
from datasets import ClassLabel

category_counts = dataset.to_pandas()["category"].value_counts()
valid_categories = category_counts[category_counts >= 2].index.tolist()
dataset = dataset.filter(lambda x: x["category"] in valid_categories)

categories = ClassLabel(num_classes=len(set(dataset["category"])), names=list(set(dataset["category"])))
dataset = dataset.cast_column("category", categories)
dataset = dataset.train_test_split(train_size=500, stratify_by_column="category", seed=42)["train"]
```

Then, as the "conversations" are formatted if a particular way, we'll parse those to be compliant with what the `AutoTokenizer.apply_chat_template` expects to pre-format those rows so that we can directly ingest those when training, which means applying the `Qwen/Qwen3.5-2B` default chat template to capture the system, user, assistant and tool calling messages and formats those as a single string (including the special tokens).

```python
import re, json
from transformers import AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen3.5-2B")

def map_to_messages_and_tools(sample: dict) -> dict:
    messages = []
    for conversation in sample["conversations"]:
        match conversation["from"]:
            case "system":
                messages.append({"role": "system", "content": conversation["value"]})
            case "human":
                messages.append({"role": "user", "content": conversation["value"]})
            case "gpt":
                pattern = r"<tool_call>\n(.*?)\n</tool_call>"
                matches = re.findall(pattern, conversation["value"], re.DOTALL)
                tool_calls = []
                for match in matches:
                    match = json.loads(match)
                    tool_calls.append({"type": "tool", "function": {"name": match["name"], "arguments": match["arguments"]}})
                messages.append({"role": "assistant", "tool_calls": tool_calls})

    return {"text": tokenizer.apply_chat_template(messages, tools=json.loads(sample["tools"]), add_generation_prompt=False, tokenize=False)}

dataset = dataset.map(map_to_messages_and_tools, remove_columns=set(dataset.features), num_proc=16, batched=False)
```

```python
dataset[0]["text"]
# <|im_start|>system\n# Tools\n\nYou have access to the following functions:\n\n<tools>\n{"type": "function", "function": {"name": "ExpertQAExtractor", "description": "Extracts a list of questions that require making logical inferences based on the information in the document. These test understanding.", "parameters": {"type": "object", "properties": {"inference_questions": {"type": "array", "items": {"type": "string"}}}, "required": ["inference_questions"]}}}\n</tools>\n\nIf you choose to call a function ...]
```

Finally, you need to save the dataset as JSON-L locally, as it will be pushed to Azure Machine Learning when running the job. Note that you could as well use any other format, but JSON-L is convenient here and, so on, preferred.

```python
dataset.to_json("data/train.jsonl")
```

## Train your model

The training script below uses [TRL's `SFTTrainer`](https://huggingface.co/docs/trl/en/sft_trainer) to fine-tune [`Qwen/Qwen3.5-2B`](https://huggingface.co/Qwen/Qwen3.5-2B) on the dataset you just prepared. Here is a summary of the key training arguments:

| Argument | Value | Description |
| --- | --- | --- |
| `num_train_epochs` | `3` | Number of full passes over the training data. |
| `per_device_train_batch_size` | `4` | Samples per forward/backward pass on the GPU. |
| `learning_rate` | `5e-6` | Peak learning rate for the optimizer. |
| `lr_scheduler_type` | `cosine` | Cosine decay schedule after warmup. |
| `optim` | `adamw_torch_fused` | Fused AdamW for faster optimizer steps on CUDA. |
| `bf16` | `True` | Use bfloat16 mixed precision to save memory and speed up training. |
| `packing` | `False` | Do not pack multiple samples into a single sequence. |
| `logging_steps` | `10` | Log training metrics every 10 steps. |

```python
%mkdir -p code/
```

```python
%%writefile code/train.py
import argparse
import os

import torch
from datasets import load_dataset
from transformers import AutoModelForCausalLM, AutoTokenizer
from trl import SFTConfig, SFTTrainer

if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    parser.add_argument("--train-file", type=str, help="Input data for training as JSON-L")
    parser.add_argument("--model-dir", type=str, help="Output directory for the fine-tuned model")
    
    args = parser.parse_args()

    tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen3.5-2B")
    
    model = AutoModelForCausalLM.from_pretrained(
        "Qwen/Qwen3.5-2B",
        dtype="auto",
        device_map="auto",
        attn_implementation="flash_attention_2",
    )

    config = SFTConfig(
        output_dir=args.model_dir,
        packing=False,
        num_train_epochs=3,
        per_device_train_batch_size=4,
        optim="adamw_torch_fused",
        logging_steps=10,
        learning_rate=5e-6,
        bf16=True,
        lr_scheduler_type="cosine",
        log_level="info",
    )
    
    train_dataset = load_dataset("json", data_files=args.train_file, split="train")
    
    trainer = SFTTrainer(
        model=model,
        args=config,
        train_dataset=train_dataset,
        processing_class=tokenizer,
    )
    train_result = trainer.train()
    
    metrics = train_result.metrics
    print(f"metrics={metrics}")

    os.makedirs(args.model_dir, exist_ok=True)
    trainer.save_model(args.model_dir)
```

The script accepts two arguments: `--train-file` (the JSON-L dataset) and `--model-dir` (where the fine-tuned weights are saved). Both are wired automatically by Azure Machine Learning through the `command` job inputs.

When you run the job, Azure Machine Learning will provision the compute, pull the container image, upload the `code/` and `data/` directories, and execute `accelerate launch train.py`. You can monitor progress in real-time from the notebook (via `client.jobs.stream`) or in the Azure Machine Learning Studio UI.

```python
from azure.ai.ml import command, Input
from azure.ai.ml.entities import ResourceConfiguration
from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient

secret_client = SecretClient(
    vault_url=f"https://{os.getenv('KEY_VAULT_NAME')}.vault.azure.net/",
    credential=DefaultAzureCredential(),
)

environment_variables = {}
if token := secret_client.get_secret("HF-TOKEN").value:
    environment_variables["HF_TOKEN"] = token

job = command(
    inputs={"train_file": Input(type="uri_file", path="data/train.jsonl"), "output": "./outputs"},
    code="./code",
    compute="gpu-h100",
    command="accelerate launch train.py --train-file ${{inputs.train_file}} --model-dir ${{inputs.output}}",
    environment=environment,
    environment_variables=environment_variables,
)

job = client.jobs.create_or_update(job)
client.jobs.stream(job.name)
```

The command above will provide an Azure Machine Learning Studio URL as:

```
RunId: elated_onion_8vpcbprcbv
Web View: https://ml.azure.com/runs/elated_onion_8vpcbprcbv?wsid=/subscriptions/.../resourcegroups/.../workspaces/...
```

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/machine-learning/tool-calling-agent-fine-tuning/azure-machine-learning-experiment.png)

It'll take ~10 minutes to complete **once queued**, as queueing might also take some time depending on the instance availability in the region.

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/machine-learning/tool-calling-agent-fine-tuning/azure-machine-learning-job.png)

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/machine-learning/tool-calling-agent-fine-tuning/azure-machine-learning-logs.png)

Once completed, you will have the artifacts available under the "Outputs + logs" tab in the experiment, that you can keep on Azure or pull those locally.

```python
client.jobs.download(name=job.name, download_path="./outputs", output_name="default")
```

## (Optional) Publish the model on the Hub

Optionally, you can upload the trained model weights to the Hugging Face Hub to share it with the open-source community as:

```python
%pip install huggingface_hub --upgrade --quiet
```

```python
from huggingface_hub import HfApi

api = HfApi()

api.create_repo(repo_id="my-awesome-qwen-agent", repo_type="model")

api.upload_folder(
    folder_path="./outputs/artifacts/outputs/",
    repo_id="my-awesome-qwen-agent",
    repo_type="model",
    commit_message="Upload model weights finetuned on Azure Machine Learning",
    ignore_patterns=["**/*.pyc", "**/__pycache__/*"],
)
```

## Run your trained agent

Now that training is complete, you have a lightweight agent that knows how to select and invoke tools on your behalf. You can load it locally with Hugging Face Transformers and test it against any function signature you provide:

```python
%pip install "transformers>=5.2.0" "torch<2.8" --upgrade --quiet
```

```python
import torch
from transformers import AutoTokenizer, AutoModelForCausalLM

tokenizer = AutoTokenizer.from_pretrained("./outputs/artifacts/outputs/")
model = AutoModelForCausalLM.from_pretrained("./outputs/artifacts/outputs/", torch_dtype=torch.bfloat16, device_map="auto")
```

```python
from typing import List

def list_blobs_in_container(container: str, count: int | None = None) -> List[str]:
    """
    Lists the blobs in a container on Azure Blob Storage.

    Args:
        container: The name of the container in Azure Blob Storage.
        count: The number of blobs to list, all if None. Defaults to None.

    Returns:
        A list of strings, where each string is a blob in the container up to
        `count` blobs.
    """
    return ...

inputs = tokenizer.apply_chat_template(
    {"role": "user", "content": "How many blobs are there in my `huggingface` container?"},
    tools=[list_blobs_in_container],
    return_tensors="pt",
    return_dict=True,
    add_generation_prompt=True,
    tokenize=True,
)
inputs.to(model.device)

output = model.generate(**inputs, max_new_tokens=128)
print(tokenizer.decode(output[0, inputs["input_ids"].shape[1]:]))
# <tool_call>
# {"arguments": {"container": "huggingface", "count": null}, "name": "list_blobs_in_container"}
# </tool_call><|im_end|>
```

## Next steps

* [Add support for MLFlow for live metrics on Azure Machine Learning](https://learn.microsoft.com/en-us/azure/machine-learning/how-to-log-view-metrics?view=azureml-api-2&tabs=interactive#list-of-training-metrics)
* Train the tool calling agent on your own tool calling dataset
* Deploy the fine-tuned model with any of vLLM, SGLang, llama.cpp, etc. containers on Microsoft Foundry or Azure Machine Learning as a Managed Online Endpoint

## References

- [Train models with Azure Machine Learning CLI, SDK, and REST API](https://learn.microsoft.com/en-us/azure/machine-learning/how-to-train-model?view=azureml-api-2&tabs=python)
- [Use authentication credential secrets in Azure Machine Learning jobs](https://learn.microsoft.com/en-us/azure/machine-learning/how-to-use-secrets-in-runs?view=azureml-api-2&tabs=managed)
- [Transformers Reinforcement Learning - TRL](https://huggingface.co/docs/trl/en/index)

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/Microsoft-Azure/tree/main/examples/machine-learning/tool-calling-agent-fine-tuning/azure-notebook.ipynb)!
