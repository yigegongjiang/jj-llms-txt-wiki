# TCDScheduler

[Trajectory Consistency Distillation](https://huggingface.co/papers/2402.19159) by Jianbin Zheng, Minghui Hu, Zhongyi Fan, Chaoyue Wang, Changxing Ding, Dacheng Tao and Tat-Jen Cham introduced a Strategic Stochastic Sampling (Algorithm 4) that is capable of generating good samples in a small number of steps. Distinguishing it as an advanced iteration of the multistep scheduler (Algorithm 1) in the [Consistency Models](https://huggingface.co/papers/2303.01469), Strategic Stochastic Sampling specifically tailored for the trajectory consistency function.

The abstract from the paper is:

*Latent Consistency Model (LCM) extends the Consistency Model to the latent space and leverages the guided consistency distillation technique to achieve impressive performance in accelerating text-to-image synthesis. However, we observed that LCM struggles to generate images with both clarity and detailed intricacy. To address this limitation, we initially delve into and elucidate the underlying causes. Our investigation identifies that the primary issue stems from errors in three distinct areas. Consequently, we introduce Trajectory Consistency Distillation (TCD), which encompasses trajectory consistency function and strategic stochastic sampling. The trajectory consistency function diminishes the distillation errors by broadening the scope of the self-consistency boundary condition and endowing the TCD with the ability to accurately trace the entire trajectory of the Probability Flow ODE. Additionally, strategic stochastic sampling is specifically designed to circumvent the accumulated errors inherent in multi-step consistency sampling, which is meticulously tailored to complement the TCD model. Experiments demonstrate that TCD not only significantly enhances image quality at low NFEs but also yields more detailed results compared to the teacher model at high NFEs.*

The original codebase can be found at [jabir-zheng/TCD](https://github.com/jabir-zheng/TCD).

## TCDScheduler[[diffusers.TCDScheduler]]
#### diffusers.TCDScheduler[[diffusers.TCDScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_tcd.py#L141)

`TCDScheduler` incorporates the `Strategic Stochastic Sampling` introduced by the paper `Trajectory Consistency
Distillation`, extending the original Multistep Consistency Sampling to enable unrestricted trajectory traversal.

This code is based on the official repo of TCD(https://github.com/jabir-zheng/TCD).

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). [~ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin) takes care of storing all config
attributes that are passed in the scheduler's `__init__` function, such as `num_train_timesteps`. They can be
accessed via `scheduler.config.num_train_timesteps`. [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) provides general loading and saving
functionality via the [SchedulerMixin.save_pretrained()](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin.save_pretrained) and [from_pretrained()](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin.from_pretrained) functions.

add_noisediffusers.TCDScheduler.add_noisehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_tcd.py#L690[{"name": "original_samples", "val": ": Tensor"}, {"name": "noise", "val": ": Tensor"}, {"name": "timesteps", "val": ": IntTensor"}]- **original_samples** (`torch.Tensor`) --
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
#### get_velocity[[diffusers.TCDScheduler.get_velocity]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_tcd.py#L733)

Compute the velocity prediction from the sample and noise according to the velocity formula.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

noise (`torch.Tensor`) : The noise tensor.

timesteps (`torch.IntTensor`) : The timesteps for velocity computation.

**Returns:**

``torch.Tensor``

The computed velocity.
#### index_for_timestep[[diffusers.TCDScheduler.index_for_timestep]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_tcd.py#L263)

Find the index of a given timestep in the timestep schedule.

**Parameters:**

timestep (`float` or `torch.Tensor`) : The timestep value to find in the schedule.

schedule_timesteps (`torch.Tensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:**

``int``

The index of the timestep in the schedule. For the very first step, returns the second index if
multiple matches exist to avoid skipping a sigma when starting mid-schedule (e.g., for image-to-image).
#### previous_timestep[[diffusers.TCDScheduler.previous_timestep]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_tcd.py#L771)

Compute the previous timestep in the diffusion chain.

**Parameters:**

timestep (`int`) : The current timestep.

**Returns:**

``int` or `torch.Tensor``

The previous timestep.
#### scale_model_input[[diffusers.TCDScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_tcd.py#L331)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`int`, *optional*) : The current timestep in the diffusion chain.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_begin_index[[diffusers.TCDScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_tcd.py#L321)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.TCDScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_tcd.py#L421)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model. If used, `timesteps` must be `None`.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

original_inference_steps (`int`, *optional*) : The original number of inference steps, which will be used to generate a linearly-spaced timestep schedule (which is different from the standard `diffusers` implementation). We will then take `num_inference_steps` timesteps from this schedule, evenly spaced in terms of indices, and use that as our final timestep schedule. If not set, this will default to the `original_inference_steps` attribute.

timesteps (`list[int]`, *optional*) : Custom timesteps used to support arbitrary spacing between timesteps. If `None`, then the default timestep spacing strategy of equal spacing between timesteps on the training/distillation timestep schedule is used. If `timesteps` is passed, `num_inference_steps` must be `None`.

strength (`float`, *optional*, defaults to 1.0) : Used to determine the number of timesteps used for inference when using img2img, inpaint, etc.
#### step[[diffusers.TCDScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_tcd.py#L583)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

eta (`float`) : A stochastic parameter (referred to as `gamma` in the paper) used to control the stochasticity in every step. When eta = 0, it represents deterministic sampling, whereas eta = 1 indicates full stochastic sampling.

generator (`torch.Generator`, *optional*) : A random number generator.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [TCDSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/tcd#diffusers.schedulers.scheduling_tcd.TCDSchedulerOutput) or `tuple`.

**Returns:**

``~schedulers.scheduling_utils.TCDSchedulerOutput` or `tuple``

If return_dict is `True`, [TCDSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/tcd#diffusers.schedulers.scheduling_tcd.TCDSchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

## TCDSchedulerOutput[[diffusers.schedulers.scheduling_tcd.TCDSchedulerOutput]]
#### diffusers.schedulers.scheduling_tcd.TCDSchedulerOutput[[diffusers.schedulers.scheduling_tcd.TCDSchedulerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_tcd.py#L35)

Output class for the scheduler's `step` function output.

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

pred_noised_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : The predicted noised sample `(x_{s})` based on the model output from the current timestep.
