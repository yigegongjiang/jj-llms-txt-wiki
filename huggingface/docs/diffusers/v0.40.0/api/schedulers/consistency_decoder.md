# ConsistencyDecoderScheduler

This scheduler is a part of the `ConsistencyDecoderPipeline` and was introduced in [DALL-E 3](https://openai.com/dall-e-3).

The original codebase can be found at [openai/consistency_models](https://github.com/openai/consistency_models).

## ConsistencyDecoderScheduler[[diffusers.schedulers.ConsistencyDecoderScheduler]]

#### diffusers.schedulers.ConsistencyDecoderScheduler[[diffusers.schedulers.ConsistencyDecoderScheduler]]

```python
diffusers.schedulers.ConsistencyDecoderScheduler(num_train_timesteps: int = 1024, sigma_data: float = 0.5)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_decoder.py#L80)

**Parameters:**

num_train_timesteps (`int`, *optional*, defaults to `1024`) : The number of diffusion steps to train the model.

sigma_data (`float`, *optional*, defaults to `0.5`) : The standard deviation of the data distribution. Used for computing the skip and output scaling factors.

A scheduler for the consistency decoder used in Stable Diffusion pipelines.

This scheduler implements a two-step denoising process using consistency models for decoding latent representations
into images.

This model inherits from [SchedulerMixin](/docs/diffusers/v0.40.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and [ConfigMixin](/docs/diffusers/v0.40.0/en/api/configuration#diffusers.ConfigMixin). Check the superclass documentation for the generic
methods the library implements for all schedulers such as loading and saving.

#### scale_model_input[[diffusers.schedulers.ConsistencyDecoderScheduler.scale_model_input]]

```python
scale_model_input(sample: Tensor, timestep: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_decoder.py#L148)

**Parameters:**

sample (`torch.Tensor`) : The input sample.

timestep (`int`, *optional*) : The current timestep in the diffusion chain.

**Returns:** `torch.Tensor`

A scaled input sample.

Ensures interchangeability with schedulers that need to scale the denoising model input depending on the
current timestep.

#### step[[diffusers.schedulers.ConsistencyDecoderScheduler.step]]

```python
step(model_output: Tensor, timestep: typing.Union[float, torch.Tensor], sample: Tensor, generator: typing.Optional[torch.Generator] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_consistency_decoder.py#L165)

**Parameters:**

model_output (`torch.Tensor`) : The direct output from the learned diffusion model.

timestep (`float` or `torch.Tensor`) : The current timestep in the diffusion chain.

sample (`torch.Tensor`) : A current instance of a sample created by the diffusion process.

generator (`torch.Generator`, *optional*) : A random number generator for reproducibility.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `ConsistencyDecoderSchedulerOutput` or `tuple`.

**Returns:** `ConsistencyDecoderSchedulerOutput` or `tuple`

If `return_dict` is `True`,
`ConsistencyDecoderSchedulerOutput` is returned, otherwise
a tuple is returned where the first element is the sample tensor.

Predict the sample from the previous timestep by reversing the SDE. This function propagates the diffusion
process from the learned model outputs (most often the predicted noise).
