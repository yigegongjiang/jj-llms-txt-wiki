# EulerDiscreteScheduler

The Euler scheduler (Algorithm 2) is from the [Elucidating the Design Space of Diffusion-Based Generative Models](https://huggingface.co/papers/2206.00364) paper by Karras et al. This is a fast scheduler which can often generate good outputs in 20-30 steps. The scheduler is based on the original [k-diffusion](https://github.com/crowsonkb/k-diffusion/blob/481677d114f6ea445aa009cf5bd7a9cdee909e47/k_diffusion/sampling.py#L51) implementation by [Katherine Crowson](https://github.com/crowsonkb/).

## EulerDiscreteScheduler[[diffusers.EulerDiscreteScheduler]]
#### diffusers.EulerDiscreteScheduler[[diffusers.EulerDiscreteScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_euler_discrete.py#L143)

Euler scheduler.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

add_noisediffusers.EulerDiscreteScheduler.add_noisehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_euler_discrete.py#L802[{"name": "original_samples", "val": ": Tensor"}, {"name": "noise", "val": ": Tensor"}, {"name": "timesteps", "val": ": Tensor"}]- **original_samples** (`torch.Tensor`) --
  The original samples to which noise will be added.
- **noise** (`torch.Tensor`) --
  The noise tensor to add to the original samples.
- **timesteps** (`torch.Tensor`) --
  The timesteps at which to add noise, determining the noise level from the schedule.0`torch.Tensor`The noisy samples with added noise scaled according to the timestep schedule.

Add noise to the original samples according to the noise schedule at the specified timesteps.

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to 0.0001) : The starting `beta` value of inference.

beta_end (`float`, defaults to 0.02) : The final `beta` value.

