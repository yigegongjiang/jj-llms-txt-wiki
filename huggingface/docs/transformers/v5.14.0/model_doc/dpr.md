# DPR

## Overview

Dense Passage Retrieval (DPR) is a set of tools and models for state-of-the-art open-domain Q&A research. It was
introduced in [Dense Passage Retrieval for Open-Domain Question Answering](https://huggingface.co/papers/2004.04906) by
Vladimir Karpukhin, Barlas Oğuz, Sewon Min, Patrick Lewis, Ledell Wu, Sergey Edunov, Danqi Chen, Wen-tau Yih.

The abstract from the paper is the following:

*Open-domain question answering relies on efficient passage retrieval to select candidate contexts, where traditional
sparse vector space models, such as TF-IDF or BM25, are the de facto method. In this work, we show that retrieval can
be practically implemented using dense representations alone, where embeddings are learned from a small number of
questions and passages by a simple dual-encoder framework. When evaluated on a wide range of open-domain QA datasets,
our dense retriever outperforms a strong Lucene-BM25 system largely by 9%-19% absolute in terms of top-20 passage
retrieval accuracy, and helps our end-to-end QA system establish new state-of-the-art on multiple open-domain QA
benchmarks.*

This model was contributed by [lhoestq](https://huggingface.co/lhoestq). The original code can be found [here](https://github.com/facebookresearch/DPR).

## Usage tips

- DPR consists in three models:

  * Question encoder: encode questions as vectors
  * Context encoder: encode contexts as vectors
  * Reader: extract the answer of the questions inside retrieved contexts, along with a relevance score (high if the inferred span actually answers the question).

## DPRConfig[[transformers.DPRConfig]]

- **vocab_size** (`int`, *optional*, defaults to `30522`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **max_position_embeddings** (`int`, *optional*, defaults to `512`) --
  The maximum sequence length that this model might ever be used with.
- **type_vocab_size** (`int`, *optional*, defaults to `2`) --
  The vocabulary size of the `token_type_ids`.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-12`) --
  The epsilon used by the layer normalization layers.
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*) --
  Token id used for end-of-stream in the vocabulary.
- **projection_dim** (`int`, *optional*, defaults to `0`) --
  Dimensionality of text and vision projection layers.
- **is_decoder** (`bool`, *optional*, defaults to `False`) --
  Whether the model is used as a decoder or not. If `False`, the model is used as an encoder.
- **add_cross_attention** (`bool`, *optional*, defaults to `False`) --
  Whether cross-attention layers should be added to the model.

This is the configuration class to store the configuration of a DPRQuestionEncoder. It is used to instantiate a Dpr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/dpr-ctx_encoder-single-nq-base](https://huggingface.co/facebook/dpr-ctx_encoder-single-nq-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import DPRConfig, DPRContextEncoder

>>> # Initializing a DPR facebook/dpr-ctx_encoder-single-nq-base style configuration
>>> configuration = DPRConfig()

>>> # Initializing a model (with random weights) from the facebook/dpr-ctx_encoder-single-nq-base style configuration
>>> model = DPRContextEncoder(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DPRContextEncoderTokenizer[[transformers.DPRContextEncoderTokenizer]]

Construct a DPRContextEncoder tokenizer.

[DPRContextEncoderTokenizer](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRContextEncoderTokenizer) is identical to [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) and runs end-to-end tokenization: punctuation
splitting and wordpiece.

Refer to superclass [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) for usage examples and documentation concerning parameters.

## DPRContextEncoderTokenizerFast[[transformers.DPRContextEncoderTokenizerFast]]

Construct a "fast" DPRContextEncoder tokenizer (backed by HuggingFace's *tokenizers* library).

[DPRContextEncoderTokenizerFast](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRContextEncoderTokenizerFast) is identical to [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) and runs end-to-end tokenization:
punctuation splitting and wordpiece.

Refer to superclass [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) for usage examples and documentation concerning parameters.

## DPRQuestionEncoderTokenizer[[transformers.DPRQuestionEncoderTokenizer]]

Constructs a DPRQuestionEncoder tokenizer.

[DPRQuestionEncoderTokenizer](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRQuestionEncoderTokenizer) is identical to [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) and runs end-to-end tokenization: punctuation
splitting and wordpiece.

Refer to superclass [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) for usage examples and documentation concerning parameters.

## DPRQuestionEncoderTokenizerFast[[transformers.DPRQuestionEncoderTokenizerFast]]

Constructs a "fast" DPRQuestionEncoder tokenizer (backed by HuggingFace's *tokenizers* library).

[DPRQuestionEncoderTokenizerFast](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRQuestionEncoderTokenizerFast) is identical to [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) and runs end-to-end tokenization:
punctuation splitting and wordpiece.

Refer to superclass [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) for usage examples and documentation concerning parameters.

## DPRReaderTokenizer[[transformers.DPRReaderTokenizer]]

- **questions** (`str` or `list[str]`) --
  The questions to be encoded. You can specify one question for many passages. In this case, the question
  will be duplicated like `[questions] * n_passages`. Otherwise you have to specify as many questions as in
  `titles` or `texts`.
- **titles** (`str` or `list[str]`) --
  The passages titles to be encoded. This can be a string or a list of strings if there are several passages.
- **texts** (`str` or `list[str]`) --
  The passages texts to be encoded. This can be a string or a list of strings if there are several passages.
- **padding** (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) --
  Activates and controls padding. Accepts the following values:

  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence
    if provided).
  - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided.
  - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different
    lengths).
- **truncation** (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) --
  Activates and controls truncation. Accepts the following values:

  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or to
    the maximum acceptable input length for the model if that argument is not provided. This will truncate
    token by token, removing a token from the longest sequence in the pair if a pair of sequences (or a batch
    of pairs) is provided.
  - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided. This will only truncate the first
    sequence of a pair if a pair of sequences (or a batch of pairs) is provided.
  - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided. This will only truncate the
    second sequence of a pair if a pair of sequences (or a batch of pairs) is provided.
  - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths
    greater than the model maximum admissible input size).
- **max_length** (`int`, *optional*) --
  Controls the maximum length to use by one of the truncation/padding parameters.

  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length
  is required by one of the truncation/padding parameters. If the model has no specific maximum input
  length (like XLNet) truncation/padding to a maximum length will be deactivated.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors instead of list of python integers. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return Numpy `np.ndarray` objects.
- **return_attention_mask** (`bool`, *optional*) --
  Whether or not to return the attention mask. If not set, will return the attention mask according to the
  specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are attention masks?](../glossary#attention-mask)`dict[str, list[list[int]]]`A dictionary with the following keys:

- `input_ids`: List of token ids to be fed to a model.
- `attention_mask`: List of indices specifying which tokens should be attended to by the model.

Construct a DPRReader tokenizer.

[DPRReaderTokenizer](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRReaderTokenizer) is almost identical to [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) and runs end-to-end tokenization: punctuation
splitting and wordpiece. The difference is that is has three inputs strings: question, titles and texts that are
combined to be fed to the [DPRReader](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRReader) model.

Refer to superclass [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) for usage examples and documentation concerning parameters.

Return a dictionary with the token ids of the input strings and other information to give to `.decode_best_spans`.
It converts the strings of a question and different passages (title and text) in a sequence of IDs (integers),
using the tokenizer and vocabulary. The resulting `input_ids` is a matrix of size `(n_passages, sequence_length)`

with the format:

```
[CLS] <question token ids> [SEP] <titles ids> [SEP] <texts ids>
```

## DPRReaderTokenizerFast[[transformers.DPRReaderTokenizerFast]]

- **questions** (`str` or `list[str]`) --
  The questions to be encoded. You can specify one question for many passages. In this case, the question
  will be duplicated like `[questions] * n_passages`. Otherwise you have to specify as many questions as in
  `titles` or `texts`.
- **titles** (`str` or `list[str]`) --
  The passages titles to be encoded. This can be a string or a list of strings if there are several passages.
- **texts** (`str` or `list[str]`) --
  The passages texts to be encoded. This can be a string or a list of strings if there are several passages.
- **padding** (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) --
  Activates and controls padding. Accepts the following values:

  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence
    if provided).
  - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided.
  - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different
    lengths).
- **truncation** (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) --
  Activates and controls truncation. Accepts the following values:

  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or to
    the maximum acceptable input length for the model if that argument is not provided. This will truncate
    token by token, removing a token from the longest sequence in the pair if a pair of sequences (or a batch
    of pairs) is provided.
  - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided. This will only truncate the first
    sequence of a pair if a pair of sequences (or a batch of pairs) is provided.
  - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided. This will only truncate the
    second sequence of a pair if a pair of sequences (or a batch of pairs) is provided.
  - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths
    greater than the model maximum admissible input size).
- **max_length** (`int`, *optional*) --
  Controls the maximum length to use by one of the truncation/padding parameters.

  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length
  is required by one of the truncation/padding parameters. If the model has no specific maximum input
  length (like XLNet) truncation/padding to a maximum length will be deactivated.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors instead of list of python integers. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return Numpy `np.ndarray` objects.
- **return_attention_mask** (`bool`, *optional*) --
  Whether or not to return the attention mask. If not set, will return the attention mask according to the
  specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are attention masks?](../glossary#attention-mask)`dict[str, list[list[int]]]`A dictionary with the following keys:

- `input_ids`: List of token ids to be fed to a model.
- `attention_mask`: List of indices specifying which tokens should be attended to by the model.

Constructs a "fast" DPRReader tokenizer (backed by HuggingFace's *tokenizers* library).

[DPRReaderTokenizerFast](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRReaderTokenizerFast) is almost identical to [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) and runs end-to-end tokenization:
punctuation splitting and wordpiece. The difference is that is has three inputs strings: question, titles and texts
that are combined to be fed to the [DPRReader](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRReader) model.

Refer to superclass [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) for usage examples and documentation concerning parameters.

Return a dictionary with the token ids of the input strings and other information to give to `.decode_best_spans`.
It converts the strings of a question and different passages (title and text) in a sequence of IDs (integers),
using the tokenizer and vocabulary. The resulting `input_ids` is a matrix of size `(n_passages, sequence_length)`
with the format:

[CLS]  [SEP]  [SEP] 

## DPR specific outputs[[transformers.models.dpr.modeling_dpr.DPRContextEncoderOutput]]

- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, embeddings_size)`) --
  The DPR encoder outputs the *pooler_output* that corresponds to the context representation. Last layer
  hidden-state of the first token of the sequence (classification token) further processed by a Linear layer.
  This output is to be used to embed contexts for nearest neighbors queries with questions embeddings.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Class for outputs of [DPRQuestionEncoder](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRQuestionEncoder).

- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, embeddings_size)`) --
  The DPR encoder outputs the *pooler_output* that corresponds to the question representation. Last layer
  hidden-state of the first token of the sequence (classification token) further processed by a Linear layer.
  This output is to be used to embed questions for nearest neighbors queries with context embeddings.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Class for outputs of [DPRQuestionEncoder](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRQuestionEncoder).

- **start_logits** (`torch.FloatTensor` of shape `(n_passages, sequence_length)`) --
  Logits of the start index of the span for each passage.
- **end_logits** (`torch.FloatTensor` of shape `(n_passages, sequence_length)`) --
  Logits of the end index of the span for each passage.
- **relevance_logits** (`torch.FloatTensor` of shape `(n_passages, )`) --
  Outputs of the QA classifier of the DPRReader that corresponds to the scores of each passage to answer the
  question, compared to all the other passages.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) --
  Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) --
  Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Class for outputs of [DPRQuestionEncoder](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRQuestionEncoder).

## DPRContextEncoder[[transformers.DPRContextEncoder]]

- **config** ([DPRConfig](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare DPRContextEncoder transformer outputting pooler outputs as context representations.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. To match pretraining, DPR input sequence should be
  formatted with [CLS] and [SEP] tokens as follows:

  (a) For sequence pairs (for a pair title+text for example):
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[DPRContextEncoderOutput](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.models.dpr.modeling_dpr.DPRContextEncoderOutput) or `tuple(torch.FloatTensor)`A [DPRContextEncoderOutput](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.models.dpr.modeling_dpr.DPRContextEncoderOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DPRConfig](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRConfig)) and inputs.
The [DPRContextEncoder](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRContextEncoder) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, embeddings_size)`) -- The DPR encoder outputs the *pooler_output* that corresponds to the context representation. Last layer
  hidden-state of the first token of the sequence (classification token) further processed by a Linear layer.
  This output is to be used to embed contexts for nearest neighbors queries with questions embeddings.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

