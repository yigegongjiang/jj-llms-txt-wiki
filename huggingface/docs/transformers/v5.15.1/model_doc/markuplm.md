# MarkupLM

## Overview

The MarkupLM model was proposed in [MarkupLM: Pre-training of Text and Markup Language for Visually-rich Document
Understanding](https://huggingface.co/papers/2110.08518) by Junlong Li, Yiheng Xu, Lei Cui, Furu Wei. MarkupLM is BERT, but
applied to HTML pages instead of raw text documents. The model incorporates additional embedding layers to improve
performance, similar to [LayoutLM](layoutlm).

The model can be used for tasks like question answering on web pages or information extraction from web pages. It obtains
state-of-the-art results on 2 important benchmarks:

- [WebSRC](https://x-lance.github.io/WebSRC/), a dataset for Web-Based Structural Reading Comprehension (a bit like SQuAD but for web pages)
- [SWDE](https://www.researchgate.net/publication/221299838_From_one_tree_to_a_forest_a_unified_solution_for_structured_web_data_extraction), a dataset
for information extraction from web pages (basically named-entity recognition on web pages)

The abstract from the paper is the following:

*Multimodal pre-training with text, layout, and image has made significant progress for Visually-rich Document
Understanding (VrDU), especially the fixed-layout documents such as scanned document images. While, there are still a
large number of digital documents where the layout information is not fixed and needs to be interactively and
dynamically rendered for visualization, making existing layout-based pre-training approaches not easy to apply. In this
paper, we propose MarkupLM for document understanding tasks with markup languages as the backbone such as
HTML/XML-based documents, where text and markup information is jointly pre-trained. Experiment results show that the
pre-trained MarkupLM significantly outperforms the existing strong baseline models on several document understanding
tasks. The pre-trained model and code will be publicly available.*

This model was contributed by [nielsr](https://huggingface.co/nielsr). The original code can be found [here](https://github.com/microsoft/unilm/tree/master/markuplm).

## Usage tips

- In addition to `input_ids`, [forward()](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMModel.forward) expects 2 additional inputs, namely `xpath_tags_seq` and `xpath_subs_seq`.
These are the XPATH tags and subscripts respectively for each token in the input sequence.
- One can use [MarkupLMProcessor](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMProcessor) to prepare all data for the model. Refer to the [usage guide](#usage-markuplmprocessor) for more info.

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/markuplm_architecture.jpg"
alt="drawing" width="600"/>

 MarkupLM architecture. Taken from the original paper. 

## Usage: MarkupLMProcessor

The easiest way to prepare data for the model is to use [MarkupLMProcessor](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMProcessor), which internally combines a feature extractor
([MarkupLMFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMFeatureExtractor)) and a tokenizer ([MarkupLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMTokenizer) or [MarkupLMTokenizerFast](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMTokenizer)). The feature extractor is
used to extract all nodes and xpaths from the HTML strings, which are then provided to the tokenizer, which turns them into the
token-level inputs of the model (`input_ids` etc.). Note that you can still use the feature extractor and tokenizer separately,
if you only want to handle one of the two tasks.

```python
from transformers import MarkupLMFeatureExtractor, MarkupLMProcessor, MarkupLMTokenizerFast

feature_extractor = MarkupLMFeatureExtractor()
tokenizer = MarkupLMTokenizerFast.from_pretrained("microsoft/markuplm-base")
processor = MarkupLMProcessor(feature_extractor, tokenizer)
```

In short, one can provide HTML strings (and possibly additional data) to [MarkupLMProcessor](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMProcessor),
and it will create the inputs expected by the model. Internally, the processor first uses
[MarkupLMFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMFeatureExtractor) to get a list of nodes and corresponding xpaths. The nodes and
xpaths are then provided to [MarkupLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMTokenizer) or [MarkupLMTokenizerFast](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMTokenizer), which converts them
to token-level `input_ids`, `attention_mask`, `token_type_ids`, `xpath_subs_seq`, `xpath_tags_seq`.
Optionally, one can provide node labels to the processor, which are turned into token-level `labels`.

[MarkupLMFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMFeatureExtractor) uses [Beautiful Soup](https://www.crummy.com/software/BeautifulSoup/bs4/doc/), a Python library for
pulling data out of HTML and XML files, under the hood. Note that you can still use your own parsing solution of
choice, and provide the nodes and xpaths yourself to [MarkupLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMTokenizer) or [MarkupLMTokenizerFast](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMTokenizer).

In total, there are 5 use cases that are supported by the processor. Below, we list them all. Note that each of these
use cases work for both batched and non-batched inputs (we illustrate them for non-batched inputs).

**Use case 1: web page classification (training, inference) + token classification (inference), parse_html = True**

This is the simplest case, in which the processor will use the feature extractor to get all nodes and xpaths from the HTML.

```python
from transformers import MarkupLMProcessor

processor = MarkupLMProcessor.from_pretrained("microsoft/markuplm-base")

html_string = """
 <!DOCTYPE html>
 <html>
 <head>
 <title>Hello world</title>
 </head>
 <body>
 <h1>Welcome</h1>
 <p>Here is my website.</p>
 </body>
 </html>"""

# note that you can also add provide all tokenizer parameters here such as padding, truncation
encoding = processor(html_string, return_tensors="pt").to(model.device)
print(encoding.keys())
dict_keys(['input_ids', 'token_type_ids', 'attention_mask', 'xpath_tags_seq', 'xpath_subs_seq'])
```

**Use case 2: web page classification (training, inference) + token classification (inference), parse_html=False**

In case one already has obtained all nodes and xpaths, one doesn't need the feature extractor. In that case, one should
provide the nodes and corresponding xpaths themselves to the processor, and make sure to set `parse_html` to `False`.

```python
from transformers import MarkupLMProcessor

processor = MarkupLMProcessor.from_pretrained("microsoft/markuplm-base")
processor.parse_html = False

nodes = ["hello", "world", "how", "are"]
xpaths = ["/html/body/div/li[1]/div/span", "/html/body/div/li[1]/div/span", "html/body", "html/body/div"]
encoding = processor(nodes=nodes, xpaths=xpaths, return_tensors="pt").to(model.device)
print(encoding.keys())
dict_keys(['input_ids', 'token_type_ids', 'attention_mask', 'xpath_tags_seq', 'xpath_subs_seq'])
```

**Use case 3: token classification (training), parse_html=False**

For token classification tasks (such as [SWDE](https://paperswithcode.com/dataset/swde)), one can also provide the
corresponding node labels in order to train a model. The processor will then convert these into token-level `labels`.
By default, it will only label the first wordpiece of a word, and label the remaining wordpieces with -100, which is the
`ignore_index` of PyTorch's CrossEntropyLoss. In case you want all wordpieces of a word to be labeled, you can
initialize the tokenizer with `only_label_first_subword` set to `False`.

```python
from transformers import MarkupLMProcessor

processor = MarkupLMProcessor.from_pretrained("microsoft/markuplm-base")
processor.parse_html = False

nodes = ["hello", "world", "how", "are"]
xpaths = ["/html/body/div/li[1]/div/span", "/html/body/div/li[1]/div/span", "html/body", "html/body/div"]
node_labels = [1, 2, 2, 1]
encoding = processor(nodes=nodes, xpaths=xpaths, node_labels=node_labels, return_tensors="pt").to(model.device)
print(encoding.keys())
dict_keys(['input_ids', 'token_type_ids', 'attention_mask', 'xpath_tags_seq', 'xpath_subs_seq', 'labels'])
```

**Use case 4: web page question answering (inference), parse_html=True**

For question answering tasks on web pages, you can provide a question to the processor. By default, the
processor will use the feature extractor to get all nodes and xpaths, and create [CLS] question tokens [SEP] word tokens [SEP].

```python
from transformers import MarkupLMProcessor

processor = MarkupLMProcessor.from_pretrained("microsoft/markuplm-base")

html_string = """
 <!DOCTYPE html>
 <html>
 <head>
 <title>Hello world</title>
 </head>
 <body>
 <h1>Welcome</h1>
 <p>My name is Niels.</p>
 </body>
 </html>"""

question = "What's his name?"
encoding = processor(html_string, questions=question, return_tensors="pt").to(model.device)
print(encoding.keys())
dict_keys(['input_ids', 'token_type_ids', 'attention_mask', 'xpath_tags_seq', 'xpath_subs_seq'])
```

**Use case 5: web page question answering (inference), parse_html=False**

For question answering tasks (such as WebSRC), you can provide a question to the processor. If you have extracted
all nodes and xpaths yourself, you can provide them directly to the processor. Make sure to set `parse_html` to `False`.

```python
from transformers import MarkupLMProcessor

processor = MarkupLMProcessor.from_pretrained("microsoft/markuplm-base")
processor.parse_html = False

nodes = ["hello", "world", "how", "are"]
xpaths = ["/html/body/div/li[1]/div/span", "/html/body/div/li[1]/div/span", "html/body", "html/body/div"]
question = "What's his name?"
encoding = processor(nodes=nodes, xpaths=xpaths, questions=question, return_tensors="pt").to(model.device)
print(encoding.keys())
dict_keys(['input_ids', 'token_type_ids', 'attention_mask', 'xpath_tags_seq', 'xpath_subs_seq'])
```

## Resources

- [Demo notebooks](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/MarkupLM)
- [Text classification task guide](../tasks/sequence_classification)
- [Token classification task guide](../tasks/token_classification)
- [Question answering task guide](../tasks/question_answering)

## MarkupLMConfig[[transformers.MarkupLMConfig]]

#### transformers.MarkupLMConfig[[transformers.MarkupLMConfig]]

```python
transformers.MarkupLMConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 30522, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, intermediate_size: int = 3072, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.1, attention_probs_dropout_prob: float | int = 0.1, max_position_embeddings: int = 512, type_vocab_size: int = 2, initializer_range: float = 0.02, layer_norm_eps: float = 1e-12, pad_token_id: int | None = 0, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 2, max_xpath_tag_unit_embeddings: int = 256, max_xpath_subs_unit_embeddings: int = 1024, tag_pad_id: int = 216, subs_pad_id: int = 1001, xpath_unit_hidden_size: int = 32, max_depth: int = 50, use_cache: bool = True, classifier_dropout: float | int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/configuration_markuplm.py#L24)

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

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

max_xpath_tag_unit_embeddings (`int`, *optional*, defaults to 256) : The maximum value that the xpath tag unit embedding might ever use. Typically set this to something large just in case (e.g., 256).

max_xpath_subs_unit_embeddings (`int`, *optional*, defaults to 1024) : The maximum value that the xpath subscript unit embedding might ever use. Typically set this to something large just in case (e.g., 1024).

tag_pad_id (`int`, *optional*, defaults to 216) : The id of the padding token in the xpath tags.

subs_pad_id (`int`, *optional*, defaults to 1001) : The id of the padding token in the xpath subscripts.

xpath_unit_hidden_size (`int`, *optional*, defaults to 32) : The hidden size of each unit in xpath.

max_depth (`int`, *optional*, defaults to 50) : The maximum depth in xpath.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

classifier_dropout (`Union[float, int]`, *optional*) : The dropout ratio for classifier.

This is the configuration class to store the configuration of a MarkupLMModel. It is used to instantiate a Markuplm
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/markuplm-base](https://huggingface.co/microsoft/markuplm-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import MarkupLMModel, MarkupLMConfig

>>> # Initializing a MarkupLM microsoft/markuplm-base style configuration
>>> configuration = MarkupLMConfig()

>>> # Initializing a model from the microsoft/markuplm-base style configuration
>>> model = MarkupLMModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MarkupLMFeatureExtractor[[transformers.MarkupLMFeatureExtractor]]

#### transformers.MarkupLMFeatureExtractor[[transformers.MarkupLMFeatureExtractor]]

```python
transformers.MarkupLMFeatureExtractor(**kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/feature_extraction_markuplm.py#L32)

Constructs a MarkupLM feature extractor. This can be used to get a list of nodes and corresponding xpaths from HTML
strings.

This feature extractor inherits from `PreTrainedFeatureExtractor()` which contains most
of the main methods. Users should refer to this superclass for more information regarding those methods.

#### __call__[[transformers.MarkupLMFeatureExtractor.__call__]]

```python
__call__(html_strings)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/feature_extraction_markuplm.py#L98)

**Parameters:**

html_strings (`str`, `list[str]`) : The HTML string or batch of HTML strings from which to extract nodes and corresponding xpaths.

**Returns:** [BatchFeature](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.BatchFeature)

A [BatchFeature](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:

- **nodes** -- Nodes.
- **xpaths** -- Corresponding xpaths.

Main method to prepare for the model one or several HTML strings.

Examples:

```python
>>> from transformers import MarkupLMFeatureExtractor

>>> page_name_1 = "page1.html"
>>> page_name_2 = "page2.html"
>>> page_name_3 = "page3.html"

>>> with open(page_name_1) as f:
...     single_html_string = f.read()

>>> feature_extractor = MarkupLMFeatureExtractor()

>>> # single example
>>> encoding = feature_extractor(single_html_string)
>>> print(encoding.keys())
>>> # dict_keys(['nodes', 'xpaths'])

>>> # batched example

>>> multi_html_strings = []

>>> with open(page_name_2) as f:
...     multi_html_strings.append(f.read())
>>> with open(page_name_3) as f:
...     multi_html_strings.append(f.read())

>>> encoding = feature_extractor(multi_html_strings)
>>> print(encoding.keys())
>>> # dict_keys(['nodes', 'xpaths'])
```

## MarkupLMTokenizer[[transformers.MarkupLMTokenizer]]

#### transformers.MarkupLMTokenizer[[transformers.MarkupLMTokenizer]]

```python
transformers.MarkupLMTokenizer(tags_dict, vocab: str | dict[str, int] | list[tuple[str, float]] | None = None, merges: str | list[str] | None = None, errors = 'replace', bos_token = '<s>', eos_token = '</s>', sep_token = '</s>', cls_token = '<s>', unk_token = '<unk>', pad_token = '<pad>', mask_token = '<mask>', add_prefix_space = False, max_depth = 50, max_width = 1000, pad_width = 1001, pad_token_label = -100, only_label_first_subword = True, trim_offsets = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/tokenization_markuplm.py#L91)

**Parameters:**

vocab (`str` or `dict[str, int]`, *optional*) : Custom vocabulary dictionary. If not provided, the vocabulary is loaded from `vocab_file`.

merges (`str` or `list[str]`, *optional*) : Custom merges list. If not provided, merges are loaded from `merges_file`.

errors (`str`, *optional*, defaults to `"replace"`) : Paradigm to follow when decoding bytes to UTF-8. See [bytes.decode](https://docs.python.org/3/library/stdtypes.html#bytes.decode) for more information.

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.    When building a sequence using special tokens, this is not the token that is used for the beginning of sequence. The token used is the `cls_token`.   

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

sep_token (`str`, *optional*, defaults to `"</s>"`) : The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for sequence classification or for a text and a question for question answering. It is also used as the last token of a sequence built with special tokens.

cls_token (`str`, *optional*, defaults to `"<s>"`) : The classifier token which is used when doing sequence classification (classification of the whole sequence instead of per-token classification). It is the first token of the sequence when built with special tokens.

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

mask_token (`str`, *optional*, defaults to `"<mask>"`) : The token used for masking values. This is the token used when training this model with masked language modeling. This is the token which the model will try to predict.

add_prefix_space (`bool`, *optional*, defaults to `False`) : Whether or not to add an initial space to the input. This allows to treat the leading word just as any other word. (RoBERTa tokenizer detect beginning of words by the preceding space).

Construct a MarkupLM tokenizer. Based on byte-level Byte-Pair-Encoding (BPE).

[MarkupLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMTokenizer) can be used to turn HTML strings into to token-level `input_ids`, `attention_mask`,
`token_type_ids`, `xpath_tags_seq` and `xpath_tags_seq`. This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which
contains most of the main methods and ensures a `tokenizers` backend is always instantiated.

Users should refer to this superclass for more information regarding those methods.

#### build_inputs_with_special_tokens[[transformers.MarkupLMTokenizer.build_inputs_with_special_tokens]]

```python
build_inputs_with_special_tokens(token_ids_0: list, token_ids_1: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/tokenization_markuplm.py#L952)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs to which the special tokens will be added.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

**Returns:** `list[int]`

List of [input IDs](../glossary#input-ids) with the appropriate special tokens.

Build model inputs from a sequence or a pair of sequence for sequence classification tasks by concatenating and
adding special tokens. A RoBERTa sequence has the following format:
- single sequence: `<s> X </s>`
- pair of sequences: `<s> A </s></s> B </s>`

#### get_special_tokens_mask[[transformers.MarkupLMTokenizer.get_special_tokens_mask]]

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

#### create_token_type_ids_from_sequences[[transformers.MarkupLMTokenizer.create_token_type_ids_from_sequences]]

```python
create_token_type_ids_from_sequences(token_ids_0: list, token_ids_1: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/tokenization_markuplm.py#L975)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

**Returns:** `list[int]`

List of zeros.

Create a mask from the two sequences passed to be used in a sequence-pair classification task. RoBERTa does not
make use of token type ids, therefore a list of zeros is returned.

#### save_vocabulary[[transformers.MarkupLMTokenizer.save_vocabulary]]

```python
save_vocabulary(save_directory: str, filename_prefix: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/tokenization_markuplm.py#L997)

## MarkupLMTokenizerFast[[transformers.MarkupLMTokenizer]]

#### transformers.MarkupLMTokenizer[[transformers.MarkupLMTokenizer]]

```python
transformers.MarkupLMTokenizer(tags_dict, vocab: str | dict[str, int] | list[tuple[str, float]] | None = None, merges: str | list[str] | None = None, errors = 'replace', bos_token = '<s>', eos_token = '</s>', sep_token = '</s>', cls_token = '<s>', unk_token = '<unk>', pad_token = '<pad>', mask_token = '<mask>', add_prefix_space = False, max_depth = 50, max_width = 1000, pad_width = 1001, pad_token_label = -100, only_label_first_subword = True, trim_offsets = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/tokenization_markuplm.py#L91)

**Parameters:**

vocab (`str` or `dict[str, int]`, *optional*) : Custom vocabulary dictionary. If not provided, the vocabulary is loaded from `vocab_file`.

merges (`str` or `list[str]`, *optional*) : Custom merges list. If not provided, merges are loaded from `merges_file`.

errors (`str`, *optional*, defaults to `"replace"`) : Paradigm to follow when decoding bytes to UTF-8. See [bytes.decode](https://docs.python.org/3/library/stdtypes.html#bytes.decode) for more information.

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.    When building a sequence using special tokens, this is not the token that is used for the beginning of sequence. The token used is the `cls_token`.   

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

sep_token (`str`, *optional*, defaults to `"</s>"`) : The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for sequence classification or for a text and a question for question answering. It is also used as the last token of a sequence built with special tokens.

cls_token (`str`, *optional*, defaults to `"<s>"`) : The classifier token which is used when doing sequence classification (classification of the whole sequence instead of per-token classification). It is the first token of the sequence when built with special tokens.

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

mask_token (`str`, *optional*, defaults to `"<mask>"`) : The token used for masking values. This is the token used when training this model with masked language modeling. This is the token which the model will try to predict.

add_prefix_space (`bool`, *optional*, defaults to `False`) : Whether or not to add an initial space to the input. This allows to treat the leading word just as any other word. (RoBERTa tokenizer detect beginning of words by the preceding space).

Construct a MarkupLM tokenizer. Based on byte-level Byte-Pair-Encoding (BPE).

[MarkupLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMTokenizer) can be used to turn HTML strings into to token-level `input_ids`, `attention_mask`,
`token_type_ids`, `xpath_tags_seq` and `xpath_tags_seq`. This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) which
contains most of the main methods and ensures a `tokenizers` backend is always instantiated.

Users should refer to this superclass for more information regarding those methods.

#### batch_encode_plus[[transformers.MarkupLMTokenizer.batch_encode_plus]]

```python
batch_encode_plus(batch_text_or_text_pairs: list[str] | list[tuple[str, str]] | list[list[str]], is_pair: bool | None = None, xpaths: list[list[list[int]]] | None = None, node_labels: list[int] | list[list[int]] | None = None, add_special_tokens: bool = True, padding: bool | str | transformers.utils.generic.PaddingStrategy = False, truncation: bool | str | transformers.tokenization_utils_base.TruncationStrategy = None, max_length: int | None = None, stride: int = 0, pad_to_multiple_of: int | None = None, padding_side: str | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, return_token_type_ids: bool | None = None, return_attention_mask: bool | None = None, return_overflowing_tokens: bool = False, return_special_tokens_mask: bool = False, return_offsets_mapping: bool = False, return_length: bool = False, verbose: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/tokenization_markuplm.py#L453)

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

add_special_tokens (`bool`, *optional*, defaults to `True`):
Whether or not to encode the sequences with the special tokens relative to their model.
padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`):
Activates and controls padding. Accepts the following values:

- `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single
  sequence if provided).
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
Controls the maximum length to use by one of the truncation/padding parameters. If left unset or set to
`None`, this will use the predefined model maximum length if a maximum length is required by one of the
truncation/padding parameters. If the model has no specific maximum input length (like XLNet)
truncation/padding to a maximum length will be deactivated.
stride (`int`, *optional*, defaults to 0):
If set to a number along with `max_length`, the overflowing tokens returned when
`return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence
returned to provide some overlap between truncated and overflowing sequences. The value of this
argument defines the number of overlapping tokens.
is_split_into_words (`bool`, *optional*, defaults to `False`):
Whether or not the input is already pretokenized (e.g. split into words). Set this to `True` if you are
passing pretokenized inputs to avoid additional tokenization.
pad_to_multiple_of (`int`, *optional*):
If set will pad the sequence to a multiple of the provided value. This is especially useful to enable
the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).
return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*):
If set, will return tensors instead of list of python integers. Acceptable values are:

- `'pt'`: Return PyTorch `torch.Tensor` objects.
- `'np'`: Return Numpy `np.ndarray` objects.

#### build_inputs_with_special_tokens[[transformers.MarkupLMTokenizer.build_inputs_with_special_tokens]]

```python
build_inputs_with_special_tokens(token_ids_0: list, token_ids_1: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/tokenization_markuplm.py#L952)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs to which the special tokens will be added.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

**Returns:** `list[int]`

List of [input IDs](../glossary#input-ids) with the appropriate special tokens.

Build model inputs from a sequence or a pair of sequence for sequence classification tasks by concatenating and
adding special tokens. A RoBERTa sequence has the following format:
- single sequence: `<s> X </s>`
- pair of sequences: `<s> A </s></s> B </s>`

#### create_token_type_ids_from_sequences[[transformers.MarkupLMTokenizer.create_token_type_ids_from_sequences]]

```python
create_token_type_ids_from_sequences(token_ids_0: list, token_ids_1: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/tokenization_markuplm.py#L975)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

**Returns:** `list[int]`

List of zeros.

Create a mask from the two sequences passed to be used in a sequence-pair classification task. RoBERTa does not
make use of token type ids, therefore a list of zeros is returned.

#### encode_plus[[transformers.MarkupLMTokenizer.encode_plus]]

```python
encode_plus(text: str | list[str], text_pair: list[str] | None = None, xpaths: list[list[int]] | None = None, node_labels: list[int] | None = None, add_special_tokens: bool = True, padding: bool | str | transformers.utils.generic.PaddingStrategy = False, truncation: bool | str | transformers.tokenization_utils_base.TruncationStrategy = None, max_length: int | None = None, stride: int = 0, pad_to_multiple_of: int | None = None, padding_side: str | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, return_token_type_ids: bool | None = None, return_attention_mask: bool | None = None, return_overflowing_tokens: bool = False, return_special_tokens_mask: bool = False, return_offsets_mapping: bool = False, return_length: bool = False, verbose: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/tokenization_markuplm.py#L518)

**Parameters:**

text (`str`, `list[str]`, `list[list[str]]`) : The first sequence to be encoded. This can be a string, a list of strings or a list of list of strings.

text_pair (`list[str]` or `list[int]`, *optional*) : Optional second sequence to be encoded. This can be a list of strings (words of a single example) or a list of list of strings (words of a batch of examples). 

add_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to add special tokens when encoding the sequences. This will use the underlying `PretrainedTokenizerBase.build_inputs_with_special_tokens` function, which defines which tokens are automatically added to the input ids. This is useful if you want to add `bos` or `eos` tokens automatically.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Activates and controls padding. Accepts the following values:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence is provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

truncation (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) : Activates and controls truncation. Accepts the following values:  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will truncate token by token, removing a token from the longest sequence in the pair if a pair of sequences (or a batch of pairs) is provided. - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths greater than the model maximum admissible input size).

max_length (`int`, *optional*) : Controls the maximum length to use by one of the truncation/padding parameters.  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length is required by one of the truncation/padding parameters. If the model has no specific maximum input length (like XLNet) truncation/padding to a maximum length will be deactivated.

stride (`int`, *optional*, defaults to 0) : If set to a number along with `max_length`, the overflowing tokens returned when `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence returned to provide some overlap between truncated and overflowing sequences. The value of this argument defines the number of overlapping tokens.

is_split_into_words (`bool`, *optional*, defaults to `False`) : Whether or not the input is already pre-tokenized (e.g., split into words). If set to `True`, the tokenizer assumes the input is already split into words (for instance, by splitting it on whitespace) which it will tokenize. This is useful for NER or token classification.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value. Requires `padding` to be activated. This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

padding_side (`str`, *optional*) : The side on which the model should have padding applied. Should be selected between ['right', 'left']. Default value is picked from the class attribute of the same name.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects. 

add_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to encode the sequences with the special tokens relative to their model.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Activates and controls padding. Accepts the following values:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

truncation (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) : Activates and controls truncation. Accepts the following values:  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will truncate token by token, removing a token from the longest sequence in the pair if a pair of sequences (or a batch of pairs) is provided. - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths greater than the model maximum admissible input size).

max_length (`int`, *optional*) : Controls the maximum length to use by one of the truncation/padding parameters. If left unset or set to `None`, this will use the predefined model maximum length if a maximum length is required by one of the truncation/padding parameters. If the model has no specific maximum input length (like XLNet) truncation/padding to a maximum length will be deactivated.

stride (`int`, *optional*, defaults to 0) : If set to a number along with `max_length`, the overflowing tokens returned when `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence returned to provide some overlap between truncated and overflowing sequences. The value of this argument defines the number of overlapping tokens.

is_split_into_words (`bool`, *optional*, defaults to `False`) : Whether or not the input is already pretokenized (e.g. split into words). Set this to `True` if you are passing pretokenized inputs to avoid additional tokenization.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value. This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects.

Tokenize and prepare for the model a sequence or a pair of sequences. .. warning:: This method is deprecated,
`__call__` should be used instead.

#### get_xpath_seq[[transformers.MarkupLMTokenizer.get_xpath_seq]]

```python
get_xpath_seq(xpath)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/tokenization_markuplm.py#L253)

Given the xpath expression of one particular node (like "/html/body/div/li[1]/div/span[2]"), return a list of
tag IDs and corresponding subscripts, taking into account max depth.

## MarkupLMProcessor[[transformers.MarkupLMProcessor]]

#### transformers.MarkupLMProcessor[[transformers.MarkupLMProcessor]]

```python
transformers.MarkupLMProcessor(feature_extractor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/processing_markuplm.py#L25)

**Parameters:**

feature_extractor (`feature_extractor_class`) : The feature extractor is a required input.

tokenizer (`MarkupLMTokenizer`) : The tokenizer is a required input.

Constructs a MarkupLMProcessor which wraps a feature extractor and a tokenizer into a single processor.

[MarkupLMProcessor](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMProcessor) offers all the functionalities of `feature_extractor_class` and [MarkupLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMTokenizer). See the
`~feature_extractor_class` and [~MarkupLMTokenizer](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMTokenizer) for more information.

#### __call__[[transformers.MarkupLMProcessor.__call__]]

```python
__call__(html_strings = None, nodes = None, xpaths = None, node_labels = None, questions = None, add_special_tokens: bool = True, padding: bool | str | transformers.utils.generic.PaddingStrategy = False, truncation: bool | str | transformers.tokenization_utils_base.TruncationStrategy = None, max_length: int | None = None, stride: int = 0, pad_to_multiple_of: int | None = None, return_token_type_ids: bool | None = None, return_attention_mask: bool | None = None, return_overflowing_tokens: bool = False, return_special_tokens_mask: bool = False, return_offsets_mapping: bool = False, return_length: bool = False, verbose: bool = True, return_tensors: str | transformers.utils.generic.TensorType | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/processing_markuplm.py#L31)

**Parameters:**

html_strings (`str` or `list[str]`, *optional*) : Raw HTML strings to parse and process. When `parse_html=True` (default), these strings are parsed to extract nodes and xpaths automatically. If provided, `nodes`, `xpaths`, and `node_labels` should not be provided. Required when `parse_html=True`.

nodes (`list[list[str]]`, *optional*) : Pre-extracted HTML nodes as a list of lists, where each inner list contains the text content of nodes for a single document. Required when `parse_html=False`. Should not be provided when `parse_html=True`.

xpaths (`list[list[str]]`, *optional*) : Pre-extracted XPath expressions corresponding to the nodes. Should be a list of lists with the same structure as `nodes`, where each XPath identifies the location of the corresponding node in the HTML tree. Required when `parse_html=False`. Should not be provided when `parse_html=True`.

node_labels (`list[list[int]]`, *optional*) : Labels for the nodes, typically used for training or fine-tuning tasks. Should be a list of lists with the same structure as `nodes`, where each label corresponds to a node. Optional and only used when `parse_html=False`.

questions (`str` or `list[str]`, *optional*) : Question strings for question-answering tasks. When provided, the tokenizer processes questions as the first sequence and nodes as the second sequence (text_pair). If a single string is provided, it is converted to a list to match the batch dimension of the parsed HTML.

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

## MarkupLMModel[[transformers.MarkupLMModel]]

#### transformers.MarkupLMModel[[transformers.MarkupLMModel]]

```python
transformers.MarkupLMModel(config, add_pooling_layer = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/modeling_markuplm.py#L497)

**Parameters:**

config ([MarkupLMModel](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether to add a pooling layer

The bare Markuplm Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MarkupLMModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, xpath_tags_seq: typing.Optional[torch.LongTensor] = None, xpath_subs_seq: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.FloatTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/modeling_markuplm.py#L521)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

xpath_tags_seq (`torch.LongTensor` of shape `(batch_size, sequence_length, config.max_depth)`, *optional*) : Tag IDs for each token in the input sequence, padded up to config.max_depth.

xpath_subs_seq (`torch.LongTensor` of shape `(batch_size, sequence_length, config.max_depth)`, *optional*) : Subscript IDs for each token in the input sequence, padded up to config.max_depth.

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MarkupLMConfig](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMConfig)) and inputs.

The [MarkupLMModel](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMModel) forward method, overrides the `__call__` special method.

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

Examples:

```python
>>> from transformers import AutoProcessor, MarkupLMModel

>>> processor = AutoProcessor.from_pretrained("microsoft/markuplm-base")
>>> model = MarkupLMModel.from_pretrained("microsoft/markuplm-base")

>>> html_string = "<html> <head> <title>Page Title</title> </head> </html>"

>>> encoding = processor(html_string, return_tensors="pt")

>>> outputs = model(**encoding)
>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 4, 768]
```

## MarkupLMForSequenceClassification[[transformers.MarkupLMForSequenceClassification]]

#### transformers.MarkupLMForSequenceClassification[[transformers.MarkupLMForSequenceClassification]]

```python
transformers.MarkupLMForSequenceClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/modeling_markuplm.py#L804)

**Parameters:**

config ([MarkupLMForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMForSequenceClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MarkupLM Model transformer with a sequence classification/regression head on top (a linear layer on top of the
pooled output) e.g. for GLUE tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MarkupLMForSequenceClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, xpath_tags_seq: typing.Optional[torch.Tensor] = None, xpath_subs_seq: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/modeling_markuplm.py#L821)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

xpath_tags_seq (`torch.LongTensor` of shape `(batch_size, sequence_length, config.max_depth)`, *optional*) : Tag IDs for each token in the input sequence, padded up to config.max_depth.

xpath_subs_seq (`torch.LongTensor` of shape `(batch_size, sequence_length, config.max_depth)`, *optional*) : Subscript IDs for each token in the input sequence, padded up to config.max_depth.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

**Returns:** [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`

A [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MarkupLMConfig](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMConfig)) and inputs.

The [MarkupLMForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMForSequenceClassification) forward method, overrides the `__call__` special method.

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

Examples:

```python
>>> from transformers import AutoProcessor, AutoModelForSequenceClassification
>>> import torch

>>> processor = AutoProcessor.from_pretrained("microsoft/markuplm-base")
>>> model = AutoModelForSequenceClassification.from_pretrained("microsoft/markuplm-base", num_labels=7)

>>> html_string = "<html> <head> <title>Page Title</title> </head> </html>"
>>> encoding = processor(html_string, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**encoding)

>>> loss = outputs.loss
>>> logits = outputs.logits
```

## MarkupLMForTokenClassification[[transformers.MarkupLMForTokenClassification]]

#### transformers.MarkupLMForTokenClassification[[transformers.MarkupLMForTokenClassification]]

```python
transformers.MarkupLMForTokenClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/modeling_markuplm.py#L709)

**Parameters:**

config ([MarkupLMForTokenClassification](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMForTokenClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MarkupLM Model with a `token_classification` head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MarkupLMForTokenClassification.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, xpath_tags_seq: typing.Optional[torch.Tensor] = None, xpath_subs_seq: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/modeling_markuplm.py#L725)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

xpath_tags_seq (`torch.LongTensor` of shape `(batch_size, sequence_length, config.max_depth)`, *optional*) : Tag IDs for each token in the input sequence, padded up to config.max_depth.

xpath_subs_seq (`torch.LongTensor` of shape `(batch_size, sequence_length, config.max_depth)`, *optional*) : Subscript IDs for each token in the input sequence, padded up to config.max_depth.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the token classification loss. Indices should be in `[0, ..., config.num_labels - 1]`.

**Returns:** [MaskedLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or `tuple(torch.FloatTensor)`

A [MaskedLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MarkupLMConfig](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMConfig)) and inputs.

The [MarkupLMForTokenClassification](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMForTokenClassification) forward method, overrides the `__call__` special method.

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

Examples:

```python
>>> from transformers import AutoProcessor, AutoModelForTokenClassification
>>> import torch

>>> processor = AutoProcessor.from_pretrained("microsoft/markuplm-base")
>>> processor.parse_html = False
>>> model = AutoModelForTokenClassification.from_pretrained("microsoft/markuplm-base", num_labels=7)

>>> nodes = ["hello", "world"]
>>> xpaths = ["/html/body/div/li[1]/div/span", "/html/body/div/li[1]/div/span"]
>>> node_labels = [1, 2]
>>> encoding = processor(nodes=nodes, xpaths=xpaths, node_labels=node_labels, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**encoding)

>>> loss = outputs.loss
>>> logits = outputs.logits
```

## MarkupLMForQuestionAnswering[[transformers.MarkupLMForQuestionAnswering]]

#### transformers.MarkupLMForQuestionAnswering[[transformers.MarkupLMForQuestionAnswering]]

```python
transformers.MarkupLMForQuestionAnswering(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/modeling_markuplm.py#L603)

**Parameters:**

config ([MarkupLMForQuestionAnswering](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMForQuestionAnswering)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Markuplm transformer with a span classification head on top for extractive question-answering tasks like
SQuAD (a linear layer on top of the hidden-states output to compute `span start logits` and `span end logits`).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MarkupLMForQuestionAnswering.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, xpath_tags_seq: typing.Optional[torch.Tensor] = None, xpath_subs_seq: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, start_positions: typing.Optional[torch.Tensor] = None, end_positions: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/markuplm/modeling_markuplm.py#L615)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

xpath_tags_seq (`torch.LongTensor` of shape `(batch_size, sequence_length, config.max_depth)`, *optional*) : Tag IDs for each token in the input sequence, padded up to config.max_depth.

xpath_subs_seq (`torch.LongTensor` of shape `(batch_size, sequence_length, config.max_depth)`, *optional*) : Subscript IDs for each token in the input sequence, padded up to config.max_depth.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

start_positions (`torch.Tensor` of shape `(batch_size,)`, *optional*) : Labels for position (index) of the start of the labelled span for computing the token classification loss. Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence are not taken into account for computing the loss.

end_positions (`torch.Tensor` of shape `(batch_size,)`, *optional*) : Labels for position (index) of the end of the labelled span for computing the token classification loss. Positions are clamped to the length of the sequence (`sequence_length`). Position outside of the sequence are not taken into account for computing the loss.

**Returns:** [QuestionAnsweringModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or `tuple(torch.FloatTensor)`

A [QuestionAnsweringModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MarkupLMConfig](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMConfig)) and inputs.

The [MarkupLMForQuestionAnswering](/docs/transformers/v5.15.1/en/model_doc/markuplm#transformers.MarkupLMForQuestionAnswering) forward method, overrides the `__call__` special method.

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

Examples:

```python
>>> from transformers import AutoProcessor, MarkupLMForQuestionAnswering
>>> import torch

>>> processor = AutoProcessor.from_pretrained("microsoft/markuplm-base-finetuned-websrc")
>>> model = MarkupLMForQuestionAnswering.from_pretrained("microsoft/markuplm-base-finetuned-websrc")

>>> html_string = "<html> <head> <title>My name is Niels</title> </head> </html>"
>>> question = "What's his name?"

>>> encoding = processor(html_string, questions=question, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**encoding)

>>> answer_start_index = outputs.start_logits.argmax()
>>> answer_end_index = outputs.end_logits.argmax()

>>> predict_answer_tokens = encoding.input_ids[0, answer_start_index : answer_end_index + 1]
>>> processor.decode(predict_answer_tokens).strip()
'Niels'
```
