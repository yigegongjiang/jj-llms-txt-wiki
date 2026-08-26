# CogVideoXDPMScheduler

`CogVideoXDPMScheduler` is based on [DPM-Solver: A Fast ODE Solver for Diffusion Probabilistic Model Sampling in Around 10 Steps](https://huggingface.co/papers/2206.00927) and [DPM-Solver++: Fast Solver for Guided Sampling of Diffusion Probabilistic Models](https://huggingface.co/papers/2211.01095), specifically for CogVideoX models.

## CogVideoXDPMScheduler[[diffusers.CogVideoXDPMScheduler]]

#### diffusers.CogVideoXDPMScheduler[[diffusers.CogVideoXDPMScheduler]]

```python
diffusers.CogVideoXDPMScheduler(num_train_timesteps: int = 1000, beta_start: float = 0.00085, beta_end: float = 0.012, beta_schedule: typing.Literal['linear', 'scaled_linear', 'squaredcos_cap_v2'] = 'scaled_linear', trained_betas: numpy.ndarray | list[float] | None = None, clip_sample: bool = True, set_alpha_to_one: bool = True, steps_offset: int = 0, prediction_type: typing.Literal['epsilon', 'sample', 'v_prediction'] = 'epsilon', clip_sample_range: float = 1.0, sample_max_value: float = 1.0, timestep_spacing: typing.Literal['leading', 'linspace', 'trailing'] = 'leading', rescale_betas_zero_snr: bool = False, snr_shift_scale: float = 3.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpm_cogvideox.py#L134)

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to 0.0001) : The starting `beta` value of inference.

beta_end (`float`, defaults to 0.02) : The final `beta` value.

beta_schedule (`str`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Choose from `linear`, `scaled_linear`, or `squaredcos_cap_v2`.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

clip_sample (`bool`, defaults to `True`) : Clip the predicted sample for numerical stability.

clip_sample_range (`float`, defaults to 1.0) : The maximum magnitude for sample clipping. Valid only when `clip_sample=True`.

set_alpha_to_one (`bool`, defaults to `True`) : Each diffusion step uses the alphas product value at that step and at the previous one. For the final step there is no previous alpha. When this option is `True` the previous alpha product is fixed to `1`, otherwise it uses the alpha value at step 0.

steps_offset (`int`, defaults to 0) : An offset added to the inference steps, as required by some model families.

prediction_type (`str`, defaults to `epsilon`, *optional*) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`) or `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

thresholding (`bool`, defaults to `False`) : Whether to use the "dynamic thresholding" method. This is unsuitable for latent-space diffusion models such as Stable Diffusion.

dynamic_thresholding_ratio (`float`, defaults to 0.995) : The ratio for the dynamic thresholding method. Valid only when `thresholding=True`.

sample_max_value (`float`, defaults to 1.0) : The threshold value for dynamic thresholding. Valid only when `thresholding=True`.

timestep_spacing (`str`, defaults to `"leading"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information. Choose from `leading`, `linspace` or `trailing`.

