# CosineDPMSolverMultistepScheduler

The [CosineDPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/cosine_dpm#diffusers.CosineDPMSolverMultistepScheduler) is a variant of [DPMSolverMultistepScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler) with cosine schedule, proposed by Nichol and Dhariwal (2021).
It is being used in the [Stable Audio Open](https://huggingface.co/papers/2407.14358) paper and the [Stability-AI/stable-audio-tool](https://github.com/Stability-AI/stable-audio-tools) codebase.

This scheduler was contributed by [Yoach Lacombe](https://huggingface.co/ylacombe).

## CosineDPMSolverMultistepScheduler[[diffusers.CosineDPMSolverMultistepScheduler]]
#### diffusers.CosineDPMSolverMultistepScheduler[[diffusers.CosineDPMSolverMultistepScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L28)

Implements a variant of `DPMSolverMultistepScheduler` with cosine schedule, proposed by Nichol and Dhariwal (2021).
This scheduler was used in Stable Audio Open [1].

[1] Evans, Parker, et al. "Stable Audio Open" https://huggingface.co/papers/2407.14358

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

add_noisediffusers.CosineDPMSolverMultistepScheduler.add_noisehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L683[{"name": "original_samples", "val": ": Tensor"}, {"name": "noise", "val": ": Tensor"}, {"name": "timesteps", "val": ": Tensor"}]- **original_samples** (`torch.Tensor`) --
  The original samples to which noise will be added.
- **noise** (`torch.Tensor`) --
  The noise tensor to add to the original samples.
- **timesteps** (`torch.Tensor`) --
  The timesteps at which to add noise, determining the noise level from the schedule.0`torch.Tensor`The noisy samples with added noise scaled according to the timestep schedule.

Add noise to the original samples according to the noise schedule at the specified timesteps.

**Parameters:**

sigma_min (`float`, defaults to `0.3`) : Minimum noise magnitude in the sigma schedule. This was set to 0.3 in Stable Audio Open [1].

sigma_max (`float`, defaults to `500`) : Maximum noise magnitude in the sigma schedule. This was set to 500 in Stable Audio Open [1].

sigma_data (`float`, defaults to `1.0`) : The standard deviation of the data distribution. This is set to 1.0 in Stable Audio Open [1].

sigma_schedule (`str`, defaults to `"exponential"`) : Sigma schedule to compute the `sigmas`. Must be one of `"exponential"` or `"karras"`. The exponential schedule was incorporated in [stabilityai/cosxl](https://huggingface.co/stabilityai/cosxl). The Karras schedule is introduced in the [EDM](https://huggingface.co/papers/2206.00364) paper.

num_train_timesteps (`int`, defaults to `1000`) : The number of diffusion steps to train the model.

solver_order (`int`, defaults to `2`) : The DPMSolver order which can be `1` or `2`. It is recommended to use `solver_order=2`.

prediction_type (`str`, defaults to `"v_prediction"`) : Prediction type of the scheduler function. Must be one of `"epsilon"` (predicts the noise of the diffusion process), `"sample"` (directly predicts the noisy sample), or `"v_prediction"` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

rho (`float`, defaults to `7.0`) : The parameter for calculating the Karras sigma schedule from the EDM [paper](https://huggingface.co/papers/2206.00364).

solver_type (`str`, defaults to `"midpoint"`) : Solver type for the second-order solver. Must be one of `"midpoint"` or `"heun"`. The solver type slightly affects the sample quality, especially for a small number of steps. It is recommended to use `"midpoint"`.

lower_order_final (`bool`, defaults to `True`) : Whether to use lower-order solvers in the final steps. Only valid for < 15 inference steps. This can stabilize the sampling of DPMSolver for steps < 15, especially for steps <= 10.

euler_at_final (`bool`, defaults to `False`) : Whether to use Euler's method in the final step. It is a trade-off between numerical stability and detail richness. This can stabilize the sampling of the SDE variant of DPMSolver for small number of inference steps, but sometimes may result in blurring.

final_sigmas_type (`str`, defaults to `"zero"`) : The final `sigma` value for the noise schedule during the sampling process. Must be one of `"zero"` or `"sigma_min"`. If `"sigma_min"`, the final sigma is the same as the last sigma in the training schedule. If `"zero"`, the final sigma is set to 0.

**Returns:**

``torch.Tensor``

The noisy samples with added noise scaled according to the timestep schedule.
#### convert_model_output[[diffusers.CosineDPMSolverMultistepScheduler.convert_model_output]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L420)

Convert the model output to the corresponding type the DPMSolver/DPMSolver++ algorithm needs. DPM-Solver is
designed to discretize an integral of the noise prediction model, and DPM-Solver++ is designed to discretize an
integral of the data prediction model.

> [!TIP] > The algorithm and model type are decoupled. You can use either DPMSolver or DPMSolver++ for both
noise > prediction and data prediction models.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The converted model output.
#### dpm_solver_first_order_update[[diffusers.CosineDPMSolverMultistepScheduler.dpm_solver_first_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L448)

One step for the first-order DPMSolver (equivalent to DDIM).

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### index_for_timestep[[diffusers.CosineDPMSolverMultistepScheduler.index_for_timestep]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L545)

Find the index for a given timestep in the schedule.

**Parameters:**

timestep (`int` or `torch.Tensor`) : The timestep for which to find the index.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:**

``int``

The index of the timestep in the schedule.
#### multistep_dpm_solver_second_order_update[[diffusers.CosineDPMSolverMultistepScheduler.multistep_dpm_solver_second_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L486)

One step for the second-order multistep DPMSolver.

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### precondition_inputs[[diffusers.CosineDPMSolverMultistepScheduler.precondition_inputs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L164)

Precondition the input sample by scaling it according to the EDM formulation.

**Parameters:**

sample (`torch.Tensor`) : The input sample tensor to precondition.

sigma (`float` or `torch.Tensor`) : The current sigma (noise level) value.

**Returns:**

``torch.Tensor``

The scaled input sample.
#### precondition_noise[[diffusers.CosineDPMSolverMultistepScheduler.precondition_noise]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L182)

Precondition the noise level by computing a normalized timestep representation.

**Parameters:**

sigma (`float` or `torch.Tensor`) : The sigma (noise level) value to precondition.

**Returns:**

``torch.Tensor``

The preconditioned noise value computed as `atan(sigma) / pi * 2`.
#### precondition_outputs[[diffusers.CosineDPMSolverMultistepScheduler.precondition_outputs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L200)

Precondition the model outputs according to the EDM formulation.

**Parameters:**

sample (`torch.Tensor`) : The input sample tensor.

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

sigma (`float` or `torch.Tensor`) : The current sigma (noise level) value.

**Returns:**

``torch.Tensor``

The denoised sample computed by combining the skip connection and output scaling.
#### scale_model_input[[diffusers.CosineDPMSolverMultistepScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L236)

Scale the denoising model input to match the Euler algorithm. Ensures interchangeability with schedulers that
need to scale the denoising model input depending on the current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample tensor.

timestep (`float` or `torch.Tensor`) : The current timestep in the diffusion chain.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_begin_index[[diffusers.CosineDPMSolverMultistepScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L153)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.CosineDPMSolverMultistepScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L260)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.
#### step[[diffusers.CosineDPMSolverMultistepScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_cosine_dpmsolver_multistep.py#L598)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the sample with
the multistep DPMSolver.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int` or `torch.Tensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator.

return_dict (`bool`, defaults to `True`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`.

**Returns:**

`[SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple``

If return_dict is `True`, [SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

## SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]
#### diffusers.schedulers.scheduling_utils.SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_utils.py#L61)

Base class for the output of a scheduler's `step` function.

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.
