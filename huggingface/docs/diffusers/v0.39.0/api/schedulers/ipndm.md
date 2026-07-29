# IPNDMScheduler

`IPNDMScheduler` is a fourth-order Improved Pseudo Linear Multistep scheduler. The original implementation can be found at [crowsonkb/v-diffusion-pytorch](https://github.com/crowsonkb/v-diffusion-pytorch/blob/987f8985e38208345c1959b0ea767a625831cc9b/diffusion/sampling.py#L296).

## IPNDMScheduler[[diffusers.IPNDMScheduler]]
#### diffusers.IPNDMScheduler[[diffusers.IPNDMScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ipndm.py#L24)

A fourth-order Improved Pseudo Linear Multistep scheduler.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

index_for_timestepdiffusers.IPNDMScheduler.index_for_timestephttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ipndm.py#L124[{"name": "timestep", "val": ": float | torch.Tensor"}, {"name": "schedule_timesteps", "val": ": torch.Tensor | None = None"}]- **timestep** (`float` or `torch.Tensor`) --
  The timestep value to find in the schedule.
- **schedule_timesteps** (`torch.Tensor`, *optional*) --
  The timestep schedule to search in. If `None`, uses `self.timesteps`.0`int`The index of the timestep in the schedule. For the very first step, returns the second index if
multiple matches exist to avoid skipping a sigma when starting mid-schedule (e.g., for image-to-image).

Find the index of a given timestep in the timestep schedule.

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

trained_betas (`np.ndarray` or `List[float]`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

**Returns:**

``int``

The index of the timestep in the schedule. For the very first step, returns the second index if
multiple matches exist to avoid skipping a sigma when starting mid-schedule (e.g., for image-to-image).
#### scale_model_input[[diffusers.IPNDMScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ipndm.py#L228)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_begin_index[[diffusers.IPNDMScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ipndm.py#L85)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.IPNDMScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ipndm.py#L95)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.
#### step[[diffusers.IPNDMScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ipndm.py#L170)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the sample with
the linear multistep method. It performs one forward pass multiple times to approximate the solution.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int` or `torch.Tensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

return_dict (`bool`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/dpm_discrete#diffusers.schedulers.scheduling_utils.SchedulerOutput) or tuple.

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
