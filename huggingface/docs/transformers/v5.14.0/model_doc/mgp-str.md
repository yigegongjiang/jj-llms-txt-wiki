# MGP-STR

## Overview

The MGP-STR model was proposed in [Multi-Granularity Prediction for Scene Text Recognition](https://huggingface.co/papers/2209.03592) by Peng Wang, Cheng Da, and Cong Yao. MGP-STR is a conceptually **simple** yet **powerful** vision Scene Text Recognition (STR) model, which is built upon the [Vision Transformer (ViT)](vit). To integrate linguistic knowledge, Multi-Granularity Prediction (MGP) strategy is proposed to inject information from the language modality into the model in an implicit way.

The abstract from the paper is the following:

*Scene text recognition (STR) has been an active research topic in computer vision for years. To tackle this challenging problem, numerous innovative methods have been successively proposed and incorporating linguistic knowledge into STR models has recently become a prominent trend. In this work, we first draw inspiration from the recent progress in Vision Transformer (ViT) to construct a conceptually simple yet powerful vision STR model, which is built upon ViT and outperforms previous state-of-the-art models for scene text recognition, including both pure vision models and language-augmented methods. To integrate linguistic knowledge, we further propose a Multi-Granularity Prediction strategy to inject information from the language modality into the model in an implicit way, i.e. , subword representations (BPE and WordPiece) widely-used in NLP are introduced into the output space, in addition to the conventional character level representation, while no independent language model (LM) is adopted. The resultant algorithm (termed MGP-STR) is able to push the performance envelop of STR to an even higher level. Specifically, it achieves an average recognition accuracy of 93.35% on standard benchmarks.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/mgp_str_architecture.png"
alt="drawing" width="600"/>

 MGP-STR architecture. Taken from the original paper. 

MGP-STR is trained on two synthetic datasets [MJSynth](http://www.robots.ox.ac.uk/~vgg/data/text/) (MJ) and [SynthText](http://www.robots.ox.ac.uk/~vgg/data/scenetext/) (ST) without fine-tuning on other datasets. It achieves state-of-the-art results on six standard Latin scene text benchmarks, including 3 regular text datasets (IC13, SVT, IIIT) and 3 irregular ones (IC15, SVTP, CUTE).
This model was contributed by [yuekun](https://huggingface.co/yuekun). The original code can be found [here](https://github.com/AlibabaResearch/AdvancedLiterateMachinery/tree/main/OCR/MGP-STR).

## Inference example

[MgpstrModel](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrModel) accepts images as input and generates three types of predictions, which represent textual information at different granularities.
The three types of predictions are fused to give the final prediction result.

The [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) class is responsible for preprocessing the input image and
[MgpstrTokenizer](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrTokenizer) decodes the generated character tokens to the target string. The
[MgpstrProcessor](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrProcessor) wraps [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) and [MgpstrTokenizer](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrTokenizer)
into a single instance to both extract the input features and decode the predicted token ids.

- Step-by-step Optical Character Recognition (OCR)

```python
import requests
from PIL import Image

from transformers import MgpstrForSceneTextRecognition, MgpstrProcessor

processor = MgpstrProcessor.from_pretrained('alibaba-damo/mgp-str-base')
model = MgpstrForSceneTextRecognition.from_pretrained('alibaba-damo/mgp-str-base', device_map="auto")

# load image from the IIIT-5k dataset
url = "https://i.postimg.cc/ZKwLg2Gw/367-14.png"
image = Image.open(requests.get(url, stream=True).raw).convert("RGB")

pixel_values = processor(images=image, return_tensors="pt").to(model.device).pixel_values
outputs = model(pixel_values)

generated_text = processor.batch_decode(outputs.logits)['generated_text']
```

## MgpstrConfig[[transformers.MgpstrConfig]]

- **image_size** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(32, 128)`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `4`) --
  The size (resolution) of each patch.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **max_token_length** (`int`, *optional*, defaults to 27) --
  The max number of output tokens.
- **num_character_labels** (`int`, *optional*, defaults to 38) --
  The number of classes for character head .
- **num_bpe_labels** (`int`, *optional*, defaults to 50257) --
  The number of classes for bpe head .
- **num_wordpiece_labels** (`int`, *optional*, defaults to 30522) --
  The number of classes for wordpiece head .
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **mlp_ratio** (`Union[float, int]`, *optional*, defaults to `4.0`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **distilled** (`bool`, *optional*, defaults to `False`) --
  Model includes a distillation token and head as in DeiT models.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **drop_rate** (`float`, *optional*, defaults to 0.0) --
  The dropout probability for all fully connected layers in the embeddings, encoder.
- **attn_drop_rate** (`float`, *optional*, defaults to 0.0) --
  The dropout ratio for the attention probabilities.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  Drop path rate for the patch fusion.
- **output_a3_attentions** (`bool`, *optional*, defaults to `False`) --
  Whether or not the model should returns A^3 module attentions.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Mgp StrModel. It is used to instantiate a Mgp Str
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [alibaba-damo/mgp-str-base](https://huggingface.co/alibaba-damo/mgp-str-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import MgpstrConfig, MgpstrForSceneTextRecognition

>>> # Initializing a Mgpstr mgp-str-base style configuration
>>> configuration = MgpstrConfig()

>>> # Initializing a model (with random weights) from the mgp-str-base style configuration
>>> model = MgpstrForSceneTextRecognition(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MgpstrTokenizer[[transformers.MgpstrTokenizer]]

- **vocab_file** (`str`) --
  Path to the vocabulary file.
- **unk_token** (`str`, *optional*, defaults to `"[GO]"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **bos_token** (`str`, *optional*, defaults to `"[GO]"`) --
  The beginning of sequence token.
- **eos_token** (`str`, *optional*, defaults to `"[s]"`) --
  The end of sequence token.
- **pad_token** (`str` or `tokenizers.AddedToken`, *optional*, defaults to `"[GO]"`) --
  A special token used to make arrays of tokens the same size for batching purpose. Will then be ignored by
  attention mechanisms or loss computation.

Construct a MGP-STR char tokenizer.

This tokenizer inherits from [PreTrainedTokenizer](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.PythonBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods.

## MgpstrProcessor[[transformers.MgpstrProcessor]]

- **image_processor** (`ViTImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`MgpstrTokenizer`) --
  The tokenizer is a required input.
Constructs a MgpstrProcessor which wraps a image processor and a tokenizer into a single processor.

[MgpstrProcessor](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrProcessor) offers all the functionalities of [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) and [MgpstrTokenizer](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrTokenizer). See the
[~ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) and [~MgpstrTokenizer](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrTokenizer) for more information.

- **text** (``) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **images** (``) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (``) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.

- **sequences** (`torch.Tensor`) --
  List of tokenized input ids.`dict[str, any]`Dictionary of all the outputs of the decoded results.
generated_text (`list[str]`): The final results after fusion of char, bpe, and wp. scores
(`list[float]`): The final scores after fusion of char, bpe, and wp. char_preds (`list[str]`): The list
of character decoded sentences. bpe_preds (`list[str]`): The list of bpe decoded sentences. wp_preds
(`list[str]`): The list of wp decoded sentences.

Convert a list of lists of token ids into a list of strings by calling decode.

This method forwards all its arguments to PreTrainedTokenizer's [batch_decode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.batch_decode). Please
refer to the docstring of this method for more information.

## MgpstrModel[[transformers.MgpstrModel]]

- **config** ([MgpstrConfig](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Mgp Str Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor). See `ViTImageProcessor.__call__()` for details ([MgpstrProcessor](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrProcessor) uses
  [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) for processing images).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MgpstrConfig](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrConfig)) and inputs.
The [MgpstrModel](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrModel) forward method, overrides the `__call__` special method.

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

## MgpstrForSceneTextRecognition[[transformers.MgpstrForSceneTextRecognition]]

- **config** ([MgpstrConfig](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MGP-STR Model transformer with three classification heads on top (three A^3 modules and three linear layer on top
of the transformer encoder output) for scene text recognition (STR) .

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor). See `ViTImageProcessor.__call__()` for details ([MgpstrProcessor](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrProcessor) uses
  [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) for processing images).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_a3_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of a3 modules. See `a3_attentions` under returned tensors
  for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`MgpstrModelOutput` or `tuple(torch.FloatTensor)`A `MgpstrModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MgpstrConfig](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrConfig)) and inputs.
The [MgpstrForSceneTextRecognition](/docs/transformers/v5.14.0/en/model_doc/mgp-str#transformers.MgpstrForSceneTextRecognition) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **logits** (`tuple(torch.FloatTensor)` of shape `(batch_size, config.num_character_labels)`) -- Tuple of `torch.FloatTensor` (one for the output of character of shape `(batch_size,
  config.max_token_length, config.num_character_labels)`, + one for the output of bpe of shape `(batch_size,
  config.max_token_length, config.num_bpe_labels)`, + one for the output of wordpiece of shape `(batch_size,
  config.max_token_length, config.num_wordpiece_labels)`) .

  Classification scores (before SoftMax) of character, bpe and wordpiece.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **a3_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_a3_attentions=True` is passed or when `config.output_a3_attentions=True`) -- Tuple of `torch.FloatTensor` (one for the attention of character, + one for the attention of bpe`, + one
  for the attention of wordpiece) of shape `(batch_size, config.max_token_length, sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import (
...     MgpstrProcessor,
...     MgpstrForSceneTextRecognition,
... )
>>> import httpx
>>> from io import BytesIO
>>> from PIL import Image

>>> # load image from the IIIT-5k dataset
>>> url = "https://i.postimg.cc/ZKwLg2Gw/367-14.png"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read())).convert("RGB")

>>> processor = MgpstrProcessor.from_pretrained("alibaba-damo/mgp-str-base")
>>> pixel_values = processor(images=image, return_tensors="pt").pixel_values

>>> model = MgpstrForSceneTextRecognition.from_pretrained("alibaba-damo/mgp-str-base")

>>> # inference
>>> outputs = model(pixel_values)
>>> out_strs = processor.batch_decode(outputs.logits)
>>> out_strs["generated_text"]
'["ticket"]'
```
