# DDIMInverseScheduler

`DDIMInverseScheduler` is the inverted scheduler from [Denoising Diffusion Implicit Models](https://huggingface.co/papers/2010.02502) (DDIM) by Jiaming Song, Chenlin Meng and Stefano Ermon.
The implementation is mostly based on the DDIM inversion definition from [Null-text Inversion for Editing Real Images using Guided Diffusion Models](https://huggingface.co/papers/2211.09794).

## DDIMInverseScheduler[[diffusers.DDIMInverseScheduler]]

#### diffusers.DDIMInverseScheduler[[diffusers.DDIMInverseScheduler]]

```python
diffusers.DDIMInverseScheduler(num_train_timesteps: int = 1000, beta_start: float = 0.0001, beta_end: float = 0.02, beta_schedule: typing.Literal['linear', 'scaled_linear', 'squaredcos_cap_v2'] = 'linear', trained_betas: numpy.ndarray | list[float] | None = None, clip_sample: bool = True, set_alpha_to_one: bool = True, steps_offset: int = 0, prediction_type: typing.Literal['epsilon', 'sample', 'v_prediction'] = 'epsilon', clip_sample_range: float = 1.0, timestep_spacing: typing.Literal['leading', 'trailing'] = 'leading', rescale_betas_zero_snr: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddim_inverse.py#L138)

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to 0.0001) : The starting `beta` value of inference.

beta_end (`float`, defaults to 0.02) : The final `beta` value.

beta_schedule (`str`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Choose from `linear`, `scaled_linear`, or `squaredcos_cap_v2`.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

clip_sample (`bool`, defaults to `True`) : Clip the predicted sample for numerical stability.

clip_sample_range (`float`, defaults to 1.0) : The maximum magnitude for sample clipping. Valid only when `clip_sample=True`.

set_alpha_to_one (`bool`, defaults to `True`) : Each diffusion step uses the alphas product value at that step and at the previous one. For the final step there is no previous alpha. When this option is `True` the previous alpha product is fixed to 0, otherwise it uses the alpha value at step `num_train_timesteps - 1`.

steps_offset (`int`, defaults to 0) : An offset added to the inference steps, as required by some model families.

prediction_type (`str`, defaults to `epsilon`, *optional*) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`) or `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

timestep_spacing (`str`, defaults to `"leading"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

rescale_betas_zero_snr (`bool`, defaults to `False`) : Whether to rescale the betas to have zero terminal SNR. This enables the model to generate very bright and dark samples instead of limiting it to samples with medium brightness. Loosely related to [`--offset_noise`](https://github.com/huggingface/diffusers/blob/74fd735eb073eb1d774b1ab4154a0876eb82f055/examples/dreambooth/train_dreambooth.py#L506).

`DDIMInverseScheduler` is the reverse scheduler of [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler).

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### scale_model_input[[diffusers.DDIMInverseScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, timestep: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddim_inverse.py#L250)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`int`, *optional*) : The current timestep in the diffusion chain.

**Returns:** `torch.Tensor`

A scaled input sample.

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

#### set_timesteps[[diffusers.DDIMInverseScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int, device: typing.Union[str, torch.device, NoneType] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddim_inverse.py#L267)

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.DDIMInverseScheduler.step]]

```python
step(model_output: Tensor, timestep: int, sample: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddim_inverse.py#L311)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~schedulers.scheduling_ddim_inverse.DDIMInverseSchedulerOutput` or `tuple`.

**Returns:** `~schedulers.scheduling_ddim_inverse.DDIMInverseSchedulerOutput` or `tuple`

If return_dict is `True`, `~schedulers.scheduling_ddim_inverse.DDIMInverseSchedulerOutput` is
returned, otherwise a tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).
