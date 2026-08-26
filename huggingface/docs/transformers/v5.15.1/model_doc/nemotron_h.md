# NemotronH

[NemotronH](https://huggingface.co/papers/2504.03624) is a hybrid architecture combining attention and state-space layers for efficient long-context language modeling. It interleaves Mamba2 and transformer blocks, using a fixed ratio to balance expressiveness with linear-time sequence processing.

The example below demonstrates how to generate text with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModelForCausalLM](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModelForCausalLM) class.

```python
from transformers import pipeline

pipe = pipeline(
    task="text-generation",
    model="nvidia/Nemotron-H-8B-Reasoning-128K",
)
pipe("Plants create energy through a process known as")
```

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("nvidia/Nemotron-H-8B-Reasoning-128K")
model = AutoModelForCausalLM.from_pretrained(
    "nvidia/Nemotron-H-8B-Reasoning-128K",
    device_map="auto",
)
input_ids = tokenizer("Plants create energy through a process known as", return_tensors="pt").to(model.device)

output = model.generate(**input_ids, max_new_tokens=50)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

## NemotronHConfig[[transformers.NemotronHConfig]]

#### transformers.NemotronHConfig[[transformers.NemotronHConfig]]

```python
transformers.NemotronHConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 131072, hidden_size: int = 4096, layers_block_type: list[str] | None = None, tie_word_embeddings: bool = False, use_cache: bool = True, num_logits_to_keep: int = 1, pad_token_id: int | None = 0, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 2, num_attention_heads: int = 32, num_key_value_heads: int = 8, head_dim: int = 128, max_position_embeddings: int = 4096, attention_bias: bool = False, attention_dropout: float | int = 0.0, sliding_window: int | None = None, intermediate_size: int = 21504, mlp_hidden_act: str = 'relu2', mlp_bias: bool = False, use_mamba_kernels: bool = True, ssm_state_size: int = 128, mamba_num_heads: int = 128, mamba_head_dim: int = 64, mamba_hidden_act: str = 'silu', n_groups: int = 8, conv_kernel: int = 4, expand: int = 2, time_step_min: float = 0.001, time_step_max: float = 0.1, time_step_limit: list[float] | tuple[float, ...] = (0.0, inf), time_step_floor: float = 0.0001, use_conv_bias: bool = True, chunk_size: int = 128, mamba_proj_bias: bool = False, mamba_ssm_cache_dtype: str = 'float32', n_routed_experts: int = 8, n_shared_experts: int = 1, moe_intermediate_size: int = 7688, moe_shared_expert_intermediate_size: int = 7688, moe_latent_size: int | None = None, moe_shared_expert_overlap: bool = True, num_experts_per_tok: int = 2, routed_scaling_factor: float | int = 1.0, n_group: int = 1, topk_group: int = 1, norm_topk_prob: bool = True, num_nextn_predict_layers: int = 0, mtp_layers_block_type: list[str] | None = None, use_bias: bool = False, initializer_range: float = 0.02, layer_norm_epsilon: float = 1e-05, residual_in_fp32: bool = False, hidden_dropout: float | int = 0.0, rescale_prenorm_residual: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron_h/configuration_nemotron_h.py#L27)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `131072`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `4096`) : Dimension of the hidden representations.

layers_block_type (`list`, *optional*) : Explicit list of layer types for each layer. Each element must be one of: "mlp", "linear_attention", "full_attention", or "moe". The number of layers is determined by the length of this list.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

num_logits_to_keep (`int`, *optional*, defaults to 1) : Number of prompt logits to calculate during generation. If `None`, all logits will be calculated.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `128`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

max_position_embeddings (`int`, *optional*, defaults to `4096`) : The maximum sequence length that this model might ever be used with.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

sliding_window (`int`, *optional*) : Sliding window attention window size. If `None`, no sliding window is applied.

intermediate_size (`int`, *optional*, defaults to `21504`) : Dimension of the MLP representations.

mlp_hidden_act (`str`, *optional*, defaults to `relu2`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

use_mamba_kernels (`bool`, *optional*, defaults to `True`) : Flag indicating whether or not to use the fast mamba kernels.

ssm_state_size (`int`, *optional*, defaults to 128) : The dimension of the mamba state space latents.

mamba_num_heads (`int`, *optional*, defaults to `128`) : The number of mamba heads used in the v2 implementation.

mamba_head_dim (`int`, *optional*, defaults to `64`) : Head embedding dimension size

mamba_hidden_act (`str`, *optional*, defaults to `"silu"`) : The non-linear activation function in the Mamba layers.

n_groups (`int`, *optional*, defaults to 8) : Number of groups for the evolution matrices of the Mamba layers.

conv_kernel (`int`, *optional*, defaults to `4`) : The size of the convolutional kernel.

expand (`int`, *optional*, defaults to 2) : Expanding factor used to determine the intermediate size in the Mamba layers.

time_step_min (`float`, *optional*, defaults to `0.001`) : Minimum `time_step` used to bound `dt_proj.bias`.

time_step_max (`float`, *optional*, defaults to `0.1`) : Maximum `time_step` used to bound `dt_proj.bias`.

time_step_limit (`Union[list[float], tuple[float, ...]]`, *optional*, defaults to `(0.0, inf)`) : Accepted range of time step values for clamping.

time_step_floor (`float`, *optional*, defaults to `0.0001`) : Minimum allowed value for the discrete time step delta after softplus activation.

use_conv_bias (`bool`, *optional*, defaults to `True`) : Whether or not to use bias in the convolution layer of the Mamba mixer block.

chunk_size (`int`, *optional*, defaults to 128) : Size of the chunks that will comprise the sequence in the Mamba layers.

mamba_proj_bias (`bool`, *optional*, defaults to `False`) : Flag indicating whether or not to use bias in the input and output projections (["in_proj", "out_proj"]) of the mamba mixer block

mamba_ssm_cache_dtype (`str`, *optional*, defaults to `"float32"`) : Data type for Mamba SSM cache states.

n_routed_experts (`int`, *optional*, defaults to `8`) : Number of routed experts.

n_shared_experts (`int`, *optional*, defaults to `1`) : Number of shared experts.

moe_intermediate_size (`int`, *optional*, defaults to `7688`) : Intermediate size of the routed expert MLPs.

moe_shared_expert_intermediate_size (`int`, *optional*, defaults to 7688) : Dimension of the MLP representations in shared experts.

moe_latent_size (`int`, *optional*) : Latent size for MoE expert projections. If `None`, uses `hidden_size`.

moe_shared_expert_overlap (`bool`, *optional*, defaults to `True`) : Whether shared experts overlap with routed experts.

num_experts_per_tok (`int`, *optional*, defaults to `2`) : Number of experts to route each token to. This is the top-k value for the token-choice routing.

routed_scaling_factor (`Union[float, int]`, *optional*, defaults to `1.0`) : Scaling factor or routed experts.

n_group (`int`, *optional*, defaults to 1) : Number of groups for expert routing.

topk_group (`int`, *optional*, defaults to `1`) : Number of selected groups for each token (for each token, ensuring the selected experts is only within `topk_group` groups).

norm_topk_prob (`bool`, *optional*, defaults to `True`) : Whether to normalize the weights of the routed experts. 

num_nextn_predict_layers (`int`, *optional*, defaults to 0) : Number of additional layers for multi-token prediction. If 0, multi-token prediction is disabled.

mtp_layers_block_type (`list`, *optional*, defaults to `['full_attention', 'moe']`) : Explicit list of layer types for multi-token prediction layers when `num_nextn_predict_layers` > 0.

use_bias (`bool`, *optional*, defaults to `False`) : Whether to use bias in the model.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_epsilon (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

residual_in_fp32 (`bool`, *optional*, defaults to `False`) : Whether or not residuals should be in `float32`.

hidden_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

rescale_prenorm_residual (`bool`, *optional*, defaults to `True`) : Whether to rescale the pre-normalization residual connections.

This is the configuration class to store the configuration of a NemotronHModel. It is used to instantiate a Nemotron H
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16](https://huggingface.co/nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import NemotronHModel, NemotronHConfig

>>> # Initializing a NemotronH configuration
>>> configuration = NemotronHConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = NemotronHModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

#### validate_layer_type[[transformers.NemotronHConfig.validate_layer_type]]

```python
validate_layer_type()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron_h/configuration_nemotron_h.py#L198)

