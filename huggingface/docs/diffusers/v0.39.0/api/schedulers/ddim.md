# DDIMScheduler

[Denoising Diffusion Implicit Models](https://huggingface.co/papers/2010.02502) (DDIM) by Jiaming Song, Chenlin Meng and Stefano Ermon.

The abstract from the paper is:

*Denoising diffusion probabilistic models (DDPMs) have achieved high quality image generation without adversarial training, yet they require simulating a Markov chain for many steps to produce a sample.
To accelerate sampling, we present denoising diffusion implicit models (DDIMs), a more efficient class of iterative implicit probabilistic models
with the same training procedure as DDPMs. In DDPMs, the generative process is defined as the reverse of a Markovian diffusion process.
We construct a class of non-Markovian diffusion processes that lead to the same training objective, but whose reverse process can be much faster to sample from.
We empirically demonstrate that DDIMs can produce high quality samples 10× to 50× faster in terms of wall-clock time compared to DDPMs, allow us to trade off computation for sample quality, and can perform semantically meaningful image interpolation directly in the latent space.*

The original codebase of this paper can be found at [ermongroup/ddim](https://github.com/ermongroup/ddim), and you can contact the author on [tsong.me](https://tsong.me/).

## Tips

The paper [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) claims that a mismatch between the training and inference settings leads to suboptimal inference generation results for Stable Diffusion. To fix this, the authors propose:

> [!WARNING]
> 🧪 This is an experimental feature!

1. rescale the noise schedule to enforce zero terminal signal-to-noise ratio (SNR)

```py
pipe.scheduler = DDIMScheduler.from_config(pipe.scheduler.config, rescale_betas_zero_snr=True)
```

2. train a model with `v_prediction` (add the following argument to the [train_text_to_image.py](https://github.com/huggingface/diffusers/blob/main/examples/text_to_image/train_text_to_image.py) or [train_text_to_image_lora.py](https://github.com/huggingface/diffusers/blob/main/examples/text_to_image/train_text_to_image_lora.py) scripts)

```bash
--prediction_type="v_prediction"
```

3. change the sampler to always start from the last timestep

```py
pipe.scheduler = DDIMScheduler.from_config(pipe.scheduler.config, timestep_spacing="trailing")
```

4. rescale classifier-free guidance to prevent over-exposure

```py
image = pipe(prompt, guidance_rescale=0.7).images[0]
```

For example:

```py
from diffusers import DiffusionPipeline, DDIMScheduler
import torch

pipe = DiffusionPipeline.from_pretrained("ptx0/pseudo-journey-v2", torch_dtype=torch.float16)
pipe.scheduler = DDIMScheduler.from_config(
    pipe.scheduler.config, rescale_betas_zero_snr=True, timestep_spacing="trailing"
)
pipe.to("cuda")

prompt = "A lion in galaxies, spirals, nebulae, stars, smoke, iridescent, intricate detail, octane render, 8k"
image = pipe(prompt, guidance_rescale=0.7).images[0]
image
```

## DDIMScheduler[[diffusers.DDIMScheduler]]
#### diffusers.DDIMScheduler[[diffusers.DDIMScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ddim.py#L139)

`DDIMScheduler` extends the denoising procedure introduced in denoising diffusion probabilistic models (DDPMs) with
non-Markovian guidance.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

add_noisediffusers.DDIMScheduler.add_noisehttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ddim.py#L517[{"name": "original_samples", "val": ": Tensor"}, {"name": "noise", "val": ": Tensor"}, {"name": "timesteps", "val": ": IntTensor"}]- **original_samples** (`torch.Tensor`) --
  The original samples to which noise will be added.
- **noise** (`torch.Tensor`) --
  The noise to add to the samples.
- **timesteps** (`torch.IntTensor`) --
  The timesteps indicating the noise level for each sample.0`torch.Tensor`The noisy samples.

Add noise to the original samples according to the noise magnitude at each timestep (this is the forward
diffusion process).

**Parameters:**

num_train_timesteps (`int`, defaults to 1000) : The number of diffusion steps to train the model.

beta_start (`float`, defaults to 0.0001) : The starting `beta` value of inference.

beta_end (`float`, defaults to 0.02) : The final `beta` value.

beta_schedule (`Literal["linear", "scaled_linear", "squaredcos_cap_v2"]`, defaults to `"linear"`) : The beta schedule, a mapping from a beta range to a sequence of betas for stepping the model. Must be one of `"linear"`, `"scaled_linear"`, or `"squaredcos_cap_v2"`.

trained_betas (`np.ndarray`, *optional*) : Pass an array of betas directly to the constructor to bypass `beta_start` and `beta_end`.

clip_sample (`bool`, defaults to `True`) : Clip the predicted sample for numerical stability.

clip_sample_range (`float`, defaults to 1.0) : The maximum magnitude for sample clipping. Valid only when `clip_sample=True`.

set_alpha_to_one (`bool`, defaults to `True`) : Each diffusion step uses the alphas product value at that step and at the previous one. For the final step there is no previous alpha. When this option is `True` the previous alpha product is fixed to `1`, otherwise it uses the alpha value at step 0.

steps_offset (`int`, defaults to 0) : An offset added to the inference steps, as required by some model families.

prediction_type (`Literal["epsilon", "sample", "v_prediction"]`, defaults to `"epsilon"`) : Prediction type of the scheduler function. Must be one of `"epsilon"` (predicts the noise of the diffusion process), `"sample"` (directly predicts the noisy sample), or `"v_prediction"` (see section 2.4 of [Imagen Video](https://huggingface.co/papers/2210.02303) paper).

thresholding (`bool`, defaults to `False`) : Whether to use the "dynamic thresholding" method. This is unsuitable for latent-space diffusion models such as Stable Diffusion.

dynamic_thresholding_ratio (`float`, defaults to 0.995) : The ratio for the dynamic thresholding method. Valid only when `thresholding=True`.

sample_max_value (`float`, defaults to 1.0) : The threshold value for dynamic thresholding. Valid only when `thresholding=True`.

timestep_spacing (`Literal["leading", "trailing", "linspace"]`, defaults to `"leading"`) : The way the timesteps should be scaled. Must be one of `"leading"`, `"trailing"`, or `"linspace"`. Refer to Table 2 of the [Common Diffusion Noise Schedules and Sample Steps are Flawed](https://huggingface.co/papers/2305.08891) for more information.

rescale_betas_zero_snr (`bool`, defaults to `False`) : Whether to rescale the betas to have zero terminal SNR. This enables the model to generate very bright and dark samples instead of limiting it to samples with medium brightness. Loosely related to [`--offset_noise`](https://github.com/huggingface/diffusers/blob/74fd735eb073eb1d774b1ab4154a0876eb82f055/examples/dreambooth/train_dreambooth.py#L506).

**Returns:**

``torch.Tensor``

The noisy samples.
#### get_velocity[[diffusers.DDIMScheduler.get_velocity]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ddim.py#L560)

Compute the velocity prediction from the sample and noise according to the velocity formula.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

noise (`torch.Tensor`) : The noise tensor.

timesteps (`torch.IntTensor`) : The timesteps for velocity computation.

**Returns:**

``torch.Tensor``

The computed velocity.
#### scale_model_input[[diffusers.DDIMScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ddim.py#L245)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`int`, *optional*) : The current timestep in the diffusion chain.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_timesteps[[diffusers.DDIMScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ddim.py#L334)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str | torch.device`, *optional*) : The device to use for the timesteps.
#### step[[diffusers.DDIMScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ddim.py#L384)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

timestep (`int`) : The current discrete timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

eta (`float`, *optional*, defaults to 0.0) : The weight of noise for added noise in diffusion step. A value of 0 corresponds to DDIM (deterministic) and 1 corresponds to DDPM (fully stochastic).

use_clipped_model_output (`bool`, *optional*, defaults to `False`) : If `True`, computes "corrected" `model_output` from the clipped predicted original sample. Necessary because predicted original sample is clipped to [-1, 1] when `self.config.clip_sample` is `True`. If no clipping has happened, "corrected" `model_output` would coincide with the one provided as input and `use_clipped_model_output` has no effect.

generator (`torch.Generator`, *optional*) : A random number generator for reproducible sampling.

variance_noise (`torch.Tensor`, *optional*) : Alternative to generating noise with `generator` by directly providing the noise for the variance itself. Useful for methods such as `CycleDiffusion`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [DDIMSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.schedulers.scheduling_ddim.DDIMSchedulerOutput) or `tuple`.

**Returns:**

`[DDIMSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.schedulers.scheduling_ddim.DDIMSchedulerOutput) or `tuple``

If return_dict is `True`, [DDIMSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.schedulers.scheduling_ddim.DDIMSchedulerOutput) is returned, otherwise a
tuple is returned where the first element is the sample tensor.

## DDIMSchedulerOutput[[diffusers.schedulers.scheduling_ddim.DDIMSchedulerOutput]]
#### diffusers.schedulers.scheduling_ddim.DDIMSchedulerOutput[[diffusers.schedulers.scheduling_ddim.DDIMSchedulerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_ddim.py#L33)

Output class for the scheduler's `step` function output.

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample `(x_{t-1})` of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

pred_original_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : The predicted denoised sample `(x_{0})` based on the model output from the current timestep. `pred_original_sample` can be used to preview progress or for guidance.
