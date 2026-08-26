# MobileBERT

[MobileBERT](https://huggingface.co/papers/2004.02984) is a lightweight and efficient variant of BERT, specifically designed for resource-limited devices such as mobile phones. It retains BERT's architecture but significantly reduces model size and inference latency while maintaining strong performance on NLP tasks. MobileBERT achieves this through a bottleneck structure and carefully balanced self-attention and feedforward networks. The model is trained by knowledge transfer from a large BERT model with an inverted bottleneck structure.

You can find the original MobileBERT checkpoint under the [Google](https://huggingface.co/google/mobilebert-uncased) organization.
> [!TIP]
> Click on the MobileBERT models in the right sidebar for more examples of how to apply MobileBERT to different language tasks.

The example below demonstrates how to predict the `[MASK]` token with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline), [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel), and from the command line.

```python
from transformers import pipeline

pipeline = pipeline(
    task="fill-mask",
    model="google/mobilebert-uncased",
    device=0
)
pipeline("The capital of France is [MASK].")
```

```python
import torch

from transformers import AutoModelForMaskedLM, AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained(
    "google/mobilebert-uncased",
)
model = AutoModelForMaskedLM.from_pretrained(
    "google/mobilebert-uncased",
    device_map="auto",
)
inputs = tokenizer("The capital of France is [MASK].", return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)
    predictions = outputs.logits

masked_index = torch.where(inputs['input_ids'] == tokenizer.mask_token_id)[1]
predicted_token_id = predictions[0, masked_index].argmax(dim=-1)
predicted_token = tokenizer.decode(predicted_token_id)

print(f"The predicted token is: {predicted_token}")
```

## Notes

- Inputs should be padded on the right because BERT uses absolute position embeddings.

## MobileBertConfig[[transformers.MobileBertConfig]]

#### transformers.MobileBertConfig[[transformers.MobileBertConfig]]

```python
transformers.MobileBertConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 30522, hidden_size: int = 512, num_hidden_layers: int = 24, num_attention_heads: int = 4, intermediate_size: int = 512, hidden_act: str = 'relu', hidden_dropout_prob: float | int = 0.0, attention_probs_dropout_prob: float | int = 0.1, max_position_embeddings: int = 512, type_vocab_size: int = 2, initializer_range: float = 0.02, layer_norm_eps: float = 1e-12, pad_token_id: int | None = 0, embedding_size: int = 128, trigram_input: bool = True, use_bottleneck: bool = True, intra_bottleneck_size: int = 128, use_bottleneck_attention: bool = False, key_query_shared_bottleneck: bool = True, num_feedforward_networks: int = 4, normalization_type: str = 'no_norm', classifier_activation: bool = True, classifier_dropout: float | int | None = None, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/configuration_mobilebert.py#L24)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `30522`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `512`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `24`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `4`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `512`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

max_position_embeddings (`int`, *optional*, defaults to `512`) : The maximum sequence length that this model might ever be used with.

type_vocab_size (`int`, *optional*, defaults to `2`) : The vocabulary size of the `token_type_ids`.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-12`) : The epsilon used by the layer normalization layers.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

embedding_size (`int`, *optional*, defaults to 128) : The dimension of the word embedding vectors.

trigram_input (`bool`, *optional*, defaults to `True`) : Use a convolution of trigram as input.

use_bottleneck (`bool`, *optional*, defaults to `True`) : Whether to use bottleneck in BERT.

intra_bottleneck_size (`int`, *optional*, defaults to 128) : Size of bottleneck layer output.

use_bottleneck_attention (`bool`, *optional*, defaults to `False`) : Whether to use attention inputs from the bottleneck transformation.

key_query_shared_bottleneck (`bool`, *optional*, defaults to `True`) : Whether to use the same linear transformation for query&key in the bottleneck.

num_feedforward_networks (`int`, *optional*, defaults to 4) : Number of FFNs in a block.

normalization_type (`str`, *optional*, defaults to `"no_norm"`) : The normalization type in MobileBERT.

classifier_activation (`bool`, *optional*, defaults to `True`) : The activation function for the classification head.

