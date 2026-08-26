# EntropyBoundScheduler

The `EntropyBoundScheduler` commits the lowest-entropy positions whose joint entropy stays under `entropy_bound`, so
roughly independent tokens are accepted together and the rest are renoised. It anneals its sampling temperature from
`t_max` on the first step down to `t_min` on the last, matching the released checkpoint's sampler. Proposed in
[Accelerated Sampling from Masked Diffusion Models via Entropy Bounded Unmasking](https://huggingface.co/papers/2505.24857).

This scheduler is used by [DiffusionGemmaPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/diffusion_gemma#diffusers.DiffusionGemmaPipeline).

## EntropyBoundScheduler[[diffusers.EntropyBoundScheduler]]

#### diffusers.EntropyBoundScheduler[[diffusers.EntropyBoundScheduler]]

```python
diffusers.EntropyBoundScheduler(entropy_bound: float = 0.1, t_max: float = 0.8, t_min: float = 0.4, num_inference_steps: int = 32)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_entropy_bound.py#L51)

**Parameters:**

entropy_bound (`float`, defaults to 0.1) : The maximum tolerated joint entropy of the accepted tokens. Larger values accept more tokens per step.

t_max (`float`, defaults to 0.8) : Sampling temperature on the first denoising step.

t_min (`float`, defaults to 0.4) : Sampling temperature on the last denoising step.

num_inference_steps (`int`, defaults to 32) : The maximum number of denoising steps.

Entropy bound scheduler for the uniform corruption process.

At each step the scheduler samples a candidate token per position and accepts the `k` lowest-entropy positions such
that `sum_i^k entropy_i - max(entropy_1, ..., entropy_k) <= entropy_bound`. The left-hand side upper-bounds the
joint mutual information between the accepted tokens, so they are approximately independent. Accepted positions
keep their sampled token; the rest are renoised with uniformly random tokens (there is no mask token).

Proposed in "Accelerated Sampling from Masked Diffusion Models via Entropy Bounded Unmasking"
(https://huggingface.co/papers/2505.24857).

The sampling temperature is annealed from `t_max` on the first step down to `t_min` on the last, matching the
released checkpoint's sampler (sharper sampling as denoising advances). It is applied to the logits before both the
candidate sampling and the entropy that drives acceptance.

#### step[[diffusers.EntropyBoundScheduler.step]]

```python
step(model_output: torch.Tensor, timestep: int | torch.Tensor, sample: torch.LongTensor, entropy_bound: float | None = None, generator: torch.Generator | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_entropy_bound.py#L118)

**Parameters:**

model_output (`torch.Tensor` of shape `(batch_size, block_length, vocab_size)`) : Raw logits from the model for the current block.

timestep (`int` or `torch.Tensor`) : Current step index within the denoising schedule; sets the annealed sampling temperature.

sample (`torch.LongTensor` of shape `(batch_size, block_length)`) : Current block token IDs.

entropy_bound (`float`, *optional*) : Overrides the configured entropy bound for this step.

generator (`torch.Generator`, *optional*) : RNG for sampling.

return_dict (`bool`) : Whether to return an [EntropyBoundSchedulerOutput](/docs/diffusers/v0.40.0/en/api/schedulers/entropy_bound#diffusers.EntropyBoundSchedulerOutput) or a plain tuple.

Accept the lowest-entropy positions under the entropy bound and renoise the rest.

## EntropyBoundSchedulerOutput[[diffusers.EntropyBoundSchedulerOutput]]

#### diffusers.EntropyBoundSchedulerOutput[[diffusers.EntropyBoundSchedulerOutput]]

```python
diffusers.EntropyBoundSchedulerOutput(prev_sample: torch.LongTensor, accepted_index: torch.BoolTensor, sampled_tokens: torch.LongTensor, sampled_probs: torch.Tensor, pred_logits: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_entropy_bound.py#L27)

**Parameters:**

prev_sample (`torch.LongTensor` of shape `(batch_size, block_length)`) : Updated block tokens after the current denoising step.

accepted_index (`torch.BoolTensor` of shape `(batch_size, block_length)`) : Boolean mask of the positions accepted (committed) in this step.

sampled_tokens (`torch.LongTensor` of shape `(batch_size, block_length)`) : Token IDs sampled from the model logits.

sampled_probs (`torch.Tensor` of shape `(batch_size, block_length)`) : Probabilities of the sampled tokens.

pred_logits (`torch.Tensor` of shape `(batch_size, block_length, vocab_size)`) : The temperature-scaled logits the candidates were drawn from, for self-conditioning the next step.

Output class for the entropy bound scheduler.
