# EdgeTAMVideo

## Overview

The EdgeTAM model was proposed in [EdgeTAM: On-Device Track Anything Model](https://huggingface.co/papers/2501.07256) Chong Zhou, Chenchen Zhu, Yunyang Xiong, Saksham Suri, Fanyi Xiao, Lemeng Wu, Raghuraman Krishnamoorthi, Bo Dai, Chen Change Loy, Vikas Chandra, Bilge Soran.

EdgeTAM is an efficient adaptation of SAM 2 that introduces a 2D Spatial Perceiver architecture to optimize memory attention mechanisms for real-time video segmentation on mobile devices.

The abstract from the paper is the following:

*On top of Segment Anything Model (SAM), SAM 2 further extends its capability from image to video inputs through a memory bank mechanism and obtains a remarkable performance compared with previous methods, making it a foundation model for video segmentation task. In this paper, we aim at making SAM 2 much more efficient so that it even runs on mobile devices while maintaining a comparable performance. Despite several works optimizing SAM for better efficiency, we find they are not sufficient for SAM 2 because they all focus on compressing the image encoder, while our benchmark shows that the newly introduced memory attention blocks are also the latency bottleneck. Given this observation, we propose EdgeTAM, which leverages a novel 2D Spatial Perceiver to reduce the computational cost. In particular, the proposed 2D Spatial Perceiver encodes the densely stored frame-level memories with a lightweight Transformer that contains a fixed set of learnable queries. Given that video segmentation is a dense prediction task, we find preserving the spatial structure of the memories is essential so that the queries are split into global-level and patch-level groups. We also propose a distillation pipeline that further improves the performance without inference overhead. As a result, EdgeTAM achieves 87.7, 70.0, 72.3, and 71.7 J&F on DAVIS 2017, MOSE, SA-V val, and SA-V test, while running at 16 FPS on iPhone 15 Pro Max.*

This model was contributed by [yonigozlan](https://huggingface.co/yonigozlan).
The original code can be found [here](https://github.com/facebookresearch/EdgeTAM).

## Usage example

### Video Segmentation and Tracking

EdgeTAM Video's key strength is its ability to track objects across video frames efficiently on mobile devices. Here's how to use it for video segmentation:

#### Basic Video Tracking

```python
from transformers import EdgeTamVideoModel, Sam2VideoProcessor
import torch

model = EdgeTamVideoModel.from_pretrained("yonigozlan/edgetam-video-1", device_map="auto")
processor = Sam2VideoProcessor.from_pretrained("yonigozlan/edgetam-video-1")

# Load video frames (example assumes you have a list of PIL Images)
# video_frames = [Image.open(f"frame_{i:05d}.jpg") for i in range(num_frames)]

# For this example, we'll use the video loading utility
from transformers.video_utils import load_video
video_url = "https://huggingface.co/datasets/hf-internal-testing/sam2-fixtures/resolve/main/bedroom.mp4"
video_frames, _ = load_video(video_url)

# Initialize video inference session
inference_session = processor.init_video_session(
    video=video_frames,
    inference_device=device,
)

# Add click on first frame to select object
ann_frame_idx = 0
ann_obj_id = 1
points = [[[[210, 350]]]]
labels = [[[1]]]

processor.add_inputs_to_inference_session(
    inference_session=inference_session,
    frame_idx=ann_frame_idx,
    obj_ids=ann_obj_id,
    input_points=points,
    input_labels=labels,
)

# Segment the object on the first frame
outputs = model(
    inference_session=inference_session,
    frame_idx=ann_frame_idx,
)
video_res_masks = processor.post_process_masks(
    [outputs.pred_masks], original_sizes=[[inference_session.video_height, inference_session.video_width]], binarize=False
)[0]
print(f"Segmentation shape: {video_res_masks.shape}")
Segmentation shape: torch.Size([1, 1, 540, 960])

# Propagate through the entire video
video_segments = {}
for sam2_video_output in model.propagate_in_video_iterator(inference_session):
    video_res_masks = processor.post_process_masks(
        [sam2_video_output.pred_masks], original_sizes=[[inference_session.video_height, inference_session.video_width]], binarize=False
    )[0]
    video_segments[sam2_video_output.frame_idx] = video_res_masks

print(f"Tracked object through {len(video_segments)} frames")
Tracked object through 200 frames
```

#### Multi-Object Video Tracking

Track multiple objects simultaneously across video frames:

```python
# Reset for new tracking session
inference_session.reset_inference_session()

# Add multiple objects on the first frame
ann_frame_idx = 0
obj_ids = [2, 3]
input_points = [[[[200, 300]], [[400, 150]]]]  # Points for two objects (batched)
input_labels = [[[1], [1]]]

processor.add_inputs_to_inference_session(
    inference_session=inference_session,
    frame_idx=ann_frame_idx,
    obj_ids=obj_ids,
    input_points=input_points,
    input_labels=input_labels,
)

# Get masks for both objects on first frame
outputs = model(
    inference_session=inference_session,
    frame_idx=ann_frame_idx,
)

# Propagate both objects through video
video_segments = {}
for sam2_video_output in model.propagate_in_video_iterator(inference_session):
    video_res_masks = processor.post_process_masks(
        [sam2_video_output.pred_masks], original_sizes=[[inference_session.video_height, inference_session.video_width]], binarize=False
    )[0]
    video_segments[sam2_video_output.frame_idx] = {
        obj_id: video_res_masks[i]
        for i, obj_id in enumerate(inference_session.obj_ids)
    }

print(f"Tracked {len(inference_session.obj_ids)} objects through {len(video_segments)} frames")
Tracked 2 objects through 200 frames
```

#### Refining Video Segmentation

You can add additional clicks on any frame to refine the tracking:

```python
# Add refinement click on a later frame
refine_frame_idx = 50
ann_obj_id = 2  # Refining first object
points = [[[[220, 280]]]]  # Additional point
labels = [[[1]]]  # Positive click

processor.add_inputs_to_inference_session(
    inference_session=inference_session,
    frame_idx=refine_frame_idx,
    obj_ids=ann_obj_id,
    input_points=points,
    input_labels=labels,
)

# Re-propagate with the additional information
video_segments = {}
for sam2_video_output in model.propagate_in_video_iterator(inference_session):
    video_res_masks = processor.post_process_masks(
        [sam2_video_output.pred_masks], original_sizes=[[inference_session.video_height, inference_session.video_width]], binarize=False
    )[0]
    video_segments[sam2_video_output.frame_idx] = video_res_masks
```

### Streaming Video Inference

For real-time applications, EdgeTAM Video supports processing video frames as they arrive:

```python
# Initialize session for streaming
inference_session = processor.init_video_session(
    inference_device=device,
)

# Process frames one by one
for frame_idx, frame in enumerate(video_frames[:10]):  # Process first 10 frames
    inputs = processor(images=frame, device=device, return_tensors="pt").to(model.device)
...
    if frame_idx == 0:
        # Add point input on first frame
        processor.add_inputs_to_inference_session(
            inference_session=inference_session,
            frame_idx=0,
            obj_ids=1,
            input_points=[[[[210, 350], [250, 220]]]],
            input_labels=[[[1, 1]]],
            original_size=inputs.original_sizes[0], # need to be provided when using streaming video inference
        )
...
    # Process current frame
    sam2_video_output = model(inference_session=inference_session, frame=inputs.pixel_values[0])
...
    video_res_masks = processor.post_process_masks(
        [sam2_video_output.pred_masks], original_sizes=inputs.original_sizes, binarize=False
    )[0]
    print(f"Frame {frame_idx}: mask shape {video_res_masks.shape}")

Frame 0: mask shape torch.Size([1, 1, 540, 960])
...
```

#### Video Batch Processing for Multiple Objects

Track multiple objects simultaneously in video by adding them all at once:

```python
# Initialize video session
inference_session = processor.init_video_session(
    video=video_frames,
    inference_device=device,
)

# Add multiple objects on the first frame using batch processing
ann_frame_idx = 0
obj_ids = [2, 3]  # Track two different objects
input_points = [
    [[[200, 300], [230, 250], [275, 175]], [[400, 150]]]
]  # Object 2: 3 points (2 positive, 1 negative); Object 3: 1 point
input_labels = [
    [[1, 1, 0], [1]]
]  # Object 2: positive, positive, negative; Object 3: positive

processor.add_inputs_to_inference_session(
    inference_session=inference_session,
    frame_idx=ann_frame_idx,
    obj_ids=obj_ids,
    input_points=input_points,
    input_labels=input_labels,
)

# Get masks for all objects on the first frame
outputs = model(
    inference_session=inference_session,
    frame_idx=ann_frame_idx,
)
video_res_masks = processor.post_process_masks(
    [outputs.pred_masks], original_sizes=[[inference_session.video_height, inference_session.video_width]], binarize=False
)[0]
print(f"Generated masks for {video_res_masks.shape[0]} objects")
Generated masks for 2 objects

# Propagate all objects through the video
video_segments = {}
for sam2_video_output in model.propagate_in_video_iterator(inference_session):
    video_res_masks = processor.post_process_masks(
        [sam2_video_output.pred_masks], original_sizes=[[inference_session.video_height, inference_session.video_width]], binarize=False
    )[0]
    video_segments[sam2_video_output.frame_idx] = {
        obj_id: video_res_masks[i]
        for i, obj_id in enumerate(inference_session.obj_ids)
    }

print(f"Tracked {len(inference_session.obj_ids)} objects through {len(video_segments)} frames")
Tracked 2 objects through 200 frames
```

## EdgeTamVideoMaskDecoderConfig[[transformers.EdgeTamVideoMaskDecoderConfig]]

#### transformers.EdgeTamVideoMaskDecoderConfig[[transformers.EdgeTamVideoMaskDecoderConfig]]

```python
transformers.EdgeTamVideoMaskDecoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 256, hidden_act: str = 'gelu', mlp_dim: int = 2048, num_hidden_layers: int = 2, num_attention_heads: int = 8, attention_downsample_rate: int = 2, num_multimask_outputs: int = 3, iou_head_depth: int = 3, iou_head_hidden_dim: int = 256, dynamic_multimask_via_stability: bool = True, dynamic_multimask_stability_delta: float = 0.05, dynamic_multimask_stability_thresh: float = 0.98)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/configuration_edgetam_video.py#L54)

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

This is the configuration class to store the configuration of a EdgeTamVideoModel. It is used to instantiate a Edgetam Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/EdgeTAM-hf](https://huggingface.co/yonigozlan/EdgeTAM-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## EdgeTamVideoPromptEncoderConfig[[transformers.EdgeTamVideoPromptEncoderConfig]]

#### transformers.EdgeTamVideoPromptEncoderConfig[[transformers.EdgeTamVideoPromptEncoderConfig]]

```python
transformers.EdgeTamVideoPromptEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 256, image_size: int | list[int] | tuple[int, int] = 1024, patch_size: int | list[int] | tuple[int, int] = 16, mask_input_channels: int = 16, num_point_embeddings: int = 4, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06, scale: int = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/configuration_edgetam_video.py#L30)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1024`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

mask_input_channels (`int`, *optional*, defaults to 16) : The number of channels to be fed to the `MaskDecoder` module.

num_point_embeddings (`int`, *optional*, defaults to 4) : The number of point embeddings to be used.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

scale (`float`, *optional*, defaults to 1) : The scale factor for the prompt encoder.

This is the configuration class to store the configuration of a EdgeTamVideoModel. It is used to instantiate a Edgetam Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/EdgeTAM-hf](https://huggingface.co/yonigozlan/EdgeTAM-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## EdgeTamVideoConfig[[transformers.EdgeTamVideoConfig]]

#### transformers.EdgeTamVideoConfig[[transformers.EdgeTamVideoConfig]]

```python
transformers.EdgeTamVideoConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, prompt_encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, mask_decoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, initializer_range: float = 0.02, num_maskmem: int = 7, image_size: int | list[int] | tuple[int, int] = 1024, sigmoid_scale_for_mem_enc: float = 20.0, sigmoid_bias_for_mem_enc: float = -10.0, enable_occlusion_spatial_embedding: bool = True, multimask_output_in_sam: bool = True, multimask_min_pt_num: int = 0, multimask_max_pt_num: int = 1, multimask_output_for_tracking: bool = True, max_object_pointers_in_encoder: int = 16, max_cond_frame_num: int = -1, enable_temporal_pos_encoding_for_object_pointers: bool = True, memory_attention_hidden_size: int = 256, memory_attention_num_layers: int = 2, memory_attention_num_attention_heads: int = 1, memory_attention_downsample_rate: int = 1, memory_attention_mlp_hidden_size: int = 2048, memory_attention_mlp_hidden_act: str = 'relu', memory_attention_dropout: float | int = 0.1, memory_attention_rope_theta: float | int = 10000, memory_attention_rope_feat_sizes: list | None = None, memory_attention_rope_k_sizes: list | None = None, memory_attention_rope_dropout: float | int = 0.1, perceiver_resampler_num_latents: int = 256, perceiver_resampler_num_latents_2d: int = 256, perceiver_resampler_hidden_size: int = 64, perceiver_resampler_mlp_intermediate_size: int = 256, perceiver_resampler_num_attention_heads: int = 1, perceiver_resampler_attention_head_dim: int = 64, perceiver_resampler_num_layers: int = 2, perceiver_resampler_hidden_dropout: float | int = 0.0, perceiver_resampler_attention_dropout: float | int = 0.0, memory_encoder_hidden_size: int = 256, memory_encoder_output_channels: int = 64, mask_downsampler_embed_dim: int = 256, memory_fuser_intermediate_dim: int = 1024, mask_downsampler_kernel_size: int = 3, mask_downsampler_stride: int = 2, mask_downsampler_padding: int = 1, mask_downsampler_total_stride: int = 16, mask_downsampler_hidden_act: str = 'gelu', memory_fuser_num_layers: int = 2, memory_fuser_embed_dim: int = 256, memory_fuser_kernel_size: int = 7, memory_fuser_padding: int = 3, memory_fuser_layer_scale_init_value: float = 1e-06, memory_fuser_hidden_act: str = 'gelu')
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/configuration_edgetam_video.py#L92)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

prompt_encoder_config (Union[`dict`, `EdgeTamVideoPromptEncoderConfig`], *optional*) : Dictionary of configuration options used to initialize [EdgeTamVideoPromptEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/edgetam_video#transformers.EdgeTamVideoPromptEncoderConfig).

mask_decoder_config (Union[`dict`, `EdgeTamVideoMaskDecoderConfig`], *optional*) : Dictionary of configuration options used to initialize [EdgeTamMaskDecoderConfig](/docs/transformers/v5.15.1/en/model_doc/edgetam#transformers.EdgeTamMaskDecoderConfig).

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

num_maskmem (`int`, *optional*, defaults to 7) : The number of memory slots for the mask memory.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1024`) : The size (resolution) of each image.

sigmoid_scale_for_mem_enc (`float`, *optional*, defaults to 20.0) : Scale factor for the sigmoid function in the memory encoder.

sigmoid_bias_for_mem_enc (`float`, *optional*, defaults to -10.0) : Bias for the sigmoid function in the memory encoder.

enable_occlusion_spatial_embedding (`bool`, *optional*, defaults to `True`) : Whether to enable spatial embedding for occlusions.

multimask_output_in_sam (`bool`, *optional*, defaults to `True`) : Whether to output multiple masks from the SAM head.

multimask_min_pt_num (`int`, *optional*, defaults to 0) : The minimum number of points to trigger multimask output.

multimask_max_pt_num (`int`, *optional*, defaults to 1) : The maximum number of points to trigger multimask output.

multimask_output_for_tracking (`bool`, *optional*, defaults to `True`) : Whether to use multimask output for tracking.

max_object_pointers_in_encoder (`int`, *optional*, defaults to 16) : The maximum number of object pointers in the encoder.

max_cond_frame_num (`int`, *optional*, defaults to -1) : Maximum number of conditioning frames to use in memory attention. Set to -1 to use all conditioning frames.

enable_temporal_pos_encoding_for_object_pointers (`bool`, *optional*, defaults to `True`) : Whether to enable temporal positional encoding for object pointers.

memory_attention_hidden_size (`int`, *optional*, defaults to 256) : Dimensionality of the memory attention hidden states.

memory_attention_num_layers (`int`, *optional*, defaults to 2) : The number of layers in the memory attention module.

memory_attention_num_attention_heads (`int`, *optional*, defaults to 1) : Number of attention heads for each attention layer in the memory attention.

memory_attention_downsample_rate (`int`, *optional*, defaults to 1) : The downsample rate for the attention layers.

memory_attention_mlp_hidden_size (`int`, *optional*, defaults to 2048) : The dimension of the feedforward network in the memory attention module.

memory_attention_mlp_hidden_act (`str`, *optional*, defaults to `"relu"`) : The non-linear activation function in the feedforward network in the memory attention module.

memory_attention_dropout (`float`, *optional*, defaults to 0.1) : The dropout rate for the memory attention module.

memory_attention_rope_theta (`float`, *optional*, defaults to 10000) : The Rope theta parameter.

memory_attention_rope_feat_sizes (`Tuple[int, int]`, *optional*, defaults to `[64, 64]`) : The feature sizes for the Rope positional encoding.

memory_attention_rope_k_sizes (`List[int]`, *optional*, defaults to `[16, 16]`) : The key feature sizes for the RoPE positional encoding in memory attention.

memory_attention_rope_dropout (`float`, *optional*, defaults to 0.1) : The dropout rate for the Rope positional encoding.

perceiver_resampler_num_latents (`int`, *optional*, defaults to 256) : The number of 1D latent tokens in the perceiver resampler.

perceiver_resampler_num_latents_2d (`int`, *optional*, defaults to 256) : The number of 2D latent tokens in the perceiver resampler.

perceiver_resampler_hidden_size (`int`, *optional*, defaults to 64) : The hidden size of the perceiver resampler.

perceiver_resampler_mlp_intermediate_size (`int`, *optional*, defaults to 256) : The intermediate size of the feedforward network in the perceiver resampler.

perceiver_resampler_num_attention_heads (`int`, *optional*, defaults to 1) : The number of attention heads in the perceiver resampler.

perceiver_resampler_attention_head_dim (`int`, *optional*, defaults to 64) : The dimension of each attention head in the perceiver resampler.

perceiver_resampler_num_layers (`int`, *optional*, defaults to 2) : The number of layers in the perceiver resampler.

perceiver_resampler_hidden_dropout (`float`, *optional*, defaults to 0.0) : The dropout rate for the hidden layers in the perceiver resampler.

perceiver_resampler_attention_dropout (`float`, *optional*, defaults to 0.0) : The dropout rate for the attention layers in the perceiver resampler.

memory_encoder_hidden_size (`int`, *optional*, defaults to 256) : Dimensionality of the memory encoder hidden states.

memory_encoder_output_channels (`int`, *optional*, defaults to 64) : The number of output channels for the memory encoder.

mask_downsampler_embed_dim (`int`, *optional*, defaults to 256) : The dimension of the mask downsampler embedding.

memory_fuser_intermediate_dim (`int`, *optional*, defaults to 1024) : The intermediate dimension of the memory fuser feedforward network.

mask_downsampler_kernel_size (`int`, *optional*, defaults to 3) : The kernel size for the mask downsampler.

mask_downsampler_stride (`int`, *optional*, defaults to 2) : The stride for the mask downsampler.

mask_downsampler_padding (`int`, *optional*, defaults to 1) : The padding for the mask downsampler.

mask_downsampler_total_stride (`int`, *optional*, defaults to 16) : The total stride for the mask downsampler.

mask_downsampler_hidden_act (`str`, *optional*, defaults to `"gelu"`) : The non-linear activation function in the mask downsampler.

memory_fuser_num_layers (`int`, *optional*, defaults to 2) : The number of layers in the memory fuser.

memory_fuser_embed_dim (`int`, *optional*, defaults to 256) : The dimension of the memory fuser embedding.

memory_fuser_kernel_size (`int`, *optional*, defaults to 7) : The kernel size for the memory fuser.

memory_fuser_padding (`int`, *optional*, defaults to 3) : The padding for the memory fuser.

memory_fuser_layer_scale_init_value (`float`, *optional*, defaults to 1e-06) : The initial value for the layer scale in the memory fuser.

memory_fuser_hidden_act (`str`, *optional*, defaults to `"gelu"`) : The non-linear activation function in the memory fuser.

This is the configuration class to store the configuration of a EdgeTamVideoModel. It is used to instantiate a Edgetam Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/EdgeTAM-hf](https://huggingface.co/yonigozlan/EdgeTAM-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     EdgeTamVisionConfig,
...     EdgeTamVideoPromptEncoderConfig,
...     EdgeTamVideoMaskDecoderConfig,
...     EdgeTamVideoModel,
...     EdgeTamVideoConfig,
... )

>>> # Initializing a EdgeTamVideoConfig with `"facebook/edgetam.1_hiera_tiny"` style configuration
>>> configuration = EdgeTamVideoConfig()

>>> # Initializing a EdgeTamVideoModel (with random weights) from the `"facebook/edgetam.1_hiera_tiny"` style configuration
>>> model = EdgeTamVideoModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a EdgeTamConfig from a EdgeTamVisionConfig, EdgeTamPromptEncoderConfig, and EdgeTamMaskDecoderConfig

>>> # Initializing EDGETAM vision encoder, memory attention, and memory encoder configurations
>>> vision_config = EdgeTamVisionConfig()
>>> prompt_encoder_config = EdgeTamVideoPromptEncoderConfig()
>>> mask_decoder_config = EdgeTamVideoMaskDecoderConfig()

>>> config = EdgeTamVideoConfig(vision_config, prompt_encoder_config, mask_decoder_config)
```

## EdgeTamVideoInferenceSession[[transformers.EdgeTamVideoInferenceSession]]

#### transformers.EdgeTamVideoInferenceSession[[transformers.EdgeTamVideoInferenceSession]]

```python
transformers.EdgeTamVideoInferenceSession(video: typing.Optional[torch.FloatTensor] = None, video_height: int | None = None, video_width: int | None = None, inference_device: typing.Union[torch.device, str] = 'cpu', inference_state_device: typing.Union[torch.device, str] = 'cpu', video_storage_device: typing.Union[torch.device, str] = 'cpu', dtype: typing.Union[torch.dtype, str] = 'float32', max_vision_features_cache_size: int = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L924)

**Parameters:**

video (`torch.FloatTensor`, *optional*) : The video to process. No need to provide when streaming.

video_height (`int`, *optional*) : The height of the video.

video_width (`int`, *optional*) : The width of the video.

inference_device (`torch.device`, *optional*, defaults to `"cpu"`) : The device to use for inference.

inference_state_device (`torch.device`, *optional*, defaults to `"cpu"`) : The device to store the inference state on.

video_storage_device (`torch.device`, *optional*, defaults to `"cpu"`) : The device to store the video on.

dtype (`torch.dtype`, *optional*, defaults to `"float32"`) : The dtype to use for the video.

max_vision_features_cache_size (`int`, *optional*, defaults to 1) : The maximum number of vision features to cache.

Manages video inference session parameters, state and cache.

#### add_mask_inputs[[transformers.EdgeTamVideoInferenceSession.add_mask_inputs]]

```python
add_mask_inputs(obj_idx: int, frame_idx: int, inputs: Tensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1044)

Add mask inputs with automatic device placement.

#### add_new_frame[[transformers.EdgeTamVideoInferenceSession.add_new_frame]]

```python
add_new_frame(pixel_values: Tensor, frame_idx: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1119)

Add new frame with automatic device placement.

#### add_point_inputs[[transformers.EdgeTamVideoInferenceSession.add_point_inputs]]

```python
add_point_inputs(obj_idx: int, frame_idx: int, inputs: dict)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1030)

Add point inputs with automatic device placement.

#### get_frame[[transformers.EdgeTamVideoInferenceSession.get_frame]]

```python
get_frame(frame_idx: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1135)

Get frame from video.

#### get_obj_num[[transformers.EdgeTamVideoInferenceSession.get_obj_num]]

```python
get_obj_num()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1025)

Get the total number of unique object ids received so far in this session.

#### get_output[[transformers.EdgeTamVideoInferenceSession.get_output]]

```python
get_output(obj_idx: int, frame_idx: int, output_key: str, is_conditioning_frame: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1092)

**Parameters:**

obj_idx (int) : The index of the object.

frame_idx (int) : The index of the frame.

output_key (str) : The key of the output.

is_conditioning_frame (bool) : Whether the output is for a conditioning frame.

Get output with smart device management.

#### obj_id_to_idx[[transformers.EdgeTamVideoInferenceSession.obj_id_to_idx]]

```python
obj_id_to_idx(obj_id: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L999)

Map object ID to index, creating new entry if needed.

#### obj_idx_to_id[[transformers.EdgeTamVideoInferenceSession.obj_idx_to_id]]

```python
obj_idx_to_id(obj_idx: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1021)

Map model-side object index to client-side object id.

#### remove_mask_inputs[[transformers.EdgeTamVideoInferenceSession.remove_mask_inputs]]

```python
remove_mask_inputs(obj_idx: int, frame_idx: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1050)

Remove mask inputs.

#### remove_point_inputs[[transformers.EdgeTamVideoInferenceSession.remove_point_inputs]]

```python
remove_point_inputs(obj_idx: int, frame_idx: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1040)

Remove point inputs.

#### reset_inference_session[[transformers.EdgeTamVideoInferenceSession.reset_inference_session]]

```python
reset_inference_session()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1151)

Reset tracking data and cache.

#### reset_tracking_data[[transformers.EdgeTamVideoInferenceSession.reset_tracking_data]]

```python
reset_tracking_data()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1139)