rescale_betas_zero_snr (`bool`, defaults to `False`) : Whether to rescale the betas to have zero terminal SNR. This enables the model to generate very bright and dark samples instead of limiting it to samples with medium brightness. Loosely related to [`--offset_noise`](https://github.com/huggingface/diffusers/blob/74fd735eb073eb1d774b1ab4154a0876eb82f055/examples/dreambooth/train_dreambooth.py#L506).

snr_shift_scale (`float`, defaults to 3.0) : Shift scale for SNR.

`DDIMScheduler` extends the denoising procedure introduced in denoising diffusion probabilistic models (DDPMs) with
non-Markovian guidance.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### add_noise[[diffusers.CogVideoXDPMScheduler.add_noise]]

```python
add_noise(original_samples: Tensor, noise: Tensor, timesteps: IntTensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpm_cogvideox.py#L522)

**Parameters:**

original_samples (`torch.Tensor`) : The original samples to which noise will be added.

noise (`torch.Tensor`) : The noise to add to the samples.

timesteps (`torch.IntTensor`) : The timesteps indicating the noise level for each sample.

**Returns:** `torch.Tensor`

The noisy samples.

Add noise to the original samples according to the noise magnitude at each timestep (this is the forward
diffusion process).

#### get_mult[[diffusers.CogVideoXDPMScheduler.get_mult]]

```python
get_mult(h: Tensor, r: Tensor, alpha_prod_t: Tensor, alpha_prod_t_prev: Tensor, alpha_prod_t_back: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpm_cogvideox.py#L364)

**Parameters:**

h (`torch.Tensor`) : The log-SNR difference.

r (`torch.Tensor`) : The ratio of log-SNR differences.

alpha_prod_t (`torch.Tensor`) : The cumulative product of alphas at the current timestep.

alpha_prod_t_prev (`torch.Tensor`) : The cumulative product of alphas at the previous timestep.

alpha_prod_t_back (`torch.Tensor`, *optional*) : The cumulative product of alphas at the timestep before the previous timestep.

**Returns:** `tuple`

A tuple containing the multipliers.

Compute the multipliers for the previous sample and the predicted original sample.

#### get_variables[[diffusers.CogVideoXDPMScheduler.get_variables]]

```python
get_variables(alpha_prod_t: Tensor, alpha_prod_t_prev: Tensor, alpha_prod_t_back: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpm_cogvideox.py#L331)

**Parameters:**

alpha_prod_t (`torch.Tensor`) : The cumulative product of alphas at the current timestep.

alpha_prod_t_prev (`torch.Tensor`) : The cumulative product of alphas at the previous timestep.

alpha_prod_t_back (`torch.Tensor`, *optional*) : The cumulative product of alphas at the timestep before the previous timestep.

**Returns:** `tuple`

A tuple containing the variables `h`, `r`, `lamb`, `lamb_next`.

Compute the variables used for DPM-Solver++ (2M) referencing the original implementation.

#### get_velocity[[diffusers.CogVideoXDPMScheduler.get_velocity]]

```python
get_velocity(sample: Tensor, noise: Tensor, timesteps: IntTensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpm_cogvideox.py#L565)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

noise (`torch.Tensor`) : The noise tensor.

timesteps (`torch.IntTensor`) : The timesteps for velocity computation.

**Returns:** `torch.Tensor`

The computed velocity.

Compute the velocity prediction from the sample and noise according to the velocity formula.

#### scale_model_input[[diffusers.CogVideoXDPMScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, timestep: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpm_cogvideox.py#L262)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`int`, *optional*) : The current timestep in the diffusion chain.

**Returns:** `torch.Tensor`

A scaled input sample.

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

#### set_timesteps[[diffusers.CogVideoXDPMScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int, device: typing.Union[str, torch.device, NoneType] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpm_cogvideox.py#L279)

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None` (the default), the timesteps are not moved.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.CogVideoXDPMScheduler.step]]

```python
step(model_output: Tensor, old_pred_original_sample: Tensor, timestep: int, timestep_back: int, sample: Tensor, eta: float = 0.0, use_clipped_model_output: bool = False, generator: typing.Optional[torch.Generator] = None, variance_noise: typing.Optional[torch.Tensor] = None, return_dict: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpm_cogvideox.py#L401)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

old_pred_original_sample (`torch.Tensor`) : The predicted original sample from the previous timestep.

timestep (`int`) : The current discrete timestep in the diffusion chain.

timestep_back (`int`) : The timestep to look back to.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

eta (`float`) : The weight of noise for added noise in diffusion step.

use_clipped_model_output (`bool`, defaults to `False`) : If `True`, computes "corrected" `model_output` from the clipped predicted original sample. Necessary because predicted original sample is clipped to [-1, 1] when `self.config.clip_sample` is `True`. If no clipping has happened, "corrected" `model_output` would coincide with the one provided as input and `use_clipped_model_output` has no effect.

generator (`torch.Generator`, *optional*) : A random number generator.

variance_noise (`torch.Tensor`) : Alternative to generating noise with `generator` by directly providing the noise for the variance itself. Useful for methods such as `CycleDiffusion`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [DDIMSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.schedulers.scheduling_ddim.DDIMSchedulerOutput) or `tuple`.

**Returns:** [DDIMSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.schedulers.scheduling_ddim.DDIMSchedulerOutput) or `tuple`

If return_dict is `True`, [DDIMSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.schedulers.scheduling_ddim.DDIMSchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).
