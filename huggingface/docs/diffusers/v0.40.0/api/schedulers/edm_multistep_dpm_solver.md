# EDMDPMSolverMultistepScheduler

`EDMDPMSolverMultistepScheduler` is a [Karras formulation](https://huggingface.co/papers/2206.00364) of `DPMSolverMultistepScheduler`, a multistep scheduler from [DPM-Solver: A Fast ODE Solver for Diffusion Probabilistic Model Sampling in Around 10 Steps](https://huggingface.co/papers/2206.00927) and [DPM-Solver++: Fast Solver for Guided Sampling of Diffusion Probabilistic Models](https://huggingface.co/papers/2211.01095) by Cheng Lu, Yuhao Zhou, Fan Bao, Jianfei Chen, Chongxuan Li, and Jun Zhu.

DPMSolver (and the improved version DPMSolver++) is a fast dedicated high-order solver for diffusion ODEs with convergence order guarantee. Empirically, DPMSolver sampling with only 20 steps can generate high-quality
samples, and it can generate quite good samples even in 10 steps.

## EDMDPMSolverMultistepScheduler[[diffusers.EDMDPMSolverMultistepScheduler]]

#### diffusers.EDMDPMSolverMultistepScheduler[[diffusers.EDMDPMSolverMultistepScheduler]]

```python
diffusers.EDMDPMSolverMultistepScheduler(sigma_min: float = 0.002, sigma_max: float = 80.0, sigma_data: float = 0.5, sigma_schedule: typing.Literal['karras', 'exponential'] = 'karras', num_train_timesteps: int = 1000, prediction_type: typing.Literal['epsilon', 'sample', 'v_prediction'] = 'epsilon', rho: float = 7.0, solver_order: int = 2, thresholding: bool = False, dynamic_thresholding_ratio: float = 0.995, sample_max_value: float = 1.0, algorithm_type: typing.Literal['dpmsolver++', 'sde-dpmsolver++'] = 'dpmsolver++', solver_type: typing.Literal['midpoint', 'heun'] = 'midpoint', lower_order_final: bool = True, euler_at_final: bool = False, final_sigmas_type: typing.Literal['zero', 'sigma_min'] = 'zero')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L28)

**Parameters:**

sigma_min (`float`, *optional*, defaults to 0.002) : Minimum noise magnitude in the sigma schedule. This was set to 0.002 in the EDM paper [1]; a reasonable range is [0, 10].

sigma_max (`float`, *optional*, defaults to 80.0) : Maximum noise magnitude in the sigma schedule. This was set to 80.0 in the EDM paper [1]; a reasonable range is [0.2, 80.0].

sigma_data (`float`, *optional*, defaults to 0.5) : The standard deviation of the data distribution. This is set to 0.5 in the EDM paper [1].

sigma_schedule (`str`, *optional*, defaults to `karras`) : Sigma schedule to compute the `sigmas`. By default, we the schedule introduced in the EDM paper (https://huggingface.co/papers/2206.00364). Other acceptable value is "exponential". The exponential schedule was incorporated in this model: https://huggingface.co/stabilityai/cosxl.

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

prediction_type (`str`, defaults to `epsilon`, *optional*) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`) or `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

rho (`float`, *optional*, defaults to 7.0) : The rho parameter in the Karras sigma schedule. This was set to 7.0 in the EDM paper [1].

solver_order (`int`, defaults to 2) : The DPMSolver order which can be `1` or `2` or `3`. It is recommended to use `solver_order=2` for guided sampling, and `solver_order=3` for unconditional sampling.

thresholding (`bool`, defaults to `False`) : Whether to use the "dynamic thresholding" method. This is unsuitable for latent-space diffusion models such as Stable Diffusion.

dynamic_thresholding_ratio (`float`, defaults to 0.995) : The ratio for the dynamic thresholding method. Valid only when `thresholding=True`.

sample_max_value (`float`, defaults to 1.0) : The threshold value for dynamic thresholding. Valid only when `thresholding=True` and `algorithm_type="dpmsolver++"`.

algorithm_type (`str`, defaults to `dpmsolver++`) : Algorithm type for the solver; can be `dpmsolver++` or `sde-dpmsolver++`. The `dpmsolver++` type implements the algorithms in the [DPMSolver++](https://huggingface.co/papers/2211.01095) paper. It is recommended to use `dpmsolver++` or `sde-dpmsolver++` with `solver_order=2` for guided sampling like in Stable Diffusion.

solver_type (`str`, defaults to `midpoint`) : Solver type for the second-order solver; can be `midpoint` or `heun`. The solver type slightly affects the sample quality, especially for a small number of steps. It is recommended to use `midpoint` solvers.

lower_order_final (`bool`, defaults to `True`) : Whether to use lower-order solvers in the final steps. Only valid for < 15 inference steps. This can stabilize the sampling of DPMSolver for steps < 15, especially for steps <= 10.

euler_at_final (`bool`, defaults to `False`) : Whether to use Euler's method in the final step. It is a trade-off between numerical stability and detail richness. This can stabilize the sampling of the SDE variant of DPMSolver for small number of inference steps, but sometimes may result in blurring.

final_sigmas_type (`str`, defaults to `"zero"`) : The final `sigma` value for the noise schedule during the sampling process. If `"sigma_min"`, the final sigma is the same as the last sigma in the training schedule. If `zero`, the final sigma is set to 0.

Implements DPMSolverMultistepScheduler in EDM formulation as presented in Karras et al. 2022 [1].
`EDMDPMSolverMultistepScheduler` is a fast dedicated high-order solver for diffusion ODEs.

[1] Karras, Tero, et al. "Elucidating the Design Space of Diffusion-Based Generative Models."
https://huggingface.co/papers/2206.00364

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### add_noise[[diffusers.EDMDPMSolverMultistepScheduler.add_noise]]

```python
add_noise(original_samples: Tensor, noise: Tensor, timesteps: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L808)

