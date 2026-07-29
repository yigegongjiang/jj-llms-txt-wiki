# DiffusionGemma

## Overview

DiffusionGemma is engineered to reduce the sequential bottlenecks of standard causal language models. It employs an encoder-decoder architecture specifically optimized for inference speed.

The encoder operates in a prefill capacity, processing the initial prompt and generating the KV cache. The decoder then utilizes bidirectional attention to process an input block (a 'canvas') of tokens, accessing the cached context via cross-attention.

During inference, DiffusionGemma leverages multi-canvas sampling. Rather than generating one token at a time, the model iteratively denoises a full block of tokens using a diffusion sampler. Once a canvas is fully denoised, it is processed by the encoder and appended to the KV cache, after which the model generates the next canvas. This block-autoregressive approach facilitates text generation at higher speeds.

You can find the model card and checkpoint [here](https://huggingface.co/google/diffusiongemma-26B-A4B-it). You can find a visual guide to the model [here](https://newsletter.maartengrootendorst.com/p/a-visual-guide-to-diffusiongemma).

## Usage examples

Despite it being a text diffusion model and having a custom generation loop, most of the interface is shared with other models that can generate text with [DiffusionGemmaGenerationMixin.generate()](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaGenerationMixin.generate). If you're using another `transformers` model in your app, you should be able to directly replace it with this model.

> [!NOTE]
> DiffusionGemma is available in both Transformers and Diffusers, but Diffusers is its primary home. Diffusers has the full set of scheduling options, pipeline utilities, and new features (additional schedulers, stopping criteria, sampling strategies) are only added there. See the [DiffusionGemma](https://huggingface.co/docs/diffusers/api/pipelines/diffusion_gemma) for usage examples, and open an [issue](https://github.com/huggingface/diffusers/issues) on Diffusers to experiment with different schedulers or request a feature. The Transformers implementation only receives bug fixes but no new features.

### Common caveats

- DiffusionGemma doesn't accept `use_cache`. It always uses a KV cache;
- Support for common flags like `top_k` won't be available at release day, but will be added over time if they are compatible with text diffusion.

### Basic example

```python
from transformers import DiffusionGemmaForBlockDiffusion, AutoProcessor

model = DiffusionGemmaForBlockDiffusion.from_pretrained(
    "google/diffusiongemma-26B-A4B-it", device_map="auto",
)
processor = AutoProcessor.from_pretrained("google/diffusiongemma-26B-A4B-it")

messages = [
    {
        "role": "user", "content": [
            {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
            {"type": "text", "text": "What is shown in this image?"},
        ]
    },
]
inputs = processor.apply_chat_template(
    messages,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
    add_generation_prompt=True,
    # Add the following to enable thinking
    # enable_thinking=True,
).to(model.device)
input_len = inputs["input_ids"].shape[-1]

# Set `cache_implementation="static"` in `generate` to trigger `torch.compile`.
# Compilation is much faster, after warming up!
output = model.generate(**inputs, max_new_tokens=256)
print(processor.decode(output.sequences[0][input_len:], skip_special_tokens=True))
```

### Streaming

Like other models that can generate text, you can set a streamer class to stream text. Unlike other models, DiffusionGemma generates intermediate drafts before the final text. You can visualize them with `TextDiffusionStreamer`

```python
from transformers import TextDiffusionStreamer

# (... copy from the example above, up to the `generate` call)
streamer = TextDiffusionStreamer(tokenizer=processor.tokenizer)
model.generate(**inputs, max_new_tokens=256, streamer=streamer)
```

### Setting a starting denoising output

The model is trained to iteratively refine blocks of 256 tokens. On some applications, it may be beneficial to provide a starting point for the decoder, rather than starting from random tokens. You can use the `decoder_input_ids`, available on all model interfaces, to set the starting canvas.

```py
initial_estimate = ... # a tensor with shape (bsz, 256)
model.generate(**inputs, max_new_tokens=256, decoder_input_ids=initial_estimate)
```

## DiffusionGemmaTextConfig[[transformers.DiffusionGemmaTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `262144`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `2304`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `9216`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `30`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `4`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **head_dim** (`int`, *optional*, defaults to `256`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **hidden_activation** (`str`, *optional*, defaults to `gelu_pytorch_tanh`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `131072`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `1`) --
  Token id used for end-of-stream in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `2`) --
  Token id used for beginning-of-stream in the vocabulary.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **rope_parameters** (`dict`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[int, float]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **sliding_window** (`int`, *optional*, defaults to `512`) --
  Sliding window attention window size. If `None`, no sliding window is applied.
- **layer_types** (`list[str]`, *optional*) --
  A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically
  generated based on config values.
- **use_bidirectional_attention** (`str`, *optional*) --
  Controls bidirectional attention behavior. When set to `"vision"`, vision tokens
  attend bidirectionally while text tokens use causal attention. When set to `"all"`,
  all tokens use bidirectional attention.
- **num_global_key_value_heads** (`int`, *optional*) --
  Number of key-value heads for global (full) attention layers. If `None`, defaults
  to `num_key_value_heads`.
- **global_head_dim** (`int`, defaults to 512) --
  Dimension of each attention head in global (full) attention layers.
- **num_experts** (`int`, *optional*) --
  Number of routed experts in MoE layers.

- **top_k_experts** (`int`, *optional*) --
  Number of experts activated per token in MoE layers.
- **moe_intermediate_size** (`int`, *optional*) --
  Intermediate (hidden) size of each expert's feed-forward network in MoE layers.

This is the configuration class to store the configuration of a DiffusionGemmaModel. It is used to instantiate a Diffusion Gemma
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/diffusiongemma-26B-A4B-it](https://huggingface.co/google/diffusiongemma-26B-A4B-it)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## DiffusionGemmaConfig[[transformers.DiffusionGemmaConfig]]

- **text_config** (`Union[~models.diffusion_gemma.configuration_diffusion_gemma.DiffusionGemmaTextConfig, dict[str, Any]]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[~configuration_utils.PreTrainedConfig, dict[str, Any]]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **boi_token_id** (`int`, *optional*, defaults to 255999) --
  The begin-of-image token index to wrap the image prompt.
- **eoi_token_id** (`int`, *optional*, defaults to 258882) --
  The end-of-image token index to wrap the image prompt.
- **image_token_id** (`int`, *optional*, defaults to `258880`) --
  The image token index used as a placeholder for input images.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **canvas_length** (`int`, *optional*, defaults to 256) --
  The size of the canvas or, in other words, the block length in block diffusion. Used to initialize an empty
  canvas.

This is the configuration class to store the configuration of a DiffusionGemmaModel. It is used to instantiate a Diffusion Gemma
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/diffusiongemma-26B-A4B-it](https://huggingface.co/google/diffusiongemma-26B-A4B-it)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
>>>     DiffusionGemmaConfig,
>>>     DiffusionGemmaModel,
>>>     DiffusionGemmaTextConfig,
>>>     Gemma4VisionConfig,
>>> )

>>> # Initializing a DiffusionGemma Text config.
>>> text_config = DiffusionGemmaTextConfig()

>>> # Initializing a Gemma 4 vision config (DiffusionGemma uses Gemma 4's vision block).
>>> vision_config = Gemma4VisionConfig()

>>> # Initializing a DiffusionGemma text config
>>> configuration = DiffusionGemmaConfig(text_config, vision_config)

>>> # Initializing a model from the configuration
>>> model = DiffusionGemmaModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DiffusionGemmaGenerationOutput[[transformers.DiffusionGemmaGenerationOutput]]

- **sequences** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  The generated sequences, including the prompt if `input_ids` was provided to the `generate` method.
- **tokens_per_forward** (`torch.LongTensor` of shape (`batch_size`)) --
  The number of tokens per forward in this `generate` call, for each member in the batch. This is often
  used as a secondary evaluation metric for text diffusion models.
- **past_key_values** (`Cache`) --
  The cache used for generation. It can be passed to subsequent calls to `generate` to speed up generation,
  in multi-turn sessions.
- **logits** (`None`) --
  Unused. Kept in the interface for BC.
- **scores** (`None`) --
  Unused. Kept in the interface for BC.
- **hidden_states** (`None`) --
  Unused. Kept in the interface for BC.

Output class for DiffusionGemma generation.

## DiffusionGemmaGenerationMixin[[transformers.DiffusionGemmaGenerationMixin]]

Mixin class for DiffusionGemma generation. Contains all the model-level methods.

- **input_ids** (*torch.LongTensor* of shape *(batch_size, sequence_length)*, *optional*) --
  The sequence used as a prompt for the generation.
- **past_key_values** ([Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache), *optional*) --
  Cache object containing the past key values and past attention masks for the decoder. If it is set,
  `input_ids` and/or `pixel_values` must correspond to uncached data only.
- **streamer** (`BaseStreamer`, *optional*) --
  Streamer object that will be used to stream the generated sequences. Generated tokens are passed
  through `streamer.put(token_ids)` and the streamer is responsible for any further processing. If the
  streamer object has a `put_draft` method, tokens from the denoising steps will be sent there.

Additional arguments for power users

- **generation_config** ([DiffusionGemmaGenerationConfig](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaGenerationConfig), *optional*) --
  The generation configuration to be used as base parametrization for the generation call, overriding
  the model defaults. If the model checkpoint has a `generation_config.json` file, the model default
  will be loaded from there. Otherwise, it will be an empty `DiffusionGemmaGenerationConfig` instance.
  As an additional shortcut, `**kwargs` matching attributes in the `generation_config` will override them.
- **logits_processor** ([LogitsProcessorList](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.LogitsProcessorList), *optional*) --
  Custom logits processors that complement the default logits processors built from arguments and
  generation config, to be applied on the diffusion logits. If provided, these processors will be first
  to be applied. This feature is intended for advanced users. You can, for instance, pass here the
  logits processors commonly used with AR LLMs.
- **stopping_criteria** ([StoppingCriteriaList](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.StoppingCriteriaList), *optional*) --
  Custom stopping criteria that complements the default block autoregressive stopping criteria built
  from arguments and a generation config. If provided, these criteria will be first to be applied. This
  feature is intended for advanced users. You can, for instance, pass here the stopping criteria commonly
  used with AR LLMs.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Ad hoc parametrization of `generation_config` and/or additional model-specific kwargs that will be
  forwarded to the `forward` function of the model. For instance, you can set the starting canvas with
  `decoder_input_ids`.`torch.LongTensor` or [DiffusionGemmaGenerationOutput](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaGenerationOutput)- `torch.LongTensor`: the generated text in ids.
- [DiffusionGemmaGenerationOutput](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaGenerationOutput): a `ModelOutput` instance containing the generated text (`sequences`),
  as well as other optional outputs.

Generates text using the diffusion model.

It contains an outer loop doing autoregressive generation of canvases (blocks of tokens), and an inner

loop doing diffusion on each canvas. The algorithm works roughly as follows:
```
1. Autoregressive canvas generation loop:
    a. Encode all previous tokens using the encoder, to get the KV cache.
    b. Prepare data for the new denoising loop
    c. For each denoising (diffusion) step:
        i.   Run the decoder, taking the current canvas, the encoder KV cache, and the self-conditioning logits
             (if available) as inputs.
        ii.  Select new canvas tokens from the output logits.
        iii. Apply the sampler acceptance and renoising logic.
        iv.  Update the diffusion stopping criteria.
        v.   Use the output logits as self-conditioning logits for the next step.
    d. Append the new denoised canvas to the sequence of generated tokens.
    e. Check if any autoregressive stopping criteria are met, and break the outer loop if all sequences have
       met them. Replaces generated tokens in finished sequences by pad.
    f. Prepare tensors for the next block
```

Examples:

```python
>>> from transformers import DiffusionGemmaForBlockDiffusion, AutoProcessor, TextDiffusionStreamer

>>> model = DiffusionGemmaForBlockDiffusion.from_pretrained(
...     "google/diffusiongemma-26B-A4B-it", device_map="auto",
>>> )

>>> chat = [{"role": "user", "content": "Why is the sky blue?"},]
>>> processor = AutoProcessor.from_pretrained("google/diffusiongemma-26B-A4B-it")
>>> input_ids = processor.apply_chat_template(chat, tokenize=True, return_tensors="pt")

>>> streamer = TextDiffusionStreamer(tokenizer=processor.tokenizer)
>>> model.generate(input_ids.to(model.device), max_new_tokens=512, streamer=streamer)
```

## DiffusionGemmaGenerationConfig[[transformers.DiffusionGemmaGenerationConfig]]

Parameters that control the length of the output

- **max_new_tokens** (`int`, *optional*) --
  The maximum number of tokens to generate, ignoring the number of tokens in the prompt.
- **max_length** (`int`, *optional*) --
  The maximum length of the output sequence. `max_new_tokens` is recommended for controlling how many tokens
  the model generates.

Diffusion parameters

- **max_denoising_steps** (`int`) --
  The maximum number of denoising steps to perform.
- **sampler_config** (`EntropyBoundSamplerConfig`) --
  The configuration for the sampler. See [EntropyBoundSampler](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.EntropyBoundSampler) to learn how a sampler operates in a
  text diffusion model.
- **t_min** (`float`) --
  The final temperature in the schedule, i.e. at the last denoising step. See
  [LinearTemperatureScheduleLogitsProcessor](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.LinearTemperatureScheduleLogitsProcessor) for more details.
- **t_max** (`float`) --
  The initial temperature in the schedule, i.e. at the first denoising step. See
  [LinearTemperatureScheduleLogitsProcessor](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.LinearTemperatureScheduleLogitsProcessor) for more details.
- **stability_threshold** (`int`) --
  The number of steps for which the accepted canvas must be the same to trigger the stopping criteria.
  See [StableAndConfidentStoppingCriteria](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.StableAndConfidentStoppingCriteria) for more details.
- **confidence_threshold** (`float`) --
  The threshold for the mean of the entropy of temperature-scaled logits to trigger the stopping criteria.
  See [StableAndConfidentStoppingCriteria](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.StableAndConfidentStoppingCriteria) for more details.

Parameters that control the cache

- **cache_implementation** (`str`, *optional*) --
  Name of the cache class that will be instantiated in `generate`, for faster decoding. Possible values are:

  - `"dynamic"`: [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache)
  - `"static"`: [StaticCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.StaticCache)
  - `"offloaded"`: `DynamicCache(offloaded=True)`
  - `"offloaded_static"`: `StaticCache(offloaded=True)`
  - `"quantized"`: [QuantizedCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.QuantizedCache)

  If none is specified, we will use the default cache for the model (which is often [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache)). See
  our [cache documentation](https://huggingface.co/docs/transformers/en/kv_cache) for further information.
- **cache_config** (`dict`, *optional*, default to `None`) --
  Arguments used in the key-value cache class can be passed in `cache_config`.

Special tokens that can be used at generation time

- **bos_token_id** (`int`, *optional*) --
  The id of the *beginning-of-sequence* token.
- **pad_token_id** (`int`, *optional*) --
  The id of the *padding* token.
- **eos_token_id** (`Union[int, list[int]]`, *optional*) --
  The id of the *end-of-sequence* token. Optionally, use a list to set multiple *end-of-sequence* tokens.

A GenerationConfig class with parameterization customized for [DiffusionGemmaGenerationMixin.generate()](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaGenerationMixin.generate).

## EntropyBoundSamplerConfig[[transformers.EntropyBoundSamplerConfig]]

- **entropy_bound** (`float`) --
  The entropy bound. The higher this value is, the more tokens will be accepted. See the docstring of
  [EntropyBoundSampler.accept_canvas()](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.EntropyBoundSampler.accept_canvas) for more details on how it is applied.

Configuration class for the entropy bound sampler.

## EntropyBoundSampler[[transformers.EntropyBoundSampler]]

- **config** (`EntropyBoundSamplerConfig`) --
  The configuration of the sampler.
- **canvas_length** (`int`) --
  The length of the canvas.
- **vocab_size** (`int`) --
  The size of the vocabulary.
- **max_denoising_steps** (`int`) --
  The maximum number of denoising steps. (Unused in this sampler)

Sampler class that initializes a canvas with random tokens, accepts tokens based on token-level entropy, and
renoises non-accepted tokens.

Here is a rough sketch of how the sampler loop works:
```
          +-----------------------+
          | Canvas initialization |
          | x_T ∈ U(V)            |
          +-----------+-----------+
                      |
                      v
           +----------+---------+       +---------------------+
+--------->| Current canvas x_t |------>| Denoiser canvas x_D |
|          +----------+---------+       +----------+----------+
|                      \                          /
|                       \                        /
|                        \   Acceptance logic   /
|                         v                    v
|                       +-------------------------+
| Stop if max           | Accepted canvas x_{t-1} |
| denosing steps        +------------+------------+      +-------------------+
| reached or                          \                  | New canvas ∈ U(V) |
| adaptive stopping                    \                 +---------+---------+
| triggers                              \    Renoising logic      /
|                                        v                       v
|                                       +-------------------------+
+---------------------------------------| Next canvas x_{t-1}     |
                                        +-------------------------+
```

- **current_canvas** (`torch.LongTensor`) --
  The current canvas.
- **denoiser_canvas** (`torch.LongTensor`) --
  The canvas sampled from the denoiser predictions.
- **logits** (`torch.FloatTensor`) --
  The logits from the denoiser.
- **cur_step** (`int`) --
  The current step.torch.LongTensorThe accepted canvas.

Accepts tokens from the denoiser based on an entropy bound. More concretely, sampling proceeds by accepting
k tokens with lowest entropy, such that

sum_i^k entropy_i - max(entropy_1, ..., entropy_k) <= entropy_bound,

where the LHS is the upper bound on the joint mutual information between these tokens, and thus the sampler
chooses k tokens that they are approximately independent.

Originally proposed in https://arxiv.org/pdf/2505.24857

Initializes and returns a new canvas of `canvas_length` tokens with random values from the vocabulary.

- **accepted_canvas** (`torch.LongTensor`) --
  The accepted canvas.
- **cur_step** (`int`) --
  The current step. (Unused in this sampler)torch.LongTensorThe renoised canvas.

Renoises all non-accepted tokens.

## StableAndConfidentStoppingCriteria[[transformers.StableAndConfidentStoppingCriteria]]

- **stability_threshold** (`int`) --
  The number of steps for which the accepted canvas must be the same to trigger the stopping criteria.
- **confidence_threshold** (`float`) --
  The threshold for the mean of the entropy of temperature-scaled logits to trigger the stopping criteria.

Adaptive stopping strategy that stops when the diffusion process is confident and stable. To be more specific:
- The diffusion process is stable when the accepted canvas are the same across `stability_threshold` steps.
- The diffusion process is confident when the mean of the entropy of the processed logits is below
  `confidence_threshold`.

## LinearTemperatureScheduleLogitsProcessor[[transformers.LinearTemperatureScheduleLogitsProcessor]]

- **t_min** (`float`) --
  The final temperature in the schedule, i.e. at the last denoising step.
- **t_max** (`float`) --
  The initial temperature in the schedule, i.e. at the first denoising step.
- **max_denoising_steps** (`int`) --
  The maximum number of denoising steps.

Logits processor that applies a linear temperature schedule to the logits. This is similar to
`TemperatureLogitsWarper`, except that the temperature is a function of the current step.

At step n out of N, the temperature t is given by t = t_min + ((t_max - t_min) * (n/N)).

## DiffusionGemmaPreTrainedModel[[transformers.DiffusionGemmaPreTrainedModel]]

- **config** ([PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

## DiffusionGemmaModel[[transformers.DiffusionGemmaModel]]

- **config** ([DiffusionGemmaConfig](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Diffusion Gemma Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Uncached token IDs for the prompt to be encoded as context for the canvas.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)` or `dict`, *optional*) --
  Mask for the input tokens.
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, canvas_length)`, *optional*) --
  Token IDs for the canvas to be refined.
- **self_conditioning_logits** (`torch.FloatTensor` of shape `(batch_size, canvas_length, vocab_size)`, *optional*) --
  Self-conditioning logits from the previous denoising step, used to compute the
  self-conditioning embeddings.
- **self_conditioning_mask** (`torch.BoolTensor` of shape `(batch_size,)`, *optional*) --
  Per-example mask over `self_conditioning_logits`: examples set to `False` get a zeroed self-conditioning
  signal, as if no logits were passed for them. Useful for training, where self-conditioning is enabled per
  example with some probability.
- **decoder_attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length+canvas_length)` or `dict`, *optional*) --
  Attention mask for the decoder KV cache. Used to specify padded/unpopulated encoder KV cached entries.
