# HeliosScheduler

`HeliosScheduler` is based on the pyramidal flow-matching sampling introduced in [Helios](https://huggingface.co/papers).

## HeliosScheduler[[diffusers.HeliosScheduler]]
#### diffusers.HeliosScheduler[[diffusers.HeliosScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios.py#L35)

convert_model_outputdiffusers.HeliosScheduler.convert_model_outputhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios.py#L371[{"name": "model_output", "val": ": Tensor"}, {"name": "*args", "val": ""}, {"name": "sample", "val": ": Tensor = None"}, {"name": "sigma", "val": ": Tensor = None"}, {"name": "**kwargs", "val": ""}]- **model_output** (`torch.Tensor`) --
  The direct output from the learned diffusion model.
- **timestep** (`int`) --
  The current discrete timestep in the diffusion chain.
- **sample** (`torch.Tensor`) --
  A current instance of a sample created by the diffusion process.0`torch.Tensor`The converted model output.

Convert the model output to the corresponding type the UniPC algorithm needs.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

**Returns:**

``torch.Tensor``

The converted model output.
#### init_sigmas[[diffusers.HeliosScheduler.init_sigmas]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios.py#L90)

initialize the global timesteps and sigmas
#### init_sigmas_for_each_stage[[diffusers.HeliosScheduler.init_sigmas_for_each_stage]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios.py#L108)

Init the timesteps for each stage
#### multistep_uni_c_bh_update[[diffusers.HeliosScheduler.multistep_uni_c_bh_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios.py#L585)

One step for the UniC (B(h) version).

**Parameters:**

this_model_output (`torch.Tensor`) : The model outputs at `x_t`.

this_timestep (`int`) : The current timestep `t`.

last_sample (`torch.Tensor`) : The generated sample before the last predictor `x_{t-1}`.

this_sample (`torch.Tensor`) : The generated sample after the last predictor `x_{t}`.

order (`int`) : The `p` of UniC-p at this step. The effective order of accuracy should be `order + 1`.

**Returns:**

``torch.Tensor``

The corrected sample tensor at the current timestep.
#### multistep_uni_p_bh_update[[diffusers.HeliosScheduler.multistep_uni_p_bh_update]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios.py#L451)

One step for the UniP (B(h) version). Alternatively, `self.solver_p` is used if is specified.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model at the current timestep.

prev_timestep (`int`) : The previous discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

order (`int`) : The order of UniP at this timestep (corresponds to the *p* in UniPC-p).

**Returns:**

``torch.Tensor``

The sample tensor at the previous timestep.
#### set_begin_index[[diffusers.HeliosScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios.py#L182)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.HeliosScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios.py#L195)

Setting the timesteps and sigmas for each stage
#### time_shift[[diffusers.HeliosScheduler.time_shift]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios.py#L259)

Apply time shifting to the sigmas.

**Parameters:**

mu (`float`) : The mu parameter for the time shift.

sigma (`float`) : The sigma parameter for the time shift.

t (`torch.Tensor`) : The input timesteps.

**Returns:**

``torch.Tensor``

The time-shifted timesteps.

scheduling_helios
