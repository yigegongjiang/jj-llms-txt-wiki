# ESMC

## Overview

ESMC (ESM Cambrian) is a family of protein language models released by [BioHub](https://biohub.org/).
It is a bidirectional Transformer encoder trained with a masked-language-modelling objective over amino-acid sequences.
Like [ESM-2](./esm), ESMC produces per-residue representations that are useful for downstream protein modelling tasks.

ESMC is suitable for fine-tuning on protein classification or token classification tasks. It is also used as the
backbone of [ESMFold2](./esmfold2), where it generates representations that are used as input to the folding head.

Pre-trained checkpoints are available on the Hugging Face Hub:

- [`biohub/ESMC-300M-hf`](https://huggingface.co/biohub/ESMC-300M-hf)
- [`biohub/ESMC-600M-hf`](https://huggingface.co/biohub/ESMC-600M-hf)
- [`biohub/ESMC-6B-hf`](https://huggingface.co/biohub/ESMC-6B-hf)

## Usage example

ESMC is registered with the auto classes (`AutoModel`, `AutoModelForMaskedLM`,
`AutoModelForSequenceClassification`, `AutoModelForTokenClassification`).

```python
import torch
from transformers import pipeline

extractor = pipeline(
    task="feature-extraction",
    model="biohub/ESMC-300M-hf",
)
# Per-residue representations of shape (batch, sequence_length, hidden_size).
representations = extractor("MKTAYIAKQRQISFVKSHFSRQLEERLGLIEVQ", return_tensors="pt")
```

```python
import torch
from transformers import AutoModel, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("biohub/ESMC-300M-hf")
model = AutoModel.from_pretrained("biohub/ESMC-300M-hf")

inputs = tokenizer("MKTAYIAKQRQISFVKSHFSRQLEERLGLIEVQ", return_tensors="pt")
with torch.no_grad():
    outputs = model(**inputs)

# Per-residue representations of shape (batch, sequence_length, hidden_size).
representations = outputs.last_hidden_state
```

## EsmcConfig[[transformers.EsmcConfig]]

#### transformers.EsmcConfig[[transformers.EsmcConfig]]

```python
transformers.EsmcConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 64, hidden_size: int = 2560, intermediate_size: int = 6912, num_hidden_layers: int = 80, num_attention_heads: int = 40, num_key_value_heads: int | None = None, hidden_act: str = 'silu', max_position_embeddings: int = 2048, initializer_range: float = 0.02, pad_token_id: int | None = 1, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 2, tie_word_embeddings: bool = False, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: int | float | None = 0.0, mlp_bias: bool = False, head_dim: int | None = None, mask_token_id: int | None = 32, classifier_dropout: float | None = 0.1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmc/configuration_esmc.py#L30)

**Parameters:**

vocab_size (*int*, *optional*, defaults to *64*) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the *input_ids*.

hidden_size (*int*, *optional*, defaults to *2560*) : Dimension of the hidden representations.

intermediate_size (*int*, *optional*, defaults to *6912*) : Dimension of the MLP representations.

num_hidden_layers (*int*, *optional*, defaults to *80*) : Number of hidden layers in the Transformer decoder.

num_attention_heads (*int*, *optional*, defaults to *40*) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (*int*, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If *num_key_value_heads=num_attention_heads*, the model will use Multi Head Attention (MHA), if *num_key_value_heads=1* the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to *num_attention_heads*.

hidden_act (*str*, *optional*, defaults to *silu*) : The non-linear activation function (function or string) in the decoder. For example, *"gelu"*, *"relu"*, *"silu"*, etc.

max_position_embeddings (*int*, *optional*, defaults to *2048*) : The maximum sequence length that this model might ever be used with.

initializer_range (*float*, *optional*, defaults to *0.02*) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

pad_token_id (*int*, *optional*, defaults to *1*) : Token id used for padding in the vocabulary.

bos_token_id (*int*, *optional*, defaults to *0*) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (*Union[int, list[int]]*, *optional*, defaults to *2*) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (*bool*, *optional*, defaults to *False*) : Whether to tie weight embeddings according to model's *tied_weights_keys* mapping.

rope_parameters (*Union[~modeling_rope_utils.RopeParameters, dict]*, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for *rope_theta* and optionally parameters used for scaling in case you want to use RoPE with longer *max_position_embeddings*.

attention_bias (*bool*, *optional*, defaults to *False*) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (*Union[int, float]*, *optional*, defaults to *0.0*) : The dropout ratio for the attention probabilities.

mlp_bias (*bool*, *optional*, defaults to *False*) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

head_dim (*int*, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

mask_token_id (*int*, *optional*, defaults to 32) : Index of the mask token in the vocabulary (`"&amp;lt;mask>"`), used for masked language modelling.

classifier_dropout (*float*, *optional*, defaults to 0.1) : Dropout ratio for the classification head.

This is the configuration class to store the configuration of a EsmcModel. It is used to instantiate a Esmc
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [biohub/ESMC-6B-hf](https://huggingface.co/biohub/ESMC-6B-hf)

Configuration objects inherit from [*PreTrainedConfig*] and can be used to control the model outputs. Read the
documentation from [*PreTrainedConfig*] for more information.

Examples:

```python
>>> from transformers import EsmcConfig, EsmcModel

>>> # Initializing an ESMC biohub/ESMC-6B-hf style configuration
>>> configuration = EsmcConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = EsmcModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## EsmcTokenizer[[transformers.EsmcTokenizer]]

#### transformers.EsmcTokenizer[[transformers.EsmcTokenizer]]

```python
transformers.EsmcTokenizer(unk_token = '<unk>', cls_token = '<cls>', pad_token = '<pad>', mask_token = '<mask>', eos_token = '<eos>', bos_token = None, chain_break_token = '|', extra_special_tokens = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmc/tokenization_esmc.py#L67)

**Parameters:**

unk_token (*str*, *optional*, defaults to *"&amp;lt;unk>"*) : The unknown token.

cls_token (*str*, *optional*, defaults to *"&amp;lt;cls>"*) : The classification token (prepended to every sequence).

pad_token (*str*, *optional*, defaults to *"&amp;lt;pad>"*) : The padding token.

mask_token (*str*, *optional*, defaults to *"&amp;lt;mask>"*) : The mask token, used for masked language modelling.

eos_token (*str*, *optional*, defaults to *"&amp;lt;eos>"*) : The end-of-sequence token (appended to every sequence).

bos_token (*str*, *optional*, defaults to *"&amp;lt;cls>"*) : The beginning-of-sequence token (prepended to every sequence). When unset, uses cls_token.

chain_break_token (*str*, *optional*, defaults to *"|"*) : Token inserted between chains in multi-chain protein inputs. Registered as a model-specific special token, so it is exposed as *chain_break_token* / *chain_break_token_id*.

extra_special_tokens (*list[str]*, *optional*) : Additional special tokens to register with the tokenizer (round-tripped from a saved config).

Construct an ESMC tokenizer.

This tokenizer is a character-level tokenizer backed by the HuggingFace *tokenizers* library.
It wraps every sequence with `&amp;lt;cls>` and `&amp;lt;eos>` tokens and supports a `|` chain-break
token for multi-chain inputs.

Examples:

```python
>>> from transformers import EsmcTokenizer

>>> tokenizer = EsmcTokenizer()
>>> tokenizer("ACDEFGHIKLMNPQRSTVWY")["input_ids"]
[0, 5, 23, 13, 9, 18, 6, 21, 12, 15, 4, 20, 17, 14, 16, 10, 8, 11, 7, 22, 19, 2]
```

## EsmcModel[[transformers.EsmcModel]]

#### transformers.EsmcModel[[transformers.EsmcModel]]

```python
transformers.EsmcModel(config: EsmcConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmc/modeling_esmc.py#L317)

**Parameters:**

config ([EsmcConfig](/docs/transformers/v5.17.0/en/model_doc/esmc#transformers.EsmcConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Esmc Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.EsmcModel.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, sequence_id: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmc/modeling_esmc.py#L327)

**Parameters:**

input_ids (*torch.Tensor* of shape *(batch_size, sequence_length)*, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [*AutoTokenizer*]. See [*PreTrainedTokenizer.encode*] and [*PreTrainedTokenizer.__call__*] for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (*torch.Tensor* of shape *(batch_size, sequence_length)*, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in *[0, 1]*:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (*torch.Tensor* of shape *(batch_size, sequence_length)*, *optional*) : Position of each token in the sequence, used by RoPE. Defaults to `arange(sequence_length)`.  ESMC positions are absolute over the whole input, and keep running *across* chain boundaries in a multi-chain input. The chain structure is carried separately by `sequence_id`, which masks attention: unlike the packed-sequence format used elsewhere in the library, these are therefore not per-chain position ids restarting at 0, and the chain structure is deliberately not inferred from them.

sequence_id (*torch.Tensor* of shape *(batch_size, sequence_length)*, *optional*) : Integer chain-ID tensor for chain-aware attention masking. Tokens with the same non-negative integer value can attend to each other; tokens with different values cannot (cross-chain masking). Padding positions should be set to `-1` and inputs must be **right-padded** (RoPE uses absolute positions starting at 0). When provided, `attention_mask` is ignored. Passing `sequence_id` builds a custom attention mask, which requires `torch>=2.6`. Multi-chain inputs additionally require a non-flash `attn_implementation` (`'sdpa'` / `'eager'` / `'flex_attention'`); flash attention only supports the single-chain case.

**Returns:** [*~modeling_outputs.BaseModelOutput*] or *tuple(torch.FloatTensor)*

A [*~modeling_outputs.BaseModelOutput*] or a tuple of
*torch.FloatTensor* (if *return_dict=False* is passed or when *config.return_dict=False*) comprising various
elements depending on the configuration ([*EsmcConfig*]) and inputs.

The [*EsmcModel*] forward method, overrides the *__call__* special method.

Although the recipe for forward pass needs to be defined within this function, one should call the [*Module*]
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (*torch.FloatTensor* of shape *(batch_size, sequence_length, hidden_size)*) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (*tuple(torch.FloatTensor)*, *optional*, returned when *output_hidden_states=True* is passed or when *config.output_hidden_states=True*) -- Tuple of *torch.FloatTensor* (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape *(batch_size, sequence_length, hidden_size)*.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (*tuple(torch.FloatTensor)*, *optional*, returned when *output_attentions=True* is passed or when *config.output_attentions=True*) -- Tuple of *torch.FloatTensor* (one for each layer) of shape *(batch_size, num_heads, sequence_length,
  sequence_length)*.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoTokenizer, EsmcModel

>>> model = EsmcModel.from_pretrained("biohub/ESMC-300M-hf")
>>> tokenizer = AutoTokenizer.from_pretrained("biohub/ESMC-300M-hf")
>>> inputs = tokenizer(["MLKNVQVQLV"], return_tensors="pt")
>>> outputs = model(**inputs)
>>> outputs.last_hidden_state.shape
torch.Size([1, 12, 960])
```

## EsmcForMaskedLM[[transformers.EsmcForMaskedLM]]

#### transformers.EsmcForMaskedLM[[transformers.EsmcForMaskedLM]]

```python
transformers.EsmcForMaskedLM(config: EsmcConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmc/modeling_esmc.py#L425)

**Parameters:**

config ([EsmcConfig](/docs/transformers/v5.17.0/en/model_doc/esmc#transformers.EsmcConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Esmc Model with a `language modeling` head on top."

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.EsmcForMaskedLM.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, sequence_id: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmc/modeling_esmc.py#L441)

**Parameters:**

input_ids (*torch.Tensor* of shape *(batch_size, sequence_length)*, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [*AutoTokenizer*]. See [*PreTrainedTokenizer.encode*] and [*PreTrainedTokenizer.__call__*] for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (*torch.Tensor* of shape *(batch_size, sequence_length)*, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in *[0, 1]*:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (*torch.Tensor* of shape *(batch_size, sequence_length)*, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range *[0, config.n_positions - 1]*.  [What are position IDs?](../glossary#position-ids)

sequence_id (*torch.Tensor* of shape *(batch_size, sequence_length)*, *optional*) : Integer chain-ID tensor forwarded to the encoder for chain-aware attention masking. See [EsmcModel.forward()](/docs/transformers/v5.17.0/en/model_doc/esmc#transformers.EsmcModel.forward) for the encoding.

labels (*torch.LongTensor* of shape *(batch_size, sequence_length)*, *optional*) : Labels for masked language modelling loss.  Positions with label `-100` are ignored.  Other positions must be in `[0, config.vocab_size)`.

**Returns:** [*~modeling_outputs.MaskedLMOutput*] or *tuple(torch.FloatTensor)*

A [*~modeling_outputs.MaskedLMOutput*] or a tuple of
*torch.FloatTensor* (if *return_dict=False* is passed or when *config.return_dict=False*) comprising various
elements depending on the configuration ([*EsmcConfig*]) and inputs.

The [*EsmcForMaskedLM*] forward method, overrides the *__call__* special method.

Although the recipe for forward pass needs to be defined within this function, one should call the [*Module*]
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (*torch.FloatTensor* of shape *(1,)*, *optional*, returned when *labels* is provided) -- Masked language modeling (MLM) loss.
- **logits** (*torch.FloatTensor* of shape *(batch_size, sequence_length, config.vocab_size)*) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (*tuple(torch.FloatTensor)*, *optional*, returned when *output_hidden_states=True* is passed or when *config.output_hidden_states=True*) -- Tuple of *torch.FloatTensor* (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape *(batch_size, sequence_length, hidden_size)*.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (*tuple(torch.FloatTensor)*, *optional*, returned when *output_attentions=True* is passed or when *config.output_attentions=True*) -- Tuple of *torch.FloatTensor* (one for each layer) of shape *(batch_size, num_heads, sequence_length,
  sequence_length)*.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoTokenizer, EsmcForMaskedLM
>>> import torch

>>> model = EsmcForMaskedLM.from_pretrained("biohub/ESMC-300M-hf")
>>> tokenizer = AutoTokenizer.from_pretrained("biohub/ESMC-300M-hf")
>>> inputs = tokenizer(["MLKNVQ&amp;lt;mask>LV"], return_tensors="pt")
>>> outputs = model(**inputs)
>>> outputs.logits.shape
torch.Size([1, 11, 64])
```

## EsmcForSequenceClassification[[transformers.EsmcForSequenceClassification]]

#### transformers.EsmcForSequenceClassification[[transformers.EsmcForSequenceClassification]]

```python
transformers.EsmcForSequenceClassification(config: EsmcConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmc/modeling_esmc.py#L522)

**Parameters:**

config ([EsmcConfig](/docs/transformers/v5.17.0/en/model_doc/esmc#transformers.EsmcConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ESMC Model transformer with a sequence classification/regression head on top (a linear layer on top of the pooled
output) e.g. for GLUE tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.EsmcForSequenceClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmc/modeling_esmc.py#L532)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

**Returns:** [SequenceClassifierOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`

A [SequenceClassifierOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EsmcConfig](/docs/transformers/v5.17.0/en/model_doc/esmc#transformers.EsmcConfig)) and inputs.

The [EsmcForSequenceClassification](/docs/transformers/v5.17.0/en/model_doc/esmc#transformers.EsmcForSequenceClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example of single-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, EsmcForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("biohub/ESMC-6B-hf")
>>> model = EsmcForSequenceClassification.from_pretrained("biohub/ESMC-6B-hf")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> model.config.id2label[predicted_class_id]
...

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = EsmcForSequenceClassification.from_pretrained("biohub/ESMC-6B-hf", num_labels=num_labels)

>>> labels = torch.tensor([1])
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

Example of multi-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, EsmcForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("biohub/ESMC-6B-hf")
>>> model = EsmcForSequenceClassification.from_pretrained("biohub/ESMC-6B-hf", problem_type="multi_label_classification")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_ids = torch.arange(0, logits.shape[-1])[torch.sigmoid(logits).squeeze(dim=0) > 0.5]

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = EsmcForSequenceClassification.from_pretrained(
...     "biohub/ESMC-6B-hf", num_labels=num_labels, problem_type="multi_label_classification"
... )

>>> labels = torch.sum(
...     torch.nn.functional.one_hot(predicted_class_ids[None, :].clone(), num_classes=num_labels), dim=1
... ).to(torch.float)
>>> loss = model(**inputs, labels=labels).loss
```

## EsmcForTokenClassification[[transformers.EsmcForTokenClassification]]

#### transformers.EsmcForTokenClassification[[transformers.EsmcForTokenClassification]]

```python
transformers.EsmcForTokenClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/esmc/modeling_esmc.py#L593)

#### forward[[transformers.EsmcForTokenClassification.forward]]

```python
forward(input_ids: torch.LongTensor | None = None, attention_mask: torch.Tensor | None = None, position_ids: torch.LongTensor | None = None, past_key_values: Cache | None = None, inputs_embeds: torch.FloatTensor | None = None, labels: torch.LongTensor | None = None, use_cache: bool | None = None, **kwargs: Unpack[TransformersKwargs])
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/modeling_layers.py#L273)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.17.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `TokenClassifierOutput`

The `GenericForTokenClassification` forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.
