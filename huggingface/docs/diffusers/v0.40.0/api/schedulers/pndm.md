# PNDMScheduler

`PNDMScheduler`, or pseudo numerical methods for diffusion models, uses more advanced ODE integration techniques like the Runge-Kutta and linear multi-step method. The original implementation can be found at [crowsonkb/k-diffusion](https://github.com/crowsonkb/k-diffusion/blob/481677d114f6ea445aa009cf5bd7a9cdee909e47/k_diffusion/sampling.py#L181).

## PNDMScheduler[[diffusers.PNDMScheduler]]

#### diffusers.PNDMScheduler[[diffusers.PNDMScheduler]]

```python
diffusers.PNDMScheduler(num_train_timesteps: int = 1000, beta_start: float = 0.0001, beta_end: float = 0.02, beta_schedule: str = 'linear', trained_betas: numpy.ndarray | list[float] | None = None, skip_prk_steps: bool = False, set_alpha_to_one: bool = False, prediction_type: typing.Literal['epsilon', 'v_prediction'] = 'epsilon', timestep_spacing: typing.Literal['linspace', 'leading', 'trailing'] = 'leading', steps_offset: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_pndm.py#L80)

**Parameters:**

num_train_timesteps (`int`, defaults to `1000`) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to `0.0001`) : The starting `beta` value of inference.

beta_end (`float`, defaults to `0.02`) : The final `beta` value.

beta_schedule (`"linear"`, `"scaled_linear"`, or `"squaredcos_cap_v2"`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

skip_prk_steps (`bool`, defaults to `False`) : Allows the scheduler to skip the Runge-Kutta steps defined in the original paper as being required before PLMS steps.

set_alpha_to_one (`bool`, defaults to `False`) : Each diffusion step uses the alphas product value at that step and at the previous one. For the final step there is no previous alpha. When this option is `True` the previous alpha product is fixed to `1`, otherwise it uses the alpha value at step 0.

prediction_type (`"epsilon"` or `"v_prediction"`, defaults to `"epsilon"`) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process) or `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

timestep_spacing (`"linspace"`, `"leading"`, or `"trailing"`, defaults to `"leading"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

steps_offset (`int`, defaults to `0`) : An offset added to the inference steps, as required by some model families.

`PNDMScheduler` uses pseudo numerical methods for diffusion models such as the Runge-Kutta and linear multi-step
method.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### add_noise[[diffusers.PNDMScheduler.add_noise]]

```python
add_noise(original_samples: Tensor, noise: Tensor, timesteps: IntTensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_pndm.py#L461)

**Parameters:**

original_samples (`torch.Tensor`) : The original samples to which noise will be added.

noise (`torch.Tensor`) : The noise to add to the samples.

timesteps (`torch.IntTensor`) : The timesteps indicating the noise level for each sample.

**Returns:** `torch.Tensor`

The noisy samples.

Add noise to the original samples according to the noise magnitude at each timestep (this is the forward
diffusion process).

#### scale_model_input[[diffusers.PNDMScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, *args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_pndm.py#L394)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

**Returns:** `torch.Tensor`

A scaled input sample.

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

#### set_timesteps[[diffusers.PNDMScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int, device: typing.Union[str, torch.device] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_pndm.py#L172)

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.PNDMScheduler.step]]

```python
step(model_output: Tensor, timestep: int, sample: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_pndm.py#L232)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

return_dict (`bool`, defaults to `True`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`.

**Returns:** [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`

If return_dict is `True`, [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise), and calls [step_prk()](/docs/diffusers/v0.40.0/en/api/schedulers/pndm#diffusers.PNDMScheduler.step_prk)
or [step_plms()](/docs/diffusers/v0.40.0/en/api/schedulers/pndm#diffusers.PNDMScheduler.step_plms) depending on the internal variable `counter`.

#### step_plms[[diffusers.PNDMScheduler.step_plms]]

```python
step_plms(model_output: Tensor, timestep: int, sample: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_pndm.py#L324)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

return_dict (`bool`, defaults to `True`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or tuple.

**Returns:** [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`

If return_dict is `True`, [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the sample with
the linear multistep method. It performs one forward pass multiple times to approximate the solution.

#### step_prk[[diffusers.PNDMScheduler.step_prk]]

```python
step_prk(model_output: Tensor, timestep: int, sample: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_pndm.py#L265)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

return_dict (`bool`, defaults to `True`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or tuple.

**Returns:** [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`

If return_dict is `True`, [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the sample with
the Runge-Kutta method. It performs four forward passes to approximate the solution to the differential
equation.

## SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

#### diffusers.schedulers.scheduling_utils.SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

```python
diffusers.schedulers.scheduling_utils.SchedulerOutput(prev_sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_utils.py#L66)

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

Base class for the output of a scheduler's `step` function.
