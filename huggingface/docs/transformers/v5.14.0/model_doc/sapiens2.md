# Sapiens2

    
        
        
    

## Overview

The Sapiens2 model was proposed in [Sapiens2](https://huggingface.co/papers/2604.21681) by Rawal Khirodkar, He Wen, Julieta Martinez, Yuan Dong, Zhaoen Su, Shunsuke Saito.
Sapiens2 is a family of high-resolution vision transformers pretrained on ~1 billion curated human images, designed for human-centric computer vision tasks including pose estimation, body-part segmentation, surface normal estimation, and pointmap estimation.

You can find all the original Sapiens2 checkpoints under the [Sapiens2](https://huggingface.co/collections/facebook/sapiens2) collection.

The abstract from the paper is the following:

*We present Sapiens2, a family of high-resolution transformers for human-centric vision focused on generalization, versatility, and high-fidelity outputs. We pretrain on ~1 billion curated high-quality human images with improved task annotations and combine masked image reconstruction with self-distilled contrastive objectives to learn both low-level and semantic features. Our models scale from 0.4B to 5B parameters and train at native 1K resolution, with hierarchical 4K variants for extended spatial reasoning. Sapiens2 achieves substantial improvements over its predecessor: +4 mAP in pose estimation, +24.3 mIoU in body-part segmentation, and 45.6% error reduction in normal estimation, while extending to new tasks like pointmap and albedo estimation. Code is publicly available.*

Tips:

- Sapiens2 uses Rotary Position Embeddings (RoPE) and supports arbitrary input resolutions. The default image processor resizes images to 1024×768 (height×width).
- The model uses Grouped Query Attention (GQA) for middle layers and full multi-head attention for the first and last 8 layers.
- Register tokens (8 by default) reduce high-norm artifacts in patch tokens, yielding cleaner attention maps and better performance on dense prediction tasks.

This model was contributed by [guarin](https://huggingface.co/guarin).
The original code can be found [here](https://github.com/facebookresearch/sapiens2).

## Usage examples

The example below shows how to obtain the CLS token (whole-image embedding) with [Sapiens2Model](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Model).

```python
import torch
from transformers import AutoImageProcessor, AutoModel
from transformers.image_utils import load_image

image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")

image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-pretrain-0.4b")
model = AutoModel.from_pretrained("facebook/sapiens2-pretrain-0.4b", device_map="auto")

inputs = image_processor(images=image, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

# outputs.pooler_output is the CLS token (whole-image embedding)
cls_token = outputs.pooler_output
print("CLS token shape:", cls_token.shape)  # [1, 1024]
```

[Sapiens2Backbone](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Backbone) exposes patch tokens already reshaped to spatial dimensions and CLS tokens directly on the output object.

```python
import torch
from transformers import AutoBackbone, AutoImageProcessor
from transformers.image_utils import load_image

image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")

image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-pretrain-0.4b")
model = AutoBackbone.from_pretrained("facebook/sapiens2-pretrain-0.4b", device_map="auto")

inputs = image_processor(images=image, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs, return_class_token=True)

# Patch tokens shaped (batch, height, width, channels)
patch_features = outputs.feature_maps[0]
cls_token = outputs.cls_tokens[0]
print("CLS token shape:", cls_token.shape)           # [1, 1024]
print("Patch features shape:", patch_features.shape) # [1, 64, 48, 1024]
```

The example below shows how to estimate surface normals with [Sapiens2ForNormalEstimation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForNormalEstimation).
The output normals are raw (unnormalized); use `post_process_normal_estimation` to resize and L2-normalize them.

```python
import torch
from transformers import AutoImageProcessor, AutoModelForNormalEstimation
from transformers.image_utils import load_image

image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")

image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-normal-0.4b")
model = AutoModelForNormalEstimation.from_pretrained("facebook/sapiens2-normal-0.4b", device_map="auto")

inputs = image_processor(image, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

# outputs.normals shape: (batch_size, 3, height, width) — raw, unnormalized XYZ normals
print("Normals shape:", outputs.normals.shape)  # [1, 3, 1024, 768]

# Remove preprocessing padding, resize to original size, and L2-normalize to unit vectors in [-1, 1]
original_size = (image.height, image.width)
result = image_processor.post_process_normal_estimation(
    outputs, source_sizes=[original_size], target_sizes=[original_size]
)
normals = result[0]["normals"]
print("Normals shape:", normals.shape)   # [3, original_height, original_width]

### Example code for visualization

# Convert L2-normalized normals in [-1, 1] to RGB in [0, 255]
normals_rgb = ((normals + 1.0) / 2.0 * 255.0).clamp(0, 255).to(torch.uint8)

# Apply background removal using the segmentation model output.
# `segmentation` is the output of `post_process_semantic_segmentation` — a (H, W) tensor
# of per-pixel class IDs, where class 0 is background.
background_mask = segmentation == 0
normals_rgb[:, background_mask] = 0
print("Normals RGB shape:", normals_rgb.shape)   # [3, original_height, original_width]
```

The example below shows how to estimate per-pixel 3D coordinates with [Sapiens2ForPointmapEstimation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForPointmapEstimation).
Use `post_process_pointmap_estimation` to remove preprocessing padding, resize to the original image size, and apply the predicted focal-length scale.

```python
import torch
from transformers import AutoImageProcessor, AutoModelForPointmapEstimation
from transformers.image_utils import load_image

image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")

image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-pointmap-0.4b")
model = AutoModelForPointmapEstimation.from_pretrained("facebook/sapiens2-pointmap-0.4b", device_map="auto")

inputs = image_processor(image, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

# outputs.pointmaps shape: (batch_size, 3, height, width) — raw XYZ in canonical camera space
print("Pointmaps shape:", outputs.pointmaps.shape)  # [1, 3, 1024, 768]

# Remove preprocessing padding, resize to original size, and apply focal-length scale
original_size = (image.height, image.width)
result = image_processor.post_process_pointmap_estimation(
    outputs, source_sizes=[original_size], target_sizes=[original_size]
)
pointmap = result[0]["pointmap"]
print("Pointmap shape:", pointmap.shape)  # [3, original_height, original_width]

### Example code for visualization

# Visualize the pointmap as an RGB image using inverse-depth and the turbo colormap.
import matplotlib.pyplot as plt

# `segmentation` is the output of `post_process_semantic_segmentation` — a (H, W) tensor
# of per-pixel class IDs, where class 0 is background.
foreground_mask = segmentation != 0
depth = pointmap[2]  # Z channel: depth in camera space, shape (H, W)
pointmap_rgb = torch.zeros(3, *depth.shape, dtype=torch.uint8)
foreground_depth = depth[foreground_mask]
if foreground_depth.numel() > 0:
    depth_low, depth_high = torch.quantile(foreground_depth, torch.tensor([0.01, 0.99]))
    inverse_depth = 1.0 / foreground_depth.clamp(min=1e-6)
    inverse_depth_low = 1.0 / depth_high.clamp(min=1e-6)
    inverse_depth_high = 1.0 / depth_low.clamp(min=1e-6)
    inverse_depth_normalized = ((inverse_depth - inverse_depth_low) / (inverse_depth_high - inverse_depth_low + 1e-8)).clamp(0, 1)
    turbo = plt.get_cmap("turbo")
    foreground_colors = torch.from_numpy(turbo(inverse_depth_normalized.cpu().numpy())[..., :3] * 255).to(torch.uint8)  # (N, 3)
    pointmap_rgb[:, foreground_mask] = foreground_colors.T
print("Pointmap RGB shape:", pointmap_rgb.shape)  # [3, original_height, original_width]
```

The example below shows how to run pose estimation with [Sapiens2ForPoseEstimation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForPoseEstimation).
The model predicts per-keypoint heatmaps; use `post_process_pose_estimation` to decode them back to
image-space keypoint coordinates. It requires `opencv-python` (`pip install opencv-python`).

```python
import torch
from transformers import AutoImageProcessor, AutoModelForPoseEstimation
from transformers.image_utils import load_image

image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")

image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-pose-0.4b")
model = AutoModelForPoseEstimation.from_pretrained("facebook/sapiens2-pose-0.4b", device_map="auto")

# Provide bounding boxes in COCO format (x, y, width, height) for each person
boxes = [[[270.8, 0.6, 294.1, 379.5]]]
inputs = image_processor(image, boxes=boxes, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

# outputs.heatmaps shape: (num_persons, num_keypoints, heatmap_height, heatmap_width)
print("Heatmaps shape:", outputs.heatmaps.shape)  # [1, 308, 256, 192]

# Decode heatmaps to image-space keypoint coordinates
results = image_processor.post_process_pose_estimation(outputs, boxes=boxes)[0]
keypoints = results[0]["keypoints"]   # (num_keypoints, 2) — x/y in image coordinates
scores = results[0]["scores"]         # (num_keypoints,) — per-keypoint confidence
print("Keypoints shape:", keypoints.shape)
```

Horizontal flip augmentation (test-time augmentation) improves keypoint accuracy by averaging
predictions from the original and mirrored image. Pass `flip_pairs` — a tensor of
`[left_keypoint, right_keypoint]` pairs — to the second forward pass. The model flips the heatmaps
back to the original orientation before returning them, so you can average both outputs directly.

```python
import torch
from transformers import AutoImageProcessor, AutoModelForPoseEstimation
from transformers.image_utils import load_image

image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")

image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-pose-0.4b")
model = AutoModelForPoseEstimation.from_pretrained("facebook/sapiens2-pose-0.4b", device_map="auto")

boxes = [[[270.8, 0.6, 294.1, 379.5]]]
inputs = image_processor(image, boxes=boxes, return_tensors="pt").to(model.device)
pixel_values = inputs["pixel_values"]

flip_pairs = torch.tensor(model.config.flip_pairs, device=model.device)

with torch.inference_mode():
    outputs = model(pixel_values)
    outputs_flipped = model(pixel_values.flip(-1), flip_pairs=flip_pairs)

results = image_processor.post_process_pose_estimation(outputs, outputs_flipped=outputs_flipped, boxes=boxes)[0]
keypoints = results[0]["keypoints"]
scores = results[0]["scores"]
```

The example below shows how to compute the supervised Masked Mean Squared Error (MSE) loss with [Sapiens2ForPoseEstimation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForPoseEstimation) by passing `labels` and optional `label_weights`. This is useful for fine-tuning using the `Trainer` API.

```python
import torch
from transformers import AutoImageProcessor, AutoModelForPoseEstimation
from transformers.image_utils import load_image

image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")

image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-pose-0.4b")
model = AutoModelForPoseEstimation.from_pretrained("facebook/sapiens2-pose-0.4b", device_map="auto")

# Provide bounding boxes in COCO format (x, y, width, height) for each person
boxes = [[[270.8, 0.6, 294.1, 379.5]]]
inputs = image_processor(image, boxes=boxes, return_tensors="pt").to(model.device)

# Create dummy labels (heatmaps) and visibility weights to simulate ground truth
# 1.0 for visible keypoints, 0.0 for occluded/invisible keypoints
batch_size, num_keypoints = 1, 308
heatmap_height, heatmap_width = 1024, 768
labels = torch.randn(batch_size, num_keypoints, heatmap_height, heatmap_width, device=model.device)
label_weights = torch.ones(batch_size, num_keypoints, 1, 1, device=model.device)

# Forward pass with loss calculation
outputs = model(**inputs, labels=labels, label_weights=label_weights)

print("Loss:", outputs.loss.item())
```

The example below shows how to perform body-part segmentation with [Sapiens2ForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForSemanticSegmentation).

```python
import torch
from transformers import AutoImageProcessor, AutoModelForSemanticSegmentation
from transformers.image_utils import load_image

image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")

image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-seg-0.4b")
model = AutoModelForSemanticSegmentation.from_pretrained("facebook/sapiens2-seg-0.4b", device_map="auto")

inputs = image_processor(image, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

# outputs.logits shape: (batch_size, num_labels, height, width)
print("Logits shape:", outputs.logits.shape)  # [1, 29, 1024, 768]

# Get per-pixel class predictions, optionally resized to the original image size
original_size = (image.height, image.width)
segmentation = image_processor.post_process_semantic_segmentation(
    outputs, target_sizes=[original_size]
)[0]
print("Segmentation map shape:", segmentation.shape)  # [original_height, original_width]
```

The example below shows how to run image matting with [Sapiens2ForImageMatting](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForImageMatting).
Outputs are sigmoid-activated and already in `[0, 1]`; use `post_process_image_matting` to resize and split
into `alphas`, `foregrounds`, and an optional `composite` image. The composite image shows
the foreground overlaid over the background with the formula: `composite = foreground * (1 - alpha) * background`.

```python
import torch
from transformers import AutoImageProcessor, AutoModelForImageMatting
from transformers.image_utils import load_image

image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")

image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-matting-1b")
model = AutoModelForImageMatting.from_pretrained("facebook/sapiens2-matting-1b", device_map="auto")

inputs = image_processor(image, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

# outputs.foregrounds: (1, 3, H, W), outputs.alphas: (1, 1, H, W) — both in [0, 1]
original_size = (image.height, image.width)

# Pass an optional background to composite the foreground over it.
# A (3, 1, 1) tensor broadcasts as a uniform color; PIL images and numpy arrays are also accepted.
background = torch.tensor([0, 177, 64], dtype=torch.uint8).view(3, 1, 1)  # chroma green in RGB
result = image_processor.post_process_image_matting(
    outputs, target_sizes=[original_size], backgrounds=background
)[0]
print("Alpha shape:", result["alpha"].shape)        # [1, original_height, original_width]
print("Foreground shape:", result["foreground"].shape)  # [3, original_height, original_width]
print("Composite shape:", result["composite"].shape)    # [3, original_height, original_width] — uint8 [0, 255]
```

## Sapiens2Config[[transformers.Sapiens2Config]]

- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rope_theta** (`float`, *optional*, defaults to 100.0) --
  The base period of the RoPE embeddings.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **query_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the query projection.
- **key_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to add a bias to the key projection.
- **value_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the value projection.
- **proj_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the output projection.
- **mlp_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.
- **layerscale_value** (`float`, *optional*, defaults to 1.0) --
  Initial value to use for layer scale.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  Drop path rate for the patch fusion.
- **use_gated_mlp** (`bool`, *optional*, defaults to `False`) --
  Whether to use the SwiGLU feedforward neural network.
- **num_register_tokens** (`int`, *optional*, defaults to 0) --
  The number of register tokens.
- **pos_embed_shift** (`float`, *optional*) --
  Amount to randomly shift position embedding coordinates in [-shift, shift],
  applied only in training mode if not `None`.
- **pos_embed_jitter** (`float`, *optional*) --
  Amount to randomly jitter position embedding coordinates in log-uniform value in [1/jitter, jitter],
  applied only in training mode if not `None`.
- **pos_embed_rescale** (`float`, *optional*, defaults to 2.0) --
  Amount to randomly rescale position embedding coordinates in log-uniform value in [1/rescale, rescale],
  applied only in training mode if not `None`.
- **reshape_hidden_states** (`bool`, *optional*, defaults to `True`) --
  Whether to reshape the hidden states to spatial dimensions when used as backbone.
- **use_mask_token** (`bool`, *optional*, defaults to `False`) --
  Whether to use a mask token in the embeddings (needed for masked image modeling pretraining).
- **rms_norm_eps** (`float`, *optional*, defaults to 1e-6) --
  Epsilon for the RMS normalization layers.
- **normalize_backbone_outputs** (`bool`, *optional*, defaults to `True`) --
  Whether to apply RMSNorm to the backbone `feature_maps` and `cls_tokens` outputs before
  returning them from the forward pass. Only applies when the model is used as a backbone.
- **use_qk_norm** (`bool`, *optional*, defaults to `True`) --
  Whether to apply RMSNorm to queries and keys before RoPE in attention layers.
- **num_key_value_heads_per_layer** (`list[int]`, *optional*) --
  Number of key/value heads for each transformer layer. Setting a layer's value equal to
  `num_attention_heads` gives full multi-head attention; a smaller value gives grouped-query
  attention. Defaults to `num_attention_heads` for the first `num_first_full_attention_layers`
  and last `num_last_full_attention_layers` layers and `num_key_valueattention_heads` for all other
  layers.
- **num_key_value_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of key/value heads for layers that use grouped-query attention when `num_key_value_heads_per_layer`
  is not set. Ignored when `num_key_value_heads_per_layer` is set.
- **num_first_full_attention_layers** (`int`, *optional*, defaults to 8) --
  Number of leading transformer layers that use full multi-head attention.
  Only used when `num_key_value_heads_per_layer` is `None`.
- **num_last_full_attention_layers** (`int`, *optional*, defaults to 8) --
  Number of trailing transformer layers that use full multi-head attention.
  Only used when `num_key_value_heads_per_layer` is `None`.
- **semantic_loss_ignore_index** (`int`, *optional*, defaults to 255) --
  Label index ignored when computing the segmentation loss.
- **flip_pairs** (`list[list[int]]`, *optional*) --
  Pairs of keypoint indices that are mirrored horizontally (e.g., left ear ↔ right ear).
  Each pair is a two-element list `[left_index, right_index]`. Used for test-time
  horizontal flip augmentation in pose estimation: pass these pairs to the second
  forward call so the model flips heatmaps back before returning them.
- **head_config** (`Sapiens2HeadConfig`, *optional*) --
  Configuration for the decode head. See [Sapiens2HeadConfig](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2HeadConfig) for the available options.

This is the configuration class to store the configuration of a Sapiens2Model. It is used to instantiate a Sapiens2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sapiens2-pretrain-0.4b](https://huggingface.co/facebook/sapiens2-pretrain-0.4b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sapiens2HeadConfig[[transformers.Sapiens2HeadConfig]]

- **upsample_out_channels** (`list[int]`, *optional*) --
  Output channel counts for each upsample block.
  The first block takes `hidden_size` channels as input; subsequent blocks use the previous output.
- **upsample_kernel_sizes** (`list[int]`, *optional*) --
  Kernel size for each upsample block. Auto-filled with `[4, ...]` when
  `upsample_out_channels` is set but this is `None`.
  Must have the same length as `upsample_out_channels`.
- **upsample_kernel_size** (`int`, defaults to 4) --
  Default kernel size for upsample blocks when `upsample_kernel_sizes` is not set.
- **use_pixel_shuffle** (`bool`, *optional*) --
  Whether the upsample head uses pixel-shuffle upsampling instead of transposed convolutions.
  When `None` (default), the head uses transposed convolutions.
- **conv_out_channels** (`list[int]`, *optional*) --
  Output channel counts for the refinement conv layers that follow the upsample blocks.
- **conv_kernel_sizes** (`list[int]`, *optional*) --
  Kernel size for each refinement conv layer. Auto-filled with `[1, ...]` when
  `conv_out_channels` is set but this is `None`.
  Must have the same length as `conv_out_channels`.
- **conv_kernel_size** (`int`, defaults to 1) --
  Default kernel size for conv layers when `conv_kernel_sizes` is not set.
- **scale_conv_out_channels** (`list[int]`, *optional*) --
  Output channel counts for the stride-2 conv layers used to predict the focal-length scale.
  When `None` (default), no scale branch is built.
- **scale_conv_kernel_sizes** (`list[int]`, *optional*) --
  Kernel size for each scale conv layer. Auto-filled with `[1, ...]` when
  `scale_conv_out_channels` is set but this is `None`.
  Must have the same length as `scale_conv_out_channels`.
- **scale_conv_kernel_size** (`int`, defaults to 1) --
  Default kernel size for scale conv layers when `scale_conv_kernel_sizes` is not set.
- **scale_final_input_size** (`int`, *optional*) --
  Flattened feature size passed into the scale MLP.
  When `None` (default), it is automatically inferred from `image_size` and `patch_size`
  in the parent [Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config).
- **scale_final_hidden_sizes** (`list[int]`, *optional*) --
  Hidden-layer sizes for the MLP that maps flattened scale features to the scalar scale output.
  When `None` (default), no scale branch is built.

This is the configuration class to store the configuration of a Sapiens2Model. It is used to instantiate a Sapiens2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sapiens2-seg-0.4b](https://huggingface.co/facebook/sapiens2-seg-0.4b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sapiens2ImageProcessor[[transformers.Sapiens2ImageProcessor]]

- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Sapiens2ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **boxes** (`list[list[list[float]]]` or `np.ndarray`, *optional*) --
  List or array of bounding boxes for each image. Each box should be a list of 4 floats
  representing the bounding box coordinates in COCO format
  (top_left_x, top_left_y, width, height). When provided, each person crop is
  affine-warped to the model input size instead of resizing the full image.
- **do_reduce_labels** (`bool`, *kwargs*, *optional*, defaults to `self.do_reduce_labels`) --
  Whether or not to reduce all label values of segmentation maps by 1. Usually used for datasets where 0
  is used for background, and background itself is not included in all classes of a dataset (e.g.
  ADE20k). The background label will be replaced by 255.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`Sapiens2ImageMattingOutput`) --
  Raw outputs of the model.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]` of length `batch_size`, *optional*) --
  Requested final `(height, width)` for each prediction. Resized with bilinear
  interpolation. If unset, predictions are returned at the model output resolution.
- **backgrounds** (`ImageInput`, *optional*) --
  Background image(s) to composite over. Can be a single image (applied to every item
  in the batch) or a list of images, one per batch item. Accepts PIL images, numpy
  arrays, or torch tensors of any dtype; integer types (e.g. uint8) are scaled to
  `[0, 1]` automatically. When provided, each result dict gains a `"composite"` key
  with the composited image as a uint8 tensor in `[0, 255]`.`list[dict]` of length `batch_size`. Each dict has- `"alpha"` (`torch.Tensor` of shape `(1, height, width)`): alpha values in `[0, 1]`.
- `"foreground"` (`torch.Tensor` of shape `(3, height, width)`): pre-multiplied RGB in `[0, 1]`.
- `"composite"` (`torch.Tensor` of shape `(3, height, width)` or `None`): foreground composited
  over `backgrounds` as a uint8 tensor in `[0, 255]`; `None` when `backgrounds` is not provided.

Converts the output of [Sapiens2ForImageMatting](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForImageMatting) into alpha mattes and foreground maps.

- **outputs** (`Sapiens2NormalEstimatorOutput`) --
  Raw outputs of the model.
- **source_sizes** (`torch.Tensor` or `list[tuple[int, int]]` of length `batch_size`, *optional*) --
  Original `(height, width)` of each image before preprocessing. When provided,
  the padding added during preprocessing is removed and predictions are resized back
  to the original image size (unless `target_sizes` overrides the final size).
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]` of length `batch_size`, *optional*) --
  Requested final `(height, width)` for each prediction. When provided, used as the
  resize target instead of `source_sizes`. Resized with bilinear interpolation after
  L2 normalization.
- **do_remove_padding** (`bool`, *optional*) --
  Whether to crop away the zero-padding added during preprocessing before resizing.
  Defaults to `True` when `source_sizes` is provided, `False` otherwise.`list[dict[str, torch.Tensor]]` of length `batch_size`. Each dict has a `"normals"` key
mapping to a tensor of shape `(3, height, width)` with L2-normalized unit vectors in
`[-1, 1]` per channel (XYZ surface normals).

Converts the output of [Sapiens2ForNormalEstimation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForNormalEstimation) into L2-normalized surface normal maps.

- **outputs** (`Sapiens2PointmapEstimatorOutput`) --
  Raw outputs of the model.
- **source_sizes** (`torch.Tensor` or `list[tuple[int, int]]` of length `batch_size`, *optional*) --
  Original `(height, width)` of each image before preprocessing. When provided,
  the padding added during preprocessing is removed and predictions are resized back
  to the original image size (unless `target_sizes` overrides the final size).
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]` of length `batch_size`, *optional*) --
  Requested final `(height, width)` for each prediction. Overrides `source_sizes`
  as the resize target.
- **do_remove_padding** (`bool`, *optional*) --
  Whether to crop away the zero-padding added during preprocessing before resizing.
  Defaults to `True` when `source_sizes` is provided, `False` otherwise.`list[dict[str, torch.Tensor]]` of length `batch_size`. Each dict has a `"pointmap"` key
mapping to a tensor of shape `(3, height, width)` with per-pixel 3D XYZ coordinates in
canonical camera space, optionally divided by `outputs.scales` to convert to metric coordinates.

Converts the output of [Sapiens2ForPointmapEstimation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForPointmapEstimation) into pointmap tensors in image space.

- **outputs** (`Sapiens2PoseEstimatorOutput`) --
  Raw outputs of the model. `outputs.heatmaps` must have shape
  `(N_total, num_keypoints, heatmap_height, heatmap_width)` where
  `N_total = sum(len(b) for b in boxes)`.
- **boxes** (`list[list[list[float]]]` or `np.ndarray`) --
  List or array of bounding boxes for each image in absolute pixel coordinates. Each box
  should be a list of 4 floats representing the bounding box coordinates in COCO format
  (top_left_x, top_left_y, width, height). Must match the `boxes` argument passed to
  `preprocess`.
- **outputs_flipped** (`Sapiens2PoseEstimatorOutput`, *optional*) --
  Outputs from running the model on horizontally flipped inputs. When provided, heatmaps
  are averaged with `outputs` before keypoint extraction to improve accuracy:
  `avg_heatmaps = (outputs.heatmaps + outputs_flipped.heatmaps) / 2`.
- **kernel_size** (`int`, *optional*, defaults to 11) --
  Kernel size for the Gaussian blur used in UDP Dark Pose refinement.
- **threshold** (`float`, *optional*) --
  Score threshold. Keypoints with scores at or below this value are
  filtered out from the result dictionaries.
- **source_sizes** (`torch.Tensor` or `list[tuple[int, int]]` of length `batch_size`, *optional*) --
  Original `(height, width)` of each image in pixels. Required when `target_sizes` is
  provided, as the source coordinate space for scaling keypoints and bounding boxes.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]` of length `batch_size`, *optional*) --
  Desired output `(height, width)` coordinate space for each image. When provided
  alongside `source_sizes`, keypoint coordinates and bounding boxes are scaled from
  source to target space.`list[list[dict]]`Outer list is over images, inner list is over persons.
