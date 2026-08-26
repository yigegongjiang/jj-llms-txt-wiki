# EDMEulerScheduler

The Karras formulation of the Euler scheduler (Algorithm 2) from the [Elucidating the Design Space of Diffusion-Based Generative Models](https://huggingface.co/papers/2206.00364) paper by Karras et al. This is a fast scheduler which can often generate good outputs in 20-30 steps. The scheduler is based on the original [k-diffusion](https://github.com/crowsonkb/k-diffusion/blob/481677d114f6ea445aa009cf5bd7a9cdee909e47/k_diffusion/sampling.py#L51) implementation by [Katherine Crowson](https://github.com/crowsonkb/).

## EDMEulerScheduler[[diffusers.EDMEulerScheduler]]

#### diffusers.EDMEulerScheduler[[diffusers.EDMEulerScheduler]]

```python
diffusers.EDMEulerScheduler(sigma_min: float = 0.002, sigma_max: float = 80.0, sigma_data: float = 0.5, sigma_schedule: typing.Literal['karras', 'exponential'] = 'karras', num_train_timesteps: int = 1000, prediction_type: typing.Literal['epsilon', 'v_prediction'] = 'epsilon', rho: float = 7.0, final_sigmas_type: typing.Literal['zero', 'sigma_min'] = 'zero')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L49)

**Parameters:**

sigma_min (`float`, *optional*, defaults to `0.002`) : Minimum noise magnitude in the sigma schedule. This was set to 0.002 in the EDM paper [1]; a reasonable range is [0, 10].

sigma_max (`float`, *optional*, defaults to `80.0`) : Maximum noise magnitude in the sigma schedule. This was set to 80.0 in the EDM paper [1]; a reasonable range is [0.2, 80.0].

sigma_data (`float`, *optional*, defaults to `0.5`) : The standard deviation of the data distribution. This is set to 0.5 in the EDM paper [1].

sigma_schedule (`Literal["karras", "exponential"]`, *optional*, defaults to `"karras"`) : Sigma schedule to compute the `sigmas`. By default, we use the schedule introduced in the EDM paper (https://huggingface.co/papers/2206.00364). The `"exponential"` schedule was incorporated in this model: https://huggingface.co/stabilityai/cosxl.

num_train_timesteps (`int`, *optional*, defaults to `1000`) : The number of diffusion steps to train the model.

prediction_type (`Literal["epsilon", "v_prediction"]`, *optional*, defaults to `"epsilon"`) : Prediction type of the scheduler function. `"epsilon"` predicts the noise of the diffusion process, and `"v_prediction"` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

rho (`float`, *optional*, defaults to `7.0`) : The rho parameter used for calculating the Karras sigma schedule, which is set to 7.0 in the EDM paper [1].

final_sigmas_type (`Literal["zero", "sigma_min"]`, *optional*, defaults to `"zero"`) : The final `sigma` value for the noise schedule during the sampling process. If `"sigma_min"`, the final sigma is the same as the last sigma in the training schedule. If `"zero"`, the final sigma is set to 0.

Implements the Euler scheduler in EDM formulation as presented in Karras et al. 2022 [1].

[1] Karras, Tero, et al. "Elucidating the Design Space of Diffusion-Based Generative Models."
https://huggingface.co/papers/2206.00364

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### add_noise[[diffusers.EDMEulerScheduler.add_noise]]

```python
add_noise(original_samples: Tensor, noise: Tensor, timesteps: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L536)

**Parameters:**

original_samples (`torch.Tensor`) : The original samples to which noise will be added.

noise (`torch.Tensor`) : The noise tensor to add to the original samples.

timesteps (`torch.Tensor`) : The timesteps at which to add noise, determining the noise level from the schedule.

**Returns:** `torch.Tensor`

The noisy samples with added noise scaled according to the timestep schedule.

Add noise to the original samples according to the noise schedule at the specified timesteps.

#### index_for_timestep[[diffusers.EDMEulerScheduler.index_for_timestep]]

```python
index_for_timestep(timestep: typing.Union[float, torch.Tensor], schedule_timesteps: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L381)

**Parameters:**

timestep (`float` or `torch.Tensor`) : The timestep value to find in the schedule.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:** `int`

The index of the timestep in the schedule. For the very first step, returns the second index if
multiple matches exist to avoid skipping a sigma when starting mid-schedule (e.g., for image-to-image).

Find the index of a given timestep in the timestep schedule.

#### precondition_inputs[[diffusers.EDMEulerScheduler.precondition_inputs]]

```python
precondition_inputs(sample: Tensor, sigma: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L177)

**Parameters:**

sample (`torch.Tensor`) : The input sample tensor to precondition.

sigma (`float` or `torch.Tensor`) : The current sigma (noise level) value.

**Returns:** `torch.Tensor`

The scaled input sample.

Precondition the input sample by scaling it according to the EDM formulation.

#### precondition_noise[[diffusers.EDMEulerScheduler.precondition_noise]]

```python
precondition_noise(sigma: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L195)

**Parameters:**

sigma (`float` or `torch.Tensor`) : The sigma (noise level) value to precondition.

**Returns:** `torch.Tensor`

The preconditioned noise value computed as `0.25 * log(sigma)`.

Precondition the noise level by applying a logarithmic transformation.

#### precondition_outputs[[diffusers.EDMEulerScheduler.precondition_outputs]]

```python
precondition_outputs(sample: Tensor, model_output: Tensor, sigma: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L214)

**Parameters:**

sample (`torch.Tensor`) : The input sample tensor.

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sigma (`float` or `torch.Tensor`) : The current sigma (noise level) value.

**Returns:** `torch.Tensor`

The denoised sample computed by combining the skip connection and output scaling.

Precondition the model outputs according to the EDM formulation.

#### scale_model_input[[diffusers.EDMEulerScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, timestep: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L249)

**Parameters:**

sample (`torch.Tensor`) : The input sample tensor.

timestep (`float` or `torch.Tensor`) : The current timestep in the diffusion chain.

**Returns:** `torch.Tensor`

A scaled input sample.

Scale the denoising model input to match the Euler algorithm. Ensures interchangeability with schedulers that
need to scale the denoising model input depending on the current timestep.

#### set_begin_index[[diffusers.EDMEulerScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L167)

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

#### set_timesteps[[diffusers.EDMEulerScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int = None, device: typing.Union[str, torch.device] = None, sigmas: typing.Union[list[float], torch.Tensor, NoneType] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L273)

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

sigmas (`torch.Tensor | list[float]`, *optional*) : Custom sigmas to use for the denoising process. If not defined, the default behavior when `num_inference_steps` is passed will be used.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.EDMEulerScheduler.step]]

