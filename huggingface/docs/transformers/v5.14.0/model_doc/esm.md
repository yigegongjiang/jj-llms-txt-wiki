# ESM

## Overview

This page provides code and pre-trained weights for Transformer protein language models from Meta AI's Fundamental
AI Research Team, providing the state-of-the-art ESMFold and ESM-2, and the previously released ESM-1b and ESM-1v.
Transformer protein language models were introduced in the paper [Biological structure and function emerge from scaling
unsupervised learning to 250 million protein sequences](https://www.pnas.org/content/118/15/e2016239118) by
Alexander Rives, Joshua Meier, Tom Sercu, Siddharth Goyal, Zeming Lin, Jason Liu, Demi Guo, Myle Ott,
C. Lawrence Zitnick, Jerry Ma, and Rob Fergus.
The first version of this paper was [preprinted in 2019](https://www.biorxiv.org/content/10.1101/622803v1?versioned=true).

ESM-2 outperforms all tested single-sequence protein language models across a range of structure prediction tasks,
and enables atomic resolution structure prediction.
It was released with the paper [Language models of protein sequences at the scale of evolution enable accurate
structure prediction](https://doi.org/10.1101/2022.07.20.500902) by Zeming Lin, Halil Akin, Roshan Rao, Brian Hie,
Zhongkai Zhu, Wenting Lu, Allan dos Santos Costa, Maryam Fazel-Zarandi, Tom Sercu, Sal Candido and Alexander Rives.

Also introduced in this paper was ESMFold. It uses an ESM-2 stem with a head that can predict folded protein
structures with state-of-the-art accuracy. Unlike [AlphaFold2](https://www.nature.com/articles/s41586-021-03819-2),
it relies on the token embeddings from the large pre-trained protein language model stem and does not perform a multiple
sequence alignment (MSA) step at inference time, which means that ESMFold checkpoints are fully "standalone" -
they do not require a database of known protein sequences and structures with associated external query tools
to make predictions, and are much faster as a result.

The abstract from
"Biological structure and function emerge from scaling unsupervised learning to 250
million protein sequences" is

*In the field of artificial intelligence, a combination of scale in data and model capacity enabled by unsupervised
learning has led to major advances in representation learning and statistical generation. In the life sciences, the
anticipated growth of sequencing promises unprecedented data on natural sequence diversity. Protein language modeling
at the scale of evolution is a logical step toward predictive and generative artificial intelligence for biology. To
this end, we use unsupervised learning to train a deep contextual language model on 86 billion amino acids across 250
million protein sequences spanning evolutionary diversity. The resulting model contains information about biological
properties in its representations. The representations are learned from sequence data alone. The learned representation
space has a multiscale organization reflecting structure from the level of biochemical properties of amino acids to
remote homology of proteins. Information about secondary and tertiary structure is encoded in the representations and
can be identified by linear projections. Representation learning produces features that generalize across a range of
applications, enabling state-of-the-art supervised prediction of mutational effect and secondary structure and
improving state-of-the-art features for long-range contact prediction.*

The abstract from
"Language models of protein sequences at the scale of evolution enable accurate structure prediction" is

*Large language models have recently been shown to develop emergent capabilities with scale, going beyond
simple pattern matching to perform higher level reasoning and generate lifelike images and text. While
language models trained on protein sequences have been studied at a smaller scale, little is known about
what they learn about biology as they are scaled up. In this work we train models up to 15 billion parameters,
the largest language models of proteins to be evaluated to date. We find that as models are scaled they learn
information enabling the prediction of the three-dimensional structure of a protein at the resolution of
individual atoms. We present ESMFold for high accuracy end-to-end atomic level structure prediction directly
from the individual sequence of a protein. ESMFold has similar accuracy to AlphaFold2 and RoseTTAFold for
sequences with low perplexity that are well understood by the language model. ESMFold inference is an
order of magnitude faster than AlphaFold2, enabling exploration of the structural space of metagenomic
proteins in practical timescales.*

The original code can be found [here](https://github.com/facebookresearch/esm) and was
was developed by the Fundamental AI Research team at Meta AI.
ESM-1b, ESM-1v and ESM-2 were contributed to huggingface by [jasonliu](https://huggingface.co/jasonliu)
and [Matt](https://huggingface.co/Rocketknight1).

ESMFold was contributed to huggingface by [Matt](https://huggingface.co/Rocketknight1) and
[Sylvain](https://huggingface.co/sgugger), with a big thank you to Nikita Smetanin, Roshan Rao and Tom Sercu for their
help throughout the process!

## Usage tips

- ESM models are trained with a masked language modeling (MLM) objective.
- The HuggingFace port of ESMFold uses portions of the [openfold](https://github.com/aqlaboratory/openfold) library. The `openfold` library is licensed under the Apache License 2.0.

## Resources

- [Text classification task guide](../tasks/sequence_classification)
- [Token classification task guide](../tasks/token_classification)
- [Masked language modeling task guide](../tasks/masked_language_modeling)

## EsmConfig[[transformers.EsmConfig]]

- **vocab_size** (`int`, *optional*) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **mask_token_id** (`int`, *optional*) --
  The index of the mask token in the vocabulary. This must be included in the config because of the
  "mask-dropout" scaling trick, which will scale the inputs depending on the number of masked tokens.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **hidden_dropout_prob** (`float`, *optional*, defaults to `0.1`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`float`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **max_position_embeddings** (`int`, *optional*, defaults to `1026`) --
  The maximum sequence length that this model might ever be used with.
- **rope_theta** (`float`, defaults to 10000.0) --
  The base period of the RoPE embeddings. Only used when `position_embedding_type` is set to `"rotary"`.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-12`) --
  The epsilon used by the layer normalization layers.
- **position_embedding_type** (`str`, *optional*, defaults to `"absolute"`) --
  Type of position embedding. Choose either `"absolute"` or "rotary"`.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **emb_layer_norm_before** (`bool`, *optional*) --
  Whether to apply layer normalization after embeddings but before the main stem of the network.
- **token_dropout** (`bool`, defaults to `False`) --
  When this is enabled, masked tokens are treated as if they had been dropped out by input dropout.
- **is_folding_model** (`bool`, defaults to `False`) --
  When this is enabled, ESMFold model will be initialized.
- **esmfold_config** (`dict`, *optional*) --
  Configuration to initiate the ESMFold module.
- **vocab_list** (`list`, *optional*) --
  List of the vocabulary items.
- **is_decoder** (`bool`, *optional*, defaults to `False`) --
  Whether the model is used as a decoder or not. If `False`, the model is used as an encoder.
- **add_cross_attention** (`bool`, *optional*, defaults to `False`) --
  Whether cross-attention layers should be added to the model.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **bos_token_id** (`int`, *optional*) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.

This is the configuration class to store the configuration of a EsmModel. It is used to instantiate a Esm
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/esm-1b](https://huggingface.co/facebook/esm-1b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import EsmModel, EsmConfig

>>> # Initializing a ESM facebook/esm-1b style configuration
>>> configuration = EsmConfig(vocab_size=33)

>>> # Initializing a model from the configuration
>>> model = EsmModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## EsmTokenizer[[transformers.EsmTokenizer]]

'"}, {"name": "cls_token", "val": " = ''"}, {"name": "pad_token", "val": " = ''"}, {"name": "mask_token", "val": " = ''"}, {"name": "eos_token", "val": " = ''"}, {"name": "**kwargs", "val": ""}]}>

Constructs an ESM tokenizer.

- **token_ids_0** (`list[int]`) --
  List of ids of the first sequence.
- **token_ids_1** (`list[int]`, *optional*) --
  List of ids of the second sequence.
- **already_has_special_tokens** (`bool`, *optional*, defaults to `False`) --
  Whether or not the token list is already formatted with special tokens for the model.A list of integers in the range [0, 1]1 for a special token, 0 for a sequence token.

Retrieves sequence ids from a token list that has no special tokens added. This method is called when adding
special tokens using the tokenizer `prepare_for_model` or `encode_plus` methods.

- **token_ids_0** (`list[int]`) --
  List of IDs.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.`list[int]`Token type IDs according to the configured pattern.

Create a mask from the two sequences passed to be used in a sequence-pair classification task.

This method dynamically builds the token type IDs based on the tokenizer's configuration attributes:
- `token_type_ids_pattern`: Pattern to use ("all_zeros" or "bert_style")
- `token_type_ids_include_special_tokens`: Whether to account for special tokens in length calculation

Examples:
```python
# All zeros pattern (default, used by RoBERTa, BART, etc.)
tokenizer.token_type_ids_pattern = "all_zeros"
# Returns: [0, 0, 0, ...] for both sequences

# BERT-style pattern (first sequence gets 0s, second gets 1s)
tokenizer.token_type_ids_pattern = "bert_style"
# Returns: [0, 0, 0, ..., 1, 1, 1, ...] for sequence pairs
```

## EsmModel[[transformers.EsmModel]]

- **config** ([EsmModel](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
- **add_pooling_layer** (`bool`, *optional*, defaults to `True`) --
  Whether to add a pooling layer

The bare Esm Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `((batch_size, sequence_length))`) --
  Indices of input sequence tokens in the vocabulary.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `((batch_size, sequence_length))`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0,
  config.max_position_embeddings - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `((batch_size, sequence_length), hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention
  if the model is configured as a decoder.
- **encoder_attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on the padding token indices of the encoder input. This mask is used in
  the cross-attention if the model is configured as a decoder. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.[BaseModelOutputWithPoolingAndCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPoolingAndCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EsmConfig](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmConfig)) and inputs.
The [EsmModel](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

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
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` and `config.add_cross_attention=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.

## EsmForMaskedLM[[transformers.EsmForMaskedLM]]

- **config** ([EsmForMaskedLM](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmForMaskedLM)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Esm Model with a `language modeling` head on top."

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
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention
  if the model is configured as a decoder.
- **encoder_attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on the padding token indices of the encoder input. This mask is used in
  the cross-attention if the model is configured as a decoder. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should be in `[-100, 0, ...,
  config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the
  loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`[MaskedLMOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or `tuple(torch.FloatTensor)`A [MaskedLMOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EsmConfig](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmConfig)) and inputs.
The [EsmForMaskedLM](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmForMaskedLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Masked language modeling (MLM) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, EsmForMaskedLM
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("facebook/esm-1b")
>>> model = EsmForMaskedLM.from_pretrained("facebook/esm-1b")

>>> inputs = tokenizer("The capital of France is <mask>.", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # retrieve index of <mask>
>>> mask_token_index = (inputs.input_ids == tokenizer.mask_token_id)[0].nonzero(as_tuple=True)[0]

>>> predicted_token_id = logits[0, mask_token_index].argmax(axis=-1)
>>> tokenizer.decode(predicted_token_id)
...

>>> labels = tokenizer("The capital of France is Paris.", return_tensors="pt")["input_ids"]
>>> # mask labels of non-<mask> tokens
>>> labels = torch.where(inputs.input_ids == tokenizer.mask_token_id, labels, -100)

>>> outputs = model(**inputs, labels=labels)
>>> round(outputs.loss.item(), 2)
...
```

## EsmForSequenceClassification[[transformers.EsmForSequenceClassification]]

- **config** ([EsmForSequenceClassification](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmForSequenceClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ESM Model transformer with a sequence classification/regression head on top (a linear layer on top of the pooled
output) e.g. for GLUE tasks.

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
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the sequence classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).[SequenceClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`A [SequenceClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EsmConfig](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmConfig)) and inputs.
The [EsmForSequenceClassification](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmForSequenceClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, EsmForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("facebook/esm-1b")
>>> model = EsmForSequenceClassification.from_pretrained("facebook/esm-1b")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> model.config.id2label[predicted_class_id]
...

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = EsmForSequenceClassification.from_pretrained("facebook/esm-1b", num_labels=num_labels)

>>> labels = torch.tensor([1])
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

Example of multi-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, EsmForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("facebook/esm-1b")
>>> model = EsmForSequenceClassification.from_pretrained("facebook/esm-1b", problem_type="multi_label_classification")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_ids = torch.arange(0, logits.shape[-1])[torch.sigmoid(logits).squeeze(dim=0) > 0.5]

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = EsmForSequenceClassification.from_pretrained(
...     "facebook/esm-1b", num_labels=num_labels, problem_type="multi_label_classification"
... )

>>> labels = torch.sum(
...     torch.nn.functional.one_hot(predicted_class_ids[None, :].clone(), num_classes=num_labels), dim=1
... ).to(torch.float)
>>> loss = model(**inputs, labels=labels).loss
```

## EsmForTokenClassification[[transformers.EsmForTokenClassification]]

- **config** ([EsmForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmForTokenClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Esm transformer with a token classification head on top (a linear layer on top of the hidden-states
output) e.g. for Named-Entity-Recognition (NER) tasks.

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
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the token classification loss. Indices should be in `[0, ..., config.num_labels - 1]`.[TokenClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or `tuple(torch.FloatTensor)`A [TokenClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EsmConfig](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmConfig)) and inputs.
The [EsmForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmForTokenClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`) -- Classification scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, EsmForTokenClassification
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("facebook/esm-1b")
>>> model = EsmForTokenClassification.from_pretrained("facebook/esm-1b")

>>> inputs = tokenizer(
...     "HuggingFace is a company based in Paris and New York", add_special_tokens=False, return_tensors="pt"
... )

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_token_class_ids = logits.argmax(-1)

>>> # Note that tokens are classified rather then input words which means that
>>> # there might be more predicted token classes than words.
>>> # Multiple token classes might account for the same word
>>> predicted_tokens_classes = [model.config.id2label[t.item()] for t in predicted_token_class_ids[0]]
>>> predicted_tokens_classes
...

>>> labels = predicted_token_class_ids
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

## EsmForProteinFolding[[transformers.EsmForProteinFolding]]

- **config** ([EsmForProteinFolding](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmForProteinFolding)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ESMForProteinFolding is the HuggingFace port of the original ESMFold model. It consists of an ESM-2 "stem" followed
by a protein folding "head", although unlike most other output heads, this "head" is similar in size and runtime to
the rest of the model combined! It outputs a dictionary containing predicted structural information about the input
protein(s).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "position_ids", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "masking_pattern", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "num_recycles", "val": ": int | None = None"}, {"name": "output_hidden_states", "val": ": bool | None = False"}, {"name": "**kwargs", "val": ""}]}>
- **input_ids** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **masking_pattern** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Locations of tokens to mask during training as a form of regularization. Mask values selected in `[0, 1]`.
- **num_recycles** (`int`, *optional*, defaults to `None`) --
  Number of times to recycle the input sequence. If `None`, defaults to `config.num_recycles`. "Recycling"
  consists of passing the output of the folding trunk back in as input to the trunk. During training, the
  number of recycles should vary with each batch, to ensure that the model learns to output valid predictions
  after each recycle. During inference, num_recycles should be set to the highest value that the model was
  trained with for maximum accuracy. Accordingly, when this value is set to `None`, config.max_recycles is
  used.
- **output_hidden_states** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.`EsmForProteinFoldingOutput` or `tuple(torch.FloatTensor)`A `EsmForProteinFoldingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EsmConfig](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmConfig)) and inputs.
The [EsmForProteinFolding](/docs/transformers/v5.14.0/en/model_doc/esm#transformers.EsmForProteinFolding) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **frames** (`torch.FloatTensor`, *optional*) -- Output frames.
- **sidechain_frames** (`torch.FloatTensor`, *optional*) -- Output sidechain frames.
- **unnormalized_angles** (`torch.FloatTensor`, *optional*) -- Predicted unnormalized backbone and side chain torsion angles.
- **angles** (`torch.FloatTensor`, *optional*) -- Predicted backbone and side chain torsion angles.
- **positions** (`torch.FloatTensor`, *optional*) -- Predicted positions of the backbone and side chain atoms.
- **states** (`torch.FloatTensor`, *optional*) -- Hidden states from the protein folding trunk.
- **s_s** (`torch.FloatTensor`, *optional*) -- Per-residue embeddings derived by concatenating the hidden states of each layer of the ESM-2 LM stem.
- **s_z** (`torch.FloatTensor`, *optional*) -- Pairwise residue embeddings.
- **distogram_logits** (`torch.FloatTensor`, *optional*) -- Input logits to the distogram used to compute residue distances.
- **lm_logits** (`torch.FloatTensor`, *optional*) -- Logits output by the ESM-2 protein language model stem.
- **aatype** (`torch.FloatTensor`, *optional*) -- Input amino acids (AlphaFold2 indices).
- **atom14_atom_exists** (`torch.FloatTensor`, *optional*) -- Whether each atom exists in the atom14 representation.
- **residx_atom14_to_atom37** (`torch.FloatTensor`, *optional*) -- Mapping between atoms in the atom14 and atom37 representations.
- **residx_atom37_to_atom14** (`torch.FloatTensor`, *optional*) -- Mapping between atoms in the atom37 and atom14 representations.
- **atom37_atom_exists** (`torch.FloatTensor`, *optional*) -- Whether each atom exists in the atom37 representation.
- **residue_index** (`torch.FloatTensor`, *optional*) -- The index of each residue in the protein chain. Unless internal padding tokens are used, this will just be
  a sequence of integers from 0 to `sequence_length`.
- **lddt_head** (`torch.FloatTensor`, *optional*) -- Raw outputs from the lddt head used to compute plddt.
- **plddt** (`torch.FloatTensor`, *optional*) -- Per-residue confidence scores. Regions of low confidence may indicate areas where the model's prediction is
  uncertain, or where the protein structure is disordered.
- **ptm_logits** (`torch.FloatTensor`, *optional*) -- Raw logits used for computing ptm.
- **ptm** (`torch.FloatTensor`, *optional*) -- TM-score output representing the model's high-level confidence in the overall structure.
- **aligned_confidence_probs** (`torch.FloatTensor`, *optional*) -- Per-residue confidence scores for the aligned structure.
- **predicted_aligned_error** (`torch.FloatTensor`, *optional*) -- Predicted error between the model's prediction and the ground truth.
- **max_predicted_aligned_error** (`torch.FloatTensor`, *optional*) -- Per-sample maximum predicted error.

Example:

```python
>>> from transformers import AutoTokenizer, EsmForProteinFolding

>>> model = EsmForProteinFolding.from_pretrained("facebook/esmfold_v1")
>>> tokenizer = AutoTokenizer.from_pretrained("facebook/esmfold_v1")
>>> inputs = tokenizer(["MLKNVQVQLV"], return_tensors="pt", add_special_tokens=False)  # A tiny random peptide
>>> outputs = model(**inputs)
>>> folded_positions = outputs.positions
```