classifier_dropout (`Union[float, int]`, *optional*) : The dropout ratio for classifier.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a MobileBertModel. It is used to instantiate a Mobilebert
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/mobilebert-uncased](https://huggingface.co/google/mobilebert-uncased)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import MobileBertConfig, MobileBertModel

>>> # Initializing a MobileBERT configuration
>>> configuration = MobileBertConfig()

>>> # Initializing a model (with random weights) from the configuration above
>>> model = MobileBertModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MobileBertTokenizer[[transformers.BertTokenizer]]

#### transformers.BertTokenizer[[transformers.BertTokenizer]]

```python
transformers.BertTokenizer(vocab: str | dict[str, int] | None = None, do_lower_case: bool = True, unk_token: str = '[UNK]', sep_token: str = '[SEP]', pad_token: str = '[PAD]', cls_token: str = '[CLS]', mask_token: str = '[MASK]', tokenize_chinese_chars: bool = True, strip_accents: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/bert/tokenization_bert.py#L41)

**Parameters:**

vocab (`str` or `dict[str, int]`, *optional*) : Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file`.

do_lower_case (`bool`, *optional*, defaults to `True`) : Whether or not to lowercase the input when tokenizing.

unk_token (`str`, *optional*, defaults to `"[UNK]"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

sep_token (`str`, *optional*, defaults to `"[SEP]"`) : The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for sequence classification or for a text and a question for question answering. It is also used as the last token of a sequence built with special tokens.

pad_token (`str`, *optional*, defaults to `"[PAD]"`) : The token used for padding, for example when batching sequences of different lengths.

cls_token (`str`, *optional*, defaults to `"[CLS]"`) : The classifier token which is used when doing sequence classification (classification of the whole sequence instead of per-token classification). It is the first token of the sequence when built with special tokens.

mask_token (`str`, *optional*, defaults to `"[MASK]"`) : The token used for masking values. This is the token used when training this model with masked language modeling. This is the token which the model will try to predict.

tokenize_chinese_chars (`bool`, *optional*, defaults to `True`) : Whether or not to tokenize Chinese characters.

strip_accents (`bool`, *optional*) : Whether or not to strip all accents. If this option is not specified, then it will be determined by the value for `lowercase` (as in the original BERT).

Construct a BERT tokenizer (backed by HuggingFace's tokenizers library). Based on WordPiece.

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods.

## MobileBertTokenizerFast[[transformers.BertTokenizer]]

#### transformers.BertTokenizer[[transformers.BertTokenizer]]

```python
transformers.BertTokenizer(vocab: str | dict[str, int] | None = None, do_lower_case: bool = True, unk_token: str = '[UNK]', sep_token: str = '[SEP]', pad_token: str = '[PAD]', cls_token: str = '[CLS]', mask_token: str = '[MASK]', tokenize_chinese_chars: bool = True, strip_accents: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/bert/tokenization_bert.py#L41)

**Parameters:**

vocab (`str` or `dict[str, int]`, *optional*) : Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file`.

do_lower_case (`bool`, *optional*, defaults to `True`) : Whether or not to lowercase the input when tokenizing.

unk_token (`str`, *optional*, defaults to `"[UNK]"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

sep_token (`str`, *optional*, defaults to `"[SEP]"`) : The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for sequence classification or for a text and a question for question answering. It is also used as the last token of a sequence built with special tokens.

pad_token (`str`, *optional*, defaults to `"[PAD]"`) : The token used for padding, for example when batching sequences of different lengths.

cls_token (`str`, *optional*, defaults to `"[CLS]"`) : The classifier token which is used when doing sequence classification (classification of the whole sequence instead of per-token classification). It is the first token of the sequence when built with special tokens.

mask_token (`str`, *optional*, defaults to `"[MASK]"`) : The token used for masking values. This is the token used when training this model with masked language modeling. This is the token which the model will try to predict.

tokenize_chinese_chars (`bool`, *optional*, defaults to `True`) : Whether or not to tokenize Chinese characters.

strip_accents (`bool`, *optional*) : Whether or not to strip all accents. If this option is not specified, then it will be determined by the value for `lowercase` (as in the original BERT).

Construct a BERT tokenizer (backed by HuggingFace's tokenizers library). Based on WordPiece.

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods.

## MobileBert specific outputs[[transformers.models.mobilebert.modeling_mobilebert.MobileBertForPreTrainingOutput]]

#### transformers.models.mobilebert.modeling_mobilebert.MobileBertForPreTrainingOutput[[transformers.models.mobilebert.modeling_mobilebert.MobileBertForPreTrainingOutput]]

```python
transformers.models.mobilebert.modeling_mobilebert.MobileBertForPreTrainingOutput(loss: typing.Optional[torch.FloatTensor] = None, prediction_logits: typing.Optional[torch.FloatTensor] = None, seq_relationship_logits: typing.Optional[torch.FloatTensor] = None, hidden_states: tuple[torch.FloatTensor] | None = None, attentions: tuple[torch.FloatTensor] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L565)

**Parameters:**

loss (`*optional*`, returned when `labels` is provided, `torch.FloatTensor` of shape `(1,)`) : Total loss as the sum of the masked language modeling loss and the next sequence prediction (classification) loss.

prediction_logits (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) : Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).

seq_relationship_logits (`torch.FloatTensor` of shape `(batch_size, 2)`) : Prediction scores of the next sequence prediction (classification) head (scores of True/False continuation before SoftMax).

hidden_states (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) : Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, + one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

attentions (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) : Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention heads.

Output type of [MobileBertForPreTraining](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForPreTraining).

## MobileBertModel[[transformers.MobileBertModel]]

#### transformers.MobileBertModel[[transformers.MobileBertModel]]

```python
transformers.MobileBertModel(config, add_pooling_layer = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L585)

**Parameters:**

config ([MobileBertModel](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether to add a pooling layer

The bare Mobilebert Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MobileBertModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L613)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileBertConfig](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertConfig)) and inputs.

The [MobileBertModel](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertModel) forward method, overrides the `__call__` special method.

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

## MobileBertForPreTraining[[transformers.MobileBertForPreTraining]]

#### transformers.MobileBertForPreTraining[[transformers.MobileBertForPreTraining]]

```python
transformers.MobileBertForPreTraining(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L661)

**Parameters:**

config ([MobileBertForPreTraining](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForPreTraining)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MobileBert Model with two heads on top as done during the pretraining: a `masked language modeling` head and a
`next sentence prediction (classification)` head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MobileBertForPreTraining.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, next_sentence_label: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L690)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should be in `[-100, 0, ..., config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`

next_sentence_label (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the next sequence prediction (classification) loss. Input should be a sequence pair (see `input_ids` docstring) Indices should be in `[0, 1]`:  - 0 indicates sequence B is a continuation of sequence A, - 1 indicates sequence B is a random sequence.

**Returns:** [MobileBertForPreTrainingOutput](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.models.mobilebert.modeling_mobilebert.MobileBertForPreTrainingOutput) or `tuple(torch.FloatTensor)`

A [MobileBertForPreTrainingOutput](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.models.mobilebert.modeling_mobilebert.MobileBertForPreTrainingOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileBertConfig](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertConfig)) and inputs.

The [MobileBertForPreTraining](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForPreTraining) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`*optional*`, returned when `labels` is provided, `torch.FloatTensor` of shape `(1,)`) -- Total loss as the sum of the masked language modeling loss and the next sequence prediction
  (classification) loss.
- **prediction_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **seq_relationship_logits** (`torch.FloatTensor` of shape `(batch_size, 2)`) -- Prediction scores of the next sequence prediction (classification) head (scores of True/False continuation
  before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoTokenizer, MobileBertForPreTraining
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("google/mobilebert-uncased")
>>> model = MobileBertForPreTraining.from_pretrained("google/mobilebert-uncased")

>>> input_ids = torch.tensor(tokenizer.encode("Hello, my dog is cute", add_special_tokens=True)).unsqueeze(0)
>>> # Batch size 1
>>> outputs = model(input_ids)

>>> prediction_logits = outputs.prediction_logits
>>> seq_relationship_logits = outputs.seq_relationship_logits
```

## MobileBertForMaskedLM[[transformers.MobileBertForMaskedLM]]

#### transformers.MobileBertForMaskedLM[[transformers.MobileBertForMaskedLM]]

```python
transformers.MobileBertForMaskedLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L760)

**Parameters:**

config ([MobileBertForMaskedLM](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForMaskedLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mobilebert Model with a `language modeling` head on top."

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MobileBertForMaskedLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L789)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should be in `[-100, 0, ..., config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`

**Returns:** [MaskedLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or `tuple(torch.FloatTensor)`

A [MaskedLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileBertConfig](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertConfig)) and inputs.

The [MobileBertForMaskedLM](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForMaskedLM) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, MobileBertForMaskedLM
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("google/mobilebert-uncased")
>>> model = MobileBertForMaskedLM.from_pretrained("google/mobilebert-uncased")

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

## MobileBertForNextSentencePrediction[[transformers.MobileBertForNextSentencePrediction]]

#### transformers.MobileBertForNextSentencePrediction[[transformers.MobileBertForNextSentencePrediction]]

```python
transformers.MobileBertForNextSentencePrediction(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L848)

**Parameters:**

config ([MobileBertForNextSentencePrediction](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForNextSentencePrediction)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MobileBert Model with a `next sentence prediction (classification)` head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MobileBertForNextSentencePrediction.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L858)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the next sequence prediction (classification) loss. Input should be a sequence pair (see `input_ids` docstring) Indices should be in `[0, 1]`.  - 0 indicates sequence B is a continuation of sequence A, - 1 indicates sequence B is a random sequence.

**Returns:** [NextSentencePredictorOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.NextSentencePredictorOutput) or `tuple(torch.FloatTensor)`

A [NextSentencePredictorOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.NextSentencePredictorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileBertConfig](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertConfig)) and inputs.

The [MobileBertForNextSentencePrediction](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForNextSentencePrediction) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `next_sentence_label` is provided) -- Next sequence prediction (classification) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, 2)`) -- Prediction scores of the next sequence prediction (classification) head (scores of True/False continuation
  before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoTokenizer, MobileBertForNextSentencePrediction
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("google/mobilebert-uncased")
>>> model = MobileBertForNextSentencePrediction.from_pretrained("google/mobilebert-uncased")

>>> prompt = "In Italy, pizza served in formal settings, such as at a restaurant, is presented unsliced."
>>> next_sentence = "The sky is blue due to the shorter wavelength of blue light."
>>> encoding = tokenizer(prompt, next_sentence, return_tensors="pt")

>>> outputs = model(**encoding, labels=torch.LongTensor([1]))
>>> loss = outputs.loss
>>> logits = outputs.logits
```

## MobileBertForSequenceClassification[[transformers.MobileBertForSequenceClassification]]

#### transformers.MobileBertForSequenceClassification[[transformers.MobileBertForSequenceClassification]]

```python
transformers.MobileBertForSequenceClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L929)

**Parameters:**

config ([MobileBertForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForSequenceClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MobileBert Model transformer with a sequence classification/regression head on top (a linear layer on top of the
pooled output) e.g. for GLUE tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MobileBertForSequenceClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L945)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

**Returns:** [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`

A [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileBertConfig](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertConfig)) and inputs.

The [MobileBertForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForSequenceClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, MobileBertForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("google/mobilebert-uncased")
>>> model = MobileBertForSequenceClassification.from_pretrained("google/mobilebert-uncased")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> model.config.id2label[predicted_class_id]
...

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = MobileBertForSequenceClassification.from_pretrained("google/mobilebert-uncased", num_labels=num_labels)

>>> labels = torch.tensor([1])
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

Example of multi-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, MobileBertForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("google/mobilebert-uncased")
>>> model = MobileBertForSequenceClassification.from_pretrained("google/mobilebert-uncased", problem_type="multi_label_classification")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_ids = torch.arange(0, logits.shape[-1])[torch.sigmoid(logits).squeeze(dim=0) > 0.5]

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = MobileBertForSequenceClassification.from_pretrained(
...     "google/mobilebert-uncased", num_labels=num_labels, problem_type="multi_label_classification"
... )

>>> labels = torch.sum(
...     torch.nn.functional.one_hot(predicted_class_ids[None, :].clone(), num_classes=num_labels), dim=1
... ).to(torch.float)
>>> loss = model(**inputs, labels=labels).loss
```

## MobileBertForMultipleChoice[[transformers.MobileBertForMultipleChoice]]

#### transformers.MobileBertForMultipleChoice[[transformers.MobileBertForMultipleChoice]]

```python
transformers.MobileBertForMultipleChoice(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L1080)

**Parameters:**

config ([MobileBertForMultipleChoice](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForMultipleChoice)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mobilebert Model with a multiple choice classification head on top (a linear layer on top of the pooled output and a
softmax) e.g. for RocStories/SWAG tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MobileBertForMultipleChoice.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L1094)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, num_choices, sequence_length)`) : Indices of input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, num_choices, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, num_choices, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, num_choices, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the multiple choice classification loss. Indices should be in `[0, ..., num_choices-1]` where `num_choices` is the size of the second dimension of the input tensors. (See `input_ids` above)

**Returns:** [MultipleChoiceModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.MultipleChoiceModelOutput) or `tuple(torch.FloatTensor)`

A [MultipleChoiceModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.MultipleChoiceModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileBertConfig](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertConfig)) and inputs.

The [MobileBertForMultipleChoice](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForMultipleChoice) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape *(1,)*, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_choices)`) -- *num_choices* is the second dimension of the input tensors. (see *input_ids* above).

  Classification scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, MobileBertForMultipleChoice
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("google/mobilebert-uncased")
>>> model = MobileBertForMultipleChoice.from_pretrained("google/mobilebert-uncased")