```
tokens:         [CLS] is this jack ##son ##ville ? [SEP] no it is not . [SEP]
token_type_ids:   0   0  0    0    0     0       0   0   1  1  1  1   1   1
```

  (b) For single sequences (for a question for example):

```
tokens:         [CLS] the dog is hairy . [SEP]
token_type_ids:   0   0   0   0  0     0   0
```

  DPR is a model with absolute position embeddings so it's usually advised to pad the inputs on the right
  rather than the left.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)

Examples:

```python
>>> from transformers import DPRContextEncoder, DPRContextEncoderTokenizer

>>> tokenizer = DPRContextEncoderTokenizer.from_pretrained("facebook/dpr-ctx_encoder-single-nq-base")
>>> model = DPRContextEncoder.from_pretrained("facebook/dpr-ctx_encoder-single-nq-base")
>>> input_ids = tokenizer("Hello, is my dog cute ?", return_tensors="pt")["input_ids"]
>>> embeddings = model(input_ids).pooler_output
```

## DPRQuestionEncoder[[transformers.DPRQuestionEncoder]]

- **config** ([DPRConfig](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare DPRQuestionEncoder transformer outputting pooler outputs as question representations.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. To match pretraining, DPR input sequence should be
  formatted with [CLS] and [SEP] tokens as follows:

  (a) For sequence pairs (for a pair title+text for example):
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[DPRQuestionEncoderOutput](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.models.dpr.modeling_dpr.DPRQuestionEncoderOutput) or `tuple(torch.FloatTensor)`A [DPRQuestionEncoderOutput](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.models.dpr.modeling_dpr.DPRQuestionEncoderOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DPRConfig](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRConfig)) and inputs.
The [DPRQuestionEncoder](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRQuestionEncoder) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, embeddings_size)`) -- The DPR encoder outputs the *pooler_output* that corresponds to the question representation. Last layer
  hidden-state of the first token of the sequence (classification token) further processed by a Linear layer.
  This output is to be used to embed questions for nearest neighbors queries with context embeddings.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

```
tokens:         [CLS] is this jack ##son ##ville ? [SEP] no it is not . [SEP]
token_type_ids:   0   0  0    0    0     0       0   0   1  1  1  1   1   1
```

  (b) For single sequences (for a question for example):

```
tokens:         [CLS] the dog is hairy . [SEP]
token_type_ids:   0   0   0   0  0     0   0
```

  DPR is a model with absolute position embeddings so it's usually advised to pad the inputs on the right
  rather than the left.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)

Examples:

```python
>>> from transformers import DPRQuestionEncoder, DPRQuestionEncoderTokenizer

