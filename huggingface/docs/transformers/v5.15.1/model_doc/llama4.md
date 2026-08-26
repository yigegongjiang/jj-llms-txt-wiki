# Llama4

    
        
        
    

[Llama 4](https://ai.meta.com/blog/llama-4-multimodal-intelligence/), developed by Meta, introduces a new auto-regressive Mixture-of-Experts (MoE) architecture.
This generation includes two models:

- The highly capable Llama 4 Maverick with 17B active parameters out of ~400B total, with 128 experts.
- The efficient Llama 4 Scout also  has 17B active parameters out of ~109B total, using just 16 experts.
-

Both models leverage early fusion for native multimodality, enabling them to process text and image inputs.
Maverick and Scout are both trained on up to 40 trillion tokens on data encompassing 200 languages
(with specific fine-tuning support for 12 languages including Arabic, Spanish, German, and Hindi).

For deployment, Llama 4 Scout is designed for accessibility, fitting on a single server-grade GPU via
on-the-fly 4-bit or 8-bit int4 quantization, while Maverick is available in BF16 and FP8 formats.
These models are released under the custom Llama 4 Community License Agreement, available on the model repositories.

You can find all the original Llama checkpoints under the [meta-llama](https://huggingface.co/meta-llama) organization.

> [!TIP]
> The Llama 4 family of models comes in two flavors: 109B, and 402B parameters. Both of these flavors are extremely
> large and won't fit on your run-of-the-mill device. See below for some examples to reduce the memory usage of the
> model.
>
> For the download to be faster and more resilient, we recommend installing the `hf_xet` dependency as followed:
> `pip install transformers[hf_xet]`

The examples below demonstrates how to generate with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel). We additionally add an example
showcasing how to toggle the right attributes to enable very long-context generations, as some flavors of Llama 4
have context lengths going up to 10 million tokens.

```python

from transformers import pipeline

model_id = "meta-llama/Llama-4-Scout-17B-16E-Instruct"

messages = [
    {"role": "user", "content": "what is the recipe of mayonnaise?"},
]

pipe = pipeline(
    "text-generation",
    model=model_id,
    device_map="auto",
)

output = pipe(messages, do_sample=False, max_new_tokens=200)
print(output[0]["generated_text"][-1]["content"])
```

```python

from transformers import AutoTokenizer, Llama4ForConditionalGeneration

model_id = "meta-llama/Llama-4-Scout-17B-16E-Instruct"

tokenizer = AutoTokenizer.from_pretrained(model_id)

messages = [
    {"role": "user", "content": "Who are you?"},
]
inputs = tokenizer.apply_chat_template(messages, add_generation_prompt=True, return_tensors="pt", return_dict=True).to(model.device)

model = Llama4ForConditionalGeneration.from_pretrained(
    model_id,
    device_map="auto",
)

outputs = model.generate(**inputs.to(model.device), max_new_tokens=100)
outputs = tokenizer.batch_decode(outputs[:, inputs["input_ids"].shape[-1]:])
print(outputs[0])
```

```python

from transformers import AutoProcessor, Llama4ForConditionalGeneration

model_id = "meta-llama/Llama-4-Scout-17B-16E-Instruct"

processor = AutoProcessor.from_pretrained(model_id)
model = Llama4ForConditionalGeneration.from_pretrained(
    model_id,
    device_map="auto",
)

img_url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/0052a70beed5bf71b92610a43a52df6d286cd5f3/diffusers/rabbit.jpg"
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": img_url},
            {"type": "text", "text": "Describe this image in two sentences."},
        ]
    },
]

inputs = processor.apply_chat_template(
    messages,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
).to(model.device)

outputs = model.generate(
    **inputs,
    max_new_tokens=256,
)

response = processor.batch_decode(outputs[:, inputs["input_ids"].shape[-1]:])[0]
print(response)
```

```python

from transformers import AutoProcessor, Llama4ForConditionalGeneration

model_id = "meta-llama/Llama-4-Scout-17B-16E-Instruct"

processor = AutoProcessor.from_pretrained(model_id)
model = Llama4ForConditionalGeneration.from_pretrained(
    model_id,
    device_map="auto",
)

url1 = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/0052a70beed5bf71b92610a43a52df6d286cd5f3/diffusers/rabbit.jpg"
url2 = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/datasets/cat_style_layout.png"
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": url1},
            {"type": "image", "url": url2},
            {"type": "text", "text": "Can you describe how these two images are similar, and how they differ?"},
        ]
    },
]

inputs = processor.apply_chat_template(
    messages,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
).to(model.device)

outputs = model.generate(
    **inputs,
    max_new_tokens=256,
)

response = processor.batch_decode(outputs[:, inputs["input_ids"].shape[-1]:])[0]
print(response)
```

Beware: the example below uses both `device_map="auto"` and flex-attention.
Please use `torchrun` to run this example in tensor-parallel mode.

We will work to enable running with `device_map="auto"` and flex-attention without
tensor-parallel in the future.

```python
import time

import torch

from transformers import AutoTokenizer, Llama4ForConditionalGeneration

file = "very_long_context_prompt.txt"
model_id = "meta-llama/Llama-4-Scout-17B-16E-Instruct"

with open(file, "r") as f:
    very_long_text = "\n".join(f.readlines())

tokenizer = AutoTokenizer.from_pretrained(model_id)
model = Llama4ForConditionalGeneration.from_pretrained(
    model_id,
    device_map="auto",
    attn_implementation="flex_attention",
)

messages = [
    {"role": "user", "content": f"Look at the following texts: [{very_long_text}]\n\n\n\nWhat are the books, and who wrote them? Make me a nice list."},
]
input_ids = tokenizer.apply_chat_template(messages, add_generation_prompt=True, return_tensors="pt").to(model.device)

torch_device_module = getattr(torch, device, torch.cuda)
torch_device_module.synchronize()
start = time.time()
out = model.generate(
    input_ids.to(model.device),
    prefill_chunk_size=2048*8,
    max_new_tokens=300,
    cache_implementation="hybrid",
)
print(time.time()-start)
print(tokenizer.batch_decode(out[:, input_ids.shape[-1]:]))
print(f"{torch_device_module.max_memory_allocated(model.device) / 1024**3:.2f} GiB")
```

## Efficiency; how to get the best out of llama 4

### The Attention methods

Updating the default attention function can significantly improve compute performance as well as memory usage. Refer to the [Attention Interface](../attention_interface) overview for an in-depth explanation of our interface.

As of release, the Llama 4 model supports the following attention methods: `eager`, `flex_attention`, `sdpa`. We recommend using `flex_attention` for best results.
Switching attention mechanism is done at the model initialization step:

Setting Flex Attention ensures the best results with the very long context the model can handle.

> [!TIP] Beware: the example below uses both `device_map="auto"` and flex-attention.
> Please use `torchrun` to run this example in tensor-parallel mode.
>
> We will work to enable running with `device_map="auto"` and flex-attention without
> tensor-parallel in the future.

```python

from transformers import Llama4ForConditionalGeneration

model = Llama4ForConditionalGeneration.from_pretrained(
    model_id,
    attn_implementation="flex_attention",
    device_map="auto",
)
```

The `sdpa` attention method is generally more compute-efficient than the `eager` method.

```python

from transformers import Llama4ForConditionalGeneration

model = Llama4ForConditionalGeneration.from_pretrained(
    model_id,
    attn_implementation="sdpa",
    device_map="auto",
)
```

The `eager` attention method is set by default, so no need for anything different when loading the model:

```python

from transformers import Llama4ForConditionalGeneration

model = Llama4ForConditionalGeneration.from_pretrained(
    model_id,
    device_map="auto",
)
```

### Quantization

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for available quantization backends.
At time of release, both FBGEMM and LLM-Compressor are supported; more quantization methods will be supported in the days that follow the release.

See below for examples using both:

Here is an example loading an BF16 model in FP8 using the FBGEMM approach:

```python

from transformers import AutoTokenizer, FbgemmFp8Config, Llama4ForConditionalGeneration

model_id = "meta-llama/Llama-4-Scout-17B-16E-Instruct"

tokenizer = AutoTokenizer.from_pretrained(model_id)

messages = [
    {"role": "user", "content": "Who are you?"},
]
inputs = tokenizer.apply_chat_template(messages, add_generation_prompt=True, return_tensors="pt", return_dict=True).to(model.device)

model = Llama4ForConditionalGeneration.from_pretrained(
    model_id,
    device_map="auto",
    quantization_config=FbgemmFp8Config()
)

outputs = model.generate(**inputs.to(model.device), max_new_tokens=100)
outputs = tokenizer.batch_decode(outputs[:, inputs["input_ids"].shape[-1]:])
print(outputs[0])
```

To use the LLM-Compressor technique, we recommend leveraging the pre-quantized FP8 checkpoint available with the release:

```python

from transformers import AutoTokenizer, Llama4ForConditionalGeneration

model_id = "meta-llama/Llama-4-Maverick-17B-128E-Instruct-FP8"

tokenizer = AutoTokenizer.from_pretrained(model_id)

messages = [
    {"role": "user", "content": "Who are you?"},
]
inputs = tokenizer.apply_chat_template(messages, add_generation_prompt=True, return_tensors="pt", return_dict=True).to(model.device)

model = Llama4ForConditionalGeneration.from_pretrained(
    model_id,
    tp_plan="auto",
    device_map="auto",
)

outputs = model.generate(**inputs.to(model.device), max_new_tokens=100)
outputs = tokenizer.batch_decode(outputs[:, inputs["input_ids"].shape[-1]:])
print(outputs[0])
```

### Offloading

Enabling CPU-offloading means that components of the model might be moved to CPU instead of GPU in case the GPU-memory available isn't sufficient to load the entire model.
At inference, different components will be loaded/unloaded from/to the GPU on the fly. This ensures that the model can be loaded on smaller machines as long as the CPU-memory is sufficient.
However, this also slows down inference as it adds communication overhead.

In order to enable CPU-offloading, you simply need to specify the `device_map` to `auto` at model load:

```python

from transformers import Llama4ForConditionalGeneration

model = Llama4ForConditionalGeneration.from_pretrained(
    model_id,
    device_map="auto",
)
```

## Llama4Config[[transformers.Llama4Config]]

#### transformers.Llama4Config[[transformers.Llama4Config]]

```python
transformers.Llama4Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, boi_token_index: int = 200080, eoi_token_index: int = 200081, image_token_index: int = 200092, tie_word_embeddings: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/configuration_llama4.py#L207)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

boi_token_index (`int`, *optional*, defaults to 200080) : The begin-of-image token index to wrap the image prompt.

eoi_token_index (`int`, *optional*, defaults to 200081) : The end-of-image token index to wrap the image prompt.

image_token_index (`int`, *optional*, defaults to `200092`) : The image token index used as a placeholder for input images.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Llama4ForConditionalGeneration. It is used to instantiate a Llama4
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [meta-llama/Llama-4-Scout-17B-16E](https://huggingface.co/meta-llama/Llama-4-Scout-17B-16E)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import Llama4Model, Llama4Config

>>> # Initializing a Llama4 7B style configuration
>>> configuration = Llama4Config()

>>> # Initializing a model from the Llama4 7B style configuration
>>> model = Llama4Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Llama4TextConfig[[transformers.Llama4TextConfig]]

#### transformers.Llama4TextConfig[[transformers.Llama4TextConfig]]

```python
transformers.Llama4TextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 202048, hidden_size: int = 5120, intermediate_size: int = 8192, intermediate_size_mlp: int = 16384, num_hidden_layers: int = 48, num_attention_heads: int = 40, num_key_value_heads: int = 8, head_dim: int = 128, hidden_act: str = 'silu', max_position_embeddings: int = 131072, initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, use_cache: bool = True, pad_token_id: int | None = None, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 2, tie_word_embeddings: bool = False, attention_dropout: float | int = 0.0, num_experts_per_tok: int = 1, num_local_experts: int = 16, moe_layers: list[int] | None = None, interleave_moe_layer_step: int = 1, use_qk_norm: bool = True, output_router_logits: bool = False, router_aux_loss_coef: float = 0.001, router_jitter_noise: float = 0.0, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, no_rope_layers: list[int] | None = None, no_rope_layer_interval: int = 4, attention_chunk_size: int | None = 8192, layer_types: list[str] | None = None, attn_temperature_tuning: bool = True, floor_scale: int = 8192, attn_scale: float = 0.1, attention_bias: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/configuration_llama4.py#L79)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `202048`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `5120`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `8192`) : Dimension of the MLP representations.

intermediate_size_mlp (`int`, *optional*, defaults to 16384) : Intermediate size of dense MLP layers. Larger value increases FFN capacity and compute.

num_hidden_layers (`int`, *optional*, defaults to `48`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `40`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `131072`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

num_experts_per_tok (`int`, *optional*, defaults to `1`) : Number of experts to route each token to. This is the top-k value for the token-choice routing.

num_local_experts (`int`, *optional*, defaults to `16`) : Number of local experts on each device. `num_experts` should be divisible by `num_local_experts`.

moe_layers (`list[int]`, *optional*) : List of layer indices that use MoE. Overrides `interleave_moe_layer_step` when set.

interleave_moe_layer_step (`int`, *optional*, defaults to 1) : Spacing between MoE layers when `moe_layers` is `None`. Larger value means fewer MoE layers.

use_qk_norm (`bool`, *optional*, defaults to `True`) : Whether to L2-normalize queries/keys on RoPE layers. Can stabilize attention when enabled.

output_router_logits (`bool`, *optional*, defaults to `False`) : Whether or not the router logits should be returned by the model. Enabling this will also allow the model to output the auxiliary loss, including load balancing loss and router z-loss.

router_aux_loss_coef (`float`, *optional*, defaults to `0.001`) : Auxiliary load balancing loss coefficient. Used to penalize uneven expert routing in MoE models.

router_jitter_noise (`float`, *optional*, defaults to `0.0`) : Amount of noise to add to the router logits during training for better load balancing.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

no_rope_layers (`list[int]`, *optional*) : List with at least the same length as the number of layers in the model. A `1` at an index position indicates that the corresponding layer will use RoPE, while a `0` indicates that it's a NoPE layer.

no_rope_layer_interval (`int`, *optional*, defaults to 4) : If `no_rope_layers` is `None`, it will be created using a NoPE layer every `no_rope_layer_interval` layers.

attention_chunk_size (`int`, *optional*, defaults to 8192) : Chunk size for the attention computation. Smaller value enforces more local attention and lowers memory.

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

attn_temperature_tuning (`bool`, *optional*, defaults to `True`) : Whether to dynamically scale the attention temperature for each query token based on sequence length. Recommended for long sequences (e.g., >32k tokens) to maintain stable output results.

floor_scale (`int`, *optional*, defaults to 8192) : Base scale (in tokens) for attention temperature tuning. Larger value delays scaling to longer positions.

attn_scale (`float`, *optional*, defaults to 0.1) : Strength of attention temperature tuning. Larger value increases scaling at long positions.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

This is the configuration class to store the configuration of a Llama4ForConditionalGeneration. It is used to instantiate a Llama4
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [meta-llama/Llama-4-Scout-17B-16E](https://huggingface.co/meta-llama/Llama-4-Scout-17B-16E)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

## Llama4VisionConfig[[transformers.Llama4VisionConfig]]

#### transformers.Llama4VisionConfig[[transformers.Llama4VisionConfig]]

```python
transformers.Llama4VisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, hidden_act: str = 'gelu', num_hidden_layers: int = 34, num_attention_heads: int = 16, num_channels: int = 3, intermediate_size: int = 5632, vision_output_dim: int = 7680, image_size: int | list[int] | tuple[int, int] = 448, patch_size: int | list[int] | tuple[int, int] = 14, norm_eps: float = 1e-05, vision_feature_select_strategy: str = 'default', initializer_range: float = 0.02, pixel_shuffle_ratio: float = 0.5, projector_input_dim: int = 4096, projector_output_dim: int = 4096, multi_modal_projector_bias: bool = False, projector_dropout: float | int = 0.0, attention_dropout: float | int = 0.0, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/configuration_llama4.py#L29)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

num_hidden_layers (`int`, *optional*, defaults to `34`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

intermediate_size (`int`, *optional*, defaults to `5632`) : Dimension of the MLP representations.

vision_output_dim (`int`, *optional*, defaults to 7680) : Dimensionality of the vision model output. Includes output of transformer encoder with intermediate layers and global transformer encoder.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `448`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) : The size (resolution) of each patch.

norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

vision_feature_select_strategy (`str`, *optional*, defaults to `default`) : The feature selection strategy used to select the vision feature from the vision backbone.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

pixel_shuffle_ratio (`float`, *optional*, defaults to 0.5) : Pixel-shuffle ratio for downsampling patch tokens. Smaller values produce fewer tokens (more downsampling).

projector_input_dim (`int`, *optional*, defaults to 4096) : Width of the vision adapter MLP before pixel shuffle. Larger value increases capacity and compute.

projector_output_dim (`int`, *optional*, defaults to 4096) : Output width of the vision adapter. Larger value yields higher-dimensional image features.

multi_modal_projector_bias (`bool`, *optional*, defaults to `False`) : Whether to use bias in the multimodal projector.

projector_dropout (`float`, *optional*, defaults to 0.0) : Dropout rate inside the vision adapter MLP. Higher value adds more regularization.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

This is the configuration class to store the configuration of a Llama4ForConditionalGeneration. It is used to instantiate a Llama4
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [meta-llama/Llama-4-Scout-17B-16E](https://huggingface.co/meta-llama/Llama-4-Scout-17B-16E)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Llama4Processor[[transformers.Llama4Processor]]

#### transformers.Llama4Processor[[transformers.Llama4Processor]]

```python
transformers.Llama4Processor(image_processor = None, tokenizer = None, patch_size: int = 14, pixel_shuffle_ratio: float = 0.5, fake_image_token = '<|image|>', image_token = '<|image|>', start_of_image_token = '<|image_start|>', end_of_image_token = '<|image_end|>', patch_token = '<|patch|>', tile_x_separator_token = '<|tile_x_separator|>', tile_y_separator_token = '<|tile_y_separator|>', chat_template = '{{- bos_token }}\n{%- if custom_tools is defined %}\n    {%- set tools = custom_tools %}\n{%- endif %}\n{%- if not tools_in_user_message is defined %}\n    {%- set tools_in_user_message = true %}\n{%- endif %}\n{%- if not date_string is defined %}\n    {%- if strftime_now is defined %}\n        {%- set date_string = strftime_now("%d %b %Y") %}\n    {%- else %}\n        {%- set date_string = "26 Jul 2024" %}\n    {%- endif %}\n{%- endif %}\n{%- if not tools is defined %}\n    {%- set tools = none %}\n{%- endif %}\n\n{#- This block extracts the system message, so we can slot it into the right place. #}\n{%- if messages[0][\'role\'] == \'system\' %}    \n    {%- if messages[0][\'content\'] is string %}\n        {%- set system_message = messages[0][\'content\']|trim %}\n    {%- else %}\n        {#- FIXME: The processor requires an array, always. #}\n        {%- set system_message = messages[0][\'content\'][0][\'text\']|trim %}\n    {%- endif %}\n    {%- set messages = messages[1:] %}\n    {%- set user_supplied_system_message = true %}\n{%- else %}\n    {%- set system_message = "" %}\n    {%- set user_supplied_system_message = false %}\n{%- endif %}\n\n{#- System message if the user supplied one #}\n{%- if user_supplied_system_message %}\n    {{- "<|header_start|>system<|header_end|>\n\n" }}\n    {%- if tools is not none %}\n        {{- "Environment: ipython\n" }}\n    {%- endif %}\n    {%- if tools is not none and not tools_in_user_message %}\n        {{- "You have access to the following functions. To call a function, please respond with JSON for a function call." }}\n        {{- \'Respond in the format {"name": function name, "parameters": dictionary of argument name and its value}.\' }}\n        {{- "Do not use variables.\n\n" }}\n        {%- for t in tools %}\n            {{- t | tojson(indent=4) }}\n            {{- "\n\n" }}\n        {%- endfor %}\n    {%- endif %}\n    {{- system_message }}\n    {{- "<|eot|>" }}\n{%- endif %}\n\n{#- Custom tools are passed in a user message with some extra guidance #}\n{%- if tools_in_user_message and not tools is none %}\n    {#- Extract the first user message so we can plug it in here #}\n    {%- if messages | length != 0 %}\n        {%- set first_user_message = messages[0][\'content\']|trim %}\n        {%- set messages = messages[1:] %}\n    {%- else %}\n        {{- raise_exception("Cannot put tools in the first user message when there\'s no first user message!") }}\n{%- endif %}\n    {{- \'<|header_start|>user<|header_end|>\n\n\' -}}\n    {{- "Given the following functions, please respond with a JSON for a function call " }}\n    {{- "with its proper arguments that best answers the given prompt.\n\n" }}\n    {{- \'Respond in the format {"name": function name, "parameters": dictionary of argument name and its value}.\' }}\n    {{- "Do not use variables.\n\n" }}\n    {%- for t in tools %}\n        {{- t | tojson(indent=4) }}\n        {{- "\n\n" }}\n    {%- endfor %}\n    {{- first_user_message + "<|eot|>"}}\n{%- endif %}\n\n{%- for message in messages %}\n    {%- if not (message.role == \'ipython\' or message.role == \'tool\' or \'tool_calls\' in message) %}\n    {{- \'<|header_start|>\' + message[\'role\'] + \'<|header_end|>\n\n\' }}\n        {%- if message[\'content\'] is string %}\n            {{- message[\'content\'] }}\n        {%- else %}\n            {%- for content in message[\'content\'] %}\n                {%- if content[\'type\'] == \'image\' %}\n                    {{- \'<|image|>\' }}\n                {%- elif content[\'type\'] == \'text\' %}\n                    {{- content[\'text\'] }}\n                {%- endif %}\n            {%- endfor %}\n        {%- endif %}\n        {{- "<|eot|>" }}\n    {%- elif \'tool_calls\' in message and message.tool_calls|length > 0 %}\n       {{- \'<|header_start|>assistant<|header_end|>\n\n\' -}}\n       {{- \'<|python_start|>\' }}\n        {%- if message[\'content\'] is string %}\n            {{- message[\'content\'] }}\n        {%- else %}\n            {%- for content in message[\'content\'] %}\n                {%- if content[\'type\'] == \'image\' %}\n                    {{- \'<|image|>\' }}\n                {%- elif content[\'type\'] == \'text\' %}\n                    {{- content[\'text\'] }}\n                {%- endif %}\n            {%- endfor %}\n        {%- endif %}\n       {{- \'<|python_end|>\' }}\n        {%- for tool_call in message.tool_calls %}\n           {{- \'{"name": "\' + tool_call.function.name + \'", \' }}\n           {{- \'"parameters": \' }}\n           {{- tool_call.function.arguments | tojson }}\n           {{- "}" }}\n        {%- endfor %}\n       {{- "<|eot|>" }}\n    {%- elif message.role == "tool" or message.role == "ipython" %}\n        {{- "<|header_start|>ipython<|header_end|>\n\n" }}\n        {%- if message.content is mapping or message.content is iterable %}\n            {{- message.content | tojson }}\n        {%- else %}\n            {{- message.content }}\n        {%- endif %}\n        {{- "<|eot|>" }}\n    {%- endif %}\n{%- endfor %}\n{%- if add_generation_prompt %}\n    {{- \'<|header_start|>assistant<|header_end|>\n\n\' }}\n{%- endif %}\n', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/processing_llama4.py#L34)

**Parameters:**

image_processor (`Llama4ImageProcessor`) : The image processor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

patch_size (`int`, *optional*, defaults to 28) : The size of image patches for tokenization.

pixel_shuffle_ratio (`float`, *optional*, defaults to `0.5`) : The ratio used for pixel shuffling when processing images. This controls the downsampling factor applied to image patches. The actual downsampling ratio is calculated as `1 / (pixel_shuffle_ratio^2)`.

fake_image_token (`str`, *optional*, defaults to `"<|image|>"`) : The placeholder token in the text that will be replaced with actual image tokens. This token serves as a marker indicating where images should be inserted in the text sequence.

image_token (`str`, *optional*, defaults to `"<|image|>"`) : The token to be used to represent an image in the text.

start_of_image_token (`str`, *optional*, defaults to `"<|image_start|>"`) : The special token that marks the beginning of an image sequence in the text. This token is prepended to image token sequences to delimit image boundaries.

end_of_image_token (`str`, *optional*, defaults to `"<|image_end|>"`) : The special token that marks the end of an image sequence in the text. This token is appended to image token sequences to delimit image boundaries.

patch_token (`str`, *optional*, defaults to `"<|patch|>"`) : The token used to represent individual image patches. Multiple patch tokens are used to represent the full image, with the number depending on the image size and patch configuration.

tile_x_separator_token (`str`, *optional*, defaults to `"<|tile_x_separator|>"`) : The token used to separate tiles (patches) horizontally within an image. This token is inserted between patches in the same row when images are split into multiple tiles.

tile_y_separator_token (`str`, *optional*, defaults to `"<|tile_y_separator|>"`) : The token used to separate tiles (patches) vertically within an image. This token is inserted between rows of patches when images are split into multiple tiles.

chat_template (`str`, defaults to `{{- bos_token }} --

{%- if custom_tools is defined %} : {%- set tools = custom_tools %}

{%- endif %} --

{%- if not tools_in_user_message is defined %} : {%- set tools_in_user_message = true %}

{%- endif %} --

{%- if not date_string is defined %} : {%- if strftime_now is defined %} {%- set date_string = strftime_now("%d %b %Y") %} {%- else %} {%- set date_string = "26 Jul 2024" %} {%- endif %}

{%- endif %} --

{%- if not tools is defined %} : {%- set tools = none %}

{%- endif %} : 

{#- This block extracts the system message, so we can slot it into the right place. #} --

{%- if messages[0]['role'] == 'system' %} : {%- if messages[0]['content'] is string %} {%- set system_message = messages[0]['content']|trim %} {%- else %} {#- FIXME: The processor requires an array, always. #} {%- set system_message = messages[0]['content'][0]['text']|trim %} {%- endif %} {%- set messages = messages[1:] %} {%- set user_supplied_system_message = true %}

{%- else %} : {%- set system_message = "" %} {%- set user_supplied_system_message = false %}

{%- endif %} : 

{#- System message if the user supplied one #} --

{%- if user_supplied_system_message %} : {{- "<|header_start|>system<|header_end|> 

" }} : {%- if tools is not none %} {{- "Environment: ipython

" }} : {%- endif %} {%- if tools is not none and not tools_in_user_message %} {{- "You have access to the following functions. To call a function, please respond with JSON for a function call." }} {{- 'Respond in the format {"name": function name, "parameters": dictionary of argument name and its value}.' }} {{- "Do not use variables. 

" }} : {%- for t in tools %} {{- t | tojson(indent=4) }} {{- " 

" }} : {%- endfor %} {%- endif %} {{- system_message }} {{- "<|eot|>" }}

{%- endif %} : 

{#- Custom tools are passed in a user message with some extra guidance #} --

{%- if tools_in_user_message and not tools is none %} : {#- Extract the first user message so we can plug it in here #} {%- if messages | length != 0 %} {%- set first_user_message = messages[0]['content']|trim %} {%- set messages = messages[1:] %} {%- else %} {{- raise_exception("Cannot put tools in the first user message when there's no first user message!") }}

{%- endif %} : {{- '<|header_start|>user<|header_end|> 

' -}} : {{- "Given the following functions, please respond with a JSON for a function call " }} {{- "with its proper arguments that best answers the given prompt. 

" }} : {{- 'Respond in the format {"name": function name, "parameters": dictionary of argument name and its value}.' }} {{- "Do not use variables. 

" }} : {%- for t in tools %} {{- t | tojson(indent=4) }} {{- " 

" }} : {%- endfor %} {{- first_user_message + "<|eot|>"}}

{%- endif %} : 

{%- for message in messages %} : {%- if not (message.role == 'ipython' or message.role == 'tool' or 'tool_calls' in message) %} {{- '<|header_start|>' + message['role'] + '<|header_end|> 

' }} : {%- if message['content'] is string %} {{- message['content'] }} {%- else %} {%- for content in message['content'] %} {%- if content['type'] == 'image' %} {{- '<|image|>' }} {%- elif content['type'] == 'text' %} {{- content['text'] }} {%- endif %} {%- endfor %} {%- endif %} {{- "<|eot|>" }} {%- elif 'tool_calls' in message and message.tool_calls|length > 0 %} {{- '<|header_start|>assistant<|header_end|> 

' -}} : {{- '<|python_start|>' }} {%- if message['content'] is string %} {{- message['content'] }} {%- else %} {%- for content in message['content'] %} {%- if content['type'] == 'image' %} {{- '<|image|>' }} {%- elif content['type'] == 'text' %} {{- content['text'] }} {%- endif %} {%- endfor %} {%- endif %} {{- '<|python_end|>' }} {%- for tool_call in message.tool_calls %} {{- '{"name": "' + tool_call.function.name + '", ' }} {{- '"parameters": ' }} {{- tool_call.function.arguments | tojson }} {{- "}" }} {%- endfor %} {{- "<|eot|>" }} {%- elif message.role == "tool" or message.role == "ipython" %} {{- "<|header_start|>ipython<|header_end|> 

" }} : {%- if message.content is mapping or message.content is iterable %} {{- message.content | tojson }} {%- else %} {{- message.content }} {%- endif %} {{- "<|eot|>" }} {%- endif %}

{%- endfor %} --

{%- if add_generation_prompt %} : {{- '<|header_start|>assistant<|header_end|> 

' }} --

{%- endif %} --

`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a Llama4Processor which wraps a image processor and a tokenizer into a single processor.

[Llama4Processor](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4Processor) offers all the functionalities of [Llama4ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4ImageProcessor) and `tokenizer_class`. See the
[~Llama4ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4ImageProcessor) and `~tokenizer_class` for more information.

#### __call__[[transformers.Llama4Processor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: str | list[str] | list[list[str]] | None = None, videos: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]], NoneType] = None, audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L651)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

videos (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) : Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If passing in videos with pixel values between 0 and 1, set `do_rescale=False`.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

## Llama4ImageProcessor[[transformers.Llama4ImageProcessor]]

#### transformers.Llama4ImageProcessor[[transformers.Llama4ImageProcessor]]

```python
transformers.Llama4ImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/image_processing_llama4.py#L296)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 336, 'width': 336}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

max_patches (`int`, *kwargs*, *optional*, defaults to 16) : The maximum number of patches to be extracted from the image. Can be overridden by the `max_patches` parameter in the `preprocess` method.

resize_to_max_canvas (`bool`, *kwargs*, *optional*, defaults to False) : Whether to resize the image to the maximum canvas size. If True, picks the canvas the allows the largest resizing without distortion. If False, downsample as little as possible, including no resizing at all, but never upsample, unless the image is smaller than the patch size.

Constructs a Llama4ImageProcessor image processor.

#### preprocess[[transformers.Llama4ImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/image_processing_llama4.py#L312)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

max_patches (`int`, *kwargs*, *optional*, defaults to 16) : The maximum number of patches to be extracted from the image. Can be overridden by the `max_patches` parameter in the `preprocess` method.

resize_to_max_canvas (`bool`, *kwargs*, *optional*, defaults to False) : Whether to resize the image to the maximum canvas size. If True, picks the canvas the allows the largest resizing without distortion. If False, downsample as little as possible, including no resizing at all, but never upsample, unless the image is smaller than the patch size.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Llama4ForConditionalGeneration[[transformers.Llama4ForConditionalGeneration]]

#### transformers.Llama4ForConditionalGeneration[[transformers.Llama4ForConditionalGeneration]]

```python
transformers.Llama4ForConditionalGeneration(config: Llama4Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/modeling_llama4.py#L1158)

#### forward[[transformers.Llama4ForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, vision_feature_select_strategy: str | None = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/modeling_llama4.py#L1232)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Llama4ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4ImageProcessor). See `Llama4ImageProcessor.__call__()` for details ([Llama4Processor](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4Processor) uses [Llama4ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4ImageProcessor) for processing images).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `Llama4CausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `Llama4CausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Llama4Config](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4Config)) and inputs.

The [Llama4ForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size (batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, LlavaForConditionalGeneration

>>> model = LlavaForConditionalGeneration.from_pretrained("llava-hf/llava-1.5-7b-hf")
>>> processor = AutoProcessor.from_pretrained("llava-hf/llava-1.5-7b-hf")

>>> prompt = "USER: <image>\nWhat's the content of the image? ASSISTANT:"
>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, text=prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(**inputs, max_new_tokens=15)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"USER:  \nWhat's the content of the image? ASSISTANT: The image features a busy city street with a stop sign prominently displayed"
```

#### get_image_features[[transformers.Llama4ForConditionalGeneration.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, vision_feature_select_strategy: str, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/modeling_llama4.py#L1190)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, channels, height, width)`) : The tensors corresponding to the input images.

vision_feature_select_strategy (`str`) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`

vision_feature_select_strategy (`str`) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Llama4Config](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4Config)) and inputs.

Obtains image last hidden states from the vision tower and apply al projection.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Llama4ForConditionalGeneration

>>> model = Llama4ForConditionalGeneration.from_pretrained("meta-llama/Llama-4-Scout-17B-16E")
>>> processor = AutoProcessor.from_pretrained("meta-llama/Llama-4-Scout-17B-16E")

>>> messages = [
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
```

## Llama4ForCausalLM[[transformers.Llama4ForCausalLM]]

#### transformers.Llama4ForCausalLM[[transformers.Llama4ForCausalLM]]

```python
transformers.Llama4ForCausalLM(config: Llama4TextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/modeling_llama4.py#L576)

#### forward[[transformers.Llama4ForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/modeling_llama4.py#L593)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`

A [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Llama4Config](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4Config)) and inputs.

The [Llama4ForCausalLM](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4ForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, Llama4ForCausalLM

>>> model = Llama4ForCausalLM.from_pretrained("meta-llama4/Llama4-2-7b-hf")
>>> tokenizer = AutoTokenizer.from_pretrained("meta-llama4/Llama4-2-7b-hf")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```

## Llama4TextModel[[transformers.Llama4TextModel]]

#### transformers.Llama4TextModel[[transformers.Llama4TextModel]]

```python
transformers.Llama4TextModel(config: Llama4TextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/modeling_llama4.py#L483)

**Parameters:**

config ([Llama4TextConfig](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4TextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Llama4 Text Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Llama4TextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/modeling_llama4.py#L510)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Llama4Config](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4Config)) and inputs.

The [Llama4TextModel](/docs/transformers/v5.15.1/en/model_doc/llama4#transformers.Llama4TextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## Llama4VisionModel[[transformers.Llama4VisionModel]]

#### transformers.Llama4VisionModel[[transformers.Llama4VisionModel]]

```python
transformers.Llama4VisionModel(config: Llama4VisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/modeling_llama4.py#L1020)

#### forward[[transformers.Llama4VisionModel.forward]]

```python
forward(pixel_values: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/llama4/modeling_llama4.py#L1057)

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, MllamaVisionModel

>>> checkpoint = "meta-llama/Llama-3.2-11B-Vision"
>>> model = MllamaVisionModel.from_pretrained(checkpoint)
>>> processor = AutoProcessor.from_pretrained(checkpoint)

>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> inputs = processor(images=image, return_tensors="pt")

>>> output = model(**inputs)

>>> print(output.last_hidden_state.shape)
torch.Size([1, 1, 4, 1025, 7680])
```
