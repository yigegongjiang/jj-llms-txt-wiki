# LUKE

## Overview

The LUKE model was proposed in [LUKE: Deep Contextualized Entity Representations with Entity-aware Self-attention](https://huggingface.co/papers/2010.01057) by Ikuya Yamada, Akari Asai, Hiroyuki Shindo, Hideaki Takeda and Yuji Matsumoto.
It is based on RoBERTa and adds entity embeddings as well as an entity-aware self-attention mechanism, which helps
improve performance on various downstream tasks involving reasoning about entities such as named entity recognition,
extractive and cloze-style question answering, entity typing, and relation classification.

The abstract from the paper is the following:

*Entity representations are useful in natural language tasks involving entities. In this paper, we propose new
pretrained contextualized representations of words and entities based on the bidirectional transformer. The proposed
model treats words and entities in a given text as independent tokens, and outputs contextualized representations of
them. Our model is trained using a new pretraining task based on the masked language model of BERT. The task involves
predicting randomly masked words and entities in a large entity-annotated corpus retrieved from Wikipedia. We also
propose an entity-aware self-attention mechanism that is an extension of the self-attention mechanism of the
transformer, and considers the types of tokens (words or entities) when computing attention scores. The proposed model
achieves impressive empirical performance on a wide range of entity-related tasks. In particular, it obtains
state-of-the-art results on five well-known datasets: Open Entity (entity typing), TACRED (relation classification),
CoNLL-2003 (named entity recognition), ReCoRD (cloze-style question answering), and SQuAD 1.1 (extractive question
answering).*

This model was contributed by [ikuyamada](https://huggingface.co/ikuyamada) and [nielsr](https://huggingface.co/nielsr). The original code can be found [here](https://github.com/studio-ousia/luke).

## Usage tips

- This implementation is the same as [RobertaModel](/docs/transformers/v5.15.1/en/model_doc/roberta#transformers.RobertaModel) with the addition of entity embeddings as well
  as an entity-aware self-attention mechanism, which improves performance on tasks involving reasoning about entities.
- LUKE treats entities as input tokens; therefore, it takes `entity_ids`, `entity_attention_mask`,
  `entity_token_type_ids` and `entity_position_ids` as extra input. You can obtain those using
  [LukeTokenizer](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeTokenizer).
- [LukeTokenizer](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeTokenizer) takes `entities` and `entity_spans` (character-based start and end
  positions of the entities in the input text) as extra input. `entities` typically consist of [MASK] entities or
  Wikipedia entities. The brief description when inputting these entities are as follows:

  - *Inputting [MASK] entities to compute entity representations*: The [MASK] entity is used to mask entities to be
    predicted during pretraining. When LUKE receives the [MASK] entity, it tries to predict the original entity by
    gathering the information about the entity from the input text. Therefore, the [MASK] entity can be used to address
    downstream tasks requiring the information of entities in text such as entity typing, relation classification, and
    named entity recognition.
  - *Inputting Wikipedia entities to compute knowledge-enhanced token representations*: LUKE learns rich information
    (or knowledge) about Wikipedia entities during pretraining and stores the information in its entity embedding. By
    using Wikipedia entities as input tokens, LUKE outputs token representations enriched by the information stored in
    the embeddings of these entities. This is particularly effective for tasks requiring real-world knowledge, such as
    question answering.

- There are three head models for the former use case:

  - [LukeForEntityClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForEntityClassification), for tasks to classify a single entity in an input text such as
    entity typing, e.g. the [Open Entity dataset](https://www.cs.utexas.edu/~eunsol/html_pages/open_entity.html).
    This model places a linear head on top of the output entity representation.
  - [LukeForEntityPairClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForEntityPairClassification), for tasks to classify the relationship between two entities
    such as relation classification, e.g. the [TACRED dataset](https://nlp.stanford.edu/projects/tacred/). This
    model places a linear head on top of the concatenated output representation of the pair of given entities.
  - [LukeForEntitySpanClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForEntitySpanClassification), for tasks to classify the sequence of entity spans, such as
    named entity recognition (NER). This model places a linear head on top of the output entity representations. You
    can address NER using this model by inputting all possible entity spans in the text to the model.

  [LukeTokenizer](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeTokenizer) has a `task` argument, which enables you to easily create an input to these
  head models by specifying `task="entity_classification"`, `task="entity_pair_classification"`, or
  `task="entity_span_classification"`. Please refer to the example code of each head models.

Usage example:

```python
from transformers import LukeForEntityPairClassification, LukeModel, LukeTokenizer

model = LukeModel.from_pretrained("studio-ousia/luke-base", device_map="auto")
tokenizer = LukeTokenizer.from_pretrained("studio-ousia/luke-base")
# Example 1: Computing the contextualized entity representation corresponding to the entity mention "Beyoncé"

text = "Beyoncé lives in Los Angeles."
entity_spans = [(0, 7)]  # character-based entity span corresponding to "Beyoncé"
inputs = tokenizer(text, entity_spans=entity_spans, add_prefix_space=True, return_tensors="pt").to(model.device)
outputs = model(**inputs)
word_last_hidden_state = outputs.last_hidden_state
entity_last_hidden_state = outputs.entity_last_hidden_state
# Example 2: Inputting Wikipedia entities to obtain enriched contextualized representations

entities = [
    "Beyoncé",
    "Los Angeles",
]  # Wikipedia entity titles corresponding to the entity mentions "Beyoncé" and "Los Angeles"
entity_spans = [(0, 7), (17, 28)]  # character-based entity spans corresponding to "Beyoncé" and "Los Angeles"
inputs = tokenizer(text, entities=entities, entity_spans=entity_spans, add_prefix_space=True, return_tensors="pt").to(model.device)
outputs = model(**inputs)
word_last_hidden_state = outputs.last_hidden_state
entity_last_hidden_state = outputs.entity_last_hidden_state
# Example 3: Classifying the relationship between two entities using LukeForEntityPairClassification head model

model = LukeForEntityPairClassification.from_pretrained("studio-ousia/luke-large-finetuned-tacred", device_map="auto")
tokenizer = LukeTokenizer.from_pretrained("studio-ousia/luke-large-finetuned-tacred")
entity_spans = [(0, 7), (17, 28)]  # character-based entity spans corresponding to "Beyoncé" and "Los Angeles"
inputs = tokenizer(text, entity_spans=entity_spans, return_tensors="pt").to(model.device)
outputs = model(**inputs)
logits = outputs.logits
predicted_class_idx = int(logits[0].argmax())
print("Predicted class:", model.config.id2label[predicted_class_idx])
```

## Resources

- [A demo notebook on how to fine-tune [LukeForEntityPairClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForEntityPairClassification) for relation classification](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/LUKE)
- [Notebooks showcasing how you to reproduce the results as reported in the paper with the HuggingFace implementation of LUKE](https://github.com/studio-ousia/luke/tree/master/notebooks)
- [Text classification task guide](../tasks/sequence_classification)
- [Token classification task guide](../tasks/token_classification)
- [Question answering task guide](../tasks/question_answering)
- [Masked language modeling task guide](../tasks/masked_language_modeling)
- [Multiple choice task guide](../tasks/multiple_choice)

## LukeConfig[[transformers.LukeConfig]]

#### transformers.LukeConfig[[transformers.LukeConfig]]

```python
transformers.LukeConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 50267, entity_vocab_size: int = 500000, hidden_size: int = 768, entity_emb_size: int = 256, num_hidden_layers: int = 12, num_attention_heads: int = 12, intermediate_size: int = 3072, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.1, attention_probs_dropout_prob: float | int = 0.1, max_position_embeddings: int = 512, type_vocab_size: int = 2, initializer_range: float = 0.02, layer_norm_eps: float = 1e-12, use_entity_aware_attention: bool = True, classifier_dropout: float | int | None = None, pad_token_id: int | None = 1, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 2, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/configuration_luke.py#L24)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `50267`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

entity_vocab_size (`int`, *optional*, defaults to 500000) : Entity vocabulary size of the LUKE model. Defines the number of different entities that can be represented by the `entity_ids` passed when calling [LukeModel](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeModel).

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

entity_emb_size (`int`, *optional*, defaults to 256) : The number of dimensions of the entity embedding.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

max_position_embeddings (`int`, *optional*, defaults to `512`) : The maximum sequence length that this model might ever be used with.

type_vocab_size (`int`, *optional*, defaults to `2`) : The vocabulary size of the `token_type_ids`.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-12`) : The epsilon used by the layer normalization layers.

use_entity_aware_attention (`bool`, *optional*, defaults to `True`) : Whether or not the model should use the entity-aware self-attention mechanism proposed in [LUKE: Deep Contextualized Entity Representations with Entity-aware Self-attention (Yamada et al.)](https://huggingface.co/papers/2010.01057).

classifier_dropout (`Union[float, int]`, *optional*) : The dropout ratio for classifier.

pad_token_id (`int`, *optional*, defaults to `1`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a LukeModel. It is used to instantiate a Luke
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [studio-ousia/luke-base](https://huggingface.co/studio-ousia/luke-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import LukeConfig, LukeModel

>>> # Initializing a LUKE configuration
>>> configuration = LukeConfig()

>>> # Initializing a model from the configuration
>>> model = LukeModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## LukeTokenizer[[transformers.LukeTokenizer]]

#### transformers.LukeTokenizer[[transformers.LukeTokenizer]]

```python
transformers.LukeTokenizer(vocab: str | dict[str, int] | None = None, merges: str | list[str] | None = None, entity_vocab: str | dict | list | None = None, errors = 'replace', bos_token = '<s>', eos_token = '</s>', sep_token = '</s>', cls_token = '<s>', unk_token = '<unk>', pad_token = '<pad>', mask_token = '<mask>', add_prefix_space = False, task = None, max_entity_length = 32, max_mention_length = 30, entity_token_1 = '<ent>', entity_token_2 = '<ent2>', entity_unk_token = '[UNK]', entity_pad_token = '[PAD]', entity_mask_token = '[MASK]', entity_mask2_token = '[MASK2]', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/tokenization_luke.py#L131)

**Parameters:**

vocab_file (`str`) : Path to the vocabulary file.

merges_file (`str`) : Path to the merges file.

vocab (`str` or `dict[str, int]`, *optional*) : Custom vocabulary dictionary. If not provided, the vocabulary is loaded from `vocab_file`.

merges (`str` or `list[str]`, *optional*) : Custom merges list. If not provided, merges are loaded from `merges_file`.

entity_vocab_file (`str`) : Path to the entity vocabulary file.

task (`str`, *optional*) : Task for which you want to prepare sequences. One of `"entity_classification"`, `"entity_pair_classification"`, or `"entity_span_classification"`. If you specify this argument, the entity sequence is automatically created based on the given entity span(s).

max_entity_length (`int`, *optional*, defaults to 32) : The maximum length of `entity_ids`.

max_mention_length (`int`, *optional*, defaults to 30) : The maximum number of tokens inside an entity span.

entity_token_1 (`str`, *optional*, defaults to `<ent>`) : The special token used to represent an entity span in a word token sequence. This token is only used when `task` is set to `"entity_classification"` or `"entity_pair_classification"`.

entity_token_2 (`str`, *optional*, defaults to `<ent2>`) : The special token used to represent an entity span in a word token sequence. This token is only used when `task` is set to `"entity_pair_classification"`.

errors (`str`, *optional*, defaults to `"replace"`) : Paradigm to follow when decoding bytes to UTF-8. See [bytes.decode](https://docs.python.org/3/library/stdtypes.html#bytes.decode) for more information.

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.    When building a sequence using special tokens, this is not the token that is used for the beginning of sequence. The token used is the `cls_token`.   

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

sep_token (`str`, *optional*, defaults to `"</s>"`) : The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for sequence classification or for a text and a question for question answering. It is also used as the last token of a sequence built with special tokens.

cls_token (`str`, *optional*, defaults to `"<s>"`) : The classifier token which is used when doing sequence classification (classification of the whole sequence instead of per-token classification). It is the first token of the sequence when built with special tokens.

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

mask_token (`str`, *optional*, defaults to `"<mask>"`) : The token used for masking values. This is the token used when training this model with masked language modeling. This is the token which the model will try to predict.

add_prefix_space (`bool`, *optional*, defaults to `False`) : Whether or not to add an initial space to the input. This allows to treat the leading word just as any other word. (LUKE tokenizer detect beginning of words by the preceding space).

Constructs a LUKE tokenizer, derived from the GPT-2 tokenizer, using byte-level Byte-Pair-Encoding.

This tokenizer has been trained to treat spaces like parts of the tokens (a bit like sentencepiece) so a word will

be encoded differently whether it is at the beginning of the sentence (without space) or not:

```python
>>> from transformers import LukeTokenizer

>>> tokenizer = LukeTokenizer.from_pretrained("studio-ousia/luke-base")
>>> tokenizer("Hello world")["input_ids"]
[0, 31414, 232, 2]

>>> tokenizer(" Hello world")["input_ids"]
[0, 20920, 232, 2]
```

You can get around that behavior by passing `add_prefix_space=True` when instantiating this tokenizer or when you
call it on some text, but since the model was not pretrained this way, it might yield a decrease in performance.

When used with `is_split_into_words=True`, this tokenizer will add a space before each word (even the first one).

This tokenizer inherits from [PreTrainedTokenizer](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.PythonBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods. It also creates entity sequences, namely
`entity_ids`, `entity_attention_mask`, `entity_token_type_ids`, and `entity_position_ids` to be used by the LUKE
model.

#### __call__[[transformers.LukeTokenizer.__call__]]

```python
__call__(text: str | list[str], text_pair: str | list[str] | None = None, entity_spans: list[tuple[int, int]] | list[list[tuple[int, int]]] | None = None, entity_spans_pair: list[tuple[int, int]] | list[list[tuple[int, int]]] | None = None, entities: list[str] | list[list[str]] | None = None, entities_pair: list[str] | list[list[str]] | None = None, add_special_tokens: bool = True, padding: bool | str | transformers.utils.generic.PaddingStrategy = False, truncation: bool | str | transformers.tokenization_utils_base.TruncationStrategy = None, max_length: int | None = None, max_entity_length: int | None = None, stride: int = 0, is_split_into_words: bool | None = False, pad_to_multiple_of: int | None = None, padding_side: str | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, return_token_type_ids: bool | None = None, return_attention_mask: bool | None = None, return_overflowing_tokens: bool = False, return_special_tokens_mask: bool = False, return_offsets_mapping: bool = False, return_length: bool = False, verbose: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/tokenization_luke.py#L425)

**Returns:** [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding)

A [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields:

- **input_ids** -- List of token ids to be fed to a model.

  [What are input IDs?](../glossary#input-ids)

- **token_type_ids** -- List of token type ids to be fed to a model (when `return_token_type_ids=True` or
  if *"token_type_ids"* is in `self.model_input_names`).

  [What are token type IDs?](../glossary#token-type-ids)

- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names`).

  [What are attention masks?](../glossary#attention-mask)

- **entity_ids** -- List of entity ids to be fed to a model.

  [What are input IDs?](../glossary#input-ids)

- **entity_position_ids** -- List of entity positions in the input sequence to be fed to a model.

- **entity_token_type_ids** -- List of entity token type ids to be fed to a model (when
  `return_token_type_ids=True` or if *"entity_token_type_ids"* is in `self.model_input_names`).

  [What are token type IDs?](../glossary#token-type-ids)

- **entity_attention_mask** -- List of indices specifying which entities should be attended to by the model
  (when `return_attention_mask=True` or if *"entity_attention_mask"* is in `self.model_input_names`).

  [What are attention masks?](../glossary#attention-mask)

- **entity_start_positions** -- List of the start positions of entities in the word token sequence (when
  `task="entity_span_classification"`).
- **entity_end_positions** -- List of the end positions of entities in the word token sequence (when
  `task="entity_span_classification"`).
- **overflowing_tokens** -- List of overflowing tokens sequences (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **num_truncated_tokens** -- Number of tokens truncated (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **special_tokens_mask** -- List of 0s and 1s, with 1 specifying added special tokens and 0 specifying
  regular sequence tokens (when `add_special_tokens=True` and `return_special_tokens_mask=True`).
- **length** -- The length of the inputs (when `return_length=True`)

add_special_tokens (`bool`, *optional*, defaults to `True`):
Whether or not to add special tokens when encoding the sequences. This will use the underlying
`PretrainedTokenizerBase.build_inputs_with_special_tokens` function, which defines which tokens are
automatically added to the input ids. This is useful if you want to add `bos` or `eos` tokens
automatically.
padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`):
Activates and controls padding. Accepts the following values:

- `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single
  sequence is provided).
- `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum
  acceptable input length for the model if that argument is not provided.
- `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different
  lengths).
truncation (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`):
Activates and controls truncation. Accepts the following values:

- `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or
  to the maximum acceptable input length for the model if that argument is not provided. This will
  truncate token by token, removing a token from the longest sequence in the pair if a pair of
  sequences (or a batch of pairs) is provided.
- `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the
  maximum acceptable input length for the model if that argument is not provided. This will only
  truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided.
- `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the
  maximum acceptable input length for the model if that argument is not provided. This will only
  truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided.
- `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths
  greater than the model maximum admissible input size).
max_length (`int`, *optional*):
Controls the maximum length to use by one of the truncation/padding parameters.

If left unset or set to `None`, this will use the predefined model maximum length if a maximum length
is required by one of the truncation/padding parameters. If the model has no specific maximum input
length (like XLNet) truncation/padding to a maximum length will be deactivated.
stride (`int`, *optional*, defaults to 0):
If set to a number along with `max_length`, the overflowing tokens returned when
`return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence
returned to provide some overlap between truncated and overflowing sequences. The value of this
argument defines the number of overlapping tokens.
is_split_into_words (`bool`, *optional*, defaults to `False`):
Whether or not the input is already pre-tokenized (e.g., split into words). If set to `True`, the
tokenizer assumes the input is already split into words (for instance, by splitting it on whitespace)
which it will tokenize. This is useful for NER or token classification.
pad_to_multiple_of (`int`, *optional*):
If set will pad the sequence to a multiple of the provided value. Requires `padding` to be activated.
This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability
`>= 7.5` (Volta).
padding_side (`str`, *optional*):
The side on which the model should have padding applied. Should be selected between ['right', 'left'].
Default value is picked from the class attribute of the same name.
return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*):
If set, will return tensors instead of list of python integers. Acceptable values are:

- `'pt'`: Return PyTorch `torch.Tensor` objects.
- `'np'`: Return Numpy `np.ndarray` objects.

return_token_type_ids (`bool`, *optional*):
Whether to return token type IDs. If left to the default, will return the token type IDs according to
the specific tokenizer's default, defined by the `return_outputs` attribute.

[What are token type IDs?](../glossary#token-type-ids)
return_attention_mask (`bool`, *optional*):
Whether to return the attention mask. If left to the default, will return the attention mask according
to the specific tokenizer's default, defined by the `return_outputs` attribute.

[What are attention masks?](../glossary#attention-mask)
return_overflowing_tokens (`bool`, *optional*, defaults to `False`):
Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch
of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead
of returning overflowing tokens.
return_special_tokens_mask (`bool`, *optional*, defaults to `False`):
Whether or not to return special tokens mask information.
return_offsets_mapping (`bool`, *optional*, defaults to `False`):
Whether or not to return `(char_start, char_end)` for each token.

This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend), if using
Python's tokenizer, this method will raise `NotImplementedError`.
return_length  (`bool`, *optional*, defaults to `False`):
Whether or not to return the lengths of the encoded inputs.
verbose (`bool`, *optional*, defaults to `True`):
Whether or not to print more information and warnings.
**kwargs: passed to the `self.tokenize()` method

#### save_vocabulary[[transformers.LukeTokenizer.save_vocabulary]]

```python
save_vocabulary(save_directory: str, filename_prefix: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_tokenizers.py#L509)

## LukeModel[[transformers.LukeModel]]

#### transformers.LukeModel[[transformers.LukeModel]]

```python
transformers.LukeModel(config: LukeConfig, add_pooling_layer: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L788)

**Parameters:**

config ([LukeConfig](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether to add a pooling layer

The bare LUKE model transformer outputting raw hidden-states for both word tokens and entities without any

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LukeModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, entity_ids: typing.Optional[torch.LongTensor] = None, entity_attention_mask: typing.Optional[torch.FloatTensor] = None, entity_token_type_ids: typing.Optional[torch.LongTensor] = None, entity_position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L818)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

entity_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`) : Indices of entity tokens in the entity vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

entity_attention_mask (`torch.FloatTensor` of shape `(batch_size, entity_length)`, *optional*) : Mask to avoid performing attention on padding entity token indices. Mask values selected in `[0, 1]`:  - 1 for entity tokens that are **not masked**, - 0 for entity tokens that are **masked**.

entity_token_type_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`, *optional*) : Segment token indices to indicate first and second portions of the entity token inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *portion A* entity token, - 1 corresponds to a *portion B* entity token.

entity_position_ids (`torch.LongTensor` of shape `(batch_size, entity_length, max_mention_length)`, *optional*) : Indices of positions of each input entity in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `BaseLukeModelOutputWithPooling` or `tuple(torch.FloatTensor)`

A `BaseLukeModelOutputWithPooling` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LukeConfig](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeConfig)) and inputs.

The [LukeModel](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) further processed by a
  Linear layer and a Tanh activation function.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **entity_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, entity_length, hidden_size)`) -- Sequence of entity hidden-states at the output of the last layer of the model.
- **entity_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, entity_length, hidden_size)`. Entity hidden-states of the model at the output of each
  layer plus the initial entity embedding outputs.

Examples:

```python
>>> from transformers import AutoTokenizer, LukeModel

>>> tokenizer = AutoTokenizer.from_pretrained("studio-ousia/luke-base")
>>> model = LukeModel.from_pretrained("studio-ousia/luke-base")
# Compute the contextualized entity representation corresponding to the entity mention "Beyoncé"

>>> text = "Beyoncé lives in Los Angeles."
>>> entity_spans = [(0, 7)]  # character-based entity span corresponding to "Beyoncé"

>>> encoding = tokenizer(text, entity_spans=entity_spans, add_prefix_space=True, return_tensors="pt")
>>> outputs = model(**encoding)
>>> word_last_hidden_state = outputs.last_hidden_state
>>> entity_last_hidden_state = outputs.entity_last_hidden_state
# Input Wikipedia entities to obtain enriched contextualized representations of word tokens

>>> text = "Beyoncé lives in Los Angeles."
>>> entities = [
...     "Beyoncé",
...     "Los Angeles",
... ]  # Wikipedia entity titles corresponding to the entity mentions "Beyoncé" and "Los Angeles"
>>> entity_spans = [
...     (0, 7),
...     (17, 28),
... ]  # character-based entity spans corresponding to "Beyoncé" and "Los Angeles"

>>> encoding = tokenizer(
...     text, entities=entities, entity_spans=entity_spans, add_prefix_space=True, return_tensors="pt"
... )
>>> outputs = model(**encoding)
>>> word_last_hidden_state = outputs.last_hidden_state
>>> entity_last_hidden_state = outputs.entity_last_hidden_state
```

## LukeForMaskedLM[[transformers.LukeForMaskedLM]]

#### transformers.LukeForMaskedLM[[transformers.LukeForMaskedLM]]

```python
transformers.LukeForMaskedLM(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1019)

**Parameters:**

config ([LukeForMaskedLM](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForMaskedLM)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LUKE model with a language modeling head and entity prediction head on top for masked language modeling and
masked entity prediction.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LukeForMaskedLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, entity_ids: typing.Optional[torch.LongTensor] = None, entity_attention_mask: typing.Optional[torch.LongTensor] = None, entity_token_type_ids: typing.Optional[torch.LongTensor] = None, entity_position_ids: typing.Optional[torch.LongTensor] = None, labels: typing.Optional[torch.LongTensor] = None, entity_labels: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1044)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

entity_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`) : Indices of entity tokens in the entity vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

entity_attention_mask (`torch.FloatTensor` of shape `(batch_size, entity_length)`, *optional*) : Mask to avoid performing attention on padding entity token indices. Mask values selected in `[0, 1]`:  - 1 for entity tokens that are **not masked**, - 0 for entity tokens that are **masked**.

entity_token_type_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`, *optional*) : Segment token indices to indicate first and second portions of the entity token inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *portion A* entity token, - 1 corresponds to a *portion B* entity token.

entity_position_ids (`torch.LongTensor` of shape `(batch_size, entity_length, max_mention_length)`, *optional*) : Indices of positions of each input entity in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should be in `[-100, 0, ..., config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`

entity_labels (`torch.LongTensor` of shape `(batch_size, entity_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should be in `[-100, 0, ..., config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `LukeMaskedLMOutput` or `tuple(torch.FloatTensor)`

A `LukeMaskedLMOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LukeConfig](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeConfig)) and inputs.

The [LukeForMaskedLM](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForMaskedLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- The sum of masked language modeling (MLM) loss and entity prediction loss.
- **mlm_loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Masked language modeling (MLM) loss.
- **mep_loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Masked entity prediction (MEP) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **entity_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the entity prediction head (scores for each entity vocabulary token before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **entity_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, entity_length, hidden_size)`. Entity hidden-states of the model at the output of each
  layer plus the initial entity embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, LukeForMaskedLM
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("studio-ousia/luke-base")
>>> model = LukeForMaskedLM.from_pretrained("studio-ousia/luke-base")

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

## LukeForEntityClassification[[transformers.LukeForEntityClassification]]

#### transformers.LukeForEntityClassification[[transformers.LukeForEntityClassification]]

```python
transformers.LukeForEntityClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1166)

**Parameters:**

config ([LukeForEntityClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForEntityClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LUKE model with a classification head on top (a linear layer on top of the hidden state of the first entity
token) for entity classification tasks, such as Open Entity.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LukeForEntityClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, entity_ids: typing.Optional[torch.LongTensor] = None, entity_attention_mask: typing.Optional[torch.FloatTensor] = None, entity_token_type_ids: typing.Optional[torch.LongTensor] = None, entity_position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1179)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

entity_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`) : Indices of entity tokens in the entity vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

entity_attention_mask (`torch.FloatTensor` of shape `(batch_size, entity_length)`, *optional*) : Mask to avoid performing attention on padding entity token indices. Mask values selected in `[0, 1]`:  - 1 for entity tokens that are **not masked**, - 0 for entity tokens that are **masked**.

entity_token_type_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`, *optional*) : Segment token indices to indicate first and second portions of the entity token inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *portion A* entity token, - 1 corresponds to a *portion B* entity token.

entity_position_ids (`torch.LongTensor` of shape `(batch_size, entity_length, max_mention_length)`, *optional*) : Indices of positions of each input entity in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size,)` or `(batch_size, num_labels)`, *optional*) : Labels for computing the classification loss. If the shape is `(batch_size,)`, the cross entropy loss is used for the single-label classification. In this case, labels should contain the indices that should be in `[0, ..., config.num_labels - 1]`. If the shape is `(batch_size, num_labels)`, the binary cross entropy loss is used for the multi-label classification. In this case, labels should only contain `[0, 1]`, where 0 and 1 indicate false and true, respectively.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `EntityClassificationOutput` or `tuple(torch.FloatTensor)`

A `EntityClassificationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LukeConfig](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeConfig)) and inputs.

The [LukeForEntityClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForEntityClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification scores (before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **entity_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, entity_length, hidden_size)`. Entity hidden-states of the model at the output of each
  layer plus the initial entity embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoTokenizer, LukeForEntityClassification

>>> tokenizer = AutoTokenizer.from_pretrained("studio-ousia/luke-large-finetuned-open-entity")
>>> model = LukeForEntityClassification.from_pretrained("studio-ousia/luke-large-finetuned-open-entity")

>>> text = "Beyoncé lives in Los Angeles."
>>> entity_spans = [(0, 7)]  # character-based entity span corresponding to "Beyoncé"
>>> inputs = tokenizer(text, entity_spans=entity_spans, return_tensors="pt")
>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> predicted_class_idx = logits.argmax(-1).item()
>>> print("Predicted class:", model.config.id2label[predicted_class_idx])
Predicted class: person
```

## LukeForEntityPairClassification[[transformers.LukeForEntityPairClassification]]

#### transformers.LukeForEntityPairClassification[[transformers.LukeForEntityPairClassification]]

```python
transformers.LukeForEntityPairClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1295)

**Parameters:**

config ([LukeForEntityPairClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForEntityPairClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LUKE model with a classification head on top (a linear layer on top of the hidden states of the two entity
tokens) for entity pair classification tasks, such as TACRED.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LukeForEntityPairClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, entity_ids: typing.Optional[torch.LongTensor] = None, entity_attention_mask: typing.Optional[torch.FloatTensor] = None, entity_token_type_ids: typing.Optional[torch.LongTensor] = None, entity_position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1308)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

entity_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`) : Indices of entity tokens in the entity vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

entity_attention_mask (`torch.FloatTensor` of shape `(batch_size, entity_length)`, *optional*) : Mask to avoid performing attention on padding entity token indices. Mask values selected in `[0, 1]`:  - 1 for entity tokens that are **not masked**, - 0 for entity tokens that are **masked**.

entity_token_type_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`, *optional*) : Segment token indices to indicate first and second portions of the entity token inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *portion A* entity token, - 1 corresponds to a *portion B* entity token.

entity_position_ids (`torch.LongTensor` of shape `(batch_size, entity_length, max_mention_length)`, *optional*) : Indices of positions of each input entity in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size,)` or `(batch_size, num_labels)`, *optional*) : Labels for computing the classification loss. If the shape is `(batch_size,)`, the cross entropy loss is used for the single-label classification. In this case, labels should contain the indices that should be in `[0, ..., config.num_labels - 1]`. If the shape is `(batch_size, num_labels)`, the binary cross entropy loss is used for the multi-label classification. In this case, labels should only contain `[0, 1]`, where 0 and 1 indicate false and true, respectively.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `EntityPairClassificationOutput` or `tuple(torch.FloatTensor)`

A `EntityPairClassificationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LukeConfig](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeConfig)) and inputs.

The [LukeForEntityPairClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForEntityPairClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification scores (before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **entity_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, entity_length, hidden_size)`. Entity hidden-states of the model at the output of each
  layer plus the initial entity embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoTokenizer, LukeForEntityPairClassification

>>> tokenizer = AutoTokenizer.from_pretrained("studio-ousia/luke-large-finetuned-tacred")
>>> model = LukeForEntityPairClassification.from_pretrained("studio-ousia/luke-large-finetuned-tacred")

>>> text = "Beyoncé lives in Los Angeles."
>>> entity_spans = [
...     (0, 7),
...     (17, 28),
... ]  # character-based entity spans corresponding to "Beyoncé" and "Los Angeles"
>>> inputs = tokenizer(text, entity_spans=entity_spans, return_tensors="pt")
>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> predicted_class_idx = logits.argmax(-1).item()
>>> print("Predicted class:", model.config.id2label[predicted_class_idx])
Predicted class: per:cities_of_residence
```

## LukeForEntitySpanClassification[[transformers.LukeForEntitySpanClassification]]

#### transformers.LukeForEntitySpanClassification[[transformers.LukeForEntitySpanClassification]]

```python
transformers.LukeForEntitySpanClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1429)

**Parameters:**

config ([LukeForEntitySpanClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForEntitySpanClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LUKE model with a span classification head on top (a linear layer on top of the hidden states output) for tasks
such as named entity recognition.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LukeForEntitySpanClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, entity_ids: typing.Optional[torch.LongTensor] = None, entity_attention_mask: typing.Optional[torch.LongTensor] = None, entity_token_type_ids: typing.Optional[torch.LongTensor] = None, entity_position_ids: typing.Optional[torch.LongTensor] = None, entity_start_positions: typing.Optional[torch.LongTensor] = None, entity_end_positions: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1442)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

entity_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`) : Indices of entity tokens in the entity vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

entity_attention_mask (`torch.FloatTensor` of shape `(batch_size, entity_length)`, *optional*) : Mask to avoid performing attention on padding entity token indices. Mask values selected in `[0, 1]`:  - 1 for entity tokens that are **not masked**, - 0 for entity tokens that are **masked**.

entity_token_type_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`, *optional*) : Segment token indices to indicate first and second portions of the entity token inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *portion A* entity token, - 1 corresponds to a *portion B* entity token.

entity_position_ids (`torch.LongTensor` of shape `(batch_size, entity_length, max_mention_length)`, *optional*) : Indices of positions of each input entity in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.

entity_start_positions (`torch.LongTensor`, *optional*) : The start positions of entities in the word token sequence.

entity_end_positions (`torch.LongTensor`, *optional*) : The end positions of entities in the word token sequence.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, entity_length)` or `(batch_size, entity_length, num_labels)`, *optional*) : Labels for computing the classification loss. If the shape is `(batch_size, entity_length)`, the cross entropy loss is used for the single-label classification. In this case, labels should contain the indices that should be in `[0, ..., config.num_labels - 1]`. If the shape is `(batch_size, entity_length, num_labels)`, the binary cross entropy loss is used for the multi-label classification. In this case, labels should only contain `[0, 1]`, where 0 and 1 indicate false and true, respectively.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `EntitySpanClassificationOutput` or `tuple(torch.FloatTensor)`

A `EntitySpanClassificationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LukeConfig](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeConfig)) and inputs.

The [LukeForEntitySpanClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForEntitySpanClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, entity_length, config.num_labels)`) -- Classification scores (before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **entity_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, entity_length, hidden_size)`. Entity hidden-states of the model at the output of each
  layer plus the initial entity embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoTokenizer, LukeForEntitySpanClassification

>>> tokenizer = AutoTokenizer.from_pretrained("studio-ousia/luke-large-finetuned-conll-2003")
>>> model = LukeForEntitySpanClassification.from_pretrained("studio-ousia/luke-large-finetuned-conll-2003")

>>> text = "Beyoncé lives in Los Angeles"
# List all possible entity spans in the text

>>> word_start_positions = [0, 8, 14, 17, 21]  # character-based start positions of word tokens
>>> word_end_positions = [7, 13, 16, 20, 28]  # character-based end positions of word tokens
>>> entity_spans = []
>>> for i, start_pos in enumerate(word_start_positions):
...     for end_pos in word_end_positions[i:]:
...         entity_spans.append((start_pos, end_pos))

>>> inputs = tokenizer(text, entity_spans=entity_spans, return_tensors="pt")
>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> predicted_class_indices = logits.argmax(-1).squeeze().tolist()
>>> for span, predicted_class_idx in zip(entity_spans, predicted_class_indices):
...     if predicted_class_idx != 0:
...         print(text[span[0] : span[1]], model.config.id2label[predicted_class_idx])
Beyoncé PER
Los Angeles LOC
```

