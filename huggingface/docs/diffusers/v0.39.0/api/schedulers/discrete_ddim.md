# DiscreteDDIMScheduler

The `DiscreteDDIMScheduler` samples each canvas position from the exact discrete posterior of the uniform corruption
process (D3PM), following [Structured Denoising Diffusion Models in Discrete State-Spaces](https://huggingface.co/papers/2107.03006).
It is parameter free, and the final step deterministically commits the predicted tokens. An optional predictor-corrector
mode adds the leave-one-out Gibbs sweeps of [Uniform Diffusion Models Revisited: Leave-One-Out Denoiser and Absorbing State Reformulation](https://huggingface.co/papers/2605.22765)
through `corrector_steps`.

This scheduler is used by [DiffusionGemmaPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/diffusion_gemma#diffusers.DiffusionGemmaPipeline).

## DiscreteDDIMScheduler[[diffusers.DiscreteDDIMScheduler]]
#### diffusers.DiscreteDDIMScheduler[[diffusers.DiscreteDDIMScheduler]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_discrete_ddim.py#L49)

Discrete DDIM scheduler for the uniform corruption process, following "Structured Denoising Diffusion Models in
Discrete State-Spaces" (D3PM, https://huggingface.co/papers/2107.03006).

On the linear schedule the survival probability of a clean token at time `t` is `alpha(t) = 1 - t`. One denoising
step from time `t` to `s  0`, the pipeline runs that many Gibbs corrector sweeps after each predictor step (see
[step_correct()](/docs/diffusers/v0.39.0/en/api/schedulers/discrete_ddim#diffusers.DiscreteDDIMScheduler.step_correct)), resampling the least-confident positions from the one-coordinate
conditional `Cat(alpha_s * x0_loo + (1 - alpha_s) / K)` while holding the rest fixed, which leaves the marginal
`p_s` invariant and improves generation at no training cost.

stepdiffusers.DiscreteDDIMScheduler.stephttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_discrete_ddim.py#L145[{"name": "model_output", "val": ": torch.Tensor"}, {"name": "timestep", "val": ": int | torch.Tensor"}, {"name": "sample", "val": ": torch.LongTensor"}, {"name": "temperature", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch.Generator | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **model_output** (`torch.Tensor` of shape `(batch_size, block_length, vocab_size)`) --
  Raw logits from the model for the current block.
- **timestep** (`int` or `torch.Tensor`) --
  Current step index within the denoising schedule, in `[0, num_inference_steps - 1]`.
- **sample** (`torch.LongTensor` of shape `(batch_size, block_length)`) --
  Current block token IDs `x_t`.
- **temperature** (`float`) --
  Sampling temperature applied to the logits when drawing `x0`.
- **generator** (`torch.Generator`, *optional*) --
  RNG for sampling.
- **return_dict** (`bool`) --
  Whether to return a [DiscreteDDIMSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/discrete_ddim#diffusers.DiscreteDDIMSchedulerOutput) or a plain tuple.0

Sample the next block from the posterior `q(x_s | x_t, x0)` of the uniform corruption process.

With `a = alpha_t / alpha_s` (survival probability from `s` to `t`) and `b = alpha_s`, the posterior mass of
each route is

clean: `b * (1 - a) / K + a * b * 1[x_t = x0]`, stay: `a * (1 - b) / K`, noise: `(1 - a) * (1 - b) / K`,

so the last step (`b = 1`) deterministically commits the predicted clean tokens.

**Parameters:**

num_inference_steps (`int`, defaults to 32) : The number of denoising steps, defining the linear time grid the posterior is evaluated on.

corrector_steps (`int`, defaults to 0) : Number of Gibbs corrector sweeps run after each predictor step. `0` recovers plain ancestral DDIM sampling.

corrector_k (`int`, defaults to 1) : Number of positions resampled per corrector sweep.

corrector_selection (`str`, defaults to `"lowest_log_margin"`) : How the resampled positions are chosen: `"lowest_log_margin"`, `"lowest_maxprob"`, `"lowest_current_prob"`, or `"random"`.

corrector_selection_tau (`float`, defaults to 1.0) : Temperature of the Gumbel-top-k position selection (lower is greedier).
#### step_correct[[diffusers.DiscreteDDIMScheduler.step_correct]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_discrete_ddim.py#L248)

Run one Gibbs corrector sweep at the post-predictor time `s`, following the leave-one-out predictor-corrector
of https://huggingface.co/papers/2605.22765.

The model logits (recomputed on the current `sample`) are converted to the LOO denoiser, the one-coordinate
conditional `p_s(x^l | x^{-l}) = Cat(alpha_s * x0_loo + (1 - alpha_s) / K)` is formed, the least-confident
`corrector_k` positions are selected, and those positions are resampled while the rest are held fixed. The
sweep preserves `p_s`, so it refines the sample without changing its marginal and needs no extra training.

**Parameters:**

model_output (`torch.Tensor` of shape `(batch_size, block_length, vocab_size)`) : Raw logits from the model recomputed on the current (post-predictor) `sample`.

timestep (`int` or `torch.Tensor`) : The predictor step index just completed; the corrector runs at the following grid point `s`.

sample (`torch.LongTensor` of shape `(batch_size, block_length)`) : Current block token IDs to refine.

generator (`torch.Generator`, *optional*) : RNG for sampling.

return_dict (`bool`) : Whether to return a [DiscreteDDIMSchedulerOutput](/docs/diffusers/v0.39.0/en/api/schedulers/discrete_ddim#diffusers.DiscreteDDIMSchedulerOutput) or a plain tuple.

## DiscreteDDIMSchedulerOutput[[diffusers.DiscreteDDIMSchedulerOutput]]
#### diffusers.DiscreteDDIMSchedulerOutput[[diffusers.DiscreteDDIMSchedulerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/schedulers/scheduling_discrete_ddim.py#L28)

Output class for the discrete DDIM scheduler.

**Parameters:**

prev_sample (`torch.LongTensor` of shape `(batch_size, block_length)`) : Updated block tokens after the current denoising step.

sampled_tokens (`torch.LongTensor` of shape `(batch_size, block_length)`) : Token IDs sampled from the model logits, i.e. the predicted clean tokens `x0`.

sampled_probs (`torch.Tensor` of shape `(batch_size, block_length)`) : Probabilities of the sampled tokens.

pred_logits (`torch.Tensor` of shape `(batch_size, block_length, vocab_size)`) : The denoiser logits, passed through for self-conditioning the next step.
