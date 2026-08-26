# Video Processor

The `VideoProcessor` provides a unified API for video pipelines to prepare inputs for VAE encoding and post-processing outputs once they're decoded. The class inherits [VaeImageProcessor](/docs/diffusers/v0.40.0/en/api/image_processor#diffusers.VaeImageProcessor) so it includes transformations such as resizing, normalization, and conversion between PIL Image, PyTorch, and NumPy arrays.

## VideoProcessor[[diffusers.VideoProcessor.preprocess_video]]

#### diffusers.VideoProcessor.preprocess_video[[diffusers.VideoProcessor.preprocess_video]]

```python
diffusers.VideoProcessor.preprocess_video(video, height: int | None = None, width: int | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/video_processor.py#L28)

**Parameters:**

video (`list[PIL.Image]`, `list[list[PIL.Image]]`, `torch.Tensor`, `np.array`, `list[torch.Tensor]`, `list[np.array]`) : The input video. It can be one of the following: * list of the PIL images. * list of list of PIL images. * 4D Torch tensors (expected shape for each tensor `(num_frames, num_channels, height, width)`). * 4D NumPy arrays (expected shape for each array `(num_frames, height, width, num_channels)`). * list of 4D Torch tensors (expected shape for each tensor `(num_frames, num_channels, height, width)`). * list of 4D NumPy arrays (expected shape for each array `(num_frames, height, width, num_channels)`). * 5D NumPy arrays: expected shape for each array `(batch_size, num_frames, height, width, num_channels)`. * 5D Torch tensors: expected shape for each array `(batch_size, num_frames, num_channels, height, width)`.

height (`int`, *optional*, defaults to `None`) : The height in preprocessed frames of the video. If `None`, will use the `get_default_height_width()` to get default height.

width (`int`, *optional*`, defaults to `None`) : The width in preprocessed frames of the video. If `None`, will use get_default_height_width()` to get the default width.

**Returns:** `torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)`

A 5D tensor holding the batched channels-first video(s).

Preprocesses input video(s). Keyword arguments will be forwarded to `VaeImageProcessor.preprocess`.

#### diffusers.VideoProcessor.postprocess_video[[diffusers.VideoProcessor.postprocess_video]]

```python
diffusers.VideoProcessor.postprocess_video(video: Tensor, output_type: str = 'np', **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/video_processor.py#L93)

**Parameters:**

video (`torch.Tensor`) : The video as a tensor.

output_type (`str`, defaults to `"np"`) : Output type of the postprocessed `video` tensor.

Converts a video tensor to a list of frames for export. Keyword arguments will be forwarded to
`VaeImageProcessor.postprocess`.