Validate layers_block_type list.

## NemotronHModel[[transformers.NemotronHModel]]

#### transformers.NemotronHModel[[transformers.NemotronHModel]]

```python
transformers.NemotronHModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron_h/modeling_nemotron_h.py#L1028)

#### forward[[transformers.NemotronHModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, use_cache: bool | None = None, attention_mask: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron_h/modeling_nemotron_h.py#L1045)

## NemotronHForCausalLM[[transformers.NemotronHForCausalLM]]

#### transformers.NemotronHForCausalLM[[transformers.NemotronHForCausalLM]]

```python
transformers.NemotronHForCausalLM(config: NemotronHConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron_h/modeling_nemotron_h.py#L1109)

#### forward[[transformers.NemotronHForCausalLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/nemotron_h/modeling_nemotron_h.py#L1121)

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
elements depending on the configuration ([NemotronHConfig](/docs/transformers/v5.15.1/en/model_doc/nemotron_h#transformers.NemotronHConfig)) and inputs.

The [NemotronHForCausalLM](/docs/transformers/v5.15.1/en/model_doc/nemotron_h#transformers.NemotronHForCausalLM) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, NemotronHForCausalLM

>>> model = NemotronHForCausalLM.from_pretrained("Zyphra/NemotronH-7B-v1")
>>> tokenizer = AutoTokenizer.from_pretrained("Zyphra/NemotronH-7B-v1")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```