## LukeForSequenceClassification[[transformers.LukeForSequenceClassification]]

#### transformers.LukeForSequenceClassification[[transformers.LukeForSequenceClassification]]

```python
transformers.LukeForSequenceClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1587)

**Parameters:**

config ([LukeForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForSequenceClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LUKE Model transformer with a sequence classification/regression head on top (a linear layer on top of the
pooled output) e.g. for GLUE tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LukeForSequenceClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, entity_ids: typing.Optional[torch.LongTensor] = None, entity_attention_mask: typing.Optional[torch.FloatTensor] = None, entity_token_type_ids: typing.Optional[torch.LongTensor] = None, entity_position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1600)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

entity_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`) : Indices of entity tokens in the entity vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

entity_attention_mask (`torch.FloatTensor` of shape `(batch_size, entity_length)`, *optional*) : Mask to avoid performing attention on padding entity token indices. Mask values selected in `[0, 1]`:  - 1 for entity tokens that are **not masked**, - 0 for entity tokens that are **masked**.

entity_token_type_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`, *optional*) : Segment token indices to indicate first and second portions of the entity token inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *portion A* entity token, - 1 corresponds to a *portion B* entity token.

entity_position_ids (`torch.LongTensor` of shape `(batch_size, entity_length, max_mention_length)`, *optional*) : Indices of positions of each input entity in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `LukeSequenceClassifierOutput` or `tuple(torch.FloatTensor)`

A `LukeSequenceClassifierOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LukeConfig](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeConfig)) and inputs.

