# Caching methods

Cache methods speedup diffusion transformers by storing and reusing intermediate outputs of specific layers, such as attention and feedforward layers, instead of recalculating them at each inference step.

## CacheMixin[[diffusers.CacheMixin]]

#### diffusers.CacheMixin[[diffusers.CacheMixin]]

```python
diffusers.CacheMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/cache_utils.py#L23)

A class for enable/disabling caching techniques on diffusion models.

Supported caching techniques:
- [Pyramid Attention Broadcast](https://huggingface.co/papers/2408.12588)
- [FasterCache](https://huggingface.co/papers/2410.19355)
- [FirstBlockCache](https://github.com/chengzeyi/ParaAttention/blob/7a266123671b55e7e5a2fe9af3121f07a36afc78/README.md#first-block-cache-our-dynamic-caching)

#### cache_context[[diffusers.CacheMixin.cache_context]]

```python
cache_context(name: str)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/cache_utils.py#L154)

Context manager that provides additional methods for cache management.

#### enable_cache[[diffusers.CacheMixin.enable_cache]]

```python
enable_cache(config)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/cache_utils.py#L39)

**Parameters:**

config (`PyramidAttentionBroadcastConfig | FasterCacheConfig | FirstBlockCacheConfig | TextKVCacheConfig`) : The configuration for applying the caching technique. Currently supported caching techniques are: - [PyramidAttentionBroadcastConfig](/docs/diffusers/v0.40.0/en/api/cache#diffusers.PyramidAttentionBroadcastConfig) - [FasterCacheConfig](/docs/diffusers/v0.40.0/en/api/cache#diffusers.FasterCacheConfig) - [FirstBlockCacheConfig](/docs/diffusers/v0.40.0/en/api/cache#diffusers.FirstBlockCacheConfig) - `TextKVCacheConfig`

Enable caching techniques on the model.

Example:

```python
>>> import torch
>>> from diffusers import CogVideoXPipeline, PyramidAttentionBroadcastConfig

