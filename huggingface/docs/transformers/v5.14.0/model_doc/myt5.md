# myt5

## Overview

The myt5 model was proposed in [MYTE: Morphology-Driven Byte Encoding for Better and Fairer Multilingual Language Modeling](https://huggingface.co/papers/2403.10691) by Tomasz Limisiewicz, Terra Blevins, Hila Gonen, Orevaoghene Ahia, and Luke Zettlemoyer.
MyT5 (**My**te **T5**) is a multilingual language model based on T5 architecture.
The model uses a **m**orphologically-driven **byte** (**MYTE**) representation described in our paper.
**MYTE** uses codepoints corresponding to morphemes in contrast to characters used in UTF-8 encoding.
As a pre-requisite, we used unsupervised morphological segmentation ([Morfessor](https://aclanthology.org/E14-2006.pdf)) to obtain morpheme inventories for 99 languages.
However, the morphological segmentation step is not needed when using the pre-defined morpheme inventory from the hub (see: [Tomli/myt5-base](https://huggingface.co/Tomlim/myt5-base)).

The abstract from the paper is the following:

*A major consideration in multilingual language modeling is how to best represent languages with diverse vocabularies and scripts. Although contemporary text encoding methods cover most of the world’s writing systems, they exhibit bias towards the high-resource languages of the Global West. As a result, texts of underrepresented languages tend to be segmented into long sequences of linguistically meaningless units. To address the disparities, we introduce a new paradigm that encodes the same information with segments of consistent size across diverse languages. Our encoding convention (MYTE) is based on morphemes, as their inventories are more balanced across languages than characters, which are used in previous methods. We show that MYTE produces shorter encodings for all 99 analyzed languages, with the most notable improvements for non-European languages and non-Latin scripts. This, in turn, improves multilingual LM performance and diminishes the perplexity gap throughout diverse languages.*

This model was contributed by [Tomasz Limisiewicz](https://huggingface.co/Tomlim).
The original code can be found [here](https://github.com/tomlimi/MYTE).

## MyT5Tokenizer[[transformers.MyT5Tokenizer]]

'"}, {"name": "unk_token", "val": " = ''"}, {"name": "pad_token", "val": " = ''"}, {"name": "extra_ids", "val": " = 125"}, {"name": "additional_special_tokens", "val": " = None"}, {"name": "**kwargs", "val": ""}]}>
- **vocab_file** (`str`) -- The file containing the byte rewriting rules.
- **eos_token** (`str`, *optional*, defaults to `"</s>"`) --
  The end of sequence token.

- **unk_token** (`str`, *optional*, defaults to `"<unk>"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **pad_token** (`str`, *optional*, defaults to `"<pad>"`) --
  The token used for padding, for example when batching sequences of different lengths.
- **extra_ids** (`int`, *optional*, defaults to 125) --
  Add a number of extra ids added to the end of the vocabulary for use as sentinels. These tokens are
  accessible as "" where "{%d}" is a number between 0 and extra_ids-1. Extra tokens are
  indexed from the end of the vocabulary up to beginning ("" is the last token in the vocabulary
  like in ByT5 preprocessing see
  [here](https://github.com/google-research/text-to-text-transfer-transformer/blob/9fd7b14a769417be33bc6c850f9598764913c833/t5/data/preprocessors.py#L2117)).
- **additional_special_tokens** (`list[str]`, *optional*) --
  Additional special tokens used by the tokenizer.

Construct a MyT5 tokenizer.

This tokenizer inherits from [PreTrainedTokenizer](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.PythonBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods.

- **token_ids_0** (`list[int]`) --
  List of IDs to which the special tokens will be added.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.`list[int]`List of [input IDs](../glossary#input-ids) with the appropriate special tokens.

Build model inputs from a sequence or a pair of sequence for sequence classification tasks by concatenating and
adding special tokens. A sequence has the following format:

- single sequence: `X </s>`
- pair of sequences: `A </s> B </s>`

- **token_ids_0** (`list[int]`) --
  List of IDs.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.
- **already_has_special_tokens** (`bool`, *optional*, defaults to `False`) --
  Whether or not the token list is already formatted with special tokens for the model.`list[int]`A list of integers in the range [0, 1]: 1 for a special token, 0 for a sequence token.

Retrieve sequence ids from a token list that has no special tokens added. This method is called when adding
special tokens using the tokenizer `prepare_for_model` method.

- **token_ids_0** (`list[int]`) --
  List of IDs.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.`list[int]`List of zeros.

Create a mask from the two sequences passed to be used in a sequence-pair classification task. MyT5 does not
make use of token type ids, therefore a list of zeros is returned.
