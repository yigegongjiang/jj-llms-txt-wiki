# Remote inference

Remote inference provides access to an [Inference Endpoint](https://huggingface.co/docs/inference-endpoints/index) to offload local generation requirements for decoding and encoding.

## remote_decode[[diffusers.utils.remote_decode]]

#### diffusers.utils.remote_decode[[diffusers.utils.remote_decode]]

```python
diffusers.utils.remote_decode(endpoint: str, tensor: 'torch.Tensor', processor: 'VaeImageProcessor' | 'VideoProcessor' | None = None, do_scaling: bool = True, scaling_factor: float | None = None, shift_factor: float | None = None, output_type: Literal['mp4', 'pil', 'pt'] = 'pil', return_type: Literal['mp4', 'pil', 'pt'] = 'pil', image_format: Literal['png', 'jpg'] = 'jpg', partial_postprocess: bool = False, input_tensor_type: Literal['binary'] = 'binary', output_tensor_type: Literal['binary'] = 'binary', height: int | None = None, width: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/remote_utils.py#L190)

**Parameters:**

endpoint (`str`) : Endpoint for Remote Decode.

tensor (`torch.Tensor`) : Tensor to be decoded.

processor (`VaeImageProcessor` or `VideoProcessor`, *optional*) : Used with `return_type="pt"`, and `return_type="pil"` for Video models.

do_scaling (`bool`, default `True`, *optional*) : **DEPRECATED**. **pass `scaling_factor`/`shift_factor` instead.** **still set do_scaling=None/do_scaling=False for no scaling until option is removed** When `True` scaling e.g. `latents / self.vae.config.scaling_factor` is applied remotely. If `False`, input must be passed with scaling applied.

scaling_factor (`float`, *optional*) : Scaling is applied when passed e.g. [`latents / self.vae.config.scaling_factor`](https://github.com/huggingface/diffusers/blob/7007febae5cff000d4df9059d9cf35133e8b2ca9/src/diffusers/pipelines/stable_diffusion/pipeline_stable_diffusion.py#L1083C37-L1083C77). - SD v1: 0.18215 - SD XL: 0.13025 - Flux: 0.3611 If `None`, input must be passed with scaling applied.

shift_factor (`float`, *optional*) : Shift is applied when passed e.g. `latents + self.vae.config.shift_factor`. - Flux: 0.1159 If `None`, input must be passed with scaling applied.

output_type (`"mp4"` or `"pil"` or `"pt", default `"pil") : **Endpoint** output type. Subject to change. Report feedback on preferred type.  `"mp4": Supported by video models. Endpoint returns `bytes` of video. `"pil"`: Supported by image and video models. Image models: Endpoint returns `bytes` of an image in `image_format`. Video models: Endpoint returns `torch.Tensor` with partial `postprocessing` applied. Requires `processor` as a flag (any `None` value will work). `"pt"`: Support by image and video models. Endpoint returns `torch.Tensor`. With `partial_postprocess=True` the tensor is postprocessed `uint8` image tensor.  Recommendations: `"pt"` with `partial_postprocess=True` is the smallest transfer for full quality. `"pt"` with `partial_postprocess=False` is the most compatible with third party code. `"pil"` with `image_format="jpg"` is the smallest transfer overall. 

return_type (`"mp4"` or `"pil"` or `"pt", default `"pil") : **Function** return type.  `"mp4": Function returns `bytes` of video. `"pil"`: Function returns `PIL.Image.Image`. With `output_type="pil" no further processing is applied. With `output_type="pt" a `PIL.Image.Image` is created. `partial_postprocess=False` `processor` is required. `partial_postprocess=True` `processor` is **not** required. `"pt"`: Function returns `torch.Tensor`. `processor` is **not** required. `partial_postprocess=False` tensor is `float16` or `bfloat16`, without denormalization. `partial_postprocess=True` tensor is `uint8`, denormalized. 

image_format (`"png"` or `"jpg"`, default `jpg`) : Used with `output_type="pil"`. Endpoint returns `jpg` or `png`. 

partial_postprocess (`bool`, default `False`) : Used with `output_type="pt"`. `partial_postprocess=False` tensor is `float16` or `bfloat16`, without denormalization. `partial_postprocess=True` tensor is `uint8`, denormalized. 

input_tensor_type (`"binary"`, default `"binary"`) : Tensor transfer type. 

output_tensor_type (`"binary"`, default `"binary"`) : Tensor transfer type. 

height (`int`, **optional**) : Required for `"packed"` latents. 

width (`int`, **optional**) : Required for `"packed"` latents.

**Returns:**

output (`Image.Image` or `list[Image.Image]` or `bytes` or `torch.Tensor`).

Hugging Face Hybrid Inference that allow running VAE decode remotely.

## remote_encode[[diffusers.utils.remote_utils.remote_encode]]

#### diffusers.utils.remote_utils.remote_encode[[diffusers.utils.remote_utils.remote_encode]]

```python
diffusers.utils.remote_utils.remote_encode(endpoint: str, image: 'torch.Tensor' | Image.Image, scaling_factor: float | None = None, shift_factor: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/remote_utils.py#L382)

**Parameters:**

endpoint (`str`) : Endpoint for Remote Decode.

image (`torch.Tensor` or `PIL.Image.Image`) : Image to be encoded.

scaling_factor (`float`, *optional*) : Scaling is applied when passed e.g. `latents * self.vae.config.scaling_factor`. - SD v1: 0.18215 - SD XL: 0.13025 - Flux: 0.3611 If `None`, input must be passed with scaling applied.

shift_factor (`float`, *optional*) : Shift is applied when passed e.g. `latents - self.vae.config.shift_factor`. - Flux: 0.1159 If `None`, input must be passed with scaling applied.

**Returns:**

output (`torch.Tensor`).

Hugging Face Hybrid Inference that allow running VAE encode remotely.