The [LukeForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForSequenceClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **entity_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, entity_length, hidden_size)`. Entity hidden-states of the model at the output of each
  layer plus the initial entity embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example of single-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, LukeForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("studio-ousia/luke-base")
>>> model = LukeForSequenceClassification.from_pretrained("studio-ousia/luke-base")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> model.config.id2label[predicted_class_id]
...

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = LukeForSequenceClassification.from_pretrained("studio-ousia/luke-base", num_labels=num_labels)

>>> labels = torch.tensor([1])
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

Example of multi-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, LukeForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("studio-ousia/luke-base")
>>> model = LukeForSequenceClassification.from_pretrained("studio-ousia/luke-base", problem_type="multi_label_classification")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_ids = torch.arange(0, logits.shape[-1])[torch.sigmoid(logits).squeeze(dim=0) > 0.5]

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = LukeForSequenceClassification.from_pretrained(
...     "studio-ousia/luke-base", num_labels=num_labels, problem_type="multi_label_classification"
... )

>>> labels = torch.sum(
...     torch.nn.functional.one_hot(predicted_class_ids[None, :].clone(), num_classes=num_labels), dim=1
... ).to(torch.float)
>>> loss = model(**inputs, labels=labels).loss
```

## LukeForMultipleChoice[[transformers.LukeForMultipleChoice]]

