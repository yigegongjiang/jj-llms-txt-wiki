# I-JEPA

[I-JEPA](https://huggingface.co/papers/2301.08243) is a self-supervised learning method that learns semantic image representations by predicting parts of an image from other parts of the image. It compares the abstract representations of the image (rather than pixel level comparisons), which avoids the typical pitfalls of data augmentation bias and pixel-level details that don't capture semantic meaning.

You can find the original I-JEPA checkpoints under the [AI at Meta](https://huggingface.co/facebook/models?search=ijepa) organization.
> [!TIP]
> This model was contributed by [jmtzt](https://huggingface.co/jmtzt).

> Click on the I-JEPA models in the right sidebar for more examples of how to apply I-JEPA to different image representation and classification tasks.

The example below demonstrates how to extract image features with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

feature_extractor = pipeline(
    task="image-feature-extraction",
    model="facebook/ijepa_vith14_1k",
    device=0,
)
features = feature_extractor("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg", return_tensors=True).to(model.device)

print(f"Feature shape: {features.shape}")
```

```python
import requests
from PIL import Image
from torch.nn.functional import cosine_similarity

from transformers import AutoModel, AutoProcessor

url_1 = "http://images.cocodataset.org/val2017/000000039769.jpg"
url_2 = "http://images.cocodataset.org/val2017/000000219578.jpg"
image_1 = Image.open(requests.get(url_1, stream=True).raw)
image_2 = Image.open(requests.get(url_2, stream=True).raw)

processor = AutoProcessor.from_pretrained("facebook/ijepa_vith14_1k")
model = AutoModel.from_pretrained("facebook/ijepa_vith14_1k", attn_implementation="sdpa", device_map="auto")

def infer(image):
    inputs = processor(image, return_tensors="pt").to(model.device)
    outputs = model(**inputs)
    return outputs.last_hidden_state.mean(dim=1)

embed_1 = infer(image_1)
embed_2 = infer(image_2)

similarity = cosine_similarity(embed_1, embed_2)
print(similarity)
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.
The example below uses [bitsandbytes](../quantization/bitsandbytes) to only quantize the weights to 4-bits.

```python
from transformers import AutoModel, AutoProcessor, BitsAndBytesConfig

quantization_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4",
    bit_use_double_quant=True,
)

url_1 = "http://images.cocodataset.org/val2017/000000039769.jpg"
url_2 = "http://images.cocodataset.org/val2017/000000219578.jpg"
image_1 = Image.open(requests.get(url_1, stream=True).raw)
image_2 = Image.open(requests.get(url_2, stream=True).raw)

processor = AutoProcessor.from_pretrained("facebook/ijepa_vitg16_22k")
model = AutoModel.from_pretrained("facebook/ijepa_vitg16_22k", quantization_config=quantization_config, attn_implementation="sdpa", device_map="auto")

def infer(image):
    inputs = processor(image, return_tensors="pt").to(model.device)
    outputs = model(**inputs)
    return outputs.last_hidden_state.mean(dim=1)

embed_1 = infer(image_1)
embed_2 = infer(image_2)

similarity = cosine_similarity(embed_1, embed_2)
print(similarity)
```

## IJepaConfig[[transformers.IJepaConfig]]

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
- **pooler_output_size** (`int`, *optional*) --
  Dimensionality of the pooler layer. If None, defaults to `hidden_size`.
- **pooler_act** (`str`, *optional*, defaults to `"tanh"`) --
  The activation function to be used by the pooler.

This is the configuration class to store the configuration of a IJepaModel. It is used to instantiate a Ijepa
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/ijepa_vith14_1k](https://huggingface.co/facebook/ijepa_vith14_1k)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import IJepaConfig, IJepaModel

>>> # Initializing a IJEPA ijepa-base-patch16-224 style configuration
>>> configuration = IJepaConfig()

>>> # Initializing a model (with random weights) from the ijepa-base-patch16-224 style configuration
>>> model = IJepaModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## IJepaModel[[transformers.IJepaModel]]

- **config** ([IJepaConfig](/docs/transformers/v5.14.0/en/model_doc/ijepa#transformers.IJepaConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
- **add_pooling_layer** (`bool`, *optional*, defaults to `True`) --
  Whether to add a pooling layer
- **use_mask_token** (`bool`, *optional*, defaults to `False`) --
  Whether to use a mask token for masked image modeling.

The bare Ijepa Model outputting raw hidden-states without any specific head on top.

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
elements depending on the configuration ([IJepaConfig](/docs/transformers/v5.14.0/en/model_doc/ijepa#transformers.IJepaConfig)) and inputs.
The [IJepaModel](/docs/transformers/v5.14.0/en/model_doc/ijepa#transformers.IJepaModel) forward method, overrides the `__call__` special method.

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

## IJepaForImageClassification[[transformers.IJepaForImageClassification]]

- **config** ([IJepaConfig](/docs/transformers/v5.14.0/en/model_doc/ijepa#transformers.IJepaConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

IJepa Model transformer with an image classification head on top (a linear layer on top of the final hidden states)
e.g. for ImageNet.

Note that it's possible to fine-tune IJepa on higher resolution images than the ones it has been trained on, by
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
  Whether to interpolate the pre-trained position encodings.[ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or `tuple(torch.FloatTensor)`A [ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([IJepaConfig](/docs/transformers/v5.14.0/en/model_doc/ijepa#transformers.IJepaConfig)) and inputs.
The [IJepaForImageClassification](/docs/transformers/v5.14.0/en/model_doc/ijepa#transformers.IJepaForImageClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, IJepaForImageClassification
>>> import torch
>>> from datasets import load_dataset

>>> dataset = load_dataset("huggingface/cats-image")
>>> image = dataset["test"]["image"][0]

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/ijepa_vith14_1k")
>>> model = IJepaForImageClassification.from_pretrained("facebook/ijepa_vith14_1k")

>>> inputs = image_processor(image, return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # model predicts one of the 1000 ImageNet classes
>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])
...
```
