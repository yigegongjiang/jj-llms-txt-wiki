# Video Processor

A **Video Processor** is a utility responsible for preparing input features for video models, as well as handling the post-processing of their outputs. It provides transformations such as resizing, normalization, and conversion into PyTorch. Along with transformations the `VideoProcessor` class handles video decoding from local paths or URLs (requires [`torchcodec`](https://pypi.org/project/torchcodec/)) and frame sampling according to model-specific strategies.

The video processor extends the functionality of image processors by allowing Vision Large Language Models (VLMs) to handle videos with a distinct set of arguments compared to images. It serves as the bridge between raw video data and the model, ensuring that input features are optimized for the VLM.

When adding a new VLM or updating an existing one to enable distinct video preprocessing, saving and reloading the processor configuration will store the video related arguments in a dedicated file named `video_preprocessing_config.json`. Don't worry if you haven't updated your VLM, the processor will try to load video related configurations from a file named `preprocessing_config.json`.

## Usage Example

Here's an example of how to load a video processor with [`llava-hf/llava-onevision-qwen2-0.5b-ov-hf`](https://huggingface.co/llava-hf/llava-onevision-qwen2-0.5b-ov-hf) model:

```python
from transformers import AutoVideoProcessor

processor = AutoVideoProcessor.from_pretrained("llava-hf/llava-onevision-qwen2-0.5b-ov-hf")
```

Currently, if using base image processor for videos, it processes video data by treating each frame as an individual image and applying transformations frame-by-frame. While functional, this approach is not highly efficient. Using `AutoVideoProcessor` allows us to take advantage of **fast video processors**, leveraging the [torchvision](https://pytorch.org/vision/stable/index.html) library. Fast processors handle the whole batch of videos at once, without iterating over each video or frame. These updates introduce GPU acceleration and significantly enhance processing speed, especially for tasks requiring high throughput.

Fast video processors are available for all models and are loaded by default when an `AutoVideoProcessor` is initialized. When using a fast video processor, you can also set the `device` argument to specify the device on which the processing should be done. By default, the processing is done on the same device as the inputs if the inputs are tensors, or on the CPU otherwise. For even more speed improvement, we can compile the processor when using an accelerator as device.

```python
import torch
from transformers.video_utils import load_video
from transformers import AutoVideoProcessor

device = torch.accelerator.current_accelerator().type if torch.accelerator.is_available() else "cpu"

video = load_video("video.mp4")
processor = AutoVideoProcessor.from_pretrained("llava-hf/llava-onevision-qwen2-0.5b-ov-hf", device=device)
processor = torch.compile(processor)
processed_video = processor(video, return_tensors="pt")
```

## Sampling behavior

The video processor can also sample video frames using the technique best suited for the given model. Sampling behavior is controlled with the `do_sample_frames` argument and can be configured through model-specific parameters such as `num_frames` or `fps` (the rate at which the video will be sampled). If the input video is given as a local path or URL (`str`), the processor will decode it automatically. To obtain metadata about the decoded video, such as sampled frame indices, original dimensions, duration, and fps, pass `return_metadata=True` to the processor.

- Specifying `num_frames` does not guarantee the output will contain exactly that number of frames. Depending on the model, the sampler may enforce minimum or maximum frame limits.

- The default decoder is [`torchcodec`](https://pypi.org/project/torchcodec/), which must be installed.

```python
import torch
from transformers import AutoVideoProcessor

device = torch.accelerator.current_accelerator().type if torch.accelerator.is_available() else "cpu"

processor = AutoVideoProcessor.from_pretrained("llava-hf/llava-onevision-qwen2-0.5b-ov-hf", device=device)
processed_video_inputs = processor(videos=["video_path.mp4"], return_metadata=True, do_sample_frames=True, return_tensors="pt")
video_metadata = processed_video_inputs["video_metadata"]

# See how many frames the original video had and what was the original FPS
print(video_metadata.total_num_frames, video_metadata.fps)
```

If you pass an already decoded video array but still want to enable model-specific frame sampling, it is strongly recommended to provide video_metadata. This allows the sampler to know the original video’s duration and FPS. You can pass metadata as a `VideoMetadata` object or as a plain dict.

```python
import torch
from transformers import AutoVideoProcessor
from transformers.video_utils import VideoMetadata

device = torch.accelerator.current_accelerator().type if torch.accelerator.is_available() else "cpu"

processor = AutoVideoProcessor.from_pretrained("llava-hf/llava-onevision-qwen2-0.5b-ov-hf", device=device)
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

#### transformers.BaseVideoProcessor[[transformers.BaseVideoProcessor]]

```python
transformers.BaseVideoProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L78)

#### convert_to_rgb[[transformers.BaseVideoProcessor.convert_to_rgb]]

```python
convert_to_rgb(video: torch.Tensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L108)

**Parameters:**

video (`"torch.Tensor"`) : The video to convert.

**Returns:** `torch.Tensor`

The converted video.

Converts a video to RGB format.

#### fetch_videos[[transformers.BaseVideoProcessor.fetch_videos]]

```python
fetch_videos(video_url_or_urls: str | list[str] | list[list[str]], sample_indices_fn = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L746)

Convert a single or a list of urls into the corresponding `np.array` objects.

If a single url is passed, the return value will be a single object. If a list is passed a list of objects is
returned.

#### from_dict[[transformers.BaseVideoProcessor.from_dict]]

```python
from_dict(video_processor_dict: dict, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L618)

**Parameters:**

video_processor_dict (`dict[str, Any]`) : Dictionary that will be used to instantiate the video processor object. Such a dictionary can be retrieved from a pretrained checkpoint by leveraging the `~video_processing_utils.VideoProcessorBase.to_dict` method.

kwargs (`dict[str, Any]`) : Additional parameters from which to initialize the video processor object.

**Returns:** `~video_processing_utils.VideoProcessorBase`

The video processor object instantiated from those
parameters.

Instantiates a type of `~video_processing_utils.VideoProcessorBase` from a Python dictionary of parameters.

#### from_json_file[[transformers.BaseVideoProcessor.from_json_file]]

```python
from_json_file(json_file: str | os.PathLike)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L701)

**Parameters:**

json_file (`str` or `os.PathLike`) : Path to the JSON file containing the parameters.

**Returns:** A video processor of type `~video_processing_utils.VideoProcessorBase`

The video_processor object
instantiated from that JSON file.

Instantiates a video processor of type `~video_processing_utils.VideoProcessorBase` from the path to a JSON
file of parameters.

#### from_pretrained[[transformers.BaseVideoProcessor.from_pretrained]]

```python
from_pretrained(pretrained_model_name_or_path: str | os.PathLike, cache_dir: str | os.PathLike | None = None, force_download: bool = False, local_files_only: bool = False, token: str | bool | None = None, revision: str = 'main', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L345)

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`) : This can be either:  - a string, the *model id* of a pretrained video hosted inside a model repo on huggingface.co. - a path to a *directory* containing a video processor file saved using the `~video_processing_utils.VideoProcessorBase.save_pretrained` method, e.g., `./my_model_directory/`. - a path to a saved video processor JSON *file*, e.g., `./my_model_directory/video_preprocessor_config.json`.

cache_dir (`str` or `os.PathLike`, *optional*) : Path to a directory in which a downloaded pretrained model video processor should be cached if the standard cache should not be used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force to (re-)download the video processor files and override the cached versions if they exist.

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, e.g., `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}.` The proxies are used on each request.

token (`str` or `bool`, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, or not specified, will use the token generated when running `hf auth login` (stored in `~/.huggingface`).

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, or a commit id, since we use a git-based system for storing models and other artifacts on huggingface.co, so `revision` can be any identifier allowed by git.     To test a pull request you made on the Hub, you can pass `revision="refs/pr/<pr_number>"`.   

return_unused_kwargs (`bool`, *optional*, defaults to `False`) : If `False`, then this function returns just the final video processor object. If `True`, then this functions returns a `Tuple(video_processor, unused_kwargs)` where *unused_kwargs* is a dictionary consisting of the key/value pairs whose keys are not video processor attributes: i.e., the part of `kwargs` which has not been used to update `video_processor` and is otherwise ignored.

subfolder (`str`, *optional*, defaults to `""`) : In case the relevant files are located inside a subfolder of the model repo on huggingface.co, you can specify the folder name here.

kwargs (`dict[str, Any]`, *optional*) : The values in kwargs of any keys which are video processor attributes will be used to override the loaded values. Behavior concerning key/value pairs whose keys are *not* video processor attributes is controlled by the `return_unused_kwargs` keyword parameter.

**Returns:**

A video processor of type `~video_processing_utils.ImagVideoProcessorBase`.

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

#### get_video_processor_dict[[transformers.BaseVideoProcessor.get_video_processor_dict]]

```python
get_video_processor_dict(pretrained_model_name_or_path: str | os.PathLike, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L492)

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`) : The identifier of the pre-trained checkpoint from which we want the dictionary of parameters.

subfolder (`str`, *optional*, defaults to `""`) : In case the relevant files are located inside a subfolder of the model repo on huggingface.co, you can specify the folder name here.

**Returns:** `tuple[Dict, Dict]`

The dictionary(ies) that will be used to instantiate the video processor object.

From a `pretrained_model_name_or_path`, resolve to a dictionary of parameters, to be used for instantiating a
video processor of type `~video_processing_utils.VideoProcessorBase` using `from_dict`.

#### preprocess[[transformers.BaseVideoProcessor.preprocess]]

```python
preprocess(videos: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]]], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L255)

**Parameters:**

videos (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`) : Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If passing in videos with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

