# PhoBERT

## Overview

The PhoBERT model was proposed in [PhoBERT: Pre-trained language models for Vietnamese](https://huggingface.co/papers/2003.00744) by Dat Quoc Nguyen, Anh Tuan Nguyen.

The abstract from the paper is the following:

*We present PhoBERT with two versions, PhoBERT-base and PhoBERT-large, the first public large-scale monolingual
language models pre-trained for Vietnamese. Experimental results show that PhoBERT consistently outperforms the recent
best pre-trained multilingual model XLM-R (Conneau et al., 2020) and improves the state-of-the-art in multiple
Vietnamese-specific NLP tasks including Part-of-speech tagging, Dependency parsing, Named-entity recognition and
Natural language inference.*

This model was contributed by [dqnguyen](https://huggingface.co/dqnguyen). The original code can be found [here](https://github.com/VinAIResearch/PhoBERT).

## Usage example

```python
import torch

from transformers import AutoModel, AutoTokenizer

phobert = AutoModel.from_pretrained("vinai/phobert-base", device_map="auto")
tokenizer = AutoTokenizer.from_pretrained("vinai/phobert-base")

# INPUT TEXT MUST BE ALREADY WORD-SEGMENTED!
line = "Tôi là sinh_viên trường đại_học Công_nghệ ."

input_ids = torch.tensor([tokenizer.encode(line)])

with torch.no_grad():
    features = phobert(input_ids)  # Models outputs are now tuples
```

PhoBERT implementation is the same as BERT, except for tokenization. Refer to [BERT documentation](bert) for information on
configuration classes and their parameters. PhoBERT-specific tokenizer is documented below.

## PhobertTokenizer[[transformers.PhobertTokenizer]]

#### transformers.PhobertTokenizer[[transformers.PhobertTokenizer]]

```python
transformers.PhobertTokenizer(vocab_file, merges_file, bos_token = '<s>', eos_token = '</s>', sep_token = '</s>', cls_token = '<s>', unk_token = '<unk>', pad_token = '<pad>', mask_token = '<mask>', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/phobert/tokenization_phobert.py#L49)

**Parameters:**

vocab_file (`str`) : Path to the vocabulary file.

merges_file (`str`) : Path to the merges file.

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.    When building a sequence using special tokens, this is not the token that is used for the beginning of sequence. The token used is the `cls_token`.   

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.    When building a sequence using special tokens, this is not the token that is used for the end of sequence. The token used is the `sep_token`.   

sep_token (`str`, *optional*, defaults to `"</s>"`) : The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for sequence classification or for a text and a question for question answering. It is also used as the last token of a sequence built with special tokens.

cls_token (`str`, *optional*, defaults to `"<s>"`) : The classifier token which is used when doing sequence classification (classification of the whole sequence instead of per-token classification). It is the first token of the sequence when built with special tokens.

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

mask_token (`str`, *optional*, defaults to `"<mask>"`) : The token used for masking values. This is the token used when training this model with masked language modeling. This is the token which the model will try to predict.

Construct a PhoBERT tokenizer. Based on Byte-Pair-Encoding.

This tokenizer inherits from [PreTrainedTokenizer](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.PythonBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods.

#### add_from_file[[transformers.PhobertTokenizer.add_from_file]]

```python
add_from_file(f)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/phobert/tokenization_phobert.py#L325)

Loads a pre-existing dictionary from a text file and adds its symbols to this instance.

#### build_inputs_with_special_tokens[[transformers.PhobertTokenizer.build_inputs_with_special_tokens]]

```python
build_inputs_with_special_tokens(token_ids_0: list, token_ids_1: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/phobert/tokenization_phobert.py#L144)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs to which the special tokens will be added.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

**Returns:** `list[int]`

List of [input IDs](../glossary#input-ids) with the appropriate special tokens.

Build model inputs from a sequence or a pair of sequence for sequence classification tasks by concatenating and
adding special tokens. A PhoBERT sequence has the following format:

- single sequence: `<s> X </s>`
- pair of sequences: `<s> A </s></s> B </s>`

#### convert_tokens_to_string[[transformers.PhobertTokenizer.convert_tokens_to_string]]

```python
convert_tokens_to_string(tokens)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/phobert/tokenization_phobert.py#L291)

Converts a sequence of tokens (string) in a single string.

#### create_token_type_ids_from_sequences[[transformers.PhobertTokenizer.create_token_type_ids_from_sequences]]

```python
create_token_type_ids_from_sequences(token_ids_0: list, token_ids_1: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/phobert/tokenization_phobert.py#L198)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

**Returns:** `list[int]`

List of zeros.

Create a mask from the two sequences passed to be used in a sequence-pair classification task. PhoBERT does not
make use of token type ids, therefore a list of zeros is returned.

#### get_special_tokens_mask[[transformers.PhobertTokenizer.get_special_tokens_mask]]

```python
get_special_tokens_mask(token_ids_0: list, token_ids_1: list[int] | None = None, already_has_special_tokens: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/phobert/tokenization_phobert.py#L170)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

already_has_special_tokens (`bool`, *optional*, defaults to `False`) : Whether or not the token list is already formatted with special tokens for the model.

**Returns:** `list[int]`

A list of integers in the range [0, 1]: 1 for a special token, 0 for a sequence token.

Retrieve sequence ids from a token list that has no special tokens added. This method is called when adding
special tokens using the tokenizer `prepare_for_model` method.