```python
step(model_output: Tensor, timestep: typing.Union[float, torch.Tensor], sample: Tensor, s_churn: float = 0.0, s_tmin: float = 0.0, s_tmax: float = inf, s_noise: float = 1.0, generator: typing.Optional[torch.Generator] = None, return_dict: bool = True, pred_original_sample: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L427)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`float` or `torch.Tensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

s_churn (`float`, *optional*, defaults to `0.0`) : The amount of stochasticity to add at each step. Higher values add more noise.

s_tmin (`float`, *optional*, defaults to `0.0`) : The minimum sigma threshold below which no noise is added.

s_tmax (`float`, *optional*, defaults to `float("inf")`) : The maximum sigma threshold above which no noise is added.

s_noise (`float`, *optional*, defaults to `1.0`) : Scaling factor for noise added to the sample.

generator (`torch.Generator`, *optional*) : A random number generator for reproducibility.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return an [EDMEulerSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/edm_euler#diffusers.schedulers.scheduling_edm_euler.EDMEulerSchedulerOutput) or tuple.

pred_original_sample (`torch.Tensor`, *optional*) : The predicted denoised sample from a previous step. If provided, skips recomputation.

**Returns:** [EDMEulerSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/edm_euler#diffusers.schedulers.scheduling_edm_euler.EDMEulerSchedulerOutput) or `tuple`

If `return_dict` is `True`, an [EDMEulerSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/edm_euler#diffusers.schedulers.scheduling_edm_euler.EDMEulerSchedulerOutput) is
returned, otherwise a tuple is returned where the first element is the previous sample tensor and the
second element is the predicted original sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

## EDMEulerSchedulerOutput[[diffusers.schedulers.scheduling_edm_euler.EDMEulerSchedulerOutput]]

#### diffusers.schedulers.scheduling_edm_euler.EDMEulerSchedulerOutput[[diffusers.schedulers.scheduling_edm_euler.EDMEulerSchedulerOutput]]

```python
diffusers.schedulers.scheduling_edm_euler.EDMEulerSchedulerOutput(prev_sample: Tensor, pred_original_sample: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_euler.py#L32)

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

pred_original_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : The predicted denoised sample `(x_{0})` based on the model output from the current timestep. `pred_original_sample` can be used to preview progress or for guidance.

Output class for the scheduler's `step` function output.
