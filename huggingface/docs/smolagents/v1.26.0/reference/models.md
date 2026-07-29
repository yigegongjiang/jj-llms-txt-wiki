# Models

Smolagents is an experimental API which is subject to change at any time. Results returned by the agents
can vary as the APIs or underlying models are prone to change.

To learn more about agents and tools make sure to read the [introductory guide](../index). This page
contains the API docs for the underlying classes.

## Models

All model classes in smolagents support passing additional keyword arguments (like `temperature`, `max_tokens`, `top_p`, etc.) directly at instantiation time.
These parameters are automatically forwarded to the underlying model's completion calls, allowing you to configure model behavior such as creativity, response length, and sampling strategies.

### Base Model[[smolagents.Model]]

The `Model` class serves as the foundation for all model implementations, providing the core interface that custom models must implement to work with agents.

#### smolagents.Model[[smolagents.Model]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L452)

Base class for all language model implementations.

This abstract class defines the core interface that all model implementations must follow
to work with agents. It provides common functionality for message handling, tool integration,
and model configuration while allowing subclasses to implement their specific generation logic.

Note:
This is an abstract base class. Subclasses must implement the `generate()` method
to provide actual model inference capabilities.

Example:
```python
class CustomModel(Model):
    def generate(self, messages, **kwargs):
        # Implementation specific to your model
        pass
```

generatesmolagents.Model.generatehttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L553[{"name": "messages", "val": ": list"}, {"name": "stop_sequences", "val": ": list[str] | None = None"}, {"name": "response_format", "val": ": dict[str, str] | None = None"}, {"name": "tools_to_call_from", "val": ": list[smolagents.tools.Tool] | None = None"}, {"name": "**kwargs", "val": ""}]- **messages** (`list[dict[str, str | list[dict]]] | list[ChatMessage]`) --
  A list of message dictionaries to be processed. Each dictionary should have the structure `{"role": "user/system", "content": "message content"}`.
- **stop_sequences** (`List[str]`, *optional*) --
  A list of strings that will stop the generation if encountered in the model's output.
- **response_format** (`dict[str, str]`, *optional*) --
  The response format to use in the model's response.
- **tools_to_call_from** (`List[Tool]`, *optional*) --
  A list of tools that the model can use to generate responses.
- ****kwargs** --
  Additional keyword arguments to be passed to the underlying model.0`ChatMessage`A chat message object containing the model's response.
Process the input messages and return the model's response.

**Parameters:**

flatten_messages_as_text (`bool`, default `False`) : Whether to flatten complex message content into plain text format.

tool_name_key (`str`, default `"name"`) : The key used to extract tool names from model responses.

tool_arguments_key (`str`, default `"arguments"`) : The key used to extract tool arguments from model responses.

model_id (`str`, *optional*) : Identifier for the specific model being used.

- ****kwargs** : Additional keyword arguments to forward to the underlying model completion call.

**Returns:**

``ChatMessage``

A chat message object containing the model's response.
#### parse_tool_calls[[smolagents.Model.parse_tool_calls]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L583)

Sometimes APIs do not return the tool call as a specific object, so we need to parse it.
#### to_dict[[smolagents.Model.to_dict]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L596)

Converts the model into a JSON-compatible dictionary.

### API Model[[smolagents.ApiModel]]

The `ApiModel` class serves as the foundation for all API-based model implementations, providing common functionality for external API interactions, rate limiting, and client management that API-specific models inherit.

#### smolagents.ApiModel[[smolagents.ApiModel]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L1138)

Base class for API-based language models.

This class serves as a foundation for implementing models that interact with
external APIs. It handles the common functionality for managing model IDs,
custom role mappings, and API client connections.

create_clientsmolagents.ApiModel.create_clienthttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L1185[]
Create the API client for the specific service.

**Parameters:**

model_id (`str`) : The identifier for the model to be used with the API.

custom_role_conversions (`dict[str, str`], **optional**) : Mapping to convert  between internal role names and API-specific role names. Defaults to None.

client (`Any`, **optional**) : Pre-configured API client instance. If not provided, a default client will be created. Defaults to None.

requests_per_minute (`float`, **optional**) : Rate limit in requests per minute.