#### transformers.LukeForMultipleChoice[[transformers.LukeForMultipleChoice]]

```python
transformers.LukeForMultipleChoice(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1934)

**Parameters:**

config ([LukeForMultipleChoice](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForMultipleChoice)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Luke Model with a multiple choice classification head on top (a linear layer on top of the pooled output and a
softmax) e.g. for RocStories/SWAG tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LukeForMultipleChoice.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, entity_ids: typing.Optional[torch.LongTensor] = None, entity_attention_mask: typing.Optional[torch.FloatTensor] = None, entity_token_type_ids: typing.Optional[torch.LongTensor] = None, entity_position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1947)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, num_choices, sequence_length)`) : Indices of input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, num_choices, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, num_choices, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.  [What are position IDs?](../glossary#position-ids)

entity_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`) : Indices of entity tokens in the entity vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

entity_attention_mask (`torch.FloatTensor` of shape `(batch_size, entity_length)`, *optional*) : Mask to avoid performing attention on padding entity token indices. Mask values selected in `[0, 1]`:  - 1 for entity tokens that are **not masked**, - 0 for entity tokens that are **masked**.

entity_token_type_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`, *optional*) : Segment token indices to indicate first and second portions of the entity token inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *portion A* entity token, - 1 corresponds to a *portion B* entity token.

