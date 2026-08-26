# SAM3

    
        
        
    

## Overview

SAM3 (Segment Anything Model 3) was introduced in [SAM 3: Segment Anything with Concepts](https://ai.meta.com/research/publications/sam-3-segment-anything-with-concepts/).

SAM3 performs **Promptable Concept Segmentation (PCS)** on images. PCS takes text and/or image exemplars as input (e.g., "yellow school bus"), and predicts instance and semantic masks for **every single object** matching the concept.

The abstract from the paper is the following:

*We present Segment Anything Model (SAM) 3, a unified model that detects, segments, and tracks objects in images and videos based on concept prompts, which we define as either short noun phrases (e.g., "yellow school bus"), image exemplars, or a combination of both. Promptable Concept Segmentation (PCS) takes such prompts and returns segmentation masks and unique identities for all matching object instances. To advance PCS, we build a scalable data engine that produces a high-quality dataset with 4M unique concept labels, including hard negatives, across images and videos. Our model consists of an image-level detector and a memory-based video tracker that share a single backbone. Recognition and localization are decoupled with a presence head, which boosts detection accuracy. SAM 3 doubles the accuracy of existing systems in both image and video PCS, and improves previous SAM capabilities on visual segmentation tasks. We open source SAM 3 along with our new Segment Anything with Concepts (SA-Co) benchmark for promptable concept segmentation.*

This model was contributed by [yonigozlan](https://huggingface.co/yonigozlan) and [ronghanghu](https://huggingface.co/ronghanghu).

## Usage examples with 🤗 Transformers

### Text-Only Prompts

```python
import requests
import torch
from PIL import Image

from transformers import Sam3Model, Sam3Processor

model = Sam3Model.from_pretrained("facebook/sam3", device_map="auto")
processor = Sam3Processor.from_pretrained("facebook/sam3")

# Load image
image_url = "http://images.cocodataset.org/val2017/000000077595.jpg"
image = Image.open(requests.get(image_url, stream=True).raw).convert("RGB")

# Segment using text prompt
inputs = processor(images=image, text="ear", return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

# Post-process results
results = processor.post_process_instance_segmentation(
    outputs,
    threshold=0.5,
    mask_threshold=0.5,
    target_sizes=inputs.get("original_sizes").tolist()
)[0]

print(f"Found {len(results['masks'])} objects")
# Results contain:
# - masks: Binary masks resized to original image size
# - boxes: Bounding boxes in absolute pixel coordinates (xyxy format)
# - scores: Confidence scores
```

### Single Bounding Box Prompt

Segment objects using a bounding box on the visual concept:

```python
# Box in xyxy format: [x1, y1, x2, y2] in pixel coordinates
# Example: laptop region
box_xyxy = [100, 150, 500, 450]
input_boxes = [[box_xyxy]]  # [batch, num_boxes, 4]
input_boxes_labels = [[1]]  # 1 = positive box

inputs = processor(
    images=image,
    input_boxes=input_boxes,
    input_boxes_labels=input_boxes_labels,
    return_tensors="pt"
).to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

# Post-process results
results = processor.post_process_instance_segmentation(
    outputs,
    threshold=0.5,
    mask_threshold=0.5,
    target_sizes=inputs.get("original_sizes").tolist()
)[0]
```

### Multiple Box Prompts (Positive and Negative)

Use multiple boxes with positive and negative labels to refine the concept:

```python
# Load kitchen image
kitchen_url = "http://images.cocodataset.org/val2017/000000136466.jpg"
kitchen_image = Image.open(requests.get(kitchen_url, stream=True).raw).convert("RGB")

# Define two positive boxes (e.g., dial and button on oven)
# Boxes are in xyxy format [x1, y1, x2, y2] in pixel coordinates
box1_xyxy = [59, 144, 76, 163]  # Dial box
box2_xyxy = [87, 148, 104, 159]  # Button box
input_boxes = [[box1_xyxy, box2_xyxy]]
input_boxes_labels = [[1, 1]]  # Both positive

inputs = processor(
    images=kitchen_image,
    input_boxes=input_boxes,
    input_boxes_labels=input_boxes_labels,
    return_tensors="pt"
).to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

# Post-process results
results = processor.post_process_instance_segmentation(
    outputs,
    threshold=0.5,
    mask_threshold=0.5,
    target_sizes=inputs.get("original_sizes").tolist()
)[0]
```

### Combined Prompts (Text + Negative Box)

Use text prompts with negative visual prompts to refine the concept:

```python
# Segment "handle" but exclude the oven handle using a negative box
text = "handle"
# Negative box covering oven handle area (xyxy): [40, 183, 318, 204]
oven_handle_box = [40, 183, 318, 204]
input_boxes = [[oven_handle_box]]

inputs = processor(
    images=kitchen_image,
    text=text,
    input_boxes=input_boxes,
    input_boxes_labels=[[0]],  # 0 = negative (exclude this region)
    return_tensors="pt"
).to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

# Post-process results
results = processor.post_process_instance_segmentation(
    outputs,
    threshold=0.5,
    mask_threshold=0.5,
    target_sizes=inputs.get("original_sizes").tolist()
)[0]
# This will segment pot handles but exclude the oven handle
```

### Batched Inference with Text Prompts

Process multiple images with different text prompts efficiently:

```python
cat_url = "http://images.cocodataset.org/val2017/000000077595.jpg"
kitchen_url = "http://images.cocodataset.org/val2017/000000136466.jpg"
images = [
    Image.open(requests.get(cat_url, stream=True).raw).convert("RGB"),
    Image.open(requests.get(kitchen_url, stream=True).raw).convert("RGB")
]

# Different text prompt for each image
text_prompts = ["ear", "dial"]

inputs = processor(images=images, text=text_prompts, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

# Post-process results for both images
results = processor.post_process_instance_segmentation(
    outputs,
    threshold=0.5,
    mask_threshold=0.5,
    target_sizes=inputs.get("original_sizes").tolist()
)

print(f"Image 1: {len(results[0]['masks'])} objects found")
print(f"Image 2: {len(results[1]['masks'])} objects found")
```

### Batched Mixed Prompts

Use different prompt types for different images in the same batch:

```python
# Image 1: text prompt "laptop"
# Image 2: visual prompt (dial box)
box2_xyxy = [59, 144, 76, 163]

inputs = processor(
    images=images,
    text=["laptop", None],  # Only first image has text
    input_boxes=[None, [box2_xyxy]],  # Only second image has box
    input_boxes_labels=[None, [1]],  # Positive box for second image
    return_tensors="pt"
).to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

# Post-process results for both images
results = processor.post_process_instance_segmentation(
    outputs,
    threshold=0.5,
    mask_threshold=0.5,
    target_sizes=inputs.get("original_sizes").tolist()
)
# Both images processed in single forward pass
```

### Semantic Segmentation Output

SAM3 also provides semantic segmentation alongside instance masks:

```python
inputs = processor(images=image, text="ear", return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

# Instance segmentation masks
instance_masks = torch.sigmoid(outputs.pred_masks)  # [batch, num_queries, H, W]

# Semantic segmentation (single channel)
semantic_seg = outputs.semantic_seg  # [batch, 1, H, W]

print(f"Instance masks: {instance_masks.shape}")
print(f"Semantic segmentation: {semantic_seg.shape}")
```

### Efficient Multi-Prompt Inference on Single Image

When running multiple text prompts on the same image, pre-compute vision embeddings to avoid redundant computation:

```python
from transformers import Sam3Processor, Sam3Model
import torch
from PIL import Image
import requests

model = Sam3Model.from_pretrained("facebook/sam3", device_map="auto")
processor = Sam3Processor.from_pretrained("facebook/sam3")

# Load image
image_url = "http://images.cocodataset.org/val2017/000000077595.jpg"
image = Image.open(requests.get(image_url, stream=True).raw).convert("RGB")

# Pre-process image and compute vision embeddings once
img_inputs = processor(images=image, return_tensors="pt").to(model.device)
with torch.no_grad():
    vision_embeds = model.get_vision_features(pixel_values=img_inputs.pixel_values)

# Run multiple text prompts efficiently
text_prompts = ["ear", "eye", "nose"]
all_results = []

for prompt in text_prompts:
    text_inputs = processor(text=prompt, return_tensors="pt").to(model.device)
    with torch.no_grad():
        outputs = model(vision_embeds=vision_embeds, **text_inputs)
...
    results = processor.post_process_instance_segmentation(
        outputs,
        threshold=0.5,
        mask_threshold=0.5,
        target_sizes=img_inputs.get("original_sizes").tolist()
    )[0]
    all_results.append({"prompt": prompt, "results": results})

for item in all_results:
    print(f"Prompt '{item['prompt']}': {len(item['results']['masks'])} objects found")
```

### Efficient Single-Prompt Inference on Multiple Images

When running the same text prompt on multiple images, pre-compute text embeddings to avoid redundant computation:

```python
from transformers import Sam3Processor, Sam3Model
import torch
from PIL import Image
import requests

model = Sam3Model.from_pretrained("facebook/sam3", device_map="auto")
processor = Sam3Processor.from_pretrained("facebook/sam3")

# Pre-compute text embeddings once
text_prompt = "ear"
text_inputs = processor(text=text_prompt, return_tensors="pt").to(model.device)
with torch.no_grad():
    text_embeds = model.get_text_features(**text_inputs)

# Load multiple images
image_urls = [
    "http://images.cocodataset.org/val2017/000000077595.jpg",
    "http://images.cocodataset.org/val2017/000000039769.jpg",
]
images = [Image.open(requests.get(url, stream=True).raw).convert("RGB") for url in image_urls]

# Run inference on each image reusing text embeddings
# Note: attention_mask must be passed along with text_embeds for proper masking
all_results = []

for image in images:
    img_inputs = processor(images=image, return_tensors="pt").to(model.device)
    with torch.no_grad():
        outputs = model(
            pixel_values=img_inputs.pixel_values,
            text_embeds=text_embeds,
            attention_mask=text_inputs.attention_mask,
        )
...
    results = processor.post_process_instance_segmentation(
        outputs,
        threshold=0.5,
        mask_threshold=0.5,
        target_sizes=img_inputs.get("original_sizes").tolist()
    )[0]
    all_results.append(results)

for i, results in enumerate(all_results):
    print(f"Image {i+1}: {len(results['masks'])} '{text_prompt}' objects found")
```

### Custom Resolution Inference

⚠️ **Performance Note**: Custom resolutions may degrade accuracy. The model is meant to be used at 1008px resolution.

For faster inference or lower memory usage:

```python
config = Sam3Config.from_pretrained("facebook/sam3")
config.image_size = 560
model = Sam3Model.from_pretrained("facebook/sam3", config=config, device_map="auto")
processor = Sam3Processor.from_pretrained("facebook/sam3", size={"height": 560, "width": 560})
```

### Prompt Label Conventions

SAM3 uses the following label conventions:

**For points and boxes:**

- `1`: Positive prompt (include this region/object)
- `0`: Negative prompt (exclude this region/object)
- `-10`: Padding value for batched inputs

**Coordinate formats:**

- **Input boxes**: `[x1, y1, x2, y2]` (xyxy format) in pixel coordinates
- **Output boxes** (raw): `[x1, y1, x2, y2]` (xyxy format), normalized to [0, 1]
- **Output boxes** (post-processed): `[x1, y1, x2, y2]` (xyxy format) in absolute pixel coordinates

## Sam3Config[[transformers.Sam3Config]]

#### transformers.Sam3Config[[transformers.Sam3Config]]

```python
transformers.Sam3Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, geometry_encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, detr_encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, detr_decoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, mask_decoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/configuration_sam3.py#L203)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

geometry_encoder_config (`dict` or `Sam3GeometryEncoderConfig`, *optional*) : Configuration for the geometry encoder.

detr_encoder_config (`dict` or `Sam3DETREncoderConfig`, *optional*) : Configuration for the DETR encoder.

detr_decoder_config (`dict` or `Sam3DETRDecoderConfig`, *optional*) : Configuration for the DETR decoder.

mask_decoder_config (`dict` or `Sam3MaskDecoderConfig`, *optional*) : Configuration for the mask decoder.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import Sam3Config, Sam3Model

>>> # Initializing a SAM3 configuration
>>> configuration = Sam3Config()

>>> # Initializing a model from the configuration
>>> model = Sam3Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Sam3ViTConfig[[transformers.Sam3ViTConfig]]

#### transformers.Sam3ViTConfig[[transformers.Sam3ViTConfig]]

```python
transformers.Sam3ViTConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 1024, intermediate_size: int = 4736, num_hidden_layers: int = 32, num_attention_heads: int = 16, num_channels: int = 3, image_size: int | list[int] | tuple[int, int] = 1008, patch_size: int | list[int] | tuple[int, int] = 14, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06, attention_dropout: float | int = 0.0, rope_theta: float = 10000.0, window_size: int = 24, global_attn_indexes: list[int] | None = None, layer_scale_init_value: float | None = None, pretrain_image_size: int | list[int] | tuple[int, int] = 336, hidden_dropout: float | int = 0.0, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/configuration_sam3.py#L27)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `4736`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1008`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) : The size (resolution) of each patch.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

rope_theta (`float`, *optional*, defaults to 10000.0) : Base frequency for RoPE.

window_size (`int`, *optional*, defaults to 24) : Window size for windowed attention.

global_attn_indexes (`list[int]`, *optional*, defaults to `[7, 15, 23, 31]`) : Indexes of layers with global attention.

layer_scale_init_value (`float`, *optional*) : Scale to use in the self-attention layers. 0.1 for base, 1e-6 for large. Set 0 to disable layer scale.

pretrain_image_size (`int`, *optional*, defaults to 336) : Pretrained model image size for position embedding initialization.

hidden_dropout (`float`, *optional*, defaults to 0.0) : Dropout probability for hidden states.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3VisionConfig[[transformers.Sam3VisionConfig]]

#### transformers.Sam3VisionConfig[[transformers.Sam3VisionConfig]]

```python
transformers.Sam3VisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, fpn_hidden_size: int = 256, backbone_feature_sizes: list | None = None, scale_factors: list[float] | None = None, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/configuration_sam3.py#L70)

**Parameters:**

backbone_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The configuration of the backbone model.

fpn_hidden_size (`int`, *optional*, defaults to 256) : The hidden dimension of the FPN.

backbone_feature_sizes (`List[List[int]]`, *optional*, defaults to `[[288, 288], [144, 144], [72, 72]]`) : The spatial sizes (height, width) of the feature maps from the backbone at different scales.

scale_factors (`list[float]`, *optional*, defaults to `[4.0, 2.0, 1.0, 0.5]`) : Scale factors for FPN multi-scale features. List of scaling factors for each FPN level.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3GeometryEncoderConfig[[transformers.Sam3GeometryEncoderConfig]]

#### transformers.Sam3GeometryEncoderConfig[[transformers.Sam3GeometryEncoderConfig]]

```python
transformers.Sam3GeometryEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 256, num_layers: int = 3, num_attention_heads: int = 8, intermediate_size: int = 2048, dropout: float | int = 0.1, hidden_act: str = 'relu', hidden_dropout: float | int = 0.0, layer_norm_eps: float = 1e-06, roi_size: int = 7, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/configuration_sam3.py#L120)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

num_layers (`int`, *optional*, defaults to `3`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `2048`) : Dimension of the MLP representations.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

roi_size (`int`, *optional*, defaults to 7) : ROI size for box pooling operations.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3DETREncoderConfig[[transformers.Sam3DETREncoderConfig]]

#### transformers.Sam3DETREncoderConfig[[transformers.Sam3DETREncoderConfig]]

```python
transformers.Sam3DETREncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 256, num_layers: int = 6, num_attention_heads: int = 8, intermediate_size: int = 2048, dropout: float | int = 0.1, hidden_act: str = 'relu', hidden_dropout: float | int = 0.0, layer_norm_eps: float = 1e-06, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/configuration_sam3.py#L142)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

num_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `2048`) : Dimension of the MLP representations.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout (`float`, *optional*, defaults to 0.0) : Dropout probability for hidden states.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3DETRDecoderConfig[[transformers.Sam3DETRDecoderConfig]]

#### transformers.Sam3DETRDecoderConfig[[transformers.Sam3DETRDecoderConfig]]

```python
transformers.Sam3DETRDecoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 256, num_layers: int = 6, num_queries: int = 200, num_attention_heads: int = 8, intermediate_size: int = 2048, dropout: float | int = 0.1, hidden_act: str = 'relu', hidden_dropout: float | int = 0.0, layer_norm_eps: float = 1e-06, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/configuration_sam3.py#L163)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

num_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder.

num_queries (`int`, *optional*, defaults to 200) : Number of object queries.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `2048`) : Dimension of the MLP representations.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3MaskDecoderConfig[[transformers.Sam3MaskDecoderConfig]]

#### transformers.Sam3MaskDecoderConfig[[transformers.Sam3MaskDecoderConfig]]

```python
transformers.Sam3MaskDecoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 256, num_upsampling_stages: int = 3, layer_norm_eps: float = 1e-06, dropout: float | int = 0.0, num_attention_heads: int = 8, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/configuration_sam3.py#L185)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

num_upsampling_stages (`int`, *optional*, defaults to 3) : Number of upsampling stages in the pixel decoder (FPN).

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The ratio for all dropout layers.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3Processor[[transformers.Sam3Processor]]

#### transformers.Sam3Processor[[transformers.Sam3Processor]]

```python
transformers.Sam3Processor(image_processor, tokenizer, target_size: int | None = None, point_pad_value: int = -10, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/processing_sam3.py#L87)

**Parameters:**

image_processor (`Sam3ImageProcessor`) : The image processor is a required input.

tokenizer (`CLIPTokenizer`) : The tokenizer is a required input.

target_size (`int`, *optional*) : The target size (target_size, target_size) to which the image will be resized.

point_pad_value (`int`, *optional*, defaults to -10) : The value used for padding input boxes.

Constructs a Sam3Processor which wraps a image processor and a tokenizer into a single processor.

[Sam3Processor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3Processor) offers all the functionalities of [Sam3ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3ImageProcessor) and [CLIPTokenizer](/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer). See the
[~Sam3ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3ImageProcessor) and [~CLIPTokenizer](/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer) for more information.

#### __call__[[transformers.Sam3Processor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: str | list[str] | list[list[str]] | None = None, segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, input_boxes: typing.Union[list[list[list[float]]], torch.Tensor, NoneType] = None, input_boxes_labels: typing.Union[list[list[list[int]]], torch.Tensor, NoneType] = None, original_sizes: typing.Union[list[list[float]], torch.Tensor, NoneType] = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/processing_sam3.py#L101)

**Parameters:**

images (`ImageInput`, *optional*) : The image(s) to process.

text (`str`, `list[str]`, `list[list[str]]`, *optional*) : The text to process.

segmentation_maps (`ImageInput`, *optional*) : The segmentation maps to process.

input_boxes (`list[list[list[float]]]`, `torch.Tensor`, *optional*) : The bounding boxes to process.

input_boxes_labels (`list[list[int]]`, `torch.Tensor`, *optional*) : The labels for the bounding boxes.

original_sizes (`list[list[float]]`, `torch.Tensor`, *optional*) : The original sizes of the images.

return_tensors (`Union[str, ~utils.generic.TensorType]`, *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

**Returns:** A [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields

- `pixel_values` (`torch.Tensor`): The processed image(s).
- `original_sizes` (`list[list[float]]`): The original sizes of the images.
- `labels` (`torch.Tensor`): The processed segmentation maps (if provided).
- `input_boxes_labels` (`torch.Tensor`): The processed labels for the bounding boxes.
- `input_boxes` (`torch.Tensor`): The processed bounding boxes.

## Sam3ImageProcessor[[transformers.Sam3ImageProcessor]]

#### transformers.Sam3ImageProcessor[[transformers.Sam3ImageProcessor]]

```python
transformers.Sam3ImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/image_processing_sam3.py#L401)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 1008, 'width': 1008}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

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

Constructs a Sam3ImageProcessor image processor.

#### preprocess[[transformers.Sam3ImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/image_processing_sam3.py#L421)

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

## Sam3ViTModel[[transformers.Sam3ViTModel]]

#### transformers.Sam3ViTModel[[transformers.Sam3ViTModel]]

```python
transformers.Sam3ViTModel(config: Sam3ViTConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/modeling_sam3.py#L801)

**Parameters:**

config ([Sam3ViTConfig](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3ViTConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Sam3 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Sam3ViTModel.forward]]

```python
forward(pixel_values: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/modeling_sam3.py#L824)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [Sam3ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3ImageProcessor). See `Sam3ImageProcessor.__call__()` for details ([Sam3Processor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3Processor) uses [Sam3ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3ImageProcessor) for processing images).

**Returns:** [BaseModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`

A [BaseModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam3Config](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3Config)) and inputs.

The [Sam3ViTModel](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3ViTModel) forward method, overrides the `__call__` special method.

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

## Sam3VisionModel[[transformers.Sam3VisionModel]]

#### transformers.Sam3VisionModel[[transformers.Sam3VisionModel]]

```python
transformers.Sam3VisionModel(config: Sam3VisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/modeling_sam3.py#L1061)

**Parameters:**

config ([Sam3VisionConfig](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3VisionConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The vision model from Sam without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Sam3VisionModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/modeling_sam3.py#L1076)

## Sam3Model[[transformers.Sam3Model]]

#### transformers.Sam3Model[[transformers.Sam3Model]]

```python
transformers.Sam3Model(config: Sam3Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/modeling_sam3.py#L2157)

#### forward[[transformers.Sam3Model.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, vision_embeds: transformers.models.sam3.modeling_sam3.Sam3VisionEncoderOutput | None = None, input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, text_embeds: typing.Optional[torch.FloatTensor] = None, input_boxes: typing.Optional[torch.FloatTensor] = None, input_boxes_labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/modeling_sam3.py#L2270)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Sam3ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3ImageProcessor). See `Sam3ImageProcessor.__call__()` for details ([Sam3Processor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3Processor) uses [Sam3ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3ImageProcessor) for processing images).

vision_embeds (`Sam3VisionEncoderOutput`, *optional*) : Pre-computed vision embeddings. Can be used to easily reuse vision embeddings. If provided, `pixel_values` should not be passed. Mutually exclusive with `pixel_values`.

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

text_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Pre-computed text embeddings. Can be used to easily reuse text embeddings. If provided, `input_ids` should not be passed. Mutually exclusive with `input_ids`.

input_boxes (`torch.FloatTensor` of shape `(batch_size, num_boxes, 4)`, *optional*) : Normalized box coordinates in [0, 1] range, in (cx, cy, w, h) format.

input_boxes_labels (`torch.LongTensor` of shape `(batch_size, num_boxes)`, *optional*) : Labels for boxes: 1 (positive), 0 (negative).

**Returns:** `Sam3ImageSegmentationOutput` or `tuple(torch.FloatTensor)`

A `Sam3ImageSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam3Config](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3Config)) and inputs.

The [Sam3Model](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, num_queries, height, width)`) -- Predicted segmentation masks for each query.
- **pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- Predicted bounding boxes in (x1, y1, x2, y2) format.
- **pred_logits** (`torch.FloatTensor` of shape `(batch_size, num_queries)`, *optional*) -- Classification confidence scores for each query, computed via dot product between
  decoder query features and text features.
- **presence_logits** (`torch.FloatTensor` of shape `(batch_size, 1)`, *optional*) -- Presence logits from the DETR decoder presence token (last layer only). These indicate whether objects
  are present in the scene. Can be used to compute final scores by multiplying with pred_logits:
  `final_scores = pred_logits.sigmoid() * presence_logits.sigmoid()`.
- **semantic_seg** (`torch.FloatTensor` of shape `(batch_size, 1, height, width)`, *optional*) -- Semantic segmentation output.
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*) -- Tuple of hidden states from all DETR decoder layers. Each tensor has shape `(batch_size, num_queries, hidden_size)`.
- **decoder_reference_boxes** (`torch.FloatTensor` of shape `(num_layers, batch_size, num_queries, 4)`, *optional*) -- Reference boxes from all DETR decoder layers.
- **encoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*) -- Tuple of hidden states from all DETR encoder layers.
- **vision_hidden_states** (`tuple[torch.FloatTensor]`, *optional*) -- Tuple of hidden states from all vision encoder (ViT) layers.
- **vision_attentions** (`tuple[torch.FloatTensor]`, *optional*) -- Attention weights from vision encoder (ViT) layers.
- **detr_encoder_attentions** (`tuple[torch.FloatTensor]`, *optional*) -- Attention weights from DETR encoder layers.
- **detr_decoder_attentions** (`tuple[torch.FloatTensor]`, *optional*) -- Attention weights from DETR decoder layers (self-attention and cross-attention).
- **mask_decoder_attentions** (`tuple[torch.FloatTensor]`, *optional*) -- Attention weights from mask decoder layers.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoModel, AutoProcessor

>>> model = AutoModel.from_pretrained("facebook/sam3")
>>> processor = AutoProcessor.from_pretrained("facebook/sam3")

>>> url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/sam-car.png"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read())).convert("RGB")
>>> text = "car"
>>> inputs = processor(images=image, text=text, return_tensors="pt")

>>> # Get segmentation output
>>> outputs = model(**inputs)
>>> pred_masks = outputs.pred_masks
>>> pred_boxes = outputs.pred_boxes
```

#### get_text_features[[transformers.Sam3Model.get_text_features]]

```python
get_text_features(input_ids: LongTensor, attention_mask: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3/modeling_sam3.py#L2197)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam3Config](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3Config)) and inputs.

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
>>> from transformers import Sam3Model, Sam3Processor
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> model = Sam3Model.from_pretrained("facebook/sam3")
>>> processor = Sam3Processor.from_pretrained("facebook/sam3")

>>> # Pre-compute text embeddings
>>> text_inputs = processor(text="cat", return_tensors="pt")
>>> text_embeds = model.get_text_features(**text_inputs).pooler_output

>>> # Reuse text embeddings for multiple images
>>> url = "http://images.cocodataset.org/val2017/000000077595.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> img_inputs = processor(images=image, return_tensors="pt")
>>> outputs = model(pixel_values=img_inputs.pixel_values, text_embeds=text_embeds)
```