- **decoder_position_ids** (`torch.LongTensor` of shape `(batch_size, canvas_length)`, *optional*) --
  The position IDs for the tokens in the canvas.`DiffusionGemmaModelOutputWithPast` or `tuple(torch.FloatTensor)`A `DiffusionGemmaModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DiffusionGemmaConfig](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaConfig)) and inputs.
The [DiffusionGemmaModel](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden states at the output of the last layer of the encoder. Only set when `input_ids` is
  provided, e.g. to compute an autoregressive loss on the encoder during training.

## DiffusionGemmaEncoderModel[[transformers.DiffusionGemmaEncoderModel]]

- **config** ([DiffusionGemmaConfig](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The DiffusionGemma encoder model comprising a vision backbone and a language model, *without* a language modeling
head. It is very similar to Gemma4Model, except that it doesn't support audio or video inputs, and always
assumes the MoE code path in the inner layers.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details ([Gemma4Processor](/docs/transformers/v5.14.0/en/model_doc/gemma4#transformers.Gemma4Processor) uses
  `image_processor_class` for processing images).
- **attention_mask** (`Union[torch.Tensor, dict]` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **mm_token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens matching each modality. For example text (0), image (1), video (2).
  Multimodal token type ids can be obtained using [AutoProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoProcessor). See [ProcessorMixin.__call__()](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **image_position_ids** (`torch.LongTensor` of shape `(batch_size, max_patches, 2)`, *optional*) --
  2D patch position coordinates from the image processor, with `(-1, -1)` indicating padding.
  Passed through to the vision encoder for positional embedding computation.[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DiffusionGemmaConfig](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaConfig)) and inputs.
The [DiffusionGemmaEncoderModel](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaEncoderModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

## DiffusionGemmaEncoderTextModel[[transformers.DiffusionGemmaEncoderTextModel]]

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`Union[torch.Tensor, dict]` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DiffusionGemmaConfig](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaConfig)) and inputs.
The [DiffusionGemmaEncoderTextModel](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaEncoderTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

## DiffusionGemmaDecoderModel[[transformers.DiffusionGemmaDecoderModel]]

Decoder model for DiffusionGemma.

Processes canvas tokens with bidirectional self-attention and cross-attention to the encoder's KV cache.
The decoder reads but does not update the KV cache. Excluding these differences, it is similar to
`DiffusionGemmaEncoderTextModel`, and they share all weights they have in common.

- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, canvas_length)`) --
  Token IDs for the canvas to be refined.
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **self_conditioning_logits** (`torch.FloatTensor` of shape `(batch_size, canvas_length, vocab_size)`, *optional*) --
  Self-conditioning logits from the previous denoising step, used to compute the
  self-conditioning embeddings.
