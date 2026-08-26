# Guiders

Guiders are components in Modular Diffusers that control how the diffusion process is guided during generation. They implement various guidance techniques to improve generation quality and control.

## BaseGuidance[[diffusers.BaseGuidance]]

#### diffusers.BaseGuidance[[diffusers.BaseGuidance]]

```python
diffusers.BaseGuidance(start: float = 0.0, stop: float = 1.0, enabled: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/guider_utils.py#L38)

Base class providing the skeleton for implementing guidance techniques.

#### cleanup_models[[diffusers.BaseGuidance.cleanup_models]]

```python
cleanup_models(denoiser: torch.nn.Module)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/guider_utils.py#L160)

Cleans up the models for the guidance technique after a given batch of data. This method should be overridden
in subclasses to implement specific model cleanup logic. It is useful for removing any hooks or other stateful
modifications made during `prepare_models`.

#### from_pretrained[[diffusers.BaseGuidance.from_pretrained]]

```python
from_pretrained(pretrained_model_name_or_path: str | os.PathLike | None = None, subfolder: str | None = None, return_unused_kwargs = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/guider_utils.py#L289)

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`, *optional*) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing the guider configuration saved with [save_pretrained()](/docs/diffusers/v0.40.0/en/api/modular_diffusers/guiders#diffusers.BaseGuidance.save_pretrained).

subfolder (`str`, *optional*) : The subfolder location of a model file within a larger model repository on the Hub or locally.

return_unused_kwargs (`bool`, *optional*, defaults to `False`) : Whether kwargs that are not consumed by the Python class should be returned or not.

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist. 

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

output_loading_info(`bool`, *optional*, defaults to `False`) : Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.

local_files_only(`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

Instantiate a guider from a pre-defined JSON configuration file in a local directory or Hub repository.

> [!TIP] > To use private or [gated models](https://huggingface.co/docs/hub/models-gated#gated-models), log-in
with `hf > auth login`. You can also activate the special >
["offline-mode"](https://huggingface.co/diffusers/installation.html#offline-mode) to use this method in a >
firewalled environment.

#### get_state[[diffusers.BaseGuidance.get_state]]

```python
get_state()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/guider_utils.py#L106)

Returns the current state of the guidance technique as a dictionary. The state variables will be included in
the __repr__ method. Returns:
`dict[str, Any]`: A dictionary containing the current state variables including:
- step: Current inference step
- num_inference_steps: Total number of inference steps
- timestep: Current timestep tensor
- count_prepared: Number of times prepare_models has been called
- enabled: Whether the guidance is enabled
- num_conditions: Number of conditions

#### new[[diffusers.BaseGuidance.new]]

```python
new(**kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/guider_utils.py#L69)

**Parameters:**

- ****kwargs** : Configuration parameters to override in the new instance. If no kwargs are provided, returns an exact copy with the same configuration.

**Returns:**

A new guider instance with the same (or updated) configuration.

Creates a copy of this guider instance, optionally with modified configuration parameters.

Example:
```python
# Create a CFG guider
guider = ClassifierFreeGuidance(guidance_scale=3.5)

# Create an exact copy
same_guider = guider.new()

# Create a copy with different start step, keeping other config the same
new_guider = guider.new(guidance_scale=5)
```

#### prepare_models[[diffusers.BaseGuidance.prepare_models]]

```python
prepare_models(denoiser: torch.nn.Module)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/guider_utils.py#L153)

Prepares the models for the guidance technique on a given batch of data. This method should be overridden in
subclasses to implement specific model preparation logic.

#### save_pretrained[[diffusers.BaseGuidance.save_pretrained]]

```python
save_pretrained(save_directory: str | os.PathLike, push_to_hub: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/guider_utils.py#L350)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory where the configuration JSON file will be saved (will be created if it does not exist).

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether or not to push your model to the Hugging Face Hub after saving it. You can specify the repository you want to push to with `repo_id` (will default to the name of `save_directory` in your namespace).

kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments passed along to the [push_to_hub()](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.utils.PushToHubMixin.push_to_hub) method.

Save a guider configuration object to a directory so that it can be reloaded using the
[from_pretrained()](/docs/diffusers/v0.40.0/en/api/modular_diffusers/guiders#diffusers.BaseGuidance.from_pretrained) class method.

## ClassifierFreeGuidance[[diffusers.ClassifierFreeGuidance]]

#### diffusers.ClassifierFreeGuidance[[diffusers.ClassifierFreeGuidance]]

```python
diffusers.ClassifierFreeGuidance(guidance_scale: float = 7.5, guidance_rescale: float = 0.0, use_original_formulation: bool = False, start: float = 0.0, stop: float = 1.0, enabled: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/classifier_free_guidance.py#L30)

**Parameters:**

guidance_scale (`float`, defaults to `7.5`) : CFG scale applied by this guider during post-processing. Higher values = stronger prompt conditioning but may reduce quality. Typical range: 1.0-20.0.

guidance_rescale (`float`, defaults to `0.0`) : Rescaling factor to prevent overexposure from high guidance scales. Based on [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891). Range: 0.0 (no rescaling) to 1.0 (full rescaling).

use_original_formulation (`bool`, defaults to `False`) : If `True`, uses the original CFG formulation from the paper. If `False` (default), uses the diffusers-native formulation from the Imagen paper.

start (`float`, defaults to `0.0`) : Fraction of denoising steps (0.0-1.0) after which CFG starts. Use > 0.0 to disable CFG in early denoising steps.

stop (`float`, defaults to `1.0`) : Fraction of denoising steps (0.0-1.0) after which CFG stops. Use < 1.0 to disable CFG in late denoising steps.

enabled (`bool`, defaults to `True`) : Whether CFG is enabled. Set to `False` to disable CFG entirely (uses only conditional predictions).

Implements Classifier-Free Guidance (CFG) for diffusion models.

Reference: https://huggingface.co/papers/2207.12598

CFG improves generation quality and prompt adherence by jointly training models on both conditional and
unconditional data, then combining predictions during inference. This allows trading off between quality (high
guidance) and diversity (low guidance).

**Two CFG Formulations:**

1. **Original formulation** (from paper):
```
x_pred = x_cond + guidance_scale * (x_cond - x_uncond)
```

   Moves conditional predictions further from unconditional ones.

2. **Diffusers-native formulation** (default, from Imagen paper):
```
x_pred = x_uncond + guidance_scale * (x_cond - x_uncond)
```

   Moves unconditional predictions toward conditional ones, effectively suppressing negative features (e.g., "bad
   quality", "watermarks"). Equivalent in theory but more intuitive.

Use `use_original_formulation=True` to switch to the original formulation.

## ClassifierFreeZeroStarGuidance[[diffusers.ClassifierFreeZeroStarGuidance]]

#### diffusers.ClassifierFreeZeroStarGuidance[[diffusers.ClassifierFreeZeroStarGuidance]]

```python
diffusers.ClassifierFreeZeroStarGuidance(guidance_scale: float = 7.5, zero_init_steps: int = 1, guidance_rescale: float = 0.0, use_original_formulation: bool = False, start: float = 0.0, stop: float = 1.0, enabled: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/classifier_free_zero_star_guidance.py#L30)

**Parameters:**

guidance_scale (`float`, defaults to `7.5`) : The scale parameter for classifier-free guidance. Higher values result in stronger conditioning on the text prompt, while lower values allow for more freedom in generation. Higher values may lead to saturation and deterioration of image quality.

zero_init_steps (`int`, defaults to `1`) : The number of inference steps for which the noise predictions are zeroed out (see Section 4.2).

guidance_rescale (`float`, defaults to `0.0`) : The rescale factor applied to the noise predictions. This is used to improve image quality and fix overexposure. Based on Section 3.4 from [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891).

use_original_formulation (`bool`, defaults to `False`) : Whether to use the original formulation of classifier-free guidance as proposed in the paper. By default, we use the diffusers-native implementation that has been in the codebase for a long time. See [~guiders.classifier_free_guidance.ClassifierFreeGuidance] for more details.

start (`float`, defaults to `0.01`) : The fraction of the total number of denoising steps after which guidance starts.

stop (`float`, defaults to `0.2`) : The fraction of the total number of denoising steps after which guidance stops.

Classifier-free Zero* (CFG-Zero*): https://huggingface.co/papers/2503.18886

This is an implementation of the Classifier-Free Zero* guidance technique, which is a variant of classifier-free
guidance. It proposes zero initialization of the noise predictions for the first few steps of the diffusion
process, and also introduces an optimal rescaling factor for the noise predictions, which can help in improving the
quality of generated images.

The authors of the paper suggest setting zero initialization in the first 4% of the inference steps.

## SkipLayerGuidance[[diffusers.SkipLayerGuidance]]

#### diffusers.SkipLayerGuidance[[diffusers.SkipLayerGuidance]]

```python
diffusers.SkipLayerGuidance(guidance_scale: float = 7.5, skip_layer_guidance_scale: float = 2.8, skip_layer_guidance_start: float = 0.01, skip_layer_guidance_stop: float = 0.2, skip_layer_guidance_layers: int | list[int] | None = None, skip_layer_config: LayerSkipConfig | list[LayerSkipConfig] | dict[str, Any] = None, guidance_rescale: float = 0.0, use_original_formulation: bool = False, start: float = 0.0, stop: float = 1.0, enabled: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/skip_layer_guidance.py#L32)

**Parameters:**

guidance_scale (`float`, defaults to `7.5`) : The scale parameter for classifier-free guidance. Higher values result in stronger conditioning on the text prompt, while lower values allow for more freedom in generation. Higher values may lead to saturation and deterioration of image quality.

skip_layer_guidance_scale (`float`, defaults to `2.8`) : The scale parameter for skip layer guidance. Anatomy and structure coherence may improve with higher values, but it may also lead to overexposure and saturation.

skip_layer_guidance_start (`float`, defaults to `0.01`) : The fraction of the total number of denoising steps after which skip layer guidance starts.

skip_layer_guidance_stop (`float`, defaults to `0.2`) : The fraction of the total number of denoising steps after which skip layer guidance stops.

skip_layer_guidance_layers (`int` or `list[int]`, *optional*) : The layer indices to apply skip layer guidance to. Can be a single integer or a list of integers. If not provided, `skip_layer_config` must be provided. The recommended values are `[7, 8, 9]` for Stable Diffusion 3.5 Medium.

skip_layer_config (`LayerSkipConfig` or `list[LayerSkipConfig]`, *optional*) : The configuration for the skip layer guidance. Can be a single `LayerSkipConfig` or a list of `LayerSkipConfig`. If not provided, `skip_layer_guidance_layers` must be provided.

guidance_rescale (`float`, defaults to `0.0`) : The rescale factor applied to the noise predictions. This is used to improve image quality and fix overexposure. Based on Section 3.4 from [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891).

use_original_formulation (`bool`, defaults to `False`) : Whether to use the original formulation of classifier-free guidance as proposed in the paper. By default, we use the diffusers-native implementation that has been in the codebase for a long time. See [~guiders.classifier_free_guidance.ClassifierFreeGuidance] for more details.

start (`float`, defaults to `0.01`) : The fraction of the total number of denoising steps after which guidance starts.

stop (`float`, defaults to `0.2`) : The fraction of the total number of denoising steps after which guidance stops.

Skip Layer Guidance (SLG): https://github.com/Stability-AI/sd3.5

Spatio-Temporal Guidance (STG): https://huggingface.co/papers/2411.18664

SLG was introduced by StabilityAI for improving structure and anotomy coherence in generated images. It works by
skipping the forward pass of specified transformer blocks during the denoising process on an additional conditional
batch of data, apart from the conditional and unconditional batches already used in CFG
([~guiders.classifier_free_guidance.ClassifierFreeGuidance]), and then scaling and shifting the CFG predictions
based on the difference between conditional without skipping and conditional with skipping predictions.

The intution behind SLG can be thought of as moving the CFG predicted distribution estimates further away from
worse versions of the conditional distribution estimates (because skipping layers is equivalent to using a worse
version of the model for the conditional prediction).

STG is an improvement and follow-up work combining ideas from SLG, PAG and similar techniques for improving
generation quality in video diffusion models.

Additional reading:
- [Guiding a Diffusion Model with a Bad Version of Itself](https://huggingface.co/papers/2406.02507)

The values for `skip_layer_guidance_scale`, `skip_layer_guidance_start`, and `skip_layer_guidance_stop` are
defaulted to the recommendations by StabilityAI for Stable Diffusion 3.5 Medium.

## SmoothedEnergyGuidance[[diffusers.SmoothedEnergyGuidance]]

#### diffusers.SmoothedEnergyGuidance[[diffusers.SmoothedEnergyGuidance]]

```python
diffusers.SmoothedEnergyGuidance(guidance_scale: float = 7.5, seg_guidance_scale: float = 2.8, seg_blur_sigma: float = 9999999.0, seg_blur_threshold_inf: float = 9999.0, seg_guidance_start: float = 0.0, seg_guidance_stop: float = 1.0, seg_guidance_layers: int | list[int] | None = None, seg_guidance_config: SmoothedEnergyGuidanceConfig | list[SmoothedEnergyGuidanceConfig] = None, guidance_rescale: float = 0.0, use_original_formulation: bool = False, start: float = 0.0, stop: float = 1.0, enabled: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/smoothed_energy_guidance.py#L32)

**Parameters:**

guidance_scale (`float`, defaults to `7.5`) : The scale parameter for classifier-free guidance. Higher values result in stronger conditioning on the text prompt, while lower values allow for more freedom in generation. Higher values may lead to saturation and deterioration of image quality.

seg_guidance_scale (`float`, defaults to `3.0`) : The scale parameter for smoothed energy guidance. Anatomy and structure coherence may improve with higher values, but it may also lead to overexposure and saturation.

seg_blur_sigma (`float`, defaults to `9999999.0`) : The amount by which we blur the attention weights. Setting this value greater than 9999.0 results in infinite blur, which means uniform queries. Controlling it exponentially is empirically effective.

seg_blur_threshold_inf (`float`, defaults to `9999.0`) : The threshold above which the blur is considered infinite.

seg_guidance_start (`float`, defaults to `0.0`) : The fraction of the total number of denoising steps after which smoothed energy guidance starts.

seg_guidance_stop (`float`, defaults to `1.0`) : The fraction of the total number of denoising steps after which smoothed energy guidance stops.

seg_guidance_layers (`int` or `list[int]`, *optional*) : The layer indices to apply smoothed energy guidance to. Can be a single integer or a list of integers. If not provided, `seg_guidance_config` must be provided. The recommended values are `[7, 8, 9]` for Stable Diffusion 3.5 Medium.

seg_guidance_config (`SmoothedEnergyGuidanceConfig` or `list[SmoothedEnergyGuidanceConfig]`, *optional*) : The configuration for the smoothed energy layer guidance. Can be a single `SmoothedEnergyGuidanceConfig` or a list of `SmoothedEnergyGuidanceConfig`. If not provided, `seg_guidance_layers` must be provided.

guidance_rescale (`float`, defaults to `0.0`) : The rescale factor applied to the noise predictions. This is used to improve image quality and fix overexposure. Based on Section 3.4 from [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891).

use_original_formulation (`bool`, defaults to `False`) : Whether to use the original formulation of classifier-free guidance as proposed in the paper. By default, we use the diffusers-native implementation that has been in the codebase for a long time. See [~guiders.classifier_free_guidance.ClassifierFreeGuidance] for more details.

start (`float`, defaults to `0.01`) : The fraction of the total number of denoising steps after which guidance starts.

stop (`float`, defaults to `0.2`) : The fraction of the total number of denoising steps after which guidance stops.

Smoothed Energy Guidance (SEG): https://huggingface.co/papers/2408.00760

SEG is only supported as an experimental prototype feature for now, so the implementation may be modified in the
future without warning or guarantee of reproducibility. This implementation assumes:
- Generated images are square (height == width)
- The model does not combine different modalities together (e.g., text and image latent streams are not combined
  together such as Flux)

## PerturbedAttentionGuidance[[diffusers.PerturbedAttentionGuidance]]

#### diffusers.PerturbedAttentionGuidance[[diffusers.PerturbedAttentionGuidance]]

```python
diffusers.PerturbedAttentionGuidance(guidance_scale: float = 7.5, perturbed_guidance_scale: float = 2.8, perturbed_guidance_start: float = 0.01, perturbed_guidance_stop: float = 0.2, perturbed_guidance_layers: int | list[int] | None = None, perturbed_guidance_config: LayerSkipConfig | list[LayerSkipConfig] | dict[str, Any] = None, guidance_rescale: float = 0.0, use_original_formulation: bool = False, start: float = 0.0, stop: float = 1.0, enabled: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/perturbed_attention_guidance.py#L36)

**Parameters:**

guidance_scale (`float`, defaults to `7.5`) : The scale parameter for classifier-free guidance. Higher values result in stronger conditioning on the text prompt, while lower values allow for more freedom in generation. Higher values may lead to saturation and deterioration of image quality.

perturbed_guidance_scale (`float`, defaults to `2.8`) : The scale parameter for perturbed attention guidance.

perturbed_guidance_start (`float`, defaults to `0.01`) : The fraction of the total number of denoising steps after which perturbed attention guidance starts.

perturbed_guidance_stop (`float`, defaults to `0.2`) : The fraction of the total number of denoising steps after which perturbed attention guidance stops.

perturbed_guidance_layers (`int` or `list[int]`, *optional*) : The layer indices to apply perturbed attention guidance to. Can be a single integer or a list of integers. If not provided, `perturbed_guidance_config` must be provided.

perturbed_guidance_config (`LayerSkipConfig` or `list[LayerSkipConfig]`, *optional*) : The configuration for the perturbed attention guidance. Can be a single `LayerSkipConfig` or a list of `LayerSkipConfig`. If not provided, `perturbed_guidance_layers` must be provided.

guidance_rescale (`float`, defaults to `0.0`) : The rescale factor applied to the noise predictions. This is used to improve image quality and fix overexposure. Based on Section 3.4 from [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891).

use_original_formulation (`bool`, defaults to `False`) : Whether to use the original formulation of classifier-free guidance as proposed in the paper. By default, we use the diffusers-native implementation that has been in the codebase for a long time. See [~guiders.classifier_free_guidance.ClassifierFreeGuidance] for more details.

start (`float`, defaults to `0.01`) : The fraction of the total number of denoising steps after which guidance starts.

stop (`float`, defaults to `0.2`) : The fraction of the total number of denoising steps after which guidance stops.

Perturbed Attention Guidance (PAG): https://huggingface.co/papers/2403.17377

The intution behind PAG can be thought of as moving the CFG predicted distribution estimates further away from
worse versions of the conditional distribution estimates. PAG was one of the first techniques to introduce the idea
of using a worse version of the trained model for better guiding itself in the denoising process. It perturbs the
attention scores of the latent stream by replacing the score matrix with an identity matrix for selectively chosen
layers.

Additional reading:
- [Guiding a Diffusion Model with a Bad Version of Itself](https://huggingface.co/papers/2406.02507)

PAG is implemented with similar implementation to SkipLayerGuidance due to overlap in the configuration parameters
and implementation details.

## AdaptiveProjectedGuidance[[diffusers.AdaptiveProjectedGuidance]]

#### diffusers.AdaptiveProjectedGuidance[[diffusers.AdaptiveProjectedGuidance]]

```python
diffusers.AdaptiveProjectedGuidance(guidance_scale: float = 7.5, adaptive_projected_guidance_momentum: float | None = None, adaptive_projected_guidance_rescale: float = 15.0, adaptive_projected_guidance_norm_dim: int | tuple[int, ...] | None = None, eta: float = 1.0, guidance_rescale: float = 0.0, use_original_formulation: bool = False, start: float = 0.0, stop: float = 1.0, enabled: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/adaptive_projected_guidance.py#L30)

**Parameters:**

guidance_scale (`float`, defaults to `7.5`) : The scale parameter for classifier-free guidance. Higher values result in stronger conditioning on the text prompt, while lower values allow for more freedom in generation. Higher values may lead to saturation and deterioration of image quality.

adaptive_projected_guidance_momentum (`float`, defaults to `None`) : The momentum parameter for the adaptive projected guidance. Disabled if set to `None`.

adaptive_projected_guidance_rescale (`float`, defaults to `15.0`) : The rescale factor applied to the noise predictions. This is used to improve image quality and fix

adaptive_projected_guidance_norm_dim (`int` or `tuple[int]`, *optional*) : Dimension(s) over which to compute the APG norm and projection. If omitted, all non-batch dimensions are used, preserving the original behavior.

guidance_rescale (`float`, defaults to `0.0`) : The rescale factor applied to the noise predictions. This is used to improve image quality and fix overexposure. Based on Section 3.4 from [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891).

use_original_formulation (`bool`, defaults to `False`) : Whether to use the original formulation of classifier-free guidance as proposed in the paper. By default, we use the diffusers-native implementation that has been in the codebase for a long time. See [~guiders.classifier_free_guidance.ClassifierFreeGuidance] for more details.

start (`float`, defaults to `0.0`) : The fraction of the total number of denoising steps after which guidance starts.

stop (`float`, defaults to `1.0`) : The fraction of the total number of denoising steps after which guidance stops.

Adaptive Projected Guidance (APG): https://huggingface.co/papers/2410.02416

## AutoGuidance[[diffusers.AutoGuidance]]

#### diffusers.AutoGuidance[[diffusers.AutoGuidance]]

```python
diffusers.AutoGuidance(guidance_scale: float = 7.5, auto_guidance_layers: int | list[int] | None = None, auto_guidance_config: LayerSkipConfig | list[LayerSkipConfig] | dict[str, Any] = None, dropout: float | None = None, guidance_rescale: float = 0.0, use_original_formulation: bool = False, start: float = 0.0, stop: float = 1.0, enabled: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/auto_guidance.py#L32)

**Parameters:**

guidance_scale (`float`, defaults to `7.5`) : The scale parameter for classifier-free guidance. Higher values result in stronger conditioning on the text prompt, while lower values allow for more freedom in generation. Higher values may lead to saturation and deterioration of image quality.

auto_guidance_layers (`int` or `list[int]`, *optional*) : The layer indices to apply skip layer guidance to. Can be a single integer or a list of integers. If not provided, `skip_layer_config` must be provided.

auto_guidance_config (`LayerSkipConfig` or `list[LayerSkipConfig]`, *optional*) : The configuration for the skip layer guidance. Can be a single `LayerSkipConfig` or a list of `LayerSkipConfig`. If not provided, `skip_layer_guidance_layers` must be provided.

dropout (`float`, *optional*) : The dropout probability for autoguidance on the enabled skip layers (either with `auto_guidance_layers` or `auto_guidance_config`). If not provided, the dropout probability will be set to 1.0.

guidance_rescale (`float`, defaults to `0.0`) : The rescale factor applied to the noise predictions. This is used to improve image quality and fix overexposure. Based on Section 3.4 from [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891).

use_original_formulation (`bool`, defaults to `False`) : Whether to use the original formulation of classifier-free guidance as proposed in the paper. By default, we use the diffusers-native implementation that has been in the codebase for a long time. See [~guiders.classifier_free_guidance.ClassifierFreeGuidance] for more details.

start (`float`, defaults to `0.0`) : The fraction of the total number of denoising steps after which guidance starts.

stop (`float`, defaults to `1.0`) : The fraction of the total number of denoising steps after which guidance stops.

AutoGuidance: https://huggingface.co/papers/2406.02507

## TangentialClassifierFreeGuidance[[diffusers.TangentialClassifierFreeGuidance]]

#### diffusers.TangentialClassifierFreeGuidance[[diffusers.TangentialClassifierFreeGuidance]]

```python
diffusers.TangentialClassifierFreeGuidance(guidance_scale: float = 7.5, guidance_rescale: float = 0.0, use_original_formulation: bool = False, start: float = 0.0, stop: float = 1.0, enabled: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/guiders/tangential_classifier_free_guidance.py#L30)

**Parameters:**

guidance_scale (`float`, defaults to `7.5`) : The scale parameter for classifier-free guidance. Higher values result in stronger conditioning on the text prompt, while lower values allow for more freedom in generation. Higher values may lead to saturation and deterioration of image quality.

guidance_rescale (`float`, defaults to `0.0`) : The rescale factor applied to the noise predictions. This is used to improve image quality and fix overexposure. Based on Section 3.4 from [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891).

use_original_formulation (`bool`, defaults to `False`) : Whether to use the original formulation of classifier-free guidance as proposed in the paper. By default, we use the diffusers-native implementation that has been in the codebase for a long time. See [~guiders.classifier_free_guidance.ClassifierFreeGuidance] for more details.

start (`float`, defaults to `0.0`) : The fraction of the total number of denoising steps after which guidance starts.

stop (`float`, defaults to `1.0`) : The fraction of the total number of denoising steps after which guidance stops.

Tangential Classifier Free Guidance (TCFG): https://huggingface.co/papers/2503.18137