entity_position_ids (`torch.LongTensor` of shape `(batch_size, entity_length, max_mention_length)`, *optional*) : Indices of positions of each input entity in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, num_choices, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the multiple choice classification loss. Indices should be in `[0, ..., num_choices-1]` where `num_choices` is the size of the second dimension of the input tensors. (See `input_ids` above)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `LukeMultipleChoiceModelOutput` or `tuple(torch.FloatTensor)`

A `LukeMultipleChoiceModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LukeConfig](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeConfig)) and inputs.

The [LukeForMultipleChoice](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForMultipleChoice) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape *(1,)*, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_choices)`) -- *num_choices* is the second dimension of the input tensors. (see *input_ids* above).

  Classification scores (before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **entity_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, entity_length, hidden_size)`. Entity hidden-states of the model at the output of each
  layer plus the initial entity embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, LukeForMultipleChoice
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("studio-ousia/luke-base")
>>> model = LukeForMultipleChoice.from_pretrained("studio-ousia/luke-base")

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

## LukeForTokenClassification[[transformers.LukeForTokenClassification]]

#### transformers.LukeForTokenClassification[[transformers.LukeForTokenClassification]]

```python
transformers.LukeForTokenClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1713)

**Parameters:**

config ([LukeForTokenClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForTokenClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LUKE Model with a token classification head on top (a linear layer on top of the hidden-states output). To
solve Named-Entity Recognition (NER) task using LUKE, `LukeForEntitySpanClassification` is more suitable than this
class.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LukeForTokenClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, entity_ids: typing.Optional[torch.LongTensor] = None, entity_attention_mask: typing.Optional[torch.FloatTensor] = None, entity_token_type_ids: typing.Optional[torch.LongTensor] = None, entity_position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1727)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

entity_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`) : Indices of entity tokens in the entity vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

entity_attention_mask (`torch.FloatTensor` of shape `(batch_size, entity_length)`, *optional*) : Mask to avoid performing attention on padding entity token indices. Mask values selected in `[0, 1]`:  - 1 for entity tokens that are **not masked**, - 0 for entity tokens that are **masked**.

entity_token_type_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`, *optional*) : Segment token indices to indicate first and second portions of the entity token inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *portion A* entity token, - 1 corresponds to a *portion B* entity token.

