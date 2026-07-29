# EdgeTAM

## Overview

The EdgeTAM model was proposed in [EdgeTAM: On-Device Track Anything Model](https://huggingface.co/papers/2501.07256) Chong Zhou, Chenchen Zhu, Yunyang Xiong, Saksham Suri, Fanyi Xiao, Lemeng Wu, Raghuraman Krishnamoorthi, Bo Dai, Chen Change Loy, Vikas Chandra, Bilge Soran.

EdgeTAM is an efficient adaptation of SAM 2 that introduces a 2D Spatial Perceiver architecture to optimize memory attention mechanisms for real-time video segmentation on mobile devices.

The abstract from the paper is the following:

*On top of Segment Anything Model (SAM), SAM 2 further extends its capability from image to video inputs through a memory bank mechanism and obtains a remarkable performance compared with previous methods, making it a foundation model for video segmentation task. In this paper, we aim at making SAM 2 much more efficient so that it even runs on mobile devices while maintaining a comparable performance. Despite several works optimizing SAM for better efficiency, we find they are not sufficient for SAM 2 because they all focus on compressing the image encoder, while our benchmark shows that the newly introduced memory attention blocks are also the latency bottleneck. Given this observation, we propose EdgeTAM, which leverages a novel 2D Spatial Perceiver to reduce the computational cost. In particular, the proposed 2D Spatial Perceiver encodes the densely stored frame-level memories with a lightweight Transformer that contains a fixed set of learnable queries. Given that video segmentation is a dense prediction task, we find preserving the spatial structure of the memories is essential so that the queries are split into global-level and patch-level groups. We also propose a distillation pipeline that further improves the performance without inference overhead. As a result, EdgeTAM achieves 87.7, 70.0, 72.3, and 71.7 J&F on DAVIS 2017, MOSE, SA-V val, and SA-V test, while running at 16 FPS on iPhone 15 Pro Max.*

This model was contributed by [yonigozlan](https://huggingface.co/yonigozlan).
The original code can be found [here](https://github.com/facebookresearch/EdgeTAM).

## Usage example

### Automatic Mask Generation with Pipeline

EdgeTAM can be used for automatic mask generation to segment all objects in an image using the `mask-generation` pipeline:

```python
from transformers import pipeline

generator = pipeline("mask-generation", model="yonigozlan/edgetam-1", device=0)
image_url = "https://huggingface.co/datasets/hf-internal-testing/sam2-fixtures/resolve/main/truck.jpg"
outputs = generator(image_url, points_per_batch=64)

len(outputs["masks"])  # Number of masks generated
39
```

### Basic Image Segmentation

#### Single Point Click

You can segment objects by providing a single point click on the object you want to segment:

```python
from transformers import Sam2Processor, EdgeTamModel
import torch
from PIL import Image
import requests

model = EdgeTamModel.from_pretrained("yonigozlan/edgetam-1", device_map="auto")
processor = Sam2Processor.from_pretrained("yonigozlan/edgetam-1")

image_url = "https://huggingface.co/datasets/hf-internal-testing/sam2-fixtures/resolve/main/truck.jpg"
raw_image = Image.open(requests.get(image_url, stream=True).raw).convert("RGB")

input_points = [[[[500, 375]]]]  # Single point click, 4 dimensions (image_dim, object_dim, point_per_object_dim, coordinates)
input_labels = [[[1]]]  # 1 for positive click, 0 for negative click, 3 dimensions (image_dim, object_dim, point_label)

inputs = processor(images=raw_image, input_points=input_points, input_labels=input_labels, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

masks = processor.post_process_masks(outputs.pred_masks.cpu(), inputs["original_sizes"])[0]

# The model outputs multiple mask predictions ranked by quality score
print(f"Generated {masks.shape[1]} masks with shape {masks.shape}")
Generated 3 masks with shape torch.Size([1, 3, 1200, 1800])
print(f"IoU scores: {outputs.iou_scores.squeeze()}")
IoU scores: tensor([0.0463, 0.4859, 0.7616], device='cuda:0')
```

#### Multiple Points for Refinement

You can provide multiple points to refine the segmentation:

```python
# Add both positive and negative points to refine the mask
input_points = [[[[500, 375], [1125, 625]]]]  # Multiple points for refinement
input_labels = [[[1, 1]]]  # Both positive clicks

inputs = processor(images=raw_image, input_points=input_points, input_labels=input_labels, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

masks = processor.post_process_masks(outputs.pred_masks.cpu(), inputs["original_sizes"])[0]
print(f"IoU scores: {outputs.iou_scores.squeeze()}")
IoU scores: tensor([0.8362, 0.6900, 0.2120], device='cuda:0')
```

#### Bounding Box Input

EdgeTAM also supports bounding box inputs for segmentation:

```python
# Define bounding box as [x_min, y_min, x_max, y_max]
input_boxes = [[[75, 275, 1725, 850]]]

inputs = processor(images=raw_image, input_boxes=input_boxes, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

masks = processor.post_process_masks(outputs.pred_masks.cpu(), inputs["original_sizes"])[0]
print(f"IoU scores: {outputs.iou_scores.squeeze()}")
IoU scores: tensor([0.9301, 0.9348, 0.6605], device='cuda:0')
```

#### Multiple Objects Segmentation

You can segment multiple objects simultaneously:

```python
# Define points for two different objects
input_points = [[[[500, 375]], [[650, 750]]]]  # Points for two objects in same image
input_labels = [[[1], [1]]]  # Positive clicks for both objects

inputs = processor(images=raw_image, input_points=input_points, input_labels=input_labels, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs, multimask_output=False)

# Each object gets its own mask
masks = processor.post_process_masks(outputs.pred_masks.cpu(), inputs["original_sizes"])[0]
print(f"Generated masks for {masks.shape[0]} objects")
Generated masks for 2 objects
print(f"IoU scores: {outputs.iou_scores.squeeze()}")
IoU scores: tensor([0.7616, 0.9465], device='cuda:0')
```

### Batch Inference

#### Batched Images

Process multiple images simultaneously for improved efficiency:

```python
from transformers import Sam2Processor, EdgeTamModel
import torch
from PIL import Image
import requests

model = EdgeTamModel.from_pretrained("yonigozlan/edgetam-1", device_map="auto")
processor = Sam2Processor.from_pretrained("yonigozlan/edgetam-1")

# Load multiple images
image_urls = [
    "https://huggingface.co/datasets/hf-internal-testing/sam2-fixtures/resolve/main/truck.jpg",
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/dog-sam.png"
]
raw_images = [Image.open(requests.get(url, stream=True).raw).convert("RGB") for url in image_urls]

# Single point per image
input_points = [[[[500, 375]]], [[[770, 200]]]]  # One point for each image
input_labels = [[[1]], [[1]]]  # Positive clicks for both images

inputs = processor(images=raw_images, input_points=input_points, input_labels=input_labels, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs, multimask_output=False)

# Post-process masks for each image
all_masks = processor.post_process_masks(outputs.pred_masks.cpu(), inputs["original_sizes"])
print(f"Processed {len(all_masks)} images, each with {all_masks[0].shape[0]} objects")
Processed 2 images, each with 1 objects
print(f"IoU scores: {outputs.iou_scores.squeeze()}")
IoU scores: tensor([0.7618, 0.7999], device='cuda:0')
```

#### Batched Objects per Image

Segment multiple objects within each image using batch inference:

```python
# Multiple objects per image - different numbers of objects per image
input_points = [
    [[[500, 375]], [[650, 750]]],  # Truck image: 2 objects
    [[[770, 200]]]  # Dog image: 1 object
]
input_labels = [
    [[1], [1]],  # Truck image: positive clicks for both objects
    [[1]]  # Dog image: positive click for the object
]

inputs = processor(images=raw_images, input_points=input_points, input_labels=input_labels, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs, multimask_output=False)

all_masks = processor.post_process_masks(outputs.pred_masks.cpu(), inputs["original_sizes"])
```

#### Batched Images with Batched Objects and Multiple Points

Handle complex batch scenarios with multiple points per object:

```python
# Add groceries image for more complex example
groceries_url = "https://huggingface.co/datasets/hf-internal-testing/sam2-fixtures/resolve/main/groceries.jpg"
groceries_image = Image.open(requests.get(groceries_url, stream=True).raw).convert("RGB")
raw_images = [raw_images[0], groceries_image]  # Use truck and groceries images

# Complex batching: multiple images, multiple objects, multiple points per object
input_points = [
    [[[500, 375]], [[650, 750]]],  # Truck image: 2 objects with 1 point each
    [[[400, 300]], [[630, 300], [550, 300]]]  # Groceries image: obj1 has 1 point, obj2 has 2 points
]
input_labels = [
    [[1], [1]],  # Truck image: positive clicks
    [[1], [1, 1]]  # Groceries image: positive clicks for refinement
]

inputs = processor(images=raw_images, input_points=input_points, input_labels=input_labels, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs, multimask_output=False)

all_masks = processor.post_process_masks(outputs.pred_masks.cpu(), inputs["original_sizes"])
```

#### Batched Bounding Boxes

Process multiple images with bounding box inputs:

```python
# Multiple bounding boxes per image (using truck and groceries images)
input_boxes = [
    [[75, 275, 1725, 850], [425, 600, 700, 875], [1375, 550, 1650, 800], [1240, 675, 1400, 750]],  # Truck image: 4 boxes
    [[450, 170, 520, 350], [350, 190, 450, 350], [500, 170, 580, 350], [580, 170, 640, 350]]  # Groceries image: 4 boxes
]

# Update images for this example
raw_images = [raw_images[0], groceries_image]  # truck and groceries

inputs = processor(images=raw_images, input_boxes=input_boxes, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs, multimask_output=False)

all_masks = processor.post_process_masks(outputs.pred_masks.cpu(), inputs["original_sizes"])
print(f"Processed {len(input_boxes)} images with {len(input_boxes[0])} and {len(input_boxes[1])} boxes respectively")
Processed 2 images with 4 and 4 boxes respectively
print(f"IoU scores: {outputs.iou_scores.squeeze()}")
IoU scores: tensor([0.9301, 0.9348, 0.6605, 0.9465], device='cuda:0')
```

### Using Previous Masks as Input

EdgeTAM can use masks from previous predictions as input to refine segmentation:

```python
# Get initial segmentation
input_points = [[[[500, 375]]]]
input_labels = [[[1]]]
inputs = processor(images=raw_image, input_points=input_points, input_labels=input_labels, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

# Use the best mask as input for refinement
mask_input = outputs.pred_masks[:, :, torch.argmax(outputs.iou_scores.squeeze())]

# Add additional points with the mask input
new_input_points = [[[[500, 375], [450, 300]]]]
new_input_labels = [[[1, 1]]]
inputs = processor(
    input_points=new_input_points,
    input_labels=new_input_labels,
    original_sizes=inputs["original_sizes"],
    return_tensors="pt",
).to(model.device)

with torch.no_grad():
    refined_outputs = model(
        **inputs,
        input_masks=mask_input,
        image_embeddings=outputs.image_embeddings,
        multimask_output=False,
    )
```

## EdgeTamConfig[[transformers.EdgeTamConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **prompt_encoder_config** (Union[`dict`, `EdgeTamPromptEncoderConfig`], *optional*) --
  Dictionary of configuration options used to initialize [EdgeTamPromptEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/edgetam#transformers.EdgeTamPromptEncoderConfig).
- **mask_decoder_config** (Union[`dict`, `EdgeTamMaskDecoderConfig`], *optional*) --
  Dictionary of configuration options used to initialize [EdgeTamMaskDecoderConfig](/docs/transformers/v5.14.0/en/model_doc/edgetam#transformers.EdgeTamMaskDecoderConfig).
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a EdgeTamModel. It is used to instantiate a Edgetam
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/EdgeTAM-hf](https://huggingface.co/yonigozlan/EdgeTAM-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     EdgeTamVisionConfig,
...     EdgeTamPromptEncoderConfig,
...     EdgeTamMaskDecoderConfig,
...     EdgeTamModel,
... )

>>> # Initializing a EdgeTamConfig with `"facebook/edgetam.1_hiera_tiny"` style configuration
>>> configuration = EdgeTamConfig()

>>> # Initializing a EdgeTamModel (with random weights) from the `"facebook/edgetam.1_hiera_tiny"` style configuration
>>> model = EdgeTamModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a EdgeTamConfig from a EdgeTamVisionConfig, EdgeTamPromptEncoderConfig, and EdgeTamMaskDecoderConfig
>>> # Initializing EDGETAM vision encoder, memory attention, and memory encoder configurations
>>> vision_config = EdgeTamVisionConfig()
>>> prompt_encoder_config = EdgeTamPromptEncoderConfig()
>>> mask_decoder_config = EdgeTamMaskDecoderConfig()

>>> config = EdgeTamConfig(vision_config, prompt_encoder_config, mask_decoder_config)
```

## EdgeTamVisionConfig[[transformers.EdgeTamVisionConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **backbone_channel_list** (`List[int]`, *optional*, defaults to `[384, 192, 96, 48]`) --
  The list of channel dimensions for the backbone.
- **backbone_feature_sizes** (`List[List[int]]`, *optional*, defaults to `[[256, 256], [128, 128], [64, 64]]`) --
  The spatial sizes of the feature maps from the backbone.
- **fpn_hidden_size** (`int`, *optional*, defaults to 256) --
  The hidden dimension of the FPN.
- **fpn_kernel_size** (`int`, *optional*, defaults to 1) --
  The kernel size for the convolutions in the neck.
- **fpn_stride** (`int`, *optional*, defaults to 1) --
  The stride for the convolutions in the neck.
- **fpn_padding** (`int`, *optional*, defaults to 0) --
  The padding for the convolutions in the neck.
- **fpn_top_down_levels** (`List[int]`, *optional*, defaults to `[2, 3]`) --
  The levels for the top-down FPN connections.
- **num_feature_levels** (`int`, *optional*, defaults to 3) --
  The number of feature levels from the FPN to use.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a EdgeTamModel. It is used to instantiate a Edgetam
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/EdgeTAM-hf](https://huggingface.co/yonigozlan/EdgeTAM-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## EdgeTamMaskDecoderConfig[[transformers.EdgeTamMaskDecoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **mlp_dim** (`int`, *optional*, defaults to 2048) --
  The dimension of the MLP in the two-way transformer.
- **num_hidden_layers** (`int`, *optional*, defaults to `2`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **attention_downsample_rate** (`int`, *optional*, defaults to 2) --
  The downsample rate for the attention layers.
- **num_multimask_outputs** (`int`, *optional*, defaults to 3) --
  The number of multimask outputs.
- **iou_head_depth** (`int`, *optional*, defaults to 3) --
  The depth of the IoU head.
- **iou_head_hidden_dim** (`int`, *optional*, defaults to 256) --
  The hidden dimension of the IoU head.
- **dynamic_multimask_via_stability** (`bool`, *optional*, defaults to `True`) --
  Whether to use dynamic multimask via stability.
- **dynamic_multimask_stability_delta** (`float`, *optional*, defaults to 0.05) --
  The stability delta for the dynamic multimask.
- **dynamic_multimask_stability_thresh** (`float`, *optional*, defaults to 0.98) --
  The stability threshold for the dynamic multimask.

This is the configuration class to store the configuration of a EdgeTamModel. It is used to instantiate a Edgetam
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/EdgeTAM-hf](https://huggingface.co/yonigozlan/EdgeTAM-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## EdgeTamPromptEncoderConfig[[transformers.EdgeTamPromptEncoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1024`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **mask_input_channels** (`int`, *optional*, defaults to 16) --
  The number of channels to be fed to the `MaskDecoder` module.
- **num_point_embeddings** (`int`, *optional*, defaults to 4) --
  The number of point embeddings to be used.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **scale** (`float`, *optional*, defaults to 1) --
  The scale factor for the prompt encoder.

This is the configuration class to store the configuration of a EdgeTamModel. It is used to instantiate a Edgetam
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/EdgeTAM-hf](https://huggingface.co/yonigozlan/EdgeTAM-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## EdgeTamVisionModel[[transformers.EdgeTamVisionModel]]

- **config** ([EdgeTamVisionConfig](/docs/transformers/v5.14.0/en/model_doc/edgetam#transformers.EdgeTamVisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The vision model from EdgeTAM without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## EdgeTamModel[[transformers.EdgeTamModel]]

- **config** ([EdgeTamConfig](/docs/transformers/v5.14.0/en/model_doc/edgetam#transformers.EdgeTamConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Segment Anything Model 2 (SAM 2) for generating segmentation masks, given an input image and
input points and labels, boxes, or masks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sam2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam2#transformers.Sam2ImageProcessor). See `Sam2ImageProcessor.__call__()` for details ([Sam2Processor](/docs/transformers/v5.14.0/en/model_doc/sam2#transformers.Sam2Processor) uses
  [Sam2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam2#transformers.Sam2ImageProcessor) for processing images).
- **input_points** (`torch.FloatTensor` of shape `(batch_size, num_points, 2)`) --
  Input 2D spatial points, this is used by the prompt encoder to encode the prompt. Generally yields to much
  better results. The points can be obtained by passing a list of list of list to the processor that will
  create corresponding `torch` tensors of dimension 4. The first dimension is the image batch size, the
  second dimension is the point batch size (i.e. how many segmentation masks do we want the model to predict
  per input point), the third dimension is the number of points per segmentation mask (it is possible to pass
  multiple points for a single mask), and the last dimension is the x (vertical) and y (horizontal)
  coordinates of the point. If a different number of points is passed either for each image, or for each
  mask, the processor will create "PAD" points that will correspond to the (0, 0) coordinate, and the
  computation of the embedding will be skipped for these points using the labels.
- **input_labels** (`torch.LongTensor` of shape `(batch_size, point_batch_size, num_points)`) --
  Input labels for the points, this is used by the prompt encoder to encode the prompt. According to the
  official implementation, there are 3 types of labels

  - `1`: the point is a point that contains the object of interest
  - `0`: the point is a point that does not contain the object of interest
  - `-1`: the point corresponds to the background

  We added the label:

  - `-10`: the point is a padding point, thus should be ignored by the prompt encoder

  The padding labels should be automatically done by the processor.
- **input_boxes** (`torch.FloatTensor` of shape `(batch_size, num_boxes, 4)`) --
  Input boxes for the points, this is used by the prompt encoder to encode the prompt. Generally yields to
  much better generated masks. The boxes can be obtained by passing a list of list of list to the processor,
  that will generate a `torch` tensor, with each dimension corresponding respectively to the image batch
  size, the number of boxes per image and the coordinates of the top left and bottom right point of the box.
  In the order (`x1`, `y1`, `x2`, `y2`):

  - `x1`: the x coordinate of the top left point of the input box
  - `y1`: the y coordinate of the top left point of the input box
  - `x2`: the x coordinate of the bottom right point of the input box
  - `y2`: the y coordinate of the bottom right point of the input box
- **input_masks** (`torch.FloatTensor` of shape `(batch_size, image_size, image_size)`) --
  SAM model also accepts segmentation masks as input. The mask will be embedded by the prompt encoder to
  generate a corresponding embedding, that will be fed later on to the mask decoder. These masks needs to be
  manually fed by the user, and they need to be of shape (`batch_size`, `image_size`, `image_size`).
- **image_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_channels, window_size, window_size)`) --
  Image embeddings, this is used by the mask decoder to generate masks and iou scores. For more memory
  efficient computation, users can first retrieve the image embeddings using the `get_image_embeddings`
  method, and then feed them to the `forward` method instead of feeding the `pixel_values`.
- **multimask_output** (`bool`, *optional*) --
  In the original implementation and paper, the model always outputs 3 masks per image (or per point / per
  bounding box if relevant). However, it is possible to just output a single mask, that corresponds to the
  "best" mask, by specifying `multimask_output=False`.
- **attention_similarity** (`torch.FloatTensor`, *optional*) --
  Attention similarity tensor, to be provided to the mask decoder for target-guided attention in case the
  model is used for personalization as introduced in [PerSAM](https://huggingface.co/papers/2305.03048).
- **target_embedding** (`torch.FloatTensor`, *optional*) --
  Embedding of the target concept, to be provided to the mask decoder for target-semantic prompting in case
  the model is used for personalization as introduced in [PerSAM](https://huggingface.co/papers/2305.03048).`EdgeTamImageSegmentationOutput` or `tuple(torch.FloatTensor)`A `EdgeTamImageSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EdgeTamConfig](/docs/transformers/v5.14.0/en/model_doc/edgetam#transformers.EdgeTamConfig)) and inputs.
The [EdgeTamModel](/docs/transformers/v5.14.0/en/model_doc/edgetam#transformers.EdgeTamModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **iou_scores** (`torch.FloatTensor` of shape `(batch_size, point_batch_size, num_masks)`) -- The Intersection over Union (IoU) scores of the predicted masks.
- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, point_batch_size, num_masks, height, width)`) -- The predicted low-resolution masks. This is an alias for `low_res_masks`. These masks need to be post-processed
  by the processor to be brought to the original image size.
- **object_score_logits** (`torch.FloatTensor` of shape `(batch_size, point_batch_size, 1)`) -- Logits for the object score, indicating if an object is present.
- **image_embeddings** (`tuple(torch.FloatTensor)`) -- The features from the FPN, which are used by the mask decoder. This is a tuple of `torch.FloatTensor` where each
  tensor has shape `(batch_size, channels, height, width)`.
- **vision_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of each stage) of shape `(batch_size, height, width, hidden_size)`.
  Hidden-states of the vision model at the output of each stage.
- **vision_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.
  Attentions weights of the vision model.
- **mask_decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length, sequence_length)`.
  Attentions weights of the mask decoder.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoModel, AutoProcessor

>>> model = AutoModel.from_pretrained("danelcsb/edgetam.1_hiera_tiny")
>>> processor = AutoProcessor.from_pretrained("danelcsb/edgetam.1_hiera_tiny")

>>> url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/sam-car.png"
>>> with httpx.stream("GET", url) as response:
...     raw_image = Image.open(BytesIO(response.read())).convert("RGB")
>>> input_points = [[[400, 650]]]  # 2D location of a window on the car
>>> inputs = processor(images=raw_image, input_points=input_points, return_tensors="pt")

>>> # Get segmentation mask
>>> outputs = model(**inputs)

>>> # Postprocess masks
>>> masks = processor.post_process_masks(
...     outputs.pred_masks, inputs["original_sizes"]
... )
```

- **pixel_values** (`torch.FloatTensor`) --
  Input pixel values of shape `(batch_size, num_channels, height, width)`.`EdgeTamVisionEncoderOutput` or `tuple(torch.FloatTensor)`A `EdgeTamVisionEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EdgeTamConfig](/docs/transformers/v5.14.0/en/model_doc/edgetam#transformers.EdgeTamConfig)) and inputs.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, height, width, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
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
- **fpn_hidden_states** (`tuple(torch.FloatTensor)`) -- Tuple of `torch.FloatTensor` (one for each feature level, from high to low resolution) of shape
  `(batch_size, hidden_size, height, width)`. Feature maps from the Feature Pyramid Network neck.
- **fpn_position_encoding** (`tuple(torch.FloatTensor)`) -- Tuple of `torch.FloatTensor` (one for each feature level, from high to low resolution) of shape
  `(batch_size, hidden_size, height, width)`. Positional encodings corresponding to the `fpn_hidden_states`.

Example:

```python
```