Each dict contains:
- `keypoints` (`torch.FloatTensor` of shape `(num_keypoints, 2)`): absolute x/y coordinates in
  the source image space, or in target space if `target_sizes` is provided.
- `scores` (`torch.FloatTensor` of shape `(num_keypoints,)`): per-keypoint confidence.
- `labels` (`torch.LongTensor` of shape `(num_keypoints,)`): keypoint indices.
- `bbox` (`torch.FloatTensor` of shape `(4,)`): bounding box in absolute (x_min, y_min, x_max, y_max)
  format, in the same coordinate space as `keypoints`.

Converts the output of [Sapiens2ForPoseEstimation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForPoseEstimation) into keypoint predictions in image space.

- **outputs** ([Sapiens2ForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForSemanticSegmentation)) --
  Raw outputs of the model.
- **target_sizes** (`list[Tuple]` of length `batch_size`, *optional*) --
  List of tuples corresponding to the requested final size (height, width) of each prediction. If unset,
  predictions will not be resized.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size (if `target_sizes` is specified).

Converts the output of [Sapiens2ForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForSemanticSegmentation) into semantic segmentation maps.

## Sapiens2Model[[transformers.Sapiens2Model]]

- **config** ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Sapiens2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "bool_masked_pos", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor). See `Sapiens2ImageProcessor.__call__()` for details (`processor_class` uses
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor) for processing images).
- **bool_masked_pos** (`torch.BoolTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Boolean masked positions. Indicates which patches are masked (1) and which aren't (0). Only relevant for
  pre-training.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) and inputs.
The [Sapiens2Model](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Model) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, AutoModel
>>> from transformers.image_utils import load_image
>>> import torch

>>> image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-pretrain-0.4b")
>>> model = AutoModel.from_pretrained("facebook/sapiens2-pretrain-0.4b")

>>> inputs = image_processor(images=image, return_tensors="pt")
>>> with torch.inference_mode():
...     outputs = model(**inputs)

>>> cls_token = outputs.pooler_output
>>> cls_token.shape
torch.Size([1, 1024])
```

## Sapiens2Backbone[[transformers.Sapiens2Backbone]]

- **config** ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Sapiens2 backbone.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor). See `Sapiens2ImageProcessor.__call__()` for details (`processor_class` uses
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor) for processing images).`Sapiens2BackboneOutput` or `tuple(torch.FloatTensor)`A `Sapiens2BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) and inputs.
The [Sapiens2Backbone](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Backbone) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **feature_maps** (`tuple(torch.FloatTensor)` of shape `(batch_size, num_channels, height, width)`) -- Feature maps of the stages.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, sequence_length, hidden_size)` or `(batch_size, num_channels, height, width)`,
  depending on the backbone.

  Hidden-states of the model at the output of each stage plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Only applicable if the backbone uses attention.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **cls_tokens** (`tuple(torch.FloatTensor)`, *optional*) -- CLS token from each selected feature stage, each of shape `(batch_size, hidden_size)`.
  Only present when `config.return_class_token=True`.