do_sample_frames (`bool`, *kwargs*, *optional*) : Whether to sample frames from the video before processing or to process the whole video.

video_metadata (`Annotated[~video_utils.VideoMetadata | dict | list[dict | ~video_utils.VideoMetadata] | list[list[dict | ~video_utils.VideoMetadata]] | None, None]`, *kwargs*) : Metadata of the video containing information about total duration, fps and total number of frames. It will be used to sample frames from video or compute timestamps. Don't pass any metadata unless you are trying to decode the video manually before processing

fps (`Annotated[int | float | None, None]`, *kwargs*) : Target frames to sample per second when `do_sample_frames=True`.

num_frames (`Annotated[int | None, None]`, *kwargs*) : Maximum number of frames to sample when `do_sample_frames=True`.

return_metadata (`bool`, *kwargs*, *optional*) : Whether to return video metadata or not. Video metadats is an object containing info about video duration, fps, decoding backend, etc.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### register_for_auto_class[[transformers.BaseVideoProcessor.register_for_auto_class]]

```python
register_for_auto_class(auto_class = 'AutoVideoProcessor')
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L720)

**Parameters:**

auto_class (`str` or `type`, *optional*, defaults to `"AutoVideoProcessor "`) : The auto class to register this new video processor with.

Register this class with a given auto class. This should only be used for custom video processors as the ones
in the library are already mapped with `AutoVideoProcessor `.

This API is experimental and may have some slight breaking changes in the next releases.

#### sample_frames[[transformers.BaseVideoProcessor.sample_frames]]

```python
sample_frames(metadata: VideoMetadata, num_frames: int | None = None, fps: int | float | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L133)

