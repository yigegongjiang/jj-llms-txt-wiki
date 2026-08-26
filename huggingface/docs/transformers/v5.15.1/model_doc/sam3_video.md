# SAM3 Video

    
        
        
    

## Overview

SAM3 (Segment Anything Model 3) was introduced in [SAM 3: Segment Anything with Concepts](https://ai.meta.com/research/publications/sam-3-segment-anything-with-concepts/).

SAM3 Video performs **Promptable Concept Segmentation (PCS)** on videos. PCS takes text as input (e.g., "yellow school bus"), and predicts instance and semantic masks for **every single object** matching the concept, while preserving object identities across video frames.

The model combines a detection module (SAM3) with a tracking module (SAM2-style tracker) to enable robust object tracking across video frames using text prompts.

The abstract from the paper is the following:

*We present Segment Anything Model (SAM) 3, a unified model that detects, segments, and tracks objects in images and videos based on concept prompts, which we define as either short noun phrases (e.g., "yellow school bus"), image exemplars, or a combination of both. Promptable Concept Segmentation (PCS) takes such prompts and returns segmentation masks and unique identities for all matching object instances. To advance PCS, we build a scalable data engine that produces a high-quality dataset with 4M unique concept labels, including hard negatives, across images and videos. Our model consists of an image-level detector and a memory-based video tracker that share a single backbone. Recognition and localization are decoupled with a presence head, which boosts detection accuracy. SAM 3 doubles the accuracy of existing systems in both image and video PCS, and improves previous SAM capabilities on visual segmentation tasks. We open source SAM 3 along with our new Segment Anything with Concepts (SA-Co) benchmark for promptable concept segmentation.*

This model was contributed by [yonigozlan](https://huggingface.co/yonigozlan) and [ronghanghu](https://huggingface.co/ronghanghu).

## Usage example

### Video Segmentation and Tracking

#### Pre-loaded Video Inference

Process a video with all frames already available using text prompts:

```python
from transformers import Sam3VideoModel, Sam3VideoProcessor
import torch

model = Sam3VideoModel.from_pretrained("facebook/sam3", device_map="auto")
processor = Sam3VideoProcessor.from_pretrained("facebook/sam3")

# Load video frames
from transformers.video_utils import load_video
video_url = "https://huggingface.co/datasets/hf-internal-testing/sam2-fixtures/resolve/main/bedroom.mp4"
video_frames, _ = load_video(video_url)

# Initialize video inference session
inference_session = processor.init_video_session(
    video=video_frames,
    inference_device=device,
    processing_device="cpu",
    video_storage_device="cpu",
)

# Add text prompt to detect and track objects
text = "person"
inference_session = processor.add_text_prompt(
    inference_session=inference_session,
    text=text,
)

# Process all frames in the video
outputs_per_frame = {}
# Pass show_progress_bar=True to display a tqdm progress bar.
for model_outputs in model.propagate_in_video_iterator(
    inference_session=inference_session, max_frame_num_to_track=50
):
    processed_outputs = processor.postprocess_outputs(inference_session, model_outputs)
    outputs_per_frame[model_outputs.frame_idx] = processed_outputs

print(f"Processed {len(outputs_per_frame)} frames")
Processed 51 frames

# Access results for a specific frame
frame_0_outputs = outputs_per_frame[0]
print(f"Detected {len(frame_0_outputs['object_ids'])} objects")
print(f"Object IDs: {frame_0_outputs['object_ids'].tolist()}")
print(f"Scores: {frame_0_outputs['scores'].tolist()}")
print(f"Boxes shape (XYXY format, absolute coordinates): {frame_0_outputs['boxes'].shape}")
print(f"Masks shape: {frame_0_outputs['masks'].shape}")
```

You can also track multiple object categories simultaneously by providing multiple prompts. The model efficiently reuses vision features across all prompts:

```python
# Add multiple text prompts (or use a list in add_text_prompt)
multi_prompt_session = processor.init_video_session(
    video=video_frames,
    inference_device=device,
    processing_device="cpu",
    video_storage_device="cpu",
)
>>>
prompts = ["person", "bed", "lamp"]
processor.add_text_prompt(multi_prompt_session, prompts)
>>>
# Process video - detects objects from ALL prompts in a single pass
multi_outputs_per_frame = {}
# Pass show_progress_bar=True to display a tqdm progress bar.
for model_outputs in model.propagate_in_video_iterator(
    inference_session=multi_prompt_session, max_frame_num_to_track=50
):
    processed_outputs = processor.postprocess_outputs(multi_prompt_session, model_outputs)
    multi_outputs_per_frame[model_outputs.frame_idx] = processed_outputs
>>>
# Check which objects were detected by each prompt
frame_0_outputs = multi_outputs_per_frame[0]
prompt_to_obj_ids = frame_0_outputs["prompt_to_obj_ids"]
for prompt, obj_ids in prompt_to_obj_ids.items():
    print(f"{prompt}: {len(obj_ids)} objects")
person: 2 objects
bed: 1 objects
lamp: 1 objects
```

#### Streaming Video Inference

⚠️ **Note on Streaming Inference Quality**: Streaming inference disables hotstart heuristics that remove unmatched and duplicate objects, as these require access to future frames to make informed decisions. This may result in more false positive detections and duplicate object tracks compared to pre-loaded video inference. For best results, use pre-loaded video inference when all frames are available.

For real-time applications, SAM3 Video supports processing video frames as they arrive:

```python
# Initialize session for streaming
streaming_inference_session = processor.init_video_session(
    inference_device=device,
    processing_device="cpu",
    video_storage_device="cpu",
)

# Add text prompt
text = "person"
streaming_inference_session = processor.add_text_prompt(
    inference_session=streaming_inference_session,
    text=text,
)

# Process frames one by one (streaming mode)
streaming_outputs_per_frame = {}
for frame_idx, frame in enumerate(video_frames[:50]):  # Process first 50 frames
    # First, process the frame using the processor
    inputs = processor(images=frame, device=device, return_tensors="pt").to(model.device)
...
    # Process frame using streaming inference - pass the processed pixel_values
    model_outputs = model(
        inference_session=streaming_inference_session,
        frame=inputs.pixel_values[0],  # Provide processed frame - this enables streaming mode
        reverse=False,
    )
...
    # Post-process outputs with original_sizes for proper resolution handling
    processed_outputs = processor.postprocess_outputs(
        streaming_inference_session,
        model_outputs,
        original_sizes=inputs.original_sizes,  # Required for streaming inference
    )
    streaming_outputs_per_frame[frame_idx] = processed_outputs
...
    if (frame_idx + 1) % 10 == 0:
        print(f"Processed {frame_idx + 1} frames...")

print(f"✓ Streaming inference complete! Processed {len(streaming_outputs_per_frame)} frames")
✓ Streaming inference complete! Processed 50 frames

# Access results
frame_0_outputs = streaming_outputs_per_frame[0]
print(f"Detected {len(frame_0_outputs['object_ids'])} objects in first frame")
print(f"Boxes are in XYXY format (absolute pixel coordinates): {frame_0_outputs['boxes'].shape}")
print(f"Masks are at original video resolution: {frame_0_outputs['masks'].shape}")
```

#### Custom Resolution Inference

⚠️ **Performance Note**: Custom resolutions may degrade accuracy. The model is meant to be used at 1008px resolution.

For faster inference or lower memory usage:

```python
config = Sam3VideoConfig.from_pretrained("facebook/sam3")
config.image_size = 560
model = Sam3VideoModel.from_pretrained("facebook/sam3", config=config, device_map="auto")
processor = Sam3VideoProcessor.from_pretrained("facebook/sam3", size={"height": 560, "width": 560})
```

## Sam3VideoConfig[[transformers.Sam3VideoConfig]]

#### transformers.Sam3VideoConfig[[transformers.Sam3VideoConfig]]

```python
transformers.Sam3VideoConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, detector_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, tracker_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, initializer_range: float = 0.02, low_res_mask_size: int = 288, score_threshold_detection: float = 0.5, det_nms_thresh: float = 0.1, assoc_iou_thresh: float = 0.1, trk_assoc_iou_thresh: float = 0.5, new_det_thresh: float = 0.7, recondition_on_trk_masks: bool = True, hotstart_delay: int = 15, hotstart_unmatch_thresh: int = 8, hotstart_dup_thresh: int = 8, suppress_unmatched_only_within_hotstart: bool = True, init_trk_keep_alive: int = 30, max_trk_keep_alive: int = 30, min_trk_keep_alive: int = -1, suppress_overlapping_based_on_recent_occlusion_threshold: float = 0.7, decrease_trk_keep_alive_for_empty_masklets: bool = False, fill_hole_area: int = 16, max_num_objects: int = 10000, recondition_every_nth_frame: int = 16, high_conf_thresh: float = 0.8, high_iou_thresh: float = 0.8)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/configuration_sam3_video.py#L28)

**Parameters:**

detector_config (`dict` or `Sam3Config`, *optional*) : Configuration for the Sam3 detector model. If not provided, default Sam3Config will be used.

tracker_config (`dict` or `Sam2VideoConfig`, *optional*) : Configuration for the Sam2Video tracker model. If not provided, default Sam2VideoConfig will be used.

initializer_range (`float`, *optional*, defaults to 0.02) : The standard deviation of the truncated_normal_initializer for initializing weight matrices.

low_res_mask_size (`int`, *optional*, defaults to 288) : Size (height and width) of the low-resolution mask outputs from the tracker before upsampling to video resolution.

score_threshold_detection (`float`, *optional*, defaults to 0.5) : Probability threshold for detection outputs - only keep detections above this threshold.

det_nms_thresh (`float`, *optional*, defaults to 0.1) : IoU threshold for detection NMS (Non-Maximum Suppression).

assoc_iou_thresh (`float`, *optional*, defaults to 0.1) : IoU threshold for detection-to-track matching. A detection is considered "matched" to a tracklet if it overlaps with the tracklet above this threshold. Often a loose threshold like 0.1.

trk_assoc_iou_thresh (`float`, *optional*, defaults to 0.5) : IoU threshold for detection-to-track matching, used to determine whether a masklet is "unmatched" by any detections. Often a stricter threshold like 0.5.

new_det_thresh (`float`, *optional*, defaults to 0.7) : Probability threshold for a detection to be added as a new object.

recondition_on_trk_masks (`bool`, *optional*, defaults to `True`) : Whether to use tracked masks (True) or detection masks (False) for reconditioning. Use True when tracked masks are higher quality and detector serves as validation signal to strengthen memory and prevent drift.

hotstart_delay (`int`, *optional*, defaults to 15) : Number of frames to buffer outputs during hotstart. We hold off the outputs for `hotstart_delay` frames and remove tracklets based on hotstart heuristics.

hotstart_unmatch_thresh (`int`, *optional*, defaults to 8) : Number of unmatched frames required to remove a tracklet during hotstart period.

hotstart_dup_thresh (`int`, *optional*, defaults to 8) : Number of overlapping frames required to remove a duplicate tracklet during hotstart period.

suppress_unmatched_only_within_hotstart (`bool`, *optional*, defaults to `True`) : Whether to suppress masks only within hotstart period. If False, we can suppress masks even if they start before hotstart period.

init_trk_keep_alive (`int`, *optional*, defaults to 30) : Initial keep-alive counter for new tracks.

max_trk_keep_alive (`int`, *optional*, defaults to 30) : Maximum keep-alive counter value. Tracks with matched detections get their counter increased up to this value.

min_trk_keep_alive (`int`, *optional*, defaults to -1) : Minimum keep-alive counter value. Tracks with unmatched detections get their counter decreased to this value.

suppress_overlapping_based_on_recent_occlusion_threshold (`float`, *optional*, defaults to 0.7) : Threshold for suppressing overlapping objects based on recent occlusion. Overlapping masks with IoU above this threshold are suppressed based on which was most recently occluded.

decrease_trk_keep_alive_for_empty_masklets (`bool`, *optional*, defaults to `False`) : Whether to decrease keep-alive counter for masklets with zero area in SAM2 prediction.

fill_hole_area (`int`, *optional*, defaults to 16) : Minimum area (in pixels) for filling holes in masks and removing small sprinkles.

max_num_objects (`int`, *optional*, defaults to 10000) : Maximum number of objects to track. Default 10000 effectively turns off this limit.

recondition_every_nth_frame (`int`, *optional*, defaults to 16) : Frequency of mask reconditioning (in frames). Set to 0 to disable reconditioning.

high_conf_thresh (`float`, *optional*, defaults to 0.8) : High confidence threshold for reconditioning. Only detections above this threshold can recondition tracklets.

high_iou_thresh (`float`, *optional*, defaults to 0.8) : High IoU threshold for reconditioning. Only detections with IoU above this threshold can recondition tracklets.

This is the configuration class to store the configuration of a Sam3VideoModel. It is used to instantiate a Sam3 Video
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam3](https://huggingface.co/facebook/sam3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import Sam3VideoConfig, Sam3VideoModel

>>> # Initializing a SAM3 Video configuration with default detector and tracker
>>> configuration = Sam3VideoConfig()

>>> # Changing image size for custom resolution inference (automatically propagates to all nested configs)
>>> configuration.image_size = 560

>>> # Initializing a model from the configuration
>>> model = Sam3VideoModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
>>> detector_config = configuration.detector_config
>>> tracker_config = configuration.tracker_config
```

## Sam3VideoProcessor[[transformers.Sam3VideoProcessor]]

#### transformers.Sam3VideoProcessor[[transformers.Sam3VideoProcessor]]

```python
transformers.Sam3VideoProcessor(image_processor, video_processor, tokenizer, target_size: int | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/processing_sam3_video.py#L39)

**Parameters:**

image_processor (`Sam3ImageProcessor`) : The image processor is a required input.

video_processor (`video_processor_class`) : The video processor is a required input.

tokenizer (`CLIPTokenizer`) : The tokenizer is a required input.

target_size (`int`, *optional*) : The target size (target_size, target_size) to which the image will be resized.

Constructs a Sam3VideoProcessor which wraps a image processor, a video processor, and a tokenizer into a single processor.

[Sam3VideoProcessor](/docs/transformers/v5.15.1/en/model_doc/sam3_video#transformers.Sam3VideoProcessor) offers all the functionalities of [Sam3ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3ImageProcessor), `video_processor_class`, and [CLIPTokenizer](/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer). See the
[~Sam3ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam3#transformers.Sam3ImageProcessor), `~video_processor_class`, and [~CLIPTokenizer](/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer) for more information.

#### __call__[[transformers.Sam3VideoProcessor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, segmentation_maps: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, original_sizes: typing.Union[list[list[float]], torch.Tensor, NoneType] = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/processing_sam3_video.py#L55)

**Parameters:**

images (`ImageInput`, *optional*) : The image(s) to process.

segmentation_maps (`ImageInput`, *optional*) : The segmentation maps to process (optional, for image processor).

original_sizes (`list[list[float]]`, `torch.Tensor`, *optional*) : The original sizes of the images. Only used when images is not provided.

return_tensors (`Union[str, ~utils.generic.TensorType]`, *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

**Returns:** A [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields

- `pixel_values` (`torch.Tensor`): The processed image(s).
- `original_sizes` (`list[list[float]]`): The original sizes of the images.
- `labels` (`torch.Tensor`, *optional*): The processed segmentation maps (if provided).

#### postprocess_outputs[[transformers.Sam3VideoProcessor.postprocess_outputs]]

```python
postprocess_outputs(inference_session, model_outputs, original_sizes: typing.Union[list[list[float]], torch.Tensor, NoneType] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/processing_sam3_video.py#L247)

**Parameters:**

inference_session (`Sam3VideoInferenceSession`) : The inference session object.

model_outputs (`Sam3VideoSegmentationOutput`) : The raw model output from `Sam3VideoModel.forward()`.

original_sizes (`list[list[float]]` or `torch.Tensor`, *optional*) : Optional original frame sizes [height, width]. Required for streaming inference when video_height/video_width are not set in the session.

**Returns:** `dict`

A dictionary containing the following keys:
- **object_ids** (`torch.Tensor` of shape `(num_objects,)`): Object IDs for each detected object.
- **scores** (`torch.Tensor` of shape `(num_objects,)`): Detection scores for each object.
- **boxes** (`torch.Tensor` of shape `(num_objects, 4)`): Bounding boxes in XYXY format
  (top_left_x, top_left_y, bottom_right_x, bottom_right_y).
- **masks** (`torch.Tensor` of shape `(num_objects, height, width)`): Binary segmentation masks
  for each object at the original video resolution.
- **prompt_to_obj_ids** (`dict[str, list[int]]`): Mapping from prompt text to list of
  object IDs detected by that prompt.

Post-process model outputs to get final masks, boxes, and scores.

#### init_video_session[[transformers.Sam3VideoProcessor.init_video_session]]

```python
init_video_session(video: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]], NoneType] = None, inference_device: typing.Union[str, ForwardRef('torch.device')] = 'cpu', inference_state_device: typing.Union[str, ForwardRef('torch.device'), NoneType] = None, processing_device: typing.Union[str, ForwardRef('torch.device'), NoneType] = None, video_storage_device: typing.Union[str, ForwardRef('torch.device'), NoneType] = None, max_vision_features_cache_size: int = 1, dtype: dtype = torch.float32)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/processing_sam3_video.py#L133)

**Parameters:**

video (`VideoInput`, *optional*) : The video to process. No need to provide when streaming.

inference_device (`str` or `torch.device`, *optional*, defaults to "cpu") : The device to use for inference.

inference_state_device (`str` or `torch.device`, *optional*) : The device to store the inference state on.

processing_device (`str` or `torch.device`, *optional*) : The device to use for video processing.

video_storage_device (`str` or `torch.device`, *optional*) : The device to store the processed video frames on.

max_vision_features_cache_size (`int`, *optional*, defaults to 1) : The maximum number of vision features to cache.

dtype (`torch.dtype`, *optional*, defaults to `torch.float32`) : The torch dtype to use for the whole session.

Initializes a video session for inference.
If a video is provided (async inference), the video will be processed and stored on the `video_storage_device`.

#### add_text_prompt[[transformers.Sam3VideoProcessor.add_text_prompt]]

```python
add_text_prompt(inference_session: Sam3VideoInferenceSession, text: str | list[str])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/processing_sam3_video.py#L101)

**Parameters:**

inference_session (`Sam3VideoInferenceSession`) : The inference session.

text (`str` or `list[str]`) : The text prompt(s) to add.

**Returns:** `Sam3VideoInferenceSession`

The inference session with the added text prompt(s).

Add text prompt(s) to the inference session.

## Sam3VideoInferenceSession[[transformers.Sam3VideoInferenceSession]]

#### transformers.Sam3VideoInferenceSession[[transformers.Sam3VideoInferenceSession]]

```python
transformers.Sam3VideoInferenceSession(video: typing.Optional[torch.FloatTensor] = None, video_height: int | None = None, video_width: int | None = None, inference_device: typing.Union[torch.device, str] = 'cpu', inference_state_device: typing.Union[torch.device, str] = 'cpu', video_storage_device: typing.Union[torch.device, str] = 'cpu', dtype: typing.Union[torch.dtype, str] = 'float32', max_vision_features_cache_size: int = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L119)

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

#### add_mask_inputs[[transformers.Sam3VideoInferenceSession.add_mask_inputs]]

```python
add_mask_inputs(obj_idx: int, frame_idx: int, inputs: Tensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L255)

Add mask inputs with automatic device placement.

#### add_new_frame[[transformers.Sam3VideoInferenceSession.add_new_frame]]

```python
add_new_frame(pixel_values: Tensor, frame_idx: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L384)

Add new frame with automatic device placement.

#### add_prompt[[transformers.Sam3VideoInferenceSession.add_prompt]]

```python
add_prompt(prompt_text: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L215)

Add a text prompt to the session and return its unique ID.
If the prompt already exists, returns the existing ID.

#### get_frame[[transformers.Sam3VideoInferenceSession.get_frame]]

```python
get_frame(frame_idx: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L400)

Get frame from video.

#### get_obj_num[[transformers.Sam3VideoInferenceSession.get_obj_num]]

```python
get_obj_num()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L251)

Get the total number of unique object ids received so far in this session.

#### get_output[[transformers.Sam3VideoInferenceSession.get_output]]

```python
get_output(obj_idx: int, frame_idx: int, output_key: str, is_conditioning_frame: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L357)

**Parameters:**

obj_idx (int) : The index of the object.

frame_idx (int) : The index of the frame.

output_key (str) : The key of the output.

is_conditioning_frame (bool) : Whether the output is for a conditioning frame.

Get output with smart device management.

#### obj_id_to_idx[[transformers.Sam3VideoInferenceSession.obj_id_to_idx]]

```python
obj_id_to_idx(obj_id: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L229)

Map object ID to index, creating new entry if needed.

#### obj_idx_to_id[[transformers.Sam3VideoInferenceSession.obj_idx_to_id]]

```python
obj_idx_to_id(obj_idx: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L247)

Map model-side object index to client-side object id.

#### remove_mask_inputs[[transformers.Sam3VideoInferenceSession.remove_mask_inputs]]

```python
remove_mask_inputs(obj_idx: int, frame_idx: int)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L261)

Remove mask inputs.

#### remove_object[[transformers.Sam3VideoInferenceSession.remove_object]]

```python
remove_object(obj_id: int, strict: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L265)

**Parameters:**

obj_id (`int`) : The object ID to remove.

strict (`bool`, *optional*, defaults to `False`) : Whether to raise an error if the object doesn't exist.

Remove an object from the inference session. This would remove the object from
all frames in the video.

#### reset_inference_session[[transformers.Sam3VideoInferenceSession.reset_inference_session]]

```python
reset_inference_session()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L416)

