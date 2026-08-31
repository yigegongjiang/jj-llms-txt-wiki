# Inference

Inference is the process of using a trained model to make predictions on new data. Because this process can be compute-intensive, running on a dedicated or external service can be an interesting option.
The `huggingface_hub` library provides a unified interface to run inference across multiple services for models hosted on the Hugging Face Hub:

1.  [Inference Providers](https://huggingface.co/docs/inference-providers/index): a streamlined, unified access to hundreds of machine learning models, powered by our serverless inference partners. This new approach builds on our previous Serverless Inference API, offering more models, improved performance, and greater reliability thanks to world-class providers. Refer to the [documentation](https://huggingface.co/docs/inference-providers/index#partners) for a list of supported providers.
2.  [Inference Endpoints](https://huggingface.co/docs/inference-endpoints/index): a product to easily deploy models to production. Inference is run by Hugging Face in a dedicated, fully managed infrastructure on a cloud provider of your choice.
3.  Local endpoints: you can also run inference with local inference servers like [llama.cpp](https://github.com/ggerganov/llama.cpp), [Ollama](https://ollama.com/), [vLLM](https://github.com/vllm-project/vllm), [LiteLLM](https://docs.litellm.ai/docs/simple_proxy), or [Text Generation Inference (TGI)](https://github.com/huggingface/text-generation-inference) by connecting the client to these local endpoints.

These services can be called with the [InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient) object. Please refer to [this guide](../guides/inference)
for more information on how to use it.

## Inference Client[[huggingface_hub.InferenceClient]]

#### huggingface_hub.InferenceClient[[huggingface_hub.InferenceClient]]

```python
huggingface_hub.InferenceClient(model: str | None = None, provider: typing.Union[typing.Literal['baseten', 'cerebras', 'cohere', 'deepinfra', 'fal-ai', 'featherless-ai', 'fireworks-ai', 'groq', 'hf-inference', 'novita', 'nscale', 'openai', 'ovhcloud', 'publicai', 'replicate', 'scaleway', 'together', 'wavespeed', 'zai-org'], typing.Literal['auto'], NoneType] = None, token: str | None = None, timeout: float | None = None, headers: dict[str, str] | None = None, cookies: dict[str, str] | None = None, bill_to: str | None = None, base_url: str | None = None, api_key: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L123)

**Parameters:**

model (`str`, `optional`) : The model to run inference with. Can be a model id hosted on the Hugging Face Hub, e.g. `meta-llama/Meta-Llama-3-8B-Instruct` or a URL to a deployed Inference Endpoint. Defaults to None, in which case a recommended model is automatically selected for the task. Note: for better compatibility with OpenAI's client, `model` has been aliased as `base_url`. Those 2 arguments are mutually exclusive. If a URL is passed as `model` or `base_url` for chat completion, the `(/v1)/chat/completions` suffix path will be appended to the URL.

provider (`str`, *optional*) : Name of the provider to use for inference. Can be `"baseten"`, `"cerebras"`, `"cohere"`, `"deepinfra"`, `"fal-ai"`, `"featherless-ai"`, `"fireworks-ai"`, `"groq"`, `"hf-inference"`, `"novita"`, `"nscale"`, `"openai"`, `"ovhcloud"`, `"publicai"`, `"replicate"`, `"scaleway"`, `"together"`, `"wavespeed"` or `"zai-org"`. Defaults to "auto": automatic routing, which defaults to "fastest" provider; you can switch to "cheapest" or "preferred" provider order at https://hf.co/settings/inference-providers. If model is a URL or `base_url` is passed, then `provider` is not used.

token (`str`, *optional*) : Hugging Face token. Will default to the locally saved token if not provided. Note: for better compatibility with OpenAI's client, `token` has been aliased as `api_key`. Those 2 arguments are mutually exclusive and have the exact same behavior.

timeout (`float`, `optional`) : The maximum number of seconds to wait for a response from the server. Defaults to None, meaning it will loop until the server is available.

headers (`dict[str, str]`, `optional`) : Additional headers to send to the server. By default only the authorization and user-agent headers are sent. Values in this dictionary will override the default values.

bill_to (`str`, `optional`) : The billing account to use for the requests. By default the requests are billed on the user's account. Requests can only be billed to an organization the user is a member of, and which has subscribed to Enterprise Hub.

cookies (`dict[str, str]`, `optional`) : Additional cookies to send to the server.

base_url (`str`, `optional`) : Base URL to run inference. This is a duplicated argument from `model` to make [InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient) follow the same pattern as `openai.OpenAI` client. Cannot be used if `model` is set. Defaults to None.

api_key (`str`, `optional`) : Token to use for authentication. This is a duplicated argument from `token` to make [InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient) follow the same pattern as `openai.OpenAI` client. Cannot be used if `token` is set. Defaults to None.

Initialize a new Inference Client.

[InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient) aims to provide a unified experience to perform inference. The client can be used
seamlessly with either the (free) Inference API, self-hosted Inference Endpoints, or third-party Inference Providers.

#### audio_classification[[huggingface_hub.InferenceClient.audio_classification]]

```python
audio_classification(audio: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, top_k: int | None = None, function_to_apply: typing.Optional[ForwardRef('AudioClassificationOutputTransform')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L307)

**Parameters:**

audio (Union[str, Path, bytes, BinaryIO]) : The audio content to classify. It can be raw audio bytes, a local audio file, or a URL pointing to an audio file.

model (`str`, *optional*) : The model to use for audio classification. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for audio classification will be used.

top_k (`int`, *optional*) : When specified, limits the output to the top K most probable classes.

function_to_apply (`"AudioClassificationOutputTransform"`, *optional*) : The function to apply to the model outputs in order to retrieve the scores.

**Returns:** `list[AudioClassificationOutputElement]`

List of [AudioClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.AudioClassificationOutputElement) items containing the predicted labels and their confidence.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform audio classification on the provided audio content.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.audio_classification("audio.flac")
[
    AudioClassificationOutputElement(score=0.4976358711719513, label='hap'),
    AudioClassificationOutputElement(score=0.3677836060523987, label='neu'),
    ...
]
```

#### audio_to_audio[[huggingface_hub.InferenceClient.audio_to_audio]]

```python
audio_to_audio(audio: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L364)

**Parameters:**

audio (Union[str, Path, bytes, BinaryIO]) : The audio content for the model. It can be raw audio bytes, a local audio file, or a URL pointing to an audio file.

model (`str`, *optional*) : The model can be any model which takes an audio file and returns another audio file. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for audio_to_audio will be used.

**Returns:** `list[AudioToAudioOutputElement]`

A list of [AudioToAudioOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.AudioToAudioOutputElement) items containing audios label, content-type, and audio content in blob.

**Raises:** ``InferenceTimeoutError`` or `HfHubHTTPError`

- ``InferenceTimeoutError`` -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Performs multiple tasks related to audio-to-audio depending on the model (eg: speech enhancement, source separation).

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> audio_output = client.audio_to_audio("audio.flac")
>>> for i, item in enumerate(audio_output):
>>>     with open(f"output_{i}.flac", "wb") as f:
            f.write(item.blob)
```

#### automatic_speech_recognition[[huggingface_hub.InferenceClient.automatic_speech_recognition]]

```python
automatic_speech_recognition(audio: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, extra_body: dict | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L416)

**Parameters:**

audio (Union[str, Path, bytes, BinaryIO]) : The content to transcribe. It can be raw audio bytes, local audio file, or a URL to an audio file.

model (`str`, *optional*) : The model to use for ASR. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for ASR will be used.

extra_body (`dict`, *optional*) : Additional provider-specific parameters to pass to the model. Refer to the provider's documentation for supported parameters.

**Returns:** [AutomaticSpeechRecognitionOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.AutomaticSpeechRecognitionOutput)

An item containing the transcribed text and optionally the timestamp chunks.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform automatic speech recognition (ASR or audio-to-text) on the given audio content.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.automatic_speech_recognition("hello_world.flac").text
"hello world"
```

#### chat_completion[[huggingface_hub.InferenceClient.chat_completion]]

```python
chat_completion(messages: list, model: str | None = None, stream: bool = False, frequency_penalty: float | None = None, logit_bias: list[float] | None = None, logprobs: bool | None = None, max_tokens: int | None = None, n: int | None = None, presence_penalty: float | None = None, response_format: typing.Union[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputResponseFormatText, huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputResponseFormatJSONSchema, huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputResponseFormatJSONObject, NoneType] = None, seed: int | None = None, stop: list[str] | None = None, stream_options: huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputStreamOptions | None = None, temperature: float | None = None, tool_choice: typing.Union[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputToolChoiceClass, ForwardRef('ChatCompletionInputToolChoiceEnum'), NoneType] = None, tool_prompt: str | None = None, tools: list[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputTool] | None = None, top_logprobs: int | None = None, top_p: float | None = None, extra_body: dict | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L543)

**Parameters:**

messages (List of [ChatCompletionInputMessage](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionInputMessage)) : Conversation history consisting of roles and content pairs.

model (`str`, *optional*) : The model to use for chat-completion. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for chat-based text-generation will be used. See https://huggingface.co/tasks/text-generation for more details. If `model` is a model ID, it is passed to the server as the `model` parameter. If you want to define a custom URL while setting `model` in the request payload, you must set `base_url` when initializing [InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient).

frequency_penalty (`float`, *optional*) : Penalizes new tokens based on their existing frequency in the text so far. Range: [-2.0, 2.0]. Defaults to 0.0.

logit_bias (`list[float]`, *optional*) : Adjusts the likelihood of specific tokens appearing in the generated output.

logprobs (`bool`, *optional*) : Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the content of message.

max_tokens (`int`, *optional*) : Maximum number of tokens allowed in the response. Defaults to 100.

n (`int`, *optional*) : The number of completions to generate for each prompt.

presence_penalty (`float`, *optional*) : Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.

response_format (`ChatCompletionInputGrammarType()`, *optional*) : Grammar constraints. Can be either a JSONSchema or a regex.

seed (Optional`int`, *optional*) : Seed for reproducible control flow. Defaults to None.

stop (`list[str]`, *optional*) : Up to four strings which trigger the end of the response. Defaults to None.

stream (`bool`, *optional*) : Enable realtime streaming of responses. Defaults to False.

stream_options ([ChatCompletionInputStreamOptions](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionInputStreamOptions), *optional*) : Options for streaming completions.

temperature (`float`, *optional*) : Controls randomness of the generations. Lower values ensure less random completions. Range: [0, 2]. Defaults to 1.0.

top_logprobs (`int`, *optional*) : An integer between 0 and 5 specifying the number of most likely tokens to return at each token position, each with an associated log probability. logprobs must be set to true if this parameter is used.

top_p (`float`, *optional*) : Fraction of the most likely next words to sample from. Must be between 0 and 1. Defaults to 1.0.

tool_choice ([ChatCompletionInputToolChoiceClass](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionInputToolChoiceClass) or `ChatCompletionInputToolChoiceEnum()`, *optional*) : The tool to use for the completion. Defaults to "auto".

tool_prompt (`str`, *optional*) : A prompt to be appended before the tools.

tools (List of [ChatCompletionInputTool](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionInputTool), *optional*) : A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for.

extra_body (`dict`, *optional*) : Additional provider-specific parameters to pass to the model. Refer to the provider's documentation for supported parameters.

**Returns:** [ChatCompletionOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionOutput) or Iterable of [ChatCompletionStreamOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionStreamOutput)

Generated text returned from the server:
- if `stream=False`, the generated text is returned as a [ChatCompletionOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionOutput) (default).
- if `stream=True`, the generated text is returned token by token as a sequence of [ChatCompletionStreamOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionStreamOutput).

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

A method for completing conversations using a specified language model.

> [!TIP]
> The `client.chat_completion` method is aliased as `client.chat.completions.create` for compatibility with OpenAI's client.
> Inputs and outputs are strictly the same and using either syntax will yield the same results.
> Check out the [Inference guide](https://huggingface.co/docs/huggingface_hub/guides/inference#openai-compatibility)
> for more details about OpenAI's compatibility.

> [!TIP]
> You can pass provider-specific parameters to the model by using the `extra_body` argument.

Example:

```py
>>> from huggingface_hub import InferenceClient
>>> messages = [{"role": "user", "content": "What is the capital of France?"}]
>>> client = InferenceClient("meta-llama/Meta-Llama-3-8B-Instruct")
>>> client.chat_completion(messages, max_tokens=100)
ChatCompletionOutput(
    choices=[
        ChatCompletionOutputComplete(
            finish_reason='eos_token',
            index=0,
            message=ChatCompletionOutputMessage(
                role='assistant',
                content='The capital of France is Paris.',
                name=None,
                tool_calls=None
            ),
            logprobs=None
        )
    ],
    created=1719907176,
    id='',
    model='meta-llama/Meta-Llama-3-8B-Instruct',
    object='text_completion',
    system_fingerprint='2.0.4-sha-f426a33',
    usage=ChatCompletionOutputUsage(
        completion_tokens=8,
        prompt_tokens=17,
        total_tokens=25
    )
)
```

Example using streaming:
```py
>>> from huggingface_hub import InferenceClient
>>> messages = [{"role": "user", "content": "What is the capital of France?"}]
>>> client = InferenceClient("meta-llama/Meta-Llama-3-8B-Instruct")
>>> for token in client.chat_completion(messages, max_tokens=10, stream=True):
...     print(token)
ChatCompletionStreamOutput(choices=[ChatCompletionStreamOutputChoice(delta=ChatCompletionStreamOutputDelta(content='The', role='assistant'), index=0, finish_reason=None)], created=1710498504)
ChatCompletionStreamOutput(choices=[ChatCompletionStreamOutputChoice(delta=ChatCompletionStreamOutputDelta(content=' capital', role='assistant'), index=0, finish_reason=None)], created=1710498504)
(...)
ChatCompletionStreamOutput(choices=[ChatCompletionStreamOutputChoice(delta=ChatCompletionStreamOutputDelta(content=' may', role='assistant'), index=0, finish_reason=None)], created=1710498504)
```

Example using OpenAI's syntax:
```py
# instead of `from openai import OpenAI`
from huggingface_hub import InferenceClient

# instead of `client = OpenAI(...)`
client = InferenceClient(
    base_url=...,
    api_key=...,
)

output = client.chat.completions.create(
    model="meta-llama/Meta-Llama-3-8B-Instruct",
    messages=[
        {"role": "system", "content": "You are a helpful assistant."},
        {"role": "user", "content": "Count to 10"},
    ],
    stream=True,
    max_tokens=1024,
)

for chunk in output:
    print(chunk.choices[0].delta.content)
```

Example using a third-party provider directly with extra (provider-specific) parameters. Usage will be billed on your Together AI account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="together",  # Use Together AI provider
...     api_key="<together_api_key>",  # Pass your Together API key directly
... )
>>> client.chat_completion(
...     model="meta-llama/Meta-Llama-3-8B-Instruct",
...     messages=[{"role": "user", "content": "What is the capital of France?"}],
...     extra_body={"safety_model": "Meta-Llama/Llama-Guard-7b"},
... )
```

Example using a third-party provider through Hugging Face Routing. Usage will be billed on your Hugging Face account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="novita",  # Use Novita provider
...     api_key="hf_...",  # Pass your HF token
... )
>>> client.chat_completion(
...     model="meta-llama/Meta-Llama-3-8B-Instruct",
...     messages=[{"role": "user", "content": "What is the capital of France?"}],
... )
```

Example using Image + Text as input:
```py
>>> from huggingface_hub import InferenceClient

# provide a remote URL
>>> image_url ="https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg"
# or a base64-encoded image
>>> image_path = "/path/to/image.jpeg"
>>> with open(image_path, "rb") as f:
...     base64_image = base64.b64encode(f.read()).decode("utf-8")
>>> image_url = f"data:image/jpeg;base64,{base64_image}"

>>> client = InferenceClient("meta-llama/Llama-3.2-11B-Vision-Instruct")
>>> output = client.chat.completions.create(
...     messages=[
...         {
...             "role": "user",
...             "content": [
...                 {
...                     "type": "image_url",
...                     "image_url": {"url": image_url},
...                 },
...                 {
...                     "type": "text",
...                     "text": "Describe this image in one sentence.",
...                 },
...             ],
...         },
...     ],
... )
>>> output
The image depicts the iconic Statue of Liberty situated in New York Harbor, New York, on a clear day.
```

Example using tools:
```py
>>> client = InferenceClient("meta-llama/Meta-Llama-3-70B-Instruct")
>>> messages = [
...     {
...         "role": "system",
...         "content": "Don't make assumptions about what values to plug into functions. Ask for clarification if a user request is ambiguous.",
...     },
...     {
...         "role": "user",
...         "content": "What's the weather like the next 3 days in San Francisco, CA?",
...     },
... ]
>>> tools = [
...     {
...         "type": "function",
...         "function": {
...             "name": "get_current_weather",
...             "description": "Get the current weather",
...             "parameters": {
...                 "type": "object",
...                 "properties": {
...                     "location": {
...                         "type": "string",
...                         "description": "The city and state, e.g. San Francisco, CA",
...                     },
...                     "format": {
...                         "type": "string",
...                         "enum": ["celsius", "fahrenheit"],
...                         "description": "The temperature unit to use. Infer this from the users location.",
...                     },
...                 },
...                 "required": ["location", "format"],
...             },
...         },
...     },
...     {
...         "type": "function",
...         "function": {
...             "name": "get_n_day_weather_forecast",
...             "description": "Get an N-day weather forecast",
...             "parameters": {
...                 "type": "object",
...                 "properties": {
...                     "location": {
...                         "type": "string",
...                         "description": "The city and state, e.g. San Francisco, CA",
...                     },
...                     "format": {
...                         "type": "string",
...                         "enum": ["celsius", "fahrenheit"],
...                         "description": "The temperature unit to use. Infer this from the users location.",
...                     },
...                     "num_days": {
...                         "type": "integer",
...                         "description": "The number of days to forecast",
...                     },
...                 },
...                 "required": ["location", "format", "num_days"],
...             },
...         },
...     },
... ]

>>> response = client.chat_completion(
...     model="meta-llama/Meta-Llama-3-70B-Instruct",
...     messages=messages,
...     tools=tools,
...     tool_choice="auto",
...     max_tokens=500,
... )
>>> response.choices[0].message.tool_calls[0].function
ChatCompletionOutputFunctionDefinition(
    arguments={
        'location': 'San Francisco, CA',
        'format': 'fahrenheit',
        'num_days': 3
    },
    name='get_n_day_weather_forecast',
    description=None
)
```

Example using response_format:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient("meta-llama/Meta-Llama-3-70B-Instruct")
>>> messages = [
...     {
...         "role": "user",
...         "content": "I saw a puppy a cat and a raccoon during my bike ride in the park. What did I see and when?",
...     },
... ]
>>> response_format = {
...     "type": "json",
...     "value": {
...         "properties": {
...             "location": {"type": "string"},
...             "activity": {"type": "string"},
...             "animals_seen": {"type": "integer", "minimum": 1, "maximum": 5},
...             "animals": {"type": "array", "items": {"type": "string"}},
...         },
...         "required": ["location", "activity", "animals_seen", "animals"],
...     },
... }
>>> response = client.chat_completion(
...     messages=messages,
...     response_format=response_format,
...     max_tokens=500,
... )
>>> response.choices[0].message.content
'{

y": "bike ride",
": ["puppy", "cat", "raccoon"],
_seen": 3,
n": "park"}'
```

#### document_question_answering[[huggingface_hub.InferenceClient.document_question_answering]]

```python
document_question_answering(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], question: str, model: str | None = None, doc_stride: int | None = None, handle_impossible_answer: bool | None = None, lang: str | None = None, max_answer_len: int | None = None, max_question_len: int | None = None, max_seq_len: int | None = None, top_k: int | None = None, word_boxes: list[list[float] | str] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L945)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO]`) : The input image for the context. It can be raw bytes, an image file, or a URL to an online image.

question (`str`) : Question to be answered.

model (`str`, *optional*) : The model to use for the document question answering task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended document question answering model will be used. Defaults to None.

doc_stride (`int`, *optional*) : If the words in the document are too long to fit with the question for the model, it will be split in several chunks with some overlap. This argument controls the size of that overlap.

handle_impossible_answer (`bool`, *optional*) : Whether to accept impossible as an answer

lang (`str`, *optional*) : Language to use while running OCR. Defaults to english.

max_answer_len (`int`, *optional*) : The maximum length of predicted answers (e.g., only answers with a shorter length are considered).

max_question_len (`int`, *optional*) : The maximum length of the question after tokenization. It will be truncated if needed.

max_seq_len (`int`, *optional*) : The maximum length of the total sentence (context + question) in tokens of each chunk passed to the model. The context will be split in several chunks (using doc_stride as overlap) if needed.

top_k (`int`, *optional*) : The number of answers to return (will be chosen by order of likelihood). Can return less than top_k answers if there are not enough options available within the context.

word_boxes (`list[Union[list[float], str`, *optional*) : A list of words and bounding boxes (normalized 0->1000). If provided, the inference will skip the OCR step and use the provided bounding boxes instead.

**Returns:** `list[DocumentQuestionAnsweringOutputElement]`

a list of [DocumentQuestionAnsweringOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.DocumentQuestionAnsweringOutputElement) items containing the predicted label, associated probability, word ids, and page number.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Answer questions on document images.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.document_question_answering(image="https://huggingface.co/spaces/impira/docquery/resolve/2359223c1837a7587402bda0f2643382a6eefeab/invoice.png", question="What is the invoice number?")
[DocumentQuestionAnsweringOutputElement(answer='us-001', end=16, score=0.9999666213989258, start=16)]
```

#### feature_extraction[[huggingface_hub.InferenceClient.feature_extraction]]

```python
feature_extraction(text: str | list[str], normalize: bool | None = None, prompt_name: str | None = None, truncate: bool | None = None, truncation_direction: typing.Optional[typing.Literal['left', 'right']] = None, dimensions: int | None = None, encoding_format: typing.Optional[typing.Literal['float', 'base64']] = None, model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1032)

**Parameters:**

text (*str* or *list[str]*) : The text or list of texts to embed.

model (*str*, *optional*) : The model to use for the feature extraction task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended feature extraction model will be used. Defaults to None.

normalize (*bool*, *optional*) : Whether to normalize the embeddings or not. Only available on server powered by Text-Embedding-Inference.

prompt_name (*str*, *optional*) : The name of the prompt that should be used by for encoding. If not set, no prompt will be applied. Must be a key in the *Sentence Transformers* configuration *prompts* dictionary. For example if `prompt_name` is "query" and the `prompts` is &amp;lcub;"query": "query: ",...}, then the sentence "What is the capital of France?" will be encoded as "query: What is the capital of France?" because the prompt text will be prepended before any text to encode.

truncate (*bool*, *optional*) : Whether to truncate the embeddings or not. Only available on server powered by Text-Embedding-Inference.

truncation_direction (*Literal["left", "right"]*, *optional*) : Which side of the input should be truncated when *truncate=True* is passed.

dimensions (*int*, *optional*) : The number of dimensions the resulting output embeddings should have. Only available on OpenAI-compatible embedding endpoints.

encoding_format (*Literal["float", "base64"]*, *optional*) : The format of the output embeddings. Either "float" or "base64". Only available on OpenAI-compatible embedding endpoints.

**Returns:** `*np.ndarray*`

The embedding representing the input text(s) as a float32 numpy array.

**Raises:** [*InferenceTimeoutError*] or [*HfHubHTTPError*]

- [*InferenceTimeoutError*] -- 
  If the model is unavailable or the request times out.
- [*HfHubHTTPError*] -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Generate embeddings for a given text or batch of texts.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.feature_extraction("Hi, who are you?")
array([[ 2.424802  ,  2.93384   ,  1.1750331 , ...,  1.240499, -0.13776633, -0.7889173 ],
[-0.42943227, -0.6364878 , -1.693462  , ...,  0.41978157, -2.4336355 ,  0.6162071 ],
...,
[ 0.28552425, -0.928395  , -1.2077185 , ...,  0.76810825, -2.1069427 ,  0.6236161 ]], dtype=float32)
```

#### fill_mask[[huggingface_hub.InferenceClient.fill_mask]]

```python
fill_mask(text: str, model: str | None = None, targets: list[str] | None = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1115)

**Parameters:**

text (`str`) : a string to be filled from, must contain the [MASK] token (check model card for exact name of the mask).

model (`str`, *optional*) : The model to use for the fill mask task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended fill mask model will be used.

targets (`list[str`, *optional*) : When passed, the model will limit the scores to the passed targets instead of looking up in the whole vocabulary. If the provided targets are not in the model vocab, they will be tokenized and the first resulting token will be used (with a warning, and that might be slower).

top_k (`int`, *optional*) : When passed, overrides the number of predictions to return.

**Returns:** `list[FillMaskOutputElement]`

a list of [FillMaskOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.FillMaskOutputElement) items containing the predicted label, associated
probability, token reference, and completed text.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Fill in a hole with a missing word (token to be precise).

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.fill_mask("The goal of life is <mask>.")
[
    FillMaskOutputElement(score=0.06897063553333282, token=11098, token_str=' happiness', sequence='The goal of life is happiness.'),
    FillMaskOutputElement(score=0.06554922461509705, token=45075, token_str=' immortality', sequence='The goal of life is immortality.')
]
```

#### get_endpoint_info[[huggingface_hub.InferenceClient.get_endpoint_info]]

```python
get_endpoint_info(model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L3279)

**Parameters:**

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

**Returns:** `dict[str, Any]`

Information about the endpoint.

Get information about the deployed endpoint.

This endpoint is only available on endpoints powered by Text-Generation-Inference (TGI) or Text-Embedding-Inference (TEI).
Endpoints powered by `transformers` return an empty payload.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient("meta-llama/Meta-Llama-3-70B-Instruct")
>>> client.get_endpoint_info()
{
    'model_id': 'meta-llama/Meta-Llama-3-70B-Instruct',
    'model_sha': None,
    'model_dtype': 'torch.float16',
    'model_device_type': 'cuda',
    'model_pipeline_tag': None,
    'max_concurrent_requests': 128,
    'max_best_of': 2,
    'max_stop_sequences': 4,
    'max_input_length': 8191,
    'max_total_tokens': 8192,
    'waiting_served_ratio': 0.3,
    'max_batch_total_tokens': 1259392,
    'max_waiting_tokens': 20,
    'max_batch_size': None,
    'validation_workers': 32,
    'max_client_batch_size': 4,
    'version': '2.0.2',
    'sha': 'dccab72549635c7eb5ddb17f43f0b7cdff07c214',
    'docker_label': 'sha-dccab72'
}
```

#### health_check[[huggingface_hub.InferenceClient.health_check]]

```python
health_check(model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L3337)

**Parameters:**

model (`str`, *optional*) : URL of the Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

**Returns:** `bool`

True if everything is working fine.

Check the health of the deployed endpoint.

Health check is only available with Inference Endpoints powered by Text-Generation-Inference (TGI) or Text-Embedding-Inference (TEI).

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient("https://jzgu0buei5.us-east-1.aws.endpoints.huggingface.cloud")
>>> client.health_check()
True
```

#### image_classification[[huggingface_hub.InferenceClient.image_classification]]

```python
image_classification(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, function_to_apply: typing.Optional[ForwardRef('ImageClassificationOutputTransform')] = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1171)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The image to classify. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

model (`str`, *optional*) : The model to use for image classification. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for image classification will be used.

function_to_apply (`"ImageClassificationOutputTransform"`, *optional*) : The function to apply to the model outputs in order to retrieve the scores.

top_k (`int`, *optional*) : When specified, limits the output to the top K most probable classes.

**Returns:** `list[ImageClassificationOutputElement]`

a list of [ImageClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ImageClassificationOutputElement) items containing the predicted label and associated probability.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform image classification on the given image using the specified model.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.image_classification("https://upload.wikimedia.org/wikipedia/commons/thumb/4/43/Cute_dog.jpg/320px-Cute_dog.jpg")
[ImageClassificationOutputElement(label='Blenheim spaniel', score=0.9779096841812134), ...]
```

#### image_segmentation[[huggingface_hub.InferenceClient.image_segmentation]]

```python
image_segmentation(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, mask_threshold: float | None = None, overlap_mask_area_threshold: float | None = None, subtask: typing.Optional[ForwardRef('ImageSegmentationSubtask')] = None, threshold: float | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1221)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The image to segment. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

model (`str`, *optional*) : The model to use for image segmentation. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for image segmentation will be used.

mask_threshold (`float`, *optional*) : Threshold to use when turning the predicted masks into binary values.

overlap_mask_area_threshold (`float`, *optional*) : Mask overlap threshold to eliminate small, disconnected segments.

subtask (`"ImageSegmentationSubtask"`, *optional*) : Segmentation task to be performed, depending on model capabilities.

threshold (`float`, *optional*) : Probability threshold to filter out predicted masks.

**Returns:** `list[ImageSegmentationOutputElement]`

A list of [ImageSegmentationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ImageSegmentationOutputElement) items containing the segmented masks and associated attributes.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform image segmentation on the given image using the specified model.

> [!WARNING]
> You must have `PIL` installed if you want to work with images (`pip install Pillow`).

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.image_segmentation("cat.jpg")
[ImageSegmentationOutputElement(score=0.989008, label='LABEL_184', mask=<PIL.PngImagePlugin.PngImageFile image mode=L size=400x300 at 0x7FDD2B129CC0>), ...]
```

#### image_to_image[[huggingface_hub.InferenceClient.image_to_image]]

```python
image_to_image(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], prompt: str | None = None, negative_prompt: str | None = None, num_inference_steps: int | None = None, guidance_scale: float | None = None, model: str | None = None, target_size: huggingface_hub.inference._generated.types.image_to_image.ImageToImageTargetSize | None = None, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1289)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The input image for translation. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

prompt (`str`, *optional*) : The text prompt to guide the image generation.

negative_prompt (`str`, *optional*) : One prompt to guide what NOT to include in image generation.

num_inference_steps (`int`, *optional*) : For diffusion models. The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*) : For diffusion models. A higher guidance scale value encourages the model to generate images closely linked to the text prompt at the expense of lower image quality.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

target_size (`ImageToImageTargetSize`, *optional*) : The size in pixels of the output image. This parameter is only supported by some providers and for specific models. It will be ignored when unsupported.

**Returns:** `Image`

The translated image.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform image-to-image translation using a specified model.

> [!WARNING]
> You must have `PIL` installed if you want to work with images (`pip install Pillow`).

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> image = client.image_to_image("cat.jpg", prompt="turn the cat into a tiger")
>>> image.save("tiger.jpg")
```

#### image_to_text[[huggingface_hub.InferenceClient.image_to_text]]

```python
image_to_text(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1444)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The input image to caption. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

**Returns:** [ImageToTextOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ImageToTextOutput)

The generated text.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Takes an input image and return text.

Models can have very different outputs depending on your use case (image captioning, optical character recognition
(OCR), Pix2Struct, etc.). Please have a look to the model card to learn more about a model's specificities.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.image_to_text("cat.jpg")
'a cat standing in a grassy field '
>>> client.image_to_text("https://upload.wikimedia.org/wikipedia/commons/thumb/4/43/Cute_dog.jpg/320px-Cute_dog.jpg")
'a dog laying on the grass next to a flower pot '
```

#### image_to_video[[huggingface_hub.InferenceClient.image_to_video]]

```python
image_to_video(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, prompt: str | None = None, negative_prompt: str | None = None, num_frames: float | None = None, num_inference_steps: int | None = None, guidance_scale: float | None = None, seed: int | None = None, target_size: huggingface_hub.inference._generated.types.image_to_video.ImageToVideoTargetSize | None = None, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1365)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The input image to generate a video from. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

prompt (`str`, *optional*) : The text prompt to guide the video generation.

negative_prompt (`str`, *optional*) : One prompt to guide what NOT to include in video generation.

num_frames (`float`, *optional*) : The num_frames parameter determines how many video frames are generated.

num_inference_steps (`int`, *optional*) : For diffusion models. The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*) : For diffusion models. A higher guidance scale value encourages the model to generate videos closely linked to the text prompt at the expense of lower image quality.

seed (`int`, *optional*) : The seed to use for the video generation.

target_size (`ImageToVideoTargetSize`, *optional*) : The size in pixel of the output video frames.

num_inference_steps (`int`, *optional*) : The number of denoising steps. More denoising steps usually lead to a higher quality video at the expense of slower inference.

seed (`int`, *optional*) : Seed for the random number generator.

**Returns:** `bytes`

The generated video.

Generate a video from an input image.

Examples:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> video = client.image_to_video("cat.jpg", model="Wan-AI/Wan2.2-I2V-A14B", prompt="turn the cat into a tiger")
>>> with open("tiger.mp4", "wb") as f:
...     f.write(video)
```

#### object_detection[[huggingface_hub.InferenceClient.object_detection]]

```python
object_detection(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, threshold: float | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1490)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The image to detect objects on. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

model (`str`, *optional*) : The model to use for object detection. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for object detection (DETR) will be used.

threshold (`float`, *optional*) : The probability necessary to make a prediction.

**Returns:** `list[ObjectDetectionOutputElement]`

A list of [ObjectDetectionOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ObjectDetectionOutputElement) items containing the bounding boxes and associated attributes.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError` or ``ValueError``

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.
- ``ValueError`` -- 
  If the request output is not a List.

Perform object detection on the given image using the specified model.

> [!WARNING]
> You must have `PIL` installed if you want to work with images (`pip install Pillow`).

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.object_detection("people.jpg")
[ObjectDetectionOutputElement(score=0.9486683011054993, label='person', box=ObjectDetectionBoundingBox(xmin=59, ymin=39, xmax=420, ymax=510)), ...]
```

#### question_answering[[huggingface_hub.InferenceClient.question_answering]]

```python
question_answering(question: str, context: str, model: str | None = None, align_to_words: bool | None = None, doc_stride: int | None = None, handle_impossible_answer: bool | None = None, max_answer_len: int | None = None, max_question_len: int | None = None, max_seq_len: int | None = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1538)

**Parameters:**

question (`str`) : Question to be answered.

context (`str`) : The context of the question.

model (`str`) : The model to use for the question answering task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint.

align_to_words (`bool`, *optional*) : Attempts to align the answer to real words. Improves quality on space separated languages. Might hurt on non-space-separated languages (like Japanese or Chinese)

doc_stride (`int`, *optional*) : If the context is too long to fit with the question for the model, it will be split in several chunks with some overlap. This argument controls the size of that overlap.

handle_impossible_answer (`bool`, *optional*) : Whether to accept impossible as an answer.

max_answer_len (`int`, *optional*) : The maximum length of predicted answers (e.g., only answers with a shorter length are considered).

max_question_len (`int`, *optional*) : The maximum length of the question after tokenization. It will be truncated if needed.

max_seq_len (`int`, *optional*) : The maximum length of the total sentence (context + question) in tokens of each chunk passed to the model. The context will be split in several chunks (using docStride as overlap) if needed.

top_k (`int`, *optional*) : The number of answers to return (will be chosen by order of likelihood). Note that we return less than topk answers if there are not enough options available within the context.

**Returns:** Union[`QuestionAnsweringOutputElement`, list[QuestionAnsweringOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.QuestionAnsweringOutputElement)]

When top_k is 1 or not provided, it returns a single `QuestionAnsweringOutputElement`.
When top_k is greater than 1, it returns a list of `QuestionAnsweringOutputElement`.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Retrieve the answer to a question from a given text.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.question_answering(question="What's my name?", context="My name is Clara and I live in Berkeley.")
QuestionAnsweringOutputElement(answer='Clara', end=16, score=0.9326565265655518, start=11)
```

#### sentence_similarity[[huggingface_hub.InferenceClient.sentence_similarity]]

```python
sentence_similarity(sentence: str, other_sentences: list, model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1622)

**Parameters:**

sentence (`str`) : The main sentence to compare to others.

other_sentences (`list[str]`) : The list of sentences to compare to.

model (`str`, *optional*) : The model to use for the sentence similarity task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended sentence similarity model will be used. Defaults to None.

**Returns:** `list[float]`

The embedding representing the input text.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Compute the semantic similarity between a sentence and a list of other sentences by comparing their embeddings.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.sentence_similarity(
...     "Machine learning is so easy.",
...     other_sentences=[
...         "Deep learning is so straightforward.",
...         "This is so difficult, like rocket science.",
...         "I can't believe how much I struggled with this.",
...     ],
... )
[0.7785726189613342, 0.45876261591911316, 0.2906220555305481]
```

#### summarization[[huggingface_hub.InferenceClient.summarization]]

```python
summarization(text: str, model: str | None = None, clean_up_tokenization_spaces: bool | None = None, generate_parameters: dict[str, typing.Any] | None = None, truncation: typing.Optional[ForwardRef('SummarizationTruncationStrategy')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1675)

**Parameters:**

text (`str`) : The input text to summarize.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for summarization will be used.

clean_up_tokenization_spaces (`bool`, *optional*) : Whether to clean up the potential extra spaces in the text output.

generate_parameters (`dict[str, Any]`, *optional*) : Additional parametrization of the text generation algorithm.

truncation (`"SummarizationTruncationStrategy"`, *optional*) : The truncation strategy to use.

**Returns:** [SummarizationOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.SummarizationOutput)

The generated summary text.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Generate a summary of a given text using a specified model.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.summarization("The Eiffel tower...")
SummarizationOutput(generated_text="The Eiffel tower is one of the most famous landmarks in the world....")
```

#### table_question_answering[[huggingface_hub.InferenceClient.table_question_answering]]

```python
table_question_answering(table: dict, query: str, model: str | None = None, padding: typing.Optional[ForwardRef('Padding')] = None, sequential: bool | None = None, truncation: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1733)

**Parameters:**

table (`str`) : A table of data represented as a dict of lists where entries are headers and the lists are all the values, all lists must have the same size.

query (`str`) : The query in plain text that you want to ask the table.

model (`str`) : The model to use for the table-question-answering task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint.

padding (`"Padding"`, *optional*) : Activates and controls padding.

sequential (`bool`, *optional*) : Whether to do inference sequentially or as a batch. Batching is faster, but models like SQA require the inference to be done sequentially to extract relations within sequences, given their conversational nature.

truncation (`bool`, *optional*) : Activates and controls truncation.

**Returns:** [TableQuestionAnsweringOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TableQuestionAnsweringOutputElement)

a table question answering output containing the answer, coordinates, cells and the aggregator used.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Retrieve the answer to a question from information given in a table.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> query = "How many stars does the transformers repository have?"
>>> table = {"Repository": ["Transformers", "Datasets", "Tokenizers"], "Stars": ["36542", "4512", "3934"]}
>>> client.table_question_answering(table, query, model="google/tapas-base-finetuned-wtq")
TableQuestionAnsweringOutputElement(answer='36542', coordinates=[[0, 1]], cells=['36542'], aggregator='AVERAGE')
```

#### tabular_classification[[huggingface_hub.InferenceClient.tabular_classification]]

```python
tabular_classification(table: dict, model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1795)

**Parameters:**

table (`dict[str, Any]`) : Set of attributes to classify.

model (`str`, *optional*) : The model to use for the tabular classification task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended tabular classification model will be used. Defaults to None.

**Returns:** `List`

a list of labels, one per row in the initial table.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Classifying a target category (a group) based on a set of attributes.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> table = {
...     "fixed_acidity": ["7.4", "7.8", "10.3"],
...     "volatile_acidity": ["0.7", "0.88", "0.32"],
...     "citric_acid": ["0", "0", "0.45"],
...     "residual_sugar": ["1.9", "2.6", "6.4"],
...     "chlorides": ["0.076", "0.098", "0.073"],
...     "free_sulfur_dioxide": ["11", "25", "5"],
...     "total_sulfur_dioxide": ["34", "67", "13"],
...     "density": ["0.9978", "0.9968", "0.9976"],
...     "pH": ["3.51", "3.2", "3.23"],
...     "sulphates": ["0.56", "0.68", "0.82"],
...     "alcohol": ["9.4", "9.8", "12.6"],
... }
>>> client.tabular_classification(table=table, model="julien-c/wine-quality")
["5", "5", "5"]
```

#### tabular_regression[[huggingface_hub.InferenceClient.tabular_regression]]

```python
tabular_regression(table: dict, model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1850)

**Parameters:**

table (`dict[str, Any]`) : Set of attributes stored in a table. The attributes used to predict the target can be both numerical and categorical.

model (`str`, *optional*) : The model to use for the tabular regression task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended tabular regression model will be used. Defaults to None.

**Returns:** `List`

a list of predicted numerical target values.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Predicting a numerical target value given a set of attributes/features in a table.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> table = {
...     "Height": ["11.52", "12.48", "12.3778"],
...     "Length1": ["23.2", "24", "23.9"],
...     "Length2": ["25.4", "26.3", "26.5"],
...     "Length3": ["30", "31.2", "31.1"],
...     "Species": ["Bream", "Bream", "Bream"],
...     "Width": ["4.02", "4.3056", "4.6961"],
... }
>>> client.tabular_regression(table, model="scikit-learn/Fish-Weight")
[110, 120, 130]
```

#### text_classification[[huggingface_hub.InferenceClient.text_classification]]

```python
text_classification(text: str, model: str | None = None, top_k: int | None = None, function_to_apply: typing.Optional[ForwardRef('TextClassificationOutputTransform')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L1900)

**Parameters:**

text (`str`) : A string to be classified.

model (`str`, *optional*) : The model to use for the text classification task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended text classification model will be used. Defaults to None.

top_k (`int`, *optional*) : When specified, limits the output to the top K most probable classes.

function_to_apply (`"TextClassificationOutputTransform"`, *optional*) : The function to apply to the model outputs in order to retrieve the scores.

**Returns:** `list[TextClassificationOutputElement]`

a list of [TextClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TextClassificationOutputElement) items containing the predicted label and associated probability.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform text classification (e.g. sentiment-analysis) on the given text.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.text_classification("I like you")
[
    TextClassificationOutputElement(label='POSITIVE', score=0.9998695850372314),
    TextClassificationOutputElement(label='NEGATIVE', score=0.0001304351753788069),
]
```

#### text_generation[[huggingface_hub.InferenceClient.text_generation]]

```python
text_generation(prompt: str, details: bool | None = None, stream: bool | None = None, model: str | None = None, adapter_id: str | None = None, best_of: int | None = None, decoder_input_details: bool | None = None, do_sample: bool | None = None, frequency_penalty: float | None = None, grammar: huggingface_hub.inference._generated.types.text_generation.TextGenerationInputGrammarType | None = None, max_new_tokens: int | None = None, repetition_penalty: float | None = None, return_full_text: bool | None = None, seed: int | None = None, stop: list[str] | None = None, stop_sequences: list[str] | None = None, temperature: float | None = None, top_k: int | None = None, top_n_tokens: int | None = None, top_p: float | None = None, truncate: int | None = None, typical_p: float | None = None, watermark: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L2108)

**Parameters:**

prompt (`str`) : Input text.

details (`bool`, *optional*) : By default, text_generation returns a string. Pass `details=True` if you want a detailed output (tokens, probabilities, seed, finish reason, etc.). Only available for models running on with the `text-generation-inference` backend.

stream (`bool`, *optional*) : By default, text_generation returns the full generated text. Pass `stream=True` if you want a stream of tokens to be returned. Only available for models running on with the `text-generation-inference` backend.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

adapter_id (`str`, *optional*) : Lora adapter id.

best_of (`int`, *optional*) : Generate best_of sequences and return the one if the highest token logprobs.

decoder_input_details (`bool`, *optional*) : Return the decoder input token logprobs and ids. You must set `details=True` as well for it to be taken into account. Defaults to `False`.

do_sample (`bool`, *optional*) : Activate logits sampling

frequency_penalty (`float`, *optional*) : Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.

grammar ([TextGenerationInputGrammarType](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TextGenerationInputGrammarType), *optional*) : Grammar constraints. Can be either a JSONSchema or a regex.

max_new_tokens (`int`, *optional*) : Maximum number of generated tokens. Defaults to 100.

repetition_penalty (`float`, *optional*) : The parameter for repetition penalty. 1.0 means no penalty. See [this paper](https://arxiv.org/pdf/1909.05858.pdf) for more details.

return_full_text (`bool`, *optional*) : Whether to prepend the prompt to the generated text

seed (`int`, *optional*) : Random sampling seed

stop (`list[str]`, *optional*) : Stop generating tokens if a member of `stop` is generated.

stop_sequences (`list[str]`, *optional*) : Deprecated argument. Use `stop` instead.

temperature (`float`, *optional*) : The value used to module the logits distribution.

top_n_tokens (`int`, *optional*) : Return information about the `top_n_tokens` most likely tokens at each generation step, instead of just the sampled token.

top_k (`int`, *optional`) : The number of highest probability vocabulary tokens to keep for top-k-filtering.

top_p (`float`, *optional`) : If set to < 1, only the smallest set of most probable tokens with probabilities that add up to `top_p` or higher are kept for generation.

truncate (`int`, *optional`) : Truncate inputs tokens to the given size.

typical_p (`float`, *optional`) : Typical Decoding mass See [Typical Decoding for Natural Language Generation](https://arxiv.org/abs/2202.00666) for more information

watermark (`bool`, *optional*) : Watermarking with [A Watermark for Large Language Models](https://arxiv.org/abs/2301.10226)

**Returns:** `Union[str, TextGenerationOutput, Iterable[str], Iterable[TextGenerationStreamOutput]]`

Generated text returned from the server:
- if `stream=False` and `details=False`, the generated text is returned as a `str` (default)
- if `stream=True` and `details=False`, the generated text is returned token by token as a `Iterable[str]`
- if `stream=False` and `details=True`, the generated text is returned with more details as a [TextGenerationOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TextGenerationOutput)
- if `details=True` and `stream=True`, the generated text is returned token by token as a iterable of [TextGenerationStreamOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TextGenerationStreamOutput)

**Raises:** ``ValidationError`` or [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- ``ValidationError`` -- 
  If input values are not valid. No HTTP call is made to the server.
- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Given a prompt, generate the following text.

> [!TIP]
> If you want to generate a response from chat messages, you should use the [InferenceClient.chat_completion()](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient.chat_completion) method.
> It accepts a list of messages instead of a single text prompt and handles the chat templating for you.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()

# Case 1: generate text
>>> client.text_generation("The huggingface_hub library is ", max_new_tokens=12)
'100% open source and built to be easy to use.'

# Case 2: iterate over the generated tokens. Useful for large generation.
>>> for token in client.text_generation("The huggingface_hub library is ", max_new_tokens=12, stream=True):
...     print(token)
100
%
open
source
and
built
to
be
easy
to
use
.

# Case 3: get more details about the generation process.
>>> client.text_generation("The huggingface_hub library is ", max_new_tokens=12, details=True)
TextGenerationOutput(
    generated_text='100% open source and built to be easy to use.',
    details=TextGenerationDetails(
        finish_reason='length',
        generated_tokens=12,
        seed=None,
        prefill=[
            TextGenerationPrefillOutputToken(id=487, text='The', logprob=None),
            TextGenerationPrefillOutputToken(id=53789, text=' hugging', logprob=-13.171875),
            (...)
            TextGenerationPrefillOutputToken(id=204, text=' ', logprob=-7.0390625)
        ],
        tokens=[
            TokenElement(id=1425, text='100', logprob=-1.0175781, special=False),
            TokenElement(id=16, text='%', logprob=-0.0463562, special=False),
            (...)
            TokenElement(id=25, text='.', logprob=-0.5703125, special=False)
        ],
        best_of_sequences=None
    )
)

# Case 4: iterate over the generated tokens with more details.
# Last object is more complete, containing the full generated text and the finish reason.
>>> for details in client.text_generation("The huggingface_hub library is ", max_new_tokens=12, details=True, stream=True):
...     print(details)
...
TextGenerationStreamOutput(token=TokenElement(id=1425, text='100', logprob=-1.0175781, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=16, text='%', logprob=-0.0463562, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=1314, text=' open', logprob=-1.3359375, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=3178, text=' source', logprob=-0.28100586, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=273, text=' and', logprob=-0.5961914, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=3426, text=' built', logprob=-1.9423828, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=271, text=' to', logprob=-1.4121094, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=314, text=' be', logprob=-1.5224609, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=1833, text=' easy', logprob=-2.1132812, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=271, text=' to', logprob=-0.08520508, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=745, text=' use', logprob=-0.39453125, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(
    id=25,
    text='.',
    logprob=-0.5703125,
    special=False),
    generated_text='100% open source and built to be easy to use.',
    details=TextGenerationStreamOutputStreamDetails(finish_reason='length', generated_tokens=12, seed=None)
)

# Case 5: generate constrained output using grammar
>>> response = client.text_generation(
...     prompt="I saw a puppy a cat and a raccoon during my bike ride in the park",
...     model="HuggingFaceH4/zephyr-orpo-141b-A35b-v0.1",
...     max_new_tokens=100,
...     repetition_penalty=1.3,
...     grammar={
...         "type": "json",
...         "value": {
...             "properties": {
...                 "location": {"type": "string"},
...                 "activity": {"type": "string"},
...                 "animals_seen": {"type": "integer", "minimum": 1, "maximum": 5},
...                 "animals": {"type": "array", "items": {"type": "string"}},
...             },
...             "required": ["location", "activity", "animals_seen", "animals"],
...         },
...     },
... )
>>> json.loads(response)
{
    "activity": "bike riding",
    "animals": ["puppy", "cat", "raccoon"],
    "animals_seen": 3,
    "location": "park"
}
```

#### text_to_image[[huggingface_hub.InferenceClient.text_to_image]]

```python
text_to_image(prompt: str, negative_prompt: str | None = None, height: int | None = None, width: int | None = None, num_inference_steps: int | None = None, guidance_scale: float | None = None, model: str | None = None, scheduler: str | None = None, seed: int | None = None, extra_body: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L2447)

**Parameters:**

prompt (`str`) : The prompt to generate an image from.

negative_prompt (`str`, *optional*) : One prompt to guide what NOT to include in image generation.

height (`int`, *optional*) : The height in pixels of the output image

width (`int`, *optional*) : The width in pixels of the output image

num_inference_steps (`int`, *optional*) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*) : A higher guidance scale value encourages the model to generate images closely linked to the text prompt, but values too high may cause saturation and other artifacts.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended text-to-image model will be used. Defaults to None.

scheduler (`str`, *optional*) : Override the scheduler with a compatible one.

seed (`int`, *optional*) : Seed for the random number generator.

extra_body (`dict[str, Any]`, *optional*) : Additional provider-specific parameters to pass to the model. Refer to the provider's documentation for supported parameters.

**Returns:** `Image`

The generated image.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Generate an image based on a given text using a specified model.

> [!WARNING]
> You must have `PIL` installed if you want to work with images (`pip install Pillow`).

> [!TIP]
> You can pass provider-specific parameters to the model by using the `extra_body` argument.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()

>>> image = client.text_to_image("An astronaut riding a horse on the moon.")
>>> image.save("astronaut.png")

>>> image = client.text_to_image(
...     "An astronaut riding a horse on the moon.",
...     negative_prompt="low resolution, blurry",
...     model="stabilityai/stable-diffusion-2-1",
... )
>>> image.save("better_astronaut.png")
```

Example using a third-party provider directly. Usage will be billed on your fal.ai account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="fal-ai",  # Use fal.ai provider
...     api_key="fal-ai-api-key",  # Pass your fal.ai API key
... )
>>> image = client.text_to_image(
...     "A majestic lion in a fantasy forest",
...     model="black-forest-labs/FLUX.1-schnell",
... )
>>> image.save("lion.png")
```

Example using a third-party provider through Hugging Face Routing. Usage will be billed on your Hugging Face account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",  # Use replicate provider
...     api_key="hf_...",  # Pass your HF token
... )
>>> image = client.text_to_image(
...     "An astronaut riding a horse on the moon.",
...     model="black-forest-labs/FLUX.1-dev",
... )
>>> image.save("astronaut.png")
```

Example using Replicate provider with extra parameters

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",  # Use replicate provider
...     api_key="hf_...",  # Pass your HF token
... )
>>> image = client.text_to_image(
...     "An astronaut riding a horse on the moon.",
...     model="black-forest-labs/FLUX.1-schnell",
...     extra_body={"output_quality": 100},
... )
>>> image.save("astronaut.png")
```

#### text_to_speech[[huggingface_hub.InferenceClient.text_to_speech]]

```python
text_to_speech(text: str, model: str | None = None, do_sample: bool | None = None, early_stopping: typing.Union[bool, ForwardRef('TextToSpeechEarlyStoppingEnum'), NoneType] = None, epsilon_cutoff: float | None = None, eta_cutoff: float | None = None, max_length: int | None = None, max_new_tokens: int | None = None, min_length: int | None = None, min_new_tokens: int | None = None, num_beam_groups: int | None = None, num_beams: int | None = None, penalty_alpha: float | None = None, temperature: float | None = None, top_k: int | None = None, top_p: float | None = None, typical_p: float | None = None, use_cache: bool | None = None, extra_body: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L2684)

**Parameters:**

text (`str`) : The text to synthesize.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended text-to-speech model will be used. Defaults to None.

do_sample (`bool`, *optional*) : Whether to use sampling instead of greedy decoding when generating new tokens.

early_stopping (`Union[bool, "TextToSpeechEarlyStoppingEnum"]`, *optional*) : Controls the stopping condition for beam-based methods.

epsilon_cutoff (`float`, *optional*) : If set to float strictly between 0 and 1, only tokens with a conditional probability greater than epsilon_cutoff will be sampled. In the paper, suggested values range from 3e-4 to 9e-4, depending on the size of the model. See [Truncation Sampling as Language Model Desmoothing](https://hf.co/papers/2210.15191) for more details.

eta_cutoff (`float`, *optional*) : Eta sampling is a hybrid of locally typical sampling and epsilon sampling. If set to float strictly between 0 and 1, a token is only considered if it is greater than either eta_cutoff or sqrt(eta_cutoff) * exp(-entropy(softmax(next_token_logits))). The latter term is intuitively the expected next token probability, scaled by sqrt(eta_cutoff). In the paper, suggested values range from 3e-4 to 2e-3, depending on the size of the model. See [Truncation Sampling as Language Model Desmoothing](https://hf.co/papers/2210.15191) for more details.

max_length (`int`, *optional*) : The maximum length (in tokens) of the generated text, including the input.

max_new_tokens (`int`, *optional*) : The maximum number of tokens to generate. Takes precedence over max_length.

min_length (`int`, *optional*) : The minimum length (in tokens) of the generated text, including the input.

min_new_tokens (`int`, *optional*) : The minimum number of tokens to generate. Takes precedence over min_length.

num_beam_groups (`int`, *optional*) : Number of groups to divide num_beams into in order to ensure diversity among different groups of beams. See [this paper](https://hf.co/papers/1610.02424) for more details.

num_beams (`int`, *optional*) : Number of beams to use for beam search.

penalty_alpha (`float`, *optional*) : The value balances the model confidence and the degeneration penalty in contrastive search decoding.

temperature (`float`, *optional*) : The value used to modulate the next token probabilities.

top_k (`int`, *optional*) : The number of highest probability vocabulary tokens to keep for top-k-filtering.

top_p (`float`, *optional*) : If set to float < 1, only the smallest set of most probable tokens with probabilities that add up to top_p or higher are kept for generation.

typical_p (`float`, *optional*) : Local typicality measures how similar the conditional probability of predicting a target token next is to the expected conditional probability of predicting a random token next, given the partial text already generated. If set to float < 1, the smallest set of the most locally typical tokens with probabilities that add up to typical_p or higher are kept for generation. See [this paper](https://hf.co/papers/2202.00666) for more details.

use_cache (`bool`, *optional*) : Whether the model should use the past last key/values attentions to speed up decoding

extra_body (`dict[str, Any]`, *optional*) : Additional provider-specific parameters to pass to the model. Refer to the provider's documentation for supported parameters.

**Returns:** `bytes`

The generated audio.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Synthesize an audio of a voice pronouncing a given text.

> [!TIP]
> You can pass provider-specific parameters to the model by using the `extra_body` argument.

Example:
```py
>>> from pathlib import Path
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()

>>> audio = client.text_to_speech("Hello world")
>>> Path("hello_world.flac").write_bytes(audio)
```

Example using a third-party provider directly. Usage will be billed on your Replicate account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",
...     api_key="your-replicate-api-key",  # Pass your Replicate API key directly
... )
>>> audio = client.text_to_speech(
...     text="Hello world",
...     model="OuteAI/OuteTTS-0.3-500M",
... )
>>> Path("hello_world.flac").write_bytes(audio)
```

Example using a third-party provider through Hugging Face Routing. Usage will be billed on your Hugging Face account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",
...     api_key="hf_...",  # Pass your HF token
... )
>>> audio =client.text_to_speech(
...     text="Hello world",
...     model="OuteAI/OuteTTS-0.3-500M",
... )
>>> Path("hello_world.flac").write_bytes(audio)
```

Example using Replicate provider with extra parameters

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",  # Use replicate provider
...     api_key="hf_...",  # Pass your HF token
... )
>>> audio = client.text_to_speech(
...     "Hello, my name is Kororo, an awesome text-to-speech model.",
...     model="hexgrad/Kokoro-82M",
...     extra_body={"voice": "af_nicole"},
... )
>>> Path("hello.flac").write_bytes(audio)
```

Example music-gen using "YuE-s1-7B-anneal-en-cot" on fal.ai

```py
>>> from huggingface_hub import InferenceClient
>>> lyrics = '''
... [verse]
... In the town where I was born
... Lived a man who sailed to sea
... And he told us of his life
... In the land of submarines
... So we sailed on to the sun
... 'Til we found a sea of green
... And we lived beneath the waves
... In our yellow submarine

... [chorus]
... We all live in a yellow submarine
... Yellow submarine, yellow submarine
... We all live in a yellow submarine
... Yellow submarine, yellow submarine
... '''
>>> genres = "pavarotti-style tenor voice"
>>> client = InferenceClient(
...     provider="fal-ai",
...     model="m-a-p/YuE-s1-7B-anneal-en-cot",
...     api_key=...,
... )
>>> audio = client.text_to_speech(lyrics, extra_body={"genres": genres})
>>> with open("output.mp3", "wb") as f:
...     f.write(audio)
```

#### text_to_video[[huggingface_hub.InferenceClient.text_to_video]]

```python
text_to_video(prompt: str, model: str | None = None, guidance_scale: float | None = None, negative_prompt: list[str] | None = None, num_frames: float | None = None, num_inference_steps: int | None = None, seed: int | None = None, extra_body: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L2587)

**Parameters:**

prompt (`str`) : The prompt to generate a video from.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended text-to-video model will be used. Defaults to None.

guidance_scale (`float`, *optional*) : A higher guidance scale value encourages the model to generate videos closely linked to the text prompt, but values too high may cause saturation and other artifacts.

negative_prompt (`list[str]`, *optional*) : One or several prompt to guide what NOT to include in video generation.

num_frames (`float`, *optional*) : The num_frames parameter determines how many video frames are generated.

num_inference_steps (`int`, *optional*) : The number of denoising steps. More denoising steps usually lead to a higher quality video at the expense of slower inference.

seed (`int`, *optional*) : Seed for the random number generator.

extra_body (`dict[str, Any]`, *optional*) : Additional provider-specific parameters to pass to the model. Refer to the provider's documentation for supported parameters.

**Returns:** `bytes`

The generated video.

Generate a video based on a given text.

> [!TIP]
> You can pass provider-specific parameters to the model by using the `extra_body` argument.

Example:

Example using a third-party provider directly. Usage will be billed on your fal.ai account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="fal-ai",  # Using fal.ai provider
...     api_key="fal-ai-api-key",  # Pass your fal.ai API key
... )
>>> video = client.text_to_video(
...     "A majestic lion running in a fantasy forest",
...     model="tencent/HunyuanVideo",
... )
>>> with open("lion.mp4", "wb") as file:
...     file.write(video)
```

Example using a third-party provider through Hugging Face Routing. Usage will be billed on your Hugging Face account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",  # Using replicate provider
...     api_key="hf_...",  # Pass your HF token
... )
>>> video = client.text_to_video(
...     "A cat running in a park",
...     model="genmo/mochi-1-preview",
... )
>>> with open("cat.mp4", "wb") as file:
...     file.write(video)
```

#### token_classification[[huggingface_hub.InferenceClient.token_classification]]

```python
token_classification(text: str, model: str | None = None, aggregation_strategy: typing.Optional[ForwardRef('TokenClassificationAggregationStrategy')] = None, ignore_labels: list[str] | None = None, stride: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L2892)

**Parameters:**

text (`str`) : A string to be classified.

model (`str`, *optional*) : The model to use for the token classification task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended token classification model will be used. Defaults to None.

aggregation_strategy (`"TokenClassificationAggregationStrategy"`, *optional*) : The strategy used to fuse tokens based on model predictions

ignore_labels (`list[str`, *optional*) : A list of labels to ignore

stride (`int`, *optional*) : The number of overlapping tokens between chunks when splitting the input text.

**Returns:** `list[TokenClassificationOutputElement]`

List of [TokenClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TokenClassificationOutputElement) items containing the entity group, confidence score, word, start and end index.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform token classification on the given text.
Usually used for sentence parsing, either grammatical, or Named Entity Recognition (NER) to understand keywords contained within text.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.token_classification("My name is Sarah Jessica Parker but you can call me Jessica")
[
    TokenClassificationOutputElement(
        entity_group='PER',
        score=0.9971321225166321,
        word='Sarah Jessica Parker',
        start=11,
        end=31,
    ),
    TokenClassificationOutputElement(
        entity_group='PER',
        score=0.9773476123809814,
        word='Jessica',
        start=52,
        end=59,
    )
]
```

#### translation[[huggingface_hub.InferenceClient.translation]]

```python
translation(text: str, model: str | None = None, src_lang: str | None = None, tgt_lang: str | None = None, clean_up_tokenization_spaces: bool | None = None, truncation: typing.Optional[ForwardRef('TranslationTruncationStrategy')] = None, generate_parameters: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L2967)

**Parameters:**

text (`str`) : A string to be translated.

model (`str`, *optional*) : The model to use for the translation task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended translation model will be used. Defaults to None.

src_lang (`str`, *optional*) : The source language of the text. Required for models that can translate from multiple languages.

tgt_lang (`str`, *optional*) : Target language to translate to. Required for models that can translate to multiple languages.

clean_up_tokenization_spaces (`bool`, *optional*) : Whether to clean up the potential extra spaces in the text output.

truncation (`"TranslationTruncationStrategy"`, *optional*) : The truncation strategy to use.

generate_parameters (`dict[str, Any]`, *optional*) : Additional parametrization of the text generation algorithm.

**Returns:** [TranslationOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TranslationOutput)

The generated translated text.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError` or ``ValueError``

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.
- ``ValueError`` -- 
  If only one of the `src_lang` and `tgt_lang` arguments are provided.

Convert text from one language to another.

Check out https://huggingface.co/tasks/translation for more information on how to choose the best model for
your specific use case. Source and target languages usually depend on the model.
However, it is possible to specify source and target languages for certain models. If you are working with one of these models,
you can use `src_lang` and `tgt_lang` arguments to pass the relevant information.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.translation("My name is Wolfgang and I live in Berlin")
'Mein Name ist Wolfgang und ich lebe in Berlin.'
>>> client.translation("My name is Wolfgang and I live in Berlin", model="Helsinki-NLP/opus-mt-en-fr")
TranslationOutput(translation_text='Je m'appelle Wolfgang et je vis à Berlin.')
```

Specifying languages:
```py
>>> client.translation("My name is Sarah Jessica Parker but you can call me Jessica", model="facebook/mbart-large-50-many-to-many-mmt", src_lang="en_XX", tgt_lang="fr_XX")
"Mon nom est Sarah Jessica Parker mais vous pouvez m'appeler Jessica"
```

#### visual_question_answering[[huggingface_hub.InferenceClient.visual_question_answering]]

```python
visual_question_answering(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], question: str, model: str | None = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L3056)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The input image for the context. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

question (`str`) : Question to be answered.

model (`str`, *optional*) : The model to use for the visual question answering task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended visual question answering model will be used. Defaults to None.

top_k (`int`, *optional*) : The number of answers to return (will be chosen by order of likelihood). Note that we return less than topk answers if there are not enough options available within the context.

**Returns:** `list[VisualQuestionAnsweringOutputElement]`

a list of [VisualQuestionAnsweringOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.VisualQuestionAnsweringOutputElement) items containing the predicted label and associated probability.

**Raises:** ``InferenceTimeoutError`` or `HfHubHTTPError`

- ``InferenceTimeoutError`` -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Answering open-ended questions based on an image.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.visual_question_answering(
...     image="https://huggingface.co/datasets/mishig/sample_images/resolve/main/tiger.jpg",
...     question="What is the animal doing?"
... )
[
    VisualQuestionAnsweringOutputElement(score=0.778609573841095, answer='laying down'),
    VisualQuestionAnsweringOutputElement(score=0.6957435607910156, answer='sitting'),
]
```

#### zero_shot_classification[[huggingface_hub.InferenceClient.zero_shot_classification]]

```python
zero_shot_classification(text: str, candidate_labels: list, multi_label: bool | None = False, hypothesis_template: str | None = None, model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L3115)

**Parameters:**

text (`str`) : The input text to classify.

candidate_labels (`list[str]`) : The set of possible class labels to classify the text into.

multi_label (`bool`, *optional*) : Whether multiple candidate labels can be true. If false, the scores are normalized such that the sum of the label likelihoods for each sequence is 1. If true, the labels are considered independent and probabilities are normalized for each candidate.

hypothesis_template (`str`, *optional*) : The sentence used in conjunction with `candidate_labels` to attempt the text classification by replacing the placeholder with the candidate labels.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. If not provided, the default recommended zero-shot classification model will be used.

**Returns:** `list[ZeroShotClassificationOutputElement]`

List of [ZeroShotClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ZeroShotClassificationOutputElement) items containing the predicted labels and their confidence.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Provide as input a text and a set of candidate labels to classify the input text.

Example with `multi_label=False`:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> text = (
...     "A new model offers an explanation for how the Galilean satellites formed around the solar system's"
...     "largest world. Konstantin Batygin did not set out to solve one of the solar system's most puzzling"
...     " mysteries when he went for a run up a hill in Nice, France."
... )
>>> labels = ["space & cosmos", "scientific discovery", "microbiology", "robots", "archeology"]
>>> client.zero_shot_classification(text, labels)
[
    ZeroShotClassificationOutputElement(label='scientific discovery', score=0.7961668968200684),
    ZeroShotClassificationOutputElement(label='space & cosmos', score=0.18570658564567566),
    ZeroShotClassificationOutputElement(label='microbiology', score=0.00730885099619627),
    ZeroShotClassificationOutputElement(label='archeology', score=0.006258360575884581),
    ZeroShotClassificationOutputElement(label='robots', score=0.004559356719255447),
]
>>> client.zero_shot_classification(text, labels, multi_label=True)
[
    ZeroShotClassificationOutputElement(label='scientific discovery', score=0.9829297661781311),
    ZeroShotClassificationOutputElement(label='space & cosmos', score=0.755190908908844),
    ZeroShotClassificationOutputElement(label='microbiology', score=0.0005462635890580714),
    ZeroShotClassificationOutputElement(label='archeology', score=0.00047131875180639327),
    ZeroShotClassificationOutputElement(label='robots', score=0.00030448526376858354),
]
```

Example with `multi_label=True` and a custom `hypothesis_template`:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()
>>> client.zero_shot_classification(
...    text="I really like our dinner and I'm very happy. I don't like the weather though.",
...    candidate_labels=["positive", "negative", "pessimistic", "optimistic"],
...    multi_label=True,
...    hypothesis_template="This text is {} towards the weather"
... )
[
    ZeroShotClassificationOutputElement(label='negative', score=0.9231801629066467),
    ZeroShotClassificationOutputElement(label='pessimistic', score=0.8760990500450134),
    ZeroShotClassificationOutputElement(label='optimistic', score=0.0008674879791215062),
    ZeroShotClassificationOutputElement(label='positive', score=0.0005250611575320363)
]
```

#### zero_shot_image_classification[[huggingface_hub.InferenceClient.zero_shot_image_classification]]

```python
zero_shot_image_classification(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], candidate_labels: list, model: str | None = None, hypothesis_template: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_client.py#L3216)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The input image to caption. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

candidate_labels (`list[str]`) : The candidate labels for this image

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. If not provided, the default recommended zero-shot image classification model will be used.

hypothesis_template (`str`, *optional*) : The sentence used in conjunction with `candidate_labels` to attempt the image classification by replacing the placeholder with the candidate labels.

**Returns:** `list[ZeroShotImageClassificationOutputElement]`

List of [ZeroShotImageClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ZeroShotImageClassificationOutputElement) items containing the predicted labels and their confidence.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Provide input image and text labels to predict text labels for the image.

Example:
```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient()

>>> client.zero_shot_image_classification(
...     "https://upload.wikimedia.org/wikipedia/commons/thumb/4/43/Cute_dog.jpg/320px-Cute_dog.jpg",
...     candidate_labels=["dog", "cat", "horse"],
... )
[ZeroShotImageClassificationOutputElement(label='dog', score=0.956),...]
```

## Async Inference Client[[huggingface_hub.AsyncInferenceClient]]

An async version of the client is also provided, based on `asyncio` and `httpx`.

#### huggingface_hub.AsyncInferenceClient[[huggingface_hub.AsyncInferenceClient]]

```python
huggingface_hub.AsyncInferenceClient(model: str | None = None, provider: typing.Union[typing.Literal['baseten', 'cerebras', 'cohere', 'deepinfra', 'fal-ai', 'featherless-ai', 'fireworks-ai', 'groq', 'hf-inference', 'novita', 'nscale', 'openai', 'ovhcloud', 'publicai', 'replicate', 'scaleway', 'together', 'wavespeed', 'zai-org'], typing.Literal['auto'], NoneType] = None, token: str | None = None, timeout: float | None = None, headers: dict[str, str] | None = None, cookies: dict[str, str] | None = None, bill_to: str | None = None, base_url: str | None = None, api_key: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L113)

**Parameters:**

model (`str`, `optional`) : The model to run inference with. Can be a model id hosted on the Hugging Face Hub, e.g. `meta-llama/Meta-Llama-3-8B-Instruct` or a URL to a deployed Inference Endpoint. Defaults to None, in which case a recommended model is automatically selected for the task. Note: for better compatibility with OpenAI's client, `model` has been aliased as `base_url`. Those 2 arguments are mutually exclusive. If a URL is passed as `model` or `base_url` for chat completion, the `(/v1)/chat/completions` suffix path will be appended to the URL.

provider (`str`, *optional*) : Name of the provider to use for inference. Can be `"baseten"`, `"cerebras"`, `"cohere"`, `"deepinfra"`, `"fal-ai"`, `"featherless-ai"`, `"fireworks-ai"`, `"groq"`, `"hf-inference"`, `"novita"`, `"nscale"`, `"openai"`, `"ovhcloud"`, `"publicai"`, `"replicate"`, `"scaleway"`, `"together"`, `"wavespeed"` or `"zai-org"`. Defaults to "auto": automatic routing, which defaults to "fastest" provider; you can switch to "cheapest" or "preferred" provider order at https://hf.co/settings/inference-providers. If model is a URL or `base_url` is passed, then `provider` is not used.

token (`str`, *optional*) : Hugging Face token. Will default to the locally saved token if not provided. Note: for better compatibility with OpenAI's client, `token` has been aliased as `api_key`. Those 2 arguments are mutually exclusive and have the exact same behavior.

timeout (`float`, `optional`) : The maximum number of seconds to wait for a response from the server. Defaults to None, meaning it will loop until the server is available.

headers (`dict[str, str]`, `optional`) : Additional headers to send to the server. By default only the authorization and user-agent headers are sent. Values in this dictionary will override the default values.

bill_to (`str`, `optional`) : The billing account to use for the requests. By default the requests are billed on the user's account. Requests can only be billed to an organization the user is a member of, and which has subscribed to Enterprise Hub.

cookies (`dict[str, str]`, `optional`) : Additional cookies to send to the server.

base_url (`str`, `optional`) : Base URL to run inference. This is a duplicated argument from `model` to make [InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient) follow the same pattern as `openai.OpenAI` client. Cannot be used if `model` is set. Defaults to None.

api_key (`str`, `optional`) : Token to use for authentication. This is a duplicated argument from `token` to make [InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient) follow the same pattern as `openai.OpenAI` client. Cannot be used if `token` is set. Defaults to None.

Initialize a new Inference Client.

[InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient) aims to provide a unified experience to perform inference. The client can be used
seamlessly with either the (free) Inference API, self-hosted Inference Endpoints, or third-party Inference Providers.

#### audio_classification[[huggingface_hub.AsyncInferenceClient.audio_classification]]

```python
audio_classification(audio: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, top_k: int | None = None, function_to_apply: typing.Optional[ForwardRef('AudioClassificationOutputTransform')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L327)

**Parameters:**

audio (Union[str, Path, bytes, BinaryIO]) : The audio content to classify. It can be raw audio bytes, a local audio file, or a URL pointing to an audio file.

model (`str`, *optional*) : The model to use for audio classification. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for audio classification will be used.

top_k (`int`, *optional*) : When specified, limits the output to the top K most probable classes.

function_to_apply (`"AudioClassificationOutputTransform"`, *optional*) : The function to apply to the model outputs in order to retrieve the scores.

**Returns:** `list[AudioClassificationOutputElement]`

List of [AudioClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.AudioClassificationOutputElement) items containing the predicted labels and their confidence.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform audio classification on the provided audio content.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.audio_classification("audio.flac")
[
    AudioClassificationOutputElement(score=0.4976358711719513, label='hap'),
    AudioClassificationOutputElement(score=0.3677836060523987, label='neu'),
    ...
]
```

#### audio_to_audio[[huggingface_hub.AsyncInferenceClient.audio_to_audio]]

```python
audio_to_audio(audio: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L385)

**Parameters:**

audio (Union[str, Path, bytes, BinaryIO]) : The audio content for the model. It can be raw audio bytes, a local audio file, or a URL pointing to an audio file.

model (`str`, *optional*) : The model can be any model which takes an audio file and returns another audio file. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for audio_to_audio will be used.

**Returns:** `list[AudioToAudioOutputElement]`

A list of [AudioToAudioOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.AudioToAudioOutputElement) items containing audios label, content-type, and audio content in blob.

**Raises:** ``InferenceTimeoutError`` or `HfHubHTTPError`

- ``InferenceTimeoutError`` -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Performs multiple tasks related to audio-to-audio depending on the model (eg: speech enhancement, source separation).

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> audio_output = await client.audio_to_audio("audio.flac")
>>> async for i, item in enumerate(audio_output):
>>>     with open(f"output_{i}.flac", "wb") as f:
            f.write(item.blob)
```

#### automatic_speech_recognition[[huggingface_hub.AsyncInferenceClient.automatic_speech_recognition]]

```python
automatic_speech_recognition(audio: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, extra_body: dict | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L438)

**Parameters:**

audio (Union[str, Path, bytes, BinaryIO]) : The content to transcribe. It can be raw audio bytes, local audio file, or a URL to an audio file.

model (`str`, *optional*) : The model to use for ASR. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for ASR will be used.

extra_body (`dict`, *optional*) : Additional provider-specific parameters to pass to the model. Refer to the provider's documentation for supported parameters.

**Returns:** [AutomaticSpeechRecognitionOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.AutomaticSpeechRecognitionOutput)

An item containing the transcribed text and optionally the timestamp chunks.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform automatic speech recognition (ASR or audio-to-text) on the given audio content.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.automatic_speech_recognition("hello_world.flac").text
"hello world"
```

#### chat_completion[[huggingface_hub.AsyncInferenceClient.chat_completion]]

```python
chat_completion(messages: list, model: str | None = None, stream: bool = False, frequency_penalty: float | None = None, logit_bias: list[float] | None = None, logprobs: bool | None = None, max_tokens: int | None = None, n: int | None = None, presence_penalty: float | None = None, response_format: typing.Union[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputResponseFormatText, huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputResponseFormatJSONSchema, huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputResponseFormatJSONObject, NoneType] = None, seed: int | None = None, stop: list[str] | None = None, stream_options: huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputStreamOptions | None = None, temperature: float | None = None, tool_choice: typing.Union[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputToolChoiceClass, ForwardRef('ChatCompletionInputToolChoiceEnum'), NoneType] = None, tool_prompt: str | None = None, tools: list[huggingface_hub.inference._generated.types.chat_completion.ChatCompletionInputTool] | None = None, top_logprobs: int | None = None, top_p: float | None = None, extra_body: dict | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L566)

**Parameters:**

messages (List of [ChatCompletionInputMessage](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionInputMessage)) : Conversation history consisting of roles and content pairs.

model (`str`, *optional*) : The model to use for chat-completion. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for chat-based text-generation will be used. See https://huggingface.co/tasks/text-generation for more details. If `model` is a model ID, it is passed to the server as the `model` parameter. If you want to define a custom URL while setting `model` in the request payload, you must set `base_url` when initializing [InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient).

frequency_penalty (`float`, *optional*) : Penalizes new tokens based on their existing frequency in the text so far. Range: [-2.0, 2.0]. Defaults to 0.0.

logit_bias (`list[float]`, *optional*) : Adjusts the likelihood of specific tokens appearing in the generated output.

logprobs (`bool`, *optional*) : Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the content of message.

max_tokens (`int`, *optional*) : Maximum number of tokens allowed in the response. Defaults to 100.

n (`int`, *optional*) : The number of completions to generate for each prompt.

presence_penalty (`float`, *optional*) : Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.

response_format (`ChatCompletionInputGrammarType()`, *optional*) : Grammar constraints. Can be either a JSONSchema or a regex.

seed (Optional`int`, *optional*) : Seed for reproducible control flow. Defaults to None.

stop (`list[str]`, *optional*) : Up to four strings which trigger the end of the response. Defaults to None.

stream (`bool`, *optional*) : Enable realtime streaming of responses. Defaults to False.

stream_options ([ChatCompletionInputStreamOptions](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionInputStreamOptions), *optional*) : Options for streaming completions.

temperature (`float`, *optional*) : Controls randomness of the generations. Lower values ensure less random completions. Range: [0, 2]. Defaults to 1.0.

top_logprobs (`int`, *optional*) : An integer between 0 and 5 specifying the number of most likely tokens to return at each token position, each with an associated log probability. logprobs must be set to true if this parameter is used.

top_p (`float`, *optional*) : Fraction of the most likely next words to sample from. Must be between 0 and 1. Defaults to 1.0.

tool_choice ([ChatCompletionInputToolChoiceClass](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionInputToolChoiceClass) or `ChatCompletionInputToolChoiceEnum()`, *optional*) : The tool to use for the completion. Defaults to "auto".

tool_prompt (`str`, *optional*) : A prompt to be appended before the tools.

tools (List of [ChatCompletionInputTool](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionInputTool), *optional*) : A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for.

extra_body (`dict`, *optional*) : Additional provider-specific parameters to pass to the model. Refer to the provider's documentation for supported parameters.

**Returns:** [ChatCompletionOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionOutput) or Iterable of [ChatCompletionStreamOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionStreamOutput)

Generated text returned from the server:
- if `stream=False`, the generated text is returned as a [ChatCompletionOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionOutput) (default).
- if `stream=True`, the generated text is returned token by token as a sequence of [ChatCompletionStreamOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ChatCompletionStreamOutput).

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

A method for completing conversations using a specified language model.

> [!TIP]
> The `client.chat_completion` method is aliased as `client.chat.completions.create` for compatibility with OpenAI's client.
> Inputs and outputs are strictly the same and using either syntax will yield the same results.
> Check out the [Inference guide](https://huggingface.co/docs/huggingface_hub/guides/inference#openai-compatibility)
> for more details about OpenAI's compatibility.

> [!TIP]
> You can pass provider-specific parameters to the model by using the `extra_body` argument.

Example:

```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> messages = [{"role": "user", "content": "What is the capital of France?"}]
>>> client = AsyncInferenceClient("meta-llama/Meta-Llama-3-8B-Instruct")
>>> await client.chat_completion(messages, max_tokens=100)
ChatCompletionOutput(
    choices=[
        ChatCompletionOutputComplete(
            finish_reason='eos_token',
            index=0,
            message=ChatCompletionOutputMessage(
                role='assistant',
                content='The capital of France is Paris.',
                name=None,
                tool_calls=None
            ),
            logprobs=None
        )
    ],
    created=1719907176,
    id='',
    model='meta-llama/Meta-Llama-3-8B-Instruct',
    object='text_completion',
    system_fingerprint='2.0.4-sha-f426a33',
    usage=ChatCompletionOutputUsage(
        completion_tokens=8,
        prompt_tokens=17,
        total_tokens=25
    )
)
```

Example using streaming:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> messages = [{"role": "user", "content": "What is the capital of France?"}]
>>> client = AsyncInferenceClient("meta-llama/Meta-Llama-3-8B-Instruct")
>>> async for token in await client.chat_completion(messages, max_tokens=10, stream=True):
...     print(token)
ChatCompletionStreamOutput(choices=[ChatCompletionStreamOutputChoice(delta=ChatCompletionStreamOutputDelta(content='The', role='assistant'), index=0, finish_reason=None)], created=1710498504)
ChatCompletionStreamOutput(choices=[ChatCompletionStreamOutputChoice(delta=ChatCompletionStreamOutputDelta(content=' capital', role='assistant'), index=0, finish_reason=None)], created=1710498504)
(...)
ChatCompletionStreamOutput(choices=[ChatCompletionStreamOutputChoice(delta=ChatCompletionStreamOutputDelta(content=' may', role='assistant'), index=0, finish_reason=None)], created=1710498504)
```

Example using OpenAI's syntax:
```py
# Must be run in an async context
# instead of `from openai import OpenAI`
from huggingface_hub import AsyncInferenceClient

# instead of `client = OpenAI(...)`
client = AsyncInferenceClient(
    base_url=...,
    api_key=...,
)

output = await client.chat.completions.create(
    model="meta-llama/Meta-Llama-3-8B-Instruct",
    messages=[
        {"role": "system", "content": "You are a helpful assistant."},
        {"role": "user", "content": "Count to 10"},
    ],
    stream=True,
    max_tokens=1024,
)

for chunk in output:
    print(chunk.choices[0].delta.content)
```

Example using a third-party provider directly with extra (provider-specific) parameters. Usage will be billed on your Together AI account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="together",  # Use Together AI provider
...     api_key="<together_api_key>",  # Pass your Together API key directly
... )
>>> client.chat_completion(
...     model="meta-llama/Meta-Llama-3-8B-Instruct",
...     messages=[{"role": "user", "content": "What is the capital of France?"}],
...     extra_body={"safety_model": "Meta-Llama/Llama-Guard-7b"},
... )
```

Example using a third-party provider through Hugging Face Routing. Usage will be billed on your Hugging Face account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="novita",  # Use Novita provider
...     api_key="hf_...",  # Pass your HF token
... )
>>> client.chat_completion(
...     model="meta-llama/Meta-Llama-3-8B-Instruct",
...     messages=[{"role": "user", "content": "What is the capital of France?"}],
... )
```

Example using Image + Text as input:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient

# provide a remote URL
>>> image_url ="https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg"
# or a base64-encoded image
>>> image_path = "/path/to/image.jpeg"
>>> with open(image_path, "rb") as f:
...     base64_image = base64.b64encode(f.read()).decode("utf-8")
>>> image_url = f"data:image/jpeg;base64,{base64_image}"

>>> client = AsyncInferenceClient("meta-llama/Llama-3.2-11B-Vision-Instruct")
>>> output = await client.chat.completions.create(
...     messages=[
...         {
...             "role": "user",
...             "content": [
...                 {
...                     "type": "image_url",
...                     "image_url": {"url": image_url},
...                 },
...                 {
...                     "type": "text",
...                     "text": "Describe this image in one sentence.",
...                 },
...             ],
...         },
...     ],
... )
>>> output
The image depicts the iconic Statue of Liberty situated in New York Harbor, New York, on a clear day.
```

Example using tools:
```py
# Must be run in an async context
>>> client = AsyncInferenceClient("meta-llama/Meta-Llama-3-70B-Instruct")
>>> messages = [
...     {
...         "role": "system",
...         "content": "Don't make assumptions about what values to plug into functions. Ask for clarification if a user request is ambiguous.",
...     },
...     {
...         "role": "user",
...         "content": "What's the weather like the next 3 days in San Francisco, CA?",
...     },
... ]
>>> tools = [
...     {
...         "type": "function",
...         "function": {
...             "name": "get_current_weather",
...             "description": "Get the current weather",
...             "parameters": {
...                 "type": "object",
...                 "properties": {
...                     "location": {
...                         "type": "string",
...                         "description": "The city and state, e.g. San Francisco, CA",
...                     },
...                     "format": {
...                         "type": "string",
...                         "enum": ["celsius", "fahrenheit"],
...                         "description": "The temperature unit to use. Infer this from the users location.",
...                     },
...                 },
...                 "required": ["location", "format"],
...             },
...         },
...     },
...     {
...         "type": "function",
...         "function": {
...             "name": "get_n_day_weather_forecast",
...             "description": "Get an N-day weather forecast",
...             "parameters": {
...                 "type": "object",
...                 "properties": {
...                     "location": {
...                         "type": "string",
...                         "description": "The city and state, e.g. San Francisco, CA",
...                     },
...                     "format": {
...                         "type": "string",
...                         "enum": ["celsius", "fahrenheit"],
...                         "description": "The temperature unit to use. Infer this from the users location.",
...                     },
...                     "num_days": {
...                         "type": "integer",
...                         "description": "The number of days to forecast",
...                     },
...                 },
...                 "required": ["location", "format", "num_days"],
...             },
...         },
...     },
... ]

>>> response = await client.chat_completion(
...     model="meta-llama/Meta-Llama-3-70B-Instruct",
...     messages=messages,
...     tools=tools,
...     tool_choice="auto",
...     max_tokens=500,
... )
>>> response.choices[0].message.tool_calls[0].function
ChatCompletionOutputFunctionDefinition(
    arguments={
        'location': 'San Francisco, CA',
        'format': 'fahrenheit',
        'num_days': 3
    },
    name='get_n_day_weather_forecast',
    description=None
)
```

Example using response_format:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient("meta-llama/Meta-Llama-3-70B-Instruct")
>>> messages = [
...     {
...         "role": "user",
...         "content": "I saw a puppy a cat and a raccoon during my bike ride in the park. What did I see and when?",
...     },
... ]
>>> response_format = {
...     "type": "json",
...     "value": {
...         "properties": {
...             "location": {"type": "string"},
...             "activity": {"type": "string"},
...             "animals_seen": {"type": "integer", "minimum": 1, "maximum": 5},
...             "animals": {"type": "array", "items": {"type": "string"}},
...         },
...         "required": ["location", "activity", "animals_seen", "animals"],
...     },
... }
>>> response = await client.chat_completion(
...     messages=messages,
...     response_format=response_format,
...     max_tokens=500,
... )
>>> response.choices[0].message.content
'{

y": "bike ride",
": ["puppy", "cat", "raccoon"],
_seen": 3,
n": "park"}'
```

#### close[[huggingface_hub.AsyncInferenceClient.close]]

```python
close()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L240)

Close the client.

This method is automatically called when using the client as a context manager.

#### document_question_answering[[huggingface_hub.AsyncInferenceClient.document_question_answering]]

```python
document_question_answering(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], question: str, model: str | None = None, doc_stride: int | None = None, handle_impossible_answer: bool | None = None, lang: str | None = None, max_answer_len: int | None = None, max_question_len: int | None = None, max_seq_len: int | None = None, top_k: int | None = None, word_boxes: list[list[float] | str] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L974)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO]`) : The input image for the context. It can be raw bytes, an image file, or a URL to an online image.

question (`str`) : Question to be answered.

model (`str`, *optional*) : The model to use for the document question answering task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended document question answering model will be used. Defaults to None.

doc_stride (`int`, *optional*) : If the words in the document are too long to fit with the question for the model, it will be split in several chunks with some overlap. This argument controls the size of that overlap.

handle_impossible_answer (`bool`, *optional*) : Whether to accept impossible as an answer

lang (`str`, *optional*) : Language to use while running OCR. Defaults to english.

max_answer_len (`int`, *optional*) : The maximum length of predicted answers (e.g., only answers with a shorter length are considered).

max_question_len (`int`, *optional*) : The maximum length of the question after tokenization. It will be truncated if needed.

max_seq_len (`int`, *optional*) : The maximum length of the total sentence (context + question) in tokens of each chunk passed to the model. The context will be split in several chunks (using doc_stride as overlap) if needed.

top_k (`int`, *optional*) : The number of answers to return (will be chosen by order of likelihood). Can return less than top_k answers if there are not enough options available within the context.

word_boxes (`list[Union[list[float], str`, *optional*) : A list of words and bounding boxes (normalized 0->1000). If provided, the inference will skip the OCR step and use the provided bounding boxes instead.

**Returns:** `list[DocumentQuestionAnsweringOutputElement]`

a list of [DocumentQuestionAnsweringOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.DocumentQuestionAnsweringOutputElement) items containing the predicted label, associated probability, word ids, and page number.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Answer questions on document images.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.document_question_answering(image="https://huggingface.co/spaces/impira/docquery/resolve/2359223c1837a7587402bda0f2643382a6eefeab/invoice.png", question="What is the invoice number?")
[DocumentQuestionAnsweringOutputElement(answer='us-001', end=16, score=0.9999666213989258, start=16)]
```

#### feature_extraction[[huggingface_hub.AsyncInferenceClient.feature_extraction]]

```python
feature_extraction(text: str | list[str], normalize: bool | None = None, prompt_name: str | None = None, truncate: bool | None = None, truncation_direction: typing.Optional[typing.Literal['left', 'right']] = None, dimensions: int | None = None, encoding_format: typing.Optional[typing.Literal['float', 'base64']] = None, model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1062)

**Parameters:**

text (*str* or *list[str]*) : The text or list of texts to embed.

model (*str*, *optional*) : The model to use for the feature extraction task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended feature extraction model will be used. Defaults to None.

normalize (*bool*, *optional*) : Whether to normalize the embeddings or not. Only available on server powered by Text-Embedding-Inference.

prompt_name (*str*, *optional*) : The name of the prompt that should be used by for encoding. If not set, no prompt will be applied. Must be a key in the *Sentence Transformers* configuration *prompts* dictionary. For example if `prompt_name` is "query" and the `prompts` is &amp;lcub;"query": "query: ",...}, then the sentence "What is the capital of France?" will be encoded as "query: What is the capital of France?" because the prompt text will be prepended before any text to encode.

truncate (*bool*, *optional*) : Whether to truncate the embeddings or not. Only available on server powered by Text-Embedding-Inference.

truncation_direction (*Literal["left", "right"]*, *optional*) : Which side of the input should be truncated when *truncate=True* is passed.

dimensions (*int*, *optional*) : The number of dimensions the resulting output embeddings should have. Only available on OpenAI-compatible embedding endpoints.

encoding_format (*Literal["float", "base64"]*, *optional*) : The format of the output embeddings. Either "float" or "base64". Only available on OpenAI-compatible embedding endpoints.

**Returns:** `*np.ndarray*`

The embedding representing the input text(s) as a float32 numpy array.

**Raises:** [*InferenceTimeoutError*] or [*HfHubHTTPError*]

- [*InferenceTimeoutError*] -- 
  If the model is unavailable or the request times out.
- [*HfHubHTTPError*] -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Generate embeddings for a given text or batch of texts.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.feature_extraction("Hi, who are you?")
array([[ 2.424802  ,  2.93384   ,  1.1750331 , ...,  1.240499, -0.13776633, -0.7889173 ],
[-0.42943227, -0.6364878 , -1.693462  , ...,  0.41978157, -2.4336355 ,  0.6162071 ],
...,
[ 0.28552425, -0.928395  , -1.2077185 , ...,  0.76810825, -2.1069427 ,  0.6236161 ]], dtype=float32)
```

#### fill_mask[[huggingface_hub.AsyncInferenceClient.fill_mask]]

```python
fill_mask(text: str, model: str | None = None, targets: list[str] | None = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1146)

**Parameters:**

text (`str`) : a string to be filled from, must contain the [MASK] token (check model card for exact name of the mask).

model (`str`, *optional*) : The model to use for the fill mask task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended fill mask model will be used.

targets (`list[str`, *optional*) : When passed, the model will limit the scores to the passed targets instead of looking up in the whole vocabulary. If the provided targets are not in the model vocab, they will be tokenized and the first resulting token will be used (with a warning, and that might be slower).

top_k (`int`, *optional*) : When passed, overrides the number of predictions to return.

**Returns:** `list[FillMaskOutputElement]`

a list of [FillMaskOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.FillMaskOutputElement) items containing the predicted label, associated
probability, token reference, and completed text.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Fill in a hole with a missing word (token to be precise).

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.fill_mask("The goal of life is <mask>.")
[
    FillMaskOutputElement(score=0.06897063553333282, token=11098, token_str=' happiness', sequence='The goal of life is happiness.'),
    FillMaskOutputElement(score=0.06554922461509705, token=45075, token_str=' immortality', sequence='The goal of life is immortality.')
]
```

#### get_endpoint_info[[huggingface_hub.AsyncInferenceClient.get_endpoint_info]]

```python
get_endpoint_info(model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L3333)

**Parameters:**

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

**Returns:** `dict[str, Any]`

Information about the endpoint.

Get information about the deployed endpoint.

This endpoint is only available on endpoints powered by Text-Generation-Inference (TGI) or Text-Embedding-Inference (TEI).
Endpoints powered by `transformers` return an empty payload.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient("meta-llama/Meta-Llama-3-70B-Instruct")
>>> await client.get_endpoint_info()
{
    'model_id': 'meta-llama/Meta-Llama-3-70B-Instruct',
    'model_sha': None,
    'model_dtype': 'torch.float16',
    'model_device_type': 'cuda',
    'model_pipeline_tag': None,
    'max_concurrent_requests': 128,
    'max_best_of': 2,
    'max_stop_sequences': 4,
    'max_input_length': 8191,
    'max_total_tokens': 8192,
    'waiting_served_ratio': 0.3,
    'max_batch_total_tokens': 1259392,
    'max_waiting_tokens': 20,
    'max_batch_size': None,
    'validation_workers': 32,
    'max_client_batch_size': 4,
    'version': '2.0.2',
    'sha': 'dccab72549635c7eb5ddb17f43f0b7cdff07c214',
    'docker_label': 'sha-dccab72'
}
```

#### health_check[[huggingface_hub.AsyncInferenceClient.health_check]]

```python
health_check(model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L3393)

**Parameters:**

model (`str`, *optional*) : URL of the Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

**Returns:** `bool`

True if everything is working fine.

Check the health of the deployed endpoint.

Health check is only available with Inference Endpoints powered by Text-Generation-Inference (TGI) or Text-Embedding-Inference (TEI).

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient("https://jzgu0buei5.us-east-1.aws.endpoints.huggingface.cloud")
>>> await client.health_check()
True
```

#### image_classification[[huggingface_hub.AsyncInferenceClient.image_classification]]

```python
image_classification(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, function_to_apply: typing.Optional[ForwardRef('ImageClassificationOutputTransform')] = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1203)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The image to classify. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

model (`str`, *optional*) : The model to use for image classification. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for image classification will be used.

function_to_apply (`"ImageClassificationOutputTransform"`, *optional*) : The function to apply to the model outputs in order to retrieve the scores.

top_k (`int`, *optional*) : When specified, limits the output to the top K most probable classes.

**Returns:** `list[ImageClassificationOutputElement]`

a list of [ImageClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ImageClassificationOutputElement) items containing the predicted label and associated probability.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform image classification on the given image using the specified model.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.image_classification("https://upload.wikimedia.org/wikipedia/commons/thumb/4/43/Cute_dog.jpg/320px-Cute_dog.jpg")
[ImageClassificationOutputElement(label='Blenheim spaniel', score=0.9779096841812134), ...]
```

#### image_segmentation[[huggingface_hub.AsyncInferenceClient.image_segmentation]]

```python
image_segmentation(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, mask_threshold: float | None = None, overlap_mask_area_threshold: float | None = None, subtask: typing.Optional[ForwardRef('ImageSegmentationSubtask')] = None, threshold: float | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1254)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The image to segment. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

model (`str`, *optional*) : The model to use for image segmentation. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for image segmentation will be used.

mask_threshold (`float`, *optional*) : Threshold to use when turning the predicted masks into binary values.

overlap_mask_area_threshold (`float`, *optional*) : Mask overlap threshold to eliminate small, disconnected segments.

subtask (`"ImageSegmentationSubtask"`, *optional*) : Segmentation task to be performed, depending on model capabilities.

threshold (`float`, *optional*) : Probability threshold to filter out predicted masks.

**Returns:** `list[ImageSegmentationOutputElement]`

A list of [ImageSegmentationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ImageSegmentationOutputElement) items containing the segmented masks and associated attributes.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform image segmentation on the given image using the specified model.

> [!WARNING]
> You must have `PIL` installed if you want to work with images (`pip install Pillow`).

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.image_segmentation("cat.jpg")
[ImageSegmentationOutputElement(score=0.989008, label='LABEL_184', mask=<PIL.PngImagePlugin.PngImageFile image mode=L size=400x300 at 0x7FDD2B129CC0>), ...]
```

#### image_to_image[[huggingface_hub.AsyncInferenceClient.image_to_image]]

```python
image_to_image(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], prompt: str | None = None, negative_prompt: str | None = None, num_inference_steps: int | None = None, guidance_scale: float | None = None, model: str | None = None, target_size: huggingface_hub.inference._generated.types.image_to_image.ImageToImageTargetSize | None = None, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1323)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The input image for translation. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

prompt (`str`, *optional*) : The text prompt to guide the image generation.

negative_prompt (`str`, *optional*) : One prompt to guide what NOT to include in image generation.

num_inference_steps (`int`, *optional*) : For diffusion models. The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*) : For diffusion models. A higher guidance scale value encourages the model to generate images closely linked to the text prompt at the expense of lower image quality.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

target_size (`ImageToImageTargetSize`, *optional*) : The size in pixels of the output image. This parameter is only supported by some providers and for specific models. It will be ignored when unsupported.

**Returns:** `Image`

The translated image.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform image-to-image translation using a specified model.

> [!WARNING]
> You must have `PIL` installed if you want to work with images (`pip install Pillow`).

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> image = await client.image_to_image("cat.jpg", prompt="turn the cat into a tiger")
>>> image.save("tiger.jpg")
```

#### image_to_text[[huggingface_hub.AsyncInferenceClient.image_to_text]]

```python
image_to_text(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1480)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The input image to caption. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

**Returns:** [ImageToTextOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ImageToTextOutput)

The generated text.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Takes an input image and return text.

Models can have very different outputs depending on your use case (image captioning, optical character recognition
(OCR), Pix2Struct, etc.). Please have a look to the model card to learn more about a model's specificities.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.image_to_text("cat.jpg")
'a cat standing in a grassy field '
>>> await client.image_to_text("https://upload.wikimedia.org/wikipedia/commons/thumb/4/43/Cute_dog.jpg/320px-Cute_dog.jpg")
'a dog laying on the grass next to a flower pot '
```

#### image_to_video[[huggingface_hub.AsyncInferenceClient.image_to_video]]

```python
image_to_video(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, prompt: str | None = None, negative_prompt: str | None = None, num_frames: float | None = None, num_inference_steps: int | None = None, guidance_scale: float | None = None, seed: int | None = None, target_size: huggingface_hub.inference._generated.types.image_to_video.ImageToVideoTargetSize | None = None, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1400)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The input image to generate a video from. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

prompt (`str`, *optional*) : The text prompt to guide the video generation.

negative_prompt (`str`, *optional*) : One prompt to guide what NOT to include in video generation.

num_frames (`float`, *optional*) : The num_frames parameter determines how many video frames are generated.

num_inference_steps (`int`, *optional*) : For diffusion models. The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*) : For diffusion models. A higher guidance scale value encourages the model to generate videos closely linked to the text prompt at the expense of lower image quality.

seed (`int`, *optional*) : The seed to use for the video generation.

target_size (`ImageToVideoTargetSize`, *optional*) : The size in pixel of the output video frames.

num_inference_steps (`int`, *optional*) : The number of denoising steps. More denoising steps usually lead to a higher quality video at the expense of slower inference.

seed (`int`, *optional*) : Seed for the random number generator.

**Returns:** `bytes`

The generated video.

Generate a video from an input image.

Examples:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> video = await client.image_to_video("cat.jpg", model="Wan-AI/Wan2.2-I2V-A14B", prompt="turn the cat into a tiger")
>>> with open("tiger.mp4", "wb") as f:
...     f.write(video)
```

#### object_detection[[huggingface_hub.AsyncInferenceClient.object_detection]]

```python
object_detection(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], model: str | None = None, threshold: float | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1527)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The image to detect objects on. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

model (`str`, *optional*) : The model to use for object detection. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for object detection (DETR) will be used.

threshold (`float`, *optional*) : The probability necessary to make a prediction.

**Returns:** `list[ObjectDetectionOutputElement]`

A list of [ObjectDetectionOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ObjectDetectionOutputElement) items containing the bounding boxes and associated attributes.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError` or ``ValueError``

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.
- ``ValueError`` -- 
  If the request output is not a List.

Perform object detection on the given image using the specified model.

> [!WARNING]
> You must have `PIL` installed if you want to work with images (`pip install Pillow`).

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.object_detection("people.jpg")
[ObjectDetectionOutputElement(score=0.9486683011054993, label='person', box=ObjectDetectionBoundingBox(xmin=59, ymin=39, xmax=420, ymax=510)), ...]
```

#### question_answering[[huggingface_hub.AsyncInferenceClient.question_answering]]

```python
question_answering(question: str, context: str, model: str | None = None, align_to_words: bool | None = None, doc_stride: int | None = None, handle_impossible_answer: bool | None = None, max_answer_len: int | None = None, max_question_len: int | None = None, max_seq_len: int | None = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1576)

**Parameters:**

question (`str`) : Question to be answered.

context (`str`) : The context of the question.

model (`str`) : The model to use for the question answering task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint.

align_to_words (`bool`, *optional*) : Attempts to align the answer to real words. Improves quality on space separated languages. Might hurt on non-space-separated languages (like Japanese or Chinese)

doc_stride (`int`, *optional*) : If the context is too long to fit with the question for the model, it will be split in several chunks with some overlap. This argument controls the size of that overlap.

handle_impossible_answer (`bool`, *optional*) : Whether to accept impossible as an answer.

max_answer_len (`int`, *optional*) : The maximum length of predicted answers (e.g., only answers with a shorter length are considered).

max_question_len (`int`, *optional*) : The maximum length of the question after tokenization. It will be truncated if needed.

max_seq_len (`int`, *optional*) : The maximum length of the total sentence (context + question) in tokens of each chunk passed to the model. The context will be split in several chunks (using docStride as overlap) if needed.

top_k (`int`, *optional*) : The number of answers to return (will be chosen by order of likelihood). Note that we return less than topk answers if there are not enough options available within the context.

**Returns:** Union[`QuestionAnsweringOutputElement`, list[QuestionAnsweringOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.QuestionAnsweringOutputElement)]

When top_k is 1 or not provided, it returns a single `QuestionAnsweringOutputElement`.
When top_k is greater than 1, it returns a list of `QuestionAnsweringOutputElement`.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Retrieve the answer to a question from a given text.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.question_answering(question="What's my name?", context="My name is Clara and I live in Berkeley.")
QuestionAnsweringOutputElement(answer='Clara', end=16, score=0.9326565265655518, start=11)
```

#### sentence_similarity[[huggingface_hub.AsyncInferenceClient.sentence_similarity]]

```python
sentence_similarity(sentence: str, other_sentences: list, model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1661)

**Parameters:**

sentence (`str`) : The main sentence to compare to others.

other_sentences (`list[str]`) : The list of sentences to compare to.

model (`str`, *optional*) : The model to use for the sentence similarity task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended sentence similarity model will be used. Defaults to None.

**Returns:** `list[float]`

The embedding representing the input text.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Compute the semantic similarity between a sentence and a list of other sentences by comparing their embeddings.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.sentence_similarity(
...     "Machine learning is so easy.",
...     other_sentences=[
...         "Deep learning is so straightforward.",
...         "This is so difficult, like rocket science.",
...         "I can't believe how much I struggled with this.",
...     ],
... )
[0.7785726189613342, 0.45876261591911316, 0.2906220555305481]
```

#### summarization[[huggingface_hub.AsyncInferenceClient.summarization]]

```python
summarization(text: str, model: str | None = None, clean_up_tokenization_spaces: bool | None = None, generate_parameters: dict[str, typing.Any] | None = None, truncation: typing.Optional[ForwardRef('SummarizationTruncationStrategy')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1715)

**Parameters:**

text (`str`) : The input text to summarize.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended model for summarization will be used.

clean_up_tokenization_spaces (`bool`, *optional*) : Whether to clean up the potential extra spaces in the text output.

generate_parameters (`dict[str, Any]`, *optional*) : Additional parametrization of the text generation algorithm.

truncation (`"SummarizationTruncationStrategy"`, *optional*) : The truncation strategy to use.

**Returns:** [SummarizationOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.SummarizationOutput)

The generated summary text.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Generate a summary of a given text using a specified model.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.summarization("The Eiffel tower...")
SummarizationOutput(generated_text="The Eiffel tower is one of the most famous landmarks in the world....")
```

#### table_question_answering[[huggingface_hub.AsyncInferenceClient.table_question_answering]]

```python
table_question_answering(table: dict, query: str, model: str | None = None, padding: typing.Optional[ForwardRef('Padding')] = None, sequential: bool | None = None, truncation: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1774)

**Parameters:**

table (`str`) : A table of data represented as a dict of lists where entries are headers and the lists are all the values, all lists must have the same size.

query (`str`) : The query in plain text that you want to ask the table.

model (`str`) : The model to use for the table-question-answering task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint.

padding (`"Padding"`, *optional*) : Activates and controls padding.

sequential (`bool`, *optional*) : Whether to do inference sequentially or as a batch. Batching is faster, but models like SQA require the inference to be done sequentially to extract relations within sequences, given their conversational nature.

truncation (`bool`, *optional*) : Activates and controls truncation.

**Returns:** [TableQuestionAnsweringOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TableQuestionAnsweringOutputElement)

a table question answering output containing the answer, coordinates, cells and the aggregator used.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Retrieve the answer to a question from information given in a table.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> query = "How many stars does the transformers repository have?"
>>> table = {"Repository": ["Transformers", "Datasets", "Tokenizers"], "Stars": ["36542", "4512", "3934"]}
>>> await client.table_question_answering(table, query, model="google/tapas-base-finetuned-wtq")
TableQuestionAnsweringOutputElement(answer='36542', coordinates=[[0, 1]], cells=['36542'], aggregator='AVERAGE')
```

#### tabular_classification[[huggingface_hub.AsyncInferenceClient.tabular_classification]]

```python
tabular_classification(table: dict, model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1837)

**Parameters:**

table (`dict[str, Any]`) : Set of attributes to classify.

model (`str`, *optional*) : The model to use for the tabular classification task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended tabular classification model will be used. Defaults to None.

**Returns:** `List`

a list of labels, one per row in the initial table.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Classifying a target category (a group) based on a set of attributes.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> table = {
...     "fixed_acidity": ["7.4", "7.8", "10.3"],
...     "volatile_acidity": ["0.7", "0.88", "0.32"],
...     "citric_acid": ["0", "0", "0.45"],
...     "residual_sugar": ["1.9", "2.6", "6.4"],
...     "chlorides": ["0.076", "0.098", "0.073"],
...     "free_sulfur_dioxide": ["11", "25", "5"],
...     "total_sulfur_dioxide": ["34", "67", "13"],
...     "density": ["0.9978", "0.9968", "0.9976"],
...     "pH": ["3.51", "3.2", "3.23"],
...     "sulphates": ["0.56", "0.68", "0.82"],
...     "alcohol": ["9.4", "9.8", "12.6"],
... }
>>> await client.tabular_classification(table=table, model="julien-c/wine-quality")
["5", "5", "5"]
```

#### tabular_regression[[huggingface_hub.AsyncInferenceClient.tabular_regression]]

```python
tabular_regression(table: dict, model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1893)

**Parameters:**

table (`dict[str, Any]`) : Set of attributes stored in a table. The attributes used to predict the target can be both numerical and categorical.

model (`str`, *optional*) : The model to use for the tabular regression task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended tabular regression model will be used. Defaults to None.

**Returns:** `List`

a list of predicted numerical target values.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Predicting a numerical target value given a set of attributes/features in a table.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> table = {
...     "Height": ["11.52", "12.48", "12.3778"],
...     "Length1": ["23.2", "24", "23.9"],
...     "Length2": ["25.4", "26.3", "26.5"],
...     "Length3": ["30", "31.2", "31.1"],
...     "Species": ["Bream", "Bream", "Bream"],
...     "Width": ["4.02", "4.3056", "4.6961"],
... }
>>> await client.tabular_regression(table, model="scikit-learn/Fish-Weight")
[110, 120, 130]
```

#### text_classification[[huggingface_hub.AsyncInferenceClient.text_classification]]

```python
text_classification(text: str, model: str | None = None, top_k: int | None = None, function_to_apply: typing.Optional[ForwardRef('TextClassificationOutputTransform')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L1944)

**Parameters:**

text (`str`) : A string to be classified.

model (`str`, *optional*) : The model to use for the text classification task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended text classification model will be used. Defaults to None.

top_k (`int`, *optional*) : When specified, limits the output to the top K most probable classes.

function_to_apply (`"TextClassificationOutputTransform"`, *optional*) : The function to apply to the model outputs in order to retrieve the scores.

**Returns:** `list[TextClassificationOutputElement]`

a list of [TextClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TextClassificationOutputElement) items containing the predicted label and associated probability.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform text classification (e.g. sentiment-analysis) on the given text.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.text_classification("I like you")
[
    TextClassificationOutputElement(label='POSITIVE', score=0.9998695850372314),
    TextClassificationOutputElement(label='NEGATIVE', score=0.0001304351753788069),
]
```

#### text_generation[[huggingface_hub.AsyncInferenceClient.text_generation]]

```python
text_generation(prompt: str, details: bool | None = None, stream: bool | None = None, model: str | None = None, adapter_id: str | None = None, best_of: int | None = None, decoder_input_details: bool | None = None, do_sample: bool | None = None, frequency_penalty: float | None = None, grammar: huggingface_hub.inference._generated.types.text_generation.TextGenerationInputGrammarType | None = None, max_new_tokens: int | None = None, repetition_penalty: float | None = None, return_full_text: bool | None = None, seed: int | None = None, stop: list[str] | None = None, stop_sequences: list[str] | None = None, temperature: float | None = None, top_k: int | None = None, top_n_tokens: int | None = None, top_p: float | None = None, truncate: int | None = None, typical_p: float | None = None, watermark: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L2153)

**Parameters:**

prompt (`str`) : Input text.

details (`bool`, *optional*) : By default, text_generation returns a string. Pass `details=True` if you want a detailed output (tokens, probabilities, seed, finish reason, etc.). Only available for models running on with the `text-generation-inference` backend.

stream (`bool`, *optional*) : By default, text_generation returns the full generated text. Pass `stream=True` if you want a stream of tokens to be returned. Only available for models running on with the `text-generation-inference` backend.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. Defaults to None.

adapter_id (`str`, *optional*) : Lora adapter id.

best_of (`int`, *optional*) : Generate best_of sequences and return the one if the highest token logprobs.

decoder_input_details (`bool`, *optional*) : Return the decoder input token logprobs and ids. You must set `details=True` as well for it to be taken into account. Defaults to `False`.

do_sample (`bool`, *optional*) : Activate logits sampling

frequency_penalty (`float`, *optional*) : Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.

grammar ([TextGenerationInputGrammarType](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TextGenerationInputGrammarType), *optional*) : Grammar constraints. Can be either a JSONSchema or a regex.

max_new_tokens (`int`, *optional*) : Maximum number of generated tokens. Defaults to 100.

repetition_penalty (`float`, *optional*) : The parameter for repetition penalty. 1.0 means no penalty. See [this paper](https://arxiv.org/pdf/1909.05858.pdf) for more details.

return_full_text (`bool`, *optional*) : Whether to prepend the prompt to the generated text

seed (`int`, *optional*) : Random sampling seed

stop (`list[str]`, *optional*) : Stop generating tokens if a member of `stop` is generated.

stop_sequences (`list[str]`, *optional*) : Deprecated argument. Use `stop` instead.

temperature (`float`, *optional*) : The value used to module the logits distribution.

top_n_tokens (`int`, *optional*) : Return information about the `top_n_tokens` most likely tokens at each generation step, instead of just the sampled token.

top_k (`int`, *optional`) : The number of highest probability vocabulary tokens to keep for top-k-filtering.

top_p (`float`, *optional`) : If set to < 1, only the smallest set of most probable tokens with probabilities that add up to `top_p` or higher are kept for generation.

truncate (`int`, *optional`) : Truncate inputs tokens to the given size.

typical_p (`float`, *optional`) : Typical Decoding mass See [Typical Decoding for Natural Language Generation](https://arxiv.org/abs/2202.00666) for more information

watermark (`bool`, *optional*) : Watermarking with [A Watermark for Large Language Models](https://arxiv.org/abs/2301.10226)

**Returns:** `Union[str, TextGenerationOutput, AsyncIterable[str], AsyncIterable[TextGenerationStreamOutput]]`

Generated text returned from the server:
- if `stream=False` and `details=False`, the generated text is returned as a `str` (default)
- if `stream=True` and `details=False`, the generated text is returned token by token as a `AsyncIterable[str]`
- if `stream=False` and `details=True`, the generated text is returned with more details as a [TextGenerationOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TextGenerationOutput)
- if `details=True` and `stream=True`, the generated text is returned token by token as a iterable of [TextGenerationStreamOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TextGenerationStreamOutput)

**Raises:** ``ValidationError`` or [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- ``ValidationError`` -- 
  If input values are not valid. No HTTP call is made to the server.
- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Given a prompt, generate the following text.

> [!TIP]
> If you want to generate a response from chat messages, you should use the [InferenceClient.chat_completion()](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient.chat_completion) method.
> It accepts a list of messages instead of a single text prompt and handles the chat templating for you.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()

# Case 1: generate text
>>> await client.text_generation("The huggingface_hub library is ", max_new_tokens=12)
'100% open source and built to be easy to use.'

# Case 2: iterate over the generated tokens. Useful for large generation.
>>> async for token in await client.text_generation("The huggingface_hub library is ", max_new_tokens=12, stream=True):
...     print(token)
100
%
open
source
and
built
to
be
easy
to
use
.

# Case 3: get more details about the generation process.
>>> await client.text_generation("The huggingface_hub library is ", max_new_tokens=12, details=True)
TextGenerationOutput(
    generated_text='100% open source and built to be easy to use.',
    details=TextGenerationDetails(
        finish_reason='length',
        generated_tokens=12,
        seed=None,
        prefill=[
            TextGenerationPrefillOutputToken(id=487, text='The', logprob=None),
            TextGenerationPrefillOutputToken(id=53789, text=' hugging', logprob=-13.171875),
            (...)
            TextGenerationPrefillOutputToken(id=204, text=' ', logprob=-7.0390625)
        ],
        tokens=[
            TokenElement(id=1425, text='100', logprob=-1.0175781, special=False),
            TokenElement(id=16, text='%', logprob=-0.0463562, special=False),
            (...)
            TokenElement(id=25, text='.', logprob=-0.5703125, special=False)
        ],
        best_of_sequences=None
    )
)

# Case 4: iterate over the generated tokens with more details.
# Last object is more complete, containing the full generated text and the finish reason.
>>> async for details in await client.text_generation("The huggingface_hub library is ", max_new_tokens=12, details=True, stream=True):
...     print(details)
...
TextGenerationStreamOutput(token=TokenElement(id=1425, text='100', logprob=-1.0175781, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=16, text='%', logprob=-0.0463562, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=1314, text=' open', logprob=-1.3359375, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=3178, text=' source', logprob=-0.28100586, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=273, text=' and', logprob=-0.5961914, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=3426, text=' built', logprob=-1.9423828, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=271, text=' to', logprob=-1.4121094, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=314, text=' be', logprob=-1.5224609, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=1833, text=' easy', logprob=-2.1132812, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=271, text=' to', logprob=-0.08520508, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(id=745, text=' use', logprob=-0.39453125, special=False), generated_text=None, details=None)
TextGenerationStreamOutput(token=TokenElement(
    id=25,
    text='.',
    logprob=-0.5703125,
    special=False),
    generated_text='100% open source and built to be easy to use.',
    details=TextGenerationStreamOutputStreamDetails(finish_reason='length', generated_tokens=12, seed=None)
)

# Case 5: generate constrained output using grammar
>>> response = await client.text_generation(
...     prompt="I saw a puppy a cat and a raccoon during my bike ride in the park",
...     model="HuggingFaceH4/zephyr-orpo-141b-A35b-v0.1",
...     max_new_tokens=100,
...     repetition_penalty=1.3,
...     grammar={
...         "type": "json",
...         "value": {
...             "properties": {
...                 "location": {"type": "string"},
...                 "activity": {"type": "string"},
...                 "animals_seen": {"type": "integer", "minimum": 1, "maximum": 5},
...                 "animals": {"type": "array", "items": {"type": "string"}},
...             },
...             "required": ["location", "activity", "animals_seen", "animals"],
...         },
...     },
... )
>>> json.loads(response)
{
    "activity": "bike riding",
    "animals": ["puppy", "cat", "raccoon"],
    "animals_seen": 3,
    "location": "park"
}
```

#### text_to_image[[huggingface_hub.AsyncInferenceClient.text_to_image]]

```python
text_to_image(prompt: str, negative_prompt: str | None = None, height: int | None = None, width: int | None = None, num_inference_steps: int | None = None, guidance_scale: float | None = None, model: str | None = None, scheduler: str | None = None, seed: int | None = None, extra_body: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L2493)

**Parameters:**

prompt (`str`) : The prompt to generate an image from.

negative_prompt (`str`, *optional*) : One prompt to guide what NOT to include in image generation.

height (`int`, *optional*) : The height in pixels of the output image

width (`int`, *optional*) : The width in pixels of the output image

num_inference_steps (`int`, *optional*) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

guidance_scale (`float`, *optional*) : A higher guidance scale value encourages the model to generate images closely linked to the text prompt, but values too high may cause saturation and other artifacts.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended text-to-image model will be used. Defaults to None.

scheduler (`str`, *optional*) : Override the scheduler with a compatible one.

seed (`int`, *optional*) : Seed for the random number generator.

extra_body (`dict[str, Any]`, *optional*) : Additional provider-specific parameters to pass to the model. Refer to the provider's documentation for supported parameters.

**Returns:** `Image`

The generated image.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Generate an image based on a given text using a specified model.

> [!WARNING]
> You must have `PIL` installed if you want to work with images (`pip install Pillow`).

> [!TIP]
> You can pass provider-specific parameters to the model by using the `extra_body` argument.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()

>>> image = await client.text_to_image("An astronaut riding a horse on the moon.")
>>> image.save("astronaut.png")

>>> image = await client.text_to_image(
...     "An astronaut riding a horse on the moon.",
...     negative_prompt="low resolution, blurry",
...     model="stabilityai/stable-diffusion-2-1",
... )
>>> image.save("better_astronaut.png")
```

Example using a third-party provider directly. Usage will be billed on your fal.ai account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="fal-ai",  # Use fal.ai provider
...     api_key="fal-ai-api-key",  # Pass your fal.ai API key
... )
>>> image = client.text_to_image(
...     "A majestic lion in a fantasy forest",
...     model="black-forest-labs/FLUX.1-schnell",
... )
>>> image.save("lion.png")
```

Example using a third-party provider through Hugging Face Routing. Usage will be billed on your Hugging Face account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",  # Use replicate provider
...     api_key="hf_...",  # Pass your HF token
... )
>>> image = client.text_to_image(
...     "An astronaut riding a horse on the moon.",
...     model="black-forest-labs/FLUX.1-dev",
... )
>>> image.save("astronaut.png")
```

Example using Replicate provider with extra parameters

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",  # Use replicate provider
...     api_key="hf_...",  # Pass your HF token
... )
>>> image = client.text_to_image(
...     "An astronaut riding a horse on the moon.",
...     model="black-forest-labs/FLUX.1-schnell",
...     extra_body={"output_quality": 100},
... )
>>> image.save("astronaut.png")
```

#### text_to_speech[[huggingface_hub.AsyncInferenceClient.text_to_speech]]

```python
text_to_speech(text: str, model: str | None = None, do_sample: bool | None = None, early_stopping: typing.Union[bool, ForwardRef('TextToSpeechEarlyStoppingEnum'), NoneType] = None, epsilon_cutoff: float | None = None, eta_cutoff: float | None = None, max_length: int | None = None, max_new_tokens: int | None = None, min_length: int | None = None, min_new_tokens: int | None = None, num_beam_groups: int | None = None, num_beams: int | None = None, penalty_alpha: float | None = None, temperature: float | None = None, top_k: int | None = None, top_p: float | None = None, typical_p: float | None = None, use_cache: bool | None = None, extra_body: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L2731)