>>> pipe = CogVideoXPipeline.from_pretrained("THUDM/CogVideoX-5b", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> config = PyramidAttentionBroadcastConfig(
...     spatial_attention_block_skip_range=2,
...     spatial_attention_timestep_skip_range=(100, 800),
...     current_timestep_callback=lambda: pipe.current_timestep,
... )
>>> pipe.transformer.enable_cache(config)
```

## PyramidAttentionBroadcastConfig[[diffusers.PyramidAttentionBroadcastConfig]]

#### diffusers.PyramidAttentionBroadcastConfig[[diffusers.PyramidAttentionBroadcastConfig]]

```python
diffusers.PyramidAttentionBroadcastConfig(spatial_attention_block_skip_range: int | None = None, temporal_attention_block_skip_range: int | None = None, cross_attention_block_skip_range: int | None = None, spatial_attention_timestep_skip_range: tuple = (100, 800), temporal_attention_timestep_skip_range: tuple = (100, 800), cross_attention_timestep_skip_range: tuple = (100, 800), spatial_attention_block_identifiers: tuple = ('blocks', 'transformer_blocks', 'single_transformer_blocks', 'layers', 'visual_transformer_blocks'), temporal_attention_block_identifiers: tuple = ('temporal_transformer_blocks',), cross_attention_block_identifiers: tuple = ('blocks', 'transformer_blocks', 'layers'), current_timestep_callback: typing.Callable[[], int] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/pyramid_attention_broadcast.py#L40)

**Parameters:**

spatial_attention_block_skip_range (`int`, *optional*, defaults to `None`) : The number of times a specific spatial attention broadcast is skipped before computing the attention states to re-use. If this is set to the value `N`, the attention computation will be skipped `N - 1` times (i.e., old attention states will be reused) before computing the new attention states again.

temporal_attention_block_skip_range (`int`, *optional*, defaults to `None`) : The number of times a specific temporal attention broadcast is skipped before computing the attention states to re-use. If this is set to the value `N`, the attention computation will be skipped `N - 1` times (i.e., old attention states will be reused) before computing the new attention states again.

cross_attention_block_skip_range (`int`, *optional*, defaults to `None`) : The number of times a specific cross-attention broadcast is skipped before computing the attention states to re-use. If this is set to the value `N`, the attention computation will be skipped `N - 1` times (i.e., old attention states will be reused) before computing the new attention states again.

spatial_attention_timestep_skip_range (`tuple[int, int]`, defaults to `(100, 800)`) : The range of timesteps to skip in the spatial attention layer. The attention computations will be conditionally skipped if the current timestep is within the specified range.

temporal_attention_timestep_skip_range (`tuple[int, int]`, defaults to `(100, 800)`) : The range of timesteps to skip in the temporal attention layer. The attention computations will be conditionally skipped if the current timestep is within the specified range.

cross_attention_timestep_skip_range (`tuple[int, int]`, defaults to `(100, 800)`) : The range of timesteps to skip in the cross-attention layer. The attention computations will be conditionally skipped if the current timestep is within the specified range.

spatial_attention_block_identifiers (`tuple[str, ...]`) : The identifiers to match against the layer names to determine if the layer is a spatial attention layer.

temporal_attention_block_identifiers (`tuple[str, ...]`) : The identifiers to match against the layer names to determine if the layer is a temporal attention layer.

cross_attention_block_identifiers (`tuple[str, ...]`) : The identifiers to match against the layer names to determine if the layer is a cross-attention layer.

Configuration for Pyramid Attention Broadcast.

#### diffusers.apply_pyramid_attention_broadcast[[diffusers.apply_pyramid_attention_broadcast]]

```python
diffusers.apply_pyramid_attention_broadcast(module: Module, config: PyramidAttentionBroadcastConfig)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/pyramid_attention_broadcast.py#L181)

**Parameters:**

module (`torch.nn.Module`) : The module to apply Pyramid Attention Broadcast to.

config (`PyramidAttentionBroadcastConfig | None`, `optional`, defaults to `None`) : The configuration to use for Pyramid Attention Broadcast.

Apply [Pyramid Attention Broadcast](https://huggingface.co/papers/2408.12588) to a given pipeline.

PAB is an attention approximation method that leverages the similarity in attention states between timesteps to
reduce the computational cost of attention computation. The key takeaway from the paper is that the attention
similarity in the cross-attention layers between timesteps is high, followed by less similarity in the temporal and
spatial layers. This allows for the skipping of attention computation in the cross-attention layers more frequently
than in the temporal and spatial layers. Applying PAB will, therefore, speedup the inference process.

Example:

```python
>>> import torch
>>> from diffusers import CogVideoXPipeline, PyramidAttentionBroadcastConfig, apply_pyramid_attention_broadcast
>>> from diffusers.utils import export_to_video

>>> pipe = CogVideoXPipeline.from_pretrained("THUDM/CogVideoX-5b", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> config = PyramidAttentionBroadcastConfig(
...     spatial_attention_block_skip_range=2,
...     spatial_attention_timestep_skip_range=(100, 800),
...     current_timestep_callback=lambda: pipe.current_timestep,
... )
>>> apply_pyramid_attention_broadcast(pipe.transformer, config)
```

## FasterCacheConfig[[diffusers.FasterCacheConfig]]

#### diffusers.FasterCacheConfig[[diffusers.FasterCacheConfig]]

```python
diffusers.FasterCacheConfig(spatial_attention_block_skip_range: int = 2, temporal_attention_block_skip_range: int | None = None, spatial_attention_timestep_skip_range: tuple = (-1, 681), temporal_attention_timestep_skip_range: tuple = (-1, 681), low_frequency_weight_update_timestep_range: tuple = (99, 901), high_frequency_weight_update_timestep_range: tuple = (-1, 301), alpha_low_frequency: float = 1.1, alpha_high_frequency: float = 1.1, unconditional_batch_skip_range: int = 5, unconditional_batch_timestep_skip_range: tuple = (-1, 641), spatial_attention_block_identifiers: tuple = ('^blocks.*attn', '^transformer_blocks.*attn', '^single_transformer_blocks.*attn'), temporal_attention_block_identifiers: tuple = ('^temporal_transformer_blocks.*attn',), attention_weight_callback: typing.Callable[[torch.nn.Module], float] = None, low_frequency_weight_callback: typing.Callable[[torch.nn.Module], float] = None, high_frequency_weight_callback: typing.Callable[[torch.nn.Module], float] = None, tensor_format: str = 'BCFHW', is_guidance_distilled: bool = False, current_timestep_callback: typing.Callable[[], int] = None, _unconditional_conditional_input_kwargs_identifiers: list = ('hidden_states', 'encoder_hidden_states', 'timestep', 'attention_mask', 'encoder_attention_mask'))
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/faster_cache.py#L50)

**Parameters:**

spatial_attention_block_skip_range (`int`, defaults to `2`) : Calculate the attention states every `N` iterations. If this is set to `N`, the attention computation will be skipped `N - 1` times (i.e., cached attention states will be reused) before computing the new attention states again.

temporal_attention_block_skip_range (`int`, *optional*, defaults to `None`) : Calculate the attention states every `N` iterations. If this is set to `N`, the attention computation will be skipped `N - 1` times (i.e., cached attention states will be reused) before computing the new attention states again.

spatial_attention_timestep_skip_range (`tuple[float, float]`, defaults to `(-1, 681)`) : The timestep range within which the spatial attention computation can be skipped without a significant loss in quality. This is to be determined by the user based on the underlying model. The first value in the tuple is the lower bound and the second value is the upper bound. Typically, diffusion timesteps for denoising are in the reversed range of 0 to 1000 (i.e. denoising starts at timestep 1000 and ends at timestep 0). For the default values, this would mean that the spatial attention computation skipping will be applicable only after denoising timestep 681 is reached, and continue until the end of the denoising process.

temporal_attention_timestep_skip_range (`tuple[float, float]`, *optional*, defaults to `None`) : The timestep range within which the temporal attention computation can be skipped without a significant loss in quality. This is to be determined by the user based on the underlying model. The first value in the tuple is the lower bound and the second value is the upper bound. Typically, diffusion timesteps for denoising are in the reversed range of 0 to 1000 (i.e. denoising starts at timestep 1000 and ends at timestep 0).

low_frequency_weight_update_timestep_range (`tuple[int, int]`, defaults to `(99, 901)`) : The timestep range within which the low frequency weight scaling update is applied. The first value in the tuple is the lower bound and the second value is the upper bound of the timestep range. The callback function for the update is called only within this range.

high_frequency_weight_update_timestep_range (`tuple[int, int]`, defaults to `(-1, 301)`) : The timestep range within which the high frequency weight scaling update is applied. The first value in the tuple is the lower bound and the second value is the upper bound of the timestep range. The callback function for the update is called only within this range.

alpha_low_frequency (`float`, defaults to `1.1`) : The weight to scale the low frequency updates by. This is used to approximate the unconditional branch from the conditional branch outputs.

alpha_high_frequency (`float`, defaults to `1.1`) : The weight to scale the high frequency updates by. This is used to approximate the unconditional branch from the conditional branch outputs.

unconditional_batch_skip_range (`int`, defaults to `5`) : Process the unconditional branch every `N` iterations. If this is set to `N`, the unconditional branch computation will be skipped `N - 1` times (i.e., cached unconditional branch states will be reused) before computing the new unconditional branch states again.

unconditional_batch_timestep_skip_range (`tuple[float, float]`, defaults to `(-1, 641)`) : The timestep range within which the unconditional branch computation can be skipped without a significant loss in quality. This is to be determined by the user based on the underlying model. The first value in the tuple is the lower bound and the second value is the upper bound.

spatial_attention_block_identifiers (`tuple[str, ...]`, defaults to `("blocks.*attn1", "transformer_blocks.*attn1", "single_transformer_blocks.*attn1")`) : The identifiers to match the spatial attention blocks in the model. If the name of the block contains any of these identifiers, FasterCache will be applied to that block. This can either be the full layer names, partial layer names, or regex patterns. Matching will always be done using a regex match.

temporal_attention_block_identifiers (`tuple[str, ...]`, defaults to `("temporal_transformer_blocks.*attn1",)`) : The identifiers to match the temporal attention blocks in the model. If the name of the block contains any of these identifiers, FasterCache will be applied to that block. This can either be the full layer names, partial layer names, or regex patterns. Matching will always be done using a regex match.

attention_weight_callback (`Callable[[torch.nn.Module], float]`, defaults to `None`) : The callback function to determine the weight to scale the attention outputs by. This function should take the attention module as input and return a float value. This is used to approximate the unconditional branch from the conditional branch outputs. If not provided, the default weight is 0.5 for all timesteps. Typically, as described in the paper, this weight should gradually increase from 0 to 1 as the inference progresses. Users are encouraged to experiment and provide custom weight schedules that take into account the number of inference steps and underlying model behaviour as denoising progresses.

low_frequency_weight_callback (`Callable[[torch.nn.Module], float]`, defaults to `None`) : The callback function to determine the weight to scale the low frequency updates by. If not provided, the default weight is 1.1 for timesteps within the range specified (as described in the paper).

high_frequency_weight_callback (`Callable[[torch.nn.Module], float]`, defaults to `None`) : The callback function to determine the weight to scale the high frequency updates by. If not provided, the default weight is 1.1 for timesteps within the range specified (as described in the paper).

tensor_format (`str`, defaults to `"BCFHW"`) : The format of the input tensors. This should be one of `"BCFHW"`, `"BFCHW"`, or `"BCHW"`. The format is used to split individual latent frames in order for low and high frequency components to be computed.

is_guidance_distilled (`bool`, defaults to `False`) : Whether the model is guidance distilled or not. If the model is guidance distilled, FasterCache will not be applied at the denoiser-level to skip the unconditional branch computation (as there is none).

_unconditional_conditional_input_kwargs_identifiers (`list[str]`, defaults to `("hidden_states", "encoder_hidden_states", "timestep", "attention_mask", "encoder_attention_mask")`) : The identifiers to match the input kwargs that contain the batchwise-concatenated unconditional and conditional inputs. If the name of the input kwargs contains any of these identifiers, FasterCache will split the inputs into unconditional and conditional branches. This must be a list of exact input kwargs names that contain the batchwise-concatenated unconditional and conditional inputs.

Configuration for [FasterCache](https://huggingface.co/papers/2410.19355).

#### diffusers.apply_faster_cache[[diffusers.apply_faster_cache]]

```python
diffusers.apply_faster_cache(module: Module, config: FasterCacheConfig)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/faster_cache.py#L486)

**Parameters:**

module (`torch.nn.Module`) : The pytorch module to apply FasterCache to. Typically, this should be a transformer architecture supported in Diffusers, such as `CogVideoXTransformer3DModel`, but external implementations may also work.

config (`FasterCacheConfig`) : The configuration to use for FasterCache.

Applies [FasterCache](https://huggingface.co/papers/2410.19355) to a given pipeline.

Example:
```python
>>> import torch
>>> from diffusers import CogVideoXPipeline, FasterCacheConfig, apply_faster_cache

>>> pipe = CogVideoXPipeline.from_pretrained("THUDM/CogVideoX-5b", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> config = FasterCacheConfig(
...     spatial_attention_block_skip_range=2,
...     spatial_attention_timestep_skip_range=(-1, 681),
...     low_frequency_weight_update_timestep_range=(99, 641),
...     high_frequency_weight_update_timestep_range=(-1, 301),
...     spatial_attention_block_identifiers=["transformer_blocks"],
...     attention_weight_callback=lambda _: 0.3,
...     tensor_format="BFCHW",
... )
>>> apply_faster_cache(pipe.transformer, config)
```

## FirstBlockCacheConfig[[diffusers.FirstBlockCacheConfig]]

#### diffusers.FirstBlockCacheConfig[[diffusers.FirstBlockCacheConfig]]

```python
diffusers.FirstBlockCacheConfig(threshold: float = 0.05)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/first_block_cache.py#L33)

**Parameters:**

threshold (`float`, defaults to `0.05`) : The threshold to determine whether or not a forward pass through all layers of the model is required. A higher threshold usually results in a forward pass through a lower number of layers and faster inference, but might lead to poorer generation quality. A lower threshold may not result in significant generation speedup. The threshold is compared against the absmean difference of the residuals between the current and cached outputs from the first transformer block. If the difference is below the threshold, the forward pass is skipped.

Configuration for [First Block
Cache](https://github.com/chengzeyi/ParaAttention/blob/7a266123671b55e7e5a2fe9af3121f07a36afc78/README.md#first-block-cache-our-dynamic-caching).

#### diffusers.apply_first_block_cache[[diffusers.apply_first_block_cache]]

```python
diffusers.apply_first_block_cache(module: Module, config: FirstBlockCacheConfig)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/first_block_cache.py#L193)

**Parameters:**

module (`torch.nn.Module`) : The pytorch module to apply FBCache to. Typically, this should be a transformer architecture supported in Diffusers, such as `CogVideoXTransformer3DModel`, but external implementations may also work.

config (`FirstBlockCacheConfig`) : The configuration to use for applying the FBCache method.

Applies [First Block
Cache](https://github.com/chengzeyi/ParaAttention/blob/4de137c5b96416489f06e43e19f2c14a772e28fd/README.md#first-block-cache-our-dynamic-caching)
to a given module.

First Block Cache builds on the ideas of [TeaCache](https://huggingface.co/papers/2411.19108). It is much simpler
to implement generically for a wide range of models and has been integrated first for experimental purposes.

Example:
```python
>>> import torch
>>> from diffusers import CogView4Pipeline
>>> from diffusers.hooks import apply_first_block_cache, FirstBlockCacheConfig

>>> pipe = CogView4Pipeline.from_pretrained("THUDM/CogView4-6B", torch_dtype=torch.bfloat16)
>>> pipe.to("cuda")

>>> apply_first_block_cache(pipe.transformer, FirstBlockCacheConfig(threshold=0.2))

>>> prompt = "A photo of an astronaut riding a horse on mars"
>>> image = pipe(prompt, generator=torch.Generator().manual_seed(42)).images[0]
>>> image.save("output.png")
```

## TaylorSeerCacheConfig[[diffusers.TaylorSeerCacheConfig]]

#### diffusers.TaylorSeerCacheConfig[[diffusers.TaylorSeerCacheConfig]]

```python
diffusers.TaylorSeerCacheConfig(cache_interval: int = 5, disable_cache_before_step: int = 3, disable_cache_after_step: int | None = None, max_order: int = 1, taylor_factors_dtype: typing.Optional[torch.dtype] = torch.bfloat16, skip_predict_identifiers: list[str] | None = None, cache_identifiers: list[str] | None = None, use_lite_mode: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/taylorseer_cache.py#L26)

**Parameters:**

cache_interval (`int`, defaults to `5`) : The interval between full computation steps. After a full computation, the cached (predicted) outputs are reused for this many subsequent denoising steps before refreshing with a new full forward pass. 

disable_cache_before_step (`int`, defaults to `3`) : The denoising step index before which caching is disabled, meaning full computation is performed for the initial steps (0 to disable_cache_before_step - 1) to gather data for Taylor series approximations. During these steps, Taylor factors are updated, but caching/predictions are not applied. Caching begins at this step. 

disable_cache_after_step (`int`, *optional*, defaults to `None`) : The denoising step index after which caching is disabled. If set, for steps >= this value, all modules run full computations without predictions or state updates, ensuring accuracy in later stages if needed. 

max_order (`int`, defaults to `1`) : The highest order in the Taylor series expansion for approximating module outputs. Higher orders provide better approximations but increase computation and memory usage. 

taylor_factors_dtype (`torch.dtype`, defaults to `torch.bfloat16`) : Data type used for storing and computing Taylor series factors. Lower precision reduces memory but may affect stability; higher precision improves accuracy at the cost of more memory. 

skip_predict_identifiers (`list[str]`, *optional*, defaults to `None`) : Regex patterns (using `re.fullmatch`) for module names to place as "skip" in "cache" mode. In this mode, the module computes fully during initial or refresh steps but returns a zero tensor (matching recorded shape) during prediction steps to skip computation cheaply. 

cache_identifiers (`list[str]`, *optional*, defaults to `None`) : Regex patterns (using `re.fullmatch`) for module names to place in Taylor-series caching mode, where outputs are approximated and cached for reuse. 

use_lite_mode (`bool`, *optional*, defaults to `False`) : Enables a lightweight TaylorSeer variant that minimizes memory usage by applying predefined patterns for skipping and caching (e.g., skipping blocks and caching projections). This overrides any custom `inactive_identifiers` or `active_identifiers`.

Configuration for TaylorSeer cache. See: https://huggingface.co/papers/2503.06923

Notes:
- Patterns are matched using `re.fullmatch` on the module name.
- If `skip_predict_identifiers` or `cache_identifiers` are provided, only matching modules are hooked.
- If neither is provided, all attention-like modules are hooked by default.

Example of inactive and active usage:

```py
def forward(x):
    x = self.module1(x)  # inactive module: returns zeros tensor based on shape recorded during full compute
    x = self.module2(x)  # active module: caches output here, avoiding recomputation of prior steps
    return x
```

#### bfloat16[[diffusers.TaylorSeerCacheConfig.taylor_factors_dtype]]

```python
bfloat16(*args, **kwargs)
```

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

#### diffusers.apply_taylorseer_cache[[diffusers.apply_taylorseer_cache]]

```python
diffusers.apply_taylorseer_cache(module: Module, config: TaylorSeerCacheConfig)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/taylorseer_cache.py#L258)

**Parameters:**

module (torch.nn.Module) : The model subtree to apply the hooks to.

config (TaylorSeerCacheConfig) : Configuration for the cache.

Applies the TaylorSeer cache to a given pipeline (typically the transformer / UNet).

This function hooks selected modules in the model to enable caching or skipping based on the provided
configuration, reducing redundant computations in diffusion denoising loops.

Example:
```python
>>> import torch
>>> from diffusers import FluxPipeline, TaylorSeerCacheConfig

>>> pipe = FluxPipeline.from_pretrained(
...     "black-forest-labs/FLUX.1-dev",
...     torch_dtype=torch.bfloat16,
... )
>>> pipe.to("cuda")

>>> config = TaylorSeerCacheConfig(
...     cache_interval=5,
...     max_order=1,
...     disable_cache_before_step=3,
...     taylor_factors_dtype=torch.float32,
... )
>>> pipe.transformer.enable_cache(config)
```

## MagCacheConfig[[diffusers.MagCacheConfig]]

#### diffusers.MagCacheConfig[[diffusers.MagCacheConfig]]

```python
diffusers.MagCacheConfig(threshold: float = 0.06, max_skip_steps: int = 3, retention_ratio: float = 0.2, num_inference_steps: int = 28, mag_ratios: typing.Union[torch.Tensor, typing.List[float], NoneType] = None, calibrate: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/mag_cache.py#L84)

**Parameters:**

threshold (`float`, defaults to `0.06`) : The threshold for the accumulated error. If the accumulated error is below this threshold, the block computation is skipped. A higher threshold allows for more aggressive skipping (faster) but may degrade quality.

max_skip_steps (`int`, defaults to `3`) : The maximum number of consecutive steps that can be skipped (K in the paper).

retention_ratio (`float`, defaults to `0.2`) : The fraction of initial steps during which skipping is disabled to ensure stability. For example, if `num_inference_steps` is 28 and `retention_ratio` is 0.2, the first 6 steps will never be skipped.

num_inference_steps (`int`, defaults to `28`) : The number of inference steps used in the pipeline. This is required to interpolate `mag_ratios` correctly.

mag_ratios (`torch.Tensor`, *optional*) : The pre-computed magnitude ratios for the model. These are checkpoint-dependent. If not provided, you must set `calibrate=True` to calculate them for your specific model. For Flux models, you can use `diffusers.hooks.mag_cache.FLUX_MAG_RATIOS`.

calibrate (`bool`, defaults to `False`) : If True, enables calibration mode. In this mode, no blocks are skipped. Instead, the hook calculates the magnitude ratios for the current run and logs them at the end. Use this to obtain `mag_ratios` for new models or schedulers.

Configuration for [MagCache](https://github.com/Zehong-Ma/MagCache).

#### diffusers.apply_mag_cache[[diffusers.apply_mag_cache]]

```python
diffusers.apply_mag_cache(module: Module, config: MagCacheConfig)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/hooks/mag_cache.py#L397)

**Parameters:**

module (`torch.nn.Module`) : The module to apply MagCache to.

config (`MagCacheConfig`) : The configuration for MagCache.

Applies MagCache to a given module (typically a Transformer).