Reset tracking data and cache.

#### reset_state[[transformers.Sam3VideoInferenceSession.reset_state]]

```python
reset_state()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L428)

Reset the inference session state.

#### reset_tracking_data[[transformers.Sam3VideoInferenceSession.reset_tracking_data]]

```python
reset_tracking_data()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L404)

Reset tracking data but keep cache.

#### store_output[[transformers.Sam3VideoInferenceSession.store_output]]

```python
store_output(obj_idx: int, frame_idx: int, output_key: str | None = None, output_value: typing.Union[torch.Tensor, dict, NoneType] = None, is_conditioning_frame: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L320)

**Parameters:**

obj_idx (int) : The index of the object.

frame_idx (int) : The index of the frame.

output_key (Optional[str]) : The key of the output. If None, the output is stored as a dictionary.

output_value (Optional[Union[torch.Tensor, dict]]) : The value of the output.

is_conditioning_frame (bool) : Whether the output is for a conditioning frame.

Store output with smart device management.
If output_key is None, the output is stored as a dictionary.

## Sam3VideoSegmentationOutput[[transformers.Sam3VideoSegmentationOutput]]

#### transformers.Sam3VideoSegmentationOutput[[transformers.Sam3VideoSegmentationOutput]]

```python
transformers.Sam3VideoSegmentationOutput(object_ids: list[int] | None = None, obj_id_to_mask: dict[int, torch.FloatTensor] | None = None, obj_id_to_score: dict[int, float] | None = None, obj_id_to_tracker_score: dict[int, float] | None = None, removed_obj_ids: set[int] | None = None, suppressed_obj_ids: set[int] | None = None, frame_idx: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L462)