**Parameters:**

text (`str`) : The text to synthesize.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended text-to-speech model will be used. Defaults to None.

do_sample (`bool`, *optional*) : Whether to use sampling instead of greedy decoding when generating new tokens.

early_stopping (`Union[bool, "TextToSpeechEarlyStoppingEnum"]`, *optional*) : Controls the stopping condition for beam-based methods.

epsilon_cutoff (`float`, *optional*) : If set to float strictly between 0 and 1, only tokens with a conditional probability greater than epsilon_cutoff will be sampled. In the paper, suggested values range from 3e-4 to 9e-4, depending on the size of the model. See [Truncation Sampling as Language Model Desmoothing](https://hf.co/papers/2210.15191) for more details.

eta_cutoff (`float`, *optional*) : Eta sampling is a hybrid of locally typical sampling and epsilon sampling. If set to float strictly between 0 and 1, a token is only considered if it is greater than either eta_cutoff or sqrt(eta_cutoff) * exp(-entropy(softmax(next_token_logits))). The latter term is intuitively the expected next token probability, scaled by sqrt(eta_cutoff). In the paper, suggested values range from 3e-4 to 2e-3, depending on the size of the model. See [Truncation Sampling as Language Model Desmoothing](https://hf.co/papers/2210.15191) for more details.

max_length (`int`, *optional*) : The maximum length (in tokens) of the generated text, including the input.

max_new_tokens (`int`, *optional*) : The maximum number of tokens to generate. Takes precedence over max_length.

min_length (`int`, *optional*) : The minimum length (in tokens) of the generated text, including the input.

min_new_tokens (`int`, *optional*) : The minimum number of tokens to generate. Takes precedence over min_length.

num_beam_groups (`int`, *optional*) : Number of groups to divide num_beams into in order to ensure diversity among different groups of beams. See [this paper](https://hf.co/papers/1610.02424) for more details.

num_beams (`int`, *optional*) : Number of beams to use for beam search.

penalty_alpha (`float`, *optional*) : The value balances the model confidence and the degeneration penalty in contrastive search decoding.

temperature (`float`, *optional*) : The value used to modulate the next token probabilities.

top_k (`int`, *optional*) : The number of highest probability vocabulary tokens to keep for top-k-filtering.

top_p (`float`, *optional*) : If set to float < 1, only the smallest set of most probable tokens with probabilities that add up to top_p or higher are kept for generation.

typical_p (`float`, *optional*) : Local typicality measures how similar the conditional probability of predicting a target token next is to the expected conditional probability of predicting a random token next, given the partial text already generated. If set to float < 1, the smallest set of the most locally typical tokens with probabilities that add up to typical_p or higher are kept for generation. See [this paper](https://hf.co/papers/2202.00666) for more details.

use_cache (`bool`, *optional*) : Whether the model should use the past last key/values attentions to speed up decoding

extra_body (`dict[str, Any]`, *optional*) : Additional provider-specific parameters to pass to the model. Refer to the provider's documentation for supported parameters.

**Returns:** `bytes`

The generated audio.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Synthesize an audio of a voice pronouncing a given text.

> [!TIP]
> You can pass provider-specific parameters to the model by using the `extra_body` argument.

Example:
```py
# Must be run in an async context
>>> from pathlib import Path
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()

>>> audio = await client.text_to_speech("Hello world")
>>> Path("hello_world.flac").write_bytes(audio)
```

Example using a third-party provider directly. Usage will be billed on your Replicate account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",
...     api_key="your-replicate-api-key",  # Pass your Replicate API key directly
... )
>>> audio = client.text_to_speech(
...     text="Hello world",
...     model="OuteAI/OuteTTS-0.3-500M",
... )
>>> Path("hello_world.flac").write_bytes(audio)
```

Example using a third-party provider through Hugging Face Routing. Usage will be billed on your Hugging Face account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",
...     api_key="hf_...",  # Pass your HF token
... )
>>> audio =client.text_to_speech(
...     text="Hello world",
...     model="OuteAI/OuteTTS-0.3-500M",
... )
>>> Path("hello_world.flac").write_bytes(audio)
```

Example using Replicate provider with extra parameters

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",  # Use replicate provider
...     api_key="hf_...",  # Pass your HF token
... )
>>> audio = client.text_to_speech(
...     "Hello, my name is Kororo, an awesome text-to-speech model.",
...     model="hexgrad/Kokoro-82M",
...     extra_body={"voice": "af_nicole"},
... )
>>> Path("hello.flac").write_bytes(audio)
```

Example music-gen using "YuE-s1-7B-anneal-en-cot" on fal.ai

```py
>>> from huggingface_hub import InferenceClient
>>> lyrics = '''
... [verse]
... In the town where I was born
... Lived a man who sailed to sea
... And he told us of his life
... In the land of submarines
... So we sailed on to the sun
... 'Til we found a sea of green
... And we lived beneath the waves
... In our yellow submarine

... [chorus]
... We all live in a yellow submarine
... Yellow submarine, yellow submarine
... We all live in a yellow submarine
... Yellow submarine, yellow submarine
... '''
>>> genres = "pavarotti-style tenor voice"
>>> client = InferenceClient(
...     provider="fal-ai",
...     model="m-a-p/YuE-s1-7B-anneal-en-cot",
...     api_key=...,
... )
>>> audio = client.text_to_speech(lyrics, extra_body={"genres": genres})
>>> with open("output.mp3", "wb") as f:
...     f.write(audio)
```

#### text_to_video[[huggingface_hub.AsyncInferenceClient.text_to_video]]

```python
text_to_video(prompt: str, model: str | None = None, guidance_scale: float | None = None, negative_prompt: list[str] | None = None, num_frames: float | None = None, num_inference_steps: int | None = None, seed: int | None = None, extra_body: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L2634)

**Parameters:**

prompt (`str`) : The prompt to generate a video from.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended text-to-video model will be used. Defaults to None.

guidance_scale (`float`, *optional*) : A higher guidance scale value encourages the model to generate videos closely linked to the text prompt, but values too high may cause saturation and other artifacts.

negative_prompt (`list[str]`, *optional*) : One or several prompt to guide what NOT to include in video generation.

num_frames (`float`, *optional*) : The num_frames parameter determines how many video frames are generated.

num_inference_steps (`int`, *optional*) : The number of denoising steps. More denoising steps usually lead to a higher quality video at the expense of slower inference.

seed (`int`, *optional*) : Seed for the random number generator.

extra_body (`dict[str, Any]`, *optional*) : Additional provider-specific parameters to pass to the model. Refer to the provider's documentation for supported parameters.

**Returns:** `bytes`

The generated video.

Generate a video based on a given text.

> [!TIP]
> You can pass provider-specific parameters to the model by using the `extra_body` argument.

Example:

Example using a third-party provider directly. Usage will be billed on your fal.ai account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="fal-ai",  # Using fal.ai provider
...     api_key="fal-ai-api-key",  # Pass your fal.ai API key
... )
>>> video = client.text_to_video(
...     "A majestic lion running in a fantasy forest",
...     model="tencent/HunyuanVideo",
... )
>>> with open("lion.mp4", "wb") as file:
...     file.write(video)
```

Example using a third-party provider through Hugging Face Routing. Usage will be billed on your Hugging Face account.

```py
>>> from huggingface_hub import InferenceClient
>>> client = InferenceClient(
...     provider="replicate",  # Using replicate provider
...     api_key="hf_...",  # Pass your HF token
... )
>>> video = client.text_to_video(
...     "A cat running in a park",
...     model="genmo/mochi-1-preview",
... )
>>> with open("cat.mp4", "wb") as file:
...     file.write(video)
```

#### token_classification[[huggingface_hub.AsyncInferenceClient.token_classification]]

```python
token_classification(text: str, model: str | None = None, aggregation_strategy: typing.Optional[ForwardRef('TokenClassificationAggregationStrategy')] = None, ignore_labels: list[str] | None = None, stride: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L2940)

**Parameters:**

text (`str`) : A string to be classified.

model (`str`, *optional*) : The model to use for the token classification task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended token classification model will be used. Defaults to None.

aggregation_strategy (`"TokenClassificationAggregationStrategy"`, *optional*) : The strategy used to fuse tokens based on model predictions

ignore_labels (`list[str`, *optional*) : A list of labels to ignore

stride (`int`, *optional*) : The number of overlapping tokens between chunks when splitting the input text.

**Returns:** `list[TokenClassificationOutputElement]`

List of [TokenClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TokenClassificationOutputElement) items containing the entity group, confidence score, word, start and end index.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Perform token classification on the given text.
Usually used for sentence parsing, either grammatical, or Named Entity Recognition (NER) to understand keywords contained within text.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.token_classification("My name is Sarah Jessica Parker but you can call me Jessica")
[
    TokenClassificationOutputElement(
        entity_group='PER',
        score=0.9971321225166321,
        word='Sarah Jessica Parker',
        start=11,
        end=31,
    ),
    TokenClassificationOutputElement(
        entity_group='PER',
        score=0.9773476123809814,
        word='Jessica',
        start=52,
        end=59,
    )
]
```

#### translation[[huggingface_hub.AsyncInferenceClient.translation]]

```python
translation(text: str, model: str | None = None, src_lang: str | None = None, tgt_lang: str | None = None, clean_up_tokenization_spaces: bool | None = None, truncation: typing.Optional[ForwardRef('TranslationTruncationStrategy')] = None, generate_parameters: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L3016)

**Parameters:**

text (`str`) : A string to be translated.

model (`str`, *optional*) : The model to use for the translation task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended translation model will be used. Defaults to None.

src_lang (`str`, *optional*) : The source language of the text. Required for models that can translate from multiple languages.

tgt_lang (`str`, *optional*) : Target language to translate to. Required for models that can translate to multiple languages.

clean_up_tokenization_spaces (`bool`, *optional*) : Whether to clean up the potential extra spaces in the text output.

truncation (`"TranslationTruncationStrategy"`, *optional*) : The truncation strategy to use.

generate_parameters (`dict[str, Any]`, *optional*) : Additional parametrization of the text generation algorithm.

**Returns:** [TranslationOutput](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.TranslationOutput)

The generated translated text.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError` or ``ValueError``

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.
- ``ValueError`` -- 
  If only one of the `src_lang` and `tgt_lang` arguments are provided.

Convert text from one language to another.

Check out https://huggingface.co/tasks/translation for more information on how to choose the best model for
your specific use case. Source and target languages usually depend on the model.
However, it is possible to specify source and target languages for certain models. If you are working with one of these models,
you can use `src_lang` and `tgt_lang` arguments to pass the relevant information.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.translation("My name is Wolfgang and I live in Berlin")
'Mein Name ist Wolfgang und ich lebe in Berlin.'
>>> await client.translation("My name is Wolfgang and I live in Berlin", model="Helsinki-NLP/opus-mt-en-fr")
TranslationOutput(translation_text='Je m'appelle Wolfgang et je vis à Berlin.')
```

Specifying languages:
```py
>>> client.translation("My name is Sarah Jessica Parker but you can call me Jessica", model="facebook/mbart-large-50-many-to-many-mmt", src_lang="en_XX", tgt_lang="fr_XX")
"Mon nom est Sarah Jessica Parker mais vous pouvez m'appeler Jessica"
```

#### visual_question_answering[[huggingface_hub.AsyncInferenceClient.visual_question_answering]]

```python
visual_question_answering(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], question: str, model: str | None = None, top_k: int | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L3106)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The input image for the context. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

question (`str`) : Question to be answered.

model (`str`, *optional*) : The model to use for the visual question answering task. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. If not provided, the default recommended visual question answering model will be used. Defaults to None.

top_k (`int`, *optional*) : The number of answers to return (will be chosen by order of likelihood). Note that we return less than topk answers if there are not enough options available within the context.

**Returns:** `list[VisualQuestionAnsweringOutputElement]`

a list of [VisualQuestionAnsweringOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.VisualQuestionAnsweringOutputElement) items containing the predicted label and associated probability.

**Raises:** ``InferenceTimeoutError`` or `HfHubHTTPError`

- ``InferenceTimeoutError`` -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Answering open-ended questions based on an image.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.visual_question_answering(
...     image="https://huggingface.co/datasets/mishig/sample_images/resolve/main/tiger.jpg",
...     question="What is the animal doing?"
... )
[
    VisualQuestionAnsweringOutputElement(score=0.778609573841095, answer='laying down'),
    VisualQuestionAnsweringOutputElement(score=0.6957435607910156, answer='sitting'),
]
```

#### zero_shot_classification[[huggingface_hub.AsyncInferenceClient.zero_shot_classification]]

```python
zero_shot_classification(text: str, candidate_labels: list, multi_label: bool | None = False, hypothesis_template: str | None = None, model: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L3166)

**Parameters:**

text (`str`) : The input text to classify.

candidate_labels (`list[str]`) : The set of possible class labels to classify the text into.

multi_label (`bool`, *optional*) : Whether multiple candidate labels can be true. If false, the scores are normalized such that the sum of the label likelihoods for each sequence is 1. If true, the labels are considered independent and probabilities are normalized for each candidate.

hypothesis_template (`str`, *optional*) : The sentence used in conjunction with `candidate_labels` to attempt the text classification by replacing the placeholder with the candidate labels.

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. If not provided, the default recommended zero-shot classification model will be used.

**Returns:** `list[ZeroShotClassificationOutputElement]`

List of [ZeroShotClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ZeroShotClassificationOutputElement) items containing the predicted labels and their confidence.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Provide as input a text and a set of candidate labels to classify the input text.

Example with `multi_label=False`:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> text = (
...     "A new model offers an explanation for how the Galilean satellites formed around the solar system's"
...     "largest world. Konstantin Batygin did not set out to solve one of the solar system's most puzzling"
...     " mysteries when he went for a run up a hill in Nice, France."
... )
>>> labels = ["space & cosmos", "scientific discovery", "microbiology", "robots", "archeology"]
>>> await client.zero_shot_classification(text, labels)
[
    ZeroShotClassificationOutputElement(label='scientific discovery', score=0.7961668968200684),
    ZeroShotClassificationOutputElement(label='space & cosmos', score=0.18570658564567566),
    ZeroShotClassificationOutputElement(label='microbiology', score=0.00730885099619627),
    ZeroShotClassificationOutputElement(label='archeology', score=0.006258360575884581),
    ZeroShotClassificationOutputElement(label='robots', score=0.004559356719255447),
]
>>> await client.zero_shot_classification(text, labels, multi_label=True)
[
    ZeroShotClassificationOutputElement(label='scientific discovery', score=0.9829297661781311),
    ZeroShotClassificationOutputElement(label='space & cosmos', score=0.755190908908844),
    ZeroShotClassificationOutputElement(label='microbiology', score=0.0005462635890580714),
    ZeroShotClassificationOutputElement(label='archeology', score=0.00047131875180639327),
    ZeroShotClassificationOutputElement(label='robots', score=0.00030448526376858354),
]
```

Example with `multi_label=True` and a custom `hypothesis_template`:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()
>>> await client.zero_shot_classification(
...    text="I really like our dinner and I'm very happy. I don't like the weather though.",
...    candidate_labels=["positive", "negative", "pessimistic", "optimistic"],
...    multi_label=True,
...    hypothesis_template="This text is {} towards the weather"
... )
[
    ZeroShotClassificationOutputElement(label='negative', score=0.9231801629066467),
    ZeroShotClassificationOutputElement(label='pessimistic', score=0.8760990500450134),
    ZeroShotClassificationOutputElement(label='optimistic', score=0.0008674879791215062),
    ZeroShotClassificationOutputElement(label='positive', score=0.0005250611575320363)
]
```

#### zero_shot_image_classification[[huggingface_hub.AsyncInferenceClient.zero_shot_image_classification]]

```python
zero_shot_image_classification(image: typing.Union[bytes, typing.BinaryIO, str, pathlib.Path, ForwardRef('Image'), bytearray, memoryview], candidate_labels: list, model: str | None = None, hypothesis_template: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/inference/_generated/_async_client.py#L3269)

**Parameters:**

image (`Union[str, Path, bytes, BinaryIO, PIL.Image.Image]`) : The input image to caption. It can be raw bytes, an image file, a URL to an online image, or a PIL Image.

candidate_labels (`list[str]`) : The candidate labels for this image

model (`str`, *optional*) : The model to use for inference. Can be a model ID hosted on the Hugging Face Hub or a URL to a deployed Inference Endpoint. This parameter overrides the model defined at the instance level. If not provided, the default recommended zero-shot image classification model will be used.

hypothesis_template (`str`, *optional*) : The sentence used in conjunction with `candidate_labels` to attempt the image classification by replacing the placeholder with the candidate labels.

**Returns:** `list[ZeroShotImageClassificationOutputElement]`

List of [ZeroShotImageClassificationOutputElement](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_types#huggingface_hub.ZeroShotImageClassificationOutputElement) items containing the predicted labels and their confidence.

**Raises:** [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) or `HfHubHTTPError`

- [InferenceTimeoutError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceTimeoutError) -- 
  If the model is unavailable or the request times out.
- `HfHubHTTPError` -- 
  If the request fails with an HTTP error status code other than HTTP 503.

Provide input image and text labels to predict text labels for the image.

Example:
```py
# Must be run in an async context
>>> from huggingface_hub import AsyncInferenceClient
>>> client = AsyncInferenceClient()

>>> await client.zero_shot_image_classification(
...     "https://upload.wikimedia.org/wikipedia/commons/thumb/4/43/Cute_dog.jpg/320px-Cute_dog.jpg",
...     candidate_labels=["dog", "cat", "horse"],
... )
[ZeroShotImageClassificationOutputElement(label='dog', score=0.956),...]
```

## InferenceTimeoutError[[huggingface_hub.InferenceTimeoutError]]

#### huggingface_hub.InferenceTimeoutError[[huggingface_hub.InferenceTimeoutError]]

```python
huggingface_hub.InferenceTimeoutError(message: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/errors.py#L155)

Error raised when a model is unavailable or the request times out.
