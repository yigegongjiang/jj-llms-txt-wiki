# ScoreSdeVeScheduler

`ScoreSdeVeScheduler` is a variance exploding stochastic differential equation (SDE) scheduler. It was introduced in the [Score-Based Generative Modeling through Stochastic Differential Equations](https://huggingface.co/papers/2011.13456) paper by Yang Song, Jascha Sohl-Dickstein, Diederik P. Kingma, Abhishek Kumar, Stefano Ermon, Ben Poole.

The abstract from the paper is:

*Creating noise from data is easy; creating data from noise is generative modeling. We present a stochastic differential equation (SDE) that smoothly transforms a complex data distribution to a known prior distribution by slowly injecting noise, and a corresponding reverse-time SDE that transforms the prior distribution back into the data distribution by slowly removing the noise. Crucially, the reverse-time SDE depends only on the time-dependent gradient field (\aka, score) of the perturbed data distribution. By leveraging advances in score-based generative modeling, we can accurately estimate these scores with neural networks, and use numerical SDE solvers to generate samples. We show that this framework encapsulates previous approaches in score-based generative modeling and diffusion probabilistic modeling, allowing for new sampling procedures and new modeling capabilities. In particular, we introduce a predictor-corrector framework to correct errors in the evolution of the discretized reverse-time SDE. We also derive an equivalent neural ODE that samples from the same distribution as the SDE, but additionally enables exact likelihood computation, and improved sampling efficiency. In addition, we provide a new way to solve inverse problems with score-based models, as demonstrated with experiments on class-conditional generation, image inpainting, and colorization. Combined with multiple architectural improvements, we achieve record-breaking performance for unconditional image generation on CIFAR-10 with an Inception score of 9.89 and FID of 2.20, a competitive likelihood of 2.99 bits/dim, and demonstrate high fidelity generation of 1024 x 1024 images for the first time from a score-based generative model.*

## ScoreSdeVeScheduler[[diffusers.ScoreSdeVeScheduler]]

#### diffusers.ScoreSdeVeScheduler[[diffusers.ScoreSdeVeScheduler]]

```python
diffusers.ScoreSdeVeScheduler(num_train_timesteps: int = 2000, snr: float = 0.15, sigma_min: float = 0.01, sigma_max: float = 1348.0, sampling_eps: float = 1e-05, correct_steps: int = 1)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_sde_ve.py#L45)

**Parameters:**

num_train_timesteps (`int`, defaults to 2000) : The number of diffusion steps to train the model.

snr (`float`, defaults to 0.15) : A coefficient weighting the step from the `model_output` sample (from the network) to the random noise.

sigma_min (`float`, defaults to 0.01) : The initial noise scale for the sigma sequence in the sampling procedure. The minimum sigma should mirror the distribution of the data.

sigma_max (`float`, defaults to 1348.0) : The maximum value used for the range of continuous timesteps passed into the model.

sampling_eps (`float`, defaults to 1e-5) : The end value of sampling where timesteps decrease progressively from 1 to epsilon.

correct_steps (`int`, defaults to 1) : The number of correction steps performed on a produced sample.

`ScoreSdeVeScheduler` is a variance exploding stochastic differential equation (SDE) scheduler.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### add_noise[[diffusers.ScoreSdeVeScheduler.add_noise]]

```python
add_noise(original_samples: Tensor, noise: Tensor, timesteps: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_sde_ve.py#L300)

**Parameters:**

original_samples (`torch.Tensor`) : The original samples to which noise will be added.

noise (`torch.Tensor`) : The noise tensor to add to the original samples.

timesteps (`torch.Tensor`) : The timesteps at which to add noise, determining the noise level from the schedule.

**Returns:** `torch.Tensor`

The noisy samples with added noise scaled according to the timestep schedule.

Add noise to the original samples according to the noise schedule at the specified timesteps. This is the
forward diffusion process.

#### get_adjacent_sigma[[diffusers.ScoreSdeVeScheduler.get_adjacent_sigma]]

```python
get_adjacent_sigma(timesteps: Tensor, t: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_sde_ve.py#L156)

**Parameters:**

timesteps (`torch.Tensor`) : The tensor of discrete timestep indices.

t (`torch.Tensor`) : The tensor of continuous timestep values.

**Returns:** `torch.Tensor`

The sigma values for the adjacent (previous) timestep index.

Returns the sigma value corresponding to the previous timestep index. When the timestep is zero (no previous
timestep), returns zero.

#### scale_model_input[[diffusers.ScoreSdeVeScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, timestep: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_sde_ve.py#L88)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`int`, *optional*) : The current timestep in the diffusion chain.

**Returns:** `torch.Tensor`

A scaled input sample.

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

#### set_sigmas[[diffusers.ScoreSdeVeScheduler.set_sigmas]]

```python
set_sigmas(num_inference_steps: int, sigma_min: float | None = None, sigma_max: float | None = None, sampling_eps: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_sde_ve.py#L124)

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

sigma_min (`float`, *optional*) : The initial noise scale value (overrides value given during scheduler instantiation).

sigma_max (`float`, *optional*) : The final noise scale value (overrides value given during scheduler instantiation).

sampling_eps (`float`, *optional*) : The final timestep value (overrides value given during scheduler instantiation).

Sets the noise scales used for the diffusion chain (to be run before inference). The sigmas control the weight
of the `drift` and `diffusion` components of the sample update.

#### set_timesteps[[diffusers.ScoreSdeVeScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int, sampling_eps: float | None = None, device: typing.Union[str, torch.device, NoneType] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_sde_ve.py#L105)

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

sampling_eps (`float`, *optional*) : The final timestep value (overrides value given during scheduler instantiation).

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

Sets the continuous timesteps used for the diffusion chain (to be run before inference).

#### step_correct[[diffusers.ScoreSdeVeScheduler.step_correct]]

```python
step_correct(model_output: Tensor, sample: Tensor, generator: typing.Optional[torch.Generator] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_sde_ve.py#L245)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`.

**Returns:** [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) or `tuple`

If return_dict is `True`, [SchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/dpm_discrete_ancestral#diffusers.schedulers.scheduling_utils.SchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

Correct the predicted sample based on the `model_output` of the network. This is often run repeatedly after
making the prediction for the previous timestep.

#### step_pred[[diffusers.ScoreSdeVeScheduler.step_pred]]

```python
step_pred(model_output: Tensor, timestep: int, sample: Tensor, generator: typing.Optional[torch.Generator] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_sde_ve.py#L177)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [SdeVeOutput](/docs/diffusers/v0.40.0/en/api/schedulers/score_sde_ve#diffusers.schedulers.scheduling_sde_ve.SdeVeOutput) or `tuple`.

**Returns:** [SdeVeOutput](/docs/diffusers/v0.40.0/en/api/schedulers/score_sde_ve#diffusers.schedulers.scheduling_sde_ve.SdeVeOutput) or `tuple`

If return_dict is `True`, [SdeVeOutput](/docs/diffusers/v0.40.0/en/api/schedulers/score_sde_ve#diffusers.schedulers.scheduling_sde_ve.SdeVeOutput) is returned, otherwise a tuple
is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

## SdeVeOutput[[diffusers.schedulers.scheduling_sde_ve.SdeVeOutput]]

#### diffusers.schedulers.scheduling_sde_ve.SdeVeOutput[[diffusers.schedulers.scheduling_sde_ve.SdeVeOutput]]

```python
diffusers.schedulers.scheduling_sde_ve.SdeVeOutput(prev_sample: Tensor, prev_sample_mean: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_sde_ve.py#L29)

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

prev_sample_mean (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Mean averaged `prev_sample` over previous timesteps.

Output class for the scheduler's `step` function output.
