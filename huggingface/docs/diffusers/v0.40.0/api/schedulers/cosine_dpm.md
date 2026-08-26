# CosineDPMSolverMultistepScheduler

The [CosineDPMSolverMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/cosine_dpm#diffusers.CosineDPMSolverMultistepScheduler) is a variant of [DPMSolverMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/multistep_dpm_solver#diffusers.DPMSolverMultistepScheduler) with cosine schedule, proposed by Nichol and Dhariwal (2021).
It is being used in the [Stable Audio Open](https://huggingface.co/papers/2407.14358) paper and the [Stability-AI/stable-audio-tool](https://github.com/Stability-AI/stable-audio-tools) codebase.

This scheduler was contributed by [Yoach Lacombe](https://huggingface.co/ylacombe).

## CosineDPMSolverMultistepScheduler[[diffusers.CosineDPMSolverMultistepScheduler]]

#### diffusers.CosineDPMSolverMultistepScheduler[[diffusers.CosineDPMSolverMultistepScheduler]]

```python
diffusers.CosineDPMSolverMultistepScheduler(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/dummy_torch_and_torchsde_objects.py#L5)

## SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

#### diffusers.schedulers.scheduling_utils.SchedulerOutput[[diffusers.schedulers.scheduling_utils.SchedulerOutput]]

```python
diffusers.schedulers.scheduling_utils.SchedulerOutput(prev_sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_utils.py#L66)

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

Base class for the output of a scheduler's `step` function.
