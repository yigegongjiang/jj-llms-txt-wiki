# MiniMaxH3Scheduler

`MiniMaxH3Scheduler` is the rectified-flow Euler scheduler (`eta = 0`) with an exponential sigma shift used by [MiniMax-H3](https://huggingface.co/MiniMaxAI), `sigma' = s * sigma / (1 + (s - 1) * sigma)`.

The MiniMax-H3 pipelines register **two** of them, because video and audio latents step down two different schedules inside a single transformer call per step: `scheduler` carries the video schedule (`shift=12.0` in the released checkpoints) and `audio_scheduler` the audio one (`shift=3.0`).

## MiniMaxH3Scheduler[[diffusers.MiniMaxH3Scheduler]]

#### diffusers.MiniMaxH3Scheduler[[diffusers.MiniMaxH3Scheduler]]

```python
diffusers.MiniMaxH3Scheduler(shift: float = 12.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_minimax_h3.py#L60)

**Parameters:**

shift (`float`, defaults to `12.0`) : Exponential shift applied to the sigma grid, `sigma' = s*sigma / (1 + (s-1)*sigma)`. The released checkpoints use `12.0` for video latents and `3.0` for audio latents.

Rectified-flow Euler scheduler (`eta = 0`) with an exponential sigma shift, as used by MiniMax-H3.

#### index_for_timestep[[diffusers.MiniMaxH3Scheduler.index_for_timestep]]

```python
index_for_timestep(timestep: typing.Union[float, torch.Tensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_minimax_h3.py#L172)

**Parameters:**

timestep (`float` or `torch.Tensor`) : A value taken from `self.timesteps`. The schedule is strictly increasing in `t`, so the match is unique.

**Returns:** `int`

The index of `timestep`.

Map a timestep value to its index in the schedule.

#### scale_noise[[diffusers.MiniMaxH3Scheduler.scale_noise]]

```python
scale_noise(sample: FloatTensor, timestep: typing.Union[float, torch.FloatTensor], noise: FloatTensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_minimax_h3.py#L193)

**Parameters:**

sample (`torch.FloatTensor`) : The clean sample `x_0`.

timestep (`float` or `torch.FloatTensor`) : The target time in `[0, 1]`; `1` returns `sample` unchanged.

noise (`torch.FloatTensor`) : The noise to mix in.

**Returns:** `torch.FloatTensor`

The noised sample.

Rectified-flow forward process, in MiniMax-H3's `t` convention: `x_t = t*x_0 + (1 - t)*noise`.

MiniMax-H3 uses this to noise its conditioning anchors, where `t` is the `noise_aug` level rather than a
schedule entry, so `timestep` is taken at face value and is *not* looked up in `self.timesteps`.

#### set_begin_index[[diffusers.MiniMaxH3Scheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_minimax_h3.py#L100)

**Parameters:**

begin_index (`int`, defaults to `0`) : The begin index for the scheduler.

Sets the begin index for the scheduler.

#### set_shift[[diffusers.MiniMaxH3Scheduler.set_shift]]

```python
set_shift(shift: float)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_minimax_h3.py#L110)

**Parameters:**

shift (`float`) : The exponential shift to use for the next schedule.

Overrides the configured sigma shift; call before [set_timesteps()](/docs/diffusers/v0.40.0/en/api/schedulers/minimax_h3#diffusers.MiniMaxH3Scheduler.set_timesteps).

MiniMax-H3 exposes this per request as `flow_shift` (video) / `audio_flow_shift` (audio).

#### set_timesteps[[diffusers.MiniMaxH3Scheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int | None = None, device: typing.Union[str, torch.device, NoneType] = None, sigmas: typing.Union[list[float], torch.Tensor, NoneType] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_minimax_h3.py#L124)

**Parameters:**

num_inference_steps (`int`, *optional*) : Number of sigma grid points, terminal `0` included. Ignored when `sigmas` is given.

device (`str` or `torch.device`, *optional*) : Device the schedule tensors are moved to. The grid itself is always built on CPU in float32 so the schedule does not depend on the accelerator.

sigmas (`list[float]` or `torch.Tensor`, *optional*) : A fully-formed sigma schedule, used verbatim (no shifting, no deduplication). It must be strictly decreasing and terminate at `0.0`.

Build the sigma / timestep schedule.

The grid is `linspace(1, 0, num_inference_steps)` pushed through the exponential shift, with consecutive
duplicates collapsed. The terminal `0` is already part of that grid — the shift maps `0` to exactly `0` — so
the schedule holds `num_inference_steps` sigmas and drives `num_inference_steps - 1` model evaluations, exposed
as `self.timesteps = 1 - sigmas[:-1]`.

#### step[[diffusers.MiniMaxH3Scheduler.step]]

```python
step(model_output: FloatTensor, timestep: typing.Union[float, torch.FloatTensor], sample: FloatTensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_minimax_h3.py#L223)

**Parameters:**

model_output (`torch.FloatTensor`) : The transformer's velocity prediction at `timestep`.

timestep (`float` or `torch.FloatTensor`) : The current timestep, one of `self.timesteps` (so `timestep == 1 - sigma`).

sample (`torch.FloatTensor`) : The current sample `x_t`.

return_dict (`bool`, defaults to `True`) : Whether to return a `MiniMaxH3SchedulerOutput` instead of a plain tuple.

**Returns:** `MiniMaxH3SchedulerOutput` or `tuple`

the sample for the next step.

Take one Euler (`eta = 0`) step.

The model output is a data-ward velocity, so the denoised estimate is `x0 = x_t + (1 - t) * v` — note the `+`,
the opposite of the usual flow-match convention. The update is then the blend `x_next = r*x_t + (1 - r)*x0`
with `r = sigma_next / sigma`, evaluated in float32 for half-precision samples.

## MiniMaxH3SchedulerOutput[[diffusers.schedulers.scheduling_minimax_h3.MiniMaxH3SchedulerOutput]]

#### diffusers.schedulers.scheduling_minimax_h3.MiniMaxH3SchedulerOutput[[diffusers.schedulers.scheduling_minimax_h3.MiniMaxH3SchedulerOutput]]

```python
diffusers.schedulers.scheduling_minimax_h3.MiniMaxH3SchedulerOutput(prev_sample: FloatTensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_minimax_h3.py#L48)

**Parameters:**

prev_sample (`torch.FloatTensor`) : Computed sample `x_{t+1}` for the next step of the denoising loop.

Output class for the scheduler's `step` function output.
