# HeunDiscreteScheduler

The Heun scheduler (Algorithm 1) is from the [Elucidating the Design Space of Diffusion-Based Generative Models](https://huggingface.co/papers/2206.00364) paper by Karras et al. The scheduler is ported from the [k-diffusion](https://github.com/crowsonkb/k-diffusion) library and created by [Katherine Crowson](https://github.com/crowsonkb/).

## HeunDiscreteScheduler[[diffusers.HeunDiscreteScheduler]]

#### diffusers.HeunDiscreteScheduler[[diffusers.HeunDiscreteScheduler]]

```python
diffusers.HeunDiscreteScheduler(num_train_timesteps: int = 1000, beta_start: float = 0.00085, beta_end: float = 0.012, beta_schedule: str = 'linear', trained_betas: numpy.ndarray | list[float] | None = None, prediction_type: str = 'epsilon', use_karras_sigmas: bool = False, use_exponential_sigmas: bool = False, use_beta_sigmas: bool = False, clip_sample: bool = False, clip_sample_range: float = 1.0, timestep_spacing: typing.Literal['linspace', 'leading', 'trailing'] = 'linspace', steps_offset: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_heun_discrete.py#L103)

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to 0.0001) : The starting `beta` value of inference.

beta_end (`float`, defaults to 0.02) : The final `beta` value.

beta_schedule (`"linear"`, `"scaled_linear"`, `"squaredcos_cap_v2"`, or `"exp"`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Choose from `linear`, `scaled_linear`, `squaredcos_cap_v2`, or `exp`.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

prediction_type (`"epsilon"`, `"sample"`, or `"v_prediction"`, defaults to `"epsilon"`, *optional*) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`) or `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

clip_sample (`bool`, defaults to `True`) : Clip the predicted sample for numerical stability.

clip_sample_range (`float`, defaults to 1.0) : The maximum magnitude for sample clipping. Valid only when `clip_sample=True`.

use_karras_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use Karras sigmas for step sizes in the noise schedule during the sampling process. If `True`, the sigmas are determined according to a sequence of noise levels {σi}.

use_exponential_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use exponential sigmas for step sizes in the noise schedule during the sampling process.

use_beta_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use beta sigmas for step sizes in the noise schedule during the sampling process. Refer to [Beta Sampling is All You Need](https://huggingface.co/papers/2407.12173) for more information.

timestep_spacing (`"linspace"`, `"leading"`, or `"trailing"`, defaults to `"linspace"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

steps_offset (`int`, defaults to 0) : An offset added to the inference steps, as required by some model families.

Scheduler with Heun steps for discrete beta schedules.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### add_noise[[diffusers.HeunDiscreteScheduler.add_noise]]

```python
add_noise(original_samples: Tensor, noise: Tensor, timesteps: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_heun_discrete.py#L662)

**Parameters:**

original_samples (`torch.Tensor`) : The original samples to which noise will be added.

noise (`torch.Tensor`) : The noise tensor to add to the original samples.

timesteps (`torch.Tensor`) : The timesteps at which to add noise, determining the noise level from the schedule.

**Returns:** `torch.Tensor`

The noisy samples with added noise scaled according to the timestep schedule.

Add noise to the original samples according to the noise schedule at the specified timesteps.

#### index_for_timestep[[diffusers.HeunDiscreteScheduler.index_for_timestep]]

```python
index_for_timestep(timestep: typing.Union[float, torch.Tensor], schedule_timesteps: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_heun_discrete.py#L198)

**Parameters:**

timestep (`float` or `torch.Tensor`) : The timestep value to find in the schedule.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:** `int`

The index of the timestep in the schedule. For the very first step, returns the second index if
multiple matches exist to avoid skipping a sigma when starting mid-schedule (e.g., for image-to-image).

Find the index of a given timestep in the timestep schedule.

#### scale_model_input[[diffusers.HeunDiscreteScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, timestep: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_heun_discrete.py#L261)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`float` or `torch.Tensor`) : The current timestep in the diffusion chain.

**Returns:** `torch.Tensor`

A scaled input sample.

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

#### set_begin_index[[diffusers.HeunDiscreteScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_heun_discrete.py#L251)

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

#### set_timesteps[[diffusers.HeunDiscreteScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int | None = None, device: typing.Union[str, torch.device] = None, num_train_timesteps: int | None = None, timesteps: list[int] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_heun_discrete.py#L287)

**Parameters:**

num_inference_steps (`int`, *optional*, defaults to `None`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str`, `torch.device`, *optional*, defaults to `None`) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

num_train_timesteps (`int`, *optional*, defaults to `None`) : The number of diffusion steps used when training the model. If `None`, the default `num_train_timesteps` attribute is used.

timesteps (`list[int]`, *optional*) : Custom timesteps used to support arbitrary spacing between timesteps. If `None`, timesteps will be generated based on the `timestep_spacing` attribute. If `timesteps` is passed, `num_inference_steps` must be `None`, and `timestep_spacing` attribute will be ignored.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.HeunDiscreteScheduler.step]]

```python
step(model_output: typing.Union[torch.Tensor, numpy.ndarray], timestep: typing.Union[float, torch.Tensor], sample: typing.Union[torch.Tensor, numpy.ndarray], return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_heun_discrete.py#L558)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`float`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

return_dict (`bool`) : Whether or not to return a `HeunDiscreteSchedulerOutput` or tuple.

**Returns:** `HeunDiscreteSchedulerOutput` or `tuple`

If return_dict is `True`, `HeunDiscreteSchedulerOutput` is
returned, otherwise a tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

## SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

#### diffusers.schedulers.scheduling_utils.SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

```python
diffusers.schedulers.scheduling_utils.SchedulerOutput(prev_sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_utils.py#L66)

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

Base class for the output of a scheduler's `step` function.
