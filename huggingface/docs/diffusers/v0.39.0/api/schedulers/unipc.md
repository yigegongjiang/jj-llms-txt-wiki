# UniPCMultistepScheduler

`UniPCMultistepScheduler` is a training-free framework designed for fast sampling of diffusion models. It was introduced in [UniPC: A Unified Predictor-Corrector Framework for Fast Sampling of Diffusion Models](https://huggingface.co/papers/2302.04867) by Wenliang Zhao, Lujia Bai, Yongming Rao, Jie Zhou, Jiwen Lu.

It consists of a corrector (UniC) and a predictor (UniP) that share a unified analytical form and support arbitrary orders.
UniPC is by design model-agnostic, supporting pixel-space/latent-space DPMs on unconditional/conditional sampling. It can also be applied to both noise prediction and data prediction models. The corrector UniC can be also applied after any off-the-shelf solvers to increase the order of accuracy.

The abstract from the paper is:

*Diffusion probabilistic models (DPMs) have demonstrated a very promising ability in high-resolution image synthesis. However, sampling from a pre-trained DPM is time-consuming due to the multiple evaluations of the denoising network, making it more and more important to accelerate the sampling of DPMs. Despite recent progress in designing fast samplers, existing methods still cannot generate satisfying images in many applications where fewer steps (e.g., <10) are favored. In this paper, we develop a unified corrector (UniC) that can be applied after any existing DPM sampler to increase the order of accuracy without extra model evaluations, and derive a unified predictor (UniP) that supports arbitrary order as a byproduct. Combining UniP and UniC, we propose a unified predictor-corrector framework called UniPC for the fast sampling of DPMs, which has a unified analytical form for any order and can significantly improve the sampling quality over previous methods, especially in extremely few steps. We evaluate our methods through extensive experiments including both unconditional and conditional sampling using pixel-space and latent-space DPMs. Our UniPC can achieve 3.87 FID on CIFAR10 (unconditional) and 7.51 FID on ImageNet 256×256 (conditional) with only 10 function evaluations. Code is available at [this https URL](https://github.com/wl-zhao/UniPC).*

## Tips

It is recommended to set `solver_order` to 2 for guide sampling, and `solver_order=3` for unconditional sampling.

Dynamic thresholding from [Imagen](https://huggingface.co/papers/2205.11487) is supported, and for pixel-space
diffusion models, you can set both `predict_x0=True` and `thresholding=True` to use dynamic thresholding. This thresholding method is unsuitable for latent-space diffusion models such as Stable Diffusion.

## UniPCMultistepScheduler[[diffusers.UniPCMultistepScheduler]]
#### diffusers.UniPCMultistepScheduler[[diffusers.UniPCMultistepScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L123)

`UniPCMultistepScheduler` is a training-free framework designed for the fast sampling of diffusion models.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

add_noisediffusers.UniPCMultistepScheduler.add_noisehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L1250[{"name": "original_samples", "val": ": Tensor"}, {"name": "noise", "val": ": Tensor"}, {"name": "timesteps", "val": ": IntTensor"}]- **original_samples** (`torch.Tensor`) --
  The original samples without noise.
- **noise** (`torch.Tensor`) --
  The noise to add to the samples.
- **timesteps** (`torch.IntTensor`) --
  The timesteps at which to add noise to the samples.0`torch.Tensor`The noisy samples.

Add noise to the original samples according to the noise schedule at the specified timesteps.

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to 0.0001) : The starting `beta` value of inference.

beta_end (`float`, defaults to 0.02) : The final `beta` value.

beta_schedule (`"linear"`, `"scaled_linear"`, or `"squaredcos_cap_v2"`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Choose from `linear`, `scaled_linear`, or `squaredcos_cap_v2`.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

solver_order (`int`, defaults to `2`) : The UniPC order which can be any positive integer. The effective order of accuracy is `solver_order + 1` due to the UniC. It is recommended to use `solver_order=2` for guided sampling, and `solver_order=3` for unconditional sampling.

prediction_type (`"epsilon"`, `"sample"`, `"v_prediction"`, or `"flow_prediction"`, defaults to `"epsilon"`, *optional*) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`), `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper), or `flow_prediction`.

thresholding (`bool`, defaults to `False`) : Whether to use the "dynamic thresholding" method. This is unsuitable for latent-space diffusion models such as Stable Diffusion.

dynamic_thresholding_ratio (`float`, defaults to 0.995) : The ratio for the dynamic thresholding method. Valid only when `thresholding=True`.

sample_max_value (`float`, defaults to 1.0) : The threshold value for dynamic thresholding. Valid only when `thresholding=True` and `predict_x0=True`.

predict_x0 (`bool`, defaults to `True`) : Whether to use the updating algorithm on the predicted x0.

solver_type (`"bh1"` or `"bh2"`, defaults to `"bh2"`) : Solver type for UniPC. It is recommended to use `bh1` for unconditional sampling when steps < 10, and `bh2` otherwise.

lower_order_final (`bool`, default `True`) : Whether to use lower-order solvers in the final steps. Only valid for < 15 inference steps. This can stabilize the sampling of DPMSolver for steps < 15, especially for steps <= 10.

disable_corrector (`list`, default `[]`) : Decides which step to disable the corrector to mitigate the misalignment between `epsilon_theta(x_t, c)` and `epsilon_theta(x_t^c, c)` which can influence convergence for a large guidance scale. Corrector is usually disabled during the first few steps.

solver_p (`SchedulerMixin`, default `None`) : Any other scheduler that if specified, the algorithm becomes `solver_p + UniC`.

use_karras_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use Karras sigmas for step sizes in the noise schedule during the sampling process. If `True`, the sigmas are determined according to a sequence of noise levels {σi}.

use_exponential_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use exponential sigmas for step sizes in the noise schedule during the sampling process.

use_beta_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use beta sigmas for step sizes in the noise schedule during the sampling process. Refer to [Beta Sampling is All You Need](https://huggingface.co/papers/2407.12173) for more information.

use_flow_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use flow sigmas for step sizes in the noise schedule during the sampling process.

timestep_spacing (`"linspace"`, `"leading"`, or `"trailing"`, defaults to `"linspace"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

steps_offset (`int`, defaults to 0) : An offset added to the inference steps, as required by some model families.

final_sigmas_type (`"zero"` or `"sigma_min"`, defaults to `"zero"`) : The final `sigma` value for the noise schedule during the sampling process. If `"sigma_min"`, the final sigma is the same as the last sigma in the training schedule. If `zero`, the final sigma is set to 0.

rescale_betas_zero_snr (`bool`, defaults to `False`) : Whether to rescale the betas to have zero terminal SNR. This enables the model to generate very bright and dark samples instead of limiting it to samples with medium brightness. Loosely related to [`--offset_noise`](https://github.com/huggingface/diffusers/blob/74fd735eb073eb1d774b1ab4154a0876eb82f055/examples/dreambooth/train_dreambooth.py#L506).

**Returns:**

``torch.Tensor``

The noisy samples.
#### convert_model_output[[diffusers.UniPCMultistepScheduler.convert_model_output]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L760)

Convert the model output to the corresponding type the UniPC algorithm needs.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The converted model output.
#### index_for_timestep[[diffusers.UniPCMultistepScheduler.index_for_timestep]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L1100)

Find the index for a given timestep in the schedule.

**Parameters:**

timestep (`int` or `torch.Tensor`) : The timestep for which to find the index.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:**

``int``

The index of the timestep in the schedule.
#### multistep_uni_c_bh_update[[diffusers.UniPCMultistepScheduler.multistep_uni_c_bh_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L962)

One step for the UniC (B(h) version).

**Parameters:**

this_model_output (`torch.Tensor`) : The model outputs at `x_t`.

this_timestep (`int`) : The current timestep `t`.

last_sample (`torch.Tensor`) : The generated sample before the last predictor `x_{t-1}`.

this_sample (`torch.Tensor`) : The generated sample after the last predictor `x_{t}`.

order (`int`) : The `p` of UniC-p at this step. The effective order of accuracy should be `order + 1`.

**Returns:**

``torch.Tensor``

The corrected sample tensor at the current timestep.
#### multistep_uni_p_bh_update[[diffusers.UniPCMultistepScheduler.multistep_uni_p_bh_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L833)

One step for the UniP (B(h) version). Alternatively, `self.solver_p` is used if is specified.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model at the current timestep.

prev_timestep (`int`) : The previous discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

order (`int`) : The order of UniP at this timestep (corresponds to the *p* in UniPC-p).

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### scale_model_input[[diffusers.UniPCMultistepScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L1234)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_begin_index[[diffusers.UniPCMultistepScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L308)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.UniPCMultistepScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L318)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

sigmas (`List[float]`, *optional*) : Custom values for sigmas to be used for each diffusion step. If `None`, the sigmas are computed automatically.

mu (`float`, *optional*) : Optional mu parameter for dynamic shifting when using exponential time shift type.
#### step[[diffusers.UniPCMultistepScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L1153)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the sample with
the multistep UniPC.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int` or `torch.Tensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

return_dict (`bool`, defaults to `True`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`.

**Returns:**

`[SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple``

If return_dict is `True`, [SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.
#### stretch_shift_to_terminal[[diffusers.UniPCMultistepScheduler.stretch_shift_to_terminal]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L506)

Stretches and shifts the timestep schedule to ensure it terminates at the configured `shift_terminal` config
value.

Reference:
https://github.com/Lightricks/LTX-Video/blob/a01a171f8fe3d99dce2728d60a73fecf4d4238ae/ltx_video/schedulers/rf.py#L51

**Parameters:**

t (`torch.Tensor`) : A tensor of timesteps to be stretched and shifted.

**Returns:**

``torch.Tensor``

A tensor of adjusted timesteps such that the final value equals `self.config.shift_terminal`.
#### time_shift[[diffusers.UniPCMultistepScheduler.time_shift]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_unipc_multistep.py#L484)

Apply time shifting to the sigmas.

**Parameters:**

mu (`float`) : The mu parameter for the time shift.

sigma (`float`) : The sigma parameter for the time shift.

t (`torch.Tensor`) : The input timesteps.

**Returns:**

``torch.Tensor``

The time-shifted timesteps.

## SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]
#### diffusers.schedulers.scheduling_utils.SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_utils.py#L61)

Base class for the output of a scheduler's `step` function.

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.