Example:

```python
>>> from transformers import AutoBackbone, AutoImageProcessor
>>> from transformers.image_utils import load_image
>>> import torch

>>> image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-pretrain-0.4b")
>>> model = AutoBackbone.from_pretrained("facebook/sapiens2-pretrain-0.4b")

>>> inputs = image_processor(images=image, return_tensors="pt")
>>> with torch.inference_mode():
...     outputs = model(**inputs, return_class_token=True)

>>> outputs.feature_maps[0].shape
torch.Size([1, 1024, 64, 48])
>>> outputs.cls_tokens[0].shape
torch.Size([1, 1024])
```

## Sapiens2ForImageMatting[[transformers.Sapiens2ForImageMatting]]

- **config** ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Sapiens2 model with a matting head on top (a PixelShuffle-based decoder that predicts a
pre-multiplied RGB foreground and an alpha matte).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor). See `Sapiens2ImageProcessor.__call__()` for details (`processor_class` uses
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor) for processing images).
- **labels** (`torch.FloatTensor` of shape `(batch_size, 4, height, width)`, *optional*) --
  Ground-truth matting targets for computing the loss.`Sapiens2ImageMattingOutput` or `tuple(torch.FloatTensor)`A `Sapiens2ImageMattingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) and inputs.
The [Sapiens2ForImageMatting](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForImageMatting) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Loss.
- **alphas** (`torch.FloatTensor` of shape `(batch_size, 1, height, width)`) -- Estimated alpha values.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **foregrounds** (`torch.FloatTensor` of shape `(batch_size, 3, height, width)`) -- Pre-multiplied RGB foreground predictions in `[0, 1]` (sigmoid-activated).

Example:

```python
>>> from transformers import AutoImageProcessor, AutoModel
>>> from transformers.image_utils import load_image
>>> import torch

