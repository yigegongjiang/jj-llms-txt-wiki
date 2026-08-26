# FlowMatchEulerDiscreteScheduler

`FlowMatchEulerDiscreteScheduler` is based on the flow-matching sampling introduced in [Stable Diffusion 3](https://huggingface.co/papers/2403.03206).

## FlowMatchEulerDiscreteScheduler[[diffusers.FlowMatchEulerDiscreteScheduler]]

#### diffusers.FlowMatchEulerDiscreteScheduler[[diffusers.FlowMatchEulerDiscreteScheduler]]

```python
diffusers.FlowMatchEulerDiscreteScheduler(num_train_timesteps: int = 1000, shift: float = 1.0, use_dynamic_shifting: bool = False, base_shift: float | None = 0.5, max_shift: float | None = 1.15, base_image_seq_len: int = 256, max_image_seq_len: int = 4096, invert_sigmas: bool = False, shift_terminal: float = None, use_karras_sigmas: bool = False, use_exponential_sigmas: bool = False, use_beta_sigmas: bool = False, time_shift_type: typing.Literal['exponential', 'linear'] = 'exponential', stochastic_sampling: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L48)

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

Euler scheduler.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### index_for_timestep[[diffusers.FlowMatchEulerDiscreteScheduler.index_for_timestep]]

```python
index_for_timestep(timestep: typing.Union[float, torch.FloatTensor], schedule_timesteps: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L384)

**Parameters:**

timestep (`float` or `torch.FloatTensor`) : The timestep to find the index for.

schedule_timesteps (`torch.FloatTensor`, *optional*) : The schedule timesteps to validate against. If `None`, the scheduler's timesteps are used.

**Returns:** `int`

The index of the timestep.

Get the index for the given timestep.

#### scale_noise[[diffusers.FlowMatchEulerDiscreteScheduler.scale_noise]]

```python
scale_noise(sample: FloatTensor, timestep: typing.Union[float, torch.FloatTensor], noise: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L188)

**Parameters:**

sample (`torch.FloatTensor`) : The input sample.

timestep (`torch.FloatTensor`) : The current timestep in the diffusion chain.

noise (`torch.FloatTensor`) : The noise tensor.

**Returns:** `torch.FloatTensor`

A scaled input sample.

Forward process in flow-matching

#### set_begin_index[[diffusers.FlowMatchEulerDiscreteScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L168)

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

#### set_shift[[diffusers.FlowMatchEulerDiscreteScheduler.set_shift]]

```python
set_shift(shift: float)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L178)

**Parameters:**

shift (`float`) : The shift value to be set.

Sets the shift value for the scheduler.

#### set_timesteps[[diffusers.FlowMatchEulerDiscreteScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int | None = None, device: typing.Union[str, torch.device] = None, sigmas: list[float] | None = None, mu: float | None = None, timesteps: list[float] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L283)

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

sigmas (`list[float]`, *optional*) : Custom values for sigmas to be used for each diffusion step. If `None`, the sigmas are computed automatically.

mu (`float`, *optional*) : Determines the amount of shifting applied to sigmas when performing resolution-dependent timestep shifting.

timesteps (`list[float]`, *optional*) : Custom values for timesteps to be used for each diffusion step. If `None`, the timesteps are computed automatically.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.FlowMatchEulerDiscreteScheduler.step]]

```python
step(model_output: FloatTensor, timestep: typing.Union[float, torch.FloatTensor], sample: FloatTensor, s_churn: float = 0.0, s_tmin: float = 0.0, s_tmax: float = inf, s_noise: float = 1.0, generator: typing.Optional[torch.Generator] = None, per_token_timesteps: typing.Optional[torch.Tensor] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L423)

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

**Returns:** `FlowMatchEulerDiscreteSchedulerOutput` or `tuple`

If return_dict is `True`,
`FlowMatchEulerDiscreteSchedulerOutput` is returned,
otherwise a tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

#### stretch_shift_to_terminal[[diffusers.FlowMatchEulerDiscreteScheduler.stretch_shift_to_terminal]]

```python
stretch_shift_to_terminal(t: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L262)

**Parameters:**

t (`torch.Tensor`) : A tensor of timesteps to be stretched and shifted.

**Returns:** `torch.Tensor`

A tensor of adjusted timesteps such that the final value equals `self.config.shift_terminal`.

Stretches and shifts the timestep schedule to ensure it terminates at the configured `shift_terminal` config
value.

Reference:
https://github.com/Lightricks/LTX-Video/blob/a01a171f8fe3d99dce2728d60a73fecf4d4238ae/ltx_video/schedulers/rf.py#L51

#### time_shift[[diffusers.FlowMatchEulerDiscreteScheduler.time_shift]]

```python
time_shift(mu: float, sigma: float, t: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_euler_discrete.py#L241)

**Parameters:**

mu (`float`) : The mu parameter for the time shift.

sigma (`float`) : The sigma parameter for the time shift.

t (`torch.Tensor`) : The input timesteps.

**Returns:** `torch.Tensor`

The time-shifted timesteps.

Apply time shifting to the sigmas.
