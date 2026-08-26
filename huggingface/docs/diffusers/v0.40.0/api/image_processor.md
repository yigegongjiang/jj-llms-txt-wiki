# VAE Image Processor

The [VaeImageProcessor](/docs/diffusers/v0.40.0/en/api/image_processor#diffusers.VaeImageProcessor) provides a unified API for [StableDiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline)s to prepare image inputs for VAE encoding and post-processing outputs once they're decoded. This includes transformations such as resizing, normalization, and conversion between PIL Image, PyTorch, and NumPy arrays.

All pipelines with [VaeImageProcessor](/docs/diffusers/v0.40.0/en/api/image_processor#diffusers.VaeImageProcessor) accept PIL Image, PyTorch tensor, or NumPy arrays as image inputs and return outputs based on the `output_type` argument by the user. You can pass encoded image latents directly to the pipeline and return latents from the pipeline as a specific output with the `output_type` argument (for example `output_type="latent"`). This allows you to take the generated latents from one pipeline and pass it to another pipeline as input without leaving the latent space. It also makes it much easier to use multiple pipelines together by passing PyTorch tensors directly between different pipelines.

## VaeImageProcessor[[diffusers.VaeImageProcessor]]

#### diffusers.VaeImageProcessor[[diffusers.VaeImageProcessor]]

```python
diffusers.VaeImageProcessor(do_resize: bool = True, vae_scale_factor: int = 8, vae_latent_channels: int = 4, resample: str = 'lanczos', reducing_gap: int | None = None, do_normalize: bool = True, do_binarize: bool = False, do_convert_rgb: bool = False, do_convert_grayscale: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L82)

**Parameters:**

do_resize (`bool`, *optional*, defaults to `True`) : Whether to downscale the image's (height, width) dimensions to multiples of `vae_scale_factor`. Can accept `height` and `width` arguments from [image_processor.VaeImageProcessor.preprocess()](/docs/diffusers/v0.40.0/en/api/image_processor#diffusers.VaeImageProcessor.preprocess) method.

vae_scale_factor (`int`, *optional*, defaults to `8`) : VAE scale factor. If `do_resize` is `True`, the image is automatically resized to multiples of this factor.

resample (`str`, *optional*, defaults to `lanczos`) : Resampling filter to use when resizing the image.

do_normalize (`bool`, *optional*, defaults to `True`) : Whether to normalize the image to [-1,1].

do_binarize (`bool`, *optional*, defaults to `False`) : Whether to binarize the image to 0/1.

do_convert_rgb (`bool`, *optional*, defaults to be `False`) : Whether to convert the images to RGB format.

do_convert_grayscale (`bool`, *optional*, defaults to be `False`) : Whether to convert the images to grayscale format.

Image processor for VAE.

#### apply_overlay[[diffusers.VaeImageProcessor.apply_overlay]]

```python
apply_overlay(mask: Image, init_image: Image, image: Image, crop_coords: tuple[int, int, int, int] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L788)

**Parameters:**

mask (`PIL.Image.Image`) : The mask image that highlights regions to overlay.

init_image (`PIL.Image.Image`) : The original image to which the overlay is applied.

image (`PIL.Image.Image`) : The image to overlay onto the original.

crop_coords (`tuple[int, int, int, int]`, *optional*) : Coordinates to crop the image. If provided, the image will be cropped accordingly.

**Returns:** `PIL.Image.Image`

The final image with the overlay applied.

Applies an overlay of the mask and the inpainted image on the original image.

#### binarize[[diffusers.VaeImageProcessor.binarize]]

```python
binarize(image: Image)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L523)

**Parameters:**

image (`PIL.Image.Image`) : The image input, should be a PIL image.

**Returns:** `PIL.Image.Image`

The binarized image. Values less than 0.5 are set to 0, values greater than 0.5 are set to 1.

Create a mask.

#### blur[[diffusers.VaeImageProcessor.blur]]

```python
blur(image: Image, blur_factor: int = 4)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L270)

**Parameters:**

image (`PIL.Image.Image`) : The PIL image to convert to grayscale.

**Returns:** `PIL.Image.Image`

The grayscale-converted PIL image.

Applies Gaussian blur to an image.

#### convert_to_grayscale[[diffusers.VaeImageProcessor.convert_to_grayscale]]

```python
convert_to_grayscale(image: Image)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L253)

**Parameters:**

image (`PIL.Image.Image`) : The input image to convert.

**Returns:** `PIL.Image.Image`

The image converted to grayscale.

Converts a given PIL image to grayscale.

#### convert_to_rgb[[diffusers.VaeImageProcessor.convert_to_rgb]]

```python
convert_to_rgb(image: Image)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L236)