>>> tokenizer = DPRQuestionEncoderTokenizer.from_pretrained("facebook/dpr-question_encoder-single-nq-base")
>>> model = DPRQuestionEncoder.from_pretrained("facebook/dpr-question_encoder-single-nq-base")
>>> input_ids = tokenizer("Hello, is my dog cute ?", return_tensors="pt")["input_ids"]
>>> embeddings = model(input_ids).pooler_output
```

## DPRReader[[transformers.DPRReader]]

- **config** ([DPRConfig](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare DPRReader transformer outputting span predictions.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`tuple[torch.LongTensor]` of shapes `(n_passages, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. It has to be a sequence triplet with 1) the question
  and 2) the passages titles and 3) the passages texts To match pretraining, DPR `input_ids` sequence should
  be formatted with [CLS] and [SEP] with the format:

  `[CLS] <question token ids> [SEP] <titles ids> [SEP] <texts ids>`

  DPR is a model with absolute position embeddings so it's usually advised to pad the inputs on the right
  rather than the left.

  Indices can be obtained using [DPRReaderTokenizer](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRReaderTokenizer). See this class documentation for more details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **inputs_embeds** (`torch.FloatTensor` of shape `(n_passages, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[DPRReaderOutput](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRReaderOutput) or `tuple(torch.FloatTensor)`A [DPRReaderOutput](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRReaderOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DPRConfig](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRConfig)) and inputs.
The [DPRReader](/docs/transformers/v5.14.0/en/model_doc/dpr#transformers.DPRReader) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **start_logits** (`torch.FloatTensor` of shape `(n_passages, sequence_length)`) -- Logits of the start index of the span for each passage.
- **end_logits** (`torch.FloatTensor` of shape `(n_passages, sequence_length)`) -- Logits of the end index of the span for each passage.
- **relevance_logits** (`torch.FloatTensor` of shape `(n_passages, )`) -- Outputs of the QA classifier of the DPRReader that corresponds to the scores of each passage to answer the
  question, compared to all the other passages.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import DPRReader, DPRReaderTokenizer

>>> tokenizer = DPRReaderTokenizer.from_pretrained("facebook/dpr-reader-single-nq-base")
>>> model = DPRReader.from_pretrained("facebook/dpr-reader-single-nq-base")
>>> encoded_inputs = tokenizer(
...     questions=["What is love ?"],
...     titles=["Haddaway"],
...     texts=["'What Is Love' is a song recorded by the artist Haddaway"],
...     return_tensors="pt",
... )
>>> outputs = model(**encoded_inputs)
>>> start_logits = outputs.start_logits
>>> end_logits = outputs.end_logits
>>> relevance_logits = outputs.relevance_logits
```
