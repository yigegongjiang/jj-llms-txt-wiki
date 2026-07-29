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

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **geometry_encoder_config** (`dict` or `Sam3GeometryEncoderConfig`, *optional*) --
  Configuration for the geometry encoder.
- **detr_encoder_config** (`dict` or `Sam3DETREncoderConfig`, *optional*) --
  Configuration for the DETR encoder.
- **detr_decoder_config** (`dict` or `Sam3DETRDecoderConfig`, *optional*) --
  Configuration for the DETR decoder.
- **mask_decoder_config** (`dict` or `Sam3MaskDecoderConfig`, *optional*) --
  Configuration for the mask decoder.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `4736`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1008`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) --
  The size (resolution) of each patch.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **rope_theta** (`float`, *optional*, defaults to 10000.0) --
  Base frequency for RoPE.
- **window_size** (`int`, *optional*, defaults to 24) --
  Window size for windowed attention.
- **global_attn_indexes** (`list[int]`, *optional*, defaults to `[7, 15, 23, 31]`) --
  Indexes of layers with global attention.
- **layer_scale_init_value** (`float`, *optional*) --
  Scale to use in the self-attention layers. 0.1 for base, 1e-6 for large. Set 0 to disable layer scale.
- **pretrain_image_size** (`int`, *optional*, defaults to 336) --
  Pretrained model image size for position embedding initialization.
- **hidden_dropout** (`float`, *optional*, defaults to 0.0) --
  Dropout probability for hidden states.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3VisionConfig[[transformers.Sam3VisionConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **fpn_hidden_size** (`int`, *optional*, defaults to 256) --
  The hidden dimension of the FPN.
- **backbone_feature_sizes** (`List[List[int]]`, *optional*, defaults to `[[288, 288], [144, 144], [72, 72]]`) --
  The spatial sizes (height, width) of the feature maps from the backbone at different scales.
- **scale_factors** (`list[float]`, *optional*, defaults to `[4.0, 2.0, 1.0, 0.5]`) --
  Scale factors for FPN multi-scale features. List of scaling factors for each FPN level.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3GeometryEncoderConfig[[transformers.Sam3GeometryEncoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **num_layers** (`int`, *optional*, defaults to `3`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the MLP representations.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **hidden_act** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **roi_size** (`int`, *optional*, defaults to 7) --
  ROI size for box pooling operations.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3DETREncoderConfig[[transformers.Sam3DETREncoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **num_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the MLP representations.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **hidden_act** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout** (`float`, *optional*, defaults to 0.0) --
  Dropout probability for hidden states.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3DETRDecoderConfig[[transformers.Sam3DETRDecoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **num_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer decoder.
- **num_queries** (`int`, *optional*, defaults to 200) --
  Number of object queries.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the MLP representations.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **hidden_act** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3MaskDecoderConfig[[transformers.Sam3MaskDecoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **num_upsampling_stages** (`int`, *optional*, defaults to 3) --
  Number of upsampling stages in the pixel decoder (FPN).
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3Model. It is used to instantiate a Sam3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3Processor[[transformers.Sam3Processor]]

- **image_processor** (`Sam3ImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`CLIPTokenizer`) --
  The tokenizer is a required input.
- **target_size** (`int`, *optional*) --
  The target size (target_size, target_size) to which the image will be resized.
- **point_pad_value** (`int`, *optional*, defaults to -10) --
  The value used for padding input boxes.
Constructs a Sam3Processor which wraps a image processor and a tokenizer into a single processor.

[Sam3Processor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3Processor) offers all the functionalities of [Sam3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3ImageProcessor) and [CLIPTokenizer](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPTokenizer). See the
[~Sam3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3ImageProcessor) and [~CLIPTokenizer](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPTokenizer) for more information.

- **images** (`ImageInput`, *optional*) --
  The image(s) to process.
- **text** (`str`, `list[str]`, `list[list[str]]`, *optional*) --
  The text to process.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to process.
- **input_boxes** (`list[list[list[float]]]`, `torch.Tensor`, *optional*) --
  The bounding boxes to process.
- **input_boxes_labels** (`list[list[int]]`, `torch.Tensor`, *optional*) --
  The labels for the bounding boxes.
- **original_sizes** (`list[list[float]]`, `torch.Tensor`, *optional*) --
  The original sizes of the images.
- **return_tensors** (`Union[str, ~utils.generic.TensorType]`, *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.A [BatchEncoding](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields- `pixel_values` (`torch.Tensor`): The processed image(s).
- `original_sizes` (`list[list[float]]`): The original sizes of the images.
- `labels` (`torch.Tensor`): The processed segmentation maps (if provided).
- `input_boxes_labels` (`torch.Tensor`): The processed labels for the bounding boxes.
- `input_boxes` (`torch.Tensor`): The processed bounding boxes.

## Sam3ImageProcessor[[transformers.Sam3ImageProcessor]]

- **mask_size** (`dict[str, *kwargs*, int]`, *optional*) --
  The size `{"height": int, "width": int}` to resize the segmentation maps to.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Sam3ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **mask_size** (`dict[str, *kwargs*, int]`, *optional*) --
  The size `{"height": int, "width": int}` to resize the segmentation maps to.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Sam3ViTModel[[transformers.Sam3ViTModel]]

- **config** ([Sam3ViTConfig](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3ViTConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Sam3 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sam3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3ImageProcessor). See `Sam3ImageProcessor.__call__()` for details ([Sam3Processor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3Processor) uses
  [Sam3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3ImageProcessor) for processing images).[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam3Config](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3Config)) and inputs.
The [Sam3ViTModel](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3ViTModel) forward method, overrides the `__call__` special method.

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

- **config** ([Sam3VisionConfig](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3VisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The vision model from Sam without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## Sam3Model[[transformers.Sam3Model]]

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sam3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3ImageProcessor). See `Sam3ImageProcessor.__call__()` for details ([Sam3Processor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3Processor) uses
  [Sam3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3ImageProcessor) for processing images).
- **vision_embeds** (`Sam3VisionEncoderOutput`, *optional*) --
  Pre-computed vision embeddings. Can be used to easily reuse vision embeddings. If provided, `pixel_values`
  should not be passed. Mutually exclusive with `pixel_values`.
- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Pre-computed text embeddings. Can be used to easily reuse text embeddings. If provided, `input_ids`
  should not be passed. Mutually exclusive with `input_ids`.
- **input_boxes** (`torch.FloatTensor` of shape `(batch_size, num_boxes, 4)`, *optional*) --
  Normalized box coordinates in [0, 1] range, in (cx, cy, w, h) format.
- **input_boxes_labels** (`torch.LongTensor` of shape `(batch_size, num_boxes)`, *optional*) --
  Labels for boxes: 1 (positive), 0 (negative).`Sam3ImageSegmentationOutput` or `tuple(torch.FloatTensor)`A `Sam3ImageSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam3Config](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3Config)) and inputs.
The [Sam3Model](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3Model) forward method, overrides the `__call__` special method.

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

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam3Config](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3Config)) and inputs.

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