- **self_conditioning_mask** (`torch.BoolTensor` of shape `(batch_size,)`, *optional*) --
  Per-example mask over `self_conditioning_logits`: examples set to `False` get a zeroed self-conditioning
  signal, as if no logits were passed for them. Useful for training, where self-conditioning is enabled per
  example with some probability.
- **decoder_attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length+canvas_length)` or `dict`, *optional*) --
  Attention mask for the decoder KV cache. Used to specify padded/unpopulated encoder KV cached entries.
- **decoder_position_ids** (`torch.LongTensor` of shape `(batch_size, canvas_length)`, *optional*) --
  The position IDs for the tokens in the canvas.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DiffusionGemmaConfig](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaConfig)) and inputs.
The [DiffusionGemmaDecoderModel](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaDecoderModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## DiffusionGemmaForBlockDiffusion[[transformers.DiffusionGemmaForBlockDiffusion]]

DiffusionGemma model for block diffusion. It calls `DiffusionGemmaModel` to obtains the hidden states for
the input canvas, conditioned by a prompt KV cache. Using its LM Head and self-conditioning blocks, it converts
those hidden states into logits to sample the next canvas, as well as the self-conditioning embeddings for the
next block diffusion step.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Uncached token IDs for the prompt to be encoded as context for the canvas.
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)` or `dict`, *optional*) --
  Mask for the input tokens.
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, canvas_length)`, *optional*) --
  Token IDs for the canvas to be refined.
- **self_conditioning_logits** (`torch.FloatTensor` of shape `(batch_size, canvas_length, vocab_size)`, *optional*) --
  Self-conditioning logits from the previous denoising step, used to compute the self-conditioning
  embeddings.
- **self_conditioning_mask** (`torch.BoolTensor` of shape `(batch_size,)`, *optional*) --
  Per-example mask over `self_conditioning_logits`: examples set to `False` get a zeroed self-conditioning
  signal, as if no logits were passed for them. Useful for training, where self-conditioning is enabled per
  example with some probability.
- **decoder_attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length+canvas_length)` or `dict`, *optional*) --
  Attention mask for the decoder KV cache. Used to specify padded/unpopulated encoder KV cached entries.
- **decoder_position_ids** (`torch.LongTensor` of shape `(batch_size, canvas_length)`, *optional*) --
  The position IDs for the tokens in the canvas.`DiffusionGemmaBlockDiffusionOutputWithPast` or `tuple(torch.FloatTensor)`A `DiffusionGemmaBlockDiffusionOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DiffusionGemmaConfig](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaConfig)) and inputs.
The [DiffusionGemmaForBlockDiffusion](/docs/transformers/v5.14.0/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaForBlockDiffusion) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*) -- Language modeling loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, canvas_length, config.text_config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden states at the output of the last layer of the encoder. Only set when `input_ids` is
  provided, e.g. to compute an autoregressive loss on the encoder during training.

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, DiffusionGemmaForBlockDiffusion

>>> model = DiffusionGemmaForBlockDiffusion.from_pretrained("google/diffusiongemma-26B-A4B-it")
>>> processor = AutoProcessor.from_pretrained("google/diffusiongemma-26B-A4B-it")

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
