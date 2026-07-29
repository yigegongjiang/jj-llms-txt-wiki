# HeliosDMDScheduler

`HeliosDMDScheduler` is based on the pyramidal flow-matching sampling introduced in [Helios](https://huggingface.co/papers).

## HeliosDMDScheduler[[diffusers.HeliosDMDScheduler]]
#### diffusers.HeliosDMDScheduler[[diffusers.HeliosDMDScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L35)

init_sigmasdiffusers.HeliosDMDScheduler.init_sigmashttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L69[]

initialize the global timesteps and sigmas
#### init_sigmas_for_each_stage[[diffusers.HeliosDMDScheduler.init_sigmas_for_each_stage]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L87)

Init the timesteps for each stage
#### set_begin_index[[diffusers.HeliosDMDScheduler.set_begin_index]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L161)

Sets the begin index for the scheduler. This function should be run from pipeline before the inference.

**Parameters:**

begin_index (`int`) : The begin index for the scheduler.
#### set_timesteps[[diffusers.HeliosDMDScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L174)

Setting the timesteps and sigmas for each stage
#### time_shift[[diffusers.HeliosDMDScheduler.time_shift]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_helios_dmd.py#L236)

Apply time shifting to the sigmas.

**Parameters:**

mu (`float`) : The mu parameter for the time shift.

sigma (`float`) : The sigma parameter for the time shift.

t (`torch.Tensor`) : The input timesteps.

**Returns:**

``torch.Tensor``

The time-shifted timesteps.

scheduling_helios_dmd
