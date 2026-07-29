# Latent Consistency Model Multistep Scheduler

## Overview

Multistep and onestep scheduler (Algorithm 3) introduced alongside latent consistency models in the paper [Latent Consistency Models: Synthesizing High-Resolution Images with Few-Step Inference](https://huggingface.co/papers/2310.04378) by Simian Luo, Yiqin Tan, Longbo Huang, Jian Li, and Hang Zhao.
This scheduler should be able to generate good samples from [LatentConsistencyModelPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/latent_consistency_models#diffusers.LatentConsistencyModelPipeline) in 1-8 steps.

## LCMScheduler[[diffusers.LCMScheduler]]
#### diffusers.LCMScheduler[[diffusers.LCMScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lcm.py#L142)

`LCMScheduler` extends the denoising procedure introduced in denoising diffusion probabilistic models (DDPMs) with
non-Markovian guidance.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). [~ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin) takes care of storing all config
attributes that are passed in the scheduler's `__init__` function, such as `num_train_timesteps`. They can be
accessed via `scheduler.config.num_train_timesteps`. [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) provides general loading and saving
functionality via the [SchedulerMixin.save_pretrained()](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin.save_pretrained) and [from_pretrained()](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin.from_pretrained) functions.

add_noisediffusers.LCMScheduler.add_noisehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lcm.py#L635[{"name": "original_samples", "val": ": Tensor"}, {"name": "noise", "val": ": Tensor"}, {"name": "timesteps", "val": ": IntTensor"}]- **original_samples** (`torch.Tensor`) --
  The original samples to which noise will be added.
- **noise** (`torch.Tensor`) --
  The noise to add to the samples.
- **timesteps** (`torch.IntTensor`) --
  The timesteps indicating the noise level for each sample.0`torch.Tensor`The noisy samples.

Add noise to the original samples according to the noise magnitude at each timestep (this is the forward
diffusion process).

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to 0.0001) : The starting `beta` value of inference.

beta_end (`float`, defaults to 0.02) : The final `beta` value.

beta_schedule (`str`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Choose from `linear`, `scaled_linear`, or `squaredcos_cap_v2`.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

original_inference_steps (`int`, *optional*, defaults to 50) : The default number of inference steps used to generate a linearly-spaced timestep schedule, from which we will ultimately take `num_inference_steps` evenly spaced timesteps to form the final timestep schedule.

clip_sample (`bool`, defaults to `True`) : Clip the predicted sample for numerical stability.

clip_sample_range (`float`, defaults to 1.0) : The maximum magnitude for sample clipping. Valid only when `clip_sample=True`.

set_alpha_to_one (`bool`, defaults to `True`) : Each diffusion step uses the alphas product value at that step and at the previous one. For the final step there is no previous alpha. When this option is `True` the previous alpha product is fixed to `1`, otherwise it uses the alpha value at step 0.

steps_offset (`int`, defaults to 0) : An offset added to the inference steps, as required by some model families.

prediction_type (`str`, defaults to `epsilon`, *optional*) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`) or `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

thresholding (`bool`, defaults to `False`) : Whether to use the "dynamic thresholding" method. This is unsuitable for latent-space diffusion models such as Stable Diffusion.

dynamic_thresholding_ratio (`float`, defaults to 0.995) : The ratio for the dynamic thresholding method. Valid only when `thresholding=True`.

sample_max_value (`float`, defaults to 1.0) : The threshold value for dynamic thresholding. Valid only when `thresholding=True`.

timestep_spacing (`str`, defaults to `"leading"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

timestep_scaling (`float`, defaults to 10.0) : The factor the timesteps will be multiplied by when calculating the consistency model boundary conditions `c_skip` and `c_out`. Increasing this will decrease the approximation error (although the approximation error at the default of `10.0` is already pretty small).

rescale_betas_zero_snr (`bool`, defaults to `False`) : Whether to rescale the betas to have zero terminal SNR. This enables the model to generate very bright and dark samples instead of limiting it to samples with medium brightness. Loosely related to [`--offset_noise`](https://github.com/huggingface/diffusers/blob/74fd735eb073eb1d774b1ab4154a0876eb82f055/examples/dreambooth/train_dreambooth.py#L506).

**Returns:**

``torch.Tensor``

The noisy samples.
#### get_velocity[[diffusers.LCMScheduler.get_velocity]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lcm.py#L678)

Compute the velocity prediction from the sample and noise according to the velocity formula.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

noise (`torch.Tensor`) : The noise tensor.

timesteps (`torch.IntTensor`) : The timesteps for velocity computation.

**Returns:**

``torch.Tensor``

The computed velocity.
#### index_for_timestep[[diffusers.LCMScheduler.index_for_timestep]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lcm.py#L262)

Find the index of a given timestep in the timestep schedule.

**Parameters:**

timestep (`float` or `torch.Tensor`) : The timestep value to find in the schedule.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:**

``int``

The index of the timestep in the schedule. For the very first step, returns the second index if
multiple matches exist to avoid skipping a sigma when starting mid-schedule (e.g., for image-to-image).
#### previous_timestep[[diffusers.LCMScheduler.previous_timestep]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lcm.py#L716)

Compute the previous timestep in the diffusion chain.

**Parameters:**

timestep (`int`) : The current timestep.

**Returns:**

``int` or `torch.Tensor``

The previous timestep.
#### scale_model_input[[diffusers.LCMScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lcm.py#L330)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`int`, *optional*) : The current timestep in the diffusion chain.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_begin_index[[diffusers.LCMScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lcm.py#L320)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.LCMScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lcm.py#L390)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model. If used, `timesteps` must be `None`.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

original_inference_steps (`int`, *optional*) : The original number of inference steps, which will be used to generate a linearly-spaced timestep schedule (which is different from the standard `diffusers` implementation). We will then take `num_inference_steps` timesteps from this schedule, evenly spaced in terms of indices, and use that as our final timestep schedule. If not set, this will default to the `original_inference_steps` attribute.

timesteps (`list[int]`, *optional*) : Custom timesteps used to support arbitrary spacing between timesteps. If `None`, then the default timestep spacing strategy of equal spacing between timesteps on the training/distillation timestep schedule is used. If `timesteps` is passed, `num_inference_steps` must be `None`.
#### step[[diffusers.LCMScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lcm.py#L538)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`float`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `LCMSchedulerOutput` or `tuple`.

**Returns:**

``~schedulers.scheduling_utils.LCMSchedulerOutput` or `tuple``

If return_dict is `True`, `LCMSchedulerOutput` is returned, otherwise a
tuple is returned where the first element is the sample tensor.
