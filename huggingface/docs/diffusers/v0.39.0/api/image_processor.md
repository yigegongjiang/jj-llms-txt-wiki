# VAE Image Processor

The [VaeImageProcessor](/docs/diffusers/v0.39.0/en/api/image_processor#diffusers.VaeImageProcessor) provides a unified API for [StableDiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/text2img#diffusers.StableDiffusionPipeline)s to prepare image inputs for VAE encoding and post-processing outputs once they're decoded. This includes transformations such as resizing, normalization, and conversion between PIL Image, PyTorch, and NumPy arrays.

All pipelines with [VaeImageProcessor](/docs/diffusers/v0.39.0/en/api/image_processor#diffusers.VaeImageProcessor) accept PIL Image, PyTorch tensor, or NumPy arrays as image inputs and return outputs based on the `output_type` argument by the user. You can pass encoded image latents directly to the pipeline and return latents from the pipeline as a specific output with the `output_type` argument (for example `output_type="latent"`). This allows you to take the generated latents from one pipeline and pass it to another pipeline as input without leaving the latent space. It also makes it much easier to use multiple pipelines together by passing PyTorch tensors directly between different pipelines.

## VaeImageProcessor[[diffusers.VaeImageProcessor]]

#### diffusers.VaeImageProcessor[[diffusers.VaeImageProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L82)

Image processor for VAE.

apply_overlaydiffusers.VaeImageProcessor.apply_overlayhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L788[{"name": "mask", "val": ": Image"}, {"name": "init_image", "val": ": Image"}, {"name": "image", "val": ": Image"}, {"name": "crop_coords", "val": ": tuple[int, int, int, int] | None = None"}]- **mask** (`PIL.Image.Image`) --
  The mask image that highlights regions to overlay.
- **init_image** (`PIL.Image.Image`) --
  The original image to which the overlay is applied.
- **image** (`PIL.Image.Image`) --
  The image to overlay onto the original.
- **crop_coords** (`tuple[int, int, int, int]`, *optional*) --
  Coordinates to crop the image. If provided, the image will be cropped accordingly.0`PIL.Image.Image`The final image with the overlay applied.

Applies an overlay of the mask and the inpainted image on the original image.

**Parameters:**

do_resize (`bool`, *optional*, defaults to `True`) : Whether to downscale the image's (height, width) dimensions to multiples of `vae_scale_factor`. Can accept `height` and `width` arguments from [image_processor.VaeImageProcessor.preprocess()](/docs/diffusers/v0.39.0/en/api/image_processor#diffusers.VaeImageProcessor.preprocess) method.

vae_scale_factor (`int`, *optional*, defaults to `8`) : VAE scale factor. If `do_resize` is `True`, the image is automatically resized to multiples of this factor.

resample (`str`, *optional*, defaults to `lanczos`) : Resampling filter to use when resizing the image.

do_normalize (`bool`, *optional*, defaults to `True`) : Whether to normalize the image to [-1,1].

do_binarize (`bool`, *optional*, defaults to `False`) : Whether to binarize the image to 0/1.

do_convert_rgb (`bool`, *optional*, defaults to be `False`) : Whether to convert the images to RGB format.

do_convert_grayscale (`bool`, *optional*, defaults to be `False`) : Whether to convert the images to grayscale format.

**Returns:**

``PIL.Image.Image``

The final image with the overlay applied.
#### binarize[[diffusers.VaeImageProcessor.binarize]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L523)

Create a mask.

**Parameters:**

image (`PIL.Image.Image`) : The image input, should be a PIL image.

**Returns:**

``PIL.Image.Image``

The binarized image. Values less than 0.5 are set to 0, values greater than 0.5 are set to 1.
#### blur[[diffusers.VaeImageProcessor.blur]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L270)

Applies Gaussian blur to an image.

**Parameters:**

image (`PIL.Image.Image`) : The PIL image to convert to grayscale.

**Returns:**

``PIL.Image.Image``

The grayscale-converted PIL image.
#### convert_to_grayscale[[diffusers.VaeImageProcessor.convert_to_grayscale]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L253)

Converts a given PIL image to grayscale.

**Parameters:**

image (`PIL.Image.Image`) : The input image to convert.

**Returns:**

``PIL.Image.Image``

The image converted to grayscale.
#### convert_to_rgb[[diffusers.VaeImageProcessor.convert_to_rgb]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L236)

Converts a PIL image to RGB format.

**Parameters:**

image (`PIL.Image.Image`) : The PIL image to convert to RGB.

**Returns:**

``PIL.Image.Image``

The RGB-converted PIL image.
#### denormalize[[diffusers.VaeImageProcessor.denormalize]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L221)

Denormalize an image array to [0,1].

**Parameters:**

images (`np.ndarray` or `torch.Tensor`) : The image array to denormalize.

**Returns:**

``np.ndarray` or `torch.Tensor``

The denormalized image array.
#### get_crop_region[[diffusers.VaeImageProcessor.get_crop_region]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L287)

Finds a rectangular region that contains all masked ares in an image, and expands region to match the aspect
ratio of the original image; for example, if user drew mask in a 128x32 region, and the dimensions for
processing are 512x512, the region will be expanded to 128x128.

**Parameters:**

mask_image (PIL.Image.Image) : Mask image.

width (int) : Width of the image to be processed.

height (int) : Height of the image to be processed.

pad (int, optional) : Padding to be added to the crop region. Defaults to 0.

**Returns:**

`tuple`

(x1, y1, x2, y2) represent a rectangular region that contains all masked ares in an image and
matches the original aspect ratio.
#### get_default_height_width[[diffusers.VaeImageProcessor.get_default_height_width]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L560)

Returns the height and width of the image, downscaled to the next integer multiple of `vae_scale_factor`.

**Parameters:**

image (`PIL.Image.Image | np.ndarray | torch.Tensor`) : The image input, which can be a PIL image, NumPy array, or PyTorch tensor. If it is a NumPy array, it should have shape `[batch, height, width]` or `[batch, height, width, channels]`. If it is a PyTorch tensor, it should have shape `[batch, channels, height, width]`.

height (`int | None`, *optional*, defaults to `None`) : The height of the preprocessed image. If `None`, the height of the `image` input will be used.

width (`int | None`, *optional*, defaults to `None`) : The width of the preprocessed image. If `None`, the width of the `image` input will be used.

**Returns:**

``tuple[int, int]``

A tuple containing the height and width, both resized to the nearest integer multiple of
`vae_scale_factor`.
#### normalize[[diffusers.VaeImageProcessor.normalize]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L206)

Normalize an image array to [-1,1].

**Parameters:**

images (`np.ndarray` or `torch.Tensor`) : The image array to normalize.

**Returns:**

``np.ndarray` or `torch.Tensor``

The normalized image array.
#### numpy_to_pil[[diffusers.VaeImageProcessor.numpy_to_pil]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L127)

Convert a numpy image or a batch of images to a PIL image.

**Parameters:**

images (`np.ndarray`) : The image array to convert to PIL format.

**Returns:**

``list[PIL.Image.Image]``

A list of PIL images.
#### numpy_to_pt[[diffusers.VaeImageProcessor.numpy_to_pt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L171)

Convert a NumPy image to a PyTorch tensor.

**Parameters:**

images (`np.ndarray`) : The NumPy image array to convert to PyTorch format.

**Returns:**

``torch.Tensor``

A PyTorch tensor representation of the images.
#### pil_to_numpy[[diffusers.VaeImageProcessor.pil_to_numpy]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L151)

Convert a PIL image or a list of PIL images to NumPy arrays.

**Parameters:**

images (`PIL.Image.Image` or `list[PIL.Image.Image]`) : The PIL image or list of images to convert to NumPy format.

**Returns:**

``np.ndarray``

A NumPy array representation of the images.
#### postprocess[[diffusers.VaeImageProcessor.postprocess]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L738)

Postprocess the image output from tensor to `output_type`.

**Parameters:**

image (`torch.Tensor`) : The image input, should be a pytorch tensor with shape `B x C x H x W`.

output_type (`str`, *optional*, defaults to `pil`) : The output type of the image, can be one of `pil`, `np`, `pt`, `latent`.

do_denormalize (`list[bool]`, *optional*, defaults to `None`) : Whether to denormalize the image to [0,1]. If `None`, will use the value of `do_normalize` in the `VaeImageProcessor` config.

**Returns:**

``PIL.Image.Image`, `np.ndarray` or `torch.Tensor``

The postprocessed image.
#### preprocess[[diffusers.VaeImageProcessor.preprocess]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L607)

Preprocess the image input.

**Parameters:**

image (`PipelineImageInput`) : The image input, accepted formats are PIL images, NumPy arrays, PyTorch tensors; Also accept list of supported formats.

height (`int`, *optional*) : The height in preprocessed image. If `None`, will use the `get_default_height_width()` to get default height.

width (`int`, *optional*) : The width in preprocessed. If `None`, will use get_default_height_width()` to get the default width.

resize_mode (`str`, *optional*, defaults to `default`) : The resize mode, can be one of `default` or `fill`. If `default`, will resize the image to fit within the specified width and height, and it may not maintaining the original aspect ratio. If `fill`, will resize the image to fit within the specified width and height, maintaining the aspect ratio, and then center the image within the dimensions, filling empty with data from image. If `crop`, will resize the image to fit within the specified width and height, maintaining the aspect ratio, and then center the image within the dimensions, cropping the excess. Note that resize_mode `fill` and `crop` are only supported for PIL image input.

crops_coords (`list[tuple[int, int, int, int]]`, *optional*, defaults to `None`) : The crop coordinates for each image in the batch. If `None`, will not crop the image.

**Returns:**

``torch.Tensor``

The preprocessed image.
#### pt_to_numpy[[diffusers.VaeImageProcessor.pt_to_numpy]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L190)

Convert a PyTorch tensor to a NumPy image.

**Parameters:**

images (`torch.Tensor`) : The PyTorch tensor to convert to NumPy format.

**Returns:**

``np.ndarray``

A NumPy array representation of the images.
#### resize[[diffusers.VaeImageProcessor.resize]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L462)

Resize image.

**Parameters:**

image (`PIL.Image.Image`, `np.ndarray` or `torch.Tensor`) : The image input, can be a PIL image, numpy array or pytorch tensor.

height (`int`) : The height to resize to.

width (`int`) : The width to resize to.

resize_mode (`str`, *optional*, defaults to `default`) : The resize mode to use, can be one of `default` or `fill`. If `default`, will resize the image to fit within the specified width and height, and it may not maintaining the original aspect ratio. If `fill`, will resize the image to fit within the specified width and height, maintaining the aspect ratio, and then center the image within the dimensions, filling empty with data from image. If `crop`, will resize the image to fit within the specified width and height, maintaining the aspect ratio, and then center the image within the dimensions, cropping the excess. Note that resize_mode `fill` and `crop` are only supported for PIL image input.

**Returns:**

``PIL.Image.Image`, `np.ndarray` or `torch.Tensor``

The resized image.

## InpaintProcessor[[diffusers.InpaintProcessor]]

The [InpaintProcessor](/docs/diffusers/v0.39.0/en/api/image_processor#diffusers.InpaintProcessor) accepts `mask` and `image` inputs and process them together. Optionally, it can accept padding_mask_crop and apply mask overlay.

#### diffusers.InpaintProcessor[[diffusers.InpaintProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L836)

Image processor for inpainting image and mask.

postprocessdiffusers.InpaintProcessor.postprocesshttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L937[{"name": "image", "val": ": Tensor"}, {"name": "output_type", "val": ": str = 'pil'"}, {"name": "original_image", "val": ": PIL.Image.Image | None = None"}, {"name": "original_mask", "val": ": PIL.Image.Image | None = None"}, {"name": "crops_coords", "val": ": tuple[int, int, int, int] | None = None"}]

Postprocess the image, optionally apply mask overlay
#### preprocess[[diffusers.InpaintProcessor.preprocess]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L881)

Preprocess the image and mask.

## VaeImageProcessorLDM3D[[diffusers.VaeImageProcessorLDM3D]]

The [VaeImageProcessorLDM3D](/docs/diffusers/v0.39.0/en/api/image_processor#diffusers.VaeImageProcessorLDM3D) accepts RGB and depth inputs and returns RGB and depth outputs.

#### diffusers.VaeImageProcessorLDM3D[[diffusers.VaeImageProcessorLDM3D]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L967)

Image processor for VAE LDM3D.

depth_pil_to_numpydiffusers.VaeImageProcessorLDM3D.depth_pil_to_numpyhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1018[{"name": "images", "val": ": list[PIL.Image.Image] | PIL.Image.Image"}]- **images** (`list[PIL.Image.Image, PIL.Image.Image]`) --
  The input image or list of images to be converted.0`np.ndarray`A NumPy array of the converted images.

Convert a PIL image or a list of PIL images to NumPy arrays.

**Parameters:**

do_resize (`bool`, *optional*, defaults to `True`) : Whether to downscale the image's (height, width) dimensions to multiples of `vae_scale_factor`.

vae_scale_factor (`int`, *optional*, defaults to `8`) : VAE scale factor. If `do_resize` is `True`, the image is automatically resized to multiples of this factor.

resample (`str`, *optional*, defaults to `lanczos`) : Resampling filter to use when resizing the image.

do_normalize (`bool`, *optional*, defaults to `True`) : Whether to normalize the image to [-1,1].

**Returns:**

``np.ndarray``

A NumPy array of the converted images.
#### numpy_to_depth[[diffusers.VaeImageProcessorLDM3D.numpy_to_depth]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1076)

Convert a NumPy depth image or a batch of images to a list of PIL images.

**Parameters:**

images (`np.ndarray`) : The input NumPy array of depth images, which can be a single image or a batch.

**Returns:**

``list[PIL.Image.Image]``

A list of PIL images converted from the input NumPy depth images.
#### numpy_to_pil[[diffusers.VaeImageProcessorLDM3D.numpy_to_pil]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L994)

Convert a NumPy image or a batch of images to a list of PIL images.

**Parameters:**

images (`np.ndarray`) : The input NumPy array of images, which can be a single image or a batch.

**Returns:**

``list[PIL.Image.Image]``

A list of PIL images converted from the input NumPy array.
#### preprocess[[diffusers.VaeImageProcessorLDM3D.preprocess]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1154)

Preprocess the image input. Accepted formats are PIL images, NumPy arrays, or PyTorch tensors.

**Parameters:**

rgb (`torch.Tensor | PIL.Image.Image | np.ndarray`) : The RGB input image, which can be a single image or a batch.

depth (`torch.Tensor | PIL.Image.Image | np.ndarray`) : The depth input image, which can be a single image or a batch.

height (`int | None`, *optional*, defaults to `None`) : The desired height of the processed image. If `None`, defaults to the height of the input image.

width (`int | None`, *optional*, defaults to `None`) : The desired width of the processed image. If `None`, defaults to the width of the input image.

target_res (`int | None`, *optional*, defaults to `None`) : Target resolution for resizing the images. If specified, overrides height and width.

**Returns:**

``tuple[torch.Tensor, torch.Tensor]``

A tuple containing the processed RGB and depth images as PyTorch tensors.
#### rgblike_to_depthmap[[diffusers.VaeImageProcessorLDM3D.rgblike_to_depthmap]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1038)

Convert an RGB-like depth image to a depth map.

## PixArtImageProcessor[[diffusers.PixArtImageProcessor]]

#### diffusers.PixArtImageProcessor[[diffusers.PixArtImageProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1374)

Image processor for PixArt image resize and crop.

classify_height_width_bindiffusers.PixArtImageProcessor.classify_height_width_binhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1415[{"name": "height", "val": ": int"}, {"name": "width", "val": ": int"}, {"name": "ratios", "val": ": dict"}]- **height** (`int`) -- The height of the image.
- **width** (`int`) -- The width of the image.
- **ratios** (`dict`) -- A dictionary where keys are aspect ratios and values are tuples of (height, width).0`tuple[int, int]`The closest binned height and width.

Returns the binned height and width based on the aspect ratio.

**Parameters:**

do_resize (`bool`, *optional*, defaults to `True`) : Whether to downscale the image's (height, width) dimensions to multiples of `vae_scale_factor`. Can accept `height` and `width` arguments from [image_processor.VaeImageProcessor.preprocess()](/docs/diffusers/v0.39.0/en/api/image_processor#diffusers.VaeImageProcessor.preprocess) method.

vae_scale_factor (`int`, *optional*, defaults to `8`) : VAE scale factor. If `do_resize` is `True`, the image is automatically resized to multiples of this factor.

resample (`str`, *optional*, defaults to `lanczos`) : Resampling filter to use when resizing the image.

do_normalize (`bool`, *optional*, defaults to `True`) : Whether to normalize the image to [-1,1].

do_binarize (`bool`, *optional*, defaults to `False`) : Whether to binarize the image to 0/1.

do_convert_rgb (`bool`, *optional*, defaults to be `False`) : Whether to convert the images to RGB format.

do_convert_grayscale (`bool`, *optional*, defaults to be `False`) : Whether to convert the images to grayscale format.

**Returns:**

``tuple[int, int]``

The closest binned height and width.
#### resize_and_crop_tensor[[diffusers.PixArtImageProcessor.resize_and_crop_tensor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1433)

Resizes and crops a tensor of images to the specified dimensions.

**Parameters:**

samples (`torch.Tensor`) : A tensor of shape (N, C, H, W) where N is the batch size, C is the number of channels, H is the height, and W is the width.

new_width (`int`) : The desired width of the output images.

new_height (`int`) : The desired height of the output images.

**Returns:**

``torch.Tensor``

A tensor containing the resized and cropped images.

## IPAdapterMaskProcessor[[diffusers.IPAdapterMaskProcessor]]

#### diffusers.IPAdapterMaskProcessor[[diffusers.IPAdapterMaskProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1270)

Image processor for IP Adapter image masks.

downsamplediffusers.IPAdapterMaskProcessor.downsamplehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/image_processor.py#L1311[{"name": "mask", "val": ": Tensor"}, {"name": "batch_size", "val": ": int"}, {"name": "num_queries", "val": ": int"}, {"name": "value_embed_dim", "val": ": int"}]- **mask** (`torch.Tensor`) --
  The input mask tensor generated with `IPAdapterMaskProcessor.preprocess()`.
- **batch_size** (`int`) --
  The batch size.
- **num_queries** (`int`) --
  The number of queries.
- **value_embed_dim** (`int`) --
  The dimensionality of the value embeddings.0`torch.Tensor`The downsampled mask tensor.

Downsamples the provided mask tensor to match the expected dimensions for scaled dot-product attention. If the
aspect ratio of the mask does not match the aspect ratio of the output image, a warning is issued.

**Parameters:**

do_resize (`bool`, *optional*, defaults to `True`) : Whether to downscale the image's (height, width) dimensions to multiples of `vae_scale_factor`.

vae_scale_factor (`int`, *optional*, defaults to `8`) : VAE scale factor. If `do_resize` is `True`, the image is automatically resized to multiples of this factor.

resample (`str`, *optional*, defaults to `lanczos`) : Resampling filter to use when resizing the image.

do_normalize (`bool`, *optional*, defaults to `False`) : Whether to normalize the image to [-1,1].

do_binarize (`bool`, *optional*, defaults to `True`) : Whether to binarize the image to 0/1.

do_convert_grayscale (`bool`, *optional*, defaults to be `True`) : Whether to convert the images to grayscale format.

**Returns:**

``torch.Tensor``

The downsampled mask tensor.