retry (`bool`, **optional**) : Whether to retry on rate limit errors, up to RETRY_MAX_ATTEMPTS times. Defaults to True.

- ****kwargs** : Additional keyword arguments to forward to the underlying model completion call.

### TransformersModel[[smolagents.TransformersModel]]

For convenience, we have added a `TransformersModel` that implements the points above by building a local `transformers` pipeline for the model_id given at initialization.

```python
from smolagents import TransformersModel

model = TransformersModel(model_id="HuggingFaceTB/SmolLM-135M-Instruct")

print(model([{"role": "user", "content": [{"type": "text", "text": "Ok!"}]}], stop_sequences=["great"]))
```
```text
>>> What a
```

You can pass any keyword arguments supported by the underlying model (such as `temperature`, `max_new_tokens`, `top_p`, etc.) directly at instantiation time. These are forwarded to the model completion call:

```python
model = TransformersModel(
    model_id="HuggingFaceTB/SmolLM-135M-Instruct",
    temperature=0.7,
    max_new_tokens=1000
)
```

> [!TIP]
> You must have `transformers` and `torch` installed on your machine. Please run `pip install 'smolagents[transformers]'` if it's not the case.

#### smolagents.TransformersModel[[smolagents.TransformersModel]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L860)

A class that uses Hugging Face's Transformers library for language model interaction.

This model allows you to load and use Hugging Face's models locally using the Transformers library. It supports features like stop sequences and grammar customization.

> [!TIP]
> You must have `transformers` and `torch` installed on your machine. Please run `pip install 'smolagents[transformers]'` if it's not the case.

Example:
```python
>>> engine = TransformersModel(
...     model_id="Qwen/Qwen3-Next-80B-A3B-Thinking",
...     device="cuda",
...     max_new_tokens=5000,
... )
>>> messages = [{"role": "user", "content": "Explain quantum mechanics in simple terms."}]
>>> response = engine(messages, stop_sequences=["END"])
>>> print(response)
"Quantum mechanics is the branch of physics that studies..."
```

**Parameters:**

model_id (`str`) : The Hugging Face model ID to be used for inference. This can be a path or model identifier from the Hugging Face model hub. For example, `"Qwen/Qwen3-Next-80B-A3B-Thinking"`.

device_map (`str`, *optional*) : The device_map to initialize your model with.

torch_dtype (`str`, *optional*) : The torch_dtype to initialize your model with.

trust_remote_code (bool, default `False`) : Some models on the Hub require running remote code: for this model, you would have to set this flag to True.

model_kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments to pass to `AutoModel.from_pretrained` (like revision, model_args, config, etc.).

max_new_tokens (`int`, default `4096`) : Maximum number of new tokens to generate, ignoring the number of tokens in the prompt.

max_tokens (`int`, *optional*) : Alias for `max_new_tokens`. If provided, this value takes precedence.

apply_chat_template_kwargs (dict, *optional*) : Additional keyword arguments to pass to the `apply_chat_template` method of the tokenizer.

- ****kwargs** : Additional keyword arguments to forward to the underlying Transformers model generate call, such as `device`.

### InferenceClientModel[[smolagents.InferenceClientModel]]

The `InferenceClientModel` wraps huggingface_hub's [InferenceClient](https://huggingface.co/docs/huggingface_hub/main/en/guides/inference) for the execution of the LLM. It supports all [Inference Providers](https://huggingface.co/docs/inference-providers/index) available on the Hub: Cerebras, Cohere, Fal, Fireworks, HF-Inference, Hyperbolic, Nebius, Novita, Replicate, SambaNova, Together, and more.

You can also set a rate limit in requests per minute by using the `requests_per_minute` argument:

```python
from smolagents import InferenceClientModel

messages = [
  {"role": "user", "content": [{"type": "text", "text": "Hello, how are you?"}]}
]

model = InferenceClientModel(provider="novita", requests_per_minute=60)
print(model(messages))
```
```text
>>> Of course! If you change your mind, feel free to reach out. Take care!
```

You can pass any keyword arguments supported by the underlying model (such as `temperature`, `max_tokens`, `top_p`, etc.) directly at instantiation time. These are forwarded to the model completion call:

```python
model = InferenceClientModel(
    provider="novita",
    requests_per_minute=60,
    temperature=0.8,
    max_tokens=500
)
```

