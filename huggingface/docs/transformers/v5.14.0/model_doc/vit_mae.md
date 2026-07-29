# ViTMAE

[ViTMAE](https://huggingface.co/papers/2111.06377) is a self-supervised vision model that is pretrained by masking large portions of an image (~75%). An encoder processes the visible image patches and a decoder reconstructs the missing pixels from the encoded patches and mask tokens. After pretraining, the encoder can be reused for downstream tasks like image classification or object detection — often outperforming models trained with supervised learning.

<img src="https://user-images.githubusercontent.com/11435359/146857310-f258c86c-fde6-48e8-9cee-badd2b21bd2c.png"
alt="drawing" width="600"/>

You can find all the original ViTMAE checkpoints under the [AI at Meta](https://huggingface.co/facebook?search_models=vit-mae) organization.

> [!TIP]
> Click on the ViTMAE models in the right sidebar for more examples of how to apply ViTMAE to vision tasks.

The example below demonstrates how to reconstruct the missing pixels with the [ViTMAEForPreTraining](/docs/transformers/v5.14.0/en/model_doc/vit_mae#transformers.ViTMAEForPreTraining) class.

```python
import requests
import torch
from PIL import Image

from transformers import ViTImageProcessor, ViTMAEForPreTraining

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)

processor = ViTImageProcessor.from_pretrained("facebook/vit-mae-base")
inputs = processor(image, return_tensors="pt").to(model.device)
inputs = {k: v.to(model.device) for k, v in inputs.items()}

model = ViTMAEForPreTraining.from_pretrained("facebook/vit-mae-base", attn_implementation="sdpa", device_map="auto")
with torch.no_grad():
    outputs = model(**inputs)

reconstruction = outputs.logits
```

## Notes

- ViTMAE is typically used in two stages. Self-supervised pretraining with [ViTMAEForPreTraining](/docs/transformers/v5.14.0/en/model_doc/vit_mae#transformers.ViTMAEForPreTraining), and then discarding the decoder and fine-tuning the encoder. After fine-tuning, the weights can be plugged into a model like [ViTForImageClassification](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTForImageClassification).
- Use [ViTImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vit#transformers.ViTImageProcessor) for input preparation.

## Resources

- Refer to this [notebook](https://github.com/NielsRogge/Transformers-Tutorials/blob/master/ViTMAE/ViT_MAE_visualization_demo.ipynb) to learn how to visualize the reconstructed pixels from [ViTMAEForPreTraining](/docs/transformers/v5.14.0/en/model_doc/vit_mae#transformers.ViTMAEForPreTraining).

## ViTMAEConfig[[transformers.ViTMAEConfig]]

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
- **decoder_num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **decoder_hidden_size** (`int`, *optional*, defaults to `512`) --
  Dimension of the hidden representations.
- **decoder_num_hidden_layers** (`int`, *optional*, defaults to 8) --
  Number of hidden layers in the decoder.
- **decoder_intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the MLP representations.
- **mask_ratio** (`float`, *optional*, defaults to 0.75) --
  The ratio of the number of masked tokens in the input sequence.
- **norm_pix_loss** (`bool`, *optional*, defaults to `False`) --
  Whether or not to train with normalized pixels (see Table 3 in the paper). Using normalized pixels improved
  representation quality in the experiments of the authors.

This is the configuration class to store the configuration of a ViTMAEModel. It is used to instantiate a Vit Mae
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/vit-mae-base](https://huggingface.co/facebook/vit-mae-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ViTMAEConfig, ViTMAEModel

>>> # Initializing a ViT MAE vit-mae-base style configuration
>>> configuration = ViTMAEConfig()

>>> # Initializing a model (with random weights) from the vit-mae-base style configuration
>>> model = ViTMAEModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ViTMAEModel[[transformers.ViTMAEModel]]

- **config** ([ViTMAEConfig](/docs/transformers/v5.14.0/en/model_doc/vit_mae#transformers.ViTMAEConfig)) --
  Configuration for the model.

The bare Vit Mae Model outputting raw hidden-states without any specific head on top.

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
- **noise** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mainly used for testing purposes to control randomness and maintain the reproducibility
- **interpolate_pos_encoding** (`bool`, *optional*, default `False`) --
  Whether to interpolate the pre-trained position encodings. This is mainly used to use the model on higher
  resolution images.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)`ViTMAEModelOutput` or `tuple(torch.FloatTensor)`A `ViTMAEModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ViTMAEConfig](/docs/transformers/v5.14.0/en/model_doc/vit_mae#transformers.ViTMAEConfig)) and inputs.
The [ViTMAEModel](/docs/transformers/v5.14.0/en/model_doc/vit_mae#transformers.ViTMAEModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) -- Tensor indicating which patches are masked (1) and which are not (0).
- **ids_restore** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) -- Tensor containing the original index of the (shuffled) masked patches.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, ViTMAEModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/vit-mae-base")
>>> model = ViTMAEModel.from_pretrained("facebook/vit-mae-base")

>>> inputs = image_processor(images=image, return_tensors="pt")
>>> outputs = model(**inputs)
>>> last_hidden_states = outputs.last_hidden_state
```

## ViTMAEForPreTraining[[transformers.ViTMAEForPreTraining]]

- **config** ([ViTMAEConfig](/docs/transformers/v5.14.0/en/model_doc/vit_mae#transformers.ViTMAEConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The ViTMAE Model transformer with the decoder on top for self-supervised pre-training.

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
- **noise** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mainly used for testing purposes to control randomness and maintain the reproducibility
- **interpolate_pos_encoding** (`bool`, *optional*, default `False`) --
  Whether to interpolate the pre-trained position encodings. This is mainly used to use the model on higher
  resolution images.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)`ViTMAEForPreTrainingOutput` or `tuple(torch.FloatTensor)`A `ViTMAEForPreTrainingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ViTMAEConfig](/docs/transformers/v5.14.0/en/model_doc/vit_mae#transformers.ViTMAEConfig)) and inputs.
The [ViTMAEForPreTraining](/docs/transformers/v5.14.0/en/model_doc/vit_mae#transformers.ViTMAEForPreTraining) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`) -- Pixel reconstruction loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, patch_size ** 2 * num_channels)`) -- Pixel reconstruction logits.
- **mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) -- Tensor indicating which patches are masked (1) and which are not (0).
- **ids_restore** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) -- Tensor containing the original index of the (shuffled) masked patches.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, ViTMAEForPreTraining
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read())).convert("RGB")

>>> image_processor = AutoImageProcessor.from_pretrained("facebook/vit-mae-base")
>>> model = ViTMAEForPreTraining.from_pretrained("facebook/vit-mae-base")

>>> inputs = image_processor(images=image, return_tensors="pt")
>>> outputs = model(**inputs)
>>> loss = outputs.loss
>>> mask = outputs.mask
>>> ids_restore = outputs.ids_restore
```
