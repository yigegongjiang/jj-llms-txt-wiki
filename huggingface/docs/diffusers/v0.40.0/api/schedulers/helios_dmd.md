# HeliosDMDScheduler

`HeliosDMDScheduler` is based on the pyramidal flow-matching sampling introduced in [Helios](https://huggingface.co/papers).

## HeliosDMDScheduler[[diffusers.HeliosDMDScheduler]]

#### diffusers.HeliosDMDScheduler[[diffusers.HeliosDMDScheduler]]

```python
diffusers.HeliosDMDScheduler(num_train_timesteps: int = 1000, shift: float = 1.0, stages: int = 3, stage_range: list = [0, 0.3333333333333333, 0.6666666666666666, 1], gamma: float = 0.3333333333333333, prediction_type: str = 'flow_prediction', use_flow_sigmas: bool = True, use_dynamic_shifting: bool = False, time_shift_type: typing.Literal['exponential', 'linear'] = 'linear')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L35)

#### init_sigmas[[diffusers.HeliosDMDScheduler.init_sigmas]]

```python
init_sigmas()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L69)

initialize the global timesteps and sigmas

#### init_sigmas_for_each_stage[[diffusers.HeliosDMDScheduler.init_sigmas_for_each_stage]]

```python
init_sigmas_for_each_stage()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L87)

Init the timesteps for each stage

#### set_begin_index[[diffusers.HeliosDMDScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L161)

**Parameters:**

begin_index (`int`) : The begin index for the scheduler.

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

#### set_timesteps[[diffusers.HeliosDMDScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int, stage_index: int | None = None, device: typing.Union[str, torch.device] = None, sigmas: bool | None = None, mu: bool | None = None, is_amplify_first_chunk: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L174)

Setting the timesteps and sigmas for each stage

#### time_shift[[diffusers.HeliosDMDScheduler.time_shift]]

```python
time_shift(mu: float, sigma: float, t: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L236)

**Parameters:**

mu (`float`) : The mu parameter for the time shift.

sigma (`float`) : The sigma parameter for the time shift.

t (`torch.Tensor`) : The input timesteps.

**Returns:** `torch.Tensor`

The time-shifted timesteps.

Apply time shifting to the sigmas.

scheduling_helios_dmd