>>> image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-matting-1b")
>>> model = AutoModel.from_pretrained("facebook/sapiens2-matting-1b")

>>> inputs = image_processor(image, return_tensors="pt")
>>> with torch.inference_mode():
...     outputs = model(**inputs)

>>> outputs.alphas.shape
torch.Size([1, 1, 1024, 768])
>>> outputs.foregrounds.shape
torch.Size([1, 3, 1024, 768])
```

## Sapiens2ForNormalEstimation[[transformers.Sapiens2ForNormalEstimation]]

- **config** ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Sapiens2 model with a normal estimation head on top (a PixelShuffle-based decoder that predicts surface normal maps).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor). See `Sapiens2ImageProcessor.__call__()` for details (`processor_class` uses
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor) for processing images).
- **labels** (`torch.FloatTensor` of shape `(batch_size, num_labels, height, width)`, *optional*) --
  Ground-truth surface normal maps for computing the loss.`Sapiens2NormalEstimatorOutput` or `tuple(torch.FloatTensor)`A `Sapiens2NormalEstimatorOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) and inputs.
The [Sapiens2ForNormalEstimation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForNormalEstimation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Normal estimation loss.
- **normals** (`torch.FloatTensor` of shape `(batch_size, num_labels, height, width)`) -- Raw normal map predictions as output by the model (unnormalized).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage)
  of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states of the model at the output of
  each layer plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one per layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Attentions weights after the attention softmax.