#### smolagents.InferenceClientModel[[smolagents.InferenceClientModel]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L1456)

A class to interact with Hugging Face's Inference Providers for language model interaction.

This model allows you to communicate with Hugging Face's models using Inference Providers. It can be used in both serverless mode, with a dedicated endpoint, or even with a local URL, supporting features like stop sequences and grammar customization.

Providers include Cerebras, Cohere, Fal, Fireworks, HF-Inference, Hyperbolic, Nebius, Novita, Replicate, SambaNova, Together, and more.

Example:
```python
>>> engine = InferenceClientModel(
...     model_id="Qwen/Qwen3-Next-80B-A3B-Thinking",
...     provider="hyperbolic",
...     token="your_hf_token_here",
...     max_tokens=5000,
... )
>>> messages = [{"role": "user", "content": "Explain quantum mechanics in simple terms."}]
>>> response = engine(messages, stop_sequences=["END"])
>>> print(response)
"Quantum mechanics is the branch of physics that studies..."
```

create_clientsmolagents.InferenceClientModel.create_clienthttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L1547[]
Create the Hugging Face client.

**Parameters:**

model_id (`str`, *optional*, default `"Qwen/Qwen3-Next-80B-A3B-Thinking"`) : The Hugging Face model ID to be used for inference. This can be a model identifier from the Hugging Face model hub or a URL to a deployed Inference Endpoint. Currently, it defaults to `"Qwen/Qwen3-Next-80B-A3B-Thinking"`, but this may change in the future.

