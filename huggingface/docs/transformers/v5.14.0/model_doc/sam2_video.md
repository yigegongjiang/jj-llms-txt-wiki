# SAM2 Video

    
        
        
    

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

### Video Segmentation and Tracking

SAM2's key strength is its ability to track objects across video frames. Here's how to use it for video segmentation:

#### Basic Video Tracking

```python
from transformers import Sam2VideoModel, Sam2VideoProcessor
import torch

model = Sam2VideoModel.from_pretrained("facebook/sam2.1-hiera-tiny", device_map="auto")
processor = Sam2VideoProcessor.from_pretrained("facebook/sam2.1-hiera-tiny")

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
Segmentation shape: torch.Size([1, 1, 480, 854])

# Propagate through the entire video
video_segments = {}
for sam2_video_output in model.propagate_in_video_iterator(inference_session):
    video_res_masks = processor.post_process_masks(
        [sam2_video_output.pred_masks], original_sizes=[[inference_session.video_height, inference_session.video_width]], binarize=False
    )[0]
    video_segments[sam2_video_output.frame_idx] = video_res_masks

print(f"Tracked object through {len(video_segments)} frames")
Tracked object through 180 frames
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
Tracked 2 objects through 180 frames
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

For real-time applications, SAM2 supports processing video frames as they arrive:

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
Tracked 2 objects through 180 frames
```

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with SAM.

