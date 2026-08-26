# KDPM2AncestralDiscreteScheduler

The `KDPM2DiscreteScheduler` with ancestral sampling is inspired by the [Elucidating the Design Space of Diffusion-Based Generative Models](https://huggingface.co/papers/2206.00364) paper, and the scheduler is ported from and created by [Katherine Crowson](https://github.com/crowsonkb/).

The original codebase can be found at [crowsonkb/k-diffusion](https://github.com/crowsonkb/k-diffusion).

## KDPM2AncestralDiscreteScheduler[[diffusers.KDPM2AncestralDiscreteScheduler]]

#### diffusers.KDPM2AncestralDiscreteScheduler[[diffusers.KDPM2AncestralDiscreteScheduler]]

```python
diffusers.KDPM2AncestralDiscreteScheduler(num_train_timesteps: int = 1000, beta_start: float = 0.00085, beta_end: float = 0.012, beta_schedule: str = 'linear', trained_betas: numpy.ndarray | list[float] | None = None, use_karras_sigmas: bool = False, use_exponential_sigmas: bool = False, use_beta_sigmas: bool = False, prediction_type: str = 'epsilon', timestep_spacing: str = 'linspace', steps_offset: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_k_dpm_2_ancestral_discrete.py#L104)

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to 0.00085) : The starting `beta` value of inference.

beta_end (`float`, defaults to 0.012) : The final `beta` value.

beta_schedule (`str`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Choose from `linear` or `scaled_linear`.

trained_betas (`np.ndarray` or `list[float]`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

use_karras_sigmas (`bool`, defaults to `False`) : Whether to use Karras sigmas for step sizes in the noise schedule during the sampling process. If `True`, the sigmas are determined according to a sequence of noise levels {σi}.

use_exponential_sigmas (`bool`, defaults to `False`) : Whether to use exponential sigmas for step sizes in the noise schedule during the sampling process.

use_beta_sigmas (`bool`, defaults to `False`) : Whether to use beta sigmas for step sizes in the noise schedule during the sampling process. Refer to [Beta Sampling is All You Need](https://huggingface.co/papers/2407.12173) for more information.

prediction_type (`str`, defaults to `"epsilon"`) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`) or `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

timestep_spacing (`str`, defaults to `"linspace"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

steps_offset (`int`, defaults to 0) : An offset added to the inference steps, as required by some model families.

KDPM2DiscreteScheduler with ancestral sampling is inspired by the DPMSolver2 and Algorithm 2 from the [Elucidating
the Design Space of Diffusion-Based Generative Models](https://huggingface.co/papers/2206.00364) paper.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### add_noise[[diffusers.KDPM2AncestralDiscreteScheduler.add_noise]]

```python
add_noise(original_samples: Tensor, noise: Tensor, timesteps: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_k_dpm_2_ancestral_discrete.py#L694)

**Parameters:**

original_samples (`torch.Tensor`) : The original samples to which noise will be added.

noise (`torch.Tensor`) : The noise tensor to add to the original samples.

timesteps (`torch.Tensor`) : The timesteps at which to add noise, determining the noise level from the schedule.

**Returns:** `torch.Tensor`

The noisy samples with added noise scaled according to the timestep schedule.

Add noise to the original samples according to the noise schedule at the specified timesteps.

#### index_for_timestep[[diffusers.KDPM2AncestralDiscreteScheduler.index_for_timestep]]

```python
index_for_timestep(timestep: typing.Union[float, torch.Tensor], schedule_timesteps: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_k_dpm_2_ancestral_discrete.py#L539)

**Parameters:**

timestep (`float` or `torch.Tensor`) : The timestep value to find in the schedule.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:** `int`

The index of the timestep in the schedule. For the very first step, returns the second index if
multiple matches exist to avoid skipping a sigma when starting mid-schedule (e.g., for image-to-image).

Find the index of a given timestep in the timestep schedule.

#### scale_model_input[[diffusers.KDPM2AncestralDiscreteScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, timestep: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_k_dpm_2_ancestral_discrete.py#L236)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`float` or `torch.Tensor`) : The current timestep in the diffusion chain.

**Returns:** `torch.Tensor`

A scaled input sample.

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

#### set_begin_index[[diffusers.KDPM2AncestralDiscreteScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_k_dpm_2_ancestral_discrete.py#L226)

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

#### set_timesteps[[diffusers.KDPM2AncestralDiscreteScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int, device: typing.Union[str, torch.device, NoneType] = None, num_train_timesteps: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_k_dpm_2_ancestral_discrete.py#L266)

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

num_train_timesteps (`int`, *optional*) : The number of diffusion steps used to train the model. If `None`, `self.config.num_train_timesteps` is used.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.KDPM2AncestralDiscreteScheduler.step]]

```python
step(model_output: typing.Union[torch.Tensor, numpy.ndarray], timestep: typing.Union[float, torch.Tensor], sample: typing.Union[torch.Tensor, numpy.ndarray], generator: typing.Optional[torch.Generator] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_k_dpm_2_ancestral_discrete.py#L585)

**Parameters:**

model_output (`torch.Tensor` or `np.ndarray`) : The direct output from learned diffusion model.

timestep (`float` or `torch.Tensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor` or `np.ndarray`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator.

return_dict (`bool`, defaults to `True`) : Whether or not to return a `KDPM2AncestralDiscreteSchedulerOutput` or tuple.

**Returns:**

`KDPM2AncestralDiscreteSchedulerOutput` or
`tuple[torch.Tensor, torch.Tensor]`:
If `return_dict` is `True`,
`KDPM2AncestralDiscreteSchedulerOutput` is
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
