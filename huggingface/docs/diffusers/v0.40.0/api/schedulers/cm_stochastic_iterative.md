# CMStochasticIterativeScheduler

[Consistency Models](https://huggingface.co/papers/2303.01469) by Yang Song, Prafulla Dhariwal, Mark Chen, and Ilya Sutskever introduced a multistep and onestep scheduler (Algorithm 1) that is capable of generating good samples in one or a small number of steps.

The abstract from the paper is:

*Diffusion models have significantly advanced the fields of image, audio, and video generation, but they depend on an iterative sampling process that causes slow generation. To overcome this limitation, we propose consistency models, a new family of models that generate high quality samples by directly mapping noise to data. They support fast one-step generation by design, while still allowing multistep sampling to trade compute for sample quality. They also support zero-shot data editing, such as image inpainting, colorization, and super-resolution, without requiring explicit training on these tasks. Consistency models can be trained either by distilling pre-trained diffusion models, or as standalone generative models altogether. Through extensive experiments, we demonstrate that they outperform existing distillation techniques for diffusion models in one- and few-step sampling, achieving the new state-of-the-art FID of 3.55 on CIFAR-10 and 6.20 on ImageNet 64x64 for one-step generation. When trained in isolation, consistency models become a new family of generative models that can outperform existing one-step, non-adversarial generative models on standard benchmarks such as CIFAR-10, ImageNet 64x64 and LSUN 256x256.*

The original codebase can be found at [openai/consistency_models](https://github.com/openai/consistency_models).

## CMStochasticIterativeScheduler[[diffusers.CMStochasticIterativeScheduler]]

#### diffusers.CMStochasticIterativeScheduler[[diffusers.CMStochasticIterativeScheduler]]

```python
diffusers.CMStochasticIterativeScheduler(num_train_timesteps: int = 40, sigma_min: float = 0.002, sigma_max: float = 80.0, sigma_data: float = 0.5, s_noise: float = 1.0, rho: float = 7.0, clip_denoised: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L43)

**Parameters:**

num_train_timesteps (`int`, defaults to 40) : The number of diffusion steps to train the model.

sigma_min (`float`, defaults to 0.002) : Minimum noise magnitude in the sigma schedule. Defaults to 0.002 from the original implementation.

sigma_max (`float`, defaults to 80.0) : Maximum noise magnitude in the sigma schedule. Defaults to 80.0 from the original implementation.

sigma_data (`float`, defaults to 0.5) : The standard deviation of the data distribution from the EDM [paper](https://huggingface.co/papers/2206.00364). Defaults to 0.5 from the original implementation.

s_noise (`float`, defaults to 1.0) : The amount of additional noise to counteract loss of detail during sampling. A reasonable range is [1.000, 1.011]. Defaults to 1.0 from the original implementation.

rho (`float`, defaults to 7.0) : The parameter for calculating the Karras sigma schedule from the EDM [paper](https://huggingface.co/papers/2206.00364). Defaults to 7.0 from the original implementation.

clip_denoised (`bool`, defaults to `True`) : Whether to clip the denoised outputs to `(-1, 1)`.

timesteps (`list` or `np.ndarray` or `torch.Tensor`, *optional*) : An explicit timestep schedule that can be optionally specified. The timesteps are expected to be in increasing order.

Multistep and onestep sampling for consistency models.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### add_noise[[diffusers.CMStochasticIterativeScheduler.add_noise]]

```python
add_noise(original_samples: Tensor, noise: Tensor, timesteps: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L461)

**Parameters:**

original_samples (`torch.Tensor`) : The original samples to which noise will be added.

noise (`torch.Tensor`) : The noise tensor to add to the original samples.

timesteps (`torch.Tensor`) : The timesteps at which to add noise, determining the noise level from the schedule.

**Returns:** `torch.Tensor`

The noisy samples with added noise scaled according to the timestep schedule.

Add noise to the original samples according to the noise schedule at the specified timesteps.

#### get_scalings[[diffusers.CMStochasticIterativeScheduler.get_scalings]]

```python
get_scalings(sigma: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L276)

**Parameters:**

sigma (`torch.Tensor`) : The current sigma value in the noise schedule.

**Returns:** `tuple[torch.Tensor, torch.Tensor]`

A tuple containing `c_skip` (scaling for the input sample) and `c_out` (scaling for the model output).

Computes the scaling factors for the consistency model output.

#### get_scalings_for_boundary_condition[[diffusers.CMStochasticIterativeScheduler.get_scalings_for_boundary_condition]]

```python
get_scalings_for_boundary_condition(sigma: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L294)

**Parameters:**

sigma (`torch.Tensor`) : The current sigma in the Karras sigma schedule.

**Returns:** `tuple[torch.Tensor, torch.Tensor]`

A two-element tuple where `c_skip` (which weights the current sample) is the first element and `c_out`
(which weights the consistency model output) is the second element.

Gets the scalings used in the consistency model parameterization (from Appendix C of the
[paper](https://huggingface.co/papers/2303.01469)) to enforce boundary condition.

> [!TIP] > `epsilon` in the equations for `c_skip` and `c_out` is set to `sigma_min`.

#### index_for_timestep[[diffusers.CMStochasticIterativeScheduler.index_for_timestep]]

```python
index_for_timestep(timestep: typing.Union[float, torch.Tensor], schedule_timesteps: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L318)

**Parameters:**

timestep (`float` or `torch.Tensor`) : The timestep value to find in the schedule.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:** `int`

The index of the timestep in the schedule. For the very first step, returns the second index if
multiple matches exist to avoid skipping a sigma when starting mid-schedule (e.g., for image-to-image).

Find the index of a given timestep in the timestep schedule.

#### scale_model_input[[diffusers.CMStochasticIterativeScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, timestep: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L136)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`float` or `torch.Tensor`) : The current timestep in the diffusion chain.

**Returns:** `torch.Tensor`

A scaled input sample.

Scales the consistency model input by `(sigma**2 + sigma_data**2) ** 0.5`.

#### set_begin_index[[diffusers.CMStochasticIterativeScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L126)

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

#### set_timesteps[[diffusers.CMStochasticIterativeScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int | None = None, device: typing.Union[str, torch.device] = None, timesteps: list[int] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L180)

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

timesteps (`list[int]`, *optional*) : Custom timesteps used to support arbitrary spacing between timesteps. If `None`, then the default timestep spacing strategy of equal spacing between timesteps is used. If `timesteps` is passed, `num_inference_steps` must be `None`.

Sets the timesteps used for the diffusion chain (to be run before inference).

#### sigma_to_t[[diffusers.CMStochasticIterativeScheduler.sigma_to_t]]

```python
sigma_to_t(sigmas: float | numpy.ndarray)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L161)

**Parameters:**

sigmas (`float` or `np.ndarray`) : A single Karras sigma or an array of Karras sigmas.

**Returns:** `np.ndarray`

A scaled input timestep array.

Gets scaled timesteps from the Karras sigmas for input to the consistency model.

#### step[[diffusers.CMStochasticIterativeScheduler.step]]

```python
step(model_output: Tensor, timestep: typing.Union[float, torch.Tensor], sample: Tensor, generator: typing.Optional[torch.Generator] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L364)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`float` or `torch.Tensor`) : The current timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator.

return_dict (`bool`, defaults to `True`) : Whether or not to return a [CMStochasticIterativeSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/cm_stochastic_iterative#diffusers.schedulers.scheduling_consistency_models.CMStochasticIterativeSchedulerOutput) or `tuple`.

**Returns:** [CMStochasticIterativeSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/cm_stochastic_iterative#diffusers.schedulers.scheduling_consistency_models.CMStochasticIterativeSchedulerOutput) or `tuple`

If return_dict is `True`,
[CMStochasticIterativeSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/cm_stochastic_iterative#diffusers.schedulers.scheduling_consistency_models.CMStochasticIterativeSchedulerOutput) is returned,
otherwise a tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

## CMStochasticIterativeSchedulerOutput[[diffusers.schedulers.scheduling_consistency_models.CMStochasticIterativeSchedulerOutput]]

#### diffusers.schedulers.scheduling_consistency_models.CMStochasticIterativeSchedulerOutput[[diffusers.schedulers.scheduling_consistency_models.CMStochasticIterativeSchedulerOutput]]

```python
diffusers.schedulers.scheduling_consistency_models.CMStochasticIterativeSchedulerOutput(prev_sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_models.py#L30)

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

Output class for the scheduler's `step` function.
