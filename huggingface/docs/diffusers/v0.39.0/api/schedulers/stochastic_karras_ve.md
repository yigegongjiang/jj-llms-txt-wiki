# KarrasVeScheduler

`KarrasVeScheduler` is a stochastic sampler tailored to variance-expanding (VE) models. It is based on the [Elucidating the Design Space of Diffusion-Based Generative Models](https://huggingface.co/papers/2206.00364) and [Score-based generative modeling through stochastic differential equations](https://huggingface.co/papers/2011.13456) papers.

## KarrasVeScheduler[[diffusers.KarrasVeScheduler]]
#### diffusers.KarrasVeScheduler[[diffusers.KarrasVeScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/deprecated/scheduling_karras_ve.py#L48)

A stochastic scheduler tailored to variance-expanding models.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

> [!TIP] > For more details on the parameters, see [Appendix E](https://huggingface.co/papers/2206.00364). The grid
search > values used to find the optimal `{s_noise, s_churn, s_min, s_max}` for a specific model are described in
Table 5 of > the paper.

add_noise_to_inputdiffusers.KarrasVeScheduler.add_noise_to_inputhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/deprecated/scheduling_karras_ve.py#L134[{"name": "sample", "val": ": Tensor"}, {"name": "sigma", "val": ": float"}, {"name": "generator", "val": ": torch._C.Generator | None = None"}]- **sample** (`torch.Tensor`) --
  The input sample.
- **sigma** (`float`) --
- **generator** (`torch.Generator`, *optional*) --
  A random number generator.0

Explicit Langevin-like "churn" step of adding noise to the sample according to a `gamma_i ≥ 0` to reach a
higher noise level `sigma_hat = sigma_i + gamma_i*sigma_i`.

**Parameters:**

sigma_min (`float`, defaults to 0.02) : The minimum noise magnitude.

sigma_max (`float`, defaults to 100) : The maximum noise magnitude.

s_noise (`float`, defaults to 1.007) : The amount of additional noise to counteract loss of detail during sampling. A reasonable range is [1.000, 1.011].

s_churn (`float`, defaults to 80) : The parameter controlling the overall amount of stochasticity. A reasonable range is [0, 100].

s_min (`float`, defaults to 0.05) : The start value of the sigma range to add noise (enable stochasticity). A reasonable range is [0, 10].

s_max (`float`, defaults to 50) : The end value of the sigma range to add noise. A reasonable range is [0.2, 80].
#### scale_model_input[[diffusers.KarrasVeScheduler.scale_model_input]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/deprecated/scheduling_karras_ve.py#L95)

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`int`, *optional*) : The current timestep in the diffusion chain.

**Returns:**

``torch.Tensor``

A scaled input sample.
#### set_timesteps[[diffusers.KarrasVeScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/deprecated/scheduling_karras_ve.py#L112)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps should be moved to. If `None`, the timesteps are not moved.
#### step[[diffusers.KarrasVeScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/deprecated/scheduling_karras_ve.py#L160)

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

sigma_hat (`float`) --

sigma_prev (`float`) --

sample_hat (`torch.Tensor`) --

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~schedulers.scheduling_karras_ve.KarrasVESchedulerOutput` or `tuple`.

**Returns:**

``~schedulers.scheduling_karras_ve.KarrasVESchedulerOutput` or `tuple``

If return_dict is `True`, `~schedulers.scheduling_karras_ve.KarrasVESchedulerOutput` is returned,
otherwise a tuple is returned where the first element is the sample tensor.
#### step_correct[[diffusers.KarrasVeScheduler.step_correct]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/deprecated/scheduling_karras_ve.py#L199)

Corrects the predicted sample based on the `model_output` of the network.

**Parameters:**

model_output (`torch.Tensor`) : The direct output from learned diffusion model.

sigma_hat (`float`) : TODO

sigma_prev (`float`) : TODO

sample_hat (`torch.Tensor`) : TODO

sample_prev (`torch.Tensor`) : TODO

derivative (`torch.Tensor`) : TODO

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [DDPMSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/ddpm#diffusers.schedulers.scheduling_ddpm.DDPMSchedulerOutput) or `tuple`.

**Returns:**

`prev_sample (TODO)`

updated sample in the diffusion chain. derivative (TODO): TODO

## KarrasVeOutput[[diffusers.schedulers.deprecated.scheduling_karras_ve.KarrasVeOutput]]
#### diffusers.schedulers.deprecated.scheduling_karras_ve.KarrasVeOutput[[diffusers.schedulers.deprecated.scheduling_karras_ve.KarrasVeOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/deprecated/scheduling_karras_ve.py#L28)

Output class for the scheduler's step function output.

**Parameters:**

prev_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Computed sample (x_{t-1}) of previous timestep. `prev_sample` should be used as next model input in the denoising loop.

derivative (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : Derivative of predicted original image sample (x_0).

pred_original_sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` for images) : The predicted denoised sample (x_{0}) based on the model output from the current timestep. `pred_original_sample` can be used to preview progress or for guidance.
