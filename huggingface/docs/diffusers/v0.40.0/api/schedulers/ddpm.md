# DDPMScheduler

[Denoising Diffusion Probabilistic Models](https://huggingface.co/papers/2006.11239) (DDPM) by Jonathan Ho, Ajay Jain and Pieter Abbeel proposes a diffusion based model of the same name. In the context of the 🤗 Diffusers library, DDPM refers to the discrete denoising scheduler from the paper as well as the pipeline.

The abstract from the paper is:

*We present high quality image synthesis results using diffusion probabilistic models, a class of latent variable models inspired by considerations from nonequilibrium thermodynamics. Our best results are obtained by training on a weighted variational bound designed according to a novel connection between diffusion probabilistic models and denoising score matching with Langevin dynamics, and our models naturally admit a progressive lossy decompression scheme that can be interpreted as a generalization of autoregressive decoding. On the unconditional CIFAR10 dataset, we obtain an Inception score of 9.46 and a state-of-the-art FID score of 3.17. On 256x256 LSUN, we obtain sample quality similar to ProgressiveGAN. Our implementation is available at [this https URL](https://github.com/hojonathanho/diffusion).*

## DDPMScheduler[[diffusers.DDPMScheduler]]

#### diffusers.DDPMScheduler[[diffusers.DDPMScheduler]]

```python
diffusers.DDPMScheduler(num_train_timesteps: int = 1000, beta_start: float = 0.0001, beta_end: float = 0.02, beta_schedule: typing.Literal['linear', 'scaled_linear', 'squaredcos_cap_v2', 'sigmoid'] = 'linear', trained_betas: numpy.ndarray | list[float] | None = None, variance_type: typing.Literal['fixed_small', 'fixed_small_log', 'fixed_large', 'fixed_large_log', 'learned', 'learned_range'] = 'fixed_small', clip_sample: bool = True, prediction_type: typing.Literal['epsilon', 'sample', 'v_prediction'] = 'epsilon', thresholding: bool = False, dynamic_thresholding_ratio: float = 0.995, clip_sample_range: float = 1.0, sample_max_value: float = 1.0, timestep_spacing: typing.Literal['linspace', 'leading', 'trailing'] = 'leading', steps_offset: int = 0, rescale_betas_zero_snr: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddpm.py#L137)

**Parameters:**

num_train_timesteps (`int`, defaults to `1000`) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to `0.0001`) : The starting `beta` value of inference.

beta_end (`float`, defaults to `0.02`) : The final `beta` value.

beta_schedule (`"linear"`, `"scaled_linear"`, `"squaredcos_cap_v2"`, or `"sigmoid"`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model.

trained_betas (`np.ndarray`, *optional*) : An array of betas to pass directly to the constructor without using `beta_start` and `beta_end`.

variance_type (`"fixed_small"`, `"fixed_small_log"`, `"fixed_large"`, `"fixed_large_log"`, `"learned"`, or `"learned_range"`, defaults to `"fixed_small"`) : Clip the variance when adding noise to the denoised sample.

clip_sample (`bool`, defaults to `True`) : Clip the predicted sample for numerical stability.

clip_sample_range (`float`, defaults to `1.0`) : The maximum magnitude for sample clipping. Valid only when `clip_sample=True`.

prediction_type (`"epsilon"`, `"sample"`, or `"v_prediction"`, defaults to `"epsilon"`) : Prediction type of the scheduler function; can be `epsilon` (predicts the noise of the diffusion process), `sample` (directly predicts the noisy sample`) or `v_prediction` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

thresholding (`bool`, defaults to `False`) : Whether to use the "dynamic thresholding" method. This is unsuitable for latent-space diffusion models such as Stable Diffusion.

dynamic_thresholding_ratio (`float`, defaults to `0.995`) : The ratio for the dynamic thresholding method. Valid only when `thresholding=True`.

sample_max_value (`float`, defaults to `1.0`) : The threshold value for dynamic thresholding. Valid only when `thresholding=True`.

timestep_spacing (`"linspace"`, `"leading"`, or `"trailing"`, defaults to `"leading"`) : The way the timesteps should be scaled. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

steps_offset (`int`, defaults to `0`) : An offset added to the inference steps, as required by some model families.

rescale_betas_zero_snr (`bool`, defaults to `False`) : Whether to rescale the betas to have zero terminal SNR. This enables the model to generate very bright and dark samples instead of limiting it to samples with medium brightness. Loosely related to [`--offset_noise`](https://github.com/huggingface/diffusers/blob/74fd735eb073eb1d774b1ab4154a0876eb82f055/examples/dreambooth/train_dreambooth.py#L506).

`DDPMScheduler` explores the connections between denoising score matching and Langevin dynamics sampling.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### add_noise[[diffusers.DDPMScheduler.add_noise]]

```python
add_noise(original_samples: Tensor, noise: Tensor, timesteps: IntTensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddpm.py#L569)

**Parameters:**

original_samples (`torch.Tensor`) : The original samples to which noise will be added.

noise (`torch.Tensor`) : The noise to add to the samples.

timesteps (`torch.IntTensor`) : The timesteps indicating the noise level for each sample.

**Returns:** `torch.Tensor`

The noisy samples.

Add noise to the original samples according to the noise magnitude at each timestep (this is the forward
diffusion process).

#### get_velocity[[diffusers.DDPMScheduler.get_velocity]]

```python
get_velocity(sample: Tensor, noise: Tensor, timesteps: IntTensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddpm.py#L611)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

noise (`torch.Tensor`) : The noise tensor.

timesteps (`torch.IntTensor`) : The timesteps for velocity computation.

**Returns:** `torch.Tensor`

The computed velocity.

Compute the velocity prediction from the sample and noise according to the velocity formula.

#### previous_timestep[[diffusers.DDPMScheduler.previous_timestep]]

```python
previous_timestep(timestep: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddpm.py#L648)

**Parameters:**

timestep (`int`) : The current timestep.

**Returns:** `int` or `torch.Tensor`

The previous timestep.

Compute the previous timestep in the diffusion chain.

#### scale_model_input[[diffusers.DDPMScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, timestep: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddpm.py#L257)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`int`, *optional*) : The current timestep in the diffusion chain.

**Returns:** `torch.Tensor`

A scaled input sample.

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

#### set_timesteps[[diffusers.DDPMScheduler.set_timesteps]]

```python
set_timesteps(num_inference_steps: int = None, device: typing.Union[str, torch.device] = None, timesteps: list[int] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddpm.py#L274)

**Parameters:**

num_inference_steps (`int`, *optional*) : The number of diffusion steps used when generating samples with a pre-trained model. If used, `timesteps` must be `None`.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.

timesteps (`list[int]`, *optional*) : Custom timesteps used to support arbitrary spacing between timesteps. If `None`, then the default timestep spacing strategy of equal spacing between timesteps is used. If `timesteps` is passed, `num_inference_steps` must be `None`.

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

#### step[[diffusers.DDPMScheduler.step]]

```python
step(model_output: Tensor, timestep: int, sample: Tensor, generator: typing.Optional[torch.Generator] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddpm.py#L461)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator.

return_dict (`bool`, defaults to `True`) : Whether or not to return a [DDPMSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/ddpm#diffusers.schedulers.scheduling_ddpm.DDPMSchedulerOutput) or `tuple`.

**Returns:** [DDPMSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/ddpm#diffusers.schedulers.scheduling_ddpm.DDPMSchedulerOutput) or `tuple`

If return_dict is `True`, [DDPMSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/ddpm#diffusers.schedulers.scheduling_ddpm.DDPMSchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

## DDPMSchedulerOutput[[diffusers.schedulers.scheduling_ddpm.DDPMSchedulerOutput]]

#### diffusers.schedulers.scheduling_ddpm.DDPMSchedulerOutput[[diffusers.schedulers.scheduling_ddpm.DDPMSchedulerOutput]]

```python
diffusers.schedulers.scheduling_ddpm.DDPMSchedulerOutput(prev_sample: Tensor, pred_original_sample: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_ddpm.py#L31)

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

pred_original_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : The predicted denoised sample `(x_{0})` based on the model output from the current timestep. `pred_original_sample` can be used to preview progress or for guidance.

Output class for the scheduler's `step` function output.
