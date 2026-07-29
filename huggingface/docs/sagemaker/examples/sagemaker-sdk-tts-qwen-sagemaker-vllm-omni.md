# Deploy Qwen3-TTS for ad voiceovers on Amazon SageMaker AI

Written by Enrique Hernández CalabrésLast updated 2026-07-27

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/sagemaker/notebooks/sagemaker-sdk/tts-qwen-sagemaker-vllm-omni/cover.png)

In this notebook, we'll deploy [`Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice`](https://huggingface.co/Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice) with the Hugging Face **vLLM-Omni** Deep Learning Container (DLC) on Amazon SageMaker. To make the example concrete, we'll use it as an **ad voiceover studio**: turn short marketing scripts into natural speech with a predefined speaker and optional delivery instructions. As an advanced optional path, we'll also show how to deploy the Base model for authorized voice cloning.

We'll walk through the following steps:

- Select a text-to-speech model and a SageMaker DLC for your use case
- Deploy the TTS model to SageMaker using the SageMaker Python SDK
- Invoke the endpoint to generate a sample audio
- Create a Gradio app to interact with the CustomVoice endpoint
- Optionally deploy the Base model for authorized voice cloning
- Optionally expose the app as an MCP server so agents can call it

For this example, you'll need AWS credentials and a SageMaker execution role. A Hugging Face token is optional for these public models, but recommended for authenticated Hub downloads from managed environments.

## How a TTS model works

Text-to-speech (TTS) models convert written text into audio. Modern neural TTS systems typically encode the text, map it to acoustic features (or discrete audio tokens), and decode those features into a waveform.

**Qwen3-TTS CustomVoice** goes a step further: besides synthesizing speech, it lets you pick from a set of **predefined premium voices** and steer delivery with natural-language **instructions** (tone, pacing, emotion). That makes it a strong fit for ad voiceovers, where brand tone and consistency matter as much as intelligibility.

The Qwen3-TTS family also includes Base variants for **voice cloning** from a reference audio clip and transcript. We'll keep that as an optional advanced section because it requires a second SageMaker endpoint and should only be used with authorized voice samples.

## What's vLLM-Omni

