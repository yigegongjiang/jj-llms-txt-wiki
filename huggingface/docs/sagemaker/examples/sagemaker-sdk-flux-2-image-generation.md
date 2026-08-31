# Decorate your house with Flux.2 and Amazon SageMaker AI

Written by Enrique Hernández CalabrésLast updated 2026-08-31

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/sagemaker/notebooks/sagemaker-sdk/flux-2-image-generation/cover.png)

In this notebook, we'll deploy [`black-forest-labs/FLUX.2-klein-4B`](https://huggingface.co/black-forest-labs/FLUX.2-klein-4B) with the Hugging Face **vLLM-Omni** Deep Learning Container (DLC) on Amazon SageMaker. To make the example concrete, we'll use it as an **interior design studio**: upload one or more room references and ask the model to redesign them in a specific style.

We'll walk through the following steps:

- Select an image generation and editing model and a SageMaker DLC for your use case
- Deploy the model to SageMaker using the SageMaker Python SDK resource APIs
- Invoke the endpoint with a local reference image and prompt
- Create a Gradio app to interact with the SageMaker endpoint
- Optionally expose the app as an MCP server so agents can call it
- Clean up the endpoint resources to avoid ongoing charges

For this example, you'll need AWS credentials, a SageMaker execution role, and a Hugging Face token that can download the model.

## How image generation and editing models work

Image generation models synthesize images from text prompts, usually by starting from noise and iteratively denoising it toward an image that matches the prompt. Image editing extends that workflow by conditioning the generation on one or more reference images, so the model can preserve structure while changing materials, colors, lighting, or style.

For an interior design workflow, this means you can provide a photo of a room and ask for a different design direction, such as Scandinavian minimalism, warm mid-century furniture, or a brighter staging for a real-estate listing.

## What's vLLM-Omni

