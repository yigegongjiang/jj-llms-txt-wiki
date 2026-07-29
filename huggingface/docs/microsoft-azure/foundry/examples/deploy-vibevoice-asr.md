# Deploy VibeVoice ASR on Microsoft Foundry: Long-Form Transcription, +50 Languages Supported & Speaker Diarization

Written by Alvaro BartolomeLast updated 2026-07-01

![](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-vibevoice-asr/thumbnail.png)

In this example, you will deploy [`microsoft/VibeVoice-ASR-HF`](https://huggingface.co/microsoft/VibeVoice-ASR-HF) on Microsoft Foundry and then use it to transcribe audio across its key capabilities: standard inference, streaming, multilingual support, long-form audio, and multi-speaker diarization.

**VibeVoice-ASR** is a unified speech-to-text model designed to handle **60-minute long-form audio** in a single pass, generating structured transcriptions containing **Who (Speaker), When (Timestamps), and What (Content)**, with support for **Customized Hotwords** and over **50 languages**.

![VibeVoice-ASR Architecture](https://huggingface.co/microsoft/VibeVoice-ASR-HF/resolve/main/figures/VibeVoice_ASR_archi.png)

- **🕒 60-minute Single-Pass Processing**: Unlike conventional ASR models that slice audio into short chunks (often losing global context), VibeVoice ASR accepts up to **60 minutes** of continuous audio input within a 64K input token length, producing structured transcriptions of up to 32K output tokens. This ensures consistent speaker tracking and semantic coherence across the entire hour.

- **👤 Customized Hotwords**: Users can provide customized hotwords (e.g., specific names, technical terms, or background info) to guide the recognition process, significantly improving accuracy on domain-specific content.

- **📝 Rich Transcription (Who, When, What)**: The model jointly performs ASR, diarization, and timestamping, producing a structured output that indicates *who* said *what* and *when*.

- **🌍 Multilingual & Code-Switching Support**: It supports over 50 languages, requires no explicit language setting, and natively handles code-switching within and across utterances.

For more information, make sure to check [their model card on the Hugging Face Hub](https://huggingface.co/microsoft/VibeVoice-ASR-HF).

## Requirements

To run the following example, you will need to meet the following prerequisites. You can also read more about them in the [Azure Machine Learning Tutorial: Create resources you need to get started](https://learn.microsoft.com/en-us/azure/machine-learning/quickstart-create-resources?view=azureml-api-2).

- You have a Microsoft Azure subscription and are logged in
- You have the `az` CLI installed
- You have the necessary permissions to:
    - Create an Azure Machine Learning Managed Online Endpoint
    - Create an Azure Machine Learning Deployment
- You have Python 3.10+ installed locally and `pip`

For more information, please go through the steps in the guide ["Configure Azure Machine Learning and Microsoft Foundry"](https://huggingface.co/docs/microsoft-azure/guides/configure-azure-ml-microsoft-foundry).

## Setup

### Set environment variables

For convenience, you can set the following environment variables to be used throughout the example:

```python
%env LOCATION eastus
%env SUBSCRIPTION_ID <YOUR_SUBSCRIPTION_ID>
%env RESOURCE_GROUP <YOUR_RESOURCE_GROUP>
%env WORKSPACE_NAME <YOUR_WORKSPACE_NAME>
%env MODEL_ID microsoft/VibeVoice-ASR-HF
```

```python
import os
from uuid import uuid4

os.environ["ENDPOINT_NAME"] = f"vibevoice-asr-{str(uuid4())[:8]}"
os.environ["DEPLOYMENT_NAME"] = f"vibevoice-asr-{str(uuid4())[:8]}"
```

### Install Azure Python SDK (+ dependencies)

You need to install some Azure Python SDK dependencies:

* `azure-identity` to use the `DefaultAzureCredential` authentication with your Managed Identity
* `azure-ai-ml` to create the Azure Machine Learning Managed Online Endpoint + Deployment, and to invoke it

```python
%pip install azure-ai-ml azure-identity --upgrade --quiet
```

### Authenticate to Azure Machine Learning

You can then authenticate to Azure Machine Learning with your Managed Identity with Python as:

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

## Create endpoint + deployment

### Create endpoint

Now you need to create the [ManagedOnlineEndpoint via the Azure Machine Learning Python SDK](https://learn.microsoft.com/en-us/python/api/azure-ai-ml/azure.ai.ml.entities.managedonlineendpoint?view=azure-python) as follows:

Every model in the Hugging Face collection is powered by an efficient inference backend, and each of those can run on a wide variety of instance types (as listed in [Supported Hardware](https://huggingface.co/docs/microsoft-azure/azure-ai/supported-hardware)). Since models and inference engines require a GPU-accelerated instance, you might need to request a quota increase as per [Manage and increase quotas and limits for resources with Azure Machine Learning](https://learn.microsoft.com/en-us/azure/machine-learning/how-to-manage-quotas?view=azureml-api-2).

```python
from azure.ai.ml.entities import ManagedOnlineEndpoint

endpoint = ManagedOnlineEndpoint(name=os.getenv("ENDPOINT_NAME"))
client.begin_create_or_update(endpoint).wait()
```

### Create deployment

Once the endpoint is created, you need to create the deployment indicating which model, hardware, and settings to use. In this case, we will be using a single NVIDIA H100, but given that the model just has 7B parameters it's supported on instances with lower memory, and we'll be increasing the default timeout per request to 180s to ensure that we can process long audio files in time.

```python
from azure.ai.ml.entities import ManagedOnlineDeployment, OnlineRequestSettings

deployment = ManagedOnlineDeployment(
    name=os.getenv("DEPLOYMENT_NAME"),
    endpoint_name=os.getenv("ENDPOINT_NAME"),
    model=f"azureml://registries/HuggingFace/models/{os.getenv('MODEL_ID').replace('/', '-').replace('_', '-').lower()}/labels/latest",
    instance_type="Standard_NC40ads_H100_v5",
    instance_count=1,
    request_settings=OnlineRequestSettings(request_timeout_ms=180000),
)
client.online_deployments.begin_create_or_update(deployment).result()
```

![Azure AI Deployment from Azure AI Foundry](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/microsoft-azure/foundry/deploy-vibevoice-asr/azure-ai-deployment.png)

The deployment might take ~10-15 minutes, but it could also take longer depending on the selected SKU availability in the region.

## Run inference on the Foundry Endpoint

With the Foundry Endpoint running, you can start transcribing audio. `microsoft/VibeVoice-ASR-HF` is served through an **OpenAI-compatible Chat Completions API** at `/v1/chat/completions`, accepting multimodal user messages that pair a `text` part, for optional hotwords or context that guide the transcription, with an `input_audio` part supplying the audio either as a publicly accessible URL or as base64-encoded bytes. The structured output it produces is a JSON array where every element represents one speech segment and carries four fields: `Start`, `End`, `Speaker`, and `Content`.

Throughout the examples below, the raw model output includes an `assistant` prefix and occasional control tokens (e.g., `<|...|>`) that need to be stripped before parsing the JSON. Each code snippet handles this clean-up step before calling `json.loads`.

### Set up the OpenAI Python SDK

```python
%pip install openai --upgrade --quiet
```

Retrieve the endpoint URL and API key, then create the OpenAI client, making sure to include the `azureml-model-deployment` header so every request is routed to the right deployment. Setting it once via `default_headers` is the recommended approach since the header needs to accompany each request.

```python
from urllib.parse import urlsplit
from openai import OpenAI
import os

api_key = client.online_endpoints.get_keys(os.getenv("ENDPOINT_NAME")).primary_key
url_parts = urlsplit(client.online_endpoints.get(os.getenv("ENDPOINT_NAME")).scoring_uri)
api_url = f"{url_parts.scheme}://{url_parts.netloc}/v1"

openai_client = OpenAI(
    base_url=api_url,
    api_key=api_key,
    default_headers={"azureml-model-deployment": os.getenv("DEPLOYMENT_NAME")},
)
```

Alternatively, you can also build the API URL manually as follows, since the URIs are globally unique per region, meaning that there will only be one endpoint named the same way within the same region:
```python
api_url = f"https://{os.getenv('ENDPOINT_NAME')}.{os.getenv('LOCATION')}.inference.ml.azure.com/v1"
```
Or just retrieve it from either Microsoft Foundry or the Azure Machine Learning Studio.

### Standard inference

The `messages` list contains a single `user` message with two content parts: a `text` entry for optional hotwords or context (pass an empty string if none are needed), and an `input_audio` entry with the audio URL or base64-encoded bytes together with its format. VibeVoice ASR returns a JSON-serializable string with one object per detected segment:

```python
response = openai_client.chat.completions.create(
    model=os.getenv("MODEL_ID"),
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "",
                },
                {
                    "type": "input_audio",
                    "input_audio": {
                        "data": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
                        "format": "wav",
                    },
                },
            ],
        }
    ],
)
print(response.choices[0].message.content)
```

The raw output contains an `assistant` prefix and a trailing newline. A small post-processing step cleans that up and parses the segments into a list of dicts:

```python
import json

raw = response.choices[0].message.content.strip().replace("assistant", "").strip()
segments = json.loads(raw)

for segment in segments:
    print(f"[{segment['Start']:.2f}s -> {segment['End']:.2f}s] Speaker {segment.get('Speaker')}: {segment['Content']}")
    # [0.00s -> 7.56s] Speaker 0: Revevoices is a novel framework designed for generating expressive, long-form, multi-speaker conversational audio.
```

Thanks to the context, we can help the model correctly transcribe words that would otherwise be transcribed incorrectly. For example, the snippet above will fail to correctly transcribe the word "VibeVoice" and will instead produce "Revevoices", partly due to the speaker's German accent. Both issues can be mitigated by providing context:

```python
response = openai_client.chat.completions.create(
    model=os.getenv("MODEL_ID"),
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "About VibeVoice",
                },
                {
                    "type": "input_audio",
                    "input_audio": {
                        "data": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
                        "format": "wav",
                    },
                },
            ],
        }
    ],
)

raw = response.choices[0].message.content.strip().replace("assistant", "").strip()
segments = json.loads(raw)

for segment in segments:
    print(f"[{segment['Start']:.2f}s -> {segment['End']:.2f}s] Speaker {segment.get('Speaker')}: {segment['Content']}")
    # [0.00s -> 7.56s] Speaker 0: VibeVoice is this novel framework designed for generating expressive, long-form, multi-speaker, conversational audio.
```

### Streaming

Streaming is supported out of the box via `stream=True`. Each chunk delivers a slice of the transcription as the model generates it, which is useful for progressive display or real-time pipelines. The model emits a few internal control tokens that need to be stripped before parsing:

```python
import json
import re

stream = openai_client.chat.completions.create(
    model=os.getenv("MODEL_ID"),
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "About VibeVoice",
                },
                {
                    "type": "input_audio",
                    "input_audio": {
                        "data": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/realtime_model/vibevoice_tts_german.wav",
                        "format": "wav",
                    },
                },
            ],
        }
    ],
    stream=True,
)

raw = ""
for chunk in stream:
    if chunk.choices and chunk.choices[0].delta.content:
        raw += chunk.choices[0].delta.content

# Strip model-specific control tokens and the assistant prefix
raw = re.sub(r"<\|[^|]+\|>", "", raw).replace("assistant", "").strip()
segments = json.loads(raw)

for segment in segments:
    print(f"[{segment['Start']:.2f}s -> {segment['End']:.2f}s] Speaker {segment.get('Speaker')}: {segment['Content']}")
    # [0.00s -> 7.56s] Speaker 0: VibeVoice is this novel framework designed for generating expressive, long-form, multi-speaker, conversational audio.
```

### Multilingual support

VibeVoice ASR supports more than **50 languages** with **automatic language detection**, no explicit language parameter or pre-processing is required. The same API call works regardless of the audio language, and hotwords are equally effective across languages. Here the model receives audio files in different languages sourced from the [`facebook/multilingual_librispeech`](https://huggingface.co/datasets/facebook/multilingual_librispeech) dataset, which itself is built on [LibriVox](https://librivox.org) recordings hosted on the Internet Archive, and transcribes them without any language hint:

```python
audios = {
    "Spanish": "https://dn710608.ca.archive.org/0/items/isaias_1603_librivox/isaias_29_reinavalera_64kb.mp3",
    "French": "http://www.archive.org/download/les1001nuits_tome1_0711_librivox/1001nuits1_019_galland_64kb.mp3",
    "Dutch": "http://www.archive.org/download/de_reis_naar_de_maan_1511_librivox/reisnaardemaan_21_verne_64kb.mp3",
    "Portuguese": "http://www.archive.org/download/canaa_1901_librivox/canaa_13_aranha_64kb.mp3",
    "Italian": "http://www.archive.org/download/glinni_sacri_1701_librivox/innisacri_5_manzoni_64kb.mp3",
}

for key, value in audios.items():
    response = openai_client.chat.completions.create(
        model=os.getenv("MODEL_ID"),
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "",
                    },
                    {
                        "type": "input_audio",
                        "input_audio": {
                            "data": value,
                            "format": "mp3",
                        },
                    },
                ],
            }
        ],
        max_tokens=32768,
    )
    
    raw = response.choices[0].message.content.strip().replace("assistant", "").strip()
    segments = json.loads(raw)

    print(f"Audio transcription in {key} for {value}")
    for segment in segments:
        print(f"[{segment['Start']:.2f}s -> {segment['End']:.2f}s] Speaker {segment.get('Speaker')}: {segment['Content']}")

    # e.g.,
    # Audio transcription in Spanish for https://dn710608.ca.archive.org/0/items/isaias_1603_librivox/isaias_29_reinavalera_64kb.mp3
    # [0.00s -> 37.62s] Speaker [0]: Capítulo 57 al 58 del libro de Isaías. Versión de la Biblia Reina Valera Antigua. Esta es una grabación de LibriVox. Todas las grabaciones de LibriVox están en el dominio público. Para más información o para ser voluntario, por favor visite LibriVox.org. Grabado por Rodrigo Inojosa. Isaías 57. Pero es el justo y no hay quien pare mientes, y los píos son recogidos, y no hay quien entienda que delante de la aflicción es recogido el justo. Entrará en la paz, descansarán en sus lechos todos los que andan delante de Dios.
    # ...
    # [372.95s -> 376.95s] Speaker [None]: [Environmental Sounds]
```

### Long-form audio

One of VibeVoice ASR's standout capabilities is accepting up to **60 minutes of audio in a single pass**, maintaining coherent speaker tracking and semantic context throughout, no chunking or stitching required. The example below uses a public domain audiobook recording from [LibriVox](https://librivox.org) hosted on the Internet Archive. The deployment was already configured with a 180-second server-side timeout; for very long recordings you may also want to set `timeout=None` on the client call to remove the client-side limit entirely, and increase `max_tokens` from its default of 256 to 32768, which is the maximum model output length.

Given that the maximum timeout on Azure Machine Learning can only be increased up to 180s per request, that means that despite the model being capable of producing up to 32768 output tokens for audios up to 60 minutes, the request will likely fail if it takes over 180s.

```
APIStatusError: upstream request timeout
Please check this guide to understand why this error code might have been returned 
https://docs.microsoft.com/en-us/azure/machine-learning/how-to-troubleshoot-online-endpoints#http-status-codes
```

```python
response = openai_client.chat.completions.create(
    model=os.getenv("MODEL_ID"),
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "An open audio-book recording of a chapter of 'Christmas Carol' by Charles Dickens",
                },
                {
                    "type": "input_audio",
                    "input_audio": {
                        "data": "https://dn720004.ca.archive.org/0/items/A_Christmas_Carol/A_Christmas_Carol_Stave_5_Dickens.mp3",
                        "format": "mp3",
                    },
                },
            ],
        }
    ],
    max_tokens=32768,
    timeout=None,
)

raw = response.choices[0].message.content.strip().replace("assistant", "").strip()
segments = json.loads(raw)

print(f"Transcribed {len(segments)} segments spanning {segments[-1]['End']:.1f}s")
for segment in segments:
    print(f"[{segment['Start']:.2f}s -> {segment['End']:.2f}s] Speaker {segment.get('Speaker')}: {segment['Content']}")
```

### Multi-speaker diarization

Every segment includes a `Speaker` field with a numeric identifier that lets you track individual voices across the entire recording. Speaker diarization is resolved jointly with the transcription in a single pass, with no separate step needed. The example below transcribes a multi-speaker conversation and groups the output by speaker:

```python
from collections import defaultdict

response = openai_client.chat.completions.create(
    model=os.getenv("MODEL_ID"),
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "A podcast about VibeVoice generated by VibeVoice",
                },
                {
                    "type": "input_audio",
                    "input_audio": {
                        "data": "https://huggingface.co/datasets/bezzam/vibevoice_samples/resolve/main/example_output/VibeVoice-1.5B_output.wav",
                        "format": "wav",
                    },
                },
            ],
        }
    ],
)

raw = response.choices[0].message.content.strip().replace("assistant", "").strip()
segments = json.loads(raw)

turns = defaultdict(list)
for segment in segments:
    turns[segment["Speaker"]].append(f"[{segment['Start']:.2f}s] {segment['Content']}")

for speaker_id, utterances in sorted(turns.items()):
    print(f"\nSpeaker {speaker_id}:")
    for utterance in utterances:
        print(f"  {utterance}")
```

## Release resources

Once you are done using the Foundry Endpoint, you can delete the resources (i.e., you will stop paying for the instance on which the model is running and all the attached costs) as follows:

```python
client.online_endpoints.begin_delete(name=os.getenv("ENDPOINT_NAME")).result()
```

## Conclusion

Throughout this example, you learned how to deploy `microsoft/VibeVoice-ASR-HF` as an Azure Machine Learning Managed Online Endpoint on Microsoft Foundry, and how to exercise its key inference capabilities: streaming, automatic multilingual transcription, long-form single-pass audio processing, and multi-speaker diarization; all through the OpenAI-compatible Chat Completions API.

If you have any doubt, issue or question about this example, feel free to [open an issue](https://github.com/huggingface/Microsoft-Azure/issues/new) and we'll do our best to help!

---

📍 Find the complete example on GitHub [here](https://github.com/huggingface/Microsoft-Azure/tree/main/examples/foundry/deploy-vibevoice-asr/azure-notebook.ipynb)!
