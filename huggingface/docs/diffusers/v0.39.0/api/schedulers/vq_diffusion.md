# VQDiffusionScheduler

`VQDiffusionScheduler` converts the transformer model's output into a sample for the unnoised image at the previous diffusion timestep. It was introduced in [Vector Quantized Diffusion Model for Text-to-Image Synthesis](https://huggingface.co/papers/2111.14822) by Shuyang Gu, Dong Chen, Jianmin Bao, Fang Wen, Bo Zhang, Dongdong Chen, Lu Yuan, Baining Guo.

The abstract from the paper is:

*We present the vector quantized diffusion (VQ-Diffusion) model for text-to-image generation. This method is based on a vector quantized variational autoencoder (VQ-VAE) whose latent space is modeled by a conditional variant of the recently developed Denoising Diffusion Probabilistic Model (DDPM). We find that this latent-space method is well-suited for text-to-image generation tasks because it not only eliminates the unidirectional bias with existing methods but also allows us to incorporate a mask-and-replace diffusion strategy to avoid the accumulation of errors, which is a serious problem with existing methods. Our experiments show that the VQ-Diffusion produces significantly better text-to-image generation results when compared with conventional autoregressive (AR) models with similar numbers of parameters. Compared with previous GAN-based text-to-image methods, our VQ-Diffusion can handle more complex scenes and improve the synthesized image quality by a large margin. Finally, we show that the image generation computation in our method can be made highly efficient by reparameterization. With traditional AR methods, the text-to-image generation time increases linearly with the output image resolution and hence is quite time consuming even for normal size images. The VQ-Diffusion allows us to achieve a better trade-off between quality and speed. Our experiments indicate that the VQ-Diffusion model with the reparameterization is fifteen times faster than traditional AR methods while achieving a better image quality.*

## VQDiffusionScheduler[[diffusers.VQDiffusionScheduler]]
#### diffusers.VQDiffusionScheduler[[diffusers.VQDiffusionScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_vq_diffusion.py#L105)

A scheduler for vector quantized diffusion.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

log_Q_t_transitioning_to_known_classdiffusers.VQDiffusionScheduler.log_Q_t_transitioning_to_known_classhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_vq_diffusion.py#L355[{"name": "t", "val": ": torch.int32"}, {"name": "x_t", "val": ": LongTensor"}, {"name": "log_onehot_x_t", "val": ": Tensor"}, {"name": "cumulative", "val": ": bool"}]- **t** (`torch.Long`) --
  The timestep that determines which transition matrix is used.
- **x_t** (`torch.LongTensor` of shape `(batch size, num latent pixels)`) --
  The classes of each latent pixel at time `t`.
- **log_onehot_x_t** (`torch.Tensor` of shape `(batch size, num classes, num latent pixels)`) --
  The log one-hot vectors of `x_t`.
- **cumulative** (`bool`) --
  If cumulative is `False`, the single step transition matrix `t-1`->`t` is used. If cumulative is
  `True`, the cumulative transition matrix `0`->`t` is used.0`torch.Tensor` of shape `(batch size, num classes - 1, num latent pixels)`Each _column_ of the returned matrix is a _row_ of log probabilities of the complete probability
transition matrix.

When non cumulative, returns `self.num_classes - 1` rows because the initial latent pixel cannot be
masked.

Where:
- `q_n` is the probability distribution for the forward process of the `n`th latent pixel.
- C_0 is a class of a latent pixel embedding
- C_k is the class of the masked latent pixel

non-cumulative result (omitting logarithms):
```
q_0(x_t | x_{t-1} = C_0) ... q_n(x_t | x_{t-1} = C_0)
          .      .                     .
          .               .            .
          .                      .     .
q_0(x_t | x_{t-1} = C_k) ... q_n(x_t | x_{t-1} = C_k)
```

cumulative result (omitting logarithms):
```
q_0_cumulative(x_t | x_0 = C_0)    ...  q_n_cumulative(x_t | x_0 = C_0)
          .               .                          .
          .                        .                 .
          .                               .          .
q_0_cumulative(x_t | x_0 = C_{k-1}) ... q_n_cumulative(x_t | x_0 = C_{k-1})
```

Calculates the log probabilities of the rows from the (cumulative or non-cumulative) transition matrix for each
latent pixel in `x_t`.

**Parameters:**

num_vec_classes (`int`) : The number of classes of the vector embeddings of the latent pixels. Includes the class for the masked latent pixel.

num_train_timesteps (`int`, defaults to 100) : The number of diffusion steps to train the model.

alpha_cum_start (`float`, defaults to 0.99999) : The starting cumulative alpha value.

alpha_cum_end (`float`, defaults to 0.00009) : The ending cumulative alpha value.

gamma_cum_start (`float`, defaults to 0.00009) : The starting cumulative gamma value.

gamma_cum_end (`float`, defaults to 0.99999) : The ending cumulative gamma value.

