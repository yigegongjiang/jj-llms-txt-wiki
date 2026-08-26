# FlowMapEulerDiscreteScheduler

`FlowMapEulerDiscreteScheduler` is an Euler-style sampler designed for flow-map-distilled diffusion
models. Flow-map models learn arbitrary-interval transitions $\mathbf{z}_t \to \mathbf{z}_r$ rather than
the fixed $\mathbf{z}_t \to \mathbf{z}_0$ mapping of consistency models. Both endpoints of the step are
caller-provided, which is what enables any-step sampling: a single distilled checkpoint can be evaluated at
1, 2, 4, 8, 16... NFE without retraining.

The scheduler was introduced in
[AnyFlow: Any-Step Video Diffusion Model with On-Policy Flow Map Distillation](https://huggingface.co/papers/2605.13724)
and ships with the `AnyFlowPipeline` and `AnyFlowFARPipeline` integrations, but it is not
AnyFlow-specific — any flow-map-distilled checkpoint can use it.

## FlowMapEulerDiscreteScheduler[[diffusers.FlowMapEulerDiscreteScheduler]]

#### diffusers.FlowMapEulerDiscreteScheduler[[diffusers.FlowMapEulerDiscreteScheduler]]

```python
diffusers.FlowMapEulerDiscreteScheduler(num_train_timesteps: int = 1000, shift: float = 1.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_map_euler_discrete.py#L42)

**Parameters:**

num_train_timesteps (*int*, defaults to 1000) : The number of diffusion steps used to train the underlying flow-map model.

shift (*float*, defaults to 1.0) : Multiplicative timestep shift applied to the inference schedule. `shift=1.0` is the identity; values greater than 1.0 push the schedule toward more denoising at later steps (e.g., `shift=5` matches the Wan2.1 default).

Euler-style sampler for flow-map-distilled diffusion models.

Flow-map models learn arbitrary-interval transitions \\(z_t \to z_r\\) rather than the fixed \\(z_t \to
z_0\\) mapping of consistency models, so a single distilled checkpoint can be evaluated at 1, 2, 4, 8, ... NFE
without retraining. The *step* method advances the sample from *timestep* to *r_timestep* along the predicted
velocity.

Introduced in [AnyFlow: Any-Step Video Diffusion Model with On-Policy Flow Map
Distillation](https://huggingface.co/papers/2605.13724) by Yuchao Gu, Guian Fang et al.

This scheduler inherits from [*SchedulerMixin*] and [*ConfigMixin*]. Check the superclass documentation for the
generic methods implemented for all schedulers (loading, saving, etc.).

#### apply_shift[[diffusers.FlowMapEulerDiscreteScheduler.apply_shift]]

```python
apply_shift(sigmas: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_map_euler_discrete.py#L117)

Apply the configured shift transformation to a sigma tensor.

#### index_for_timestep[[diffusers.FlowMapEulerDiscreteScheduler.index_for_timestep]]

```python
index_for_timestep(timestep: typing.Union[float, torch.FloatTensor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_map_euler_discrete.py#L207)

Return the index of `timestep` on the current schedule, or `None` if off-schedule.

Lookup is done against `self.timesteps` with a small fp tolerance. Used to recover the corresponding sigma
without assuming the linear `timesteps = sigmas * num_train_timesteps` relationship — that way a custom
schedule (e.g. non-linear shift, manually-set timesteps) still resolves correctly.

#### scale_model_input[[diffusers.FlowMapEulerDiscreteScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, *args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_map_euler_discrete.py#L99)

No-op identity scaling. Provided for API compatibility with other Diffusers schedulers.

#### scale_noise[[diffusers.FlowMapEulerDiscreteScheduler.scale_noise]]

```python
scale_noise(sample: FloatTensor, timestep: typing.Union[float, torch.FloatTensor], noise: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_map_euler_discrete.py#L103)

Linearly interpolate `sample` toward `noise` according to the normalized `timestep`.

#### set_begin_index[[diffusers.FlowMapEulerDiscreteScheduler.set_begin_index]]

```python
set_begin_index(begin_index: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_map_euler_discrete.py#L94)

Set the begin index for the scheduler. Pipelines that start mid-schedule (e.g. image-to-image)
call this between `set_timesteps` and the first `step` to anchor the rollout.

#### set_timesteps[[diffusers.FlowMapEulerDiscreteScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: typing.Optional[int] = None, device: typing.Union[str, torch.device] = None, sigmas: typing.Optional[typing.List[float]] = None, timesteps: typing.Optional[typing.List[float]] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_map_euler_discrete.py#L123)

**Parameters:**

num_inference_steps (*int*, *optional*) : Number of inference steps. If `None`, must pass `sigmas` or `timesteps`.

device (*str* or *torch.device*, *optional*) : Target device for `self.sigmas` / `self.timesteps`.

sigmas (*List[float]*, *optional*) : Custom sigma schedule of length `num_inference_steps`. The terminal `0` sigma is appended automatically. The configured `shift` is applied on top.

timesteps (*List[float]*, *optional*) : Custom timestep schedule of length `num_inference_steps`, in the same units as `self.timesteps` (i.e. scaled by `num_train_timesteps`). Converted to sigmas internally. If both `sigmas` and `timesteps` are passed, their lengths must match.

Build the inference timestep schedule.

Internally tracks `self.sigmas` of length `num_inference_steps + 1` (the configured shift applied to a
linspace from `1.0` to `0.0` by default); `self.timesteps` exposes the first `num_inference_steps`
sigmas scaled by `num_train_timesteps` — i.e. one timestep per inference step, matching
[FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler). The final sigma (`0`) is the implicit
r-endpoint of the last step and is appended automatically when `sigmas` / `timesteps` are user-provided.

#### step[[diffusers.FlowMapEulerDiscreteScheduler.step]]

```python
step(model_output: FloatTensor, timestep: typing.Union[float, torch.FloatTensor], sample: FloatTensor, r_timestep: typing.Union[float, torch.FloatTensor, NoneType] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_flow_map_euler_discrete.py#L223)

**Parameters:**

model_output (*torch.Tensor*) : Direct output from the flow-map model (predicted mean velocity).

timestep (*float* or *torch.Tensor*) : Source timestep `t` in the same units as `self.timesteps`.

sample (*torch.Tensor*) : Current sample \\(z_t\\).

r_timestep (*float* or *torch.Tensor*, *optional*) : Target timestep `r`. Defaults to the next timestep on the schedule when `None`; pass an explicit value for any-step sampling. `r_timestep == timestep` is a no-op.

return_dict (*bool*, defaults to *True*) : Whether to return a [*FlowMapEulerDiscreteSchedulerOutput*] (the default) or a plain tuple.

**Returns:** [*FlowMapEulerDiscreteSchedulerOutput*] or *tuple*

When `return_dict=True`, returns a [*FlowMapEulerDiscreteSchedulerOutput*] whose `prev_sample` is
\\(z_r\\). Otherwise returns a 1-tuple `(prev_sample,)`.

Advance `sample` from `timestep` to `r_timestep` using the model-predicted velocity.

Unlike a standard Euler scheduler, both endpoints of the interval can be caller-provided so that any-step
sampling is possible: a single model call can step from *t* to any chosen target *r* (including *r=0* for a
one-shot generation). When `r_timestep` is omitted, it defaults to the next timestep on the schedule
(matching `FlowMatchEulerDiscreteScheduler` semantics).

Internally the source and target sigmas are recovered by indexing `self.sigmas` via
`index_for_timestep` rather than by dividing the input timesteps by `num_train_timesteps`, so any
schedule whose timestep / sigma relationship is non-linear (for example a custom shift) stays correct. For an
off-schedule `r_timestep`, the scheduler falls back to `r_timestep / num_train_timesteps` so any-step
sampling outside the schedule remains supported.
