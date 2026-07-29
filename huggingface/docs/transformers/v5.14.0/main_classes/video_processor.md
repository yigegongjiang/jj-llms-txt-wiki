# Video Processor

A **Video Processor** is a utility responsible for preparing input features for video models, as well as handling the post-processing of their outputs. It provides transformations such as resizing, normalization, and conversion into PyTorch. Along ith transformations the `VideoProcessor` class handles video decoding from local paths or URLs (requires [`torchcodec`](https://pypi.org/project/torchcodec/)) and frame sampling according to model-specific strategies.

The video processor extends the functionality of image processors by allowing Vision Large Language Models (VLMs) to handle videos with a distinct set of arguments compared to images. It serves as the bridge between raw video data and the model, ensuring that input features are optimized for the VLM.

When adding a new VLM or updating an existing one to enable distinct video preprocessing, saving and reloading the processor configuration will store the video related arguments in a dedicated file named `video_preprocessing_config.json`. Don't worry if you haven't updated your VLM, the processor will try to load video related configurations from a file named `preprocessing_config.json`.

## Usage Example

Here's an example of how to load a video processor with [`llava-hf/llava-onevision-qwen2-0.5b-ov-hf`](https://huggingface.co/llava-hf/llava-onevision-qwen2-0.5b-ov-hf) model:

```python
from transformers import AutoVideoProcessor

processor = AutoVideoProcessor.from_pretrained("llava-hf/llava-onevision-qwen2-0.5b-ov-hf")
```

Currently, if using base image processor for videos, it processes video data by treating each frame as an individual image and applying transformations frame-by-frame. While functional, this approach is not highly efficient. Using `AutoVideoProcessor` allows us to take advantage of **fast video processors**, leveraging the [torchvision](https://pytorch.org/vision/stable/index.html) library. Fast processors handle the whole batch of videos at once, without iterating over each video or frame. These updates introduce GPU acceleration and significantly enhance processing speed, especially for tasks requiring high throughput.

Fast video processors are available for all models and are loaded by default when an `AutoVideoProcessor` is initialized. When using a fast video processor, you can also set the `device` argument to specify the device on which the processing should be done. By default, the processing is done on the same device as the inputs if the inputs are tensors, or on the CPU otherwise. For even more speed improvement, we can compile the processor when using 'cuda' as device.

```python
import torch
from transformers.video_utils import load_video
from transformers import AutoVideoProcessor

video = load_video("video.mp4")
processor = AutoVideoProcessor.from_pretrained("llava-hf/llava-onevision-qwen2-0.5b-ov-hf", device="cuda")
processor = torch.compile(processor)
processed_video = processor(video, return_tensors="pt")
```

## Sampling behavior

The video processor can also sample video frames using the technique best suited for the given model. Sampling behavior is controlled with the `do_sample_frames` argument and can be configured through model-specific parameters such as `num_frames` or `fps` (the rate at which the video will be sampled). If the input video is given as a local path or URL (`str`), the processor will decode it automatically. To obtain metadata about the decoded video, such as sampled frame indices, original dimensions, duration, and fps, pass `return_metadata=True` to the processor.

- Specifying `num_frames` does not guarantee the output will contain exactly that number of frames. Depending on the model, the sampler may enforce minimum or maximum frame limits.

- The default decoder is [`torchcodec`](https://pypi.org/project/torchcodec/), which must be installed.

```python
from transformers import AutoVideoProcessor

processor = AutoVideoProcessor.from_pretrained("llava-hf/llava-onevision-qwen2-0.5b-ov-hf", device="cuda")
processed_video_inputs = processor(videos=["video_path.mp4"], return_metadata=True, do_sample_frames=True, return_tensors="pt")
video_metadata = processed_video_inputs["video_metadata"]

# See how many frames the original video had and what was the original FPS
print(video_metadata.total_num_frames, video_metadata.fps)
```

If you pass an already decoded video array but still want to enable model-specific frame sampling, it is strongly recommended to provide video_metadata. This allows the sampler to know the original video’s duration and FPS. You can pass metadata as a `VideoMetadata` object or as a plain dict.

```python
from transformers import AutoVideoProcessor
from transformers.video_utils import VideoMetadata

processor = AutoVideoProcessor.from_pretrained("llava-hf/llava-onevision-qwen2-0.5b-ov-hf", device="cuda")
my_decodec_video = torch.randint(0, 255, size=(100, 3, 1280, 1280)) # short video of 100 frames
video_metadata = VideoMetadata(
    total_num_frames=100,
    fps=24,
    duration=4.1, # in seconds
)
processed_video_inputs = processor(videos=["video_path.mp4"], video_metadata=video_metadata, do_sample_frames=True, num_frames=10, return_tensors="pt")
print(processed_video_inputs.pixel_values_videos.shape)
>>> [10, 3, 384, 384]
```

## BaseVideoProcessor[[transformers.BaseVideoProcessor]]

- **do_resize** (`bool`, *optional*, defaults to `self.do_resize`) --
  Whether to resize the video's (height, width) dimensions to the specified `size`. Can be overridden by the
  `do_resize` parameter in the `preprocess` method.
- **size** (`dict`, *optional*, defaults to `self.size`) --
  Size of the output video after resizing. Can be overridden by the `size` parameter in the `preprocess`
  method.
- **size_divisor** (`int`, *optional*, defaults to `self.size_divisor`) --
  The size by which to make sure both the height and width can be divided.
- **default_to_square** (`bool`, *optional*, defaults to `self.default_to_square`) --
  Whether to default to a square video when resizing, if size is an int.
- **resample** (`PILImageResampling`, *optional*, defaults to `self.resample`) --
  Resampling filter to use if resizing the video. Only has an effect if `do_resize` is set to `True`. Can be
  overridden by the `resample` parameter in the `preprocess` method.
- **do_center_crop** (`bool`, *optional*, defaults to `self.do_center_crop`) --
  Whether to center crop the video to the specified `crop_size`. Can be overridden by `do_center_crop` in the
  `preprocess` method.
- **crop_size** (`dict[str, int]` *optional*, defaults to `self.crop_size`) --
  Size of the output video after applying `center_crop`. Can be overridden by `crop_size` in the `preprocess`
  method.
- **do_rescale** (`bool`, *optional*, defaults to `self.do_rescale`) --
  Whether to rescale the video by the specified scale `rescale_factor`. Can be overridden by the
  `do_rescale` parameter in the `preprocess` method.
- **rescale_factor** (`int` or `float`, *optional*, defaults to `self.rescale_factor`) --
  Scale factor to use if rescaling the video. Only has an effect if `do_rescale` is set to `True`. Can be
  overridden by the `rescale_factor` parameter in the `preprocess` method.
- **do_normalize** (`bool`, *optional*, defaults to `self.do_normalize`) --
  Whether to normalize the video. Can be overridden by the `do_normalize` parameter in the `preprocess`
  method. Can be overridden by the `do_normalize` parameter in the `preprocess` method.
- **image_mean** (`float` or `list[float]`, *optional*, defaults to `self.image_mean`) --
  Mean to use if normalizing the video. This is a float or list of floats the length of the number of
  channels in the video. Can be overridden by the `image_mean` parameter in the `preprocess` method. Can be
  overridden by the `image_mean` parameter in the `preprocess` method.
- **image_std** (`float` or `list[float]`, *optional*, defaults to `self.image_std`) --
  Standard deviation to use if normalizing the video. This is a float or list of floats the length of the
  number of channels in the video. Can be overridden by the `image_std` parameter in the `preprocess` method.
  Can be overridden by the `image_std` parameter in the `preprocess` method.
- **do_convert_rgb** (`bool`, *optional*, defaults to `self.image_std`) --
  Whether to convert the video to RGB.
- **video_metadata** (`VideoMetadata`, *optional*) --
  Metadata of the video containing information about total duration, fps and total number of frames.
- **do_sample_frames** (`int`, *optional*, defaults to `self.do_sample_frames`) --
  Whether to sample frames from the video before processing or to process the whole video.
- **num_frames** (`int`, *optional*, defaults to `self.num_frames`) --
  Maximum number of frames to sample when `do_sample_frames=True`.
- **fps** (`int` or `float`, *optional*, defaults to `self.fps`) --
  Target frames to sample per second when `do_sample_frames=True`.
- **return_tensors** (`str` or `TensorType`, *optional*) --
  Returns stacked tensors if set to `pt, otherwise returns a list of tensors.
- **data_format** (`ChannelDimension` or `str`, *optional*, defaults to `ChannelDimension.FIRST`) --
  The channel dimension format for the output video. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: video in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: video in (height, width, num_channels) format.
  - Unset: Use the channel dimension format of the input video.
- **input_data_format** (`ChannelDimension` or `str`, *optional*) --
  The channel dimension format for the input video. If unset, the channel dimension format is inferred
  from the input video. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: video in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: video in (height, width, num_channels) format.
  - `"none"` or `ChannelDimension.NONE`: video in (height, width) format.
- **device** (`torch.device`, *optional*) --
  The device to process the videos on. If unset, the device is inferred from the input videos.
- **return_metadata** (`bool`, *optional*) --
  Whether to return video metadata or not.
Constructs a base VideoProcessor.

- **video** (`"torch.Tensor"`) --
  The video to convert.`torch.Tensor`The converted video.

Converts a video to RGB format.

Convert a single or a list of urls into the corresponding `np.array` objects.

If a single url is passed, the return value will be a single object. If a list is passed a list of objects is
returned.

- **video_processor_dict** (`dict[str, Any]`) --
  Dictionary that will be used to instantiate the video processor object. Such a dictionary can be
  retrieved from a pretrained checkpoint by leveraging the
  `~video_processing_utils.VideoProcessorBase.to_dict` method.
- **kwargs** (`dict[str, Any]`) --
  Additional parameters from which to initialize the video processor object.`~video_processing_utils.VideoProcessorBase`The video processor object instantiated from those
parameters.

Instantiates a type of `~video_processing_utils.VideoProcessorBase` from a Python dictionary of parameters.

- **json_file** (`str` or `os.PathLike`) --
  Path to the JSON file containing the parameters.A video processor of type `~video_processing_utils.VideoProcessorBase`The video_processor object
instantiated from that JSON file.

Instantiates a video processor of type `~video_processing_utils.VideoProcessorBase` from the path to a JSON
file of parameters.

- **pretrained_model_name_or_path** (`str` or `os.PathLike`) --
  This can be either:

  - a string, the *model id* of a pretrained video hosted inside a model repo on
    huggingface.co.
  - a path to a *directory* containing a video processor file saved using the
    `~video_processing_utils.VideoProcessorBase.save_pretrained` method, e.g.,
    `./my_model_directory/`.
  - a path to a saved video processor JSON *file*, e.g.,
    `./my_model_directory/video_preprocessor_config.json`.
- **cache_dir** (`str` or `os.PathLike`, *optional*) --
  Path to a directory in which a downloaded pretrained model video processor should be cached if the
  standard cache should not be used.
- **force_download** (`bool`, *optional*, defaults to `False`) --
  Whether or not to force to (re-)download the video processor files and override the cached versions if
  they exist.
- **proxies** (`dict[str, str]`, *optional*) --
  A dictionary of proxy servers to use by protocol or endpoint, e.g., `{'http': 'foo.bar:3128',
  'http://hostname': 'foo.bar:4012'}.` The proxies are used on each request.
- **token** (`str` or `bool`, *optional*) --
  The token to use as HTTP bearer authorization for remote files. If `True`, or not specified, will use
  the token generated when running `hf auth login` (stored in `~/.huggingface`).
- **revision** (`str`, *optional*, defaults to `"main"`) --
  The specific model version to use. It can be a branch name, a tag name, or a commit id, since we use a
  git-based system for storing models and other artifacts on huggingface.co, so `revision` can be any
  identifier allowed by git.

  

  To test a pull request you made on the Hub, you can pass `revision="refs/pr/<pr_number>"`.

  

- **return_unused_kwargs** (`bool`, *optional*, defaults to `False`) --
  If `False`, then this function returns just the final video processor object. If `True`, then this
  functions returns a `Tuple(video_processor, unused_kwargs)` where *unused_kwargs* is a dictionary
  consisting of the key/value pairs whose keys are not video processor attributes: i.e., the part of
  `kwargs` which has not been used to update `video_processor` and is otherwise ignored.
- **subfolder** (`str`, *optional*, defaults to `""`) --
  In case the relevant files are located inside a subfolder of the model repo on huggingface.co, you can
  specify the folder name here.
- **kwargs** (`dict[str, Any]`, *optional*) --
  The values in kwargs of any keys which are video processor attributes will be used to override the
  loaded values. Behavior concerning key/value pairs whose keys are *not* video processor attributes is
  controlled by the `return_unused_kwargs` keyword parameter.A video processor of type `~video_processing_utils.ImagVideoProcessorBase`.

Instantiate a type of `~video_processing_utils.VideoProcessorBase` from an video processor.

Examples:

```python
# We can't instantiate directly the base class *VideoProcessorBase* so let's show the examples on a
# derived class: *LlavaOnevisionVideoProcessor*
video_processor = LlavaOnevisionVideoProcessor.from_pretrained(
    "llava-hf/llava-onevision-qwen2-0.5b-ov-hf"
)  # Download video_processing_config from huggingface.co and cache.
video_processor = LlavaOnevisionVideoProcessor.from_pretrained(
    "./test/saved_model/"
)  # E.g. video processor (or model) was saved using *save_pretrained('./test/saved_model/')*
video_processor = LlavaOnevisionVideoProcessor.from_pretrained("./test/saved_model/video_preprocessor_config.json")
video_processor = LlavaOnevisionVideoProcessor.from_pretrained(
    "llava-hf/llava-onevision-qwen2-0.5b-ov-hf", do_normalize=False, foo=False
)
assert video_processor.do_normalize is False
video_processor, unused_kwargs = LlavaOnevisionVideoProcessor.from_pretrained(
    "llava-hf/llava-onevision-qwen2-0.5b-ov-hf", do_normalize=False, foo=False, return_unused_kwargs=True
)
assert video_processor.do_normalize is False
assert unused_kwargs == {"foo": False}
```

- **pretrained_model_name_or_path** (`str` or `os.PathLike`) --
  The identifier of the pre-trained checkpoint from which we want the dictionary of parameters.
- **subfolder** (`str`, *optional*, defaults to `""`) --
  In case the relevant files are located inside a subfolder of the model repo on huggingface.co, you can
  specify the folder name here.`tuple[Dict, Dict]`The dictionary(ies) that will be used to instantiate the video processor object.

From a `pretrained_model_name_or_path`, resolve to a dictionary of parameters, to be used for instantiating a
video processor of type `~video_processing_utils.VideoProcessorBase` using `from_dict`.

- **do_resize** (`bool`, *optional*, defaults to `self.do_resize`) --
  Whether to resize the video's (height, width) dimensions to the specified `size`. Can be overridden by the
  `do_resize` parameter in the `preprocess` method.
- **size** (`dict`, *optional*, defaults to `self.size`) --
  Size of the output video after resizing. Can be overridden by the `size` parameter in the `preprocess`
  method.
- **size_divisor** (`int`, *optional*, defaults to `self.size_divisor`) --
  The size by which to make sure both the height and width can be divided.
- **default_to_square** (`bool`, *optional*, defaults to `self.default_to_square`) --
  Whether to default to a square video when resizing, if size is an int.
- **resample** (`PILImageResampling`, *optional*, defaults to `self.resample`) --
  Resampling filter to use if resizing the video. Only has an effect if `do_resize` is set to `True`. Can be
  overridden by the `resample` parameter in the `preprocess` method.
- **do_center_crop** (`bool`, *optional*, defaults to `self.do_center_crop`) --
  Whether to center crop the video to the specified `crop_size`. Can be overridden by `do_center_crop` in the
  `preprocess` method.
- **crop_size** (`dict[str, int]` *optional*, defaults to `self.crop_size`) --
  Size of the output video after applying `center_crop`. Can be overridden by `crop_size` in the `preprocess`
  method.
- **do_rescale** (`bool`, *optional*, defaults to `self.do_rescale`) --
  Whether to rescale the video by the specified scale `rescale_factor`. Can be overridden by the
  `do_rescale` parameter in the `preprocess` method.
- **rescale_factor** (`int` or `float`, *optional*, defaults to `self.rescale_factor`) --
  Scale factor to use if rescaling the video. Only has an effect if `do_rescale` is set to `True`. Can be
  overridden by the `rescale_factor` parameter in the `preprocess` method.
- **do_normalize** (`bool`, *optional*, defaults to `self.do_normalize`) --
  Whether to normalize the video. Can be overridden by the `do_normalize` parameter in the `preprocess`
  method. Can be overridden by the `do_normalize` parameter in the `preprocess` method.
- **image_mean** (`float` or `list[float]`, *optional*, defaults to `self.image_mean`) --
  Mean to use if normalizing the video. This is a float or list of floats the length of the number of
  channels in the video. Can be overridden by the `image_mean` parameter in the `preprocess` method. Can be
  overridden by the `image_mean` parameter in the `preprocess` method.
- **image_std** (`float` or `list[float]`, *optional*, defaults to `self.image_std`) --
  Standard deviation to use if normalizing the video. This is a float or list of floats the length of the
  number of channels in the video. Can be overridden by the `image_std` parameter in the `preprocess` method.
  Can be overridden by the `image_std` parameter in the `preprocess` method.
- **do_convert_rgb** (`bool`, *optional*, defaults to `self.image_std`) --
  Whether to convert the video to RGB.
- **video_metadata** (`VideoMetadata`, *optional*) --
  Metadata of the video containing information about total duration, fps and total number of frames.
- **do_sample_frames** (`int`, *optional*, defaults to `self.do_sample_frames`) --
  Whether to sample frames from the video before processing or to process the whole video.
- **num_frames** (`int`, *optional*, defaults to `self.num_frames`) --
  Maximum number of frames to sample when `do_sample_frames=True`.
- **fps** (`int` or `float`, *optional*, defaults to `self.fps`) --
  Target frames to sample per second when `do_sample_frames=True`.
- **return_tensors** (`str` or `TensorType`, *optional*) --
  Returns stacked tensors if set to `pt, otherwise returns a list of tensors.
- **data_format** (`ChannelDimension` or `str`, *optional*, defaults to `ChannelDimension.FIRST`) --
  The channel dimension format for the output video. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: video in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: video in (height, width, num_channels) format.
  - Unset: Use the channel dimension format of the input video.
- **input_data_format** (`ChannelDimension` or `str`, *optional*) --
  The channel dimension format for the input video. If unset, the channel dimension format is inferred
  from the input video. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: video in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: video in (height, width, num_channels) format.
  - `"none"` or `ChannelDimension.NONE`: video in (height, width) format.
- **device** (`torch.device`, *optional*) --
  The device to process the videos on. If unset, the device is inferred from the input videos.
- **return_metadata** (`bool`, *optional*) --
  Whether to return video metadata or not.

- **auto_class** (`str` or `type`, *optional*, defaults to `"AutoVideoProcessor "`) --
  The auto class to register this new video processor with.

Register this class with a given auto class. This should only be used for custom video processors as the ones
in the library are already mapped with `AutoVideoProcessor `.

This API is experimental and may have some slight breaking changes in the next releases.

- **metadata** (`VideoMetadata`) --
  Metadata of the video containing information about total duration, fps and total number of frames.
- **num_frames** (`int`, *optional*) --
  Maximum number of frames to sample. Defaults to `self.num_frames`.
- **fps** (`int` or `float`, *optional*) --
  Target frames to sample per second. Defaults to `self.fps`.np.ndarrayIndices to sample video frames.

Default sampling function which uniformly samples the desired number of frames between 0 and total number of frames.
If `fps` is passed along with metadata, `fps` frames per second are sampled uniformty. Arguments `num_frames`
and `fps` are mutually exclusive.

- **save_directory** (`str` or `os.PathLike`) --
  Directory where the video processor JSON file will be saved (will be created if it does not exist).
- **push_to_hub** (`bool`, *optional*, defaults to `False`) --
  Whether or not to push your model to the Hugging Face model hub after saving it. You can specify the
  repository you want to push to with `repo_id` (will default to the name of `save_directory` in your
  namespace).
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional key word arguments passed along to the [push_to_hub()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.utils.PushToHubMixin.push_to_hub) method.

Save an video processor object to the directory `save_directory`, so that it can be re-loaded using the
`~video_processing_utils.VideoProcessorBase.from_pretrained` class method.

`dict[str, Any]`Dictionary of all the attributes that make up this video processor instance.

Serializes this instance to a Python dictionary.