**Parameters:**

object_ids (`list[int]`, *optional*) : List of object IDs being tracked in the current frame.

obj_id_to_mask (`dict[int, torch.FloatTensor]`, *optional*) : Dictionary mapping object IDs to their predicted low-resolution masks. Each mask has shape `(1, H_low, W_low)`.

obj_id_to_score (`dict[int, float]`, *optional*) : Dictionary mapping object IDs to their detection scores.

obj_id_to_tracker_score (`dict[int, float]`, *optional*) : Dictionary mapping object IDs to their tracker scores for the current frame.

removed_obj_ids (`set[int]`, *optional*) : Set of object IDs that have been removed (e.g., via hotstart heuristics).

suppressed_obj_ids (`set[int]`, *optional*) : Set of object IDs that have been suppressed in the current frame.

frame_idx (`int`, *optional*) : The frame index of the video.

Base class for the Sam3Video model's output.

## Sam3VideoModel[[transformers.Sam3VideoModel]]

#### transformers.Sam3VideoModel[[transformers.Sam3VideoModel]]

```python
transformers.Sam3VideoModel(config: Sam3VideoConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L507)

**Parameters:**

config ([Sam3VideoConfig](/docs/transformers/v5.15.1/en/model_doc/sam3_video#transformers.Sam3VideoConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Sam3 Video Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Sam3VideoModel.forward]]

```python
forward(inference_session: Sam3VideoInferenceSession, frame_idx: int | None = None, frame: typing.Optional[torch.Tensor] = None, reverse: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L1693)