**Parameters:**

image (`PIL.Image.Image`) : The PIL image to convert to RGB.

**Returns:** `PIL.Image.Image`

The RGB-converted PIL image.

Converts a PIL image to RGB format.

#### denormalize[[diffusers.VaeImageProcessor.denormalize]]

```python
denormalize(images: typing.Union[numpy.ndarray, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L221)

**Parameters:**

images (`np.ndarray` or `torch.Tensor`) : The image array to denormalize.

**Returns:** `np.ndarray` or `torch.Tensor`

The denormalized image array.

Denormalize an image array to [0,1].

#### get_crop_region[[diffusers.VaeImageProcessor.get_crop_region]]

```python
get_crop_region(mask_image: Image, width: int, height: int, pad = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L287)

**Parameters:**

mask_image (PIL.Image.Image) : Mask image.

width (int) : Width of the image to be processed.

height (int) : Height of the image to be processed.

pad (int, optional) : Padding to be added to the crop region. Defaults to 0.

**Returns:** `tuple`

(x1, y1, x2, y2) represent a rectangular region that contains all masked ares in an image and
matches the original aspect ratio.

Finds a rectangular region that contains all masked ares in an image, and expands region to match the aspect
ratio of the original image; for example, if user drew mask in a 128x32 region, and the dimensions for
processing are 512x512, the region will be expanded to 128x128.

#### get_default_height_width[[diffusers.VaeImageProcessor.get_default_height_width]]

```python
get_default_height_width(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor], height: int | None = None, width: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L560)

**Parameters:**

image (`PIL.Image.Image | np.ndarray | torch.Tensor`) : The image input, which can be a PIL image, NumPy array, or PyTorch tensor. If it is a NumPy array, it should have shape `[batch, height, width]` or `[batch, height, width, channels]`. If it is a PyTorch tensor, it should have shape `[batch, channels, height, width]`.

height (`int | None`, *optional*, defaults to `None`) : The height of the preprocessed image. If `None`, the height of the `image` input will be used.

width (`int | None`, *optional*, defaults to `None`) : The width of the preprocessed image. If `None`, the width of the `image` input will be used.

**Returns:** `tuple[int, int]`

A tuple containing the height and width, both resized to the nearest integer multiple of
`vae_scale_factor`.

Returns the height and width of the image, downscaled to the next integer multiple of `vae_scale_factor`.

#### normalize[[diffusers.VaeImageProcessor.normalize]]

```python
normalize(images: typing.Union[numpy.ndarray, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L206)

**Parameters:**

images (`np.ndarray` or `torch.Tensor`) : The image array to normalize.

**Returns:** `np.ndarray` or `torch.Tensor`

The normalized image array.

Normalize an image array to [-1,1].

#### numpy_to_pil[[diffusers.VaeImageProcessor.numpy_to_pil]]

```python
numpy_to_pil(images: ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L127)

**Parameters:**

images (`np.ndarray`) : The image array to convert to PIL format.

**Returns:** `list[PIL.Image.Image]`

A list of PIL images.

Convert a numpy image or a batch of images to a PIL image.

#### numpy_to_pt[[diffusers.VaeImageProcessor.numpy_to_pt]]

```python
numpy_to_pt(images: ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L171)

**Parameters:**

images (`np.ndarray`) : The NumPy image array to convert to PyTorch format.

**Returns:** `torch.Tensor`

A PyTorch tensor representation of the images.

Convert a NumPy image to a PyTorch tensor.

#### pil_to_numpy[[diffusers.VaeImageProcessor.pil_to_numpy]]

```python
pil_to_numpy(images: list[PIL.Image.Image] | PIL.Image.Image)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L151)

**Parameters:**

images (`PIL.Image.Image` or `list[PIL.Image.Image]`) : The PIL image or list of images to convert to NumPy format.

**Returns:** `np.ndarray`

A NumPy array representation of the images.

Convert a PIL image or a list of PIL images to NumPy arrays.

#### postprocess[[diffusers.VaeImageProcessor.postprocess]]

```python
postprocess(image: Tensor, output_type: str = 'pil', do_denormalize: list[bool] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L738)

**Parameters:**

image (`torch.Tensor`) : The image input, should be a pytorch tensor with shape `B x C x H x W`.

output_type (`str`, *optional*, defaults to `pil`) : The output type of the image, can be one of `pil`, `np`, `pt`, `latent`.

do_denormalize (`list[bool]`, *optional*, defaults to `None`) : Whether to denormalize the image to [0,1]. If `None`, will use the value of `do_normalize` in the `VaeImageProcessor` config.

**Returns:** `PIL.Image.Image`, `np.ndarray` or `torch.Tensor`

The postprocessed image.

Postprocess the image output from tensor to `output_type`.

#### preprocess[[diffusers.VaeImageProcessor.preprocess]]

```python
preprocess(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]], height: int | None = None, width: int | None = None, resize_mode: str = 'default', crops_coords: tuple[int, int, int, int] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L607)

**Parameters:**

image (`PipelineImageInput`) : The image input, accepted formats are PIL images, NumPy arrays, PyTorch tensors; Also accept list of supported formats.

height (`int`, *optional*) : The height in preprocessed image. If `None`, will use the `get_default_height_width()` to get default height.

width (`int`, *optional*) : The width in preprocessed. If `None`, will use get_default_height_width()` to get the default width.

resize_mode (`str`, *optional*, defaults to `default`) : The resize mode, can be one of `default` or `fill`. If `default`, will resize the image to fit within the specified width and height, and it may not maintaining the original aspect ratio. If `fill`, will resize the image to fit within the specified width and height, maintaining the aspect ratio, and then center the image within the dimensions, filling empty with data from image. If `crop`, will resize the image to fit within the specified width and height, maintaining the aspect ratio, and then center the image within the dimensions, cropping the excess. Note that resize_mode `fill` and `crop` are only supported for PIL image input.

crops_coords (`list[tuple[int, int, int, int]]`, *optional*, defaults to `None`) : The crop coordinates for each image in the batch. If `None`, will not crop the image.

**Returns:** `torch.Tensor`

The preprocessed image.

Preprocess the image input.

#### pt_to_numpy[[diffusers.VaeImageProcessor.pt_to_numpy]]

```python
pt_to_numpy(images: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L190)

**Parameters:**

images (`torch.Tensor`) : The PyTorch tensor to convert to NumPy format.

**Returns:** `np.ndarray`

A NumPy array representation of the images.

Convert a PyTorch tensor to a NumPy image.

#### resize[[diffusers.VaeImageProcessor.resize]]

```python
resize(image: typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor], height: int, width: int, resize_mode: str = 'default')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L462)