Reset tracking data but keep cache.

#### store_output[[transformers.EdgeTamVideoInferenceSession.store_output]]

```python
store_output(obj_idx: int, frame_idx: int, output_key: str | None = None, output_value: typing.Union[torch.Tensor, dict, NoneType] = None, is_conditioning_frame: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L1055)

**Parameters:**

obj_idx (int) : The index of the object.

frame_idx (int) : The index of the frame.

output_key (Optional[str]) : The key of the output. If None, the output is stored as a dictionary.

output_value (Optional[Union[torch.Tensor, dict]]) : The value of the output.

is_conditioning_frame (bool) : Whether the output is for a conditioning frame.

Store output with smart device management.
If output_key is None, the output is stored as a dictionary.

## EdgeTamVideoModel[[transformers.EdgeTamVideoModel]]

#### transformers.EdgeTamVideoModel[[transformers.EdgeTamVideoModel]]

```python
transformers.EdgeTamVideoModel(config: EdgeTamVideoConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L2022)

**Parameters:**

config ([EdgeTamVideoConfig](/docs/transformers/v5.15.1/en/model_doc/edgetam_video#transformers.EdgeTamVideoConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Edgetam Video Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.EdgeTamVideoModel.forward]]

```python
forward(inference_session: EdgeTamVideoInferenceSession, frame_idx: int | None = None, frame: typing.Optional[torch.Tensor] = None, reverse: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L2156)

**Parameters:**

inference_session (`~models.edgetam_video.modeling_edgetam_video.EdgeTamVideoInferenceSession`) : The video inference session object.

frame_idx (`int`, *optional*) : The index of the frame on which to run inference. No need to provide when inferring on a new streamed frame.

frame (`torch.Tensor`, *optional*) : The frame to process. Provide when streaming.

reverse (`bool`, *optional*, defaults to `False`) : Whether to propagate in reverse.

**Returns:** `EdgeTamVideoSegmentationOutput` or `tuple(torch.FloatTensor)`

A `EdgeTamVideoSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EdgeTamVideoConfig](/docs/transformers/v5.15.1/en/model_doc/edgetam_video#transformers.EdgeTamVideoConfig)) and inputs.

Propagate the objects through a streamed video frame.

- **object_ids** (`list[int]`, *optional*) -- List of object IDs being tracked in the current frame.
- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, num_masks, height, width)`) -- The predicted masks stored at the model's resolution.
- **object_score_logits** (`torch.FloatTensor` of shape `(batch_size,)`, *optional*) -- Logits for the object scores, indicating if objects are present.
- **frame_idx** (`int`, *optional*) -- The frame index of the video.

#### get_image_features[[transformers.EdgeTamVideoModel.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/edgetam_video/modeling_edgetam_video.py#L2255)

**Parameters:**

pixel_values (`torch.FloatTensor`) : Input pixel values of shape `(batch_size, num_channels, height, width)`.

**Returns:** `EdgeTamVideoVisionEncoderOutput` or `tuple(torch.FloatTensor)`

A `EdgeTamVideoVisionEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EdgeTamVideoConfig](/docs/transformers/v5.15.1/en/model_doc/edgetam_video#transformers.EdgeTamVideoConfig)) and inputs.

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