[vLLM-Omni](https://docs.vllm.ai/projects/vllm-omni/en/latest/) is an extension of the [vLLM](https://github.com/vllm-project/vllm) inference engine built for multimodal and multi-task serving. It keeps vLLM's high-throughput serving stack and adds support for additional modalities and tasks beyond classic text generation.

In this example, we'll use its **image-text-to-image** capabilities. Requests are sent as OpenAI-compatible chat-completion payloads that include text instructions and image URLs. For local images, we'll encode the image as a base64 `data:` URI before sending it to the SageMaker endpoint.

## Setup

To run this example, we'll install the SageMaker Python SDK for model deployment, `huggingface_hub` for authentication, and `gradio[mcp]` for the interactive image app and optional MCP server.

```python
%pip install -q "sagemaker>=3" huggingface_hub "gradio[mcp]" pillow
```

We are going to need:

- An `HF_TOKEN`: This token will be used to download the model from Hugging Face.
- A SageMaker execution role: This role will be used to pull the image from the registry and deploy the model to SageMaker.

Let's start by setting up the execution role and the token.

```python
from huggingface_hub import get_token, notebook_login

if not (HF_TOKEN := get_token()):
    notebook_login()
    HF_TOKEN = get_token()

print("HF_TOKEN_LOADED")
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

## Choosing a model and a DLC

There are many image generation and editing models on the Hugging Face Hub, from small specialist models to larger general-purpose systems.

For this example, we'll use [`black-forest-labs/FLUX.2-klein-4B`](https://huggingface.co/black-forest-labs/FLUX.2-klein-4B), a compact FLUX.2 model that supports image generation and editing. If you need higher quality and have more GPU capacity, you can adapt the same deployment pattern to a larger FLUX.2 variant.

We'll serve the model with the Hugging Face **vLLM-Omni** DLC, which exposes multimodal APIs suitable for SageMaker hosting. You can browse supported models in the [vLLM-Omni docs](https://docs.vllm.ai/projects/vllm-omni/en/latest/models/supported_models/), and other official Hugging Face DLCs on the [AWS Available Images](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-dg.pdf#available-images) page.

We'll target an `ml.g5.xlarge` instance for the example endpoint. For larger models, higher resolutions, or heavier concurrency, consider larger GPU instance types such as `ml.g5.2xlarge`, `ml.g6e.*`, or other available SageMaker GPU instances in your region.

```python
from time import strftime

MODEL_ID = "black-forest-labs/FLUX.2-klein-4B"
RESOURCE_SUFFIX = strftime("%Y%m%d-%H%M%S")

MODEL_NAME = f"flux-2-image-generation-model-{RESOURCE_SUFFIX}"
ENDPOINT_CONFIG_NAME = f"flux-2-image-generation-config-{RESOURCE_SUFFIX}"
ENDPOINT_NAME = f"flux-2-image-generation-endpoint-{RESOURCE_SUFFIX}"

DLC = "huggingface-vllm-omni"
INSTANCE_TYPE = "ml.g5.xlarge"
```

In order to tell AWS which DLC we want to use, we need to specify its `image_uri`. As mentioned before, this URI can be found in the [AWS Available Images](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-dg.pdf#available-images) page. If you want to obtain it programmatically, the SageMaker Python SDK also provides a helper:

```python
from sagemaker.core import image_uris

VLLM_OMNI_IMAGE_TAG = "0.20.0-transformers5.8.1-gpu-py312-cu130-amzn2023"

try:
    image_uri = image_uris.retrieve(
        DLC,
        region=REGION,
        image_scope="inference",
        instance_type=INSTANCE_TYPE,
    )
except Exception as e:
    image_uri = f"763104351884.dkr.ecr.{REGION}.amazonaws.com/huggingface-vllm-omni:{VLLM_OMNI_IMAGE_TAG}"
    print(f"SageMaker SDK image lookup failed ({e}); using fallback URI.")

print(image_uri)
```

If the helper does not yet return the image URI you need, set the URI explicitly to a vLLM-Omni SageMaker image in your region. SDK catalogs can lag behind new DLC releases, so the [AWS Available Images](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-dg.pdf#available-images) page is the source to check when you need the latest published image tag.

## Deploying the model to SageMaker

Now that we have the image URI, we can deploy the model to SageMaker. The SageMaker Python SDK v3 supports several deployment styles. In this notebook we'll use the low-level resource APIs - `Model`, `EndpointConfig`, and `Endpoint` - which give explicit control over the container environment, instance type, and optional inference AMI.

Before creating the endpoint, we'll define the environment variables used to configure the DLC and the image editing task.

```python
env_vars = {
    "HF_TOKEN": HF_TOKEN,  # Token to download the model from Hugging Face
    "HF_TASK": "image-text-to-image",  # Task to perform
    "SM_VLLM_MODEL": MODEL_ID,  # Model ID loaded by the vLLM-Omni container
}
```

A few notes on those variables:

- `HF_TOKEN` authenticates downloads from the Hub.
- `HF_TASK` tells the serving stack this is an **image-text-to-image** workload.
- `SM_VLLM_MODEL` selects the model loaded by the container.

The `inference_ami_version` on the production variant pins the host GPU driver / CUDA stack on the instance (separate from the OS inside the container image). For `ml.g5.*` instances, `al2-ami-sagemaker-inference-gpu-2-1` is a solid default; omit it if you prefer SageMaker's instance-type default.

**Cost note:** the next cell creates a billable GPU-backed SageMaker real-time endpoint. Delete it in the cleanup section when you are done.

Create the endpoint and wait for it to become `InService`:

```python
from sagemaker.core.resources import Endpoint, EndpointConfig, Model
from sagemaker.core.shapes import ContainerDefinition, ProductionVariant

model = Model.create(
    model_name=MODEL_NAME,
    primary_container=ContainerDefinition(
        image=image_uri,
        environment=env_vars,
    ),
    execution_role_arn=role,
    region=REGION,
)

endpoint_config = EndpointConfig.create(
    endpoint_config_name=ENDPOINT_CONFIG_NAME,
    production_variants=[
        ProductionVariant(
            variant_name="default",
            model_name=MODEL_NAME,
            instance_type=INSTANCE_TYPE,
            initial_instance_count=1,
            inference_ami_version="al2-ami-sagemaker-inference-gpu-2-1",
        ),
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

## Testing the endpoint

Once the endpoint is in service, we can call it with the SageMaker Runtime `invoke_endpoint` API. For image editing through this DLC, requests go to the chat-completions route via `CustomAttributes="route=/v1/chat/completions"`.

The payload contains:

- `messages`: a user message with text instructions and one or more image URLs
- `extra_body`: image generation parameters such as height, width, inference steps, guidance scale, and seed

The response contains a base64 image URL in the assistant message content. We'll decode that data URI and display the generated image in the notebook.

### Routing requests through the vLLM-Omni middleware

SageMaker sends real-time endpoint requests to the container's standard `/invocations` route. The vLLM-Omni DLC includes middleware that reads SageMaker's `CustomAttributes` request property and uses it to forward the request internally to the matching vLLM-Omni API route.

For image editing, set:

```python
CustomAttributes="route=/v1/chat/completions"
```

The middleware then redirects the request from SageMaker's `/invocations` entry point to vLLM-Omni's OpenAI-compatible `/v1/chat/completions` endpoint. This is why the invocation payload follows the chat-completions format while the request is still made through SageMaker Runtime's `invoke_endpoint` API.

We'll use a kitchen reference image to test the endpoint. If you want to test it with your own room photo, replace `SAMPLE_IMAGE_PATH` with the path to your image.

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/sagemaker/notebooks/sagemaker-sdk/flux-2-image-generation/old_kitchen.png)

The image dimensions are resized to multiples of 16 before invocation, which is a common requirement for diffusion-style image models.

```python
from pathlib import Path

from IPython.display import display
from PIL import Image

SAMPLE_IMAGE_PATH = Path("assets/old_kitchen.png")

def resize_to_multiple_of_16(image: Image.Image) -> Image.Image:
    width, height = image.size
    resized_width = max(16, round(width / 16) * 16)
    resized_height = max(16, round(height / 16) * 16)

    if (resized_width, resized_height) == image.size:
        return image

    return image.resize((resized_width, resized_height), Image.Resampling.LANCZOS)

input_image = Image.open(SAMPLE_IMAGE_PATH).convert("RGB")
input_image = resize_to_multiple_of_16(input_image)
width, height = input_image.size

display(input_image)
```

```python
import base64
import io
import json

runtime = boto_sess.client("sagemaker-runtime")

def image_to_data_uri(image: Image.Image) -> str:
    with io.BytesIO() as buffer:
        image.convert("RGB").save(buffer, format="PNG")
        encoded_image = base64.b64encode(buffer.getvalue()).decode("utf-8")
    return f"data:image/png;base64,{encoded_image}"

def extract_image_from_response(response_body: bytes) -> Image.Image:
    payload = json.loads(response_body)
    content = payload["choices"][0]["message"]["content"]

    if isinstance(content, str):
        content = json.loads(content)

    image_url = next(item["image_url"]["url"] for item in content if item.get("type") == "image_url")
    encoded_image = image_url.split(",", 1)[1]
    return Image.open(io.BytesIO(base64.b64decode(encoded_image)))

PROMPT = (
    "Redesign this kitchen as a warm Scandinavian interior "
    "with oak furniture, soft daylight, plants, and a clean editorial style."
)

response = runtime.invoke_endpoint(
    EndpointName=ENDPOINT_NAME,
    ContentType="application/json",
    Body=json.dumps(
        {
            "model": MODEL_ID,
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": PROMPT,
                        },
                        {
                            "type": "image_url",
                            "image_url": {"url": image_to_data_uri(input_image)},
                        },
                    ],
                }
            ],
            "extra_body": {
                "height": height,
                "width": width,
                "num_inference_steps": 30,
                "guidance_scale": 1,
                "seed": 42,
            },
        }
    ),
    CustomAttributes="route=/v1/chat/completions",
)

generated_image = extract_image_from_response(response["Body"].read())
generated_image.save("redesigned-kitchen.png")
display(generated_image)
```

```python
from PIL import ImageDraw

comparison_size = (512, 512)
old_image = input_image.copy()
new_image = generated_image.convert("RGB")

old_image.thumbnail(comparison_size)
new_image.thumbnail(comparison_size)

title_height = 40
image_height = max(old_image.height, new_image.height)

comparison = Image.new(
    "RGB",
    (comparison_size[0] * 2, title_height + image_height),
    "white",
)
comparison.paste(old_image, ((comparison_size[0] - old_image.width) // 2, title_height))
comparison.paste(new_image, (comparison_size[0] + (comparison_size[0] - new_image.width) // 2, title_height))

draw = ImageDraw.Draw(comparison)
draw.text((10, 10), "Original reference", fill="black")
draw.text((comparison_size[0] + 10, 10), "Edited result", fill="black")
comparison.save("redesigned-kitchen-comparison.png")
display(comparison)
```

The generated result keeps the reference image as visual context while applying the requested redesign. In the example below, the kitchen is restyled with a warmer color palette, oak furniture, and a clean editorial look.

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/sagemaker/notebooks/sagemaker-sdk/flux-2-image-generation/comparison.png)

### Optional: generate a new image without a reference image

The same endpoint can also be used for text-to-image generation by sending only text content in the chat message. Use this as a quick smoke test for pure generation.

```python
# Smoke test: generate an image without a reference image.
text_to_image_response = runtime.invoke_endpoint(
    EndpointName=ENDPOINT_NAME,
    ContentType="application/json",
    Body=json.dumps(
        {
            "model": MODEL_ID,
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": "A cozy kitchen with a warm color palette, oak furniture, and a clean editorial style.",
                        }
                    ],
                }
            ],
            "extra_body": {
                "height": 512,
                "width": 512,
                "num_inference_steps": 30,
                "guidance_scale": 1,
                "seed": 42,
            },
        }
    ),
    CustomAttributes="route=/v1/chat/completions",
)

text_to_image = extract_image_from_response(text_to_image_response["Body"].read())
display(text_to_image)
```

## Creating a Gradio app

Now that the SageMaker endpoint works, we can wrap it in a Gradio app for interactive use. The app lets you upload multiple room reference images, provide an interior design prompt, and send one request per image to the same SageMaker endpoint.

The requests are submitted in parallel, with a small concurrency limit to keep the endpoint responsive. The more images you upload, the longer it will take to generate all results, and each image still consumes endpoint capacity while it runs.

**Security and cost warning:** `share=True` creates a public Gradio URL. Anyone with that URL can access your app and submit image-generation requests, which invoke the already-deployed billable SageMaker endpoint using your AWS credentials. Share the URL only with trusted users, and disable the app or delete the endpoint when you are finished.

Launching with `mcp_server=True` also exposes the generation workflow to agent harnesses, which is covered in the optional MCP section.

```python
import json
import tempfile
from concurrent.futures import ThreadPoolExecutor

import gradio as gr
from PIL import Image

runtime = boto_sess.client("sagemaker-runtime")

MAX_PARALLEL_REQUESTS = 4

STYLE_PRESETS = {
    "Custom": "",
    "Warm Scandinavian": "warm Scandinavian interior, oak furniture, soft daylight, neutral textiles, plants",
    "Mid-century staging": "mid-century modern staging, walnut furniture, sculptural lighting, warm color accents",
    "Minimal luxury": "minimal luxury interior, stone surfaces, refined furniture, gallery lighting, calm palette",
    "Japandi calm": "Japandi interior, low furniture, natural materials, paper lamps, quiet composition",
}

def extract_image_from_payload(payload: dict) -> Image.Image:
    content = payload["choices"][0]["message"]["content"]

    if isinstance(content, str):
        content = json.loads(content)

    image_url = next(item["image_url"]["url"] for item in content if item.get("type") == "image_url")
    encoded_image = image_url.split(",", 1)[1]
    return Image.open(io.BytesIO(base64.b64decode(encoded_image)))

def normalize_file_paths(uploaded_files) -> list[str]:
    if not uploaded_files:
        raise gr.Error("Upload at least one reference image.")

    file_paths = []
    for uploaded_file in uploaded_files:
        file_paths.append(uploaded_file if isinstance(uploaded_file, str) else uploaded_file.name)

    return file_paths

def invoke_flux(image_path: str, prompt: str, seed: int, steps: int, guidance_scale: float) -> str:
    reference_image = Image.open(image_path).convert("RGB")
    reference_image = resize_to_multiple_of_16(reference_image)
    width, height = reference_image.size

    content = [
        {"type": "text", "text": prompt.strip()},
        {"type": "image_url", "image_url": {"url": image_to_data_uri(reference_image)}},
    ]

    response = runtime.invoke_endpoint(
        EndpointName=ENDPOINT_NAME,
        ContentType="application/json",
        Body=json.dumps(
            {
                "model": MODEL_ID,
                "messages": [{"role": "user", "content": content}],
                "extra_body": {
                    "height": height,
                    "width": width,
                    "num_inference_steps": int(steps),
                    "guidance_scale": float(guidance_scale),
                    "seed": int(seed),
                },
            }
        ),
        CustomAttributes="route=/v1/chat/completions",
    )

    image = extract_image_from_payload(json.loads(response["Body"].read()))
    with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as f:
        image.save(f.name)
        return f.name

def generate_room_designs(
    reference_images,
    style_preset,
    custom_prompt,
    seed,
    steps,
    guidance_scale,
):
    file_paths = normalize_file_paths(reference_images)
    preset_prompt = STYLE_PRESETS[style_preset]

    if style_preset == "Custom":
        if not custom_prompt or not custom_prompt.strip():
            raise gr.Error("Enter a design prompt.")
        prompt = custom_prompt.strip()
    elif custom_prompt and custom_prompt.strip():
        prompt = f"{custom_prompt.strip()}. Style direction: {preset_prompt}."
    else:
        prompt = f"Redesign this room as a {preset_prompt}. Preserve the room layout and camera angle."

    max_workers = min(MAX_PARALLEL_REQUESTS, len(file_paths))
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        futures = [
            executor.submit(invoke_flux, image_path, prompt, int(seed) + index, steps, guidance_scale)
            for index, image_path in enumerate(file_paths)
        ]
        return [future.result() for future in futures]

with gr.Blocks(title="FLUX.2 Interior Studio") as demo:
    gr.Markdown("# FLUX.2 Interior Studio")

    with gr.Row():
        with gr.Column():
            reference_images = gr.File(
                label="Reference images",
                file_count="multiple",
                file_types=["image"],
                type="filepath",
            )

        with gr.Column():
            style_preset = gr.Dropdown(
                choices=list(STYLE_PRESETS),
                value="Warm Scandinavian",
                label="Style",
            )
            custom_prompt = gr.Textbox(
                label="Prompt",
                value="Preserve the layout and redesign the space for a bright residential listing.",
                lines=4,
            )
            seed = gr.Number(label="Seed", value=42, precision=0)
            steps = gr.Slider(label="Steps", minimum=10, maximum=80, value=30, step=1)
            guidance_scale = gr.Slider(label="Guidance", minimum=1.0, maximum=12.0, value=1.0, step=0.5)
            generate_btn = gr.Button("Generate", variant="primary")

    output_gallery = gr.Gallery(
        label="Results",
        columns=2,
        height=640,
        object_fit="contain",
    )

    generate_btn.click(
        fn=generate_room_designs,
        inputs=[
            reference_images,
            style_preset,
            custom_prompt,
            seed,
            steps,
            guidance_scale,
        ],
        outputs=output_gallery,
    )

ENABLE_MCP_SERVER = False

demo.launch(
    share=True,
    server_name="0.0.0.0",
    server_port=7860,
    show_error=True,
    mcp_server=ENABLE_MCP_SERVER,
)
```

## Optional: Using the Gradio app as an MCP server

If `ENABLE_MCP_SERVER=True` when the Gradio app launches, it also exposes an MCP endpoint that agent harnesses can call. That means tools like Cursor, Claude Desktop, or any other MCP-compatible client can generate room redesigns through the same Gradio app without going through the UI.

Add the following configuration to your MCP settings, replacing the URL with the Gradio server address printed when the app launches. For a shared link, use the `.../gradio_api/mcp/` URL shown in the launch logs:

```json
{
  "mcpServers": {
    "gradio": {
      "url": "http://your-server:port/gradio_api/mcp/"
    }
  }
}
```

Once connected, the agent can use the app's tool as part of a larger workflow, such as inspecting a room photo, suggesting design directions, and generating candidate redesigns through the SageMaker endpoint.

## Cleanup

To stop the endpoint and avoid ongoing charges once you're done, delete the endpoint, endpoint configuration, and model resources:

```python
endpoint.delete()
endpoint_config.delete()
model.delete()
```

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/flux-2-image-generation/sagemaker-notebook.ipynb)!
