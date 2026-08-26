# LMSDiscreteScheduler

`LMSDiscreteScheduler` is a linear multistep scheduler for discrete beta schedules. The scheduler is ported from and created by [Katherine Crowson](https://github.com/crowsonkb/), and the original implementation can be found at [crowsonkb/k-diffusion](https://github.com/crowsonkb/k-diffusion/blob/481677d114f6ea445aa009cf5bd7a9cdee909e47/k_diffusion/sampling.py#L181).

## LMSDiscreteScheduler[[diffusers.LMSDiscreteScheduler]]

#### diffusers.LMSDiscreteScheduler[[diffusers.LMSDiscreteScheduler]]

```python
diffusers.LMSDiscreteScheduler(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/utils/dummy_torch_and_scipy_objects.py#L5)

## LMSDiscreteSchedulerOutput[[diffusers.schedulers.scheduling_lms_discrete.LMSDiscreteSchedulerOutput]]

#### diffusers.schedulers.scheduling_lms_discrete.LMSDiscreteSchedulerOutput[[diffusers.schedulers.scheduling_lms_discrete.LMSDiscreteSchedulerOutput]]

```python
diffusers.schedulers.scheduling_lms_discrete.LMSDiscreteSchedulerOutput(prev_sample: Tensor, pred_original_sample: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_lms_discrete.py#L34)

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

pred_original_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : The predicted denoised sample `(x_{0})` based on the model output from the current timestep. `pred_original_sample` can be used to preview progress or for guidance.

Output class for the scheduler's `step` function output.
