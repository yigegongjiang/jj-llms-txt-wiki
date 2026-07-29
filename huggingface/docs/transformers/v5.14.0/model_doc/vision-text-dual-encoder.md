# VisionTextDualEncoder

## Overview

The [VisionTextDualEncoderModel](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderModel) can be used to initialize a vision-text dual encoder model with
any pretrained vision autoencoding model as the vision encoder (*e.g.* [ViT](vit), [BEiT](beit), [DeiT](deit)) and any pretrained text autoencoding model as the text encoder (*e.g.* [RoBERTa](roberta), [BERT](bert)). Two projection layers are added on top of both the vision and text encoder to project the output embeddings
to a shared latent space. The projection layers are randomly initialized so the model should be fine-tuned on a
downstream task. This model can be used to align the vision-text embeddings using CLIP like contrastive image-text
training and then can be used for zero-shot vision tasks such image-classification or retrieval.

In [LiT: Zero-Shot Transfer with Locked-image Text Tuning](https://huggingface.co/papers/2111.07991) it is shown how
leveraging pre-trained (locked/frozen) image and text model for contrastive learning yields significant improvement on
new zero-shot vision tasks such as image classification or retrieval.

## VisionTextDualEncoderConfig[[transformers.VisionTextDualEncoderConfig]]

- **projection_dim** (`int`, *optional*, defaults to `512`) --
  Dimensionality of text and vision projection layers.
- **logit_scale_init_value** (`Union[int, float]`, *optional*, defaults to `2.6592`) --
  The initial value of the *logit_scale* parameter.

This is the configuration class to store the configuration of a Vision Text Dual EncoderModel. It is used to instantiate a Vision Text Dual Encoder
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [None](https://huggingface.co/None)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import ViTConfig, BertConfig, VisionTextDualEncoderConfig, VisionTextDualEncoderModel

>>> # Initializing a BERT and ViT configuration
>>> config_vision = ViTConfig()
>>> config_text = BertConfig()

>>> config = VisionTextDualEncoderConfig.from_vision_text_configs(config_vision, config_text, projection_dim=512)

>>> # Initializing a BERT and ViT model (with random weights)
>>> model = VisionTextDualEncoderModel(config=config)

>>> # Accessing the model configuration
>>> config_vision = model.config.vision_config
>>> config_text = model.config.text_config

>>> # Saving the model, including its configuration
>>> model.save_pretrained("vit-bert")

>>> # loading model and config from pretrained folder
>>> vision_text_config = VisionTextDualEncoderConfig.from_pretrained("vit-bert")
>>> model = VisionTextDualEncoderModel.from_pretrained("vit-bert", config=vision_text_config)
```

[VisionTextDualEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderConfig)An instance of a configuration object

Instantiate a [VisionTextDualEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderConfig) (or a derived class) from text model configuration and vision
model configuration.

## VisionTextDualEncoderProcessor[[transformers.VisionTextDualEncoderProcessor]]

- **image_processor** (`image_processor_class`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
Constructs a VisionTextDualEncoderProcessor which wraps a image processor and a tokenizer into a single processor.

[VisionTextDualEncoderProcessor](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderProcessor) offers all the functionalities of `image_processor_class` and `tokenizer_class`. See the
`~image_processor_class` and `~tokenizer_class` for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **videos** (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) --
  Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If
  passing in videos with pixel values between 0 and 1, set `do_rescale=False`.
- **audio** (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) --
  The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor.
  In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels,
  and T is the sample length of the audio.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.

## VisionTextDualEncoderModel[[transformers.VisionTextDualEncoderModel]]

- **config** ([VisionTextDualEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderConfig), *optional*) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
- **vision_model** (`~modeling_utils.PreTrainedModel`, *optional*) --
  The vision model to use.
- **text_model** (`~modeling_utils.PreTrainedModel`, *optional*) --
  The text model to use.

The bare Vision Text Dual Encoder Model outputting raw hidden-states without any specific head on top.

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
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details ([VisionTextDualEncoderProcessor](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderProcessor) uses
  `image_processor_class` for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **return_loss** (`bool`, *optional*) --
  Whether or not to return the contrastive loss.
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`CLIPOutput` or `tuple(torch.FloatTensor)`A `CLIPOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VisionTextDualEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderConfig)) and inputs.
The [VisionTextDualEncoderModel](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Contrastive loss for image-text similarity.
- **logits_per_image** (`torch.FloatTensor` of shape `(image_batch_size, text_batch_size)`) -- The scaled dot product scores between `image_embeds` and `text_embeds`. This represents the image-text
  similarity scores.
- **logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, image_batch_size)`) -- The scaled dot product scores between `text_embeds` and `image_embeds`. This represents the text-image
  similarity scores.
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The text embeddings obtained by applying the projection layer to the pooled output of [CLIPTextModel](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPTextModel).
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The image embeddings obtained by applying the projection layer to the pooled output of [CLIPVisionModel](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPVisionModel).
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [CLIPTextModel](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPTextModel).
- **vision_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [CLIPVisionModel](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPVisionModel).

Examples:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import (
...     VisionTextDualEncoderModel,
...     VisionTextDualEncoderProcessor,
...     AutoImageProcessor,
...     AutoTokenizer,
... )

>>> tokenizer = AutoTokenizer.from_pretrained("google-bert/bert-base-uncased")
>>> image_processor = AutoImageProcessor.from_pretrained("google/vit-base-patch16-224")
>>> processor = VisionTextDualEncoderProcessor(image_processor, tokenizer)
>>> model = VisionTextDualEncoderModel.from_vision_text_pretrained(
...     "google/vit-base-patch16-224", "google-bert/bert-base-uncased"
... )

>>> # contrastive training
>>> urls = [
...     "http://images.cocodataset.org/val2017/000000039769.jpg",
...     "https://farm3.staticflickr.com/2674/5850229113_4fe05d5265_z.jpg",
... ]
>>> with httpx.stream("GET", urls[0]) as response:
...     image1 = Image.open(BytesIO(response.read()))

>>> with httpx.stream("GET", urls[1]) as response:
...     image2 = Image.open(BytesIO(response.read()))

>>> images = [image1, image2]

>>> inputs = processor(
...     text=["a photo of a cat", "a photo of a dog"], images=images, return_tensors="pt", padding=True
... )
>>> outputs = model(
...     input_ids=inputs.input_ids,
...     attention_mask=inputs.attention_mask,
...     pixel_values=inputs.pixel_values,
...     return_loss=True,
... )
>>> loss, logits_per_image = outputs.loss, outputs.logits_per_image  # this is the image-text similarity score

>>> # save and load from pretrained
>>> model.save_pretrained("vit-bert")
>>> model = VisionTextDualEncoderModel.from_pretrained("vit-bert")

>>> # inference
>>> outputs = model(**inputs)
>>> logits_per_image = outputs.logits_per_image  # this is the image-text similarity score
>>> probs = logits_per_image.softmax(dim=1)  # we can take the softmax to get the label probabilities
```

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "position_ids", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "token_type_ids", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_ids** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **token_type_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VisionTextDualEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderConfig)) and inputs.

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
>>> import torch
>>> from transformers import VisionTextDualEncoderModel, AutoTokenizer

>>> model = VisionTextDualEncoderModel.from_pretrained("clip-italian/clip-italian")
>>> tokenizer = AutoTokenizer.from_pretrained("clip-italian/clip-italian")

>>> inputs = tokenizer(["una foto di un gatto", "una foto di un cane"], padding=True, return_tensors="pt")
>>> with torch.inference_mode():
...     text_features = model.get_text_features(**inputs)
```

)>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details ([VisionTextDualEncoderProcessor](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderProcessor) uses
  `image_processor_class` for processing images).[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VisionTextDualEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/vision-text-dual-encoder#transformers.VisionTextDualEncoderConfig)) and inputs.

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
>>> import torch
>>> from transformers import VisionTextDualEncoderModel, AutoImageProcessor
>>> from transformers.image_utils import load_image

>>> model = VisionTextDualEncoderModel.from_pretrained("clip-italian/clip-italian")
>>> image_processor = AutoImageProcessor.from_pretrained("google/vit-base-patch16-224")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = load_image(url)

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> with torch.inference_mode():
...     image_features = model.get_image_features(**inputs)
```
