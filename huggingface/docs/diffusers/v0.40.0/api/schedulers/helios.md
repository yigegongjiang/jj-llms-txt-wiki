# HeliosScheduler

`HeliosScheduler` is based on the pyramidal flow-matching sampling introduced in [Helios](https://huggingface.co/papers).

## HeliosScheduler[[diffusers.HeliosScheduler]]

#### diffusers.HeliosScheduler[[diffusers.HeliosScheduler]]

```python
diffusers.HeliosScheduler(num_train_timesteps: int = 1000, shift: float = 1.0, stages: int = 3, stage_range: list = [0, 0.3333333333333333, 0.6666666666666666, 1], gamma: float = 0.3333333333333333, thresholding: bool = False, prediction_type: str = 'flow_prediction', solver_order: int = 2, predict_x0: bool = True, solver_type: str = 'bh2', lower_order_final: bool = True, disable_corrector: list = [], solver_p: SchedulerMixin = None, use_flow_sigmas: bool = True, scheduler_type: str = 'unipc', use_dynamic_shifting: bool = False, time_shift_type: typing.Literal['exponential', 'linear'] = 'exponential')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios.py#L35)

#### convert_model_output[[diffusers.HeliosScheduler.convert_model_output]]

```python
convert_model_output(model_output: Tensor, *args, sample: Tensor = None, sigma: Tensor = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios.py#L371)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

sigma (`torch.Tensor`, *optional*) : The sigma of the current step in the noise schedule.

**Returns:** `torch.Tensor`

The converted model output.

Convert the model output to the corresponding type the UniPC algorithm needs.

#### init_sigmas[[diffusers.HeliosScheduler.init_sigmas]]

```python
init_sigmas()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios.py#L90)

initialize the global timesteps and sigmas

#### init_sigmas_for_each_stage[[diffusers.HeliosScheduler.init_sigmas_for_each_stage]]

```python
init_sigmas_for_each_stage()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios.py#L108)

Init the timesteps for each stage

#### multistep_uni_c_bh_update[[diffusers.HeliosScheduler.multistep_uni_c_bh_update]]

```python
multistep_uni_c_bh_update(this_model_output: Tensor, *args, last_sample: Tensor = None, this_sample: Tensor = None, order: int = None, sigma_before: Tensor = None, sigma: Tensor = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios.py#L591)

**Parameters:**

this_model_output (`torch.Tensor`) : The model outputs at `x_t`.

this_timestep (`int`) : The current timestep `t`.

last_sample (`torch.Tensor`) : The generated sample before the last predictor `x_{t-1}`.

this_sample (`torch.Tensor`) : The generated sample after the last predictor `x_{t}`.

order (`int`) : The `p` of UniC-p at this step. The effective order of accuracy should be `order + 1`.

sigma_before (`torch.Tensor`, *optional*) : The sigma of the previous step in the noise schedule.

sigma (`torch.Tensor`, *optional*) : The sigma of the current step in the noise schedule.

**Returns:** `torch.Tensor`

The corrected sample tensor at the current timestep.

One step for the UniC (B(h) version).

#### multistep_uni_p_bh_update[[diffusers.HeliosScheduler.multistep_uni_p_bh_update]]

```python
multistep_uni_p_bh_update(model_output: Tensor, *args, sample: Tensor = None, order: int = None, sigma: Tensor = None, sigma_next: Tensor = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios.py#L453)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model at the current timestep.

prev_timestep (`int`) : The previous discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

order (`int`) : The order of UniP at this timestep (corresponds to the *p* in UniPC-p).

sigma (`torch.Tensor`, *optional*) : The sigma of the current step in the noise schedule.

sigma_next (`torch.Tensor`, *optional*) : The sigma of the next step in the noise schedule.

**Returns:** `torch.Tensor`

The sample tensor at the previous timestep.

One step for the UniP (B(h) version). Alternatively, `self.solver_p` is used if is specified.

#### set_begin_index[[diffusers.HeliosScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios.py#L182)

**Parameters:**

begin_index (`int`) : The begin index for the scheduler.

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

#### set_timesteps[[diffusers.HeliosScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int, stage_index: int | None = None, device: typing.Union[str, torch.device] = None, sigmas: bool | None = None, mu: bool | None = None, is_amplify_first_chunk: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios.py#L195)

Setting the timesteps and sigmas for each stage

#### time_shift[[diffusers.HeliosScheduler.time_shift]]

```python
time_shift(mu: float, sigma: float, t: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_helios.py#L259)

**Parameters:**

mu (`float`) : The mu parameter for the time shift.

sigma (`float`) : The sigma parameter for the time shift.

t (`torch.Tensor`) : The input timesteps.

**Returns:** `torch.Tensor`

The time-shifted timesteps.

Apply time shifting to the sigmas.

scheduling_helios
