# CPMAnt

[CPMAnt](https://github.com/OpenBMB/CPM-Live/tree/cpm-ant/cpm-live) is a 10B-parameter open-source Chinese pre-trained language model and the first milestone of the CPM-Live open training project. It achieves strong results with delta tuning on the CUGE benchmark, and compressed variants are available for different hardware configurations.

The example below demonstrates how to generate text with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [CpmAntForCausalLM](/docs/transformers/v5.14.0/en/model_doc/cpmant#transformers.CpmAntForCausalLM) class.

```python
from transformers import pipeline

pipe = pipeline(
    task="text-generation",
    model="openbmb/cpm-ant-10b",
)
pipe("今天天气很好，")
```

```python
from transformers import CpmAntForCausalLM, CpmAntTokenizer

tokenizer = CpmAntTokenizer.from_pretrained("openbmb/cpm-ant-10b")
model = CpmAntForCausalLM.from_pretrained(
    "openbmb/cpm-ant-10b",
    device_map="auto",
)
input_ids = tokenizer("今天天气很好，", return_tensors="pt").to(model.device)

output = model.generate(**input_ids, max_new_tokens=50)
print(tokenizer.decode(output[0], skip_special_tokens=True))
```

## CpmAntConfig[[transformers.CpmAntConfig]]

- **vocab_size** (`int`, *optional*, defaults to `30720`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the hidden representations.
- **num_attention_heads** (`int`, *optional*, defaults to `32`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **dim_head** (`int`, *optional*, defaults to `128`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **dim_ff** (`int`, *optional*, defaults to `10240`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `48`) --
  Number of hidden layers in the Transformer decoder.
- **dropout_p** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **position_bias_num_buckets** (`int`, *optional*, defaults to 512) --
  The number of position_bias buckets.
- **position_bias_max_distance** (`int`, *optional*, defaults to 2048) --
  The maximum sequence length that this model might ever be used with. Typically set this to something large
  just in case (e.g., 512 or 1024 or 2048).
- **eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **init_std** (`float`, *optional*, defaults to `1.0`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **prompt_types** (`int`, *optional*, defaults to 32) --
  The type of prompt.
- **prompt_length** (`int`, *optional*, defaults to 32) --
  The length of prompt.
- **segment_types** (`int`, *optional*, defaults to 32) --
  The type of segment.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a CpmAntModel. It is used to instantiate a Cpmant
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [openbmb/cpm-ant-10b](https://huggingface.co/openbmb/cpm-ant-10b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import CpmAntModel, CpmAntConfig

>>> # Initializing a CPMAnt cpm-ant-10b style configuration
>>> configuration = CpmAntConfig()

>>> # Initializing a model from the cpm-ant-10b style configuration
>>> model = CpmAntModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## CpmAntTokenizer[[transformers.CpmAntTokenizer]]

'"}, {"name": "eod_token", "val": " = ''"}, {"name": "bos_token", "val": " = ''"}, {"name": "eos_token", "val": " = ''"}, {"name": "pad_token", "val": " = ''"}, {"name": "unk_token", "val": " = ''"}, {"name": "line_token", "val": " = ''"}, {"name": "space_token", "val": " = ''"}, {"name": "padding_side", "val": " = 'left'"}, {"name": "**kwargs", "val": ""}]}>
- **vocab_file** (`str`) --
  Path to the vocabulary file.
- **bod_token** (`str`, *optional*, defaults to `"<d>"`) --
  The beginning of document token.
- **eod_token** (`str`, *optional*, defaults to `"</d>"`) --
  The end of document token.
- **bos_token** (`str`, *optional*, defaults to `"<s>"`) --
  The beginning of sequence token.
- **eos_token** (`str`, *optional*, defaults to `"</s>"`) --
  The end of sequence token.
- **pad_token** (`str`, *optional*, defaults to `"<pad>"`) --
  The token used for padding.
- **unk_token** (`str`, *optional*, defaults to `"<unk>"`) --
  The unknown token.
- **line_token** (`str`, *optional*, defaults to `"</n>"`) --
  The line token.
- **space_token** (`str`, *optional*, defaults to `"</_>"`) --
  The space token.

Construct a CPMAnt tokenizer. Based on byte-level Byte-Pair-Encoding.

## CpmAntModel[[transformers.CpmAntModel]]

- **config** ([CpmAntConfig](/docs/transformers/v5.14.0/en/model_doc/cpmant#transformers.CpmAntConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Cpmant Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.Tensor` of shape `(batch_size, seq_len)`) --
  Indices of input sequence tokens in the vocabulary.

  Indices can be obtained using `CPMAntTokenizer`. See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
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
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CpmAntConfig](/docs/transformers/v5.14.0/en/model_doc/cpmant#transformers.CpmAntConfig)) and inputs.
The [CpmAntModel](/docs/transformers/v5.14.0/en/model_doc/cpmant#transformers.CpmAntModel) forward method, overrides the `__call__` special method.

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

## CpmAntForCausalLM[[transformers.CpmAntForCausalLM]]

- **config** ([CpmAntConfig](/docs/transformers/v5.14.0/en/model_doc/cpmant#transformers.CpmAntConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The CPMAnt Model with a language modeling head on top (linear layer with weights tied to the input embeddings).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.Tensor` of shape `(batch_size, seq_len)`) --
  Indices of input sequence tokens in the vocabulary.

  Indices can be obtained using `CPMAntTokenizer`. See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
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
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **labels** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([CpmAntConfig](/docs/transformers/v5.14.0/en/model_doc/cpmant#transformers.CpmAntConfig)) and inputs.
The [CpmAntForCausalLM](/docs/transformers/v5.14.0/en/model_doc/cpmant#transformers.CpmAntForCausalLM) forward method, overrides the `__call__` special method.

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

Text Generation with CpmAntForCausalLM.

```python
>>> from transformers import CPMAntTokenizer, CpmAntForCausalLM

>>> texts = "今天天气不错，"
>>> model = CpmAntForCausalLM.from_pretrained("openbmb/cpm-ant-10b")
>>> tokenizer = CPMAntTokenizer.from_pretrained("openbmb/cpm-ant-10b")
>>> input_ids = tokenizer(texts, return_tensors="pt")
>>> outputs = model.generate(**input_ids)
>>> output_texts = tokenizer.batch_decode(outputs)
>>> print(output_texts)
['今天天气不错，阳光明媚，我和妈妈一起去超市买东西。\n在超市里，我看到了一个很好玩的玩具，它的名字叫“机器人”。它有一个圆圆的脑袋，两只圆圆的眼睛，还有一个圆圆的']
```
