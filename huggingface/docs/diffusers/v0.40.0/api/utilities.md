# Utilities

Utility and helper functions for working with 🤗 Diffusers.

## numpy_to_pil[[diffusers.utils.numpy_to_pil]]

#### diffusers.utils.numpy_to_pil[[diffusers.utils.numpy_to_pil]]

```python
diffusers.utils.numpy_to_pil(images)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/pil_utils.py#L35)

Convert a numpy image or a batch of images to a PIL image.

## pt_to_pil[[diffusers.utils.pt_to_pil]]

#### diffusers.utils.pt_to_pil[[diffusers.utils.pt_to_pil]]

```python
diffusers.utils.pt_to_pil(images)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/pil_utils.py#L25)

Convert a torch image to a PIL image.

## load_image[[diffusers.utils.load_image]]

#### diffusers.utils.load_image[[diffusers.utils.load_image]]

```python
diffusers.utils.load_image(image: str | PIL.Image.Image, convert_method: typing.Optional[typing.Callable[[PIL.Image.Image], PIL.Image.Image]] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/loading_utils.py#L14)

**Parameters:**

image (`str` or `PIL.Image.Image`) : The image to convert to the PIL Image format.

convert_method (Callable[[PIL.Image.Image], PIL.Image.Image], *optional*) : A conversion method to apply to the image after loading it. When set to `None` the image will be converted "RGB".

**Returns:** `PIL.Image.Image`

A PIL Image.

Loads `image` to a PIL Image.

## load_video[[diffusers.utils.load_video]]

#### diffusers.utils.load_video[[diffusers.utils.load_video]]

```python
diffusers.utils.load_video(video: str, convert_method: typing.Optional[typing.Callable[[list[PIL.Image.Image]], list[PIL.Image.Image]]] = None, return_fps: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/loading_utils.py#L57)

**Parameters:**

video (`str`) : A URL or Path to a video to convert to a list of PIL Image format.

convert_method (Callable[[list[PIL.Image.Image]], list[PIL.Image.Image]], *optional*) : A conversion method to apply to the video after loading it. When set to `None` the images will be converted to "RGB".

return_fps (`bool`, *optional*, defaults to `False`) : Whether to also return the frame rate the video was encoded at. Needed by pipelines that resample the input to the frame rate their model works at, since a list of frames does not carry that information.

**Returns:** `list[PIL.Image.Image]` or `tuple[list[PIL.Image.Image], float]`

The video as a list of PIL images, and its frame rate if `return_fps` is set.

Loads `video` to a list of PIL Image.

## export_to_gif[[diffusers.utils.export_to_gif]]

#### diffusers.utils.export_to_gif[[diffusers.utils.export_to_gif]]

```python
diffusers.utils.export_to_gif(image: list[PIL.Image.Image], output_gif_path: str = None, fps: int = 10)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/export_utils.py#L37)

## export_to_video[[diffusers.utils.export_to_video]]

#### diffusers.utils.export_to_video[[diffusers.utils.export_to_video]]

```python
diffusers.utils.export_to_video(video_frames: list[np.ndarray] | list[PIL.Image.Image], output_video_path: str = None, fps: int = 10, quality: float = 5.0, bitrate: int | None = None, macro_block_size: int | None = 16)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/export_utils.py#L150)

quality:
Video output quality. Default is 5. Uses variable bit rate. Highest quality is 10, lowest is 0. Set to None to
prevent variable bitrate flags to FFMPEG so you can manually specify them using output_params instead.
Specifying a fixed bitrate using `bitrate` disables this parameter.

bitrate:
Set a constant bitrate for the video encoding. Default is None causing `quality` parameter to be used instead.
Better quality videos with smaller file sizes will result from using the `quality` variable bitrate parameter
rather than specifying a fixed bitrate with this parameter.

macro_block_size:
Size constraint for video. Width and height, must be divisible by this number. If not divisible by this number
imageio will tell ffmpeg to scale the image up to the next closest size divisible by this number. Most codecs
are compatible with a macroblock size of 16 (default), some can go smaller (4, 8). To disable this automatic
feature set it to None or 1, however be warned many players can't decode videos that are odd in size and some
codecs will produce poor results or fail. See https://en.wikipedia.org/wiki/Macroblock.

## encode_video[[diffusers.utils.encode_video]]

#### diffusers.utils.encode_video[[diffusers.utils.encode_video]]

```python
diffusers.utils.encode_video(video: list[PIL.Image.Image] | np.ndarray | 'torch.Tensor' | Iterator['torch.Tensor'], fps: int, output_path: str, audio: 'torch.Tensor' | None = None, audio_sample_rate: int | None = None, video_chunks_number: int = 1)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/export_utils.py#L303)

**Parameters:**

video (`List[PIL.Image.Image]` or `np.ndarray` or `torch.Tensor`) : A video tensor of shape [frames, height, width, channels] with integer pixel values in [0, 255]. If the input is a `np.ndarray`, it is expected to be a float array with values in [0, 1] (which is what pipelines usually return with `output_type="np"`).

fps (`int`) : The frames per second (FPS) of the encoded video.

output_path (`str`) : The path to save the encoded video to.

audio (`torch.Tensor`, *optional*) : An audio waveform of shape [audio_channels, samples].

audio_sample_rate : (`int`, *optional*): The sampling rate of the audio waveform.

video_chunks_number (`int`, *optional*, defaults to `1`) : The number of chunks to split the video into for encoding. Each chunk will be encoded separately. The number of chunks to use often depends on the tiling config for the video VAE.

Encodes a video with optional audio using the PyAV library. Based on code from the original LTX-2 repo:
https://github.com/Lightricks/LTX-2/blob/4f410820b198e05074a1e92de793e3b59e9ab5a0/packages/ltx-pipelines/src/ltx_pipelines/utils/media_io.py#L182

## make_image_grid[[diffusers.utils.make_image_grid]]

#### diffusers.utils.make_image_grid[[diffusers.utils.make_image_grid]]

```python
diffusers.utils.make_image_grid(images: list, rows: int, cols: int, resize: int = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/pil_utils.py#L51)

Prepares a single grid of images. Useful for visualization purposes.

## randn_tensor[[diffusers.utils.torch_utils.randn_tensor]]

#### diffusers.utils.torch_utils.randn_tensor[[diffusers.utils.torch_utils.randn_tensor]]

```python
diffusers.utils.torch_utils.randn_tensor(shape: tuple | list, generator: list['torch.Generator'] | 'torch.Generator' | None = None, device: str | 'torch.device' | None = None, dtype: 'torch.dtype' | None = None, layout: 'torch.layout' | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/torch_utils.py#L183)

A helper function to create random tensors on the desired `device` with the desired `dtype`. When
passing a list of generators, you can seed each batch size individually. If CPU generators are passed, the tensor
is always created on the CPU.

## apply_layerwise_casting[[diffusers.hooks.apply_layerwise_casting]]

#### diffusers.hooks.apply_layerwise_casting[[diffusers.hooks.apply_layerwise_casting]]

```python
diffusers.hooks.apply_layerwise_casting(module: Module, storage_dtype: dtype, compute_dtype: dtype, skip_modules_pattern: str | tuple[str, ...] = 'auto', skip_modules_classes: tuple[typing.Type[torch.nn.Module], ...] | None = None, non_blocking: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/layerwise_casting.py#L101)

**Parameters:**

module (`torch.nn.Module`) : The module whose leaf modules will be cast to a high precision dtype for computation, and to a low precision dtype for storage.

storage_dtype (`torch.dtype`) : The dtype to cast the module to before/after the forward pass for storage.

compute_dtype (`torch.dtype`) : The dtype to cast the module to during the forward pass for computation.

skip_modules_pattern (`tuple[str, ...]`, defaults to `"auto"`) : A list of patterns to match the names of the modules to skip during the layerwise casting process. If set to `"auto"`, the default patterns are used. If set to `None`, no modules are skipped. If set to `None` alongside `skip_modules_classes` being `None`, the layerwise casting is applied directly to the module instead of its internal submodules.

skip_modules_classes (`tuple[Type[torch.nn.Module], ...]`, defaults to `None`) : A list of module classes to skip during the layerwise casting process.

non_blocking (`bool`, defaults to `False`) : If `True`, the weight casting operations are non-blocking.

Applies layerwise casting to a given module. The module expected here is a Diffusers ModelMixin but it can be any
nn.Module using diffusers layers or pytorch primitives.

Example:

```python
>>> import torch
>>> from diffusers import CogVideoXTransformer3DModel

>>> transformer = CogVideoXTransformer3DModel.from_pretrained(
...     model_id, subfolder="transformer", torch_dtype=torch.bfloat16
... )

>>> apply_layerwise_casting(
...     transformer,
...     storage_dtype=torch.float8_e4m3fn,
...     compute_dtype=torch.bfloat16,
...     skip_modules_pattern=["patch_embed", "norm", "proj_out"],
...     non_blocking=True,
... )
```

## apply_group_offloading[[diffusers.hooks.apply_group_offloading]]

#### diffusers.hooks.apply_group_offloading[[diffusers.hooks.apply_group_offloading]]

```python
diffusers.hooks.apply_group_offloading(module: Module, onload_device: typing.Union[str, torch.device], offload_device: typing.Union[str, torch.device] = torch.device(), offload_type: str | diffusers.hooks.group_offloading.GroupOffloadingType = 'block_level', num_blocks_per_group: int | None = None, non_blocking: bool = False, use_stream: bool = False, record_stream: bool = False, low_cpu_mem_usage: bool = False, offload_to_disk_path: str | None = None, block_modules: list[str] | None = None, exclude_kwargs: list[str] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/group_offloading.py#L565)

**Parameters:**

module (`torch.nn.Module`) : The module to which group offloading is applied.

onload_device (`torch.device`) : The device to which the group of modules are onloaded.

offload_device (`torch.device`, defaults to `torch.device("cpu")`) : The device to which the group of modules are offloaded. This should typically be the CPU. Default is CPU.

offload_type (`str` or `GroupOffloadingType`, defaults to "block_level") : The type of offloading to be applied. Can be one of "block_level" or "leaf_level". Default is "block_level".

offload_to_disk_path (`str`, *optional*, defaults to `None`) : The path to the directory where parameters will be offloaded. Setting this option can be useful in limited RAM environment settings where a reasonable speed-memory trade-off is desired.

num_blocks_per_group (`int`, *optional*) : The number of blocks per group when using offload_type="block_level". This is required when using offload_type="block_level".

non_blocking (`bool`, defaults to `False`) : If True, offloading and onloading is done with non-blocking data transfer.

use_stream (`bool`, defaults to `False`) : If True, offloading and onloading is done asynchronously using a CUDA stream. This can be useful for overlapping computation and data transfer.

record_stream (`bool`, defaults to `False`) : When enabled with `use_stream`, it marks the current tensor as having been used by this stream. It is faster at the expense of slightly more memory usage. Refer to the [PyTorch official docs](https://pytorch.org/docs/stable/generated/torch.Tensor.record_stream.html) more details.

low_cpu_mem_usage (`bool`, defaults to `False`) : If True, the CPU memory usage is minimized by pinning tensors on-the-fly instead of pre-pinning them. This option only matters when using streamed CPU offloading (i.e. `use_stream=True`). This can be useful when the CPU memory is a bottleneck but may counteract the benefits of using streams.

block_modules (`list[str]`, *optional*) : List of module names that should be treated as blocks for offloading. If provided, only these modules will be considered for block-level offloading. If not provided, the default block detection logic will be used.

exclude_kwargs (`list[str]`, *optional*) : List of kwarg keys that should not be processed by send_to_device. This is useful for mutable state like caching lists that need to maintain their object identity across forward passes. If not provided, will be inferred from the module's `_skip_keys` attribute if it exists.

Applies group offloading to the internal layers of a torch.nn.Module. To understand what group offloading is, and
where it is beneficial, we need to first provide some context on how other supported offloading methods work.

Typically, offloading is done at two levels:
- Module-level: In Diffusers, this can be enabled using the `ModelMixin::enable_model_cpu_offload()` method. It
  works by offloading each component of a pipeline to the CPU for storage, and onloading to the accelerator device
  when needed for computation. This method is more memory-efficient than keeping all components on the accelerator,
  but the memory requirements are still quite high. For this method to work, one needs memory equivalent to size of
  the model in runtime dtype + size of largest intermediate activation tensors to be able to complete the forward
  pass.
- Leaf-level: In Diffusers, this can be enabled using the `ModelMixin::enable_sequential_cpu_offload()` method. It
  works by offloading the lowest leaf-level parameters of the computation graph to the CPU for storage, and
  onloading only the leafs to the accelerator device for computation. This uses the lowest amount of accelerator
  memory, but can be slower due to the excessive number of device synchronizations.

Group offloading is a middle ground between the two methods. It works by offloading groups of internal layers,
(either `torch.nn.ModuleList` or `torch.nn.Sequential`). This method uses lower memory than module-level
offloading. It is also faster than leaf-level/sequential offloading, as the number of device synchronizations is
reduced.

Another supported feature (for CUDA devices with support for asynchronous data transfer streams) is the ability to
overlap data transfer and computation to reduce the overall execution time compared to sequential offloading. This
is enabled using layer prefetching with streams, i.e., the layer that is to be executed next starts onloading to
the accelerator device while the current layer is being executed - this increases the memory requirements slightly.
Note that this implementation also supports leaf-level offloading but can be made much faster when using streams.

Example:
```python
>>> from diffusers import CogVideoXTransformer3DModel
>>> from diffusers.hooks import apply_group_offloading

>>> transformer = CogVideoXTransformer3DModel.from_pretrained(
...     "THUDM/CogVideoX-5b", subfolder="transformer", torch_dtype=torch.bfloat16
... )

>>> apply_group_offloading(
...     transformer,
...     onload_device=torch.device("cuda"),
...     offload_device=torch.device("cpu"),
...     offload_type="block_level",
...     num_blocks_per_group=2,
...     use_stream=True,
... )
```
