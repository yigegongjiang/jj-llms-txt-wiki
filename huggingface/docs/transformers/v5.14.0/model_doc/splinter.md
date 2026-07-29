# Splinter

## Overview

The Splinter model was proposed in [Few-Shot Question Answering by Pretraining Span Selection](https://huggingface.co/papers/2101.00438) by Ori Ram, Yuval Kirstain, Jonathan Berant, Amir Globerson, Omer Levy. Splinter
is an encoder-only transformer (similar to BERT) pretrained using the recurring span selection task on a large corpus
comprising Wikipedia and the Toronto Book Corpus.

The abstract from the paper is the following:

In several question answering benchmarks, pretrained models have reached human parity through fine-tuning on an order
of 100,000 annotated questions and answers. We explore the more realistic few-shot setting, where only a few hundred
training examples are available, and observe that standard models perform poorly, highlighting the discrepancy between
current pretraining objectives and question answering. We propose a new pretraining scheme tailored for question
answering: recurring span selection. Given a passage with multiple sets of recurring spans, we mask in each set all
recurring spans but one, and ask the model to select the correct span in the passage for each masked span. Masked spans
are replaced with a special token, viewed as a question representation, that is later used during fine-tuning to select
the answer span. The resulting model obtains surprisingly good results on multiple benchmarks (e.g., 72.7 F1 on SQuAD
with only 128 training examples), while maintaining competitive performance in the high-resource setting.

This model was contributed by [yuvalkirstain](https://huggingface.co/yuvalkirstain) and [oriram](https://huggingface.co/oriram). The original code can be found [here](https://github.com/oriram/splinter).

## Usage tips

- Splinter was trained to predict answers spans conditioned on a special [QUESTION] token. These tokens contextualize
  to question representations which are used to predict the answers. This layer is called QASS, and is the default
  behaviour in the [SplinterForQuestionAnswering](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterForQuestionAnswering) class. Therefore:
- Use [SplinterTokenizer](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterTokenizer) (rather than [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer)), as it already
  contains this special token. Also, its default behavior is to use this token when two sequences are given (for
  example, in the *run_qa.py* script).
- If you plan on using Splinter outside *run_qa.py*, please keep in mind the question token - it might be important for
  the success of your model, especially in a few-shot setting.
- Please note there are two different checkpoints for each size of Splinter. Both are basically the same, except that
  one also has the pretrained weights of the QASS layer (*tau/splinter-base-qass* and *tau/splinter-large-qass*) and one
  doesn't (*tau/splinter-base* and *tau/splinter-large*). This is done to support randomly initializing this layer at
  fine-tuning, as it is shown to yield better results for some cases in the paper.

## Resources

- [Question answering task guide](../tasks/question_answering)

## SplinterConfig[[transformers.SplinterConfig]]

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
- **question_token_id** (`int`, *optional*, defaults to 104) --
  The id of the `[QUESTION]` token.

This is the configuration class to store the configuration of a SplinterModel. It is used to instantiate a Splinter
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [tau/splinter-base](https://huggingface.co/tau/splinter-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import SplinterModel, SplinterConfig

>>> # Initializing a Splinter tau/splinter-base style configuration
>>> configuration = SplinterConfig()

>>> # Initializing a model from the tau/splinter-base style configuration
>>> model = SplinterModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SplinterTokenizer[[transformers.SplinterTokenizer]]

- **vocab_file** (`str`, *optional*) --
  Path to a vocabulary file.
- **tokenizer_file** (`str`, *optional*) --
  Path to a tokenizers JSON file containing the serialization of a tokenizer.
- **do_lower_case** (`bool`, *optional*, defaults to `True`) --
  Whether or not to lowercase the input when tokenizing.
- **unk_token** (`str`, *optional*, defaults to `"[UNK]"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **sep_token** (`str`, *optional*, defaults to `"[SEP]"`) --
  The separator token, which is used when building a sequence from multiple sequences.
- **pad_token** (`str`, *optional*, defaults to `"[PAD]"`) --
  The token used for padding, for example when batching sequences of different lengths.
- **cls_token** (`str`, *optional*, defaults to `"[CLS]"`) --
  The classifier token which is used when doing sequence classification.
- **mask_token** (`str`, *optional*, defaults to `"[MASK]"`) --
  The token used for masking values.
- **question_token** (`str`, *optional*, defaults to `"[QUESTION]"`) --
  The token used for constructing question representations.
- **tokenize_chinese_chars** (`bool`, *optional*, defaults to `True`) --
  Whether or not to tokenize Chinese characters.
- **strip_accents** (`bool`, *optional*) --
  Whether or not to strip all accents. If this option is not specified, then it will be determined by the
  value for `lowercase`.
- **vocab** (`str`, `dict` or `list`, *optional*) --
  Custom vocabulary dictionary. If not provided, a minimal vocabulary is created.

Construct a Splinter tokenizer (backed by HuggingFace's tokenizers library). Based on WordPiece.

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

- **token_ids_0** -- List of IDs for the (possibly already formatted) sequence.
- **token_ids_1** -- Unused when `already_has_special_tokens=True`. Must be None in that case.
- **already_has_special_tokens** -- Whether the sequence is already formatted with special tokens.A list of integers in the range [0, 1]1 for a special token, 0 for a sequence token.

Retrieve sequence ids from a token list that has no special tokens added.

For fast tokenizers, data collators call this with `already_has_special_tokens=True` to build a mask over an
already-formatted sequence. In that case, we compute the mask by checking membership in `all_special_ids`.

## SplinterTokenizerFast[[transformers.SplinterTokenizer]]

- **vocab_file** (`str`, *optional*) --
  Path to a vocabulary file.
- **tokenizer_file** (`str`, *optional*) --
  Path to a tokenizers JSON file containing the serialization of a tokenizer.
- **do_lower_case** (`bool`, *optional*, defaults to `True`) --
  Whether or not to lowercase the input when tokenizing.
- **unk_token** (`str`, *optional*, defaults to `"[UNK]"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **sep_token** (`str`, *optional*, defaults to `"[SEP]"`) --
  The separator token, which is used when building a sequence from multiple sequences.
- **pad_token** (`str`, *optional*, defaults to `"[PAD]"`) --
  The token used for padding, for example when batching sequences of different lengths.
- **cls_token** (`str`, *optional*, defaults to `"[CLS]"`) --
  The classifier token which is used when doing sequence classification.
- **mask_token** (`str`, *optional*, defaults to `"[MASK]"`) --
  The token used for masking values.
- **question_token** (`str`, *optional*, defaults to `"[QUESTION]"`) --
  The token used for constructing question representations.
- **tokenize_chinese_chars** (`bool`, *optional*, defaults to `True`) --
  Whether or not to tokenize Chinese characters.
- **strip_accents** (`bool`, *optional*) --
  Whether or not to strip all accents. If this option is not specified, then it will be determined by the
  value for `lowercase`.
- **vocab** (`str`, `dict` or `list`, *optional*) --
  Custom vocabulary dictionary. If not provided, a minimal vocabulary is created.

Construct a Splinter tokenizer (backed by HuggingFace's tokenizers library). Based on WordPiece.

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

## SplinterModel[[transformers.SplinterModel]]

- **config** ([SplinterModel](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Splinter Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `batch_size, sequence_length`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0,
  1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `batch_size, sequence_length`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0,
  config.max_position_embeddings - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SplinterConfig](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterConfig)) and inputs.
The [SplinterModel](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterModel) forward method, overrides the `__call__` special method.

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

## SplinterForQuestionAnswering[[transformers.SplinterForQuestionAnswering]]

- **config** ([SplinterForQuestionAnswering](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterForQuestionAnswering)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Splinter transformer with a span classification head on top for extractive question-answering tasks like
SQuAD (a linear layer on top of the hidden-states output to compute `span start logits` and `span end logits`).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `batch_size, sequence_length`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0,
  1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `batch_size, sequence_length`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0,
  config.max_position_embeddings - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **start_positions** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for position (index) of the start of the labelled span for computing the token classification loss.
  Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence
  are not taken into account for computing the loss.
- **end_positions** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for position (index) of the end of the labelled span for computing the token classification loss.
  Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence
  are not taken into account for computing the loss.
- **question_positions** (`torch.LongTensor` of shape `(batch_size, num_questions)`, *optional*) --
  The positions of all question tokens. If given, start_logits and end_logits will be of shape `(batch_size,
  num_questions, sequence_length)`. If None, the first question token in each sequence in the batch will be
  the only one for which start_logits and end_logits are calculated and they will be of shape `(batch_size,
  sequence_length)`.</paramsdesc><rettype>[QuestionAnsweringModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or `tuple(torch.FloatTensor)`A [QuestionAnsweringModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SplinterConfig](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterConfig)) and inputs.
The [SplinterForQuestionAnswering](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterForQuestionAnswering) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Total span extraction loss is the sum of a Cross-Entropy for the start and end positions.
- **start_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) -- Span-start scores (before SoftMax).
- **end_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) -- Span-end scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, SplinterForQuestionAnswering
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("tau/splinter-base")
>>> model = SplinterForQuestionAnswering.from_pretrained("tau/splinter-base")

>>> question, text = "Who was Jim Henson?", "Jim Henson was a nice puppet"

>>> inputs = tokenizer(question, text, return_tensors="pt")
>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> answer_start_index = outputs.start_logits.argmax()
>>> answer_end_index = outputs.end_logits.argmax()

>>> predict_answer_tokens = inputs.input_ids[0, answer_start_index : answer_end_index + 1]
>>> tokenizer.decode(predict_answer_tokens, skip_special_tokens=True)
...

>>> # target is "nice puppet"
>>> target_start_index = torch.tensor([14])
>>> target_end_index = torch.tensor([15])

>>> outputs = model(**inputs, start_positions=target_start_index, end_positions=target_end_index)
>>> loss = outputs.loss
>>> round(loss.item(), 2)
...
```

## SplinterForPreTraining[[transformers.SplinterForPreTraining]]

- **config** ([SplinterForPreTraining](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterForPreTraining)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Splinter Model for the recurring span selection task as done during the pretraining. The difference to the QA task
is that we do not have a question, but multiple question tokens that replace the occurrences of recurring spans
instead.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, num_questions, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `batch_size, num_questions, sequence_length`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0,
  1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `batch_size, num_questions, sequence_length`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0,
  config.max_position_embeddings - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, num_questions, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert *input_ids* indices into associated vectors than the
  model's internal embedding lookup matrix.
- **start_positions** (`torch.LongTensor` of shape `(batch_size, num_questions)`, *optional*) --
  Labels for position (index) of the start of the labelled span for computing the token classification loss.
  Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence
  are not taken into account for computing the loss.
- **end_positions** (`torch.LongTensor` of shape `(batch_size, num_questions)`, *optional*) --
  Labels for position (index) of the end of the labelled span for computing the token classification loss.
  Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence
  are not taken into account for computing the loss.
- **question_positions** (`torch.LongTensor` of shape `(batch_size, num_questions)`, *optional*) --
  The positions of all question tokens. If given, start_logits and end_logits will be of shape `(batch_size,
  num_questions, sequence_length)`. If None, the first question token in each sequence in the batch will be
  the only one for which start_logits and end_logits are calculated and they will be of shape `(batch_size,
  sequence_length)`.</paramsdesc><rettype>`SplinterForPreTrainingOutput` or `tuple(torch.FloatTensor)`</rettype><retdesc>A `SplinterForPreTrainingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SplinterConfig](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterConfig)) and inputs.
The [SplinterForPreTraining](/docs/transformers/v5.14.0/en/model_doc/splinter#transformers.SplinterForPreTraining) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when start and end positions are provided) -- Total span extraction loss is the sum of a Cross-Entropy for the start and end positions.
- **start_logits** (`torch.FloatTensor` of shape `(batch_size, num_questions, sequence_length)`) -- Span-start scores (before SoftMax).
- **end_logits** (`torch.FloatTensor` of shape `(batch_size, num_questions, sequence_length)`) -- Span-end scores (before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
```
