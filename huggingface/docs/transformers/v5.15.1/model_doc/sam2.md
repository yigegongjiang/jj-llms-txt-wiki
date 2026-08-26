# SAM2

    
        
        
    

## Overview

SAM2 (Segment Anything Model 2) was proposed in [Segment Anything in Images and Videos](https://ai.meta.com/research/publications/sam-2-segment-anything-in-images-and-videos/) by Nikhila Ravi, Valentin Gabeur, Yuan-Ting Hu, Ronghang Hu, Chaitanya Ryali, Tengyu Ma, Haitham Khedr, Roman Rädle, Chloe Rolland, Laura Gustafson, Eric Mintun, Junting Pan, Kalyan Vasudev Alwala, Nicolas Carion, Chao-Yuan Wu, Ross Girshick, Piotr Dollár, Christoph Feichtenhofer.

The model can be used to predict segmentation masks of any object of interest given an input image or video, and input points or bounding boxes.

![example image](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/sam2_header.gif)

The abstract from the paper is the following:

*We present Segment Anything Model 2 (SAM 2), a foundation model towards solving promptable visual segmentation in images and videos. We build a data engine, which improves model and data via user interaction, to collect the largest video segmentation dataset to date. Our model is a simple transformer architecture with streaming memory for real-time video processing. SAM 2 trained on our data provides strong performance across a wide range of tasks. In video segmentation, we observe better accuracy, using 3x fewer interactions than prior approaches. In image segmentation, our model is more accurate and 6x faster than the Segment Anything Model (SAM). We believe that our data, model, and insights will serve as a significant milestone for video segmentation and related perception tasks. We are releasing a version of our model, the dataset and an interactive demo.*

Tips:

- Batch & Video Support: SAM2 natively supports batch processing and seamless video segmentation, while original SAM is designed for static images and simpler one-image-at-a-time workflows.
- Accuracy & Generalization: SAM2 shows improved segmentation quality, robustness, and zero-shot generalization to new domains compared to the original SAM, especially with mixed prompts.

This model was contributed by [sangbumchoi](https://github.com/SangbumChoi) and [yonigozlan](https://huggingface.co/yonigozlan).
The original code can be found [here](https://github.com/facebookresearch/sam2/tree/main).

## Usage example

### Automatic Mask Generation with Pipeline

SAM2 can be used for automatic mask generation to segment all objects in an image using the `mask-generation` pipeline:

```python
from transformers import pipeline

generator = pipeline("mask-generation", model="facebook/sam2.1-hiera-large", device=0)
image_url = "https://huggingface.co/datasets/hf-internal-testing/sam2-fixtures/resolve/main/truck.jpg"
outputs = generator(image_url, points_per_batch=64)

len(outputs["masks"])  # Number of masks generated
39
```

### Basic Image Segmentation

#### Single Point Click

You can segment objects by providing a single point click on the object you want to segment:

```python
from transformers import Sam2Processor, Sam2Model
import torch
from PIL import Image
import requests

model = Sam2Model.from_pretrained("facebook/sam2.1-hiera-large", device_map="auto")
processor = Sam2Processor.from_pretrained("facebook/sam2.1-hiera-large")

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
Generated 3 masks with shape torch.Size(1, 3, 1500, 2250)
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
```

#### Bounding Box Input

SAM2 also supports bounding box inputs for segmentation:

```python
# Define bounding box as [x_min, y_min, x_max, y_max]
input_boxes = [[[75, 275, 1725, 850]]]

inputs = processor(images=raw_image, input_boxes=input_boxes, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

masks = processor.post_process_masks(outputs.pred_masks.cpu(), inputs["original_sizes"])[0]
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
```

### Batch Inference

#### Batched Images

Process multiple images simultaneously for improved efficiency:

```python
from transformers import Sam2Processor, Sam2Model
import torch
from PIL import Image
import requests

model = Sam2Model.from_pretrained("facebook/sam2.1-hiera-large", device_map="auto")
processor = Sam2Processor.from_pretrained("facebook/sam2.1-hiera-large")

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
```

### Using Previous Masks as Input

SAM2 can use masks from previous predictions as input to refine segmentation:

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

## Sam2Config[[transformers.Sam2Config]]

#### transformers.Sam2Config[[transformers.Sam2Config]]

```python
transformers.Sam2Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, prompt_encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, mask_decoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/configuration_sam2.py#L224)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

prompt_encoder_config (Union[`dict`, `Sam2PromptEncoderConfig`], *optional*) : Dictionary of configuration options used to initialize [Sam2PromptEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2PromptEncoderConfig).

mask_decoder_config (Union[`dict`, `Sam2MaskDecoderConfig`], *optional*) : Dictionary of configuration options used to initialize [Sam2MaskDecoderConfig](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2MaskDecoderConfig).

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam2Model. It is used to instantiate a Sam2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam2.1-hiera-tiny](https://huggingface.co/facebook/sam2.1-hiera-tiny)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     Sam2VisionConfig,
...     Sam2PromptEncoderConfig,
...     Sam2MaskDecoderConfig,
...     Sam2Model,
... )

>>> # Initializing a Sam2Config with `"facebook/sam2.1_hiera_tiny"` style configuration
>>> configuration = Sam2Config()

>>> # Initializing a Sam2Model (with random weights) from the `"facebook/sam2.1_hiera_tiny"` style configuration
>>> model = Sam2Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a Sam2Config from a Sam2VisionConfig, Sam2PromptEncoderConfig, and Sam2MaskDecoderConfig

>>> # Initializing SAM2 vision encoder, memory attention, and memory encoder configurations
>>> vision_config = Sam2VisionConfig()
>>> prompt_encoder_config = Sam2PromptEncoderConfig()
>>> mask_decoder_config = Sam2MaskDecoderConfig()

>>> config = Sam2Config(vision_config, prompt_encoder_config, mask_decoder_config)
```

## Sam2HieraDetConfig[[transformers.Sam2HieraDetConfig]]

#### transformers.Sam2HieraDetConfig[[transformers.Sam2HieraDetConfig]]

```python
transformers.Sam2HieraDetConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 96, num_attention_heads: int = 1, num_channels: int = 3, image_size: int | list[int] | None = None, patch_kernel_size: int | list[int] | None = None, patch_stride: int | list[int] | None = None, patch_padding: int | list[int] | None = None, query_stride: int | list[int] | None = None, window_positional_embedding_background_size: list[int] | None = None, num_query_pool_stages: int = 3, blocks_per_stage: list[int] | None = None, embed_dim_per_stage: list[int] | None = None, num_attention_heads_per_stage: list[int] | None = None, window_size_per_stage: list[int] | None = None, global_attention_blocks: list[int] | None = None, mlp_ratio: float = 4.0, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/configuration_sam2.py#L25)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `96`) : Dimension of the hidden representations.

num_attention_heads (`int`, *optional*, defaults to `1`) : Number of attention heads for each attention layer in the Transformer decoder.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

image_size (`Union[int, list[int]]`, *optional*) : The size (resolution) of each image.

patch_kernel_size (`list[int]`, *optional*, defaults to `[7, 7]`) : The kernel size of the patch.

patch_stride (`list[int]`, *optional*, defaults to `[4, 4]`) : The stride of the patch.

patch_padding (`list[int]`, *optional*, defaults to `[3, 3]`) : The padding of the patch.

query_stride (`list[int]`, *optional*, defaults to `[2, 2]`) : The downsample stride between stages.

window_positional_embedding_background_size (`list[int]`, *optional*, defaults to `[7, 7]`) : The window size per stage when not using global attention.

num_query_pool_stages (`int`, *optional*, defaults to 3) : The number of query pool stages.

blocks_per_stage (`list[int]`, *optional*, defaults to `[1, 2, 7, 2]`) : The number of blocks per stage.

embed_dim_per_stage (`list[int]`, *optional*, defaults to `[96, 192, 384, 768]`) : The embedding dimension per stage.

num_attention_heads_per_stage (`list[int]`, *optional*, defaults to `[1, 2, 4, 8]`) : The number of attention heads per stage.

window_size_per_stage (`list[int]`, *optional*, defaults to `[8, 4, 14, 7]`) : The window size per stage.

global_attention_blocks (`list[int]`, *optional*, defaults to `[5, 7, 9]`) : The blocks where global attention is used.

mlp_ratio (`float`, *optional*, defaults to `4.0`) : Ratio of the MLP hidden dim to the embedding dim.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam2Model. It is used to instantiate a Sam2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam2.1-hiera-tiny](https://huggingface.co/facebook/sam2.1-hiera-tiny)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam2VisionConfig[[transformers.Sam2VisionConfig]]

#### transformers.Sam2VisionConfig[[transformers.Sam2VisionConfig]]

```python
transformers.Sam2VisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, backbone_channel_list: list[int] | None = None, backbone_feature_sizes: list | None = None, fpn_hidden_size: int = 256, fpn_kernel_size: int = 1, fpn_stride: int = 1, fpn_padding: int = 0, fpn_top_down_levels: list[int] | None = None, num_feature_levels: int = 3, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/configuration_sam2.py#L103)

**Parameters:**

backbone_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The configuration of the backbone model.

backbone_channel_list (`List[int]`, *optional*, defaults to `[768, 384, 192, 96]`) : The list of channel dimensions for the backbone.

backbone_feature_sizes (`List[List[int]]`, *optional*, defaults to `[[256, 256], [128, 128], [64, 64]]`) : The spatial sizes of the feature maps from the backbone.

fpn_hidden_size (`int`, *optional*, defaults to 256) : The hidden dimension of the FPN.

fpn_kernel_size (`int`, *optional*, defaults to 1) : The kernel size for the convolutions in the neck.

fpn_stride (`int`, *optional*, defaults to 1) : The stride for the convolutions in the neck.

fpn_padding (`int`, *optional*, defaults to 0) : The padding for the convolutions in the neck.

fpn_top_down_levels (`List[int]`, *optional*, defaults to `[2, 3]`) : The levels for the top-down FPN connections.

num_feature_levels (`int`, *optional*, defaults to 3) : The number of feature levels from the FPN to use.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam2Model. It is used to instantiate a Sam2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam2.1-hiera-tiny](https://huggingface.co/facebook/sam2.1-hiera-tiny)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam2MaskDecoderConfig[[transformers.Sam2MaskDecoderConfig]]

#### transformers.Sam2MaskDecoderConfig[[transformers.Sam2MaskDecoderConfig]]

```python
transformers.Sam2MaskDecoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 256, hidden_act: str = 'gelu', mlp_dim: int = 2048, num_hidden_layers: int = 2, num_attention_heads: int = 8, attention_downsample_rate: int = 2, num_multimask_outputs: int = 3, iou_head_depth: int = 3, iou_head_hidden_dim: int = 256, dynamic_multimask_via_stability: bool = True, dynamic_multimask_stability_delta: float = 0.05, dynamic_multimask_stability_thresh: float = 0.98)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/configuration_sam2.py#L186)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

mlp_dim (`int`, *optional*, defaults to 2048) : The dimension of the MLP in the two-way transformer.

num_hidden_layers (`int`, *optional*, defaults to `2`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

attention_downsample_rate (`int`, *optional*, defaults to 2) : The downsample rate for the attention layers.

num_multimask_outputs (`int`, *optional*, defaults to 3) : The number of multimask outputs.

iou_head_depth (`int`, *optional*, defaults to 3) : The depth of the IoU head.

iou_head_hidden_dim (`int`, *optional*, defaults to 256) : The hidden dimension of the IoU head.

dynamic_multimask_via_stability (`bool`, *optional*, defaults to `True`) : Whether to use dynamic multimask via stability.

dynamic_multimask_stability_delta (`float`, *optional*, defaults to 0.05) : The stability delta for the dynamic multimask.

dynamic_multimask_stability_thresh (`float`, *optional*, defaults to 0.98) : The stability threshold for the dynamic multimask.

This is the configuration class to store the configuration of a Sam2Model. It is used to instantiate a Sam2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam2.1-hiera-tiny](https://huggingface.co/facebook/sam2.1-hiera-tiny)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam2PromptEncoderConfig[[transformers.Sam2PromptEncoderConfig]]

#### transformers.Sam2PromptEncoderConfig[[transformers.Sam2PromptEncoderConfig]]

```python
transformers.Sam2PromptEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 256, image_size: int | list[int] | tuple[int, int] = 1024, patch_size: int | list[int] | tuple[int, int] = 16, mask_input_channels: int = 16, num_point_embeddings: int = 4, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06, scale: int = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/configuration_sam2.py#L162)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1024`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

mask_input_channels (`int`, *optional*, defaults to 16) : The number of channels to be fed to the `MaskDecoder` module.

num_point_embeddings (`int`, *optional*, defaults to 4) : The number of point embeddings to be used.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

scale (`float`, *optional*, defaults to 1) : The scale factor for the prompt encoder.

This is the configuration class to store the configuration of a Sam2Model. It is used to instantiate a Sam2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam2.1-hiera-tiny](https://huggingface.co/facebook/sam2.1-hiera-tiny)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam2Processor[[transformers.Sam2Processor]]

#### transformers.Sam2Processor[[transformers.Sam2Processor]]

```python
transformers.Sam2Processor(image_processor, target_size: int | None = None, point_pad_value: int = -10, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/processing_sam2.py#L37)

**Parameters:**

image_processor (`Sam2ImageProcessor`) : The image processor is a required input.

target_size (`int`, *optional*) : The target size (in pixels) for normalizing input points and bounding boxes. If not provided, defaults to the image processor's size configuration. All input coordinates (points and boxes) are normalized to this size before being passed to the model. This ensures consistent coordinate representation regardless of the original image dimensions.

point_pad_value (`int`, *optional*, defaults to -10) : The value used for padding input points when batching sequences of different lengths. This value is used to mark padded positions and is preserved during coordinate normalization.

Constructs a Sam2Processor which wraps a image processor into a single processor.

[Sam2Processor](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2Processor) offers all the functionalities of [Sam2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2ImageProcessor). See the
[~Sam2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2ImageProcessor) for more information.

#### __call__[[transformers.Sam2Processor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, input_points: typing.Union[list[list[list[list[float]]]], torch.Tensor, NoneType] = None, input_labels: typing.Union[list[list[list[int]]], torch.Tensor, NoneType] = None, input_boxes: typing.Union[list[list[list[float]]], torch.Tensor, NoneType] = None, original_sizes: typing.Union[list[list[float]], torch.Tensor, NoneType] = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/processing_sam2.py#L53)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

segmentation_maps (`ImageInput`, *optional*) : The segmentation maps to process.

input_points (`list[list[list[list[float]]]]`, `torch.Tensor`, *optional*) : The points to add to the frame.

input_labels (`list[list[list[int]]]`, `torch.Tensor`, *optional*) : The labels for the points.

input_boxes (`list[list[list[float]]]`, `torch.Tensor`, *optional*) : The bounding boxes to add to the frame.

original_sizes (`list[list[float]]`, `torch.Tensor`, *optional*) : The original sizes of the images.

return_tensors (`Union[str, ~utils.generic.TensorType]`, *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

**Returns:** A [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields

- `pixel_values` (`torch.Tensor`): The processed image(s).
- `original_sizes` (`list[list[float]]`): The original sizes of the images.
- `labels` (`torch.Tensor`): The processed segmentation maps (if provided).
- `input_points` (`torch.Tensor`): The processed points.
- `input_labels` (`torch.Tensor`): The processed labels.
- `input_boxes` (`torch.Tensor`): The processed bounding boxes.

#### post_process_masks[[transformers.Sam2Processor.post_process_masks]]

```python
post_process_masks(masks, original_sizes, mask_threshold = 0.0, binarize = True, max_hole_area = 0.0, max_sprinkle_area = 0.0, apply_non_overlapping_constraints = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/processing_sam2.py#L459)

**Parameters:**

masks (`Union[List[torch.Tensor], List[np.ndarray]]`) : Batched masks from the mask_decoder in (batch_size, num_channels, height, width) format.

original_sizes (`Union[torch.Tensor, List[Tuple[int,int]]]`) : The original sizes of each image before it was resized to the model's expected input shape, in (height, width) format.

mask_threshold (`float`, *optional*, defaults to 0.0) : Threshold for binarization and post-processing operations.

binarize (`bool`, *optional*, defaults to `True`) : Whether to binarize the masks.

max_hole_area (`float`, *optional*, defaults to 0.0) : The maximum area of a hole to fill.

max_sprinkle_area (`float`, *optional*, defaults to 0.0) : The maximum area of a sprinkle to fill.

apply_non_overlapping_constraints (`bool`, *optional*, defaults to `False`) : Whether to apply non-overlapping constraints to the masks.

**Returns:** (`torch.Tensor`)

Batched masks in batch_size, num_channels, height, width) format, where (height, width)
is given by original_size.

Remove padding and upscale masks to the original image size.

## Sam2ImageProcessor[[transformers.Sam2ImageProcessor]]

#### transformers.Sam2ImageProcessor[[transformers.Sam2ImageProcessor]]

```python
transformers.Sam2ImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/image_processing_sam2.py#L370)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 1024, 'width': 1024}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `None`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `None`) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

mask_size (`dict[str, *kwargs*, int]`, *optional*) : The size `{"height": int, "width": int}` to resize the segmentation maps to.

Constructs a Sam2ImageProcessor image processor.

#### preprocess[[transformers.Sam2ImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/image_processing_sam2.py#L390)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

segmentation_maps (`ImageInput`, *optional*) : The segmentation maps to preprocess.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

mask_size (`dict[str, *kwargs*, int]`, *optional*) : The size `{"height": int, "width": int}` to resize the segmentation maps to.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Sam2HieraDetModel[[transformers.Sam2HieraDetModel]]

#### transformers.Sam2HieraDetModel[[transformers.Sam2HieraDetModel]]

```python
transformers.Sam2HieraDetModel(config: Sam2HieraDetConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/modeling_sam2.py#L603)

#### forward[[transformers.Sam2HieraDetModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/modeling_sam2.py#L646)

## Sam2VisionModel[[transformers.Sam2VisionModel]]

#### transformers.Sam2VisionModel[[transformers.Sam2VisionModel]]

```python
transformers.Sam2VisionModel(config: Sam2VisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/modeling_sam2.py#L677)

**Parameters:**

config ([Sam2VisionConfig](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2VisionConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The vision model from Sam without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Sam2VisionModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/modeling_sam2.py#L699)

## Sam2Model[[transformers.Sam2Model]]

#### transformers.Sam2Model[[transformers.Sam2Model]]

```python
transformers.Sam2Model(config: Sam2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/modeling_sam2.py#L1314)

**Parameters:**

config ([Sam2Config](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Segment Anything Model 2 (SAM 2) for generating segmentation masks, given an input image and
input points and labels, boxes, or masks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Sam2Model.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, input_points: typing.Optional[torch.FloatTensor] = None, input_labels: typing.Optional[torch.LongTensor] = None, input_boxes: typing.Optional[torch.FloatTensor] = None, input_masks: typing.Optional[torch.LongTensor] = None, image_embeddings: typing.Optional[torch.FloatTensor] = None, multimask_output: bool = True, attention_similarity: typing.Optional[torch.FloatTensor] = None, target_embedding: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/modeling_sam2.py#L1413)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Sam2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2ImageProcessor). See `Sam2ImageProcessor.__call__()` for details ([Sam2Processor](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2Processor) uses [Sam2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2ImageProcessor) for processing images).

input_points (`torch.FloatTensor` of shape `(batch_size, num_points, 2)`) : Input 2D spatial points, this is used by the prompt encoder to encode the prompt. Generally yields to much better results. The points can be obtained by passing a list of list of list to the processor that will create corresponding `torch` tensors of dimension 4. The first dimension is the image batch size, the second dimension is the point batch size (i.e. how many segmentation masks do we want the model to predict per input point), the third dimension is the number of points per segmentation mask (it is possible to pass multiple points for a single mask), and the last dimension is the x (vertical) and y (horizontal) coordinates of the point. If a different number of points is passed either for each image, or for each mask, the processor will create "PAD" points that will correspond to the (0, 0) coordinate, and the computation of the embedding will be skipped for these points using the labels.

input_labels (`torch.LongTensor` of shape `(batch_size, point_batch_size, num_points)`) : Input labels for the points, this is used by the prompt encoder to encode the prompt. According to the official implementation, there are 3 types of labels  - `1`: the point is a point that contains the object of interest - `0`: the point is a point that does not contain the object of interest - `-1`: the point corresponds to the background  We added the label:  - `-10`: the point is a padding point, thus should be ignored by the prompt encoder  The padding labels should be automatically done by the processor.

input_boxes (`torch.FloatTensor` of shape `(batch_size, num_boxes, 4)`) : Input boxes for the points, this is used by the prompt encoder to encode the prompt. Generally yields to much better generated masks. The boxes can be obtained by passing a list of list of list to the processor, that will generate a `torch` tensor, with each dimension corresponding respectively to the image batch size, the number of boxes per image and the coordinates of the top left and bottom right point of the box. In the order (`x1`, `y1`, `x2`, `y2`):  - `x1`: the x coordinate of the top left point of the input box - `y1`: the y coordinate of the top left point of the input box - `x2`: the x coordinate of the bottom right point of the input box - `y2`: the y coordinate of the bottom right point of the input box

input_masks (`torch.FloatTensor` of shape `(batch_size, image_size, image_size)`) : SAM model also accepts segmentation masks as input. The mask will be embedded by the prompt encoder to generate a corresponding embedding, that will be fed later on to the mask decoder. These masks needs to be manually fed by the user, and they need to be of shape (`batch_size`, `image_size`, `image_size`).

image_embeddings (`torch.FloatTensor` of shape `(batch_size, output_channels, window_size, window_size)`) : Image embeddings, this is used by the mask decoder to generate masks and iou scores. For more memory efficient computation, users can first retrieve the image embeddings using the `get_image_embeddings` method, and then feed them to the `forward` method instead of feeding the `pixel_values`.

multimask_output (`bool`, *optional*) : In the original implementation and paper, the model always outputs 3 masks per image (or per point / per bounding box if relevant). However, it is possible to just output a single mask, that corresponds to the "best" mask, by specifying `multimask_output=False`.

attention_similarity (`torch.FloatTensor`, *optional*) : Attention similarity tensor, to be provided to the mask decoder for target-guided attention in case the model is used for personalization as introduced in [PerSAM](https://huggingface.co/papers/2305.03048).

target_embedding (`torch.FloatTensor`, *optional*) : Embedding of the target concept, to be provided to the mask decoder for target-semantic prompting in case the model is used for personalization as introduced in [PerSAM](https://huggingface.co/papers/2305.03048).

**Returns:** `Sam2ImageSegmentationOutput` or `tuple(torch.FloatTensor)`

A `Sam2ImageSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam2Config](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2Config)) and inputs.

The [Sam2Model](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2Model) forward method, overrides the `__call__` special method.

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

>>> model = AutoModel.from_pretrained("danelcsb/sam2.1_hiera_tiny")
>>> processor = AutoProcessor.from_pretrained("danelcsb/sam2.1_hiera_tiny")

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

#### get_image_features[[transformers.Sam2Model.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam2/modeling_sam2.py#L1589)

**Parameters:**

pixel_values (`torch.FloatTensor`) : Input pixel values of shape `(batch_size, num_channels, height, width)`.

**Returns:** `Sam2VisionEncoderOutput` or `tuple(torch.FloatTensor)`

A `Sam2VisionEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam2Config](/docs/transformers/v5.15.1/en/model_doc/sam2#transformers.Sam2Config)) and inputs.

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
