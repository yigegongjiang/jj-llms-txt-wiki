# FlowMatchEulerDiscreteScheduler

`FlowMatchEulerDiscreteScheduler` is based on the flow-matching sampling introduced in [Stable Diffusion 3](https://huggingface.co/papers/2403.03206).

## FlowMatchEulerDiscreteScheduler[[diffusers.FlowMatchEulerDiscreteScheduler]]
#### diffusers.FlowMatchEulerDiscreteScheduler[[diffusers.FlowMatchEulerDiscreteScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L48)

Euler scheduler.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

index_for_timestepdiffusers.FlowMatchEulerDiscreteScheduler.index_for_timestephttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L387[{"name": "timestep", "val": ": typing.Union[float, torch.FloatTensor]"}, {"name": "schedule_timesteps", "val": ": typing.Optional[torch.FloatTensor] = None"}]- **timestep** (`float` or `torch.FloatTensor`) --
  The timestep to find the index for.
- **schedule_timesteps** (`torch.FloatTensor`, *optional*) --
  The schedule timesteps to validate against. If `None`, the scheduler's timesteps are used.0`int`The index of the timestep.

Get the index for the given timestep.

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

shift (`float`, defaults to 1.0) : The shift value for the timestep schedule.

use_dynamic_shifting (`bool`, defaults to False) : Whether to apply timestep shifting on-the-fly based on the image resolution.

base_shift (`float`, defaults to 0.5) : Value to stabilize image generation. Increasing `base_shift` reduces variation and image is more consistent with desired output.

max_shift (`float`, defaults to 1.15) : Value change allowed to latent vectors. Increasing `max_shift` encourages more variation and image may be more exaggerated or stylized.

base_image_seq_len (`int`, defaults to 256) : The base image sequence length.

max_image_seq_len (`int`, defaults to 4096) : The maximum image sequence length.

invert_sigmas (`bool`, defaults to False) : Whether to invert the sigmas.

shift_terminal (`float`, defaults to None) : The end value of the shifted timestep schedule.

use_karras_sigmas (`bool`, defaults to False) : Whether to use Karras sigmas for step sizes in the noise schedule during sampling.

use_exponential_sigmas (`bool`, defaults to False) : Whether to use exponential sigmas for step sizes in the noise schedule during sampling.

use_beta_sigmas (`bool`, defaults to False) : Whether to use beta sigmas for step sizes in the noise schedule during sampling.

time_shift_type (`str`, defaults to "exponential") : The type of dynamic resolution-dependent timestep shifting to apply. Either "exponential" or "linear".

stochastic_sampling (`bool`, defaults to False) : Whether to use stochastic sampling.

**Returns:**

``int``

The index of the timestep.
#### scale_noise[[diffusers.FlowMatchEulerDiscreteScheduler.scale_noise]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L188)

Forward process in flow-matching

**Parameters:**

sample (`torch.FloatTensor`) : The input sample.

timestep (`torch.FloatTensor`) : The current timestep in the diffusion chain.

noise (`torch.FloatTensor`) : The noise tensor.

**Returns:**

``torch.FloatTensor``

A scaled input sample.
#### set_begin_index[[diffusers.FlowMatchEulerDiscreteScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L168)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.
#### set_shift[[diffusers.FlowMatchEulerDiscreteScheduler.set_shift]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L178)

Sets the shift value for the scheduler.

**Parameters:**

shift (`float`) : The shift value to be set.
#### set_timesteps[[diffusers.FlowMatchEulerDiscreteScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L283)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

sigmas (`list[float]`, *optional*) : Custom values for sigmas to be used for each diffusion step. If `None`, the sigmas are computed automatically.

mu (`float`, *optional*) : Determines the amount of shifting applied to sigmas when performing resolution-dependent timestep shifting.

timesteps (`list[float]`, *optional*) : Custom values for timesteps to be used for each diffusion step. If `None`, the timesteps are computed automatically.
#### step[[diffusers.FlowMatchEulerDiscreteScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L426)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

**Parameters:**

model_output (`torch.FloatTensor`) : The direct output from learned diffusion model.

timestep (`float`) : The current discrete timestep in the diffusion chain.

sample (`torch.FloatTensor`) : A current instance of a sample created by the diffusion process.

s_churn (`float`) --

s_tmin  (`float`) --

s_tmax  (`float`) --

s_noise (`float`, defaults to 1.0) : Scaling factor for noise added to the sample.

generator (`torch.Generator`, *optional*) : A random number generator.

per_token_timesteps (`torch.Tensor`, *optional*) : The timesteps for each token in the sample.

return_dict (`bool`, defaults to `True`) : Whether or not to return a `FlowMatchEulerDiscreteSchedulerOutput` or tuple.

**Returns:**

``FlowMatchEulerDiscreteSchedulerOutput` or `tuple``

If return_dict is `True`,
`FlowMatchEulerDiscreteSchedulerOutput` is returned,
otherwise a tuple is returned where the first element is the sample tensor.
#### stretch_shift_to_terminal[[diffusers.FlowMatchEulerDiscreteScheduler.stretch_shift_to_terminal]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L262)

Stretches and shifts the timestep schedule to ensure it terminates at the configured `shift_terminal` config
value.

Reference:
https://github.com/Lightricks/LTX-Video/blob/a01a171f8fe3d99dce2728d60a73fecf4d4238ae/ltx_video/schedulers/rf.py#L51

**Parameters:**

t (`torch.Tensor`) : A tensor of timesteps to be stretched and shifted.

**Returns:**

``torch.Tensor``

A tensor of adjusted timesteps such that the final value equals `self.config.shift_terminal`.
#### time_shift[[diffusers.FlowMatchEulerDiscreteScheduler.time_shift]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L241)

Apply time shifting to the sigmas.

**Parameters:**

mu (`float`) : The mu parameter for the time shift.

sigma (`float`) : The sigma parameter for the time shift.

t (`torch.Tensor`) : The input timesteps.

**Returns:**

``torch.Tensor``

The time-shifted timesteps.
