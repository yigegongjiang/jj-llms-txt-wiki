# LMSDiscreteScheduler

`LMSDiscreteScheduler` is a linear multistep scheduler for discrete beta schedules. The scheduler is ported from and created by [Katherine Crowson](https://github.com/crowsonkb/), and the original implementation can be found at [crowsonkb/k-diffusion](https://github.com/crowsonkb/k-diffusion/blob/481677d114f6ea445aa009cf5bd7a9cdee909e47/k_diffusion/sampling.py#L181).

## LMSDiscreteScheduler[[diffusers.LMSDiscreteScheduler]]
#### diffusers.LMSDiscreteScheduler[[diffusers.LMSDiscreteScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lms_discrete.py#L104)

A linear multistep scheduler for discrete beta schedules.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

add_noisediffusers.LMSDiscreteScheduler.add_noisehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lms_discrete.py#L626[{"name": "original_samples", "val": ": Tensor"}, {"name": "noise", "val": ": Tensor"}, {"name": "timesteps", "val": ": Tensor"}]- **original_samples** (`torch.Tensor`) --
  The original samples to which noise will be added.
- **noise** (`torch.Tensor`) --
  The noise tensor to add to the original samples.
- **timesteps** (`torch.Tensor`) --
  The timesteps at which to add noise, determining the noise level from the schedule.0`torch.Tensor`The noisy samples with added noise scaled according to the timestep schedule.

Add noise to the original samples according to the noise schedule at the specified timesteps.

**Parameters:**

num_train_timesteps (`int`, defaults to `1000`) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to `0.0001`) : The starting `beta` value of inference.

beta_end (`float`, defaults to `0.02`) : The final `beta` value.

beta_schedule (`"linear"`, `"scaled_linear"`, or `"squaredcos_cap_v2"`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

use_karras_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use Karras sigmas for step sizes in the noise schedule during the sampling process. If `True`, the sigmas are determined according to a sequence of noise levels {σi}.

use_exponential_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use exponential sigmas for step sizes in the noise schedule during the sampling process.

use_beta_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use beta sigmas for step sizes in the noise schedule during the sampling process. Refer to [Beta Sampling is All You Need](https://huggingface.co/papers/2407.12173) for more information.

prediction_type (`"epsilon"`, `"sample"`, or `"v_prediction"`, defaults to `"epsilon"`) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`) or `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

timestep_spacing (`"linspace"`, `"leading"`, or `"trailing"`, defaults to `"linspace"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

steps_offset (`int`, defaults to `0`) : An offset added to the inference steps, as required by some model families.

**Returns:**

``torch.Tensor``

The noisy samples with added noise scaled according to the timestep schedule.
#### get_lms_coefficient[[diffusers.LMSDiscreteScheduler.get_lms_coefficient]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lms_discrete.py#L267)

Compute the linear multistep coefficient.

**Parameters:**

order (`int`) : The order of the linear multistep method.

t (`int`) : The current timestep index.

current_order (`int`) : The current order for which to compute the coefficient.

**Returns:**

``float``

The computed linear multistep coefficient.
#### index_for_timestep[[diffusers.LMSDiscreteScheduler.index_for_timestep]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lms_discrete.py#L355)

Find the index of a given timestep in the timestep schedule.

**Parameters:**

timestep (`float` or `torch.Tensor`) : The timestep value to find in the schedule.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:**

``int``

The index of the timestep in the schedule. For the very first step, returns the second index if
multiple matches exist to avoid skipping a sigma when starting mid-schedule (e.g., for image-to-image).
#### scale_model_input[[diffusers.LMSDiscreteScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lms_discrete.py#L243)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`float` or `torch.Tensor`) : The current timestep in the diffusion chain.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_begin_index[[diffusers.LMSDiscreteScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lms_discrete.py#L233)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.LMSDiscreteScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lms_discrete.py#L296)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.
#### step[[diffusers.LMSDiscreteScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lms_discrete.py#L545)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`float` or `torch.Tensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

order (`int`, defaults to 4) : The order of the linear multistep method.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) or tuple.

**Returns:**

`[SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple``

If return_dict is `True`, [SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

## LMSDiscreteSchedulerOutput[[diffusers.schedulers.scheduling_lms_discrete.LMSDiscreteSchedulerOutput]]
#### diffusers.schedulers.scheduling_lms_discrete.LMSDiscreteSchedulerOutput[[diffusers.schedulers.scheduling_lms_discrete.LMSDiscreteSchedulerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_lms_discrete.py#L34)

Output class for the scheduler's `step` function output.

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

pred_original_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : The predicted denoised sample `(x_{0})` based on the model output from the current timestep. `pred_original_sample` can be used to preview progress or for guidance.