Example:

```python
>>> from transformers import AutoImageProcessor, AutoModel
>>> from transformers.image_utils import load_image
>>> import torch

>>> image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-normal-0.4b")
>>> model = AutoModel.from_pretrained("facebook/sapiens2-normal-0.4b")

>>> inputs = image_processor(image, return_tensors="pt")
>>> with torch.inference_mode():
...     outputs = model(**inputs)

>>> outputs.normals.shape
torch.Size([1, 3, 1024, 768])
```

## Sapiens2ForPointmapEstimation[[transformers.Sapiens2ForPointmapEstimation]]

- **config** ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Sapiens2 model with a pointmap head on top (a PixelShuffle-based decoder that predicts per-pixel 3D XYZ
coordinates, plus an optional scale branch for focal-length normalization).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor). See `Sapiens2ImageProcessor.__call__()` for details (`processor_class` uses
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor) for processing images).
- **labels** (`torch.FloatTensor` of shape `(batch_size, 3, height, width)`, *optional*) --
  Ground-truth pointmap for computing the loss.`Sapiens2PointmapEstimatorOutput` or `tuple(torch.FloatTensor)`A `Sapiens2PointmapEstimatorOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) and inputs.
The [Sapiens2ForPointmapEstimation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForPointmapEstimation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Pointmap estimation loss.
- **pointmaps** (`torch.FloatTensor` of shape `(batch_size, 3, height, width)`) -- Per-pixel 3D XYZ coordinate predictions in canonical camera space.
- **scales** (`torch.FloatTensor` of shape `(batch_size, 1)`, *optional*) -- Canonical focal length / actual focal length ratio. `None` when no scale branch is configured.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage)
  of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states of the model at the output of
  each layer plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one per layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Attentions weights after the attention softmax.

Example:

```python
>>> from transformers import AutoImageProcessor, AutoModel
>>> from transformers.image_utils import load_image
>>> import torch

