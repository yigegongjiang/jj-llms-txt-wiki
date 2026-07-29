# Vision Transformer (ViT)

[Vision Transformer (ViT)](https://huggingface.co/papers/2010.11929) is a transformer adapted for computer vision tasks. An image is split into smaller fixed-sized patches which are treated as a sequence of tokens, similar to words for NLP tasks. ViT requires less resources to pretrain compared to convolutional architectures and its performance on large datasets can be transferred to smaller downstream tasks.

You can find all the original ViT checkpoints under the [Google](https://huggingface.co/google?search_models=vit) organization.

> [!TIP]
> Click on the ViT models in the right sidebar for more examples of how to apply ViT to different computer vision tasks.

The example below demonstrates how to classify an image with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipeline = pipeline(
    task="image-classification",
    model="google/vit-base-patch16-224",
    device=0
)
pipeline("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForImageClassification

image_processor = AutoImageProcessor.from_pretrained(
    "google/vit-base-patch16-224",
    use_fast=True,
)
model = AutoModelForImageClassification.from_pretrained(
    "google/vit-base-patch16-224",
    device_map="auto",
    attn_implementation="sdpa"
)
url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
inputs = image_processor(image, return_tensors="pt").to(model.device)

with torch.no_grad():
  logits = model(**inputs).logits
predicted_class_id = logits.argmax(dim=-1).item()

class_labels = model.config.id2label
predicted_class_label = class_labels[predicted_class_id]
print(f"The predicted class label is: {predicted_class_label}")
```

## Notes

- The best results are obtained with supervised pretraining, and during fine-tuning, it may be better to use images with a resolution higher than 224x224.
- Use [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) to resize (or rescale) and normalize images to the expected size.
- The patch and image resolution are reflected in the checkpoint name. For example, google/vit-base-patch16-224, is the **base-sized** architecture with a patch resolution of 16x16 and fine-tuning resolution of 224x224.

## ViTConfig[[transformers.ViTConfig]]

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
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-12`) --
  The epsilon used by the layer normalization layers.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **encoder_stride** (`int`, *optional*, defaults to 16) --
  Factor to increase the spatial resolution by in the decoder head for masked image modeling.
- **pooler_output_size** (`int`, *optional*) --
  Dimensionality of the pooler layer. If None, defaults to `hidden_size`.
- **pooler_act** (`str`, *optional*, defaults to `"tanh"`) --
  The activation function to be used by the pooler.

This is the configuration class to store the configuration of a ViTModel. It is used to instantiate a Vit
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/vit-base-patch16-224](https://huggingface.co/google/vit-base-patch16-224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ViTConfig, ViTModel

>>> # Initializing a ViT vit-base-patch16-224 style configuration
>>> configuration = ViTConfig()

>>> # Initializing a model (with random weights) from the vit-base-patch16-224 style configuration
>>> model = ViTModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ViTImageProcessor[[transformers.ViTImageProcessor]]

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## ViTImageProcessorPil[[transformers.ViTImageProcessorPil]]

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## ViTModel[[transformers.ViTModel]]

- **config** ([ViTConfig](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
- **add_pooling_layer** (`bool`, *optional*, defaults to `True`) --
  Whether to add a pooling layer
- **use_mask_token** (`bool`, *optional*, defaults to `False`) --
  Whether to use a mask token for masked image modeling.

The bare Vit Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor). See `ViTImageProcessor.__call__()` for details (`processor_class` uses
  [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) for processing images).
- **bool_masked_pos** (`torch.BoolTensor` of shape `(batch_size, num_patches)`, *optional*) --
  Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).
- **interpolate_pos_encoding** (`bool`, *optional*) --
  Whether to interpolate the pre-trained position encodings.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ViTConfig](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTConfig)) and inputs.
The [ViTModel](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTModel) forward method, overrides the `__call__` special method.

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

Example:

```python
```

## ViTForMaskedImageModeling[[transformers.ViTForMaskedImageModeling]]

- **config** ([ViTConfig](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ViT Model with a decoder on top for masked image modeling, as proposed in [SimMIM](https://huggingface.co/papers/2111.09886).

Note that we provide a script to pre-train this model on custom data in our [examples
directory](https://github.com/huggingface/transformers/tree/main/examples/pytorch/image-pretraining).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor). See `ViTImageProcessor.__call__()` for details (`processor_class` uses
  [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) for processing images).
- **bool_masked_pos** (`torch.BoolTensor` of shape `(batch_size, num_patches)`) --
  Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).
- **interpolate_pos_encoding** (`bool`, *optional*) --
  Whether to interpolate the pre-trained position encodings.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)`MaskedImageModelingOutput` or `tuple(torch.FloatTensor)`A `MaskedImageModelingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ViTConfig](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTConfig)) and inputs.
The [ViTForMaskedImageModeling](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTForMaskedImageModeling) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `bool_masked_pos` is provided) -- Reconstruction loss.
- **reconstruction** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Reconstructed / completed images.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or
  when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when
  `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`. Attentions weights after the attention softmax, used to compute the weighted average in
  the self-attention heads.

Examples:
```python
>>> from transformers import AutoImageProcessor, ViTForMaskedImageModeling
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("google/vit-base-patch16-224-in21k")
>>> model = ViTForMaskedImageModeling.from_pretrained("google/vit-base-patch16-224-in21k")

>>> num_patches = (model.config.image_size // model.config.patch_size) ** 2
>>> pixel_values = image_processor(images=image, return_tensors="pt").pixel_values
>>> # create random boolean mask of shape (batch_size, num_patches)
>>> bool_masked_pos = torch.randint(low=0, high=2, size=(1, num_patches)).bool()

>>> outputs = model(pixel_values, bool_masked_pos=bool_masked_pos)
>>> loss, reconstructed_pixel_values = outputs.loss, outputs.reconstruction
>>> list(reconstructed_pixel_values.shape)
[1, 3, 224, 224]
```

## ViTForImageClassification[[transformers.ViTForImageClassification]]

- **config** ([ViTConfig](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ViT Model transformer with an image classification head on top (a linear layer on top of the final hidden state of
the [CLS] token) e.g. for ImageNet.

Note that it's possible to fine-tune ViT on higher resolution images than the ones it has been trained on, by
setting `interpolate_pos_encoding` to `True` in the forward of the model. This will interpolate the pre-trained
position embeddings to the higher resolution.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor). See `ViTImageProcessor.__call__()` for details (`processor_class` uses
  [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the image classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).
- **interpolate_pos_encoding** (`bool`, *optional*) --
  Whether to interpolate the pre-trained position encodings.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)[ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or `tuple(torch.FloatTensor)`A [ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ViTConfig](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTConfig)) and inputs.
The [ViTForImageClassification](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoImageProcessor, ViTForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("google/vit-base-patch16-224")
>>> model = ViTForImageClassification.from_pretrained("google/vit-base-patch16-224")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