[vLLM-Omni](https://docs.vllm.ai/projects/vllm-omni/en/latest/) is an extension of the [vLLM](https://github.com/vllm-project/vllm) inference engine built for multimodal and multi-task serving. It keeps vLLM's high-throughput serving stack and adds support for additional modalities and tasks beyond classic text generation.

In this example, we'll use its **text-to-speech** capabilities to generate voiceovers from a script, a chosen speaker, a language, and optional delivery instructions.

## Setup

To run this example, we'll install the SageMaker Python SDK for model deployment, plus `huggingface_hub` for authentication and `gradio[mcp]` for the interactive voiceover app and its MCP server.

```python
%pip install -q "sagemaker>=3" huggingface_hub "gradio[mcp]"
```

We are going to need:

- An optional `HF_TOKEN`: used for authenticated model downloads from Hugging Face.
- A SageMaker execution role: used to pull the DLC from ECR and deploy the model to SageMaker.

Let's start by detecting any existing token and setting up the execution role.

```python
>> from huggingface_hub import get_token

>> HF_TOKEN = get_token()

>> if HF_TOKEN:
...     print("HF_TOKEN_LOADED")
>> else:
...     print("No HF_TOKEN found. Public Qwen3-TTS downloads should still work.")
```

HF_TOKEN_LOADED

```python
>> import os

>> import boto3
>> from sagemaker.core.helper.session_helper import Session, get_execution_role

>> REGION = boto3.Session().region_name or os.environ.get("AWS_REGION", "us-east-1")
>> boto_sess = boto3.Session(region_name=REGION)
>> sess = Session(boto_session=boto_sess)

>> try:
...     role = get_execution_role(sagemaker_session=sess)
...     print(f"Role extracted from execution role: {role}")
>> except Exception:
...     role_name = "sagemaker_execution_role"
...     iam_client = boto_sess.client("iam")
...     role = iam_client.get_role(RoleName=role_name)["Role"]["Arn"]
...     print(f"Role extracted from iam client: {role}")
```

Role extracted from iam client: arn:aws:iam::754289655784:role/sagemaker_execution_role

## Choosing a model and a DLC

There are many TTS models on the Hugging Face Hub, from lightweight monolingual synthesizers to large multilingual systems with style control.

The main path in this notebook uses [`Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice`](https://huggingface.co/Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice), which has **9 premium speakers** and instruction-based style control. Use it when a predefined voice matches the campaign.

As an optional advanced path, we'll also include [`Qwen/Qwen3-TTS-12Hz-1.7B-Base`](https://huggingface.co/Qwen/Qwen3-TTS-12Hz-1.7B-Base), which supports zero-shot voice cloning from a reference recording and transcript. Use it only with authorized voice samples.

Both models support languages such as Chinese, English, Japanese, Korean, and several European languages. We'll serve them with the Hugging Face **vLLM-Omni** DLC, which exposes a speech API suitable for SageMaker hosting. You can browse supported models in the [vLLM-Omni docs](https://docs.vllm.ai/projects/vllm-omni/en/latest/models/supported_models/), and other official Hugging Face DLCs on the [AWS Available Images](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-dg.pdf#available-images) page.

We'll target an `ml.g5.xlarge` instance for the core CustomVoice endpoint. If you run the optional voice-cloning section, it creates a second `ml.g5.xlarge` endpoint. For higher concurrency or larger models, consider larger GPU instance types (for example `ml.g5.2xlarge` or `ml.g6e.*`).

```python
from time import strftime

CUSTOM_VOICE_MODEL_ID = "Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice"
BASE_MODEL_ID = "Qwen/Qwen3-TTS-12Hz-1.7B-Base"

RESOURCE_SUFFIX = strftime("%Y%m%d-%H%M%S")

CUSTOM_VOICE_MODEL_NAME = f"qwen-tts-custom-voice-model-{RESOURCE_SUFFIX}"
CUSTOM_VOICE_ENDPOINT_CONFIG_NAME = f"qwen-tts-custom-voice-config-{RESOURCE_SUFFIX}"
CUSTOM_VOICE_ENDPOINT_NAME = f"qwen-tts-endpoint-{RESOURCE_SUFFIX}"

BASE_MODEL_NAME = f"qwen-tts-base-model-{RESOURCE_SUFFIX}"
BASE_ENDPOINT_CONFIG_NAME = f"qwen-tts-base-config-{RESOURCE_SUFFIX}"
BASE_ENDPOINT_NAME = f"qwen-tts-base-endpoint-{RESOURCE_SUFFIX}"
BASE_ENDPOINT_DEPLOYED = False

DLC = "huggingface-vllm-omni"
INSTANCE_TYPE = "ml.g5.xlarge"
```

In order to tell AWS which DLC we want to use, we need to specify its `image_uri`. As mentioned before, this URI can be found in the [AWS Available Images](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-dg.pdf#available-images) page. If you want to obtain it programmatically, the SageMaker Python SDK also provides a helper:

```python
>> from sagemaker.core import image_uris

>> VLLM_OMNI_IMAGE_TAG = "0.20.0-transformers5.8.1-gpu-py312-cu130-amzn2023"

>> try:
...     image_uri = image_uris.retrieve(
...         DLC,
...         region=REGION,
...         image_scope="inference",
...         instance_type=INSTANCE_TYPE,
...     )
>> except Exception as e:
...     image_uri = (
...         f"763104351884.dkr.ecr.{REGION}.amazonaws.com/"
...         f"huggingface-vllm-omni:{VLLM_OMNI_IMAGE_TAG}"
...     )
...     print(f"SageMaker SDK image lookup failed ({e}); using fallback URI.")

>> print(image_uri)
```

763104351884.dkr.ecr.us-east-1.amazonaws.com/huggingface-vllm-omni:0.20.0-transformers5.8.1-gpu-py312-cu130-amzn2023

If the helper does not yet return the image URI you need (SDK catalogs can lag behind new DLC releases), set the URI explicitly to a vLLM-Omni SageMaker image in your region. Also, if you are not sure where to find the image URI, you can always check the [AWS Available Images](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-dg.pdf#available-images) page for the latest image URIs from Hugging Face.

## Deploying the model to SageMaker

Now that we have the image URI, we can deploy the model to SageMaker. The SageMaker Python SDK v3 supports several deployment styles. In this notebook we'll use the low-level resource APIs — `Model`, `EndpointConfig`, and `Endpoint` — which give explicit control over the container environment, instance type, and optional inference AMI.

Before creating the endpoint, we'll define the environment variables used to configure the DLC and the TTS task.

```python
custom_voice_env_vars = {
    "HF_TASK": "text-to-speech",  # Task to perform
    "SM_VLLM_MODEL": CUSTOM_VOICE_MODEL_ID,  # Model ID
}
if HF_TOKEN:
    custom_voice_env_vars["HF_TOKEN"] = HF_TOKEN

base_env_vars = {
    **custom_voice_env_vars,
    "SM_VLLM_MODEL": BASE_MODEL_ID,
}
```

A few notes on those variables:

- `HF_TOKEN` authenticates downloads from the Hub when a token is available.
- `HF_TASK` tells the serving stack this is a **text-to-speech** workload.
- `SM_VLLM_MODEL` selects the model loaded by the container. We define one environment for **CustomVoice** and one for **Base** voice cloning.

Each SageMaker endpoint loads one model. The core notebook creates one CustomVoice endpoint; the optional voice-cloning section creates a second Base endpoint, which incurs its own charges while it is running.

The `inference_ami_version` on the production variant pins the host GPU driver / CUDA stack on the instance (separate from the OS inside the container image). For `ml.g5.*` instances, `al2-ami-sagemaker-inference-gpu-2-1` is a solid default; omit it if you prefer SageMaker's instance-type default.

**Cost note:** the next cell creates a billable GPU-backed SageMaker real-time endpoint. Delete it in the cleanup section when you are done.

Create the CustomVoice endpoint and wait for it to become `InService`:

```python
from sagemaker.core.resources import Endpoint, EndpointConfig, Model
from sagemaker.core.shapes import ContainerDefinition, ProductionVariant

custom_voice_model = Model.create(
    model_name=CUSTOM_VOICE_MODEL_NAME,
    primary_container=ContainerDefinition(
        image=image_uri,
        environment=custom_voice_env_vars,
    ),
    execution_role_arn=role,
    region=REGION,
)

custom_voice_endpoint_config = EndpointConfig.create(
    endpoint_config_name=CUSTOM_VOICE_ENDPOINT_CONFIG_NAME,
    production_variants=[
        ProductionVariant(
            variant_name="default",
            model_name=CUSTOM_VOICE_MODEL_NAME,
            instance_type=INSTANCE_TYPE,
            initial_instance_count=1,
            inference_ami_version="al2-ami-sagemaker-inference-gpu-2-1",
        ),
    ],
    region=REGION,
)

custom_voice_endpoint = Endpoint.create(
    endpoint_name=CUSTOM_VOICE_ENDPOINT_NAME,
    endpoint_config_name=CUSTOM_VOICE_ENDPOINT_CONFIG_NAME,
    region=REGION,
)
custom_voice_endpoint.wait_for_status("InService")
```

## Testing the endpoint

Once the endpoint is in service, we can call it with the SageMaker Runtime `invoke_endpoint` API. For TTS through this DLC, requests go to the speech route via `CustomAttributes="route=/v1/audio/speech"`.

The payload expects:

- `input`: the script / caption to narrate
- `voice`: one of the predefined CustomVoice speakers (for example `vivian`, `ryan`, `aiden`)
- `language`: target language (or let the model adapt)
- `instructions` *(optional)*: natural-language delivery guidance (tone, pacing, emotion)

The response body is raw WAV audio, which we can save and play back in the notebook.

### Routing requests through the vLLM-Omni middleware

SageMaker sends real-time endpoint requests to the container's standard `/invocations` route. The vLLM-Omni DLC includes middleware that reads SageMaker's `CustomAttributes` request property and uses it to forward the request internally to the matching vLLM-Omni API route.

For text-to-speech, set:

```python
CustomAttributes="route=/v1/audio/speech"
```

The middleware then redirects the request from SageMaker's `/invocations` entry point to vLLM-Omni's OpenAI-compatible `/v1/audio/speech` endpoint. This is why the invocation payload follows the vLLM-Omni speech API format while the request is still made through SageMaker Runtime's `invoke_endpoint` API.

```python
import json

from IPython.display import Audio

client = boto_sess.client("sagemaker-runtime")

response = client.invoke_endpoint(
    EndpointName=CUSTOM_VOICE_ENDPOINT_NAME,
    ContentType="application/json",
    Body=json.dumps({
        "model": CUSTOM_VOICE_MODEL_ID,
        "input": "Did you know how easy it is to deploy a model to Amazon SageMaker?",
        "voice": "vivian",
        "language": "English",
        "instructions": "Please speak in a slow and clear manner, like in a professional ad for a product.",
    }),
    CustomAttributes="route=/v1/audio/speech",
)

# Save and play the generated WAV audio.
with open("custom-voice-response.wav", "wb") as f:
    f.write(response["Body"].read())

Audio("custom-voice-response.wav")
```

## Optional: Voice cloning with Qwen3-TTS Base

[`Qwen/Qwen3-TTS-12Hz-1.7B-Base`](https://huggingface.co/Qwen/Qwen3-TTS-12Hz-1.7B-Base) supports zero-shot voice cloning. Instead of selecting a predefined speaker, provide a short reference audio clip and its transcript; the model uses them to synthesize new text in a similar voice.

Only use reference audio when you have the speaker's consent and the rights to use their voice. Do not use this capability to impersonate people or create deceptive content.

The Base model is deployed to its own endpoint because a SageMaker endpoint loads a single model image and model ID at a time. Skip this section if you only need predefined CustomVoice speakers.

**Cost note:** running this section creates a second billable GPU-backed endpoint.

```python
>> from sagemaker.core.resources import Endpoint, EndpointConfig, Model
>> from sagemaker.core.shapes import ContainerDefinition, ProductionVariant

>> base_model = Model.create(
...     model_name=BASE_MODEL_NAME,
...     primary_container=ContainerDefinition(
...         image=image_uri,
...         environment=base_env_vars,
...     ),
...     execution_role_arn=role,
...     region=REGION,
... )

>> base_endpoint_config = EndpointConfig.create(
...     endpoint_config_name=BASE_ENDPOINT_CONFIG_NAME,
...     production_variants=[
...         ProductionVariant(
...             variant_name="default",
...             model_name=BASE_MODEL_NAME,
...             instance_type=INSTANCE_TYPE,
...             initial_instance_count=1,
...             inference_ami_version="al2-ami-sagemaker-inference-gpu-2-1",
...         ),
...     ],
...     region=REGION,
... )

>> base_endpoint = Endpoint.create(
...     endpoint_name=BASE_ENDPOINT_NAME,
...     endpoint_config_name=BASE_ENDPOINT_CONFIG_NAME,
...     region=REGION,
... )
>> base_endpoint.wait_for_status("InService")
>> BASE_ENDPOINT_DEPLOYED = True
```

sagemaker.config INFO - Not applying SDK defaults from location: /Library/Application Support/sagemaker/config.yaml
sagemaker.config INFO - Not applying SDK defaults from location: /Users/ehcalabres/Library/Application Support/sagemaker/config.yaml

### Invoke the Voice Cloning endpoint

The speech API accepts `ref_audio` as a URL, a data URI, or a base64-encoded audio payload. The helper below converts a local audio file into a `data:` URI so it can be sent in the SageMaker request. Supplying `ref_text` is recommended because it improves voice-cloning quality.

Set `REFERENCE_AUDIO_PATH` to a local WAV, MP3, or other supported reference file and replace the transcript with the words spoken in that clip.

```python
import base64
import mimetypes
from pathlib import Path

def encode_audio_to_data_uri(audio_path):
    mime_type = mimetypes.guess_type(str(audio_path))[0] or "audio/wav"
    with open(audio_path, "rb") as f:
        encoded_audio = base64.b64encode(f.read()).decode("utf-8")
    return f"data:{mime_type};base64,{encoded_audio}"

REFERENCE_AUDIO_PATH = Path("audio_message.wav")
REFERENCE_TRANSCRIPT = "Hello, my friend Enrique, I here working while there is noise in the street and I cannot focus and I seeing people in the swimming pool right now and I'm jealous and I will have chicken for lunch today."

if not REFERENCE_AUDIO_PATH.is_file():
    raise FileNotFoundError(
        "Set REFERENCE_AUDIO_PATH to a local audio file before running this cell."
    )

response = client.invoke_endpoint(
    EndpointName=BASE_ENDPOINT_NAME,
    ContentType="application/json",
    Body=json.dumps({
        "model": BASE_MODEL_ID,
        "task_type": "Base",
        "input": "Hello, this is a cloned voice.",
        "voice": "default",
        "language": "English",
        "ref_audio": encode_audio_to_data_uri(REFERENCE_AUDIO_PATH),
        "ref_text": REFERENCE_TRANSCRIPT,
        "response_format": "wav",
    }),
    CustomAttributes="route=/v1/audio/speech",
)

with open("cloned-voice-response.wav", "wb") as f:
    f.write(response["Body"].read())

Audio("cloned-voice-response.wav")
```

Note that results may vary depending on the reference audio and transcript provided to the model. If your results are not satisfactory, record a new reference audio and try again.

## Creating a Gradio app

Now that the CustomVoice endpoint works, we can wrap it in a Gradio app for interactive use. Gradio makes it easy to prototype UIs around ML models — see the [component docs](https://gradio.app/docs/components/) for more options.

The **Ad Voiceover Studio** below always includes the predefined-speaker workflow. If you ran the optional Base deployment section, it also includes a voice-cloning workflow:

- **Predefined voice** uses the CustomVoice endpoint. Choose one of the curated speakers and guide the delivery with optional style instructions.
- **Voice cloning** uses the Base endpoint. Upload a reference recording and provide its transcript to produce narration in a similar voice.

Launching with `mcp_server=True` also exposes the available workflows to agent harnesses (covered in the optional MCP section).

```python
import base64
import json
import mimetypes
import tempfile

import gradio as gr

runtime = boto_sess.client("sagemaker-runtime")

VOICES = {
    "Vivian — bright, slightly edgy young female (Chinese)": "vivian",
    "Serena — warm, gentle young female (Chinese)": "serena",
    "Uncle Fu — seasoned male, low mellow timbre (Chinese)": "uncle_fu",
    "Dylan — youthful Beijing male (Chinese)": "dylan",
    "Eric — lively Chengdu male (Chinese)": "eric",
    "Ryan — dynamic male with strong rhythm (English)": "ryan",
    "Aiden — sunny American male (English)": "aiden",
    "Ono Anna — playful Japanese female (Japanese)": "ono_anna",
    "Sohee — warm Korean female (Korean)": "sohee",
}

LANGUAGES = [
    "Auto", "English", "Chinese", "Japanese", "Korean", "German", "French",
    "Russian", "Portuguese", "Spanish", "Italian",
]

def encode_audio_to_data_uri(audio_path: str) -> str:
    """Encode a local audio file as a data URI for the speech API."""
    mime_type = mimetypes.guess_type(str(audio_path))[0] or "audio/wav"
    with open(audio_path, "rb") as f:
        encoded_audio = base64.b64encode(f.read()).decode("utf-8")
    return f"data:{mime_type};base64,{encoded_audio}"

def invoke_tts(endpoint_name: str, payload: dict[str, object]) -> str:
    """Invoke a SageMaker TTS endpoint and return the generated WAV file path."""
    response = runtime.invoke_endpoint(
        EndpointName=endpoint_name,
        ContentType="application/json",
        Body=json.dumps(payload),
        CustomAttributes="route=/v1/audio/speech",
    )
    with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as f:
        f.write(response["Body"].read())
        return f.name

def generate_predefined_voice(
    script: str,
    voice_label: str,
    language: str,
    instructions: str | None,
) -> str:
    """Generate a voiceover with a predefined Qwen3-TTS CustomVoice speaker.

    Args:
        script: The ad script or caption to synthesize.
        voice_label: The selected speaker label from the voice dropdown.
        language: Target language, or Auto for model-adaptive language selection.
        instructions: Optional tone, pacing, or emotion guidance.

    Returns:
        Path to the generated WAV audio file.
    """
    if not script or not script.strip():
        raise gr.Error("Please enter an ad script or caption to narrate.")

    payload = {
        "model": CUSTOM_VOICE_MODEL_ID,
        "input": script.strip(),
        "voice": VOICES[voice_label],
        "language": language,
        "response_format": "wav",
    }
    if instructions and instructions.strip():
        payload["instructions"] = instructions.strip()

    return invoke_tts(CUSTOM_VOICE_ENDPOINT_NAME, payload)

def generate_cloned_voice(
    script: str,
    language: str,
    reference_audio: str | None,
    reference_transcript: str | None,
    x_vector_only: bool,
) -> str:
    """Generate a voiceover using authorized reference audio for voice cloning.

    Args:
        script: The ad script or caption to synthesize.
        language: Target language, or Auto for model-adaptive language selection.
        reference_audio: Local path to the uploaded reference recording.
        reference_transcript: Transcript of the reference recording.
        x_vector_only: Whether to clone from speaker embedding only.

    Returns:
        Path to the generated WAV audio file.
    """
    if not script or not script.strip():
        raise gr.Error("Please enter an ad script or caption to narrate.")
    if not reference_audio:
        raise gr.Error("Upload a reference audio clip to clone its voice.")
    if not x_vector_only and not (reference_transcript and reference_transcript.strip()):
        raise gr.Error("Provide the reference transcript, or enable x-vector-only mode.")

    payload = {
        "model": BASE_MODEL_ID,
        "task_type": "Base",
        "input": script.strip(),
        "voice": "default",
        "language": language,
        "ref_audio": encode_audio_to_data_uri(reference_audio),
        "response_format": "wav",
    }
    if reference_transcript and reference_transcript.strip():
        payload["ref_text"] = reference_transcript.strip()
    if x_vector_only:
        payload["x_vector_only_mode"] = True

    return invoke_tts(BASE_ENDPOINT_NAME, payload)

with gr.Blocks(title="Ad Voiceover Studio", theme=gr.themes.Soft()) as demo:
    gr.Markdown(
        """
        # Ad Voiceover Studio
        Generate ad voiceovers with **Qwen3-TTS** on SageMaker.
        Choose a curated speaker, or enable the optional cloning endpoint for authorized reference voices.
        """
    )

    with gr.Tab("Predefined voice"):
        gr.Markdown("Choose a CustomVoice speaker and optionally set the ad's tone, pacing, or emotion.")
        with gr.Row():
            with gr.Column(scale=2):
                custom_script = gr.Textbox(
                    label="Ad script / caption",
                    placeholder="Write the line your voiceover should read over the video...",
                    lines=6,
                )
                with gr.Row():
                    custom_voice = gr.Dropdown(
                        label="Voice",
                        choices=list(VOICES.keys()),
                        value="Ryan — dynamic male with strong rhythm (English)",
                    )
                    custom_language = gr.Dropdown(
                        label="Language", choices=LANGUAGES, value="English"
                    )
                custom_instructions = gr.Textbox(
                    label="Delivery instructions (optional)",
                    placeholder="e.g. Speak slowly and clearly, like a premium product ad.",
                    lines=3,
                )
                custom_generate_btn = gr.Button("Generate voiceover", variant="primary", size="lg")
            with gr.Column(scale=1):
                custom_audio = gr.Audio(label="Generated voiceover", type="filepath")
                gr.Markdown(
                    """**Tips for ads**
                    - Keep scripts short (1-3 sentences) for social clips
                    - Match voice + language to your audience
                    - Use instructions for brand tone: urgent, calm, luxury, or playful
                    """
                )

        gr.Examples(
            examples=[
                [
                    "Introducing the all-new summer collection. Soft fabrics, bold colors, and free shipping on every order.",
                    "Aiden — sunny American male (English)",
                    "English",
                    "Warm and inviting lifestyle-ad tone. Slight smile in the voice, unhurried but engaging.",
                ],
            ],
            inputs=[custom_script, custom_voice, custom_language, custom_instructions],
            label="Example ad script",
        )
        custom_generate_btn.click(
            fn=generate_predefined_voice,
            inputs=[custom_script, custom_voice, custom_language, custom_instructions],
            outputs=custom_audio,
        )

    if BASE_ENDPOINT_DEPLOYED:
        with gr.Tab("Voice cloning"):
            gr.Markdown(
                """
                Upload a reference recording and its transcript to synthesize your script in a similar voice.
                **Only upload audio you are authorized to use.**
                """
            )
            with gr.Row():
                with gr.Column(scale=2):
                    clone_script = gr.Textbox(
                        label="Ad script / caption",
                        placeholder="Write the line the cloned voice should read...",
                        lines=6,
                    )
                    clone_language = gr.Dropdown(
                        label="Language", choices=LANGUAGES, value="English"
                    )
                    reference_audio = gr.Audio(
                        label="Reference audio",
                        type="filepath",
                        sources=["upload", "microphone"],
                    )
                    reference_transcript = gr.Textbox(
                        label="Reference audio transcript",
                        placeholder="Enter the words spoken in the reference clip...",
                        lines=3,
                    )
                    x_vector_only = gr.Checkbox(
                        label="Use x-vector-only mode",
                        info="Use only the speaker embedding; leave off for transcript-guided cloning.",
                    )
                    clone_generate_btn = gr.Button("Generate cloned voiceover", variant="primary", size="lg")
                with gr.Column(scale=1):
                    clone_audio = gr.Audio(label="Generated cloned voiceover", type="filepath")
                    gr.Markdown(
                        """**Reference recording tips**
                        - Use a clear, single-speaker clip
                        - Provide an accurate transcript
                        - Avoid music, background noise, and overlapping speech
                        """
                    )

            clone_generate_btn.click(
                fn=generate_cloned_voice,
                inputs=[
                    clone_script,
                    clone_language,
                    reference_audio,
                    reference_transcript,
                    x_vector_only,
                ],
                outputs=clone_audio,
            )
    else:
        with gr.Tab("Voice cloning (optional)"):
            gr.Markdown(
                "Run the optional Qwen3-TTS Base deployment section before using voice cloning."
            )

ENABLE_MCP_SERVER = True

demo.launch(
    share=True,
    server_name="0.0.0.0",
    server_port=7860,
    show_error=True,
    mcp_server=ENABLE_MCP_SERVER,
)
```

Here's a screenshot of the Gradio app in action:

![Gradio app in action](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/sagemaker/notebooks/sagemaker-sdk/tts-qwen-sagemaker-vllm-omni/gradio-app-screenshot.png)

## Optional: Using the Gradio app as an MCP server

If `ENABLE_MCP_SERVER=True` when the Gradio app launches, it also exposes an MCP endpoint that agent harnesses can call. That means tools like Cursor, Claude Desktop, or any other MCP-compatible client can generate predefined-speaker voiceovers through the same Gradio app — without going through the UI. If you ran the optional Base deployment section, the voice-cloning workflow is exposed too.

Add the following configuration to your MCP settings, replacing the URL with the Gradio server address printed when the app launches (for a shared link, use the `.../gradio_api/mcp/` URL shown in the launch logs):

```json
{
  "mcpServers": {
    "gradio": {
      "url": "http://your-server:port/gradio_api/mcp/"
    }
  }
}
```

Once connected, the agent can use the app's tools as part of a larger workflow — for example, drafting ad copy, proposing delivery variants, generating a voiceover with a curated speaker, or producing a clone from an authorized reference recording.

Here's a screenshot of a Cursor agent using the Gradio app as an MCP server, but the same would apply to any other MCP-compatible client.

![Cursor agent output](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/sagemaker/notebooks/sagemaker-sdk/tts-qwen-sagemaker-vllm-omni/cursor-agent-output.png)

## Cleanup

To stop both endpoints and avoid ongoing charges once you're done, delete the endpoints, endpoint configurations, and model resources:

```python
custom_voice_endpoint.delete()
custom_voice_endpoint_config.delete()
custom_voice_model.delete()

if BASE_ENDPOINT_DEPLOYED:
    base_endpoint.delete()
    base_endpoint_config.delete()
    base_model.delete()
```

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/hub-docs/tree/main/docs/sagemaker/notebooks/sagemaker-sdk/tts-qwen-sagemaker-vllm-omni/sagemaker-notebook.ipynb)!