>>> image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-pointmap-0.4b")
>>> model = AutoModel.from_pretrained("facebook/sapiens2-pointmap-0.4b")

>>> inputs = image_processor(image, return_tensors="pt")
>>> with torch.inference_mode():
...     outputs = model(**inputs)

>>> outputs.pointmaps.shape
torch.Size([1, 3, 1024, 768])
```

## Sapiens2ForPoseEstimation[[transformers.Sapiens2ForPoseEstimation]]

- **config** ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Sapiens2 model with a pose estimation head on top (a set of heatmap predictors on top of the hidden states output).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor). See `Sapiens2ImageProcessor.__call__()` for details (`processor_class` uses
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor) for processing images).
- **flip_pairs** (`torch.Tensor` of shape `(num_pairs, 2)`, *optional*) --
  Pairs of keypoints which are mirrored (for example, left ear -- right ear), used for
  test-time flip augmentation. When provided, the model assumes `pixel_values` contains
  horizontally-flipped images and calls `flip_back` on the output heatmaps to restore the
  original orientation.
- **labels** (`torch.FloatTensor` of shape `(batch_size, num_keypoints, height, width)`, *optional*) --
  Heatmap ground truth for computing the loss.
- **label_weights** (`torch.FloatTensor` of shape `(batch_size, num_labels, 1, 1)` or `(batch_size, num_labels, height, width)`, *optional*) --
  Visibility weights for each keypoint. Must be broadcastable to the shape of `labels`.`Sapiens2PoseEstimatorOutput` or `tuple(torch.FloatTensor)`A `Sapiens2PoseEstimatorOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) and inputs.
