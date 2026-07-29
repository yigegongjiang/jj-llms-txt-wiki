# LayoutLMv3

## Overview

The LayoutLMv3 model was proposed in [LayoutLMv3: Pre-training for Document AI with Unified Text and Image Masking](https://huggingface.co/papers/2204.08387) by Yupan Huang, Tengchao Lv, Lei Cui, Yutong Lu, Furu Wei.
LayoutLMv3 simplifies [LayoutLMv2](layoutlmv2) by using patch embeddings (as in [ViT](vit)) instead of leveraging a CNN backbone, and pre-trains the model on 3 objectives: masked language modeling (MLM), masked image modeling (MIM)
and word-patch alignment (WPA).

The abstract from the paper is the following:

*Self-supervised pre-training techniques have achieved remarkable progress in Document AI. Most multimodal pre-trained models use a masked language modeling objective to learn bidirectional representations on the text modality, but they differ in pre-training objectives for the image modality. This discrepancy adds difficulty to multimodal representation learning. In this paper, we propose LayoutLMv3 to pre-train multimodal Transformers for Document AI with unified text and image masking. Additionally, LayoutLMv3 is pre-trained with a word-patch alignment objective to learn cross-modal alignment by predicting whether the corresponding image patch of a text word is masked. The simple unified architecture and training objectives make LayoutLMv3 a general-purpose pre-trained model for both text-centric and image-centric Document AI tasks. Experimental results show that LayoutLMv3 achieves state-of-the-art performance not only in text-centric tasks, including form understanding, receipt understanding, and document visual question answering, but also in image-centric tasks such as document image classification and document layout analysis.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/layoutlmv3_architecture.png"
alt="drawing" width="600"/>

 LayoutLMv3 architecture. Taken from the original paper. 

This model was contributed by [nielsr](https://huggingface.co/nielsr). The original code can be found [here](https://github.com/microsoft/unilm/tree/master/layoutlmv3).

## Usage tips

- In terms of data processing, LayoutLMv3 is identical to its predecessor [LayoutLMv2](layoutlmv2), except that:
  - images need to be resized and normalized with channels in regular RGB format. LayoutLMv2 on the other hand normalizes the images internally and expects the channels in BGR format.
  - text is tokenized using byte-pair encoding (BPE), as opposed to WordPiece.
  Due to these differences in data preprocessing, one can use [LayoutLMv3Processor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Processor) which internally combines a [LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor) (for the image modality) and a [LayoutLMv3Tokenizer](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Tokenizer)/[LayoutLMv3TokenizerFast](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Tokenizer) (for the text modality) to prepare all data for the model.
- Regarding usage of [LayoutLMv3Processor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Processor), we refer to the [usage guide](layoutlmv2#usage-layoutlmv2processor) of its predecessor.

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with LayoutLMv3. If you're interested in submitting a resource to be included here, please feel free to open a Pull Request and we'll review it! The resource should ideally demonstrate something new instead of duplicating an existing resource.

LayoutLMv3 is nearly identical to LayoutLMv2, so we've also included LayoutLMv2 resources you can adapt for LayoutLMv3 tasks. For these notebooks, take care to use [LayoutLMv2Processor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv2#transformers.LayoutLMv2Processor) instead when preparing data for the model!

- Demo notebooks for LayoutLMv3 can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/LayoutLMv3).
- Demo scripts can be found [here](https://github.com/huggingface/transformers-research-projects/tree/main/layoutlmv3).

- [LayoutLMv2ForSequenceClassification](/docs/transformers/v5.14.0/en/model_doc/layoutlmv2#transformers.LayoutLMv2ForSequenceClassification) is supported by this [notebook](https://colab.research.google.com/github/NielsRogge/Transformers-Tutorials/blob/master/LayoutLMv2/RVL-CDIP/Fine_tuning_LayoutLMv2ForSequenceClassification_on_RVL_CDIP.ipynb).
- [Text classification task guide](../tasks/sequence_classification)

- [LayoutLMv3ForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ForTokenClassification) is supported by this [example script](https://github.com/huggingface/transformers-research-projects/tree/main/layoutlmv3) and [notebook](https://colab.research.google.com/github/NielsRogge/Transformers-Tutorials/blob/master/LayoutLMv3/Fine_tune_LayoutLMv3_on_FUNSD_(HuggingFace_Trainer).ipynb).
- A [notebook](https://colab.research.google.com/github/NielsRogge/Transformers-Tutorials/blob/master/LayoutLMv2/FUNSD/Inference_with_LayoutLMv2ForTokenClassification.ipynb) for how to perform inference with [LayoutLMv2ForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/layoutlmv2#transformers.LayoutLMv2ForTokenClassification) and a [notebook](https://colab.research.google.com/github/NielsRogge/Transformers-Tutorials/blob/master/LayoutLMv2/FUNSD/True_inference_with_LayoutLMv2ForTokenClassification_%2B_Gradio_demo.ipynb) for how to perform inference when no labels are available with [LayoutLMv2ForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/layoutlmv2#transformers.LayoutLMv2ForTokenClassification).
- A [notebook](https://colab.research.google.com/github/NielsRogge/Transformers-Tutorials/blob/master/LayoutLMv2/FUNSD/Fine_tuning_LayoutLMv2ForTokenClassification_on_FUNSD_using_HuggingFace_Trainer.ipynb) for how to finetune [LayoutLMv2ForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/layoutlmv2#transformers.LayoutLMv2ForTokenClassification) with the 🤗 Trainer.
- [Token classification task guide](../tasks/token_classification)

- [LayoutLMv2ForQuestionAnswering](/docs/transformers/v5.14.0/en/model_doc/layoutlmv2#transformers.LayoutLMv2ForQuestionAnswering) is supported by this [notebook](https://colab.research.google.com/github/NielsRogge/Transformers-Tutorials/blob/master/LayoutLMv2/DocVQA/Fine_tuning_LayoutLMv2ForQuestionAnswering_on_DocVQA.ipynb).
- [Question answering task guide](../tasks/question_answering)

**Document question answering**

- [Document question answering task guide](../tasks/document_question_answering)

## LayoutLMv3Config[[transformers.LayoutLMv3Config]]

- **vocab_size** (`int`, *optional*, defaults to `50265`) --
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
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **pad_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **max_2d_position_embeddings** (`int`, *optional*, defaults to 1024) --
  The maximum value that the 2D position embedding might ever be used with. Typically set this to something
  large just in case (e.g., 1024).
- **coordinate_size** (`int`, *optional*, defaults to `128`) --
  Dimension of the coordinate embeddings.
- **shape_size** (`int`, *optional*, defaults to `128`) --
  Dimension of the width and height embeddings.
- **has_relative_attention_bias** (`bool`, *optional*, defaults to `True`) --
  Whether or not to use a relative attention bias in the self-attention mechanism.
- **rel_pos_bins** (`int`, *optional*, defaults to 32) --
  The number of relative position bins to be used in the self-attention mechanism.
- **max_rel_pos** (`int`, *optional*, defaults to 128) --
  The maximum number of relative positions to be used in the self-attention mechanism.
- **rel_2d_pos_bins** (`int`, *optional*, defaults to 64) --
  The number of 2D relative position bins in the self-attention mechanism.
- **max_rel_2d_pos** (`int`, *optional*, defaults to 256) --
  The maximum number of relative 2D positions in the self-attention mechanism.
- **has_spatial_attention_bias** (`bool`, *optional*, defaults to `True`) --
  Whether or not to use a spatial attention bias in the self-attention mechanism.
- **text_embed** (`bool`, *optional*, defaults to `True`) --
  Whether or not to add text embeddings.
- **visual_embed** (`bool`, *optional*, defaults to `True`) --
  Whether or not to add patch embeddings.
- **input_size** (`int`, *optional*, defaults to `224`) --
  The size (resolution) of the images.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **classifier_dropout** (`Union[float, int]`, *optional*) --
  The dropout ratio for classifier.

This is the configuration class to store the configuration of a LayoutLMv3Model. It is used to instantiate a Layoutlmv3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/layoutlmv3-base](https://huggingface.co/microsoft/layoutlmv3-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import LayoutLMv3Config, LayoutLMv3Model

>>> # Initializing a LayoutLMv3 microsoft/layoutlmv3-base style configuration
>>> configuration = LayoutLMv3Config()

>>> # Initializing a model (with random weights) from the microsoft/layoutlmv3-base style configuration
>>> model = LayoutLMv3Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## LayoutLMv3ImageProcessor[[transformers.LayoutLMv3ImageProcessor]]

- **apply_ocr** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to apply the Tesseract OCR engine to get words + normalized bounding boxes. Can be overridden by
  the `apply_ocr` parameter in the `preprocess` method.
- **ocr_lang** (`str`, *kwargs*, *optional*) --
  The language, specified by its ISO code, to be used by the Tesseract OCR engine. By default, English is
  used. Can be overridden by the `ocr_lang` parameter in the `preprocess` method.
- **tesseract_config** (`str`, *kwargs*, *optional*) --
  Any additional custom configuration flags that are forwarded to the `config` parameter when calling
  Tesseract. For example: '--psm 6'. Can be overridden by the `tesseract_config` parameter in the
  `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a LayoutLMv3ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **apply_ocr** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to apply the Tesseract OCR engine to get words + normalized bounding boxes. Can be overridden by
  the `apply_ocr` parameter in the `preprocess` method.
- **ocr_lang** (`str`, *kwargs*, *optional*) --
  The language, specified by its ISO code, to be used by the Tesseract OCR engine. By default, English is
  used. Can be overridden by the `ocr_lang` parameter in the `preprocess` method.
- **tesseract_config** (`str`, *kwargs*, *optional*) --
  Any additional custom configuration flags that are forwarded to the `config` parameter when calling
  Tesseract. For example: '--psm 6'. Can be overridden by the `tesseract_config` parameter in the
  `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## LayoutLMv3ImageProcessorPil[[transformers.LayoutLMv3ImageProcessorPil]]

- **apply_ocr** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to apply the Tesseract OCR engine to get words + normalized bounding boxes. Can be overridden by
  the `apply_ocr` parameter in the `preprocess` method.
- **ocr_lang** (`str`, *kwargs*, *optional*) --
  The language, specified by its ISO code, to be used by the Tesseract OCR engine. By default, English is
  used. Can be overridden by the `ocr_lang` parameter in the `preprocess` method.
- **tesseract_config** (`str`, *kwargs*, *optional*) --
  Any additional custom configuration flags that are forwarded to the `config` parameter when calling
  Tesseract. For example: '--psm 6'. Can be overridden by the `tesseract_config` parameter in the
  `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a LayoutLMv3ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **apply_ocr** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to apply the Tesseract OCR engine to get words + normalized bounding boxes. Can be overridden by
  the `apply_ocr` parameter in the `preprocess` method.
- **ocr_lang** (`str`, *kwargs*, *optional*) --
  The language, specified by its ISO code, to be used by the Tesseract OCR engine. By default, English is
  used. Can be overridden by the `ocr_lang` parameter in the `preprocess` method.
- **tesseract_config** (`str`, *kwargs*, *optional*) --
  Any additional custom configuration flags that are forwarded to the `config` parameter when calling
  Tesseract. For example: '--psm 6'. Can be overridden by the `tesseract_config` parameter in the
  `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## LayoutLMv3Tokenizer[[transformers.LayoutLMv3Tokenizer]]

'"}, {"name": "eos_token", "val": " = ''"}, {"name": "sep_token", "val": " = ''"}, {"name": "cls_token", "val": " = ''"}, {"name": "unk_token", "val": " = ''"}, {"name": "pad_token", "val": " = ''"}, {"name": "mask_token", "val": " = ''"}, {"name": "add_prefix_space", "val": " = True"}, {"name": "cls_token_box", "val": " = [0, 0, 0, 0]"}, {"name": "sep_token_box", "val": " = [0, 0, 0, 0]"}, {"name": "pad_token_box", "val": " = [0, 0, 0, 0]"}, {"name": "pad_token_label", "val": " = -100"}, {"name": "only_label_first_subword", "val": " = True"}, {"name": "vocab", "val": ": str | dict[str, int] | None = None"}, {"name": "merges", "val": ": str | list[str] | None = None"}, {"name": "**kwargs", "val": ""}]}>
- **errors** (`str`, *optional*, defaults to `"replace"`) --
  Paradigm to follow when decoding bytes to UTF-8. See
  [bytes.decode](https://docs.python.org/3/library/stdtypes.html#bytes.decode) for more information.
- **bos_token** (`str`, *optional*, defaults to `"<s>"`) --
  The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.
- **eos_token** (`str`, *optional*, defaults to `"</s>"`) --
  The end of sequence token.
- **sep_token** (`str`, *optional*, defaults to `"</s>"`) --
  The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for
  sequence classification or for a text and a question for question answering. It is also used as the last
  token of a sequence built with special tokens.
- **cls_token** (`str`, *optional*, defaults to `"<s>"`) --
  The classifier token which is used when doing sequence classification (classification of the whole sequence
  instead of per-token classification). It is the first token of the sequence when built with special tokens.
- **unk_token** (`str`, *optional*, defaults to `"<unk>"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **pad_token** (`str`, *optional*, defaults to `"<pad>"`) --
  The token used for padding, for example when batching sequences of different lengths.
- **mask_token** (`str`, *optional*, defaults to `"<mask>"`) --
  The token used for masking values. This is the token used when training this model with masked language
  modeling. This is the token which the model will try to predict.
- **add_prefix_space** (`bool`, *optional*, defaults to `True`) --
  Whether or not to add an initial space to the input. This allows to treat the leading word just as any
  other word.
- **cls_token_box** (`list[int]`, *optional*, defaults to `[0, 0, 0, 0]`) --
  The bounding box to use for the special [CLS] token.
- **sep_token_box** (`list[int]`, *optional*, defaults to `[0, 0, 0, 0]`) --
  The bounding box to use for the special [SEP] token.
- **pad_token_box** (`list[int]`, *optional*, defaults to `[0, 0, 0, 0]`) --
  The bounding box to use for the special [PAD] token.
- **pad_token_label** (`int`, *optional*, defaults to -100) --
  The label to use for padding tokens. Defaults to -100, which is the `ignore_index` of PyTorch's
  CrossEntropyLoss.
- **only_label_first_subword** (`bool`, *optional*, defaults to `True`) --
  Whether or not to only label the first subword, in case word labels are provided.
- **vocab** (`str` or `dict[str, int]`, *optional*) --
  Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file` when using
  `from_pretrained`.
- **merges** (`str` or `list[str]`, *optional*) --
  Custom merges list. If not provided, merges are loaded from `merges_file` when using `from_pretrained`.

Construct a LayoutLMv3 tokenizer (backed by HuggingFace's *tokenizers* library). Based on byte-level BPE.

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

- **text** (`str`, `List[str]`, `List[List[str]]`) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string, a list of strings
  (words of a single example or questions of a batch of examples) or a list of list of strings (batch of
  words).
- **text_pair** (`List[str]`, `List[List[str]]`) --
  The sequence or batch of sequences to be encoded. Each sequence should be a list of strings
  (pretokenized string).
- **boxes** (`List[List[int]]`, `List[List[List[int]]]`) --
  Word-level bounding boxes. Each bounding box should be normalized to be on a 0-1000 scale.
- **word_labels** (`List[int]`, `List[List[int]]`, *optional*) --
  Word-level integer labels (for token classification tasks such as FUNSD, CORD).

- **add_special_tokens** (`bool`, *optional*, defaults to `True`) --
  Whether or not to encode the sequences with the special tokens relative to their model.
- **padding** (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) --
  Activates and controls padding. Accepts the following values:

  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single
    sequence if provided).
  - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided.
  - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different
    lengths).
- **truncation** (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) --
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
- **max_length** (`int`, *optional*) --
  Controls the maximum length to use by one of the truncation/padding parameters.

  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length
  is required by one of the truncation/padding parameters. If the model has no specific maximum input
  length (like XLNet) truncation/padding to a maximum length will be deactivated.
- **stride** (`int`, *optional*, defaults to 0) --
  If set to a number along with `max_length`, the overflowing tokens returned when
  `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence
  returned to provide some overlap between truncated and overflowing sequences. The value of this
  argument defines the number of overlapping tokens.
- **pad_to_multiple_of** (`int`, *optional*) --
  If set will pad the sequence to a multiple of the provided value. This is especially useful to enable
  the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

- **return_token_type_ids** (`bool`, *optional*) --
  Whether to return token type IDs. If left to the default, will return the token type IDs according to
  the specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are token type IDs?](../glossary#token-type-ids)
- **return_attention_mask** (`bool`, *optional*) --
  Whether to return the attention mask. If left to the default, will return the attention mask according
  to the specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are attention masks?](../glossary#attention-mask)
- **return_overflowing_tokens** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch
  of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead
  of returning overflowing tokens.
- **return_special_tokens_mask** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return special tokens mask information.
- **return_offsets_mapping** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return `(char_start, char_end)` for each token.

  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend), if using
  Python's tokenizer, this method will raise `NotImplementedError`.
- **return_length**  (`bool`, *optional*, defaults to `False`) --
  Whether or not to return the lengths of the encoded inputs.
- **verbose** (`bool`, *optional*, defaults to `True`) --
  Whether or not to print more information and warnings.
- ****kwargs** -- passed to the `self.tokenize()` method

Main method to tokenize and prepare for the model one or several sequence(s) or one or several pair(s) of
sequences with word-level normalized bounding boxes and optional labels.

## LayoutLMv3TokenizerFast[[transformers.LayoutLMv3Tokenizer]]

'"}, {"name": "eos_token", "val": " = ''"}, {"name": "sep_token", "val": " = ''"}, {"name": "cls_token", "val": " = ''"}, {"name": "unk_token", "val": " = ''"}, {"name": "pad_token", "val": " = ''"}, {"name": "mask_token", "val": " = ''"}, {"name": "add_prefix_space", "val": " = True"}, {"name": "cls_token_box", "val": " = [0, 0, 0, 0]"}, {"name": "sep_token_box", "val": " = [0, 0, 0, 0]"}, {"name": "pad_token_box", "val": " = [0, 0, 0, 0]"}, {"name": "pad_token_label", "val": " = -100"}, {"name": "only_label_first_subword", "val": " = True"}, {"name": "vocab", "val": ": str | dict[str, int] | None = None"}, {"name": "merges", "val": ": str | list[str] | None = None"}, {"name": "**kwargs", "val": ""}]}>
- **errors** (`str`, *optional*, defaults to `"replace"`) --
  Paradigm to follow when decoding bytes to UTF-8. See
  [bytes.decode](https://docs.python.org/3/library/stdtypes.html#bytes.decode) for more information.
- **bos_token** (`str`, *optional*, defaults to `"<s>"`) --
  The beginning of sequence token that was used during pretraining. Can be used a sequence classifier token.
- **eos_token** (`str`, *optional*, defaults to `"</s>"`) --
  The end of sequence token.
- **sep_token** (`str`, *optional*, defaults to `"</s>"`) --
  The separator token, which is used when building a sequence from multiple sequences, e.g. two sequences for
  sequence classification or for a text and a question for question answering. It is also used as the last
  token of a sequence built with special tokens.
- **cls_token** (`str`, *optional*, defaults to `"<s>"`) --
  The classifier token which is used when doing sequence classification (classification of the whole sequence
  instead of per-token classification). It is the first token of the sequence when built with special tokens.
- **unk_token** (`str`, *optional*, defaults to `"<unk>"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **pad_token** (`str`, *optional*, defaults to `"<pad>"`) --
  The token used for padding, for example when batching sequences of different lengths.
- **mask_token** (`str`, *optional*, defaults to `"<mask>"`) --
  The token used for masking values. This is the token used when training this model with masked language
  modeling. This is the token which the model will try to predict.
- **add_prefix_space** (`bool`, *optional*, defaults to `True`) --
  Whether or not to add an initial space to the input. This allows to treat the leading word just as any
  other word.
- **cls_token_box** (`list[int]`, *optional*, defaults to `[0, 0, 0, 0]`) --
  The bounding box to use for the special [CLS] token.
- **sep_token_box** (`list[int]`, *optional*, defaults to `[0, 0, 0, 0]`) --
  The bounding box to use for the special [SEP] token.
- **pad_token_box** (`list[int]`, *optional*, defaults to `[0, 0, 0, 0]`) --
  The bounding box to use for the special [PAD] token.
- **pad_token_label** (`int`, *optional*, defaults to -100) --
  The label to use for padding tokens. Defaults to -100, which is the `ignore_index` of PyTorch's
  CrossEntropyLoss.
- **only_label_first_subword** (`bool`, *optional*, defaults to `True`) --
  Whether or not to only label the first subword, in case word labels are provided.
- **vocab** (`str` or `dict[str, int]`, *optional*) --
  Custom vocabulary dictionary. If not provided, vocabulary is loaded from `vocab_file` when using
  `from_pretrained`.
- **merges** (`str` or `list[str]`, *optional*) --
  Custom merges list. If not provided, merges are loaded from `merges_file` when using `from_pretrained`.

Construct a LayoutLMv3 tokenizer (backed by HuggingFace's *tokenizers* library). Based on byte-level BPE.

This tokenizer inherits from [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) which contains most of the main methods. Users should
refer to this superclass for more information regarding those methods.

- **text** (`str`, `List[str]`, `List[List[str]]`) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string, a list of strings
  (words of a single example or questions of a batch of examples) or a list of list of strings (batch of
  words).
- **text_pair** (`List[str]`, `List[List[str]]`) --
  The sequence or batch of sequences to be encoded. Each sequence should be a list of strings
  (pretokenized string).
- **boxes** (`List[List[int]]`, `List[List[List[int]]]`) --
  Word-level bounding boxes. Each bounding box should be normalized to be on a 0-1000 scale.
- **word_labels** (`List[int]`, `List[List[int]]`, *optional*) --
  Word-level integer labels (for token classification tasks such as FUNSD, CORD).

- **add_special_tokens** (`bool`, *optional*, defaults to `True`) --
  Whether or not to encode the sequences with the special tokens relative to their model.
- **padding** (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) --
  Activates and controls padding. Accepts the following values:

  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single
    sequence if provided).
  - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided.
  - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different
    lengths).
- **truncation** (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) --
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
- **max_length** (`int`, *optional*) --
  Controls the maximum length to use by one of the truncation/padding parameters.

  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length
  is required by one of the truncation/padding parameters. If the model has no specific maximum input
  length (like XLNet) truncation/padding to a maximum length will be deactivated.
- **stride** (`int`, *optional*, defaults to 0) --
  If set to a number along with `max_length`, the overflowing tokens returned when
  `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence
  returned to provide some overlap between truncated and overflowing sequences. The value of this
  argument defines the number of overlapping tokens.
- **pad_to_multiple_of** (`int`, *optional*) --
  If set will pad the sequence to a multiple of the provided value. This is especially useful to enable
  the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

- **return_token_type_ids** (`bool`, *optional*) --
  Whether to return token type IDs. If left to the default, will return the token type IDs according to
  the specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are token type IDs?](../glossary#token-type-ids)
- **return_attention_mask** (`bool`, *optional*) --
  Whether to return the attention mask. If left to the default, will return the attention mask according
  to the specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are attention masks?](../glossary#attention-mask)
- **return_overflowing_tokens** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch
  of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead
  of returning overflowing tokens.
- **return_special_tokens_mask** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return special tokens mask information.
- **return_offsets_mapping** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return `(char_start, char_end)` for each token.

  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend), if using
  Python's tokenizer, this method will raise `NotImplementedError`.
- **return_length**  (`bool`, *optional*, defaults to `False`) --
  Whether or not to return the lengths of the encoded inputs.
- **verbose** (`bool`, *optional*, defaults to `True`) --
  Whether or not to print more information and warnings.
- ****kwargs** -- passed to the `self.tokenize()` method

Main method to tokenize and prepare for the model one or several sequence(s) or one or several pair(s) of
sequences with word-level normalized bounding boxes and optional labels.

## LayoutLMv3Processor[[transformers.LayoutLMv3Processor]]

- **image_processor** (`LayoutLMv3ImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`LayoutLMv3Tokenizer`) --
  The tokenizer is a required input.
Constructs a LayoutLMv3Processor which wraps a image processor and a tokenizer into a single processor.

[LayoutLMv3Processor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Processor) offers all the functionalities of [LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor) and [LayoutLMv3Tokenizer](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Tokenizer). See the
[~LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor) and [~LayoutLMv3Tokenizer](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Tokenizer) for more information.

- **images** (``) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **text_pair** (`str, list[str] or list[int]`, *optional*) --
  Optional second sequence to be encoded. This can be a string, a list of strings (tokenized string using
  the `tokenize` method) or a list of integers (tokenized string ids using the `convert_tokens_to_ids`
  method).
- **boxes** (`list[list[int]] or list[list[list[int]]]`, *optional*) --
  Word-level bounding boxes. Each bounding box should be normalized to be on a 0-1000 scale.
- **word_labels** (`list[int] or list[list[int]]`, *optional*) --
  Word-level integer labels (for token classification tasks such as FUNSD, CORD).
- **add_special_tokens** (`bool`, *optional*, defaults to `True`) --
  Whether or not to add special tokens when encoding the sequences. This will use the underlying
  `PretrainedTokenizerBase.build_inputs_with_special_tokens` function, which defines which tokens are
  automatically added to the input ids. This is useful if you want to add `bos` or `eos` tokens
  automatically.
- **padding** (bool, str or [PaddingStrategy](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) --
  Activates and controls padding. Accepts the following values:

  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single
    sequence is provided).
  - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided.
  - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different
    lengths).
- **truncation** (bool, str or [TruncationStrategy](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*) --
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
- **max_length** (`int`, *optional*) --
  Controls the maximum length to use by one of the truncation/padding parameters.

  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length
  is required by one of the truncation/padding parameters. If the model has no specific maximum input
  length (like XLNet) truncation/padding to a maximum length will be deactivated.
- **stride** (`int`, *optional*, defaults to `0`) --
  If set to a number along with `max_length`, the overflowing tokens returned when
  `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence
  returned to provide some overlap between truncated and overflowing sequences. The value of this
  argument defines the number of overlapping tokens.
- **pad_to_multiple_of** (`int`, *optional*) --
  If set will pad the sequence to a multiple of the provided value. Requires `padding` to be activated.
  This is especially useful to enable using Tensor Cores on NVIDIA hardware with compute capability
  `>= 7.5` (Volta).
- **return_token_type_ids** (`bool`, *optional*) --
  Whether to return token type IDs. If left to the default, will return the token type IDs according to
  the specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are token type IDs?](../glossary#token-type-ids)
- **return_attention_mask** (`bool`, *optional*) --
  Whether to return the attention mask. If left to the default, will return the attention mask according
  to the specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are attention masks?](../glossary#attention-mask)
- **return_overflowing_tokens** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch
  of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead
  of returning overflowing tokens.
- **return_special_tokens_mask** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return special tokens mask information.
- **return_offsets_mapping** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return `(char_start, char_end)` for each token.

  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend), if using
  Python's tokenizer, this method will raise `NotImplementedError`.
- **return_length** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return the lengths of the encoded inputs.
- **verbose** (`bool`, *optional*, defaults to `True`) --
  Whether or not to print more information and warnings.
- **return_tensors** (`Union[str, ~utils.generic.TensorType]`, *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.`~tokenization_utils_base.BatchEncoding`- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the `__call__`/`encode_plus`/`batch_encode_plus` methods
  ('input_ids', 'attention_mask', etc.).
- **encoding** (`tokenizers.Encoding` or `Sequence[tokenizers.Encoding]`, *optional*) -- If the tokenizer is a fast tokenizer which outputs additional information like mapping from word/character
  space to token space the `tokenizers.Encoding` instance or list of instance (for batches) hold this
  information.
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **prepend_batch_axis** (`bool`, *optional*, defaults to `False`) -- Whether or not to add a batch axis when converting to tensors (see `tensor_type` above). Note that this
  parameter has an effect if the parameter `tensor_type` is set, *otherwise has no effect*.
- **n_sequences** (`int`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## LayoutLMv3Model[[transformers.LayoutLMv3Model]]

- **config** ([LayoutLMv3Model](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Model)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Layoutlmv3 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, token_sequence_length)`) --
  Indices of input sequence tokens in the vocabulary.

  Note that `sequence_length = token_sequence_length + patch_sequence_length + 1` where `1` is for [CLS]
  token. See `pixel_values` for `patch_sequence_length`.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **bbox** (`torch.LongTensor` of shape `(batch_size, token_sequence_length, 4)`, *optional*) --
  Bounding boxes of each input sequence tokens. Selected in the range `[0,
  config.max_2d_position_embeddings-1]`. Each bounding box should be a normalized version in (x0, y0, x1, y1)
  format, where (x0, y0) corresponds to the position of the upper left corner in the bounding box, and (x1,
  y1) represents the position of the lower right corner.

  Note that `sequence_length = token_sequence_length + patch_sequence_length + 1` where `1` is for [CLS]
  token. See `pixel_values` for `patch_sequence_length`.
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, token_sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0,
  1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  Note that `sequence_length = token_sequence_length + patch_sequence_length + 1` where `1` is for [CLS]
  token. See `pixel_values` for `patch_sequence_length`.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, token_sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0,
  config.max_position_embeddings - 1]`.

  Note that `sequence_length = token_sequence_length + patch_sequence_length + 1` where `1` is for [CLS]
  token. See `pixel_values` for `patch_sequence_length`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, token_sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert *input_ids* indices into associated vectors than the
  model's internal embedding lookup matrix.
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor). See `LayoutLMv3ImageProcessor.__call__()` for details ([LayoutLMv3Processor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Processor) uses
  [LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor) for processing images).[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LayoutLMv3Config](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Config)) and inputs.
The [LayoutLMv3Model](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Model) forward method, overrides the `__call__` special method.

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

Examples:

```python
>>> from transformers import AutoProcessor, AutoModel
>>> from datasets import load_dataset

>>> processor = AutoProcessor.from_pretrained("microsoft/layoutlmv3-base", apply_ocr=False)
>>> model = AutoModel.from_pretrained("microsoft/layoutlmv3-base")

>>> dataset = load_dataset("nielsr/funsd-layoutlmv3", split="train")
>>> example = dataset[0]
>>> image = example["image"]
>>> words = example["tokens"]
>>> boxes = example["bboxes"]

>>> encoding = processor(image, words, boxes=boxes, return_tensors="pt")

>>> outputs = model(**encoding)
>>> last_hidden_states = outputs.last_hidden_state
```

## LayoutLMv3ForSequenceClassification[[transformers.LayoutLMv3ForSequenceClassification]]

- **config** ([LayoutLMv3ForSequenceClassification](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ForSequenceClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

LayoutLMv3 Model with a sequence classification head on top (a linear layer on top of the final hidden state of the
[CLS] token) e.g. for document image classification tasks such as the
[RVL-CDIP](https://www.cs.cmu.edu/~aharley/rvl-cdip/) dataset.

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
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **bbox** (`torch.LongTensor` of shape `(batch_size, sequence_length, 4)`, *optional*) --
  Bounding boxes of each input sequence tokens. Selected in the range `[0,
  config.max_2d_position_embeddings-1]`. Each bounding box should be a normalized version in (x0, y0, x1, y1)
  format, where (x0, y0) corresponds to the position of the upper left corner in the bounding box, and (x1,
  y1) represents the position of the lower right corner.
- **pixel_values** (`torch.LongTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor). See `LayoutLMv3ImageProcessor.__call__()` for details ([LayoutLMv3Processor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Processor) uses
  [LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor) for processing images).[SequenceClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`A [SequenceClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LayoutLMv3Config](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Config)) and inputs.
The [LayoutLMv3ForSequenceClassification](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ForSequenceClassification) forward method, overrides the `__call__` special method.

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
>>> from datasets import load_dataset
>>> import torch

>>> processor = AutoProcessor.from_pretrained("microsoft/layoutlmv3-base", apply_ocr=False)
>>> model = AutoModelForSequenceClassification.from_pretrained("microsoft/layoutlmv3-base")

>>> dataset = load_dataset("nielsr/funsd-layoutlmv3", split="train")
>>> example = dataset[0]
>>> image = example["image"]
>>> words = example["tokens"]
>>> boxes = example["bboxes"]

>>> encoding = processor(image, words, boxes=boxes, return_tensors="pt")
>>> sequence_label = torch.tensor([1])

>>> outputs = model(**encoding, labels=sequence_label)
>>> loss = outputs.loss
>>> logits = outputs.logits
```

## LayoutLMv3ForTokenClassification[[transformers.LayoutLMv3ForTokenClassification]]

- **config** ([LayoutLMv3ForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ForTokenClassification)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

LayoutLMv3 Model with a token classification head on top (a linear layer on top of the final hidden states) e.g.
for sequence labeling (information extraction) tasks such as [FUNSD](https://guillaumejaume.github.io/FUNSD/),
[SROIE](https://rrc.cvc.uab.es/?ch=13), [CORD](https://github.com/clovaai/cord) and
[Kleister-NDA](https://github.com/applicaai/kleister-nda).

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
- **bbox** (`torch.LongTensor` of shape `(batch_size, sequence_length, 4)`, *optional*) --
  Bounding boxes of each input sequence tokens. Selected in the range `[0,
  config.max_2d_position_embeddings-1]`. Each bounding box should be a normalized version in (x0, y0, x1, y1)
  format, where (x0, y0) corresponds to the position of the upper left corner in the bounding box, and (x1,
  y1) represents the position of the lower right corner.
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the token classification loss. Indices should be in `[0, ..., config.num_labels - 1]`.
- **pixel_values** (`torch.LongTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor). See `LayoutLMv3ImageProcessor.__call__()` for details ([LayoutLMv3Processor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Processor) uses
  [LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor) for processing images).[TokenClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or `tuple(torch.FloatTensor)`A [TokenClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LayoutLMv3Config](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Config)) and inputs.
The [LayoutLMv3ForTokenClassification](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ForTokenClassification) forward method, overrides the `__call__` special method.

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

Examples:

```python
>>> from transformers import AutoProcessor, AutoModelForTokenClassification
>>> from datasets import load_dataset

>>> processor = AutoProcessor.from_pretrained("microsoft/layoutlmv3-base", apply_ocr=False)
>>> model = AutoModelForTokenClassification.from_pretrained("microsoft/layoutlmv3-base", num_labels=7)

>>> dataset = load_dataset("nielsr/funsd-layoutlmv3", split="train")
>>> example = dataset[0]
>>> image = example["image"]
>>> words = example["tokens"]
>>> boxes = example["bboxes"]
>>> word_labels = example["ner_tags"]

>>> encoding = processor(image, words, boxes=boxes, word_labels=word_labels, return_tensors="pt")

>>> outputs = model(**encoding)
>>> loss = outputs.loss
>>> logits = outputs.logits
```

## LayoutLMv3ForQuestionAnswering[[transformers.LayoutLMv3ForQuestionAnswering]]

- **config** ([LayoutLMv3ForQuestionAnswering](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ForQuestionAnswering)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Layoutlmv3 transformer with a span classification head on top for extractive question-answering tasks like
SQuAD (a linear layer on top of the hidden-states output to compute `span start logits` and `span end logits`).

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
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
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
- **bbox** (`torch.LongTensor` of shape `(batch_size, sequence_length, 4)`, *optional*) --
  Bounding boxes of each input sequence tokens. Selected in the range `[0,
  config.max_2d_position_embeddings-1]`. Each bounding box should be a normalized version in (x0, y0, x1, y1)
  format, where (x0, y0) corresponds to the position of the upper left corner in the bounding box, and (x1,
  y1) represents the position of the lower right corner.
- **pixel_values** (`torch.LongTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor). See `LayoutLMv3ImageProcessor.__call__()` for details ([LayoutLMv3Processor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Processor) uses
  [LayoutLMv3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ImageProcessor) for processing images).[QuestionAnsweringModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or `tuple(torch.FloatTensor)`A [QuestionAnsweringModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.QuestionAnsweringModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LayoutLMv3Config](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3Config)) and inputs.
The [LayoutLMv3ForQuestionAnswering](/docs/transformers/v5.14.0/en/model_doc/layoutlmv3#transformers.LayoutLMv3ForQuestionAnswering) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoProcessor, AutoModelForQuestionAnswering
>>> from datasets import load_dataset
>>> import torch

>>> processor = AutoProcessor.from_pretrained("microsoft/layoutlmv3-base", apply_ocr=False)
>>> model = AutoModelForQuestionAnswering.from_pretrained("microsoft/layoutlmv3-base")

>>> dataset = load_dataset("nielsr/funsd-layoutlmv3", split="train")
>>> example = dataset[0]
>>> image = example["image"]
>>> question = "what's his name?"
>>> words = example["tokens"]
>>> boxes = example["bboxes"]

>>> encoding = processor(image, question, words, boxes=boxes, return_tensors="pt")
>>> start_positions = torch.tensor([1])
>>> end_positions = torch.tensor([3])

>>> outputs = model(**encoding, start_positions=start_positions, end_positions=end_positions)
>>> loss = outputs.loss
>>> start_scores = outputs.start_logits
>>> end_scores = outputs.end_logits
```