**Parameters:**

image (`PIL.Image.Image`, `np.ndarray` or `torch.Tensor`) : The image input, can be a PIL image, numpy array or pytorch tensor.

height (`int`) : The height to resize to.

width (`int`) : The width to resize to.

resize_mode (`str`, *optional*, defaults to `default`) : The resize mode to use, can be one of `default` or `fill`. If `default`, will resize the image to fit within the specified width and height, and it may not maintaining the original aspect ratio. If `fill`, will resize the image to fit within the specified width and height, maintaining the aspect ratio, and then center the image within the dimensions, filling empty with data from image. If `crop`, will resize the image to fit within the specified width and height, maintaining the aspect ratio, and then center the image within the dimensions, cropping the excess. Note that resize_mode `fill` and `crop` are only supported for PIL image input.

**Returns:** `PIL.Image.Image`, `np.ndarray` or `torch.Tensor`

The resized image.

Resize image.

## InpaintProcessor[[diffusers.InpaintProcessor]]

The [InpaintProcessor](/docs/diffusers/v0.40.0/en/api/image_processor#diffusers.InpaintProcessor) accepts `mask` and `image` inputs and process them together. Optionally, it can accept padding_mask_crop and apply mask overlay.

#### diffusers.InpaintProcessor[[diffusers.InpaintProcessor]]

```python
diffusers.InpaintProcessor(do_resize: bool = True, vae_scale_factor: int = 8, vae_latent_channels: int = 4, resample: str = 'lanczos', reducing_gap: int | None = None, do_normalize: bool = True, do_binarize: bool = False, do_convert_grayscale: bool = False, mask_do_normalize: bool = False, mask_do_binarize: bool = True, mask_do_convert_grayscale: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L836)

Image processor for inpainting image and mask.

#### postprocess[[diffusers.InpaintProcessor.postprocess]]

```python
postprocess(image: Tensor, output_type: str = 'pil', original_image: PIL.Image.Image | None = None, original_mask: PIL.Image.Image | None = None, crops_coords: tuple[int, int, int, int] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L937)

Postprocess the image, optionally apply mask overlay

#### preprocess[[diffusers.InpaintProcessor.preprocess]]

```python
preprocess(image: Image, mask: PIL.Image.Image | None = None, height: int | None = None, width: int | None = None, padding_mask_crop: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L881)

Preprocess the image and mask.

## VaeImageProcessorLDM3D[[diffusers.VaeImageProcessorLDM3D]]

The [VaeImageProcessorLDM3D](/docs/diffusers/v0.40.0/en/api/image_processor#diffusers.VaeImageProcessorLDM3D) accepts RGB and depth inputs and returns RGB and depth outputs.

#### diffusers.VaeImageProcessorLDM3D[[diffusers.VaeImageProcessorLDM3D]]

```python
diffusers.VaeImageProcessorLDM3D(do_resize: bool = True, vae_scale_factor: int = 8, resample: str = 'lanczos', do_normalize: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L967)

**Parameters:**

do_resize (`bool`, *optional*, defaults to `True`) : Whether to downscale the image's (height, width) dimensions to multiples of `vae_scale_factor`.

vae_scale_factor (`int`, *optional*, defaults to `8`) : VAE scale factor. If `do_resize` is `True`, the image is automatically resized to multiples of this factor.

resample (`str`, *optional*, defaults to `lanczos`) : Resampling filter to use when resizing the image.

do_normalize (`bool`, *optional*, defaults to `True`) : Whether to normalize the image to [-1,1].

Image processor for VAE LDM3D.

#### depth_pil_to_numpy[[diffusers.VaeImageProcessorLDM3D.depth_pil_to_numpy]]

```python
depth_pil_to_numpy(images: list[PIL.Image.Image] | PIL.Image.Image)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L1018)

**Parameters:**

images (`list[PIL.Image.Image, PIL.Image.Image]`) : The input image or list of images to be converted.

**Returns:** `np.ndarray`

A NumPy array of the converted images.

Convert a PIL image or a list of PIL images to NumPy arrays.

#### numpy_to_depth[[diffusers.VaeImageProcessorLDM3D.numpy_to_depth]]

```python
numpy_to_depth(images: ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L1076)

**Parameters:**

images (`np.ndarray`) : The input NumPy array of depth images, which can be a single image or a batch.

**Returns:** `list[PIL.Image.Image]`

A list of PIL images converted from the input NumPy depth images.

Convert a NumPy depth image or a batch of images to a list of PIL images.

#### numpy_to_pil[[diffusers.VaeImageProcessorLDM3D.numpy_to_pil]]

```python
numpy_to_pil(images: ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L994)

**Parameters:**

images (`np.ndarray`) : The input NumPy array of images, which can be a single image or a batch.

**Returns:** `list[PIL.Image.Image]`

A list of PIL images converted from the input NumPy array.

Convert a NumPy image or a batch of images to a list of PIL images.

#### preprocess[[diffusers.VaeImageProcessorLDM3D.preprocess]]

```python
preprocess(rgb: typing.Union[torch.Tensor, PIL.Image.Image, numpy.ndarray], depth: typing.Union[torch.Tensor, PIL.Image.Image, numpy.ndarray], height: int | None = None, width: int | None = None, target_res: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L1154)

**Parameters:**

rgb (`torch.Tensor | PIL.Image.Image | np.ndarray`) : The RGB input image, which can be a single image or a batch.

depth (`torch.Tensor | PIL.Image.Image | np.ndarray`) : The depth input image, which can be a single image or a batch.

height (`int | None`, *optional*, defaults to `None`) : The desired height of the processed image. If `None`, defaults to the height of the input image.

width (`int | None`, *optional*, defaults to `None`) : The desired width of the processed image. If `None`, defaults to the width of the input image.

target_res (`int | None`, *optional*, defaults to `None`) : Target resolution for resizing the images. If specified, overrides height and width.

**Returns:** `tuple[torch.Tensor, torch.Tensor]`

A tuple containing the processed RGB and depth images as PyTorch tensors.

Preprocess the image input. Accepted formats are PIL images, NumPy arrays, or PyTorch tensors.

#### rgblike_to_depthmap[[diffusers.VaeImageProcessorLDM3D.rgblike_to_depthmap]]

```python
rgblike_to_depthmap(image: typing.Union[numpy.ndarray, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L1038)

Convert an RGB-like depth image to a depth map.

## PixArtImageProcessor[[diffusers.PixArtImageProcessor]]

#### diffusers.PixArtImageProcessor[[diffusers.PixArtImageProcessor]]

```python
diffusers.PixArtImageProcessor(do_resize: bool = True, vae_scale_factor: int = 8, resample: str = 'lanczos', do_normalize: bool = True, do_binarize: bool = False, do_convert_grayscale: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L1374)

**Parameters:**

do_resize (`bool`, *optional*, defaults to `True`) : Whether to downscale the image's (height, width) dimensions to multiples of `vae_scale_factor`. Can accept `height` and `width` arguments from [image_processor.VaeImageProcessor.preprocess()](/docs/diffusers/v0.40.0/en/api/image_processor#diffusers.VaeImageProcessor.preprocess) method.

vae_scale_factor (`int`, *optional*, defaults to `8`) : VAE scale factor. If `do_resize` is `True`, the image is automatically resized to multiples of this factor.

resample (`str`, *optional*, defaults to `lanczos`) : Resampling filter to use when resizing the image.

do_normalize (`bool`, *optional*, defaults to `True`) : Whether to normalize the image to [-1,1].

do_binarize (`bool`, *optional*, defaults to `False`) : Whether to binarize the image to 0/1.

do_convert_rgb (`bool`, *optional*, defaults to be `False`) : Whether to convert the images to RGB format.

do_convert_grayscale (`bool`, *optional*, defaults to be `False`) : Whether to convert the images to grayscale format.

Image processor for PixArt image resize and crop.

#### classify_height_width_bin[[diffusers.PixArtImageProcessor.classify_height_width_bin]]

```python
classify_height_width_bin(height: int, width: int, ratios: dict)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L1415)

**Parameters:**

height (`int`) : The height of the image.

width (`int`) : The width of the image.

ratios (`dict`) : A dictionary where keys are aspect ratios and values are tuples of (height, width).

**Returns:** `tuple[int, int]`

The closest binned height and width.

Returns the binned height and width based on the aspect ratio.

#### resize_and_crop_tensor[[diffusers.PixArtImageProcessor.resize_and_crop_tensor]]

```python
resize_and_crop_tensor(samples: Tensor, new_width: int, new_height: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L1433)

**Parameters:**

samples (`torch.Tensor`) : A tensor of shape (N, C, H, W) where N is the batch size, C is the number of channels, H is the height, and W is the width.

new_width (`int`) : The desired width of the output images.

new_height (`int`) : The desired height of the output images.

**Returns:** `torch.Tensor`

A tensor containing the resized and cropped images.

Resizes and crops a tensor of images to the specified dimensions.

## IPAdapterMaskProcessor[[diffusers.IPAdapterMaskProcessor]]

#### diffusers.IPAdapterMaskProcessor[[diffusers.IPAdapterMaskProcessor]]

```python
diffusers.IPAdapterMaskProcessor(do_resize: bool = True, vae_scale_factor: int = 8, resample: str = 'lanczos', do_normalize: bool = False, do_binarize: bool = True, do_convert_grayscale: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L1270)

**Parameters:**

do_resize (`bool`, *optional*, defaults to `True`) : Whether to downscale the image's (height, width) dimensions to multiples of `vae_scale_factor`.

vae_scale_factor (`int`, *optional*, defaults to `8`) : VAE scale factor. If `do_resize` is `True`, the image is automatically resized to multiples of this factor.

resample (`str`, *optional*, defaults to `lanczos`) : Resampling filter to use when resizing the image.

do_normalize (`bool`, *optional*, defaults to `False`) : Whether to normalize the image to [-1,1].

do_binarize (`bool`, *optional*, defaults to `True`) : Whether to binarize the image to 0/1.

do_convert_grayscale (`bool`, *optional*, defaults to be `True`) : Whether to convert the images to grayscale format.

Image processor for IP Adapter image masks.

#### downsample[[diffusers.IPAdapterMaskProcessor.downsample]]

```python
downsample(mask: Tensor, batch_size: int, num_queries: int, value_embed_dim: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/image_processor.py#L1311)

**Parameters:**

mask (`torch.Tensor`) : The input mask tensor generated with `IPAdapterMaskProcessor.preprocess()`.

batch_size (`int`) : The batch size.

num_queries (`int`) : The number of queries.

value_embed_dim (`int`) : The dimensionality of the value embeddings.

**Returns:** `torch.Tensor`

The downsampled mask tensor.

Downsamples the provided mask tensor to match the expected dimensions for scaled dot-product attention. If the
aspect ratio of the mask does not match the aspect ratio of the output image, a warning is issued.
