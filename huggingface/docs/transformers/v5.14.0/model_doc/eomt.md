# EoMT

## Overview

[The Encoder-only Mask Transformer]((https://www.tue-mps.org/eomt)) (EoMT) model was introduced in the CVPR 2025 Highlight Paper *[Your ViT is Secretly an Image Segmentation Model](https://huggingface.co/papers/2503.19108)* by Tommie Kerssies, Niccolò Cavagnero, Alexander Hermans, Narges Norouzi, Giuseppe Averta, Bastian Leibe, Gijs Dubbelman, and Daan de Geus.
EoMT reveals Vision Transformers can perform image segmentation efficiently without task-specific components.

The abstract from the paper is the following:

*Vision Transformers (ViTs) have shown remarkable performance and scalability across various computer vision tasks. To apply single-scale ViTs to image segmentation, existing methods adopt a convolutional adapter to generate multi-scale features, a pixel decoder to fuse these features, and a Transformer decoder that uses the fused features to make predictions. In this paper, we show that the inductive biases introduced by these task-specific components can instead be learned by the ViT itself, given sufficiently large models and extensive pre-training. Based on these findings, we introduce the Encoder-only Mask Transformer (EoMT), which repurposes the plain ViT architecture to conduct image segmentation. With large-scale models and pre-training, EoMT obtains a segmentation accuracy similar to state-of-the-art models that use task-specific components. At the same time, EoMT is significantly faster than these methods due to its architectural simplicity, e.g., up to 4x faster with ViT-L. Across a range of model sizes, EoMT demonstrates an optimal balance between segmentation accuracy and prediction speed, suggesting that compute resources are better spent on scaling the ViT itself rather than adding architectural complexity.*

This model was contributed by [Yaswanth Gali](https://huggingface.co/yaswanthgali).
The original code can be found [here](https://github.com/tue-mps/eomt).

## Architecture Info

The `EoMT` model uses a DINOv2-pretrained Vision Transformer with **register tokens** as its backbone. EoMT simplifies the segmentation pipeline by relying solely on the encoder, eliminating the need for task-specific decoders commonly used in prior approaches.

Architecturally, EoMT introduces a small set of **learned queries** and a lightweight **mask prediction module**. These queries are injected into the final encoder blocks, enabling **joint attention** between image patches and object queries. During training, **masked attention** is applied to constrain each query to focus on its corresponding region—effectively mimicking cross-attention. This constraint is gradually phased out via a **mask annealing strategy**, allowing for **efficient, decoder-free inference** without compromising segmentation performance.

  <img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/eomt_architecture.png"
       alt="drawing" width="500"/>

The model supports semantic, instance, and panoptic segmentation using a unified architecture and task-specific post-processing.

## Usage Examples

Use the Hugging Face implementation of EoMT for inference with pre-trained models.

### Semantic Segmentation

The EoMT model performs semantic segmentation using sliding-window inference. The input image is resized such that the shorter side matches the target input size, then it is split into overlapping crops. Each crop is then passed through the model. After inference, the predicted logits from each crop are stitched back together and rescaled to the original image size to get the final segmentation mask.

> **Note:**  
> If you want to use a custom target size for **semantic segmentation**, specify it in the following format:  
> `{"shortest_edge": 512}`  
> Notice that `longest_edge` is not provided here — this is intentional. For semantic segmentation, images are typically **scaled so that the shortest edge is greater than or equal to the target size** hence longest_edge is not necessary.

```python
import matplotlib.pyplot as plt
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, EomtForUniversalSegmentation

model_id = "tue-mps/ade20k_semantic_eomt_large_512"
processor = AutoImageProcessor.from_pretrained(model_id)
model = EomtForUniversalSegmentation.from_pretrained(model_id, device_map="auto")

image = Image.open(requests.get("http://images.cocodataset.org/val2017/000000039769.jpg", stream=True).raw)

inputs = processor(
    images=image,
    return_tensors="pt",
)

with torch.inference_mode():
    outputs = model(**inputs)

# Prepare the original image size in the format (height, width)
target_sizes = [(image.height, image.width)]

# Post-process the model outputs to get final segmentation prediction
preds = processor.post_process_semantic_segmentation(
    outputs,
    target_sizes=target_sizes,
)

# Visualize the segmentation mask
plt.imshow(preds[0])
plt.axis("off")
plt.title("Semantic Segmentation")
plt.show()
```

### Instance Segmentation

The EoMT model performs instance segmentation using padded inference. The input image is resized so that the longer side matches the target input size, and the shorter side is zero-padded to form a square. The resulting mask and class logits are combined through post-processing (adapted from Mask2Former) to produce a unified instance segmentation map, along with segment metadata like segment id, class labels and confidence scores.

> **Note:**  
> To use a custom target size, specify the size as a dictionary in the following format:  
> `{"shortest_edge": 512, "longest_edge": 512}`  
> For both instance and panoptic segmentation, input images will be **scaled and padded** to this target size.

```python
import matplotlib.pyplot as plt
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, EomtForUniversalSegmentation

model_id = "tue-mps/coco_instance_eomt_large_640"
processor = AutoImageProcessor.from_pretrained(model_id)
model = EomtForUniversalSegmentation.from_pretrained(model_id, device_map="auto")

image = Image.open(requests.get("http://images.cocodataset.org/val2017/000000039769.jpg", stream=True).raw)

inputs = processor(
    images=image,
    return_tensors="pt",
)

with torch.inference_mode():
    outputs = model(**inputs)

# Prepare the original image size in the format (height, width)
target_sizes = [(image.height, image.width)]

# Post-process the model outputs to get final segmentation prediction
preds = processor.post_process_instance_segmentation(
    outputs,
    target_sizes=target_sizes,
)

# Visualize the segmentation mask
plt.imshow(preds[0]["segmentation"])
plt.axis("off")
plt.title("Instance Segmentation")
plt.show()
```

### Panoptic Segmentation

The EoMT model performs panoptic segmentation using the same padded inference strategy as in instance segmentation. After padding and normalization, the model predicts both thing (instances) and stuff (amorphous regions) classes. The resulting mask and class logits are combined through post-processing (adapted from Mask2Former) to produce a unified panoptic segmentation map, along with segment metadata like segment id, class labels and confidence scores.

```python
import matplotlib.pyplot as plt
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, EomtForUniversalSegmentation

model_id = "tue-mps/coco_panoptic_eomt_large_640"
processor = AutoImageProcessor.from_pretrained(model_id)
model = EomtForUniversalSegmentation.from_pretrained(model_id, device_map="auto")

image = Image.open(requests.get("http://images.cocodataset.org/val2017/000000039769.jpg", stream=True).raw)

inputs = processor(
    images=image,
    return_tensors="pt",
)

with torch.inference_mode():
    outputs = model(**inputs)

# Prepare the original image size in the format (height, width)
target_sizes = [(image.height, image.width)]

# Post-process the model outputs to get final segmentation prediction
preds = processor.post_process_panoptic_segmentation(
    outputs,
    target_sizes=target_sizes,
)

# Visualize the panoptic segmentation mask
plt.imshow(preds[0]["segmentation"])
plt.axis("off")
plt.title("Panoptic Segmentation")
plt.show()
```

## EomtImageProcessor[[transformers.EomtImageProcessor]]

- **do_split_image** (`bool`, *kwargs*, *optional*, defaults to `self.do_split_image`) --
  Whether to split the input images into overlapping patches for semantic segmentation. If set to `True`, the
  input images will be split into patches of size `size["shortest_edge"]` with an overlap between patches.
  Otherwise, the input images will be padded to the target size.
- **ignore_index** (`int`, *kwargs*, *optional*, defaults to `self.ignore_index`) --
  Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels
  denoted with 0 (background) will be replaced with `ignore_index`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a EomtImageProcessor image processor.

)>] | None = None"}, {"name": "instance_id_to_semantic_id", "val": ": dict[int, int] | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess for corresponding images.
- **instance_id_to_semantic_id** (`list[dict[int, int]]` or `dict[int, int]`, *optional*) --
  A mapping between object instance ids and class ids.
- **do_split_image** (`bool`, *kwargs*, *optional*, defaults to `self.do_split_image`) --
  Whether to split the input images into overlapping patches for semantic segmentation. If set to `True`, the
  input images will be split into patches of size `size["shortest_edge"]` with an overlap between patches.
  Otherwise, the input images will be padded to the target size.
- **ignore_index** (`int`, *kwargs*, *optional*, defaults to `self.ignore_index`) --
  Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels
  denoted with 0 (background) will be replaced with `ignore_index`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** ([EomtForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtForUniversalSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple[int, int]]`) --
  A list of tuples (`tuple[int, int]`) containing the target size (height, width) of each image in the
  batch.
- **size** (`dict[str, int]`, *optional*) --
  The size to which the intermediate masks are interpolated. Defaults to `self.size`.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size.

Converts the output of [EomtForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtForUniversalSegmentation) into semantic segmentation maps.

Post-processes model outputs into Instance Segmentation Predictions.

Post-processes model outputs into final panoptic segmentation prediction.

## EomtImageProcessorPil[[transformers.EomtImageProcessorPil]]

- **do_split_image** (`bool`, *kwargs*, *optional*, defaults to `self.do_split_image`) --
  Whether to split the input images into overlapping patches for semantic segmentation. If set to `True`, the
  input images will be split into patches of size `size["shortest_edge"]` with an overlap between patches.
  Otherwise, the input images will be padded to the target size.
- **ignore_index** (`int`, *kwargs*, *optional*, defaults to `self.ignore_index`) --
  Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels
  denoted with 0 (background) will be replaced with `ignore_index`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a EomtImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess for corresponding images.
- **instance_id_to_semantic_id** (`list[dict[int, int]]` or `dict[int, int]`, *optional*) --
  A mapping between object instance ids and class ids.
- **do_split_image** (`bool`, *kwargs*, *optional*, defaults to `self.do_split_image`) --
  Whether to split the input images into overlapping patches for semantic segmentation. If set to `True`, the
  input images will be split into patches of size `size["shortest_edge"]` with an overlap between patches.
  Otherwise, the input images will be padded to the target size.
- **ignore_index** (`int`, *kwargs*, *optional*, defaults to `self.ignore_index`) --
  Label to be assigned to background pixels in segmentation maps. If provided, segmentation map pixels
  denoted with 0 (background) will be replaced with `ignore_index`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** ([EomtForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtForUniversalSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple[int, int]]`) --
  A list of tuples (`tuple[int, int]`) containing the target size (height, width) of each image in the
  batch.
- **size** (`dict[str, int]`, *optional*) --
  The size to which the intermediate masks are interpolated. Defaults to `self.size`.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size.

Converts the output of [EomtForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtForUniversalSegmentation) into semantic segmentation maps.

Post-processes model outputs into Instance Segmentation Predictions.

Post-processes model outputs into final panoptic segmentation prediction.

## EomtConfig[[transformers.EomtConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `640`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **mlp_ratio** (`int`, *optional*, defaults to `4`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **layerscale_value** (`float`, *optional*, defaults to 1.0) --
  Initial value for the LayerScale parameter.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  Drop path rate for the patch fusion.
- **num_upscale_blocks** (`int`, *optional*, defaults to 2) --
  Number of upsampling blocks used in the decoder or segmentation head.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **use_swiglu_ffn** (`bool`, *optional*, defaults to `False`) --
  Whether to use the SwiGLU feedforward neural network.
- **num_blocks** (`int`, *optional*, defaults to 4) --
  Number of feature blocks or stages in the architecture.
- **no_object_weight** (`float`, *optional*, defaults to 0.1) --
  Loss weight for the 'no object' class in panoptic/instance segmentation.
- **class_weight** (`float`, *optional*, defaults to 2.0) --
  Loss weight for classification targets.
- **mask_weight** (`float`, *optional*, defaults to 5.0) --
  Loss weight for mask prediction.
- **dice_weight** (`float`, *optional*, defaults to `5.0`) --
  Relative weight of the dice loss in the panoptic segmentation loss.
- **train_num_points** (`int`, *optional*, defaults to 12544) --
  Number of points to sample for mask loss computation during training.
- **oversample_ratio** (`float`, *optional*, defaults to 3.0) --
  Oversampling ratio used in point sampling for mask training.
- **importance_sample_ratio** (`float`, *optional*, defaults to 0.75) --
  Ratio of points to sample based on importance during training.
- **num_queries** (`int`, *optional*, defaults to 200) --
  Number of object queries in the Transformer.
- **num_register_tokens** (`int`, *optional*, defaults to 4) --
  Number of learnable register tokens added to the transformer input.

This is the configuration class to store the configuration of a EomtModel. It is used to instantiate a Eomt
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [tue-mps/coco_panoptic_eomt_large_640](https://huggingface.co/tue-mps/coco_panoptic_eomt_large_640)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import EomtConfig, EomtForUniversalSegmentation

>>> # Initialize configuration
>>> config = EomtConfig()

>>> # Initialize model
>>> model = EomtForUniversalSegmentation(config)

>>> # Access config
>>> config = model.config
```

## EomtForUniversalSegmentation[[transformers.EomtForUniversalSegmentation]]

- **config** ([EomtConfig](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The EoMT Model with head on top for instance/semantic/panoptic segmentation.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "mask_labels", "val": ": list[)>] | None = None"}, {"name": "class_labels", "val": ": list[)>] | None = None"}, {"name": "patch_offsets", "val": ": list[)>] | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [EomtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtImageProcessor). See `EomtImageProcessor.__call__()` for details (`processor_class` uses
  [EomtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtImageProcessor) for processing images).
- **mask_labels** (`list[torch.Tensor]`, *optional*) --
  list of mask labels of shape `(num_labels, height, width)` to be fed to a model
- **class_labels** (`list[torch.LongTensor]`, *optional*) --
  list of target class labels of shape `(num_labels, height, width)` to be fed to a model. They identify the
  labels of `mask_labels`, e.g. the label of `mask_labels[i][j]` if `class_labels[i][j]`.
- **patch_offsets** (`list[torch.Tensor]`, *optional*) --
  list of tuples indicating the image index and start and end positions of patches for semantic segmentation.`EomtForUniversalSegmentationOutput` or `tuple(torch.FloatTensor)`A `EomtForUniversalSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EomtConfig](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtConfig)) and inputs.
The [EomtForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtForUniversalSegmentation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.Tensor`, *optional*) -- The computed loss, returned when labels are present.
- **class_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, num_labels + 1)` representing the proposed classes for each
  query. Note the `+ 1` is needed because we incorporate the null class.
- **masks_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, height, width)` representing the proposed masks for each
  query.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Last hidden states (final feature map) of the last layer.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states all layers of the model.
- **attentions** (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Self and Cross Attentions weights from transformer decoder.
- **patch_offsets** (`list[torch.Tensor]`, *optional*) -- list of tuples indicating the image index and start and end positions of patches for semantic segmentation.