entity_position_ids (`torch.LongTensor` of shape `(batch_size, entity_length, max_mention_length)`, *optional*) : Indices of positions of each input entity in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the multiple choice classification loss. Indices should be in `[0, ..., num_choices-1]` where `num_choices` is the size of the second dimension of the input tensors. (See `input_ids` above)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `LukeTokenClassifierOutput` or `tuple(torch.FloatTensor)`

A `LukeTokenClassifierOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LukeConfig](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeConfig)) and inputs.

The [LukeForTokenClassification](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForTokenClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`) -- Classification scores (before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **entity_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, entity_length, hidden_size)`. Entity hidden-states of the model at the output of each
  layer plus the initial entity embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, LukeForTokenClassification
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("studio-ousia/luke-base")
>>> model = LukeForTokenClassification.from_pretrained("studio-ousia/luke-base")

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

## LukeForQuestionAnswering[[transformers.LukeForQuestionAnswering]]

#### transformers.LukeForQuestionAnswering[[transformers.LukeForQuestionAnswering]]

```python
transformers.LukeForQuestionAnswering(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1816)

**Parameters:**

config ([LukeForQuestionAnswering](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForQuestionAnswering)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Luke transformer with a span classification head on top for extractive question-answering tasks like
SQuAD (a linear layer on top of the hidden-states output to compute `span start logits` and `span end logits`).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.LukeForQuestionAnswering.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.FloatTensor] = None, entity_ids: typing.Optional[torch.LongTensor] = None, entity_attention_mask: typing.Optional[torch.FloatTensor] = None, entity_token_type_ids: typing.Optional[torch.LongTensor] = None, entity_position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, start_positions: typing.Optional[torch.LongTensor] = None, end_positions: typing.Optional[torch.LongTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/luke/modeling_luke.py#L1828)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

entity_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`) : Indices of entity tokens in the entity vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

entity_attention_mask (`torch.FloatTensor` of shape `(batch_size, entity_length)`, *optional*) : Mask to avoid performing attention on padding entity token indices. Mask values selected in `[0, 1]`:  - 1 for entity tokens that are **not masked**, - 0 for entity tokens that are **masked**.

entity_token_type_ids (`torch.LongTensor` of shape `(batch_size, entity_length)`, *optional*) : Segment token indices to indicate first and second portions of the entity token inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *portion A* entity token, - 1 corresponds to a *portion B* entity token.

entity_position_ids (`torch.LongTensor` of shape `(batch_size, entity_length, max_mention_length)`, *optional*) : Indices of positions of each input entity in the position embeddings. Selected in the range `[0, config.max_position_embeddings - 1]`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

start_positions (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for position (index) of the start of the labelled span for computing the token classification loss. Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence are not taken into account for computing the loss.

end_positions (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for position (index) of the end of the labelled span for computing the token classification loss. Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence are not taken into account for computing the loss.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `LukeQuestionAnsweringModelOutput` or `tuple(torch.FloatTensor)`

A `LukeQuestionAnsweringModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LukeConfig](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeConfig)) and inputs.

The [LukeForQuestionAnswering](/docs/transformers/v5.15.1/en/model_doc/luke#transformers.LukeForQuestionAnswering) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Total span extraction loss is the sum of a Cross-Entropy for the start and end positions.
- **start_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) -- Span-start scores (before SoftMax).
- **end_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) -- Span-end scores (before SoftMax).
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **entity_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, entity_length, hidden_size)`. Entity hidden-states of the model at the output of each
  layer plus the initial entity embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, LukeForQuestionAnswering
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("studio-ousia/luke-base")
>>> model = LukeForQuestionAnswering.from_pretrained("studio-ousia/luke-base")

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
