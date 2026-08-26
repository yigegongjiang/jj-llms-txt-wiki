# LayoutXLM

## Overview

LayoutXLM was proposed in [LayoutXLM: Multimodal Pre-training for Multilingual Visually-rich Document Understanding](https://huggingface.co/papers/2104.08836) by Yiheng Xu, Tengchao Lv, Lei Cui, Guoxin Wang, Yijuan Lu, Dinei Florencio, Cha
Zhang, Furu Wei. It's a multilingual extension of the [LayoutLMv2 model](https://huggingface.co/papers/2012.14740) trained
on 53 languages.

The abstract from the paper is the following:

*Multimodal pre-training with text, layout, and image has achieved SOTA performance for visually-rich document
understanding tasks recently, which demonstrates the great potential for joint learning across different modalities. In
this paper, we present LayoutXLM, a multimodal pre-trained model for multilingual document understanding, which aims to
bridge the language barriers for visually-rich document understanding. To accurately evaluate LayoutXLM, we also
introduce a multilingual form understanding benchmark dataset named XFUN, which includes form understanding samples in
7 languages (Chinese, Japanese, Spanish, French, Italian, German, Portuguese), and key-value pairs are manually labeled
for each language. Experiment results show that the LayoutXLM model has significantly outperformed the existing SOTA
cross-lingual pre-trained models on the XFUN dataset.*

This model was contributed by [nielsr](https://huggingface.co/nielsr). The original code can be found [here](https://github.com/microsoft/unilm).

## Usage tips and examples

One can directly plug in the weights of LayoutXLM into a LayoutLMv2 model, like so:

```python
from transformers import LayoutLMv2Model

model = LayoutLMv2Model.from_pretrained("microsoft/layoutxlm-base", device_map="auto")
```

Note that LayoutXLM has its own tokenizer, based on
[LayoutXLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/layoutxlm#transformers.LayoutXLMTokenizer)/[LayoutXLMTokenizerFast](/docs/transformers/v5.15.1/en/model_doc/layoutxlm#transformers.LayoutXLMTokenizer). You can initialize it as
follows:

```python
from transformers import LayoutXLMTokenizer

tokenizer = LayoutXLMTokenizer.from_pretrained("microsoft/layoutxlm-base")
```

Similar to LayoutLMv2, you can use [LayoutXLMProcessor](/docs/transformers/v5.15.1/en/model_doc/layoutxlm#transformers.LayoutXLMProcessor) (which internally applies
[LayoutLMv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/layoutlmv2#transformers.LayoutLMv2ImageProcessor) and
[LayoutXLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/layoutxlm#transformers.LayoutXLMTokenizer)/[LayoutXLMTokenizerFast](/docs/transformers/v5.15.1/en/model_doc/layoutxlm#transformers.LayoutXLMTokenizer) in sequence) to prepare all
data for the model.

As LayoutXLM's architecture is equivalent to that of LayoutLMv2, one can refer to [LayoutLMv2's documentation page](layoutlmv2) for all tips, code examples and notebooks.

## LayoutXLMConfig[[transformers.LayoutXLMConfig]]

#### transformers.LayoutXLMConfig[[transformers.LayoutXLMConfig]]

```python
transformers.LayoutXLMConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 30522, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, intermediate_size: int = 3072, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.1, attention_probs_dropout_prob: float | int = 0.1, max_position_embeddings: int = 512, type_vocab_size: int = 2, initializer_range: float = 0.02, layer_norm_eps: float = 1e-12, pad_token_id: int | None = 0, max_2d_position_embeddings: int = 1024, max_rel_pos: int = 128, rel_pos_bins: int = 32, fast_qkv: bool = True, max_rel_2d_pos: int = 256, rel_2d_pos_bins: int = 64, convert_sync_batchnorm: bool = True, image_feature_pool_shape: list[int] | tuple[int, ...] = (7, 7, 256), coordinate_size: int = 128, shape_size: int = 128, has_relative_attention_bias: bool = True, has_spatial_attention_bias: bool = True, has_visual_segment_embedding: bool = False, detectron2_config_args: dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/layoutxlm/configuration_layoutxlm.py#L35)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `30522`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

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

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

max_2d_position_embeddings (`int`, *optional*, defaults to 1024) : The maximum value that the 2D position embedding might ever be used with. Typically set this to something large just in case (e.g., 1024).

max_rel_pos (`int`, *optional*, defaults to 128) : The maximum number of relative positions to be used in the self-attention mechanism.

rel_pos_bins (`int`, *optional*, defaults to 32) : The number of relative position bins to be used in the self-attention mechanism.

fast_qkv (`bool`, *optional*, defaults to `True`) : Whether or not to use a single matrix for the queries, keys, values in the self-attention layers.

max_rel_2d_pos (`int`, *optional*, defaults to 256) : The maximum number of relative 2D positions in the self-attention mechanism.

rel_2d_pos_bins (`int`, *optional*, defaults to 64) : The number of 2D relative position bins in the self-attention mechanism.

convert_sync_batchnorm (`bool`, *optional*, defaults to `True`) : Whether or not to convert batch normalization layers to synchronized batch normalization layers.

image_feature_pool_shape (`list[int]`, *optional*, defaults to `[7, 7, 256]`) : The shape of the average-pooled feature map.

coordinate_size (`int`, *optional*, defaults to 128) : Dimension of the coordinate embeddings.

shape_size (`int`, *optional*, defaults to 128) : Dimension of the width and height embeddings.

has_relative_attention_bias (`bool`, *optional*, defaults to `True`) : Whether or not to use a relative attention bias in the self-attention mechanism.

has_spatial_attention_bias (`bool`, *optional*, defaults to `True`) : Whether or not to use a spatial attention bias in the self-attention mechanism.

has_visual_segment_embedding (`bool`, *optional*, defaults to `False`) : Whether or not to add visual segment embeddings.

detectron2_config_args (`dict`, *optional*) : Dictionary containing the configuration arguments of the Detectron2 visual backbone. Refer to [this file](https://github.com/microsoft/unilm/blob/master/layoutlmft/layoutlmft/models/layoutxlm/detectron2_config.py) for details regarding default values.

This is the configuration class to store the configuration of a LayoutxlmModel. It is used to instantiate a Layoutxlm
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/layoutxlm-base](https://huggingface.co/microsoft/layoutxlm-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import LayoutXLMConfig, LayoutXLMModel

>>> # Initializing a LayoutXLM microsoft/layoutxlm-base style configuration
>>> configuration = LayoutXLMConfig()

>>> # Initializing a model (with random weights) from the microsoft/layoutxlm-base style configuration
>>> model = LayoutXLMModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## LayoutXLMTokenizer[[transformers.LayoutXLMTokenizer]]

#### transformers.LayoutXLMTokenizer[[transformers.LayoutXLMTokenizer]]

```python
transformers.LayoutXLMTokenizer(vocab: str | list | None = None, bos_token = '<s>', eos_token = '</s>', sep_token = '</s>', cls_token = '<s>', unk_token = '<unk>', pad_token = '<pad>', mask_token = '<mask>', cls_token_box = [0, 0, 0, 0], sep_token_box = [1000, 1000, 1000, 1000], pad_token_box = [0, 0, 0, 0], pad_token_label = -100, only_label_first_subword = True, add_prefix_space = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/layoutxlm/tokenization_layoutxlm.py#L140)

**Parameters:**

vocab (`str`, `dict` or `list`, *optional*) : Vocabulary for the tokenizer as a path, a dictionary or a list of `(token, score)` tuples.

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.    When building a sequence using special tokens, this is not the token that is used for the beginning of sequence. The token used is the `cls_token`.   

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

sep_token (`str`, *optional*, defaults to `"</s>"`) : The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for sequence classification or for a text and a question for question answering. It is also used as the last token of a sequence built with special tokens.

cls_token (`str`, *optional*, defaults to `"<s>"`) : The classifier token which is used when doing sequence classification (classification of the whole sequence instead of per-token classification). It is the first token of the sequence when built with special tokens.

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

mask_token (`str`, *optional*, defaults to `"<mask>"`) : The token used for masking values. This is the token used when training this model with masked language modeling. This is the token which the model will try to predict.

cls_token_box (`list[int]`, *optional*, defaults to `[0, 0, 0, 0]`) : The bounding box to use for the special [CLS] token.

sep_token_box (`list[int]`, *optional*, defaults to `[1000, 1000, 1000, 1000]`) : The bounding box to use for the special [SEP] token.

pad_token_box (`list[int]`, *optional*, defaults to `[0, 0, 0, 0]`) : The bounding box to use for the special [PAD] token.

pad_token_label (`int`, *optional*, defaults to -100) : The label to use for padding tokens. Defaults to -100, which is the `ignore_index` of PyTorch's CrossEntropyLoss.

only_label_first_subword (`bool`, *optional*, defaults to `True`) : Whether or not to only label the first subword, in case word labels are provided.

add_prefix_space (`bool`, *optional*, defaults to `True`) : Whether or not to add an initial space to the input.

additional_special_tokens (`list[str]`, *optional*, defaults to `["<s>NOTUSED", "</s>NOTUSED"]`) : Additional special tokens used by the tokenizer.

Construct a "fast" LayoutXLM tokenizer (backed by HuggingFace's *tokenizers* library). Adapted from
[RobertaTokenizer](/docs/transformers/v5.15.1/en/model_doc/roberta#transformers.RobertaTokenizer) and [XLNetTokenizer](/docs/transformers/v5.15.1/en/model_doc/xlnet#transformers.XLNetTokenizer). Based on
[BPE](https://huggingface.co/docs/tokenizers/python/latest/components.html?highlight=BPE#models).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

#### __call__[[transformers.LayoutXLMTokenizer.__call__]]

```python
__call__(text: str | list[str] | list[list[str]], text_pair: list[str] | list[list[str]] | None = None, boxes: list[list[int]] | list[list[list[int]]] | None = None, word_labels: list[int] | list[list[int]] | None = None, add_special_tokens: bool = True, padding: bool | str | transformers.utils.generic.PaddingStrategy = False, truncation: bool | str | transformers.tokenization_utils_base.TruncationStrategy = None, max_length: int | None = None, stride: int = 0, pad_to_multiple_of: int | None = None, padding_side: str | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, return_token_type_ids: bool | None = None, return_attention_mask: bool | None = None, return_overflowing_tokens: bool = False, return_special_tokens_mask: bool = False, return_offsets_mapping: bool = False, return_length: bool = False, verbose: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/layoutxlm/tokenization_layoutxlm.py#L413)

**Parameters:**

text (`str`, `list[str]`, `list[list[str]]`) : The sequence or batch of sequences to be encoded. Each sequence can be a string, a list of strings (words of a single example or questions of a batch of examples) or a list of list of strings (batch of words).

text_pair (`list[str]`, `list[list[str]]`) : The sequence or batch of sequences to be encoded. Each sequence should be a list of strings (pretokenized string).

boxes (`list[list[int]]`, `list[list[list[int]]]`) : Word-level bounding boxes. Each bounding box should be normalized to be on a 0-1000 scale.

word_labels (`list[int]`, `list[list[int]]`, *optional*) : Word-level integer labels (for token classification tasks such as FUNSD, CORD). 

add_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to encode the sequences with the special tokens relative to their model.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Activates and controls padding. Accepts the following values:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

truncation (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) : Activates and controls truncation. Accepts the following values:  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will truncate token by token, removing a token from the longest sequence in the pair if a pair of sequences (or a batch of pairs) is provided. - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths greater than the model maximum admissible input size).

max_length (`int`, *optional*) : Controls the maximum length to use by one of the truncation/padding parameters.  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length is required by one of the truncation/padding parameters. If the model has no specific maximum input length (like XLNet) truncation/padding to a maximum length will be deactivated.

stride (`int`, *optional*, defaults to 0) : If set to a number along with `max_length`, the overflowing tokens returned when `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence returned to provide some overlap between truncated and overflowing sequences. The value of this argument defines the number of overlapping tokens.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value. This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects.

return_token_type_ids (`bool`, *optional*) : Whether to return token type IDs. If left to the default, will return the token type IDs according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are token type IDs?](../glossary#token-type-ids)

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are attention masks?](../glossary#attention-mask)

return_overflowing_tokens (`bool`, *optional*, defaults to `False`) : Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead of returning overflowing tokens.

return_special_tokens_mask (`bool`, *optional*, defaults to `False`) : Whether or not to return special tokens mask information.

return_offsets_mapping (`bool`, *optional*, defaults to `False`) : Whether or not to return `(char_start, char_end)` for each token.  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend), if using Python's tokenizer, this method will raise `NotImplementedError`.

return_length  (`bool`, *optional*, defaults to `False`) : Whether or not to return the lengths of the encoded inputs.

verbose (`bool`, *optional*, defaults to `True`) : Whether or not to print more information and warnings.

- ****kwargs** : passed to the `self.tokenize()` method

**Returns:** [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding)

A [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields:

- **input_ids** -- List of token ids to be fed to a model.

  [What are input IDs?](../glossary#input-ids)

- **bbox** -- List of bounding boxes to be fed to a model.

- **token_type_ids** -- List of token type ids to be fed to a model (when `return_token_type_ids=True` or
  if *"token_type_ids"* is in `self.model_input_names`).

  [What are token type IDs?](../glossary#token-type-ids)

- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names`).

  [What are attention masks?](../glossary#attention-mask)

- **labels** -- List of labels to be fed to a model. (when `word_labels` is specified).
- **overflowing_tokens** -- List of overflowing tokens sequences (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **num_truncated_tokens** -- Number of tokens truncated (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **special_tokens_mask** -- List of 0s and 1s, with 1 specifying added special tokens and 0 specifying
  regular sequence tokens (when `add_special_tokens=True` and `return_special_tokens_mask=True`).
- **length** -- The length of the inputs (when `return_length=True`).

Main method to tokenize and prepare for the model one or several sequence(s) or one or several pair(s) of
sequences with word-level normalized bounding boxes and optional labels.

#### build_inputs_with_special_tokens[[transformers.LayoutXLMTokenizer.build_inputs_with_special_tokens]]

```python
build_inputs_with_special_tokens(token_ids_0: list, token_ids_1: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/layoutxlm/tokenization_layoutxlm.py#L892)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs to which the special tokens will be added.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

**Returns:** `list[int]`

List of [input IDs](../glossary#input-ids) with the appropriate special tokens.

Build model inputs from a sequence or a pair of sequence for sequence classification tasks by concatenating and
adding special tokens. An XLM-RoBERTa sequence has the following format:

- single sequence: `<s> X </s>`
- pair of sequences: `<s> A </s></s> B </s>`

#### get_special_tokens_mask[[transformers.LayoutXLMTokenizer.get_special_tokens_mask]]

```python
get_special_tokens_mask(token_ids_0: list[int], token_ids_1: list[int] | None = None, already_has_special_tokens: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_base.py#L1311)

**Parameters:**

token_ids_0 : List of IDs for the (possibly already formatted) sequence.

token_ids_1 : Unused when `already_has_special_tokens=True`. Must be None in that case.

already_has_special_tokens : Whether the sequence is already formatted with special tokens.

**Returns:** A list of integers in the range [0, 1]

1 for a special token, 0 for a sequence token.

Retrieve sequence ids from a token list that has no special tokens added.

For fast tokenizers, data collators call this with `already_has_special_tokens=True` to build a mask over an
already-formatted sequence. In that case, we compute the mask by checking membership in `all_special_ids`.

#### create_token_type_ids_from_sequences[[transformers.LayoutXLMTokenizer.create_token_type_ids_from_sequences]]

```python
create_token_type_ids_from_sequences(token_ids_0: list, token_ids_1: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/layoutxlm/tokenization_layoutxlm.py#L918)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

**Returns:** `list[int]`

List of zeros.

Create a mask from the two sequences passed to be used in a sequence-pair classification task. XLM-RoBERTa does
not make use of token type ids, therefore a list of zeros is returned.

#### save_vocabulary[[transformers.LayoutXLMTokenizer.save_vocabulary]]

```python
save_vocabulary(save_directory: str, filename_prefix: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_tokenizers.py#L509)

## LayoutXLMTokenizerFast[[transformers.LayoutXLMTokenizer]]

#### transformers.LayoutXLMTokenizer[[transformers.LayoutXLMTokenizer]]

```python
transformers.LayoutXLMTokenizer(vocab: str | list | None = None, bos_token = '<s>', eos_token = '</s>', sep_token = '</s>', cls_token = '<s>', unk_token = '<unk>', pad_token = '<pad>', mask_token = '<mask>', cls_token_box = [0, 0, 0, 0], sep_token_box = [1000, 1000, 1000, 1000], pad_token_box = [0, 0, 0, 0], pad_token_label = -100, only_label_first_subword = True, add_prefix_space = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/layoutxlm/tokenization_layoutxlm.py#L140)

**Parameters:**

vocab (`str`, `dict` or `list`, *optional*) : Vocabulary for the tokenizer as a path, a dictionary or a list of `(token, score)` tuples.

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.    When building a sequence using special tokens, this is not the token that is used for the beginning of sequence. The token used is the `cls_token`.   

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

sep_token (`str`, *optional*, defaults to `"</s>"`) : The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for sequence classification or for a text and a question for question answering. It is also used as the last token of a sequence built with special tokens.

cls_token (`str`, *optional*, defaults to `"<s>"`) : The classifier token which is used when doing sequence classification (classification of the whole sequence instead of per-token classification). It is the first token of the sequence when built with special tokens.

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

mask_token (`str`, *optional*, defaults to `"<mask>"`) : The token used for masking values. This is the token used when training this model with masked language modeling. This is the token which the model will try to predict.

cls_token_box (`list[int]`, *optional*, defaults to `[0, 0, 0, 0]`) : The bounding box to use for the special [CLS] token.

sep_token_box (`list[int]`, *optional*, defaults to `[1000, 1000, 1000, 1000]`) : The bounding box to use for the special [SEP] token.

pad_token_box (`list[int]`, *optional*, defaults to `[0, 0, 0, 0]`) : The bounding box to use for the special [PAD] token.

pad_token_label (`int`, *optional*, defaults to -100) : The label to use for padding tokens. Defaults to -100, which is the `ignore_index` of PyTorch's CrossEntropyLoss.

only_label_first_subword (`bool`, *optional*, defaults to `True`) : Whether or not to only label the first subword, in case word labels are provided.

add_prefix_space (`bool`, *optional*, defaults to `True`) : Whether or not to add an initial space to the input.

additional_special_tokens (`list[str]`, *optional*, defaults to `["<s>NOTUSED", "</s>NOTUSED"]`) : Additional special tokens used by the tokenizer.

Construct a "fast" LayoutXLM tokenizer (backed by HuggingFace's *tokenizers* library). Adapted from
[RobertaTokenizer](/docs/transformers/v5.15.1/en/model_doc/roberta#transformers.RobertaTokenizer) and [XLNetTokenizer](/docs/transformers/v5.15.1/en/model_doc/xlnet#transformers.XLNetTokenizer). Based on
[BPE](https://huggingface.co/docs/tokenizers/python/latest/components.html?highlight=BPE#models).

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

#### __call__[[transformers.LayoutXLMTokenizer.__call__]]

```python
__call__(text: str | list[str] | list[list[str]], text_pair: list[str] | list[list[str]] | None = None, boxes: list[list[int]] | list[list[list[int]]] | None = None, word_labels: list[int] | list[list[int]] | None = None, add_special_tokens: bool = True, padding: bool | str | transformers.utils.generic.PaddingStrategy = False, truncation: bool | str | transformers.tokenization_utils_base.TruncationStrategy = None, max_length: int | None = None, stride: int = 0, pad_to_multiple_of: int | None = None, padding_side: str | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, return_token_type_ids: bool | None = None, return_attention_mask: bool | None = None, return_overflowing_tokens: bool = False, return_special_tokens_mask: bool = False, return_offsets_mapping: bool = False, return_length: bool = False, verbose: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/layoutxlm/tokenization_layoutxlm.py#L413)

**Parameters:**

text (`str`, `list[str]`, `list[list[str]]`) : The sequence or batch of sequences to be encoded. Each sequence can be a string, a list of strings (words of a single example or questions of a batch of examples) or a list of list of strings (batch of words).

text_pair (`list[str]`, `list[list[str]]`) : The sequence or batch of sequences to be encoded. Each sequence should be a list of strings (pretokenized string).

boxes (`list[list[int]]`, `list[list[list[int]]]`) : Word-level bounding boxes. Each bounding box should be normalized to be on a 0-1000 scale.

word_labels (`list[int]`, `list[list[int]]`, *optional*) : Word-level integer labels (for token classification tasks such as FUNSD, CORD). 

add_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to encode the sequences with the special tokens relative to their model.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Activates and controls padding. Accepts the following values:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

truncation (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) : Activates and controls truncation. Accepts the following values:  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will truncate token by token, removing a token from the longest sequence in the pair if a pair of sequences (or a batch of pairs) is provided. - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths greater than the model maximum admissible input size).

max_length (`int`, *optional*) : Controls the maximum length to use by one of the truncation/padding parameters.  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length is required by one of the truncation/padding parameters. If the model has no specific maximum input length (like XLNet) truncation/padding to a maximum length will be deactivated.

stride (`int`, *optional*, defaults to 0) : If set to a number along with `max_length`, the overflowing tokens returned when `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence returned to provide some overlap between truncated and overflowing sequences. The value of this argument defines the number of overlapping tokens.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value. This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects.

return_token_type_ids (`bool`, *optional*) : Whether to return token type IDs. If left to the default, will return the token type IDs according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are token type IDs?](../glossary#token-type-ids)

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are attention masks?](../glossary#attention-mask)

return_overflowing_tokens (`bool`, *optional*, defaults to `False`) : Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead of returning overflowing tokens.

return_special_tokens_mask (`bool`, *optional*, defaults to `False`) : Whether or not to return special tokens mask information.

return_offsets_mapping (`bool`, *optional*, defaults to `False`) : Whether or not to return `(char_start, char_end)` for each token.  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend), if using Python's tokenizer, this method will raise `NotImplementedError`.

return_length  (`bool`, *optional*, defaults to `False`) : Whether or not to return the lengths of the encoded inputs.

verbose (`bool`, *optional*, defaults to `True`) : Whether or not to print more information and warnings.

- ****kwargs** : passed to the `self.tokenize()` method

**Returns:** [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding)

A [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields:

- **input_ids** -- List of token ids to be fed to a model.

  [What are input IDs?](../glossary#input-ids)

- **bbox** -- List of bounding boxes to be fed to a model.

- **token_type_ids** -- List of token type ids to be fed to a model (when `return_token_type_ids=True` or
  if *"token_type_ids"* is in `self.model_input_names`).

  [What are token type IDs?](../glossary#token-type-ids)

- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names`).

  [What are attention masks?](../glossary#attention-mask)

- **labels** -- List of labels to be fed to a model. (when `word_labels` is specified).
- **overflowing_tokens** -- List of overflowing tokens sequences (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **num_truncated_tokens** -- Number of tokens truncated (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **special_tokens_mask** -- List of 0s and 1s, with 1 specifying added special tokens and 0 specifying
  regular sequence tokens (when `add_special_tokens=True` and `return_special_tokens_mask=True`).
- **length** -- The length of the inputs (when `return_length=True`).

Main method to tokenize and prepare for the model one or several sequence(s) or one or several pair(s) of
sequences with word-level normalized bounding boxes and optional labels.

## LayoutXLMProcessor[[transformers.LayoutXLMProcessor]]

#### transformers.LayoutXLMProcessor[[transformers.LayoutXLMProcessor]]

```python
transformers.LayoutXLMProcessor(image_processor = None, tokenizer = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/layoutxlm/processing_layoutxlm.py#L24)

**Parameters:**

image_processor (`LayoutLMv2ImageProcessor`) : The image processor is a required input.

tokenizer (`LayoutXLMTokenizer`) : The tokenizer is a required input.

Constructs a LayoutXLMProcessor which wraps a image processor and a tokenizer into a single processor.

[LayoutXLMProcessor](/docs/transformers/v5.15.1/en/model_doc/layoutxlm#transformers.LayoutXLMProcessor) offers all the functionalities of [LayoutLMv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/layoutlmv2#transformers.LayoutLMv2ImageProcessor) and [LayoutXLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/layoutxlm#transformers.LayoutXLMTokenizer). See the
[~LayoutLMv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/layoutlmv2#transformers.LayoutLMv2ImageProcessor) and [~LayoutXLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/layoutxlm#transformers.LayoutXLMTokenizer) for more information.

#### __call__[[transformers.LayoutXLMProcessor.__call__]]

```python
__call__(images, text: str | list[str] | list[list[str]] = None, text_pair: list[str] | list[list[str]] | None = None, boxes: list[list[int]] | list[list[list[int]]] | None = None, word_labels: list[int] | list[list[int]] | None = None, add_special_tokens: bool = True, padding: bool | str | transformers.utils.generic.PaddingStrategy = False, truncation: bool | str | transformers.tokenization_utils_base.TruncationStrategy = None, max_length: int | None = None, stride: int = 0, pad_to_multiple_of: int | None = None, return_token_type_ids: bool | None = None, return_attention_mask: bool | None = None, return_overflowing_tokens: bool = False, return_special_tokens_mask: bool = False, return_offsets_mapping: bool = False, return_length: bool = False, verbose: bool = True, return_tensors: str | transformers.utils.generic.TensorType | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/layoutxlm/processing_layoutxlm.py#L28)

**Parameters:**

images (``) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

text_pair (`str, list[str] or list[int]`, *optional*) : Optional second sequence to be encoded. This can be a string, a list of strings (tokenized string using the `tokenize` method) or a list of integers (tokenized string ids using the `convert_tokens_to_ids` method).

boxes (`list[list[int]] or list[list[list[int]]]`, *optional*) : Word-level bounding boxes. Each bounding box should be normalized to be on a 0-1000 scale.

word_labels (`list[int] or list[list[int]]`, *optional*) : Word-level integer labels (for token classification tasks such as FUNSD, CORD).

add_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to add special tokens when encoding the sequences. This will use the underlying `PretrainedTokenizerBase.build_inputs_with_special_tokens` function, which defines which tokens are automatically added to the input ids. This is useful if you want to add `bos` or `eos` tokens automatically.

padding (bool, str or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Activates and controls padding. Accepts the following values:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence is provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

truncation (bool, str or [TruncationStrategy](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*) : Activates and controls truncation. Accepts the following values:  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will truncate token by token, removing a token from the longest sequence in the pair if a pair of sequences (or a batch of pairs) is provided. - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths greater than the model maximum admissible input size).

max_length (`int`, *optional*) : Controls the maximum length to use by one of the truncation/padding parameters.  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length is required by one of the truncation/padding parameters. If the model has no specific maximum input length (like XLNet) truncation/padding to a maximum length will be deactivated.

stride (`int`, *optional*, defaults to `0`) : If set to a number along with `max_length`, the overflowing tokens returned when `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence returned to provide some overlap between truncated and overflowing sequences. The value of this argument defines the number of overlapping tokens.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value. Requires `padding` to be activated. This is especially useful to enable using Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

return_token_type_ids (`bool`, *optional*) : Whether to return token type IDs. If left to the default, will return the token type IDs according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are token type IDs?](../glossary#token-type-ids)

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are attention masks?](../glossary#attention-mask)

return_overflowing_tokens (`bool`, *optional*, defaults to `False`) : Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead of returning overflowing tokens.

return_special_tokens_mask (`bool`, *optional*, defaults to `False`) : Whether or not to return special tokens mask information.

return_offsets_mapping (`bool`, *optional*, defaults to `False`) : Whether or not to return `(char_start, char_end)` for each token.  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend), if using Python's tokenizer, this method will raise `NotImplementedError`.

return_length (`bool`, *optional*, defaults to `False`) : Whether or not to return the lengths of the encoded inputs.

verbose (`bool`, *optional*, defaults to `True`) : Whether or not to print more information and warnings.

return_tensors (`Union[str, ~utils.generic.TensorType]`, *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

**Returns:** `~tokenization_utils_base.BatchEncoding`

- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the `__call__`/`encode_plus`/`batch_encode_plus` methods
  ('input_ids', 'attention_mask', etc.).
- **encoding** (`tokenizers.Encoding` or `Sequence[tokenizers.Encoding]`, *optional*) -- If the tokenizer is a fast tokenizer which outputs additional information like mapping from word/character
  space to token space the `tokenizers.Encoding` instance or list of instance (for batches) hold this
  information.
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **prepend_batch_axis** (`bool`, *optional*, defaults to `False`) -- Whether or not to add a batch axis when converting to tensors (see `tensor_type` above). Note that this
  parameter has an effect if the parameter `tensor_type` is set, *otherwise has no effect*.
- **n_sequences** (`int`, *optional*) -- The number of input sequences represented by each encoding (`None` for unknown, `1` for a single sequence and `2` for a pair of sequences).
