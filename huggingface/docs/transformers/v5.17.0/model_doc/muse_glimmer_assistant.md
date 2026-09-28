# MuseGlimmerAssistant

[MuseGlimmerAssistant](https://research.meta.ai/blog/introducing-muse-glimmer-open-agentic-model) is the [DFlash](https://huggingface.co/papers/2602.06036) drafter for [MuseGlimmer](./muse_glimmer). It is not a standalone language model. It has 5 sliding window layers and no embeddings of its own. It borrows the main model's input and output embeddings, and reads the main model's hidden states at `target_layer_ids` (layers 1, 13, 25, 37, and 49 by default) as context.

Rather than drafting one token at a time, the drafter denoises a whole block of `block_size` masked tokens in a single forward pass, like a diffusion window. The main model then verifies the block in one step. Meta reports 3.1x faster decoding on an RTX 5090 and 1.5-1.8x on Apple M-series chips.

Pass the drafter to [generate()](/docs/transformers/v5.17.0/en/main_classes/text_generation#transformers.GenerationMixin.generate) as `assistant_model` and set `speculation_type="dflash"`. The drafter must be loaded in the same dtype and on the same device as the main model.

```python
from transformers import AutoProcessor, MuseGlimmerAssistantModel, MuseGlimmerForConditionalGeneration

processor = AutoProcessor.from_pretrained("meta-models/Muse-Glimmer-30B")
model = MuseGlimmerForConditionalGeneration.from_pretrained(
    "meta-models/Muse-Glimmer-30B",
    device_map="auto",
)
drafter = MuseGlimmerAssistantModel.from_pretrained(
    "meta-models/Muse-Glimmer-30B-assistant",
    device_map="auto",
)

messages = [
    {
        "role": "user",
        "content": [{"type": "text", "text": "Write a bash one-liner that counts lines of Python in a repo."}],
    },
]
inputs = processor.apply_chat_template(
    messages,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
).to(model.device)
input_len = inputs["input_ids"].shape[-1]

outputs = model.generate(
    **inputs,
    assistant_model=drafter,
    speculation_type="dflash",
    max_new_tokens=256,
)
response = processor.decode(outputs[0][input_len:], skip_special_tokens=False)
print(response)
```

## Notes

- The drafter needs the main model's hidden states, so `generate` forces `output_hidden_states=True` for the target model when `speculation_type="dflash"`.
- See the [Meta is back with Muse Glimmer: local, agentic, multimodal, and open source!](https://huggingface.co/blog/muse-glimmer) blog post for more details and example usage.

## MuseGlimmerAssistantConfig[[transformers.MuseGlimmerAssistantConfig]]

#### transformers.MuseGlimmerAssistantConfig[[transformers.MuseGlimmerAssistantConfig]]

```python
transformers.MuseGlimmerAssistantConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 6656, intermediate_size: int = 19968, num_hidden_layers: int = 5, num_attention_heads: int = 32, num_key_value_heads: int = 8, head_dim: int = 128, rms_norm_eps: float = 1e-05, rope_parameters: dict | None = None, max_position_embeddings: int = 131072, sliding_window: int = 2048, layer_types: list[str] | None = None, attention_dropout: float | int = 0, hidden_act: str = 'silu', bos_token_id: int | None = 200000, eos_token_id: int | None = 200001, pad_token_id: int | None = 200018, block_size: int = 16, mask_token_id: int = 201818, target_layer_ids: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/muse_glimmer_assistant/configuration_muse_glimmer_assistant.py#L27)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `6656`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `19968`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `5`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

rope_parameters (`dict`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

max_position_embeddings (`int`, *optional*, defaults to `131072`) : The maximum sequence length that this model might ever be used with.

sliding_window (`int`, *optional*, defaults to `2048`) : Sliding window attention window size. If `None`, no sliding window is applied.

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0`) : The dropout ratio for the attention probabilities.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

bos_token_id (`int`, *optional*, defaults to `200000`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`int`, *optional*, defaults to `200001`) : Token id used for end-of-stream in the vocabulary.

pad_token_id (`int`, *optional*, defaults to `200018`) : Token id used for padding in the vocabulary.

block_size (`int`, *optional*) : The block size of noise inputs that will be denoised.

mask_token_id (`int`, *optional*) : Mask token ids used as noisey input to model.

target_layer_ids (`list[int]`, *optional*) : Zero indexed layer ids whose hidden states are concatenated as context for the model.

This is the configuration class to store the configuration of a MuseGlimmerAssistantModel. It is used to instantiate a Muse Glimmer Assistant
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [meta-models/Muse-Glimmer-30B-assistant](https://huggingface.co/meta-models/Muse-Glimmer-30B-assistant)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import MuseGlimmerAssistantConfig, MuseGlimmerAssistantModel

>>> # Initializing a Muse Glimmer Assistant config similar to `meta-models/Muse-Glimmer-30B-assistant`.
>>> configuration = MuseGlimmerAssistantConfig(text_config)

>>> # Initializing a model from the `meta-models/Muse-Glimmer-30B-assistant` configuration.
>>> model = MuseGlimmerAssistantModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MuseGlimmerAssistantPreTrainedModel[[transformers.MuseGlimmerAssistantPreTrainedModel]]

#### transformers.MuseGlimmerAssistantPreTrainedModel[[transformers.MuseGlimmerAssistantPreTrainedModel]]

```python
transformers.MuseGlimmerAssistantPreTrainedModel(config: PreTrainedConfig, *inputs, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/muse_glimmer_assistant/modeling_muse_glimmer_assistant.py#L275)

**Parameters:**

config ([PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## MuseGlimmerAssistantModel[[transformers.MuseGlimmerAssistantModel]]

#### transformers.MuseGlimmerAssistantModel[[transformers.MuseGlimmerAssistantModel]]

```python
transformers.MuseGlimmerAssistantModel(config: MuseGlimmerAssistantConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/muse_glimmer_assistant/modeling_muse_glimmer_assistant.py#L371)

**Parameters:**

config ([MuseGlimmerAssistantConfig](/docs/transformers/v5.17.0/en/model_doc/muse_glimmer_assistant#transformers.MuseGlimmerAssistantConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Muse Glimmer Assistant Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MuseGlimmerAssistantModel.forward]]

```python
forward(noise_embeds: FloatTensor, context_hidden_states: FloatTensor, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.DFlashCache | None = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/muse_glimmer_assistant/modeling_muse_glimmer_assistant.py#L385)

**Parameters:**

noise_embeds (`torch.FloatTensor` of shape `[batch_size, config.block_size, dim]`) : Input embedding for the last generated anchor token and mask tokens to be denoised.

context_hidden_states (`torch.FloatTensor` of shape `[batch_size, number_of_previous_accepted_tokens, dim * len(config.target_layer_ids)]`) : Context hidden states from target model's selected layer ids concatenated in the last dim.

attention_mask (`torch.Tensor` of shape `[batch_size, number_of_previous_accepted_tokens + config.block_size]`) : Similar to the usual attention_mask, but note that it has length `number_of_previous_accepted_tokens + config.block_size`, because the Attention will first concatenate `context_hidden_states` and the hidden states derived from `noise_embeds`, so that k/v states do not have the same length as q_states, even before the `cache.update()` call. Thus the kv_seq_len dimension of the attention mask needs to span the additional positions.

position_ids (`torch.Tensor` of shape `[batch_size, number_of_previous_accepted_tokens + config.block_size]`) : Similar to the usual position_ids, but note that it has length `number_of_previous_accepted_tokens + config.block_size`, because the Attention will first concatenate `context_hidden_states` and the hidden states derived from `noise_embeds`, so that k/v states do not have the same length as q_states, even before the `cache.update()` call. Thus the `position_ids` and the derived `position_embeddings` need to span all the additional positions.

past_key_values (`~cache_utils.DFlashCache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MuseGlimmerAssistantConfig](/docs/transformers/v5.17.0/en/model_doc/muse_glimmer_assistant#transformers.MuseGlimmerAssistantConfig)) and inputs.

The [MuseGlimmerAssistantModel](/docs/transformers/v5.17.0/en/model_doc/muse_glimmer_assistant#transformers.MuseGlimmerAssistantModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
