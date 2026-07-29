# Fine-tune Gemma 4 with TRL on Vertex AI

Written by Alvaro Bartolome

Gemma 4 is the latest multimodal family of open models from Google DeepMind, now available on Hugging Face with support across the tooling many teams already use for inference, agents, and fine-tuning. The family combines strong text quality, multimodal inputs, long context windows, and model sizes that range from compact edge-friendly variants to larger frontier-class checkpoints.

All Gemma 4 variants support text and image inputs, while the smaller E2B and E4B variants also support audio. The family is released under Apache 2.0 and is designed to be broadly usable across libraries and runtimes.

| Model | Parameters | Context window | Modalities | Checkpoint |
| --- | --- | --- | --- | --- |
| Gemma 4 E2B | 2.3B effective, 5.1B with embeddings | 128K | text, image, audio | [google/gemma-4-E2B-it](https://huggingface.co/google/gemma-4-E2B-it) |
| Gemma 4 E4B | 4.5B effective, 8B with embeddings | 128K | text, image, audio | [google/gemma-4-E4B-it](https://huggingface.co/google/gemma-4-E4B-it) |
| Gemma 4 31B | 31B dense | 256K | text, image | [google/gemma-4-31B-it](https://huggingface.co/google/gemma-4-31B-it) |
| Gemma 4 26B A4B | 26B total, 4B active MoE | 256K | text, image | [google/gemma-4-26B-A4B-it](https://huggingface.co/google/gemma-4-26B-A4B-it) |

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/google-cloud/examples/vertex-ai/notebooks/fine-tune-gemma-4/thumbnail.png)

This example shows how to fine-tune `google/gemma-4-E2B-it` for tool calling with TRL on Vertex AI Serverless Training Jobs using a custom training container and a single H100 Spot VM. The training script uses the `NousResearch/hermes-function-calling-v1` dataset, stages the script in Cloud Storage, and launches the run through a Vertex AI Serverless Training Job backed by a custom container.

## Setup and Requirements

Before creating any Google Cloud resources, make sure the local environment and credentials are ready.

- You have a Google Cloud project and permissions to:
    - Create Google Cloud Storage buckets
    - Create Artifact Registry repositories for Docker
    - Run Cloud Builds to remotely build Docker containers
    - Create and run Vertex AI Serverless jobs
- You are authenticated with `gcloud` locally.
- You have access to the gated Gemma 4 model on the Hugging Face Hub and a `HF_TOKEN` with read access.

## Enable APIs and Set Environment Variables

The next steps define the project-specific values used across the notebook and make sure the required Google Cloud services are enabled before any resources are created.

### Set environment variables

Set the Google Cloud project, region, bucket, and image names that will be reused throughout the example. Keeping these values in one place makes it easier to change the deployment target, rename resources, or rerun the notebook in a different project.

```python
%env PROJECT_ID=
%env LOCATION=us-central1
%env BUCKET_URI=gs://fine-tune-gemma-4
%env REPOSITORY_NAME=huggingface-images
%env IMAGE_NAME=trl-training-cuda
```

### Authenticate and enable APIs

Install the Vertex AI SDK, authenticate both the `gcloud` CLI and application default credentials, then set the active project. After that, enable the APIs required for Vertex AI training, Artifact Registry, Cloud Build, and Cloud Storage so the later commands can run without manual setup in the console.

```python
%pip install google-cloud-aiplatform huggingface_hub --upgrade --quiet
```

```python
!gcloud auth login
```

```python
!gcloud auth application-default login
```

```python
!gcloud config set project $PROJECT_ID
```

```python
!gcloud services enable aiplatform.googleapis.com artifactregistry.googleapis.com cloudbuild.googleapis.com storage.googleapis.com
```

## Create the staging bucket

Create the Cloud Storage bucket used to upload the training script and store training outputs such as checkpoints and final model artifacts. The command below first checks whether the bucket already exists and only creates it if needed, so it is safe to rerun.

```python
%%bash
if ! gcloud storage buckets describe $BUCKET_URI >/dev/null 2>&1; then
    gcloud storage buckets create $BUCKET_URI --project $PROJECT_ID --location=$LOCATION --default-storage-class=STANDARD --uniform-bucket-level-access
fi
```

## Build the training container

Next, define the custom training container and build it with Cloud Build.

Given the target CUDA driver versions on the NVIDIA instances on Google Cloud, the maximum supported CUDA version on the build platform is CUDA 12.6; CUDA 12.8 or later won't work on Vertex AI.

```python
%%writefile Dockerfile
FROM nvidia/cuda:12.6.3-devel-ubuntu24.04
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

RUN echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] https://packages.cloud.google.com/apt cloud-sdk main" \
    | tee -a /etc/apt/sources.list.d/google-cloud-sdk.list && \
    curl https://packages.cloud.google.com/apt/doc/apt-key.gpg \
    | gpg --dearmor -o /usr/share/keyrings/cloud.google.gpg && \
    apt-get update -y && \
    apt-get install google-cloud-cli -y && \
    apt-get clean autoremove --yes && \
    rm -rf /var/lib/{apt,dpkg,cache,log}

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

RUN uv pip install "torch==2.10.0" torchvision torchaudio --torch-backend=cu126

RUN uv pip install --no-cache-dir \
    "transformers[audio,chat_template,kernels,video,vision]>=5.5.0"
    "huggingface_hub[hf_xet]==1.8.0" \
    "datasets==4.8.4" \
    "accelerate==1.13.0" \
    "trl[vlm]==1.0.0"

RUN uv pip install --no-cache-dir \
    google-cloud-storage \
    google-cloud-bigquery \
    google-cloud-aiplatform \
    google-cloud-pubsub \
    google-cloud-logging \
    gcsfs
```

```python
%%bash
if ! gcloud artifacts repositories describe $REPOSITORY_NAME --location=$LOCATION --project=$PROJECT_ID >/dev/null 2>&1; then
    gcloud artifacts repositories create $REPOSITORY_NAME --repository-format=docker --location=$LOCATION --project=$PROJECT_ID
fi
```

```python
!gcloud builds submit --tag $LOCATION-docker.pkg.dev/$PROJECT_ID/$REPOSITORY_NAME/$IMAGE_NAME:latest .
```

## Write the training script

Write the fine-tuning logic to a local `train.py` file, then upload that file to the staging bucket. This keeps the container reusable while allowing the training logic to evolve independently from the image.

The script loads the dataset from the Hugging Face Hub at runtime, prepares the tool-calling format, loads the Gemma 4 checkpoint in BF16, and launches the TRL `SFTTrainer`. When the Vertex AI Serverless Training Job starts, the Cloud Storage bucket is mounted under `/gcs/...`, so the uploaded script can be executed directly without being baked into the container.

```python
%%writefile train.py
import json
import os
import re

import torch
from datasets import ClassLabel, load_dataset
from google.cloud import aiplatform
from transformers import AutoModelForCausalLM, AutoTokenizer
from trl import SFTConfig, SFTTrainer

model_id = os.environ["MODEL_ID"]
output_dir = os.environ["AIP_MODEL_DIR"].replace("gs://", "/gcs/")

dataset = load_dataset(
    "NousResearch/hermes-function-calling-v1", "func_calling_singleturn", split="train"
)

tokenizer = AutoTokenizer.from_pretrained(model_id)

def map_to_messages_and_tools(sample):
    messages = []
    for conversation in sample["conversations"]:
        if conversation["from"] == "system":
            messages.append({"role": "system", "content": conversation["value"]})
        elif conversation["from"] == "human":
            messages.append({"role": "user", "content": conversation["value"]})
        elif conversation["from"] == "gpt":
            matches = re.findall(
                r"\n(.*?)\n", conversation["value"], re.DOTALL
            )
            tool_calls = []
            for match in matches:
                call = json.loads(match)
                tool_calls.append(
                    {
                        "type": "tool",
                        "function": {
                            "name": call["name"],
                            "arguments": call["arguments"],
                        },
                    }
                )
            messages.append({"role": "assistant", "tool_calls": tool_calls})
    return {
        "text": tokenizer.apply_chat_template(
            messages,
            tools=json.loads(sample["tools"]),
            add_generation_prompt=False,
            tokenize=False,
        )
    }

dataset = dataset.map(
    map_to_messages_and_tools, remove_columns=dataset.column_names, num_proc=8
)

model = AutoModelForCausalLM.from_pretrained(
    model_id, dtype=torch.bfloat16, attn_implementation="sdpa"
)

# NOTE: As we just want to fine-tune the text capabilities, we freeze the audio and vision towers and embeddings
for name, param in model.named_parameters():
    if not name.startswith("model.language_model"):
        param.requires_grad = False

trainer = SFTTrainer(
    model=model,
    args=SFTConfig(
        output_dir=output_dir,
        num_train_epochs=3,
        per_device_train_batch_size=4,
        gradient_accumulation_steps=2,
        gradient_checkpointing=True,
        gradient_checkpointing_kwargs={"use_reentrant": False},
        use_cache=False,
        learning_rate=5e-6,
        lr_scheduler_type="cosine",
        optim="adamw_torch_fused",
        warmup_ratio=0.03,
        bf16=True,
        logging_steps=10,
        packing=False,
        save_strategy="epoch",
        # NOTE: Liger kernels disabled as those lead to "CUDA illegal memory access encountered"
        use_liger_kernel=False,
    ),
    train_dataset=dataset,
    processing_class=tokenizer,
)

train_result = trainer.train()
trainer.save_model(output_dir)
tokenizer.save_pretrained(output_dir)
```

The `gcloud storage cp` command below copies the latest local `train.py` into the staging bucket before job submission. That makes the upload step explicit and easy to rerun whenever the script changes.

```python
!gcloud storage cp train.py $BUCKET_URI
```

## Fine-tune on Vertex AI

Submit a `CustomContainerTrainingJob` to Vertex AI Serverless Training Jobs using the image built earlier and the training script stored in GCS. This step requests the target GPU shape, passes the model configuration through environment variables, and writes checkpoints and final model artifacts to the configured output directory.

The job definition below is the main place to adapt the example to a different setup. You can change the GPU type, output path, base model ID, dataset token, or scheduling mode without rebuilding the container. In this example the job runs on a single H100 Spot VM inside Vertex AI Serverless Training Jobs and uses the local Hugging Face Hub token for access to the gated base model.

```python
from google.cloud import aiplatform
from huggingface_hub import get_token

aiplatform.init(
    project=os.getenv("PROJECT_ID"),
    location=os.getenv("LOCATION"),
    staging_bucket=os.getenv("BUCKET_URI"),
)

job = aiplatform.CustomContainerTrainingJob(
    display_name="gemma-4-fine-tuning",
    container_uri=f"{os.getenv('LOCATION')}-docker.pkg.dev/{os.getenv('PROJECT_ID')}/{os.getenv('REPOSITORY_NAME')}/{os.getenv('IMAGE_NAME')}:latest",
    command=["python", f"{os.getenv('BUCKET_URI').replace('gs://', '/gcs/')}/train.py"],
)

job = job.submit(
    replica_count=1,
    machine_type="a3-highgpu-1g",
    accelerator_type="NVIDIA_H100_80GB",
    accelerator_count=1,
    scheduling_strategy=aiplatform.compat.types.custom_job.Scheduling.Strategy.SPOT,
    base_output_dir=f"{os.getenv('BUCKET_URI')}/output-dir",
    environment_variables={"MODEL_ID": "google/gemma-4-E2B-it", "HF_TOKEN": get_token()},
    boot_disk_size_gb=500,
    timeout=60 * 60 * 3,
    create_request_timeout=60 * 10,
)
```

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/google-cloud/examples/vertex-ai/notebooks/fine-tune-gemma-4/vertex-ai-job.png)

## Use fine-tune with `transformers`

After training finishes, download the exported model artifacts from Google Cloud Storage to a local folder, then load that folder with `transformers`. The code cells below show how to copy the saved weights locally, load the fine-tuned checkpoint for regular generation, and run a multimodal function-calling example with the fine-tuned model itself rather than the original base checkpoint.

This is the quickest way to validate that the fine-tuned model behaves as expected before you deploy it elsewhere. If you prefer, you can also point `from_pretrained` at any local copy of the exported Vertex AI model directory.

```python
!mkdir -p ./gemma-4-e2b-it-tool-calling-ft
!gcloud storage cp --recursive $BUCKET_URI/output-dir/model/* ./gemma-4-e2b-it-tool-calling-ft/
```

```python
import re
import torch
from transformers import AutoModelForCausalLM, AutoProcessor

model_path = "./gemma-4-e2b-it-tool-calling-ft"
processor = AutoProcessor.from_pretrained(model_path)
model = AutoModelForCausalLM.from_pretrained(
    model_path,
    dtype=torch.bfloat16,
    device_map="auto",
)

WEATHER_TOOL = {
    "type": "function",
    "function": {
        "name": "get_weather",
        "description": "Gets the current weather for a specific location.",
        "parameters": {
            "type": "object",
            "properties": {
                "city": {"type": "string", "description": "The city name"},
            },
            "required": ["city"],
        },
    },
}

messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "image": "https://huggingface.co/datasets/merve/vlm_test_images/resolve/main/thailand.jpg"},
            {"type": "text", "text": "What is the city in this image? Check the weather there right now."},
        ],
    }
]

inputs = processor.apply_chat_template(
    messages,
    tools=[WEATHER_TOOL],
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
    add_generation_prompt=True,
    enable_thinking=True,
).to(model.device)

output = model.generate(**inputs, max_new_tokens=1000)
input_len = inputs.input_ids.shape[-1]
generated_text_ids = output[0][input_len:]
generated_text = processor.decode(generated_text_ids, skip_special_tokens=True)
result = processor.parse_response(generated_text)
print(result["content"])
```

## Share fine-tune on the Hugging Face Hub

If you want to share the fine-tuned checkpoint, create the repository if needed and upload the exported model folder to the Hugging Face Hub.

```python
from huggingface_hub import HfApi

repo_id = "/gemma-4-e2b-it-tool-calling-ft"
api = HfApi()
api.create_repo(repo_id=repo_id, exist_ok=True)
api.upload_folder(repo_id=repo_id, folder_path="./gemma-4-e2b-it-tool-calling-ft")
```

## References

- [Welcome Gemma 4](https://huggingface.co/blog/gemma4)
- [Vertex AI custom training with custom containers](https://cloud.google.com/vertex-ai/docs/training/create-custom-container)
- [Vertex AI Python SDK training classes](https://cloud.google.com/vertex-ai/docs/python-sdk/training-classes)
