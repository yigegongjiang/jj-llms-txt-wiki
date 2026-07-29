# DEISMultistepScheduler

Diffusion Exponential Integrator Sampler (DEIS) is proposed in [Fast Sampling of Diffusion Models with Exponential Integrator](https://huggingface.co/papers/2204.13902) by Qinsheng Zhang and Yongxin Chen. `DEISMultistepScheduler` is a fast high order solver for diffusion ordinary differential equations (ODEs).

This implementation modifies the polynomial fitting formula in log-rho space instead of the original linear `t` space in the DEIS paper. The modification enjoys closed-form coefficients for exponential multistep update instead of replying on the numerical solver.

The abstract from the paper is:

*The past few years have witnessed the great success of Diffusion models~(DMs) in generating high-fidelity samples in generative modeling tasks. A major limitation of the DM is its notoriously slow sampling procedure which normally requires hundreds to thousands of time discretization steps of the learned diffusion process to reach the desired accuracy. Our goal is to develop a fast sampling method for DMs with a much less number of steps while retaining high sample quality. To this end, we systematically analyze the sampling procedure in DMs and identify key factors that affect the sample quality, among which the method of discretization is most crucial. By carefully examining the learned diffusion process, we propose Diffusion Exponential Integrator Sampler~(DEIS). It is based on the Exponential Integrator designed for discretizing ordinary differential equations (ODEs) and leverages a semilinear structure of the learned diffusion process to reduce the discretization error. The proposed method can be applied to any DMs and can generate high-fidelity samples in as few as 10 steps. In our experiments, it takes about 3 minutes on one A6000 GPU to generate 50k images from CIFAR10. Moreover, by directly using pre-trained DMs, we achieve the state-of-art sampling performance when the number of score function evaluation~(NFE) is limited, e.g., 4.17 FID with 10 NFEs, 3.37 FID, and 9.74 IS with only 15 NFEs on CIFAR10. Code is available at [this https URL](https://github.com/qsh-zh/deis).*

## Tips

It is recommended to set `solver_order` to 2 or 3, while `solver_order=1` is equivalent to [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler).

Dynamic thresholding from [Imagen](https://huggingface.co/papers/2205.11487) is supported, and for pixel-space
diffusion models, you can set `thresholding=True` to use the dynamic thresholding.

## DEISMultistepScheduler[[diffusers.DEISMultistepScheduler]]
#### diffusers.DEISMultistepScheduler[[diffusers.DEISMultistepScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L86)

`DEISMultistepScheduler` is a fast high order solver for diffusion ordinary differential equations (ODEs).

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

add_noisediffusers.DEISMultistepScheduler.add_noisehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L997[{"name": "original_samples", "val": ": Tensor"}, {"name": "noise", "val": ": Tensor"}, {"name": "timesteps", "val": ": IntTensor"}]- **original_samples** (`torch.Tensor`) --
  The original samples without noise.
- **noise** (`torch.Tensor`) --
  The noise to add to the samples.
- **timesteps** (`torch.IntTensor`) --
  The timesteps at which to add noise to the samples.0`torch.Tensor`The noisy samples.

Add noise to the original samples according to the noise schedule at the specified timesteps.

**Parameters:**

num_train_timesteps (`int`, defaults to `1000`) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to `0.0001`) : The starting `beta` value of inference.

beta_end (`float`, defaults to `0.02`) : The final `beta` value.

beta_schedule (`"linear"`, `"scaled_linear"`, or `"squaredcos_cap_v2"`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Choose from `linear`, `scaled_linear`, or `squaredcos_cap_v2`.

trained_betas (`np.ndarray` or `list[float]`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

solver_order (`int`, defaults to `2`) : The DEIS order which can be `1` or `2` or `3`. It is recommended to use `solver_order=2` for guided sampling, and `solver_order=3` for unconditional sampling.

prediction_type (`"epsilon"`, `"sample"`, `"v_prediction"`, or `"flow_prediction"`, defaults to `"epsilon"`) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`), `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper), or `flow_prediction`.

thresholding (`bool`, defaults to `False`) : Whether to use the "dynamic thresholding" method. This is unsuitable for latent-space diffusion models such as Stable Diffusion.

dynamic_thresholding_ratio (`float`, defaults to `0.995`) : The ratio for the dynamic thresholding method. Valid only when `thresholding=True`.

sample_max_value (`float`, defaults to `1.0`) : The threshold value for dynamic thresholding. Valid only when `thresholding=True`.

algorithm_type (`"deis"`, defaults to `"deis"`) : The algorithm type for the solver.

solver_type (`"logrho"`, defaults to `"logrho"`) : Solver type for DEIS.

lower_order_final (`bool`, defaults to `True`) : Whether to use lower-order solvers in the final steps. Only valid for < 15 inference steps.

use_karras_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use Karras sigmas for step sizes in the noise schedule during the sampling process. If `True`, the sigmas are determined according to a sequence of noise levels {σi}.

use_exponential_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use exponential sigmas for step sizes in the noise schedule during the sampling process.

use_beta_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use beta sigmas for step sizes in the noise schedule during the sampling process. Refer to [Beta Sampling is All You Need](https://huggingface.co/papers/2407.12173) for more information.

use_flow_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use flow sigmas for step sizes in the noise schedule during the sampling process.

flow_shift (`float`, *optional*, defaults to `1.0`) : The flow shift parameter for flow-based models.

timestep_spacing (`"linspace"`, `"leading"`, or `"trailing"`, defaults to `"linspace"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

steps_offset (`int`, defaults to `0`) : An offset added to the inference steps, as required by some model families.

use_dynamic_shifting (`bool`, defaults to `False`) : Whether to use dynamic shifting for the noise schedule.

time_shift_type (`"exponential"`, defaults to `"exponential"`) : The type of time shifting to apply.

**Returns:**

``torch.Tensor``

The noisy samples.
#### convert_model_output[[diffusers.DEISMultistepScheduler.convert_model_output]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L581)

Convert the model output to the corresponding type the DEIS algorithm needs.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The converted model output.
#### deis_first_order_update[[diffusers.DEISMultistepScheduler.deis_first_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L641)

One step for the first-order DEIS (equivalent to DDIM).

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

prev_timestep (`int`) : The previous discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### index_for_timestep[[diffusers.DEISMultistepScheduler.index_for_timestep]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L864)

Find the index for a given timestep in the schedule.

**Parameters:**

timestep (`int` or `torch.Tensor`) : The timestep for which to find the index.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:**

``int``

The index of the timestep in the schedule.
#### multistep_deis_second_order_update[[diffusers.DEISMultistepScheduler.multistep_deis_second_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L702)

One step for the second-order multistep DEIS.

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### multistep_deis_third_order_update[[diffusers.DEISMultistepScheduler.multistep_deis_third_order_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L775)

One step for the third-order multistep DEIS.

**Parameters:**

model_output_list (`list[torch.Tensor]`) : The direct outputs from learned diffusion model at current and latter timesteps.

sample (`torch.Tensor`) : A current instance of a sample created by diffusion process.

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### scale_model_input[[diffusers.DEISMultistepScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L981)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_begin_index[[diffusers.DEISMultistepScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L262)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.DEISMultistepScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L272)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

mu (`float`, *optional*) : The mu parameter for dynamic shifting. Only used when `use_dynamic_shifting=True` and `time_shift_type="exponential"`.
#### step[[diffusers.DEISMultistepScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_deis_multistep.py#L917)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the sample with
the multistep DEIS.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int` or `torch.Tensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

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