provider (`str`, *optional*) : Name of the provider to use for inference. A list of supported providers can be found in the [Inference Providers documentation](https://huggingface.co/docs/inference-providers/index#partners). Defaults to "auto" i.e. the first of the providers available for the model, sorted by the user's order [here](https://hf.co/settings/inference-providers). If `base_url` is passed, then `provider` is not used.

token (`str`, *optional*) : Token used by the Hugging Face API for authentication. This token need to be authorized 'Make calls to the serverless Inference Providers'. If the model is gated (like Llama-3 models), the token also needs 'Read access to contents of all public gated repos you can access'. If not provided, the class will try to use environment variable 'HF_TOKEN', else use the token stored in the Hugging Face CLI configuration.

timeout (`int`, *optional*, defaults to 120) : Timeout for the API request, in seconds.

client_kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments to pass to the Hugging Face InferenceClient.

custom_role_conversions (`dict[str, str]`, *optional*) : Custom role conversion mapping to convert message roles in others. Useful for specific models that do not support specific message roles like "system".

api_key (`str`, *optional*) : Token to use for authentication. This is a duplicated argument from `token` to make [InferenceClientModel](/docs/smolagents/v1.26.0/en/reference/models#smolagents.InferenceClientModel) follow the same pattern as `openai.OpenAI` client. Cannot be used if `token` is set. Defaults to None.

bill_to (`str`, *optional*) : The billing account to use for the requests. By default the requests are billed on the user's account. Requests can only be billed to an organization the user is a member of, and which has subscribed to Enterprise Hub.

base_url (`str`, `optional`) : Base URL to run inference. This is a duplicated argument from `model` to make [InferenceClientModel](/docs/smolagents/v1.26.0/en/reference/models#smolagents.InferenceClientModel) follow the same pattern as `openai.OpenAI` client. Cannot be used if `model` is set. Defaults to None.

- ****kwargs** : Additional keyword arguments to forward to the underlying Hugging Face InferenceClient completion call.

### LiteLLMModel[[smolagents.LiteLLMModel]]

The `LiteLLMModel` leverages [LiteLLM](https://www.litellm.ai/) to support 100+ LLMs from various providers.
You can pass kwargs upon model initialization that will then be used whenever using the model, for instance below we pass `temperature`. You can also set a rate limit in requests per minute by using the `requests_per_minute` argument.

```python
from smolagents import LiteLLMModel

messages = [
  {"role": "user", "content": [{"type": "text", "text": "Hello, how are you?"}]}
]

model = LiteLLMModel(model_id="anthropic/claude-3-5-sonnet-latest", temperature=0.2, max_tokens=10, requests_per_minute=60)
print(model(messages))
```

#### smolagents.LiteLLMModel[[smolagents.LiteLLMModel]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L1205)

Model to use [LiteLLM Python SDK](https://docs.litellm.ai/docs/#litellm-python-sdk) to access hundreds of LLMs.

create_clientsmolagents.LiteLLMModel.create_clienthttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L1255[]
Create the LiteLLM client.

**Parameters:**

model_id (`str`) : The model identifier to use on the server (e.g. "gpt-3.5-turbo").

api_base (`str`, *optional*) : The base URL of the provider API to call the model.

api_key (`str`, *optional*) : The API key to use for authentication.

custom_role_conversions (`dict[str, str]`, *optional*) : Custom role conversion mapping to convert message roles in others. Useful for specific models that do not support specific message roles like "system".

flatten_messages_as_text (`bool`, *optional*) : Whether to flatten messages as text. Defaults to `True` for models that start with "ollama", "groq", "cerebras".

- ****kwargs** : Additional keyword arguments to forward to the underlying LiteLLM completion call.

### LiteLLMRouterModel[[smolagents.LiteLLMRouterModel]]

The `LiteLLMRouterModel` is a wrapper around the [LiteLLM Router](https://docs.litellm.ai/docs/routing) that leverages
advanced routing strategies: load-balancing across multiple deployments, prioritizing critical requests via queueing,
and implementing basic reliability measures such as cooldowns, fallbacks, and exponential backoff retries.

```python
from smolagents import LiteLLMRouterModel

messages = [
  {"role": "user", "content": [{"type": "text", "text": "Hello, how are you?"}]}
]

model = LiteLLMRouterModel(
    model_id="llama-3.3-70b",
    model_list=[
        {
            "model_name": "llama-3.3-70b",
            "litellm_params": {"model": "groq/llama-3.3-70b", "api_key": os.getenv("GROQ_API_KEY")},
        },
        {
            "model_name": "llama-3.3-70b",
            "litellm_params": {"model": "cerebras/llama-3.3-70b", "api_key": os.getenv("CEREBRAS_API_KEY")},
        },
    ],
    client_kwargs={
        "routing_strategy": "simple-shuffle",
    },
)
print(model(messages))
```

#### smolagents.LiteLLMRouterModel[[smolagents.LiteLLMRouterModel]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L1363)

Router‑based client for interacting with the [LiteLLM Python SDK Router](https://docs.litellm.ai/docs/routing).

This class provides a high-level interface for distributing requests among multiple language models using
the LiteLLM SDK's routing capabilities. It is responsible for initializing and configuring the router client,
applying custom role conversions, and managing message formatting to ensure seamless integration with various LLMs.

Example:
```python
>>> import os
>>> from smolagents import CodeAgent, WebSearchTool, LiteLLMRouterModel
>>> os.environ["OPENAI_API_KEY"] = ""
>>> os.environ["AWS_ACCESS_KEY_ID"] = ""
>>> os.environ["AWS_SECRET_ACCESS_KEY"] = ""
>>> os.environ["AWS_REGION"] = ""
>>> llm_loadbalancer_model_list = [
...     {
...         "model_name": "model-group-1",
...         "litellm_params": {
...             "model": "gpt-4o-mini",
...             "api_key": os.getenv("OPENAI_API_KEY"),
...         },
...     },
...     {
...         "model_name": "model-group-1",
...         "litellm_params": {
...             "model": "bedrock/anthropic.claude-3-sonnet-20240229-v1:0",
...             "aws_access_key_id": os.getenv("AWS_ACCESS_KEY_ID"),
...             "aws_secret_access_key": os.getenv("AWS_SECRET_ACCESS_KEY"),
...             "aws_region_name": os.getenv("AWS_REGION"),
...         },
...     },
>>> ]
>>> model = LiteLLMRouterModel(
...    model_id="model-group-1",
...    model_list=llm_loadbalancer_model_list,
...    client_kwargs={
...        "routing_strategy":"simple-shuffle"
...    }
>>> )
>>> agent = CodeAgent(tools=[WebSearchTool()], model=model)
>>> agent.run("How many seconds would it take for a leopard at full speed to run through Pont des Arts?")
```

**Parameters:**

model_id (`str`) : Identifier for the model group to use from the model list (e.g., "model-group-1").

model_list (`list[dict[str, Any]]`) : Model configurations to be used for routing. Each configuration should include the model group name and any necessary parameters. For more details, refer to the [LiteLLM Routing](https://docs.litellm.ai/docs/routing#quick-start) documentation.

client_kwargs (`dict[str, Any]`, *optional*) : Additional configuration parameters for the Router client. For more details, see the [LiteLLM Routing Configurations](https://docs.litellm.ai/docs/routing).

custom_role_conversions (`dict[str, str]`, *optional*) : Custom role conversion mapping to convert message roles in others. Useful for specific models that do not support specific message roles like "system".

flatten_messages_as_text (`bool`, *optional*) : Whether to flatten messages as text. Defaults to `True` for models that start with "ollama", "groq", "cerebras".

- ****kwargs** : Additional keyword arguments to forward to the underlying LiteLLM Router completion call.

### OpenAIModel[[smolagents.OpenAIModel]]

This class lets you call any OpenAIServer compatible model.
Here's how you can set it (you can customise the `api_base` url to point to another server):
```py
import os
from smolagents import OpenAIModel

model = OpenAIModel(
    model_id="gpt-4o",
    api_base="https://api.openai.com/v1",
    api_key=os.environ["OPENAI_API_KEY"],
)
```

You can pass any keyword arguments supported by the underlying model (such as `temperature`, `max_tokens`, `top_p`, etc.) directly at instantiation time. These are forwarded to the model completion call:

```py
model = OpenAIModel(
    model_id="gpt-4o",
    api_base="https://api.openai.com/v1",
    api_key=os.environ["OPENAI_API_KEY"],
    temperature=0.7,
    max_tokens=1000,
    top_p=0.9,
)
```

#### smolagents.OpenAIModel[[smolagents.OpenAIModel]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L1646)

This model connects to an OpenAI-compatible API server.

**Parameters:**

model_id (`str`) : The model identifier to use on the server (e.g. "gpt-5").

api_base (`str`, *optional*) : The base URL of the OpenAI-compatible API server.

api_key (`str`, *optional*) : The API key to use for authentication.

organization (`str`, *optional*) : The organization to use for the API request.

project (`str`, *optional*) : The project to use for the API request.

client_kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments to pass to the OpenAI client (like organization, project, max_retries etc.).

custom_role_conversions (`dict[str, str]`, *optional*) : Custom role conversion mapping to convert message roles in others. Useful for specific models that do not support specific message roles like "system".

flatten_messages_as_text (`bool`, default `False`) : Whether to flatten messages as text.

- ****kwargs** : Additional keyword arguments to forward to the underlying OpenAI API completion call, for instance `temperature`.

### AzureOpenAIModel[[smolagents.AzureOpenAIModel]]

`AzureOpenAIModel` allows you to connect to any Azure OpenAI deployment. 

Below you can find an example of how to set it up, note that you can omit the `azure_endpoint`, `api_key`, and `api_version` arguments, provided you've set the corresponding environment variables -- `AZURE_OPENAI_ENDPOINT`, `AZURE_OPENAI_API_KEY`, and `OPENAI_API_VERSION`.

Pay attention to the lack of an `AZURE_` prefix for `OPENAI_API_VERSION`, this is due to the way the underlying [openai](https://github.com/openai/openai-python) package is designed. 

```py
import os

from smolagents import AzureOpenAIModel

model = AzureOpenAIModel(
    model_id = os.environ.get("AZURE_OPENAI_MODEL"),
    azure_endpoint=os.environ.get("AZURE_OPENAI_ENDPOINT"),
    api_key=os.environ.get("AZURE_OPENAI_API_KEY"),
    api_version=os.environ.get("OPENAI_API_VERSION")    
)
```

#### smolagents.AzureOpenAIModel[[smolagents.AzureOpenAIModel]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L1799)

This model connects to an Azure OpenAI deployment.

**Parameters:**

model_id (`str`) : The model deployment name to use when connecting (e.g. "gpt-4o-mini").

azure_endpoint (`str`, *optional*) : The Azure endpoint, including the resource, e.g. `https://example-resource.azure.openai.com/`. If not provided, it will be inferred from the `AZURE_OPENAI_ENDPOINT` environment variable.

api_key (`str`, *optional*) : The API key to use for authentication. If not provided, it will be inferred from the `AZURE_OPENAI_API_KEY` environment variable.

api_version (`str`, *optional*) : The API version to use. If not provided, it will be inferred from the `OPENAI_API_VERSION` environment variable.

client_kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments to pass to the AzureOpenAI client (like organization, project, max_retries etc.).

custom_role_conversions (`dict[str, str]`, *optional*) : Custom role conversion mapping to convert message roles in others. Useful for specific models that do not support specific message roles like "system".

- ****kwargs** : Additional keyword arguments to forward to the underlying Azure OpenAI API completion call.

### AmazonBedrockModel[[smolagents.AmazonBedrockModel]]

`AmazonBedrockModel` helps you connect to Amazon Bedrock and run your agent with any available models.

Below is an example setup. This class also offers additional options for customization.

```py
import os

from smolagents import AmazonBedrockModel

model = AmazonBedrockModel(
    model_id = os.environ.get("AMAZON_BEDROCK_MODEL_ID"),
)
```

#### smolagents.AmazonBedrockModel[[smolagents.AmazonBedrockModel]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L1859)

A model class for interacting with Amazon Bedrock Server models through the Bedrock API.

This class provides an interface to interact with various Bedrock language models,
allowing for customized model inference, guardrail configuration, message handling,
and other parameters allowed by boto3 API.

Authentication:

Amazon Bedrock supports multiple authentication methods:
- Default AWS credentials:
  Use the default AWS credential chain (e.g., IAM roles, IAM users).
- API Key Authentication (requires `boto3 >= 1.39.0`):
  Set the API key using the `AWS_BEARER_TOKEN_BEDROCK` environment variable.

> [!TIP]
> API key support requires `boto3 >= 1.39.0`.
> For users not relying on API key authentication, the minimum supported version is `boto3 >= 1.36.18`.

Examples:

Creating a model instance with default settings:
```python
>>> bedrock_model = AmazonBedrockModel(
...     model_id='us.amazon.nova-pro-v1:0'
... )
```

Creating a model instance with a custom boto3 client:
```python
>>> import boto3
>>> client = boto3.client('bedrock-runtime', region_name='us-west-2')
>>> bedrock_model = AmazonBedrockModel(
...     model_id='us.amazon.nova-pro-v1:0',
...     client=client
... )
```

Creating a model instance with client_kwargs for internal client creation:
```python
>>> bedrock_model = AmazonBedrockModel(
...     model_id='us.amazon.nova-pro-v1:0',
...     client_kwargs={'region_name': 'us-west-2', 'endpoint_url': 'https://custom-endpoint.com'}
... )
```

Creating a model instance with inference and guardrail configurations:
```python
>>> additional_api_config = {
...     "inferenceConfig": {
...         "maxTokens": 3000
...     },
...     "guardrailConfig": {
...         "guardrailIdentifier": "identify1",
...         "guardrailVersion": 'v1'
...     },
... }
>>> bedrock_model = AmazonBedrockModel(
...     model_id='anthropic.claude-3-haiku-20240307-v1:0',
...     **additional_api_config
... )
```

**Parameters:**

model_id (`str`) : The model identifier to use on Bedrock (e.g. "us.amazon.nova-pro-v1:0").

client (`boto3.client`, *optional*) : A custom boto3 client for AWS interactions. If not provided, a default client will be created.

client_kwargs (dict[str, Any], *optional*) : Keyword arguments used to configure the boto3 client if it needs to be created internally. Examples include `region_name`, `config`, or `endpoint_url`.

custom_role_conversions (`dict[str, str]`, *optional*) : Custom role conversion mapping to convert message roles in others. Useful for specific models that do not support specific message roles like "system". Defaults to converting all roles to "user" role to enable using all the Bedrock models.

flatten_messages_as_text (`bool`, default `False`) : Whether to flatten messages as text.

- ****kwargs** : Additional keyword arguments to forward to the underlying Amazon Bedrock model converse call.

### MLXModel[[smolagents.MLXModel]]

```python
from smolagents import MLXModel

model = MLXModel(model_id="HuggingFaceTB/SmolLM-135M-Instruct")

print(model([{"role": "user", "content": "Ok!"}], stop_sequences=["great"]))
```
```text
>>> What a
```

> [!TIP]
> You must have `mlx-lm` installed on your machine. Please run `pip install 'smolagents[mlx-lm]'` if it's not the case.

#### smolagents.MLXModel[[smolagents.MLXModel]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L751)

A class to interact with models loaded using MLX on Apple silicon.

> [!TIP]
> You must have `mlx-lm` installed on your machine. Please run `pip install 'smolagents[mlx-lm]'` if it's not the case.

Example:
```python
>>> engine = MLXModel(
...     model_id="mlx-community/Qwen2.5-Coder-32B-Instruct-4bit",
...     max_tokens=10000,
... )
>>> messages = [
...     {
...         "role": "user",
...         "content": "Explain quantum mechanics in simple terms."
...     }
... ]
>>> response = engine(messages, stop_sequences=["END"])
>>> print(response)
"Quantum mechanics is the branch of physics that studies..."
```

**Parameters:**

model_id (str) : The Hugging Face model ID to be used for inference. This can be a path or model identifier from the Hugging Face model hub.

tool_name_key (str) : The key, which can usually be found in the model's chat template, for retrieving a tool name.

tool_arguments_key (str) : The key, which can usually be found in the model's chat template, for retrieving tool arguments.

trust_remote_code (bool, default `False`) : Some models on the Hub require running remote code: for this model, you would have to set this flag to True.

load_kwargs (dict[str, Any], *optional*) : Additional keyword arguments to pass to the `mlx.lm.load` method when loading the model and tokenizer.

apply_chat_template_kwargs (dict, *optional*) : Additional keyword arguments to pass to the `apply_chat_template` method of the tokenizer.

- ****kwargs** : Additional keyword arguments to forward to the underlying MLX model stream_generate call, for instance `max_tokens`.

### VLLMModel[[smolagents.VLLMModel]]

Model to use [vLLM](https://docs.vllm.ai/) for fast LLM inference and serving.

```python
from smolagents import VLLMModel

model = VLLMModel(model_id="HuggingFaceTB/SmolLM-135M-Instruct")

print(model([{"role": "user", "content": "Ok!"}], stop_sequences=["great"]))
```

> [!TIP]
> You must have `vllm` installed on your machine. Please run `pip install 'smolagents[vllm]'` if it's not the case.

#### smolagents.VLLMModel[[smolagents.VLLMModel]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/models.py#L633)

Model to use [vLLM](https://docs.vllm.ai/) for fast LLM inference and serving.

**Parameters:**

model_id (`str`) : The Hugging Face model ID to be used for inference. This can be a path or model identifier from the Hugging Face model hub.

model_kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments to forward to the vLLM LLM instantiation, such as `revision`, `max_model_len`, etc.

apply_chat_template_kwargs (dict, *optional*) : Additional keyword arguments to pass to the `apply_chat_template` method of the tokenizer.

- ****kwargs** : Additional keyword arguments to forward to the underlying vLLM model generate call.

### Custom Model

You're free to create and use your own models to power your agent.

You could subclass the base `Model` class to create a model for your agent.
The main criteria is to subclass the `generate` method, with these two criteria:
1. It follows the [messages format](./chat_templating) (`List[Dict[str, str]]`) for its input `messages`, and it returns an object with a `.content` attribute.
2. It stops generating outputs at the sequences passed in the argument `stop_sequences`.

For defining your LLM, you can make a `CustomModel` class that inherits from the base `Model` class.
It should have a generate method that takes a list of [messages](./chat_templating) and returns an object with a .content attribute containing the text. The `generate` method also needs to accept a `stop_sequences` argument that indicates when to stop generating.

```python
from huggingface_hub import login, InferenceClient

from smolagents import Model

login("<YOUR_HUGGINGFACEHUB_API_TOKEN>")

model_id = "meta-llama/Llama-3.3-70B-Instruct"

client = InferenceClient(model=model_id)

class CustomModel(Model):
    def generate(messages, stop_sequences=["Task"]):
        response = client.chat_completion(messages, stop=stop_sequences, max_tokens=1024)
        answer = response.choices[0].message
        return answer

custom_model = CustomModel()
```

Additionally, `generate` can also take a `grammar` argument to allow [constrained generation](https://huggingface.co/docs/text-generation-inference/conceptual/guidance) in order to force properly-formatted agent outputs.