The [Sapiens2ForPoseEstimation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForPoseEstimation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Pose estimation loss.
- **heatmaps** (`torch.FloatTensor` of shape `(batch_size, num_keypoints, height, width)`) -- Heatmaps as predicted by the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoImageProcessor, AutoModel
>>> from transformers.image_utils import load_image
>>> import torch

>>> image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-pose-0.4b")
>>> model = AutoModel.from_pretrained("facebook/sapiens2-pose-0.4b")

>>> boxes = [[[270.8, 0.6, 294.1, 379.5]]]
>>> inputs = image_processor(image, boxes=boxes, return_tensors="pt")
>>> with torch.inference_mode():
...     outputs = model(**inputs)

>>> outputs.heatmaps.shape
torch.Size([1, 308, 256, 192])
```

## Sapiens2ForSemanticSegmentation[[transformers.Sapiens2ForSemanticSegmentation]]

- **config** ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Sapiens2 Model with a semantic segmentation head on top e.g. for ADE20K, CityScapes.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor). See `Sapiens2ImageProcessor.__call__()` for details (`processor_class` uses
  [Sapiens2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Ground truth semantic segmentation maps for computing the loss.
  Indices should be in `[0, ..., config.num_labels - 1]`.
  If `config.num_labels > 1`, a classification loss is computed (Cross-Entropy).[SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or `tuple(torch.FloatTensor)`A [SemanticSegmenterOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.SemanticSegmenterOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sapiens2Config](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2Config)) and inputs.
The [Sapiens2ForSemanticSegmentation](/docs/transformers/v5.14.0/en/model_doc/sapiens2#transformers.Sapiens2ForSemanticSegmentation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels, logits_height, logits_width)`) -- Classification scores for each pixel.

  

  The logits returned do not necessarily have the same size as the `pixel_values` passed as inputs. This is
  to avoid doing two interpolations and lose some quality when a user needs to resize the logits to the
  original image size as post-processing. You should always check your logits shape and resize as needed.

  
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, patch_size, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoImageProcessor, AutoModel
>>> from transformers.image_utils import load_image
>>> import torch

>>> image = load_image("http://images.cocodataset.org/val2017/000000004016.jpg")
>>> image_processor = AutoImageProcessor.from_pretrained("facebook/sapiens2-seg-0.4b")
>>> model = AutoModel.from_pretrained("facebook/sapiens2-seg-0.4b")

>>> inputs = image_processor(image, return_tensors="pt")
>>> with torch.inference_mode():
...     outputs = model(**inputs)

>>> outputs.logits.shape
torch.Size([1, 29, 1024, 768])
```
