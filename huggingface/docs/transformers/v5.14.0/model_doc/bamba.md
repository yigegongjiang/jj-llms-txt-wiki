# Bamba

[Bamba](https://huggingface.co/blog/bamba) is a 9B parameter decoder-only language model built on the [Mamba-2](./mamba2) architecture. It is pretrained in two stages - it starts by training on 2T tokens from the [Dolma v1.7](https://huggingface.co/datasets/allenai/dolma) dataset and then trained on an additional 200B tokens from [FineWeb](https://huggingface.co/datasets/HuggingFaceFW/fineweb) and [Cosmopedia](https://huggingface.co/datasets/HuggingFaceTB/cosmopedia).

You can find all the original Bamba checkpoints under the [Bamba](https://huggingface.co/collections/ibm-ai-platform/bamba-674f1388b9bbc98b413c7bab) collection.

> [!TIP]
> This model was contributed by [ani300](https://github.com/ani300) and [fabianlim](https://github.com/fabianlim).
>
> Click on the Bamba models in the right sidebar for more examples of how to apply Bamba to different text generation tasks.

The example below demonstrates how to generate text with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline), [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel), and from the command line.

```python
from transformers import pipeline

pipeline = pipeline(
    task="text-generation",
    model="ibm-ai-platform/Bamba-9B-v2",
    device=0
)
pipeline("Plants create energy through a process known as")
```

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("ibm-ai-platform/Bamba-9B-v2")
model = AutoModelForCausalLM.from_pretrained("ibm-ai-platform/Bamba-9B-v2", device_map="auto", attn_implementation="sdpa")
input_ids = tokenizer("Plants create energy through a process known as", return_tensors="pt").to(model.device)

output = model.generate(**input_ids)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [torchao](../quantization/torchao) to only quantize the weights to int4.

```python
from transformers import AutoModelForCausalLM, AutoTokenizer, TorchAoConfig

quantization_config = TorchAoConfig("int4_weight_only", group_size=128)
tokenizer = AutoTokenizer.from_pretrained("ibm-ai-platform/Bamba-9B-v2")
model = AutoModelForCausalLM.from_pretrained(
   "ibm-ai-platform/Bamba-9B-v2",
   quantization_config=quantization_config,
   device_map="auto",
   attn_implementation="sdpa"
)

inputs = tokenizer("Plants create energy through a process known as", return_tensors="pt").to(model.device)
output = model.generate(**inputs)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

## Notes

- Bamba supports padding-free training which concatenates distinct training examples while still processing inputs as separate batches. It can significantly accelerate inference by [~2x](https://github.com/huggingface/transformers/pull/35861#issue-2807873129) (depending on model and data distribution) and reduce memory-usage if there are examples of varying lengths by avoiding unnecessary compute and memory overhead from padding tokens.

  Padding-free training requires the `flash-attn`, `mamba-ssm`, and `causal-conv1d` packages and the following arguments must be passed to the model in addition to `input_ids` and `labels`.

  - `position_ids: torch.LongTensor`: the position index of each token in each sequence.
  - `seq_idx: torch.IntTensor`: the index of each sequence in the batch.
  - Each of the `FlashAttentionKwargs`
    - `cu_seq_lens_q: torch.LongTensor`: the cumulative sequence lengths of all queries.
    - `cu_seq_lens_k: torch.LongTensor`: the cumulative sequence lengths of all keys.
    - `max_length_q: int`: the longest query length in the batch.
    - `max_length_k: int`: the longest key length in the batch.

  The `attention_mask` inputs should not be provided. The [DataCollatorWithFlattening](/docs/transformers/v5.14.0/en/main_classes/data_collator#transformers.DataCollatorWithFlattening) programmatically generates the set of additional arguments above using `return_seq_idx=True` and `return_flash_attn_kwargs=True`. See the [Improving Hugging Face Training Efficiency Through Packing with Flash Attention](https://huggingface.co/blog/packing-with-FA2) blog post for additional information.

  ```python
  from transformers import DataCollatorWithFlattening

  # Example of using padding-free training
  data_collator = DataCollatorWithFlattening(
      tokenizer=tokenizer,
      return_seq_idx=True,
      return_flash_attn_kwargs=True
  )
  ```

## BambaConfig[[transformers.BambaConfig]]

- **vocab_size** (`int`, *optional*, defaults to `128000`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **hidden_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `14336`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `32`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `8`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **num_logits_to_keep** (`int` or `None`, *optional*, defaults to 1) --
  Number of prompt logits to calculate during generation. If `None`, all logits will be calculated. If an
  integer value, only last `num_logits_to_keep` logits will be calculated. Default is 1 because only the
  logits of the last prompt token are needed for generation. For long sequences, the logits for the entire
  sequence may use a lot of memory so, setting `num_logits_to_keep=1` will reduce memory footprint
  significantly.
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **max_position_embeddings** (`int`, *optional*, defaults to `262144`) --
  The maximum sequence length that this model might ever be used with.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **attn_layer_indices** (`list`, *optional*) --
  Specifies the layer indices that will have full attention. Must contain values at most num_hidden_layers.
- **mamba_n_heads** (`int`, *optional*, defaults to `128`) --
  The number of mamba heads used in the v2 implementation.
- **mamba_d_head** (`Union[str, int]`, *optional*, defaults to `auto`) --
  Head embedding dimension size
- **mamba_n_groups** (`int`, *optional*, defaults to `1`) --
  The number of the mamba groups used in the v2 implementation.
- **mamba_d_state** (`int`, *optional*, defaults to `256`) --
  Size of the SSM state (latent state dimension) in the Mamba layers.
- **mamba_d_conv** (`int`, *optional*, defaults to `4`) --
  The size of the mamba convolution kernel
- **mamba_expand** (`int`, *optional*, defaults to `2`) --
  Expanding factor (relative to hidden_size) used to determine the mamba intermediate size
- **mamba_chunk_size** (`int`, *optional*, defaults to `256`) --
  The chunks in which to break the sequence when doing prefill/training
- **mamba_conv_bias** (`bool`, *optional*, defaults to `True`) --
  Flag indicating whether or not to use bias in the convolution layer of the mamba mixer block.
- **mamba_proj_bias** (`bool`, *optional*, defaults to `False`) --
  Flag indicating whether or not to use bias in the input and output projections (["in_proj", "out_proj"]) of the mamba mixer block
- **time_step_min** (`float`, *optional*, defaults to `0.001`) --
  Minimum `time_step` used to bound `dt_proj.bias`.
- **time_step_max** (`float`, *optional*, defaults to `0.1`) --
  Maximum `time_step` used to bound `dt_proj.bias`.
- **time_step_limit** (`Union[list[float], tuple[float, float]]`, *optional*, defaults to `(0.0, inf)`) --
  Accepted range of time step values for clamping.
- **z_loss_coefficient** (`float`, *optional*, defaults to 0.0) --
  Coefficient for auxiliary z-loss used to control logit growth during training
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **mlp_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

The BambaModel is a hybrid [mamba2](https://github.com/state-spaces/mamba) architecture with SwiGLU.
The checkpoints are  jointly trained by IBM, Princeton, and UIUC.

## BambaModel[[transformers.BambaModel]]

- **config** ([BambaConfig](/docs/transformers/v5.14.0/en/model_doc/bamba#transformers.BambaConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Bamba Model outputting raw hidden-states without any specific head on top.

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
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
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
  model's internal embedding lookup matrix.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([BambaConfig](/docs/transformers/v5.14.0/en/model_doc/bamba#transformers.BambaConfig)) and inputs.
The [BambaModel](/docs/transformers/v5.14.0/en/model_doc/bamba#transformers.BambaModel) forward method, overrides the `__call__` special method.

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

## BambaForCausalLM[[transformers.BambaForCausalLM]]

- **config** ([BambaForCausalLM](/docs/transformers/v5.14.0/en/model_doc/bamba#transformers.BambaForCausalLM)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Bamba Model for causal language modeling.

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
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
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
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([BambaConfig](/docs/transformers/v5.14.0/en/model_doc/bamba#transformers.BambaConfig)) and inputs.
The [BambaForCausalLM](/docs/transformers/v5.14.0/en/model_doc/bamba#transformers.BambaForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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

Example:

```python
>>> from transformers import AutoTokenizer, BambaForCausalLM

>>> model = BambaForCausalLM.from_pretrained("...")
>>> tokenizer = AutoTokenizer.from_pretrained("...")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```