**Parameters:**

inference_session (`~models.sam3_video.modeling_sam3_video.Sam3VideoInferenceSession`) : The video inference session object.

frame_idx (`int`, *optional*) : The index of the frame on which to run inference. No need to provide when inferring on a new streamed frame.

frame (`torch.Tensor`, *optional*) : The frame to process. Provide when streaming.

reverse (`bool`, *optional*, defaults to `False`) : Whether to propagate in reverse.

Propagate the objects through a streamed video frame.

#### propagate_in_video_iterator[[transformers.Sam3VideoModel.propagate_in_video_iterator]]

```python
propagate_in_video_iterator(inference_session: Sam3VideoInferenceSession, start_frame_idx: int = 0, max_frame_num_to_track: int | None = None, reverse: bool = False, show_progress_bar: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam3_video/modeling_sam3_video.py#L1784)

**Parameters:**

inference_session (`~models.sam3_video.modeling_sam3_video.Sam3VideoInferenceSession`) : The video inference session object.

start_frame_idx (`int`, *optional*, defaults to `0`) : The starting frame index for propagation.

max_frame_num_to_track (`int`, *optional*) : The maximum number of frames to track. If not provided, all frames in the video will be tracked.

reverse (`bool`, *optional*, defaults to `False`) : Whether to propagate in reverse.

show_progress_bar (`bool`, *optional*, defaults to `False`) : Whether to show a progress bar during propagation.

**Returns:** [Sam3VideoSegmentationOutput](/docs/transformers/v5.15.1/en/model_doc/sam3_video#transformers.Sam3VideoSegmentationOutput) or `tuple(torch.FloatTensor)`

A [Sam3VideoSegmentationOutput](/docs/transformers/v5.15.1/en/model_doc/sam3_video#transformers.Sam3VideoSegmentationOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam3VideoConfig](/docs/transformers/v5.15.1/en/model_doc/sam3_video#transformers.Sam3VideoConfig)) and inputs.

Propagate the prompts to get grounding results for the entire video. Used when initializing an inference session with a whole video.
Yields Sam3VideoSegmentationOutput for each frame.

- **object_ids** (`list[int]`, *optional*) -- List of object IDs being tracked in the current frame.
- **obj_id_to_mask** (`dict[int, torch.FloatTensor]`, *optional*) -- Dictionary mapping object IDs to their predicted low-resolution masks.
  Each mask has shape `(1, H_low, W_low)`.
- **obj_id_to_score** (`dict[int, float]`, *optional*) -- Dictionary mapping object IDs to their detection scores.
- **obj_id_to_tracker_score** (`dict[int, float]`, *optional*) -- Dictionary mapping object IDs to their tracker scores for the current frame.
- **removed_obj_ids** (`set[int]`, *optional*) -- Set of object IDs that have been removed (e.g., via hotstart heuristics).
- **suppressed_obj_ids** (`set[int]`, *optional*) -- Set of object IDs that have been suppressed in the current frame.
- **frame_idx** (`int`, *optional*) -- The frame index of the video.
