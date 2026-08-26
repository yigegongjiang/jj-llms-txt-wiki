# FalconH1

## Overview

The [FalconH1](https://huggingface.co/blog/tiiuae/falcon-h1) model was developed by the TII Pretraining team. A comprehensive research paper covering the architecture, pretraining dynamics, experimental results, and conclusions is forthcoming. You can read more about this series in [this website](https://github.com/tiiuae/Falcon-H1).

## Contributors

This model was contributed by [DhiyaEddine](https://huggingface.co/DhiyaEddine), [ybelkada](https://huggingface.co/ybelkada), [JingweiZuo](https://huggingface.co/JingweiZuo), [IlyasChahed](https://huggingface.co/IChahed), and [MaksimVelikanov](https://huggingface.co/yellowvm).
The original code can be found [here](https://github.com/tiiuae/Falcon-H1).

## FalconH1Config[[transformers.FalconH1Config]]

| Model     | Depth | Dim  | Attn Heads | KV | Mamba Heads | d_head       | d_state | Ctx Len        |
|-----------|--------|------|------------|----|--------------|--------------|------|-----------------|
| H1 0.5B   | 36     | 1024 | 8          | 2  | 24           | 64 / 64      | 128  | 4K, 16K-SFT     |
| H1 1.5B   | 24     | 2048 | 8          | 2  | 48           | 128 / 64     | 256  | 128K            |
| H1 1.5B-d | 66     | 1280 | 6          | 2  | 24           | 128 / 64     | 256  | 128K            |
| H1 3B     | 32     | 2560 | 10         | 2  | 32           | 128 / 128    | 256  | 128K            |
| H1 7B     | 44     | 3072 | 12         | 2  | 24           | 128 / 128    | 256  | 256K            |
| H1 34B    | 72     | 5120 | 20         | 4  | 32           | 128 / 128    | 256  | 256K            |

#### transformers.FalconH1Config[[transformers.FalconH1Config]]

```python
transformers.FalconH1Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 128000, tie_word_embeddings: bool = False, hidden_size: int = 4096, intermediate_size: int = 14336, num_hidden_layers: int = 32, num_attention_heads: int = 32, num_key_value_heads: int | None = 8, hidden_act: str = 'silu', initializer_range: float = 0.02, rms_norm_eps: float = 1e-05, use_cache: bool | None = True, num_logits_to_keep: int | None = 1, pad_token_id: int | None = 0, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 2, max_position_embeddings: int = 8192, attention_dropout: float | int | None = 0.0, mamba_d_ssm: int | None = 1024, mamba_n_heads: int | None = 128, mamba_d_head: str | int | None = 'auto', mamba_n_groups: int | None = 1, mamba_d_state: int | None = 256, mamba_d_conv: int | None = 4, mamba_expand: int | None = 2, mamba_chunk_size: int | None = 256, mamba_conv_bias: bool | None = True, mamba_proj_bias: bool | None = False, mamba_norm_before_gate: bool | None = True, mamba_rms_norm: bool | None = False, time_step_limit: list[float, float] | tuple[float, float] | None = (0.0, inf), projectors_bias: bool | None = False, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, lm_head_multiplier: float | None = 1.0, embedding_multiplier: float | None = 1.0, mlp_multipliers: list[float] | None = None, key_multiplier: float | None = 1.0, attention_out_multiplier: float | None = 1.0, attention_in_multiplier: float | None = 1.0, ssm_multipliers: list[float] | None = None, ssm_in_multiplier: float | None = 1.0, ssm_out_multiplier: float | None = 1.0, attention_bias: bool = False, mlp_bias: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/falcon_h1/configuration_falcon_h1.py#L25)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `128000`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

hidden_size (`int`, *optional*, defaults to `4096`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `14336`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

num_logits_to_keep (`int` or `None`, *optional*, defaults to 1) : Number of prompt logits to calculate during generation. If `None`, all logits will be calculated. If an integer value, only last `num_logits_to_keep` logits will be calculated. Default is 1 because only the logits of the last prompt token are needed for generation. For long sequences, the logits for the entire sequence may use a lot of memory so, setting `num_logits_to_keep=1` will reduce memory footprint significantly.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

max_position_embeddings (`int`, *optional*, defaults to `8192`) : The maximum sequence length that this model might ever be used with.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

mamba_d_ssm (`int`, *optional*, defaults to `1024`) : Inner state size of the SSM (state-space model) in the Mamba layers of FalconH1.

mamba_n_heads (`int`, *optional*, defaults to `128`) : The number of mamba heads used in the v2 implementation.

mamba_d_head (`Union[str, int]`, *optional*, defaults to `auto`) : Head embedding dimension size

mamba_n_groups (`int`, *optional*, defaults to `1`) : The number of the mamba groups used in the v2 implementation.

mamba_d_state (`int`, *optional*, defaults to `256`) : Size of the SSM state (latent state dimension) in the Mamba layers.

mamba_d_conv (`int`, *optional*, defaults to `4`) : The size of the mamba convolution kernel

mamba_expand (`int`, *optional*, defaults to `2`) : Expanding factor (relative to hidden_size) used to determine the mamba intermediate size

mamba_chunk_size (`int`, *optional*, defaults to `256`) : The chunks in which to break the sequence when doing prefill/training

mamba_conv_bias (`bool`, *optional*, defaults to `True`) : Flag indicating whether or not to use bias in the convolution layer of the mamba mixer block.

mamba_proj_bias (`bool`, *optional*, defaults to `False`) : Flag indicating whether or not to use bias in the input and output projections (["in_proj", "out_proj"]) of the mamba mixer block

mamba_norm_before_gate (`bool`, *optional*, defaults to `True`) : Whether to apply normalization before the gating mechanism in the Mamba mixer.

mamba_rms_norm (`bool`, *optional*, defaults to `False`) : Whether to use RMS normalization in the Mamba layers (as opposed to standard LayerNorm).

time_step_limit (`Union[list[float, float], tuple[float, float]]`, *optional*, defaults to `(0.0, inf)`) : Accepted range of time step values for clamping.

projectors_bias (`bool`, *optional*, defaults to `False`) : Flag indicating whether or not to use bias in the input and output projections (["in_proj", "out_proj"]) of the attention block

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

lm_head_multiplier (`float`, *optional*, defaults to 1.0) : The multiplier for the LM head. This is used to scale the output of the LM head.

embedding_multiplier (`float`, *optional*, defaults to 1.0) : The multiplier for the embedding layer. This is used to scale the output of the embedding layer.

mlp_multipliers (`list[float]`, *optional*) : The multipliers for the MLP layers. This is used to scale the output of the MLP layers. The first value is the multiplier of gate layer, the second value is the multiplier of the down_proj layer.

key_multiplier (`float`, *optional*) : The multiplier for the key layer. This is used to scale the output of the key layer.

attention_out_multiplier (`float`, *optional*) : The multiplier for the attention output layer. This is used to scale the output of the attention output

attention_in_multiplier (`float`, *optional*) : The multiplier for the attention input layer. This is used to scale the output of the attention input layer.

ssm_multipliers (`list[float]`, *optional*) : The multipliers for the SSM layers. This is used to scale the output of the SSM layers.

ssm_in_multiplier (`float`, *optional*) : The multiplier for the SSM input layer. This is used to scale the output of the SSM input layer.

ssm_out_multiplier (`float`, *optional*) : The multiplier for the SSM output layer. This is used to scale the output of the SSM output layer.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

This is the configuration class to store the configuration of a FalconH1Model. It is used to instantiate a Falcon H1
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [tiiuae/Falcon-H1-1.5B-Deep-Instruct](https://huggingface.co/tiiuae/Falcon-H1-1.5B-Deep-Instruct)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## FalconH1ForCausalLM[[transformers.FalconH1ForCausalLM]]

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained("tiiuae/Falcon-H1-7B-Instruct", device_map="auto")
tokenizer = AutoTokenizer.from_pretrained("tiiuae/Falcon-H1-7B-Instruct")

message = ["Mamba is a snake with following properties  "]
inputs = tokenizer(message, return_tensors='pt', return_token_type_ids=False).to(model.device)
response = model.generate(**inputs, max_new_tokens=64)
print(tokenizer.batch_decode(response, skip_special_tokens=True)[0])
```

#### transformers.FalconH1ForCausalLM[[transformers.FalconH1ForCausalLM]]

```python
transformers.FalconH1ForCausalLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/falcon_h1/modeling_falcon_h1.py#L1109)

**Parameters:**

config ([FalconH1ForCausalLM](/docs/transformers/v5.15.1/en/model_doc/falcon_h1#transformers.FalconH1ForCausalLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Falcon H1 Model for causal language modeling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.FalconH1ForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/falcon_h1/modeling_falcon_h1.py#L1124)

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
elements depending on the configuration ([FalconH1Config](/docs/transformers/v5.15.1/en/model_doc/falcon_h1#transformers.FalconH1Config)) and inputs.

The [FalconH1ForCausalLM](/docs/transformers/v5.15.1/en/model_doc/falcon_h1#transformers.FalconH1ForCausalLM) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, FalconH1ForCausalLM

>>> model = FalconH1ForCausalLM.from_pretrained("...")
>>> tokenizer = AutoTokenizer.from_pretrained("...")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```

This HF implementation is contributed by [younesbelkada](https://github.com/younesbelkada) and [DhiaEddineRhaiem](https://github.com/dhiaEddineRhaiem).
