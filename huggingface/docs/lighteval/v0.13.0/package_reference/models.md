# Model Configs

The model configs are used to define the model and its parameters. All the parameters can be
set in the `model-args` or in the model yaml file (see example
[here](https://github.com/huggingface/lighteval/blob/main/examples/model_configs/vllm_model_config.yaml)).

### Base model config[[lighteval.models.abstract_model.ModelConfig]]
#### lighteval.models.abstract_model.ModelConfig[[lighteval.models.abstract_model.ModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/abstract_model.py#L41)

Base configuration class for all model types in Lighteval.

This is the foundation class that all specific model configurations inherit from.
It provides common functionality for parsing configuration from files and command-line arguments,
as well as shared attributes that are used by all models like generation parameters and system prompts.

Methods:
from_path(path: str):
Load configuration from a YAML file.
from_args(args: str):
Parse configuration from a command-line argument string.
_parse_args(args: str):
Static method to parse argument strings into configuration dictionaries.

Example:
```python
# Load from YAML file
config = ModelConfig.from_path("model_config.yaml")

# Load from command line arguments
config = ModelConfig.from_args("model_name=meta-llama/Llama-3.1-8B-Instruct,system_prompt='You are a helpful assistant.',generation_parameters={temperature=0.7}")

# Direct instantiation
config = ModelConfig(
    model_name="meta-llama/Llama-3.1-8B-Instruct",
    generation_parameters=GenerationParameters(temperature=0.7),
    system_prompt="You are a helpful assistant."
)
```

**Parameters:**

model_name (str) : The model name or unique id

generation_parameters (GenerationParameters) : Configuration parameters that control text generation behavior, including temperature, top_p, max_new_tokens, etc. Defaults to empty GenerationParameters.

system_prompt (str | None) : Optional system prompt to be used with chat models. This prompt sets the behavior and context for the model during evaluation.

cache_dir (str) : Directory to cache the model. Defaults to "~/.cache/huggingface/lighteval".

## Local Models

### Transformers Model[[lighteval.models.transformers.transformers_model.TransformersModelConfig]]
#### lighteval.models.transformers.transformers_model.TransformersModelConfig[[lighteval.models.transformers.transformers_model.TransformersModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/transformers/transformers_model.py#L73)

Configuration class for HuggingFace Transformers models.

This configuration is used to load and configure models from the HuggingFace Transformers library.

Example:
```python
config = TransformersModelConfig(
    model_name="meta-llama/Llama-3.1-8B-Instruct",
    batch_size=4,
    dtype="float16",
    generation_parameters=GenerationParameters(
        temperature=0.7,
        max_new_tokens=100
    )
)
```

Note:
This configuration supports quantization (4-bit and 8-bit) through the dtype parameter.
When using quantization, ensure you have the required dependencies installed
(bitsandbytes for 4-bit/8-bit quantization).

**Parameters:**

model_name (str) : HuggingFace Hub model ID or path to a pre-trained model. This corresponds to the `pretrained_model_name_or_path` argument in HuggingFace's `from_pretrained` method.

tokenizer (str | None) : Optional HuggingFace Hub tokenizer ID. If not specified, uses the same ID as model_name. Useful when the tokenizer is different from the model (e.g., for multilingual models).

subfolder (str | None) : Subfolder within the model repository. Used when models are stored in subdirectories.

revision (str) : Git revision of the model to load. Defaults to "main".

batch_size (PositiveInt | None) : Batch size for model inference. If None, will be automatically determined.

max_length (PositiveInt | None) : Maximum sequence length for the model. If None, uses model's default.

model_loading_kwargs (dict) : Additional keyword arguments passed to `from_pretrained`. Defaults to empty dict.

add_special_tokens (bool) : Whether to add special tokens during tokenization. Defaults to True.

skip_special_tokens (bool) : Whether the tokenizer should output special tokens back during generation. Needed for reasoning models. Defaults to True

model_parallel (bool | None) : Whether to use model parallelism across multiple GPUs. If None, automatically determined based on available GPUs and model size.

dtype (str | None) : Data type for model weights. Can be "float16", "bfloat16", "float32", "auto", "4bit", "8bit". If "auto", uses the model's default dtype.

device (Union[int, str]) : Device to load the model on. Can be "cuda", "cpu", or GPU index. Defaults to "cuda".

trust_remote_code (bool) : Whether to trust remote code when loading models. Defaults to False.

compile (bool) : Whether to compile the model using torch.compile for optimization. Defaults to False.

multichoice_continuations_start_space (bool | None) : Whether to add a space before multiple choice continuations. If None, uses model default. True forces adding space, False removes leading space if present.

pairwise_tokenization (bool) : Whether to tokenize context and continuation separately or together. Defaults to False.

continuous_batching (bool) : Whether to use continuous batching for generation. Defaults to False.

override_chat_template (bool) : If True, we force the model to use a chat template. If alse, we prevent the model from using a chat template. If None, we use the default (true if present in the tokenizer, false otherwise)

generation_parameters (GenerationParameters, optional, defaults to empty GenerationParameters) : Configuration parameters that control text generation behavior, including temperature, top_p, max_new_tokens, etc.

system_prompt (str | None, optional, defaults to None) : Optional system prompt to be used with chat models. This prompt sets the behavior and context for the model during evaluation.

cache_dir (str, optional, defaults to "~/.cache/huggingface/lighteval") : Directory to cache the model.

#### lighteval.models.transformers.adapter_model.requires..inner_fn..Placeholder[[lighteval.models.transformers.adapter_model.requires..inner_fn..Placeholder]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/transformers/adapter_model.py#L43)

#### lighteval.models.transformers.delta_model.DeltaModelConfig[[lighteval.models.transformers.delta_model.DeltaModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/transformers/delta_model.py#L38)

Configuration class for delta models (weight difference models).

This configuration is used to load models that represent the difference between a
fine-tuned model and its base model. The delta weights are added to the base model
during loading to reconstruct the full fine-tuned model.

**Parameters:**

base_model (str) : HuggingFace Hub model ID or path to the base model. This is the original pre-trained model that the delta was computed from.

### VLLM Model[[lighteval.models.vllm.vllm_model.VLLMModelConfig]]
#### lighteval.models.vllm.vllm_model.VLLMModelConfig[[lighteval.models.vllm.vllm_model.VLLMModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/vllm/vllm_model.py#L76)

Configuration class for VLLM inference engine.

This configuration is used to load and configure models using the VLLM inference engine,
which provides high-performance inference for large language models with features like
PagedAttention, continuous batching, and efficient memory management.

vllm doc: https://docs.vllm.ai/en/v0.7.1/serving/engine_args.html

Example:
```python
config = VLLMModelConfig(
    model_name="meta-llama/Llama-3.1-8B-Instruct",
    tensor_parallel_size=2,
    gpu_memory_utilization=0.8,
    max_model_length=4096,
    generation_parameters=GenerationParameters(
        temperature=0.7,
        max_new_tokens=100
    )
)
```

**Parameters:**

model_name (str) : HuggingFace Hub model ID or path to the model to load.

tokenizer (str | None) : HuggingFace Hub model ID or path to the tokenizer to load.

revision (str) : Git revision of the model. Defaults to "main".

dtype (str) : Data type for model weights. Defaults to "bfloat16". Options: "float16", "bfloat16", "float32".

tensor_parallel_size (PositiveInt) : Number of GPUs to use for tensor parallelism. Defaults to 1.

data_parallel_size (PositiveInt) : Number of GPUs to use for data parallelism. Defaults to 1.

pipeline_parallel_size (PositiveInt) : Number of GPUs to use for pipeline parallelism. Defaults to 1.

gpu_memory_utilization (NonNegativeFloat) : Fraction of GPU memory to use. Lower this if running out of memory. Defaults to 0.9.

enable_prefix_caching (bool) : Whether to enable prefix caching to speed up generation. May use more memory. Should be disabled for LFM2. Defaults to True.

max_model_length (PositiveInt | None) : Maximum sequence length for the model. If None, automatically inferred. Reduce this if encountering OOM issues (4096 is usually sufficient).

quantization (str | None) : Quantization method.

load_format (str | None) : The format of the model weights to load. choices: auto, pt, safetensors, npcache, dummy, tensorizer, sharded_state, gguf, bitsandbytes, mistral, runai_streamer.

swap_space (PositiveInt) : CPU swap space size in GiB per GPU. Defaults to 4.

seed (NonNegativeInt) : Random seed for reproducibility. Defaults to 1234.

trust_remote_code (bool) : Whether to trust remote code when loading models. Defaults to False.

add_special_tokens (bool) : Whether to add special tokens during tokenization. Defaults to True.

multichoice_continuations_start_space (bool) : Whether to add a space before multiple choice continuations. Defaults to True.

pairwise_tokenization (bool) : Whether to tokenize context and continuation separately for loglikelihood evals. Defaults to False.

max_num_seqs (PositiveInt) : Maximum number of sequences per iteration. Controls batch size at prefill stage. Defaults to 128.

max_num_batched_tokens (PositiveInt) : Maximum number of tokens per batch. Defaults to 2048.

subfolder (str | None) : Subfolder within the model repository. Defaults to None.

is_async (bool) : Whether to use the async version of VLLM. Defaults to False.

override_chat_template (bool) : If True, we force the model to use a chat template. If alse, we prevent the model from using a chat template. If None, we use the default (true if present in the tokenizer, false otherwise)

generation_parameters (GenerationParameters, optional, defaults to empty GenerationParameters) : Configuration parameters that control text generation behavior, including temperature, top_p, max_new_tokens, etc.

system_prompt (str | None, optional, defaults to None) : Optional system prompt to be used with chat models. This prompt sets the behavior and context for the model during evaluation.

cache_dir (str, optional, defaults to "~/.cache/huggingface/lighteval") : Directory to cache the model.

### SGLang Model[[lighteval.models.sglang.sglang_model.SGLangModelConfig]]
#### lighteval.models.sglang.sglang_model.SGLangModelConfig[[lighteval.models.sglang.sglang_model.SGLangModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/sglang/sglang_model.py#L54)

Configuration class for SGLang inference engine.

This configuration is used to load and configure models using the SGLang inference engine,
which provides high-performance inference.

sglang doc: https://docs.sglang.ai/index.html#

Example:
```python
config = SGLangModelConfig(
    model_name="meta-llama/Llama-3.1-8B-Instruct",
    tp_size=2,
    context_length=8192,
    generation_parameters=GenerationParameters(
        temperature=0.7,
        max_new_tokens=100
    )
)
```

**Parameters:**

model_name (str) : HuggingFace Hub model ID or path to the model to load.

load_format (str) : The format of the model weights to load. choices: auto, pt, safetensors, npcache, dummy, tensorizer, sharded_state, gguf, bitsandbytes, mistral, runai_streamer.

dtype (str) : Data type for model weights. Defaults to "auto". Options: "auto", "float16", "bfloat16", "float32".

tp_size (PositiveInt) : Number of GPUs to use for tensor parallelism. Defaults to 1.

dp_size (PositiveInt) : Number of GPUs to use for data parallelism. Defaults to 1.

context_length (PositiveInt | None) : Maximum context length for the model.

random_seed (PositiveInt | None) : Random seed for reproducibility. Defaults to 1234.

trust_remote_code (bool) : Whether to trust remote code when loading models. Defaults to False.

device (str) : Device to load the model on. Defaults to "cuda".

skip_tokenizer_init (bool) : Whether to skip tokenizer initialization. Defaults to False.

kv_cache_dtype (str) : Data type for key-value cache. Defaults to "auto".

add_special_tokens (bool) : Whether to add special tokens during tokenization. Defaults to True.

pairwise_tokenization (bool) : Whether to tokenize context and continuation separately for loglikelihood evals. Defaults to False.

sampling_backend (str | None) : Sampling backend to use. If None, uses default.

attention_backend (str | None) : Attention backend to use. If None, uses default.

mem_fraction_static (PositiveFloat) : Fraction of GPU memory to use for static allocation. Defaults to 0.8.

chunked_prefill_size (PositiveInt) : Size of chunks for prefill operations. Defaults to 4096.

override_chat_template (bool) : If True, we force the model to use a chat template. If alse, we prevent the model from using a chat template. If None, we use the default (true if present in the tokenizer, false otherwise)

generation_parameters (GenerationParameters, optional, defaults to empty GenerationParameters) : Configuration parameters that control text generation behavior, including temperature, top_p, max_new_tokens, etc.

system_prompt (str | None, optional, defaults to None) : Optional system prompt to be used with chat models. This prompt sets the behavior and context for the model during evaluation.

cache_dir (str, optional, defaults to "~/.cache/huggingface/lighteval") : Directory to cache the model.

### Dummy Model[[lighteval.models.dummy.dummy_model.DummyModelConfig]]
#### lighteval.models.dummy.dummy_model.DummyModelConfig[[lighteval.models.dummy.dummy_model.DummyModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/dummy/dummy_model.py#L35)

Configuration class for dummy models used for testing and baselines.

This configuration is used to create dummy models that generate random responses
or baselines for evaluation purposes. Useful for testing evaluation pipelines
without requiring actual model inference.

Example:
```python
config = DummyModelConfig(
    model_name="my_dummy",
    seed=123,
)
```

**Parameters:**

model_name (str) : Name of your choice - "dummy" by default

seed (int) : Random seed for reproducible dummy responses. Defaults to 42. This seed controls the randomness of the generated responses and log probabilities.

## Endpoints-based Models

### Inference Providers Model[[lighteval.models.endpoints.inference_providers_model.InferenceProvidersModelConfig]]
#### lighteval.models.endpoints.inference_providers_model.InferenceProvidersModelConfig[[lighteval.models.endpoints.inference_providers_model.InferenceProvidersModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/endpoints/inference_providers_model.py#L45)

Configuration class for HuggingFace's inference providers (like Together AI, Anyscale, etc.).

inference providers doc: https://huggingface.co/docs/inference-providers/en/index

Example:
```python
config = InferenceProvidersModelConfig(
    model_name="deepseek-ai/DeepSeek-R1-0528",
    provider="together",
    parallel_calls_count=5,
    generation_parameters=GenerationParameters(
        temperature=0.7,
        max_new_tokens=100
    )
)
```

Note:
- Requires HF API keys to be set in environment variable
- Different providers have different rate limits and pricing

**Parameters:**

model_name (str) : Name or identifier of the model to use.

provider (str) : Name of the inference provider. Examples: "together", "anyscale", "runpod", etc.

timeout (int | None) : Request timeout in seconds. If None, uses provider default.

proxies (Any | None) : Proxy configuration for requests. Can be a dict or proxy URL string.

org_to_bill (str | None) : Organization to bill for API usage. If None, bills the user's account.

parallel_calls_count (NonNegativeInt) : Number of parallel API calls to make. Defaults to 10. Higher values increase throughput but may hit rate limits.

generation_parameters (GenerationParameters, optional, defaults to empty GenerationParameters) : Configuration parameters that control text generation behavior, including temperature, top_p, max_new_tokens, etc.

system_prompt (str | None, optional, defaults to None) : Optional system prompt to be used with chat models. This prompt sets the behavior and context for the model during evaluation.

cache_dir (str, optional, defaults to "~/.cache/huggingface/lighteval") : Directory to cache the model.

### InferenceEndpointModel[[lighteval.models.endpoints.endpoint_model.InferenceEndpointModelConfig]]
#### lighteval.models.endpoints.endpoint_model.InferenceEndpointModelConfig[[lighteval.models.endpoints.endpoint_model.InferenceEndpointModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/endpoints/endpoint_model.py#L108)

Configuration class for HuggingFace Inference Endpoints (dedicated infrastructure).

This configuration is used to create and manage dedicated inference endpoints
on HuggingFace's infrastructure. These endpoints provide dedicated compute
resources and can handle larger batch sizes and higher throughput.

Methods:
model_post_init():
Validates configuration and ensures proper parameter combinations.
get_dtype_args():
Returns environment variables for dtype configuration.
get_custom_env_vars():
Returns custom environment variables for the endpoint.

Example:
```python
config = InferenceEndpointModelConfig(
    model_name="microsoft/DialoGPT-medium",
    instance_type="nvidia-a100",
    instance_size="x1",
    vendor="aws",
    region="us-east-1",
    dtype="float16",
    generation_parameters=GenerationParameters(
        temperature=0.7,
        max_new_tokens=100
    )
)
```

Note:
- Creates dedicated infrastructure for model inference
- Supports various quantization methods and hardware configurations
- Auto-scaling available for optimal resource utilization
- Requires HuggingFace Pro subscription for most features
- Endpoints can take several minutes to start up
- Billed based on compute usage and duration

**Parameters:**

endpoint_name (str | None) : Name for the inference endpoint. If None, auto-generated from model_name.

model_name (str | None) : HuggingFace Hub model ID to deploy. Required if endpoint_name is None.

reuse_existing (bool) : Whether to reuse an existing endpoint with the same name. Defaults to False.

accelerator (str) : Type of accelerator to use. Defaults to "gpu". Options: "gpu", "cpu".

dtype (str | None) : Model data type. If None, uses model default. Options: "float16", "bfloat16", "awq", "gptq", "8bit", "4bit".

vendor (str) : Cloud vendor for the endpoint. Defaults to "aws". Options: "aws", "azure", "gcp".

region (str) : Cloud region for the endpoint. Defaults to "us-east-1".

instance_size (str | None) : Instance size for the endpoint. If None, auto-scaled.

instance_type (str | None) : Instance type for the endpoint. If None, auto-scaled.

framework (str) : ML framework to use. Defaults to "pytorch".

endpoint_type (str) : Type of endpoint. Defaults to "protected". Options: "protected", "public".

add_special_tokens (bool) : Whether to add special tokens during tokenization. Defaults to True.

revision (str) : Git revision of the model. Defaults to "main".

namespace (str | None) : Namespace for the endpoint. If None, uses current user's namespace.

image_url (str | None) : Custom Docker image URL. If None, uses default TGI image.

env_vars (dict | None) : Additional environment variables for the endpoint.

batch_size (int) : Batch size for requests. Defaults to 1.

generation_parameters (GenerationParameters, optional, defaults to empty GenerationParameters) : Configuration parameters that control text generation behavior, including temperature, top_p, max_new_tokens, etc.

system_prompt (str | None, optional, defaults to None) : Optional system prompt to be used with chat models. This prompt sets the behavior and context for the model during evaluation.

cache_dir (str, optional, defaults to "~/.cache/huggingface/lighteval") : Directory to cache the model.

#### lighteval.models.endpoints.endpoint_model.ServerlessEndpointModelConfig[[lighteval.models.endpoints.endpoint_model.ServerlessEndpointModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/endpoints/endpoint_model.py#L71)

Configuration class for HuggingFace Inference API (inference endpoints).

https://huggingface.co/inference-endpoints/dedicated

Example:
```python
config = ServerlessEndpointModelConfig(
    model_name="meta-llama/Llama-3.1-8B-Instruct",
    generation_parameters=GenerationParameters(
        temperature=0.7,
        max_new_tokens=100
    )
)
```

**Parameters:**

model_name (str) : HuggingFace Hub model ID to use with the Inference API. Example: "meta-llama/Llama-3.1-8B-Instruct"

add_special_tokens (bool) : Whether to add special tokens during tokenization. Defaults to True.

batch_size (int) : Batch size for requests. Defaults to 1 (serverless API limitation).

generation_parameters (GenerationParameters, optional, defaults to empty GenerationParameters) : Configuration parameters that control text generation behavior, including temperature, top_p, max_new_tokens, etc.

system_prompt (str | None, optional, defaults to None) : Optional system prompt to be used with chat models. This prompt sets the behavior and context for the model during evaluation.

cache_dir (str, optional, defaults to "~/.cache/huggingface/lighteval") : Directory to cache the model.

### TGI ModelClient[[lighteval.models.endpoints.tgi_model.TGIModelConfig]]
#### lighteval.models.endpoints.tgi_model.TGIModelConfig[[lighteval.models.endpoints.tgi_model.TGIModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/endpoints/tgi_model.py#L55)

Configuration class for Text Generation Inference (TGI) backend.

doc: https://huggingface.co/docs/text-generation-inference/en/index

This configuration is used to connect to TGI servers that serve HuggingFace models
using the text-generation-inference library. TGI provides high-performance inference
with features like continuous batching and efficient memory management.

Example:
```python
config = TGIModelConfig(
    inference_server_address="http://localhost:8080",
    inference_server_auth="your-auth-token",
    model_name="meta-llama/Llama-3.1-8B-Instruct",
    generation_parameters=GenerationParameters(
        temperature=0.7,
        max_new_tokens=100
    )
)
```

**Parameters:**

inference_server_address (str | None) : Address of the TGI server. Format: "http://host:port" or "https://host:port". Example: "http://localhost:8080"

inference_server_auth (str | None) : Authentication token for the TGI server. If None, no authentication is used.

model_name (str | None) : Optional model name override. If None, uses the model name from server info.

generation_parameters (GenerationParameters, optional, defaults to empty GenerationParameters) : Configuration parameters that control text generation behavior, including temperature, top_p, max_new_tokens, etc.

system_prompt (str | None, optional, defaults to None) : Optional system prompt to be used with chat models. This prompt sets the behavior and context for the model during evaluation.

cache_dir (str, optional, defaults to "~/.cache/huggingface/lighteval") : Directory to cache the model.

### Litellm Model[[lighteval.models.endpoints.litellm_model.LiteLLMModelConfig]]
#### lighteval.models.endpoints.litellm_model.LiteLLMModelConfig[[lighteval.models.endpoints.litellm_model.LiteLLMModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/endpoints/litellm_model.py#L61)

Configuration class for LiteLLM unified API client.

This configuration is used to connect to various LLM providers through the LiteLLM
unified API. LiteLLM provides a consistent interface to multiple providers including
OpenAI, Anthropic, Google, and many others.

litellm doc: https://docs.litellm.ai/docs/

Example:
```python
config = LiteLLMModelConfig(
    model_name="gpt-4",
    provider="openai",
    base_url="https://api.openai.com/v1",
    concurrent_requests=5,
    generation_parameters=GenerationParameters(
        temperature=0.7,
        max_new_tokens=100
    )
)
```

**Parameters:**

model_name (str) : Model identifier. Can include provider prefix (e.g., "gpt-4", "claude-3-sonnet") or use provider/model format (e.g., "openai/gpt-4", "anthropic/claude-3-sonnet").

provider (str | None) : Optional provider name override. If None, inferred from model_name. Examples: "openai", "anthropic", "google", "cohere", etc.

base_url (str | None) : Custom base URL for the API. If None, uses provider's default URL. Useful for using custom endpoints or local deployments.

api_key (str | None) : API key for authentication. If None, reads from environment variables. Environment variable names are provider-specific (e.g., OPENAI_API_KEY).

concurrent_requests (int) : Maximum number of concurrent API requests to execute in parallel. Higher values can improve throughput for batch processing but may hit rate limits or exhaust API quotas faster. Default is 10.

verbose (bool) : Whether to enable verbose logging. Default is False.

max_model_length (int | None) : Maximum context length for the model. If None, infers the model's default max length.

api_max_retry (int) : Maximum number of retries for API requests. Default is 8.

api_retry_sleep (float) : Initial sleep time (in seconds) between retries. Default is 1.0.

api_retry_multiplier (float) : Multiplier for increasing sleep time between retries. Default is 2.0.

timeout (float) : Request timeout in seconds. Default is None (no timeout).

generation_parameters (GenerationParameters, optional, defaults to empty GenerationParameters) : Configuration parameters that control text generation behavior, including temperature, top_p, max_new_tokens, etc.

system_prompt (str | None, optional, defaults to None) : Optional system prompt to be used with chat models. This prompt sets the behavior and context for the model during evaluation.

cache_dir (str, optional, defaults to "~/.cache/huggingface/lighteval") : Directory to cache the model.

## Custom Model[[lighteval.models.custom.custom_model.CustomModelConfig]]
#### lighteval.models.custom.custom_model.CustomModelConfig[[lighteval.models.custom.custom_model.CustomModelConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/models/custom/custom_model.py#L26)

Configuration class for loading custom model implementations in Lighteval.

This config allows users to define and load their own model implementations by specifying
a Python file containing a custom model class that inherits from LightevalModel.

The custom model file should contain exactly one class that inherits from LightevalModel.
This class will be automatically detected and instantiated when loading the model.

Example usage:
```python
# Define config
config = CustomModelConfig(
    model="my-custom-model",
    model_definition_file_path="path/to/my_model.py"
)

# Example custom model file (my_model.py):
from lighteval.models.abstract_model import LightevalModel

class MyCustomModel(LightevalModel):
    def __init__(self, config, env_config):
        super().__init__(config, env_config)
        # Custom initialization...

    def greedy_until(self, docs: list[Doc]) -> list[ModelResponse]:
        # Custom generation logic...
        pass

    def loglikelihood(self, docs: list[Doc]) -> list[ModelResponse]:
        pass
```

An example of a custom model can be found in `examples/custom_models/google_translate_model.py`.

**Parameters:**

model (str) : An identifier for the model. This can be used to track which model was evaluated in the results and logs. 

model_definition_file_path (str) : Path to a Python file containing the custom model implementation. This file must define exactly one class that inherits from LightevalModel. The class should implement all required methods from the LightevalModel interface.