**Parameters:**

original_samples (`torch.Tensor`) : The original samples to which noise will be added.

noise (`torch.Tensor`) : The noise tensor to add to the original samples.

timesteps (`torch.Tensor`) : The timesteps at which to add noise, determining the noise level from the schedule.

**Returns:** `torch.Tensor`

The noisy samples with added noise scaled according to the timestep schedule.

Add noise to the original samples according to the noise schedule at the specified timesteps.

#### convert_model_output[[diffusers.EDMDPMSolverMultistepScheduler.convert_model_output]]

```python
convert_model_output(model_output: Tensor, sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L471)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:** `torch.Tensor`

The converted model output.

Convert the model output to the corresponding type the DPMSolver/DPMSolver++ algorithm needs. DPM-Solver is
designed to discretize an integral of the noise prediction model, and DPM-Solver++ is designed to discretize an
integral of the data prediction model.

> [!TIP] > The algorithm and model type are decoupled. You can use either DPMSolver or DPMSolver++ for both
noise > prediction and data prediction models.

#### dpm_solver_first_order_update[[diffusers.EDMDPMSolverMultistepScheduler.dpm_solver_first_order_update]]

```python
dpm_solver_first_order_update(model_output: Tensor, sample: Tensor, noise: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L502)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

noise (`torch.Tensor`, *optional*) : The noise tensor to add to the original samples.

**Returns:** `torch.Tensor`

The sample tensor at the previous timestep.

One step for the first-order DPMSolver (equivalent to DDIM).

#### index_for_timestep[[diffusers.EDMDPMSolverMultistepScheduler.index_for_timestep]]

```python
index_for_timestep(timestep: typing.Union[int, torch.Tensor], schedule_timesteps: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L673)

**Parameters:**

timestep (`int` or `torch.Tensor`) : The timestep for which to find the index.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:** `int`

The index of the timestep in the schedule.

Find the index for a given timestep in the schedule.

#### multistep_dpm_solver_second_order_update[[diffusers.EDMDPMSolverMultistepScheduler.multistep_dpm_solver_second_order_update]]

```python
multistep_dpm_solver_second_order_update(model_output_list: typing.List[torch.Tensor], sample: Tensor, noise: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L545)

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

noise (`torch.Tensor`, *optional*) : The noise tensor to add to the original samples.

**Returns:** `torch.Tensor`

The sample tensor at the previous timestep.

One step for the second-order multistep DPMSolver.

#### multistep_dpm_solver_third_order_update[[diffusers.EDMDPMSolverMultistepScheduler.multistep_dpm_solver_third_order_update]]

```python
multistep_dpm_solver_third_order_update(model_output_list: list, sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L618)

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

sample (`torch.Tensor`) : A current instance of a sample created by diffusion process.

**Returns:** `torch.Tensor`

The sample tensor at the previous timestep.

One step for the third-order multistep DPMSolver.

#### precondition_inputs[[diffusers.EDMDPMSolverMultistepScheduler.precondition_inputs]]

```python
precondition_inputs(sample: Tensor, sigma: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L180)

**Parameters:**

sample (`torch.Tensor`) : The input sample tensor to precondition.

sigma (`float` or `torch.Tensor`) : The current sigma (noise level) value.

**Returns:** `torch.Tensor`

The scaled input sample.

Precondition the input sample by scaling it according to the EDM formulation.

#### precondition_noise[[diffusers.EDMDPMSolverMultistepScheduler.precondition_noise]]

```python
precondition_noise(sigma: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L199)

**Parameters:**

sigma (`float` or `torch.Tensor`) : The sigma (noise level) value to precondition.

**Returns:** `torch.Tensor`

The preconditioned noise value computed as `0.25 * log(sigma)`.

Precondition the noise level by applying a logarithmic transformation.

#### precondition_outputs[[diffusers.EDMDPMSolverMultistepScheduler.precondition_outputs]]

```python
precondition_outputs(sample: Tensor, model_output: Tensor, sigma: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L219)

**Parameters:**

sample (`torch.Tensor`) : The input sample tensor.

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sigma (`float` or `torch.Tensor`) : The current sigma (noise level) value.

**Returns:** `torch.Tensor`

The denoised sample computed by combining the skip connection and output scaling.

Precondition the model outputs according to the EDM formulation.

#### scale_model_input[[diffusers.EDMDPMSolverMultistepScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, timestep: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L255)

**Parameters:**

sample (`torch.Tensor`) : The input sample tensor.

timestep (`float` or `torch.Tensor`) : The current timestep in the diffusion chain.

**Returns:** `torch.Tensor`

A scaled input sample.

Scale the denoising model input to match the Euler algorithm. Ensures interchangeability with schedulers that
need to scale the denoising model input depending on the current timestep.

#### set_begin_index[[diffusers.EDMDPMSolverMultistepScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L169)

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

#### set_timesteps[[diffusers.EDMDPMSolverMultistepScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int = None, device: typing.Union[str, torch.device, NoneType] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L279)

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.EDMDPMSolverMultistepScheduler.step]]

```python
step(model_output: Tensor, timestep: typing.Union[int, torch.Tensor], sample: Tensor, generator: typing.Optional[torch.Generator] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_edm_dpmsolver_multistep.py#L726)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator.

return_dict (`bool`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`.

**Returns:** [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`

If return_dict is `True`, [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the sample with
the multistep DPMSolver.

## SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

#### diffusers.schedulers.scheduling_utils.SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

```python
diffusers.schedulers.scheduling_utils.SchedulerOutput(prev_sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_utils.py#L66)

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

Base class for the output of a scheduler's `step` function.
