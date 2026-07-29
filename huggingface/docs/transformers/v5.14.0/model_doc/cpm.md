# CPM

## Overview

The CPM model was proposed in [CPM: A Large-scale Generative Chinese Pre-trained Language Model](https://huggingface.co/papers/2012.00413) by Zhengyan Zhang, Xu Han, Hao Zhou, Pei Ke, Yuxian Gu, Deming Ye, Yujia Qin,
Yusheng Su, Haozhe Ji, Jian Guan, Fanchao Qi, Xiaozhi Wang, Yanan Zheng, Guoyang Zeng, Huanqi Cao, Shengqi Chen,
Daixuan Li, Zhenbo Sun, Zhiyuan Liu, Minlie Huang, Wentao Han, Jie Tang, Juanzi Li, Xiaoyan Zhu, Maosong Sun.

The abstract from the paper is the following:

*Pre-trained Language Models (PLMs) have proven to be beneficial for various downstream NLP tasks. Recently, GPT-3,
with 175 billion parameters and 570GB training data, drew a lot of attention due to the capacity of few-shot (even
zero-shot) learning. However, applying GPT-3 to address Chinese NLP tasks is still challenging, as the training corpus
of GPT-3 is primarily English, and the parameters are not publicly available. In this technical report, we release the
Chinese Pre-trained Language Model (CPM) with generative pre-training on large-scale Chinese training data. To the best
of our knowledge, CPM, with 2.6 billion parameters and 100GB Chinese training data, is the largest Chinese pre-trained
language model, which could facilitate several downstream Chinese NLP tasks, such as conversation, essay generation,
cloze test, and language understanding. Extensive experiments demonstrate that CPM achieves strong performance on many
NLP tasks in the settings of few-shot (even zero-shot) learning.*

This model was contributed by [canwenxu](https://huggingface.co/canwenxu). The original implementation can be found
here: https://github.com/TsinghuaAI/CPM-Generate

CPM's architecture is the same as GPT-2, except for tokenization method. Refer to [GPT-2 documentation](gpt2) for
API reference information.

## CpmTokenizer[[transformers.CpmTokenizer]]

'"}, {"name": "eos_token", "val": " = ''"}, {"name": "unk_token", "val": " = ''"}, {"name": "sep_token", "val": " = ''"}, {"name": "pad_token", "val": " = ''"}, {"name": "cls_token", "val": " = ''"}, {"name": "mask_token", "val": " = ''"}, {"name": "additional_special_tokens", "val": " = ['', '']"}, {"name": "sp_model_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "**kwargs", "val": ""}]}>

Runs pre-tokenization with Jieba-RS segmentation tool. It is used in CPM models.

- **token_ids_0** (`list[int]`) --
  List of IDs to which the special tokens will be added.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.`list[int]`List of [input IDs](../glossary#input-ids) with the appropriate special tokens.

Build model inputs from a sequence or a pair of sequence for sequence classification tasks by concatenating and
adding special tokens. An XLNet sequence has the following format:

- single sequence: `X <sep> <cls>`
- pair of sequences: `A <sep> B <sep> <cls>`

Converts a sequence of tokens (strings for sub-words) in a single string.

- **token_ids_0** (`list[int]`) --
  List of IDs.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.`list[int]`List of [token type IDs](../glossary#token-type-ids) according to the given sequence(s).

Create a mask from the two sequences passed to be used in a sequence-pair classification task. An XLNet

sequence pair mask has the following format:

```
0 0 0 0 0 0 0 0 0 0 0 1 1 1 1 1 1 1 1 1
| first sequence    | second sequence |
```

If `token_ids_1` is `None`, this method only returns the first portion of the mask (0s).

- **token_ids_0** (`list[int]`) --
  List of IDs.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.
- **already_has_special_tokens** (`bool`, *optional*, defaults to `False`) --
  Whether or not the token list is already formatted with special tokens for the model.`list[int]`A list of integers in the range [0, 1]: 1 for a special token, 0 for a sequence token.

Retrieve sequence ids from a token list that has no special tokens added. This method is called when adding
special tokens using the tokenizer `prepare_for_model` method.

## CpmTokenizerFast[[transformers.CpmTokenizerFast]]

'"}, {"name": "eos_token", "val": " = ''"}, {"name": "unk_token", "val": " = ''"}, {"name": "sep_token", "val": " = ''"}, {"name": "pad_token", "val": " = ''"}, {"name": "cls_token", "val": " = ''"}, {"name": "mask_token", "val": " = ''"}, {"name": "additional_special_tokens", "val": " = ['', '']"}, {"name": "**kwargs", "val": ""}]}>

Runs pre-tokenization with Jieba-RS segmentation tool. It is used in CPM models.

- **token_ids_0** (`list[int]`) --
  List of IDs to which the special tokens will be added.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.`list[int]`List of [input IDs](../glossary#input-ids) with the appropriate special tokens.

Build model inputs from a sequence or a pair of sequence for sequence classification tasks by concatenating and
adding special tokens. An XLNet sequence has the following format:

- single sequence: `X <sep> <cls>`
- pair of sequences: `A <sep> B <sep> <cls>`

- **token_ids_0** (`list[int]`) --
  List of IDs.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.`list[int]`List of [token type IDs](../glossary#token-type-ids) according to the given sequence(s).

Create a mask from the two sequences passed to be used in a sequence-pair classification task. An XLNet

sequence pair mask has the following format:

```
0 0 0 0 0 0 0 0 0 0 0 1 1 1 1 1 1 1 1 1
| first sequence    | second sequence |
```

If `token_ids_1` is `None`, this method only returns the first portion of the mask (0s).