>>> prompt = "In Italy, pizza served in formal settings, such as at a restaurant, is presented unsliced."
>>> choice0 = "It is eaten with a fork and a knife."
>>> choice1 = "It is eaten while held in the hand."
>>> labels = torch.tensor(0).unsqueeze(0)  # choice0 is correct (according to Wikipedia ;)), batch size 1

>>> encoding = tokenizer([prompt, prompt], [choice0, choice1], return_tensors="pt", padding=True)
>>> outputs = model(**{k: v.unsqueeze(0) for k, v in encoding.items()}, labels=labels)  # batch size is 1

>>> # the linear classifier still needs to be trained
>>> loss = outputs.loss
>>> logits = outputs.logits
```

## MobileBertForTokenClassification[[transformers.MobileBertForTokenClassification]]

#### transformers.MobileBertForTokenClassification[[transformers.MobileBertForTokenClassification]]

```python
transformers.MobileBertForTokenClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L1179)

**Parameters:**

config ([MobileBertForTokenClassification](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForTokenClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mobilebert transformer with a token classification head on top (a linear layer on top of the hidden-states
output) e.g. for Named-Entity-Recognition (NER) tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MobileBertForTokenClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L1194)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the token classification loss. Indices should be in `[0, ..., config.num_labels - 1]`.

**Returns:** [TokenClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or `tuple(torch.FloatTensor)`

A [TokenClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileBertConfig](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertConfig)) and inputs.

The [MobileBertForTokenClassification](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForTokenClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, MobileBertForTokenClassification
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("google/mobilebert-uncased")
>>> model = MobileBertForTokenClassification.from_pretrained("google/mobilebert-uncased")

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

## MobileBertForQuestionAnswering[[transformers.MobileBertForQuestionAnswering]]

#### transformers.MobileBertForQuestionAnswering[[transformers.MobileBertForQuestionAnswering]]

```python
transformers.MobileBertForQuestionAnswering(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L1011)

**Parameters:**

config ([MobileBertForQuestionAnswering](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForQuestionAnswering)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Mobilebert transformer with a span classification head on top for extractive question-answering tasks like
SQuAD (a linear layer on top of the hidden-states output to compute `span start logits` and `span end logits`).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MobileBertForQuestionAnswering.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, start_positions: typing.Optional[torch.Tensor] = None, end_positions: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/mobilebert/modeling_mobilebert.py#L1022)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

start_positions (`torch.Tensor` of shape `(batch_size,)`, *optional*) : Labels for position (index) of the start of the labelled span for computing the token classification loss. Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence are not taken into account for computing the loss.

end_positions (`torch.Tensor` of shape `(batch_size,)`, *optional*) : Labels for position (index) of the end of the labelled span for computing the token classification loss. Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence are not taken into account for computing the loss.

**Returns:** [QuestionAnsweringModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or `tuple(torch.FloatTensor)`

A [QuestionAnsweringModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MobileBertConfig](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertConfig)) and inputs.

The [MobileBertForQuestionAnswering](/docs/transformers/v5.15.1/en/model_doc/mobilebert#transformers.MobileBertForQuestionAnswering) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, MobileBertForQuestionAnswering
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("google/mobilebert-uncased")
>>> model = MobileBertForQuestionAnswering.from_pretrained("google/mobilebert-uncased")

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