**Parameters:**

metadata (`VideoMetadata`) : Metadata of the video containing information about total duration, fps and total number of frames.

num_frames (`int`, *optional*) : Maximum number of frames to sample. Defaults to `self.num_frames`.

fps (`int` or `float`, *optional*) : Target frames to sample per second. Defaults to `self.fps`.

**Returns:** `np.ndarray`

Indices to sample video frames.

Default sampling function which uniformly samples the desired number of frames between 0 and total number of frames.
If `fps` is passed along with metadata, `fps` frames per second are sampled uniformly. Arguments `num_frames`
and `fps` are mutually exclusive.

#### save_pretrained[[transformers.BaseVideoProcessor.save_pretrained]]

```python
save_pretrained(save_directory: str | os.PathLike, push_to_hub: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L444)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory where the video processor JSON file will be saved (will be created if it does not exist).

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether or not to push your model to the Hugging Face model hub after saving it. You can specify the repository you want to push to with `repo_id` (will default to the name of `save_directory` in your namespace).

kwargs (`dict[str, Any]`, *optional*) : Additional key word arguments passed along to the [push_to_hub()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.utils.PushToHubMixin.push_to_hub) method.

Save an video processor object to the directory `save_directory`, so that it can be re-loaded using the
`~video_processing_utils.VideoProcessorBase.from_pretrained` class method.

#### to_dict[[transformers.BaseVideoProcessor.to_dict]]

```python
to_dict()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/video_processing_utils.py#L659)

**Returns:** `dict[str, Any]`

Dictionary of all the attributes that make up this video processor instance.

Serializes this instance to a Python dictionary.