**Returns:**

``torch.Tensor` of shape `(batch size, num classes - 1, num latent pixels)``

Each _column_ of the returned matrix is a _row_ of log probabilities of the complete probability
transition matrix.

When non cumulative, returns `self.num_classes - 1` rows because the initial latent pixel cannot be
masked.

Where:
- `q_n` is the probability distribution for the forward process of the `n`th latent pixel.
- C_0 is a class of a latent pixel embedding
- C_k is the class of the masked latent pixel

non-cumulative result (omitting logarithms):
```
q_0(x_t | x_{t-1} = C_0) ... q_n(x_t | x_{t-1} = C_0)
          .      .                     .
          .               .            .
          .                      .     .
q_0(x_t | x_{t-1} = C_k) ... q_n(x_t | x_{t-1} = C_k)
```

cumulative result (omitting logarithms):
```
q_0_cumulative(x_t | x_0 = C_0)    ...  q_n_cumulative(x_t | x_0 = C_0)
          .               .                          .
          .                        .                 .
          .                               .          .
q_0_cumulative(x_t | x_0 = C_{k-1}) ... q_n_cumulative(x_t | x_0 = C_{k-1})
```
#### q_posterior[[diffusers.VQDiffusionScheduler.q_posterior]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_vq_diffusion.py#L244)

Calculates the log probabilities for the predicted classes of the image at timestep `t-1`:

```
p(x_{t-1} | x_t) = sum( q(x_t | x_{t-1}) * q(x_{t-1} | x_0) * p(x_0) / q(x_t | x_0) )
```

**Parameters:**

log_p_x_0 (`torch.Tensor` of shape `(batch size, num classes - 1, num latent pixels)`) : The log probabilities for the predicted classes of the initial latent pixels. Does not include a prediction for the masked class as the initial unnoised image cannot be masked.

x_t (`torch.LongTensor` of shape `(batch size, num latent pixels)`) : The classes of each latent pixel at time `t`.

t (`torch.Long`) : The timestep that determines which transition matrix is used.

**Returns:**

``torch.Tensor` of shape `(batch size, num classes, num latent pixels)``

The log probabilities for the predicted classes of the image at timestep `t-1`.
#### set_timesteps[[diffusers.VQDiffusionScheduler.set_timesteps]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_vq_diffusion.py#L177)

Sets the discrete timesteps used for the diffusion chain (to be run before inference).

**Parameters:**

num_inference_steps (`int`) : The number of diffusion steps used when generating samples with a pre-trained model.

device (`str` or `torch.device`, *optional*) : The device to which the timesteps and diffusion process parameters (alpha, beta, gamma) should be moved to.
#### step[[diffusers.VQDiffusionScheduler.step]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_vq_diffusion.py#L199)

Predict the sample from the previous timestep by the reverse transition distribution. See
[q_posterior()](/docs/diffusers/v0.39.0/en/api/schedulers/vq_diffusion#diffusers.VQDiffusionScheduler.q_posterior) for more details about how the distribution is computer.

**Parameters:**

log_p_x_0 : (`torch.Tensor` of shape `(batch size, num classes - 1, num latent pixels)`): The log probabilities for the predicted classes of the initial latent pixels. Does not include a prediction for the masked class as the initial unnoised image cannot be masked.

t (`torch.long`) : The timestep that determines which transition matrices are used.

x_t (`torch.LongTensor` of shape `(batch size, num latent pixels)`) : The classes of each latent pixel at time `t`.

generator (`torch.Generator`, or `None`) : A random number generator for the noise applied to `p(x_{t-1} | x_t)` before it is sampled from.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [VQDiffusionSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/vq_diffusion#diffusers.schedulers.scheduling_vq_diffusion.VQDiffusionSchedulerOutput) or `tuple`.

**Returns:**

`[VQDiffusionSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/vq_diffusion#diffusers.schedulers.scheduling_vq_diffusion.VQDiffusionSchedulerOutput) or `tuple``

If return_dict is `True`, [VQDiffusionSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/vq_diffusion#diffusers.schedulers.scheduling_vq_diffusion.VQDiffusionSchedulerOutput) is
returned, otherwise a tuple is returned where the first element is the sample tensor.

## VQDiffusionSchedulerOutput[[diffusers.schedulers.scheduling_vq_diffusion.VQDiffusionSchedulerOutput]]
#### diffusers.schedulers.scheduling_vq_diffusion.VQDiffusionSchedulerOutput[[diffusers.schedulers.scheduling_vq_diffusion.VQDiffusionSchedulerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_vq_diffusion.py#L27)

Output class for the scheduler's step function output.

**Parameters:**

prev_sample (`torch.LongTensor` of shape `(batch size, num latent pixels)`) : Computed sample x_{t-1} of previous timestep. `prev_sample` should be used as next model input in the denoising loop.
