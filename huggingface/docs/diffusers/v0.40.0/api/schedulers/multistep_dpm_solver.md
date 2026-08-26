# DPMSolverMultistepScheduler

`DPMSolverMultistepScheduler` is a multistep scheduler from [DPM-Solver: A Fast ODE Solver for Diffusion Probabilistic Model Sampling in Around 10 Steps](https://huggingface.co/papers/2206.00927) and [DPM-Solver++: Fast Solver for Guided Sampling of Diffusion Probabilistic Models](https://huggingface.co/papers/2211.01095) by Cheng Lu, Yuhao Zhou, Fan Bao, Jianfei Chen, Chongxuan Li, and Jun Zhu.

DPMSolver (and the improved version DPMSolver++) is a fast dedicated high-order solver for diffusion ODEs with convergence order guarantee. Empirically, DPMSolver sampling with only 20 steps can generate high-quality
samples, and it can generate quite good samples even in 10 steps.

## Tips

It is recommended to set `solver_order` to 2 for guide sampling, and `solver_order=3` for unconditional sampling.

Dynamic thresholding from [Imagen](https://huggingface.co/papers/2205.11487) is supported, and for pixel-space
diffusion models, you can set both `algorithm_type="dpmsolver++"` and `thresholding=True` to use the dynamic
thresholding. This thresholding method is unsuitable for latent-space diffusion models such as
Stable Diffusion.

The SDE variant of DPMSolver and DPM-Solver++ is also supported, but only for the first and second-order solvers. This is a fast SDE solver for the reverse diffusion SDE. It is recommended to use the second-order `sde-dpmsolver++`.

## DPMSolverMultistepScheduler[[diffusers.DPMSolverMultistepScheduler]]

#### diffusers.DPMSolverMultistepScheduler[[diffusers.DPMSolverMultistepScheduler]]

```python
diffusers.DPMSolverMultistepScheduler(num_train_timesteps: int = 1000, beta_start: float = 0.0001, beta_end: float = 0.02, beta_schedule: str = 'linear', trained_betas: numpy.ndarray | list[float] | None = None, solver_order: int = 2, prediction_type: typing.Literal['epsilon', 'sample', 'v_prediction', 'flow_prediction'] = 'epsilon', thresholding: bool = False, dynamic_thresholding_ratio: float = 0.995, sample_max_value: float = 1.0, algorithm_type: typing.Literal['dpmsolver', 'dpmsolver++', 'sde-dpmsolver', 'sde-dpmsolver++'] = 'dpmsolver++', solver_type: typing.Literal['midpoint', 'heun'] = 'midpoint', lower_order_final: bool = True, euler_at_final: bool = False, use_karras_sigmas: bool = False, use_exponential_sigmas: bool = False, use_beta_sigmas: bool = False, use_lu_lambdas: bool = False, use_flow_sigmas: bool = False, flow_shift: float = 1.0, final_sigmas_type: typing.Literal['zero', 'sigma_min'] = 'zero', lambda_min_clipped: float = -inf, variance_type: typing.Optional[typing.Literal['learned', 'learned_range']] = None, timestep_spacing: typing.Literal['linspace', 'leading', 'trailing'] = 'linspace', steps_offset: int = 0, rescale_betas_zero_snr: bool = False, use_dynamic_shifting: bool = False, time_shift_type: typing.Literal['exponential'] = 'exponential')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L123)

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to 0.0001) : The starting `beta` value of inference.

beta_end (`float`, defaults to 0.02) : The final `beta` value.

beta_schedule (`"linear"`, `"scaled_linear"`, or `"squaredcos_cap_v2"`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

solver_order (`int`, defaults to 2) : The DPMSolver order which can be `1` or `2` or `3`. It is recommended to use `solver_order=2` for guided sampling, and `solver_order=3` for unconditional sampling.

prediction_type (`"epsilon"`, `"sample"`, `"v_prediction"`, or `"flow_prediction"`, defaults to `"epsilon"`) : Prediction type of the scheduler function. `epsilon` predicts the noise of the diffusion process, `sample` directly predicts the noisy sample, `v_prediction` predicts the velocity (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper), and `flow_prediction` predicts the flow.

thresholding (`bool`, defaults to `False`) : Whether to use the "dynamic thresholding" method. This is unsuitable for latent-space diffusion models such as Stable Diffusion.

dynamic_thresholding_ratio (`float`, defaults to 0.995) : The ratio for the dynamic thresholding method. Valid only when `thresholding=True`.

sample_max_value (`float`, defaults to 1.0) : The threshold value for dynamic thresholding. Valid only when `thresholding=True` and `algorithm_type="dpmsolver++"`.

algorithm_type (`"dpmsolver"`, `"dpmsolver++"`, `"sde-dpmsolver"`, or `"sde-dpmsolver++"`, defaults to `"dpmsolver++"`) : Algorithm type for the solver. The `dpmsolver` type implements the algorithms in the [DPMSolver](https://huggingface.co/papers/2206.00927) paper, and the `dpmsolver++` type implements the algorithms in the [DPMSolver++](https://huggingface.co/papers/2211.01095) paper. It is recommended to use `dpmsolver++` or `sde-dpmsolver++` with `solver_order=2` for guided sampling like in Stable Diffusion.

solver_type (`"midpoint"` or `"heun"`, defaults to `"midpoint"`) : Solver type for the second-order solver. The solver type slightly affects the sample quality, especially for a small number of steps. It is recommended to use `midpoint` solvers.

lower_order_final (`bool`, defaults to `True`) : Whether to use lower-order solvers in the final steps. Only valid for < 15 inference steps. This can stabilize the sampling of DPMSolver for steps < 15, especially for steps <= 10.

euler_at_final (`bool`, defaults to `False`) : Whether to use Euler's method in the final step. It is a trade-off between numerical stability and detail richness. This can stabilize the sampling of the SDE variant of DPMSolver for small number of inference steps, but sometimes may result in blurring.

use_karras_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use Karras sigmas for step sizes in the noise schedule during the sampling process. If `True`, the sigmas are determined according to a sequence of noise levels {σi}.

use_exponential_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use exponential sigmas for step sizes in the noise schedule during the sampling process.

use_beta_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use beta sigmas for step sizes in the noise schedule during the sampling process. Refer to [Beta Sampling is All You Need](https://huggingface.co/papers/2407.12173) for more information.

use_lu_lambdas (`bool`, *optional*, defaults to `False`) : Whether to use the uniform-logSNR for step sizes proposed by Lu's DPM-Solver in the noise schedule during the sampling process. If `True`, the sigmas and time steps are determined according to a sequence of `lambda(t)`.

use_flow_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use flow sigmas for step sizes in the noise schedule during the sampling process.

flow_shift (`float`, *optional*, defaults to 1.0) : The shift value for the timestep schedule for flow matching.

final_sigmas_type (`"zero"` or `"sigma_min"`, *optional*, defaults to `"zero"`) : The final `sigma` value for the noise schedule during the sampling process. If `"sigma_min"`, the final sigma is the same as the last sigma in the training schedule. If `"zero"`, the final sigma is set to 0.

lambda_min_clipped (`float`, defaults to `-inf`) : Clipping threshold for the minimum value of `lambda(t)` for numerical stability. This is critical for the cosine (`squaredcos_cap_v2`) noise schedule.

variance_type (`"learned"` or `"learned_range"`, *optional*) : Set to `"learned"` or `"learned_range"` for diffusion models that predict variance. If set, the model's output contains the predicted Gaussian variance.

timestep_spacing (`"linspace"`, `"leading"`, or `"trailing"`, defaults to `"linspace"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

steps_offset (`int`, defaults to 0) : An offset added to the inference steps, as required by some model families.

rescale_betas_zero_snr (`bool`, defaults to `False`) : Whether to rescale the betas to have zero terminal SNR. This enables the model to generate very bright and dark samples instead of limiting it to samples with medium brightness. Loosely related to [`--offset_noise`](https://github.com/huggingface/diffusers/blob/74fd735eb073eb1d774b1ab4154a0876eb82f055/examples/dreambooth/train_dreambooth.py#L506).

use_dynamic_shifting (`bool`, defaults to `False`) : Whether to use dynamic shifting for the timestep schedule.

time_shift_type (`"exponential"`, defaults to `"exponential"`) : The type of time shift to apply when using dynamic shifting.

`DPMSolverMultistepScheduler` is a fast dedicated high-order solver for diffusion ODEs.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### add_noise[[diffusers.DPMSolverMultistepScheduler.add_noise]]

```python
add_noise(original_samples: Tensor, noise: Tensor, timesteps: IntTensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L1303)

**Parameters:**

original_samples (`torch.Tensor`) : The original samples without noise.

noise (`torch.Tensor`) : The noise to add to the samples.

timesteps (`torch.IntTensor`) : The timesteps at which to add noise to the samples.

**Returns:** `torch.Tensor`

The noisy samples.

Add noise to the original samples according to the noise schedule at the specified timesteps.

#### convert_model_output[[diffusers.DPMSolverMultistepScheduler.convert_model_output]]

```python
convert_model_output(model_output: Tensor, *args, sample: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L749)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sample (`torch.Tensor`, *optional*) : A current instance of a sample created by the diffusion process.

**Returns:** `torch.Tensor`

The converted model output.

Convert the model output to the corresponding type the DPMSolver/DPMSolver++ algorithm needs. DPM-Solver is
designed to discretize an integral of the noise prediction model, and DPM-Solver++ is designed to discretize an
integral of the data prediction model.

> [!TIP] > The algorithm and model type are decoupled. You can use either DPMSolver or DPMSolver++ for both
noise > prediction and data prediction models.

#### dpm_solver_first_order_update[[diffusers.DPMSolverMultistepScheduler.dpm_solver_first_order_update]]

```python
dpm_solver_first_order_update(model_output: Tensor, *args, sample: typing.Optional[torch.Tensor] = None, noise: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L847)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sample (`torch.Tensor`, *optional*) : A current instance of a sample created by the diffusion process.

noise (`torch.Tensor`, *optional*) : The noise tensor.

**Returns:** `torch.Tensor`

The sample tensor at the previous timestep.

One step for the first-order DPMSolver (equivalent to DDIM).

#### index_for_timestep[[diffusers.DPMSolverMultistepScheduler.index_for_timestep]]

```python
index_for_timestep(timestep: typing.Union[int, torch.Tensor], schedule_timesteps: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L1144)

**Parameters:**

timestep (`int` or `torch.Tensor`) : The timestep for which to find the index.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:** `int`

The index of the timestep in the schedule.

Find the index for a given timestep in the schedule.

#### multistep_dpm_solver_second_order_update[[diffusers.DPMSolverMultistepScheduler.multistep_dpm_solver_second_order_update]]

```python
multistep_dpm_solver_second_order_update(model_output_list: list, *args, sample: typing.Optional[torch.Tensor] = None, noise: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L921)

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

sample (`torch.Tensor`, *optional*) : A current instance of a sample created by the diffusion process.

noise (`torch.Tensor`, *optional*) : Random noise used by the stochastic (`sde-*`) solver variants. Required when `algorithm_type` is set to one of them, and unused otherwise.

**Returns:** `torch.Tensor`

The sample tensor at the previous timestep.

One step for the second-order multistep DPMSolver.

#### multistep_dpm_solver_third_order_update[[diffusers.DPMSolverMultistepScheduler.multistep_dpm_solver_third_order_update]]

```python
multistep_dpm_solver_third_order_update(model_output_list: list, *args, sample: typing.Optional[torch.Tensor] = None, noise: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L1047)

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

sample (`torch.Tensor`, *optional*) : A current instance of a sample created by diffusion process.

noise (`torch.Tensor`, *optional*) : The noise tensor.

**Returns:** `torch.Tensor`

The sample tensor at the previous timestep.

One step for the third-order multistep DPMSolver.

#### scale_model_input[[diffusers.DPMSolverMultistepScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, *args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L1288)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

**Returns:** `torch.Tensor`

A scaled input sample.

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

#### set_begin_index[[diffusers.DPMSolverMultistepScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L357)

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

#### set_timesteps[[diffusers.DPMSolverMultistepScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int = None, device: typing.Union[str, torch.device] = None, mu: float | None = None, timesteps: list[int] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L367)

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

mu (`float`, *optional*) : Exponent for the dynamic time shift. Requires `use_dynamic_shifting` and a `time_shift_type` of `"exponential"`; when passed, `flow_shift` is set to `exp(mu)`.

timesteps (`list[int]`, *optional*) : Custom timesteps used to support arbitrary timesteps schedule. If `None`, timesteps will be generated based on the `timestep_spacing` attribute. If `timesteps` is passed, `num_inference_steps` and `sigmas` must be `None`, and `timestep_spacing` attribute will be ignored.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.DPMSolverMultistepScheduler.step]]

```python
step(model_output: Tensor, timestep: typing.Union[int, torch.Tensor], sample: Tensor, generator: typing.Optional[torch.Generator] = None, variance_noise: typing.Optional[torch.Tensor] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_dpmsolver_multistep.py#L1196)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`int` or `torch.Tensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator.

variance_noise (`torch.Tensor`, *optional*) : Alternative to generating noise with `generator` by directly providing the noise for the variance itself. Useful for methods such as `LEdits++`.

return_dict (`bool`, defaults to `True`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`.

**Returns:** [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`

If `return_dict` is `True`, [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) is returned, otherwise a
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