beta_schedule (`Literal["linear", "scaled_linear", "squaredcos_cap_v2"]`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Choose from `"linear"`, `"scaled_linear"`, or `"squaredcos_cap_v2"`.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

prediction_type (`Literal["epsilon", "sample", "v_prediction"]`, defaults to `"epsilon"`, *optional*) : Prediction type of the scheduler function; can be `"epsilon"` (predicts the noise of the diffusion process), `"sample"` (directly predicts the noisy sample`) or `"v_prediction"` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

interpolation_type (`Literal["linear", "log_linear"]`, defaults to `"linear"`, *optional*) : The interpolation type to compute intermediate sigmas for the scheduler denoising steps. Should be one of `"linear"` or `"log_linear"`.

use_karras_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use Karras sigmas for step sizes in the noise schedule during the sampling process. If `True`, the sigmas are determined according to a sequence of noise levels {σi}.

use_exponential_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use exponential sigmas for step sizes in the noise schedule during the sampling process.

use_beta_sigmas (`bool`, *optional*, defaults to `False`) : Whether to use beta sigmas for step sizes in the noise schedule during the sampling process. Refer to [Beta Sampling is All You Need](https://huggingface.co/papers/2407.12173) for more information.

sigma_min (`float`, *optional*) : The minimum sigma value for the noise schedule. If not provided, defaults to the last sigma in the schedule.

sigma_max (`float`, *optional*) : The maximum sigma value for the noise schedule. If not provided, defaults to the first sigma in the schedule.

timestep_spacing (`Literal["linspace", "leading", "trailing"]`, defaults to `"linspace"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

timestep_type (`Literal["discrete", "continuous"]`, defaults to `"discrete"`) : The type of timesteps to use. Can be `"discrete"` or `"continuous"`.

steps_offset (`int`, defaults to 0) : An offset added to the inference steps, as required by some model families.

rescale_betas_zero_snr (`bool`, defaults to `False`) : Whether to rescale the betas to have zero terminal SNR. This enables the model to generate very bright and dark samples instead of limiting it to samples with medium brightness. Loosely related to [`--offset_noise`](https://github.com/huggingface/diffusers/blob/74fd735eb073eb1d774b1ab4154a0876eb82f055/examples/dreambooth/train_dreambooth.py#L506).

final_sigmas_type (`Literal["zero", "sigma_min"]`, defaults to `"zero"`) : The final `sigma` value for the noise schedule during the sampling process. If `"sigma_min"`, the final sigma is the same as the last sigma in the training schedule. If `"zero"`, the final sigma is set to 0.

**Returns:**

``torch.Tensor``

The noisy samples with added noise scaled according to the timestep schedule.
#### get_velocity[[diffusers.EulerDiscreteScheduler.get_velocity]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_euler_discrete.py#L850)

Compute the velocity prediction for the given sample and noise at the specified timesteps.

This method implements the velocity prediction used in v-prediction models, which predicts a linear combination
of the sample and noise.

**Parameters:**

sample (`torch.Tensor`) : The input sample for which to compute the velocity.

noise (`torch.Tensor`) : The noise tensor corresponding to the sample.

timesteps (`torch.Tensor`) : The timesteps at which to compute the velocity.

**Returns:**

``torch.Tensor``

The velocity prediction computed as `sqrt(alpha_prod) * noise - sqrt(1 - alpha_prod) * sample`.
#### index_for_timestep[[diffusers.EulerDiscreteScheduler.index_for_timestep]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_euler_discrete.py#L640)

Find the index of a given timestep in the timestep schedule.

**Parameters:**

timestep (`float` or `torch.Tensor`) : The timestep value to find in the schedule.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:**

``int``

The index of the timestep in the schedule. For the very first step, returns the second index if
multiple matches exist to avoid skipping a sigma when starting mid-schedule (e.g., for image-to-image).
#### scale_model_input[[diffusers.EulerDiscreteScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_euler_discrete.py#L326)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep. Scales the denoising model input by `(sigma**2 + 1) ** 0.5` to match the Euler algorithm.

**Parameters:**

sample (`torch.Tensor`) : The input sample to be scaled.

timestep (`float` or `torch.Tensor`) : The current timestep in the diffusion chain.

**Returns:**

``torch.Tensor``

A scaled input sample, divided by `(sigma**2 + 1) ** 0.5`.
#### set_begin_index[[diffusers.EulerDiscreteScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_euler_discrete.py#L316)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.EulerDiscreteScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_euler_discrete.py#L350)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model. If `None`, `timesteps` or `sigmas` must be provided.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

timesteps (`list[int]`, *optional*) : Custom timesteps used to support arbitrary timesteps schedule. If `None`, timesteps will be generated based on the `timestep_spacing` attribute. If `timesteps` is passed, `num_inference_steps` and `sigmas` must be `None`, and `timestep_spacing` attribute will be ignored.

sigmas (`list[float]`, *optional*) : Custom sigmas used to support arbitrary timesteps schedule. If `None`, timesteps and sigmas will be generated based on the relevant scheduler attributes. If `sigmas` is passed, `num_inference_steps` and `timesteps` must be `None`, and the timesteps will be generated based on the custom sigmas schedule.
#### step[[diffusers.EulerDiscreteScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_euler_discrete.py#L685)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`float` or `torch.Tensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

s_churn (`float`, *optional*, defaults to `0.0`) : Stochasticity parameter that controls the amount of noise added during sampling. Higher values increase randomness.

s_tmin (`float`, *optional*, defaults to `0.0`) : Minimum timestep threshold for applying stochasticity. Only timesteps above this value will have noise added.

s_tmax (`float`, *optional*, defaults to `inf`) : Maximum timestep threshold for applying stochasticity. Only timesteps below this value will have noise added.

s_noise (`float`, *optional*, defaults to `1.0`) : Scaling factor for noise added to the sample.

generator (`torch.Generator`, *optional*) : A random number generator for reproducible sampling.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [EulerDiscreteSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/euler#diffusers.schedulers.scheduling_euler_discrete.EulerDiscreteSchedulerOutput) or tuple.

**Returns:**

`[EulerDiscreteSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/euler#diffusers.schedulers.scheduling_euler_discrete.EulerDiscreteSchedulerOutput) or `tuple``

If `return_dict` is `True`, [EulerDiscreteSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/euler#diffusers.schedulers.scheduling_euler_discrete.EulerDiscreteSchedulerOutput) is
returned, otherwise a tuple is returned where the first element is the sample tensor and the second
element is the predicted original sample.

## EulerDiscreteSchedulerOutput[[diffusers.schedulers.scheduling_euler_discrete.EulerDiscreteSchedulerOutput]]
#### diffusers.schedulers.scheduling_euler_discrete.EulerDiscreteSchedulerOutput[[diffusers.schedulers.scheduling_euler_discrete.EulerDiscreteSchedulerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_euler_discrete.py#L36)

Output class for the scheduler's `step` function output.

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

pred_original_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : The predicted denoised sample `(x_{0})` based on the model output from the current timestep. `pred_original_sample` can be used to preview progress or for guidance.
