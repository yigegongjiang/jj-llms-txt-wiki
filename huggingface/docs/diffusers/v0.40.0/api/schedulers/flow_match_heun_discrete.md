# FlowMatchHeunDiscreteScheduler

`FlowMatchHeunDiscreteScheduler` is based on the flow-matching sampling introduced in [EDM](https://huggingface.co/papers/2403.03206).

## FlowMatchHeunDiscreteScheduler[[diffusers.FlowMatchHeunDiscreteScheduler]]

#### diffusers.FlowMatchHeunDiscreteScheduler[[diffusers.FlowMatchHeunDiscreteScheduler]]

```python
diffusers.FlowMatchHeunDiscreteScheduler(num_train_timesteps: int = 1000, shift: float = 1.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_heun_discrete.py#L43)

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

shift (`float`, defaults to 1.0) : The shift value for the timestep schedule.

Heun scheduler.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### index_for_timestep[[diffusers.FlowMatchHeunDiscreteScheduler.index_for_timestep]]

```python
index_for_timestep(timestep: typing.Union[float, torch.FloatTensor], schedule_timesteps: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_heun_discrete.py#L179)

**Parameters:**

timestep (`float` or `torch.FloatTensor`) : The timestep value to find in the schedule.

schedule_timesteps (`torch.FloatTensor`, *optional*) : The timestep schedule to search in. If `None`, uses `self.timesteps`.

**Returns:** `int`

The index of the timestep in the schedule.

Find the index of a given timestep in the timestep schedule.

#### scale_noise[[diffusers.FlowMatchHeunDiscreteScheduler.scale_noise]]

```python
scale_noise(sample: FloatTensor, timestep: typing.Union[float, torch.FloatTensor], noise: FloatTensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_heun_discrete.py#L106)

**Parameters:**

sample (`torch.FloatTensor`) : The input sample.

timestep (`float` or `torch.FloatTensor`) : The current timestep in the diffusion chain.

noise (`torch.FloatTensor`) : The noise tensor.

**Returns:** `torch.FloatTensor`

A scaled input sample.

Forward process in flow-matching

#### set_begin_index[[diffusers.FlowMatchHeunDiscreteScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_heun_discrete.py#L96)

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

#### set_timesteps[[diffusers.FlowMatchHeunDiscreteScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int, device: typing.Union[str, torch.device] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_heun_discrete.py#L139)

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.FlowMatchHeunDiscreteScheduler.step]]

```python
step(model_output: FloatTensor, timestep: typing.Union[float, torch.FloatTensor], sample: FloatTensor, s_churn: float = 0.0, s_tmin: float = 0.0, s_tmax: float = inf, s_noise: float = 1.0, generator: typing.Optional[torch.Generator] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_match_heun_discrete.py#L225)

**Parameters:**

model_output (`torch.FloatTensor`) : The direct output from learned diffusion model.

timestep (`float` or `torch.FloatTensor`) : The current discrete timestep in the diffusion chain.

sample (`torch.FloatTensor`) : A current instance of a sample created by the diffusion process.

s_churn (`float`) : Stochasticity parameter that controls the amount of noise added during sampling. Higher values increase randomness.

s_tmin (`float`) : Minimum timestep threshold for applying stochasticity. Only timesteps above this value will have noise added.

s_tmax (`float`) : Maximum timestep threshold for applying stochasticity. Only timesteps below this value will have noise added.

s_noise (`float`, defaults to 1.0) : Scaling factor for noise added to the sample.

generator (`torch.Generator`, *optional*) : A random number generator.

return_dict (`bool`) : Whether or not to return a `FlowMatchHeunDiscreteSchedulerOutput` tuple.

**Returns:** `FlowMatchHeunDiscreteSchedulerOutput` or `tuple`

If return_dict is `True`,
`FlowMatchHeunDiscreteSchedulerOutput` is returned,
otherwise a tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).