- [Demo notebook 🌎](https://colab.research.google.com/drive/1Z0NGLE7p8qnc9UpuI8KBETHd2xBbOEhv?usp=sharing) for using the model, contributed by [Sangbum Choi](https://github.com/SangbumChoi).

## Sam2VideoConfig[[transformers.Sam2VideoConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **prompt_encoder_config** (Union[`dict`, `Sam2PromptEncoderConfig`], *optional*) --
  Dictionary of configuration options used to initialize [Sam2PromptEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/sam2#transformers.Sam2PromptEncoderConfig).
- **mask_decoder_config** (Union[`dict`, `Sam2MaskDecoderConfig`], *optional*) --
  Dictionary of configuration options used to initialize [Sam2MaskDecoderConfig](/docs/transformers/v5.14.0/en/model_doc/sam2#transformers.Sam2MaskDecoderConfig).
- **initializer_range** (`float`, *optional*, defaults to 0.02) --
  Standard deviation for parameter initialization.
- **num_maskmem** (`int`, *optional*, defaults to 7) --
  The number of memory slots for the mask memory.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1024`) --
  The size (resolution) of each image.
- **sigmoid_scale_for_mem_enc** (`float`, *optional*, defaults to 20.0) --
  Scale factor for the sigmoid function in the memory encoder.
- **sigmoid_bias_for_mem_enc** (`float`, *optional*, defaults to -10.0) --
  Bias for the sigmoid function in the memory encoder.
- **enable_occlusion_spatial_embedding** (`bool`, *optional*, defaults to `True`) --
  Whether to enable spatial embedding for occlusions.
- **multimask_output_in_sam** (`bool`, *optional*, defaults to `True`) --
  Whether to output multiple masks from the SAM head.
- **multimask_min_pt_num** (`int`, *optional*, defaults to 0) --
  The minimum number of points to trigger multimask output.
- **multimask_max_pt_num** (`int`, *optional*, defaults to 1) --
  The maximum number of points to trigger multimask output.
- **multimask_output_for_tracking** (`bool`, *optional*, defaults to `True`) --
  Whether to use multimask output for tracking.
- **max_object_pointers_in_encoder** (`int`, *optional*, defaults to 16) --
  The maximum number of object pointers in the encoder.
- **max_cond_frame_num** (`int`, *optional*, defaults to -1) --
  Maximum number of conditioning frames to use in memory attention. Set to -1 to use all conditioning frames.
- **enable_temporal_pos_encoding_for_object_pointers** (`bool`, *optional*, defaults to `True`) --
  Whether to enable temporal positional encoding for object pointers.
- **memory_attention_hidden_size** (`int`, *optional*, defaults to 256) --
  Dimensionality of the memory attention hidden states.
- **memory_attention_num_layers** (`int`, *optional*, defaults to 4) --
  The number of layers in the memory attention module.
- **memory_attention_num_attention_heads** (`int`, *optional*, defaults to 1) --
  Number of attention heads for each attention layer in the memory attention.
- **memory_attention_downsample_rate** (`int`, *optional*, defaults to 1) --
  The downsample rate for the attention layers.
- **memory_attention_feed_forward_hidden_size** (`int`, *optional*, defaults to 2048) --
  The dimension of the feedforward network in the memory attention module.
- **memory_attention_feed_forward_hidden_act** (`str`, *optional*, defaults to `"relu"`) --
  The non-linear activation function in the feedforward network in the memory attention module.
- **memory_attention_dropout** (`float`, *optional*, defaults to 0.1) --
  The dropout rate for the memory attention module.
- **memory_attention_rope_theta** (`float`, *optional*, defaults to 10000) --
  The Rope theta parameter.
- **memory_attention_rope_feat_sizes** (`list[int]`, *optional*, defaults to `[64, 64]`) --
  The feature sizes for the Rope positional encoding.
- **memory_attention_rope_dropout** (`float`, *optional*, defaults to 0.1) --
  The dropout rate for the Rope positional encoding.
- **memory_encoder_hidden_size** (`int`, *optional*, defaults to 256) --
  Dimensionality of the memory encoder hidden states.
- **memory_encoder_output_channels** (`int`, *optional*, defaults to 64) --
  The number of output channels for the memory encoder.
- **mask_downsampler_embed_dim** (`int`, *optional*, defaults to 256) --
  The dimension of the mask downsampler embedding.
- **mask_downsampler_kernel_size** (`int`, *optional*, defaults to 3) --
  The kernel size for the mask downsampler.
- **mask_downsampler_stride** (`int`, *optional*, defaults to 2) --
  The stride for the mask downsampler.
- **mask_downsampler_padding** (`int`, *optional*, defaults to 1) --
  The padding for the mask downsampler.
- **mask_downsampler_total_stride** (`int`, *optional*, defaults to 16) --
  The total stride for the mask downsampler.
- **mask_downsampler_hidden_act** (`str`, *optional*, defaults to `"gelu"`) --
  The non-linear activation function in the mask downsampler.
- **memory_fuser_num_layers** (`int`, *optional*, defaults to 2) --
  The number of layers in the memory fuser.
- **memory_fuser_embed_dim** (`int`, *optional*, defaults to 256) --
  The dimension of the embedding layer in the memory fuser.
- **memory_fuser_intermediate_dim** (`int`, *optional*, defaults to 1024) --
  The dimension of the intermediate layer in the memory fuser.
- **memory_fuser_kernel_size** (`int`, *optional*, defaults to 7) --
  The kernel size for the memory fuser.
- **memory_fuser_padding** (`int`, *optional*, defaults to 3) --
  The padding for the memory fuser.
- **memory_fuser_layer_scale_init_value** (`float`, *optional*, defaults to 1e-06) --
  The initial value for the layer scale in the memory fuser.
- **memory_fuser_hidden_act** (`str`, *optional*, defaults to `"gelu"`) --
  The non-linear activation function in the memory fuser..

This is the configuration class to store the configuration of a Sam2VideoModel. It is used to instantiate a Sam2 Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam2.1-hiera-tiny](https://huggingface.co/facebook/sam2.1-hiera-tiny)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     Sam2VisionConfig,
...     Sam2PromptEncoderConfig,
...     Sam2MaskDecoderConfig,
...     Sam2Model,
... )

>>> # Initializing a Sam2Config with `"facebook/sam2.1_hiera_tiny"` style configuration
>>> configuration = Sam2config()

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

## Sam2VideoMaskDecoderConfig[[transformers.Sam2VideoMaskDecoderConfig]]

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

This is the configuration class to store the configuration of a Sam2VideoModel. It is used to instantiate a Sam2 Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam2_video.1-hiera-tiny](https://huggingface.co/facebook/sam2_video.1-hiera-tiny)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam2VideoPromptEncoderConfig[[transformers.Sam2VideoPromptEncoderConfig]]

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

This is the configuration class to store the configuration of a Sam2VideoModel. It is used to instantiate a Sam2 Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam2_video.1-hiera-tiny](https://huggingface.co/facebook/sam2_video.1-hiera-tiny)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam2VideoProcessor[[transformers.Sam2VideoProcessor]]

- **image_processor** (`Sam2ImageProcessor`) --
  The image processor is a required input.
- **video_processor** (`Sam2VideoVideoProcessor`) --
  The video processor is a required input.
- **target_size** (`int`, *optional*) --
  The target size (in pixels) for normalizing input points and bounding boxes. If not provided, defaults
  to the image processor's size configuration. All input coordinates (points and boxes) are normalized
  to this size before being passed to the model. This ensures consistent coordinate representation
  regardless of the original image dimensions.
- **point_pad_value** (`int`, *optional*, defaults to -10) --
  The value used for padding input points when batching sequences of different lengths. This value is
  used to mark padded positions and is preserved during coordinate normalization.
Constructs a Sam2VideoProcessor which wraps a image processor and a video processor into a single processor.

[Sam2VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/sam2_video#transformers.Sam2VideoProcessor) offers all the functionalities of [Sam2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam2#transformers.Sam2ImageProcessor) and [Sam2VideoVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/sam2_video#transformers.Sam2VideoVideoProcessor). See the
[~Sam2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam2#transformers.Sam2ImageProcessor) and [~Sam2VideoVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/sam2_video#transformers.Sam2VideoVideoProcessor) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to process.
- **input_points** (`list[list[list[list[float]]]]`, `torch.Tensor`, *optional*) --
  The points to add to the frame.
- **input_labels** (`list[list[list[int]]]`, `torch.Tensor`, *optional*) --
  The labels for the points.
- **input_boxes** (`list[list[list[float]]]`, `torch.Tensor`, *optional*) --
  The bounding boxes to add to the frame.
- **original_sizes** (`list[list[float]]`, `torch.Tensor`, *optional*) --
  The original sizes of the images.
- **return_tensors** (`Union[str, ~utils.generic.TensorType]`, *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.A [BatchEncoding](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields- `pixel_values` (`torch.Tensor`): The processed image(s).
- `original_sizes` (`list[list[float]]`): The original sizes of the images.
- `labels` (`torch.Tensor`): The processed segmentation maps (if provided).
- `input_points` (`torch.Tensor`): The processed points.
- `input_labels` (`torch.Tensor`): The processed labels.
- `input_boxes` (`torch.Tensor`): The processed bounding boxes.

- **masks** (`Union[List[torch.Tensor], List[np.ndarray]]`) --
  Batched masks from the mask_decoder in (batch_size, num_channels, height, width) format.
- **original_sizes** (`Union[torch.Tensor, List[Tuple[int,int]]]`) --
  The original sizes of each image before it was resized to the model's expected input shape, in (height,
  width) format.
- **mask_threshold** (`float`, *optional*, defaults to 0.0) --
  Threshold for binarization and post-processing operations.
- **binarize** (`bool`, *optional*, defaults to `True`) --
  Whether to binarize the masks.
- **max_hole_area** (`float`, *optional*, defaults to 0.0) --
  The maximum area of a hole to fill.
- **max_sprinkle_area** (`float`, *optional*, defaults to 0.0) --
  The maximum area of a sprinkle to fill.
- **apply_non_overlapping_constraints** (`bool`, *optional*, defaults to `False`) --
  Whether to apply non-overlapping constraints to the masks.(`torch.Tensor`)Batched masks in batch_size, num_channels, height, width) format, where (height, width)
is given by original_size.

Remove padding and upscale masks to the original image size.

- **video** (`VideoInput`, *optional*) --
  The video to process. No need to provide when streaming.
- **inference_device** (`str` or `torch.device`, *optional*, defaults to "cpu") --
  The device to use for inference.
- **inference_state_device** (`str` or `torch.device`, *optional*) --
  The device to store the inference state on.
- **processing_device** (`str` or `torch.device`, *optional*) --
  The device to use for video processing.
- **video_storage_device** (`str` or `torch.device`, *optional*) --
  The device to store the processed video frames on.
- **max_vision_features_cache_size** (`int`, *optional*, defaults to 1) --
  The maximum number of vision features to cache.
- **dtype** (`torch.dtype`, *optional*, defaults to `torch.float32`) --
  The torch dtype to use for the whole session.

Initializes a video session for inference.
If a video is provided (async inference), the video will be processed and stored on the `video_storage_device`.

)>], NoneType] = None"}, {"name": "original_size", "val": ": tuple[int, int] | None = None"}, {"name": "clear_old_inputs", "val": ": bool = True"}]}>
- **inference_session** (`Sam2VideoInferenceSession`) --
  The inference session for the video.
- **frame_idx** (`int`) --
  The index of the frame to process.
- **obj_ids** (`list[int]` or `int`) --
  The object ID(s) to associate with the points or box.
  These can be any integers and can be reused later on to specify an object.
- **input_points** (`list[list[list[list[float]]]]`, `torch.Tensor`, *optional*) --
  The points to add to the frame.
- **input_labels** (`list[list[list[int]]]`, `torch.Tensor`, *optional*) --
  The labels for the points.
- **input_boxes** (`list[list[list[float]]]`, `torch.Tensor`, *optional*) --
  The bounding boxes to add to the frame.
- **input_masks** (`np.ndarray`, `torch.Tensor`, `list[np.ndarray]`, or `list[torch.Tensor]`, *optional*) --
  The mask(s) to add to the frame.
- **original_size** (`tuple[int, int]`, *optional*) --
  The original size of the video. Provide when streaming.
- **clear_old_inputs** (`bool`, *optional*, defaults to `True`) --
  Whether to clear old inputs for the object.

Process new points, boxes, or masks for a video frame and add them to the inference session.

## Sam2VideoVideoProcessor[[transformers.Sam2VideoVideoProcessor]]

- **masks** (`Union[List[torch.Tensor], List[np.ndarray]]`) --
  Batched masks from the mask_decoder in (batch_size, num_channels, height, width) format.
- **original_sizes** (`Union[torch.Tensor, List[Tuple[int,int]]]`) --
  The original sizes of each image before it was resized to the model's expected input shape, in (height,
  width) format.
- **reshaped_input_sizes** (`Union[torch.Tensor, List[Tuple[int,int]]]`) --
  The size of each image as it is fed to the model, in (height, width) format. Used to remove padding.
- **mask_threshold** (`float`, *optional*, defaults to 0.0) --
  The threshold to use for binarizing the masks.
- **binarize** (`bool`, *optional*, defaults to `True`) --
  Whether to binarize the masks.
- **pad_size** (`int`, *optional*, defaults to `self.pad_size`) --
  The target size the images were padded to before being passed to the model. If None, the target size is
  assumed to be the processor's `pad_size`.(`torch.Tensor`)Batched masks in batch_size, num_channels, height, width) format, where (height, width)
is given by original_size.

Remove padding and upscale masks to the original image size.

## Sam2VideoInferenceSession[[transformers.Sam2VideoInferenceSession]]

- **video** (`torch.FloatTensor`, *optional*) --
  The video to process. No need to provide when streaming.
- **video_height** (`int`, *optional*) --
  The height of the video.
- **video_width** (`int`, *optional*) --
  The width of the video.
- **inference_device** (`torch.device`, *optional*, defaults to `"cpu"`) --
  The device to use for inference.
- **inference_state_device** (`torch.device`, *optional*, defaults to `"cpu"`) --
  The device to store the inference state on.
- **video_storage_device** (`torch.device`, *optional*, defaults to `"cpu"`) --
  The device to store the video on.
- **dtype** (`torch.dtype`, *optional*, defaults to `"float32"`) --
  The dtype to use for the video.
- **max_vision_features_cache_size** (`int`, *optional*, defaults to 1) --
  The maximum number of vision features to cache.

Manages video inference session parameters, state and cache.

)>"}]}>

Add mask inputs with automatic device placement.

)>"}, {"name": "frame_idx", "val": ": int | None = None"}]}>

Add new frame with automatic device placement.

Add point inputs with automatic device placement.

Get frame from video.

Get the total number of unique object ids received so far in this session.

- **obj_idx** (int) -- The index of the object.
- **frame_idx** (int) -- The index of the frame.
- **output_key** (str) -- The key of the output.
- **is_conditioning_frame** (bool) -- Whether the output is for a conditioning frame.

Get output with smart device management.

Map object ID to index, creating new entry if needed.

Map model-side object index to client-side object id.

Remove mask inputs.

Remove point inputs.

Reset tracking data and cache.

Reset tracking data but keep cache.

- **obj_idx** (int) -- The index of the object.
- **frame_idx** (int) -- The index of the frame.
- **output_key** (Optional[str]) -- The key of the output. If None, the output is stored as a dictionary.
- **output_value** (Optional[Union[torch.Tensor, dict]]) -- The value of the output.
- **is_conditioning_frame** (bool) -- Whether the output is for a conditioning frame.

Store output with smart device management.
If output_key is None, the output is stored as a dictionary.

## Sam2VideoModel[[transformers.Sam2VideoModel]]

- **config** ([Sam2VideoConfig](/docs/transformers/v5.14.0/en/model_doc/sam2_video#transformers.Sam2VideoConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Sam2 Video Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **inference_session** (`~models.sam2_video.modeling_sam2_video.Sam2VideoInferenceSession`) --
  The video inference session object.
- **frame_idx** (`int`, *optional*) --
  The index of the frame on which to run inference. No need to provide when inferring
  on a new streamed frame.
- **frame** (`torch.Tensor`, *optional*) --
  The frame to process. Provide when streaming.
- **reverse** (`bool`, *optional*, defaults to `False`) --
  Whether to propagate in reverse.
- **run_mem_encoder** (`bool`, *optional*, defaults to `True`) --
  Whether to run the memory encoder on predicted masks. The memory encoder is batched across all objects for efficiency.`Sam2VideoSegmentationOutput` or `tuple(torch.FloatTensor)`A `Sam2VideoSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam2VideoConfig](/docs/transformers/v5.14.0/en/model_doc/sam2_video#transformers.Sam2VideoConfig)) and inputs.
Propagate the objects through a streamed video frame.

- **object_ids** (`list[int]`, *optional*) -- List of object IDs being tracked in the current frame.
- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, num_masks, height, width)`) -- The predicted masks stored at the model's resolution.
- **object_score_logits** (`torch.FloatTensor` of shape `(batch_size,)`, *optional*) -- Logits for the object scores, indicating if objects are present.
- **frame_idx** (`int`, *optional*) -- The frame index of the video.

- **inference_session** (`~models.sam2_video.modeling_sam2_video.Sam2VideoInferenceSession`) --
  The video inference session object.
- **start_frame_idx** (`int`, *optional*) --
  The starting frame index for propagation.
  Need to be provided if `forward` hasn't been called on new inputs yet.
  If not provided, the starting frame index will be the earliest frame with input points.
- **max_frame_num_to_track** (`int`, *optional*) --
  The maximum number of frames to track.
- **reverse** (`bool`, *optional*, defaults to `False`) --
  Whether to propagate in reverse.
- **show_progress_bar** (`bool`, *optional*, defaults to `False`) --
  Whether to show a progress bar during propagation.`Sam2VideoSegmentationOutput` or `tuple(torch.FloatTensor)`A `Sam2VideoSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam2VideoConfig](/docs/transformers/v5.14.0/en/model_doc/sam2_video#transformers.Sam2VideoConfig)) and inputs.

Propagate the objects through the video frames. Used when initializing an inference session with a whole video.
Yields Sam2VideoSegmentationOutput for each frame.

- **object_ids** (`list[int]`, *optional*) -- List of object IDs being tracked in the current frame.
- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, num_masks, height, width)`) -- The predicted masks stored at the model's resolution.
- **object_score_logits** (`torch.FloatTensor` of shape `(batch_size,)`, *optional*) -- Logits for the object scores, indicating if objects are present.
- **frame_idx** (`int`, *optional*) -- The frame index of the video.

- **pixel_values** (`torch.FloatTensor`) --
  Input pixel values of shape `(batch_size, num_channels, height, width)`.`Sam2VideoVisionEncoderOutput` or `tuple(torch.FloatTensor)`A `Sam2VideoVisionEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam2VideoConfig](/docs/transformers/v5.14.0/en/model_doc/sam2_video#transformers.Sam2VideoConfig)) and inputs.

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
