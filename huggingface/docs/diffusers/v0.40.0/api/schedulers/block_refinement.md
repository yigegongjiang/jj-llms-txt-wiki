# BlockRefinementScheduler

The `BlockRefinementScheduler` manages block-wise iterative refinement for discrete token diffusion. At each step it
commits the most confident tokens and optionally edits already-committed tokens when the model predicts a different
token with high confidence.

This scheduler is used by [LLaDA2Pipeline](/docs/diffusers/v0.40.0/en/api/pipelines/llada2#diffusers.LLaDA2Pipeline).

## BlockRefinementScheduler[[diffusers.BlockRefinementScheduler]]

#### diffusers.BlockRefinementScheduler[[diffusers.BlockRefinementScheduler]]

```python
diffusers.BlockRefinementScheduler(block_length: int = 32, num_inference_steps: int = 32, threshold: float = 0.95, editing_threshold: float | None = None, minimal_topk: int = 1)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_block_refinement.py#L54)

Scheduler for block-wise iterative refinement (commit-by-confidence).

At each step, the scheduler samples candidate tokens from model logits and commits those with the highest
confidence. The number of tokens to commit per step is determined by evenly distributing the block length across
the number of refinement steps.

Optionally supports editing: after all mask tokens are resolved, tokens can be replaced if the model predicts a
different token with confidence above a positive `editing_threshold` (`None`, `0.0`, or negative disables editing).

#### add_noise[[diffusers.BlockRefinementScheduler.add_noise]]

```python
add_noise(original_samples: torch.LongTensor, attention_mask: torch.LongTensor, prompt_length: int, block_length: int, mask_token_id: int, generator: torch.Generator | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_block_refinement.py#L456)

**Parameters:**

original_samples (`torch.LongTensor` of shape `(batch_size, seq_len)`) : Clean token IDs.

attention_mask (`torch.LongTensor` of shape `(batch_size, seq_len)`) : Padding mask (1 for valid, 0 for padding).

prompt_length (`int`) : Number of leading prompt tokens to keep unmasked.

block_length (`int`) : Block size for masking.

mask_token_id (`int`) : Token ID to use for masked positions.

generator (`torch.Generator`, *optional*) : RNG for reproducibility.

**Returns:** `tuple[torch.LongTensor, torch.LongTensor, torch.BoolTensor, torch.BoolTensor]`

`(noisy, noisy_rev, masked, masked_rev)` — the two complementary noisy sequences and their
corresponding boolean masks.

Apply the forward (noising) process for semi-autoregressive block masking.

For each block after the prompt, a random fraction of valid (non-padding) tokens are replaced with
`mask_token_id`. Two complementary views are returned: `noisy` and `noisy_rev`, where the masked positions in
one are the unmasked positions in the other.

#### check_block_should_continue[[diffusers.BlockRefinementScheduler.check_block_should_continue]]

```python
check_block_should_continue(step_idx: int, masks_remaining: bool, editing_enabled: bool, editing_transfer_index: torch.BoolTensor, post_steps: int, max_post_steps: int, finished: torch.BoolTensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_block_refinement.py#L412)

**Parameters:**

step_idx (`int`) : Current refinement step index within this block.

masks_remaining (`bool`) : Whether any mask tokens remain in the block.

editing_enabled (`bool`) : Whether editing mode is active.

editing_transfer_index (`torch.BoolTensor`) : Which tokens were edited in this step.

post_steps (`int`) : Number of post-mask editing steps taken so far.

max_post_steps (`int`) : Maximum allowed post-mask editing steps.

finished (`torch.BoolTensor`) : Per-batch finished flags (from EOS detection).

**Returns:** `bool`

`True` if refinement should continue, `False` to break.

Determine whether the inner refinement loop should continue for the current block.

#### check_eos_finished[[diffusers.BlockRefinementScheduler.check_eos_finished]]

```python
check_eos_finished(cur_x: torch.LongTensor, sampled_tokens: torch.LongTensor, final_transfer: torch.BoolTensor, finished: torch.BoolTensor, eos_token_id: int, mask_token_id: int, prompt_length: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_block_refinement.py#L362)

**Parameters:**

cur_x (`torch.LongTensor` of shape `(batch_size, seq_len)`) : Current full sequence including all blocks up to the current window.

sampled_tokens (`torch.LongTensor` of shape `(batch_size, block_length)`) : Tokens sampled by the scheduler in this step.

final_transfer (`torch.BoolTensor` of shape `(batch_size, block_length)`) : Combined mask of committed and edited positions.

finished (`torch.BoolTensor` of shape `(batch_size,)`) : Current per-batch finished flags.

eos_token_id (`int`) : EOS token ID.

mask_token_id (`int`) : Mask token ID.

prompt_length (`int`) : Number of prompt tokens at the start of the sequence.

**Returns:** `torch.BoolTensor`

Updated finished flags.

Update per-batch finished flags when EOS tokens are committed.

#### get_num_transfer_tokens[[diffusers.BlockRefinementScheduler.get_num_transfer_tokens]]

```python
get_num_transfer_tokens(block_length: int, num_inference_steps: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_block_refinement.py#L102)

Evenly distribute `block_length` token commits across `num_inference_steps` steps.

#### step[[diffusers.BlockRefinementScheduler.step]]

```python
step(model_output: torch.Tensor, timestep: int | torch.Tensor, sample: torch.LongTensor, mask_token_id: int | None = None, temperature: float = 0.0, top_p: float | None = None, top_k: int | None = None, sampling_method: str = 'auto', threshold: float | None = None, editing_threshold: float | None = None, minimal_topk: int | None = None, prompt_mask: torch.BoolTensor | None = None, generator: torch.Generator | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_block_refinement.py#L181)

**Parameters:**

model_output (`torch.Tensor` of shape `(batch_size, block_length, vocab_size)`) : Raw logits from the model for the current block.

timestep (`int` or `torch.Tensor`) : Current step index within the block's refinement schedule.

sample (`torch.LongTensor` of shape `(batch_size, block_length)`) : Current block token IDs (contains mask tokens for uncommitted positions in the mask-based mode).

mask_token_id (`int`, *optional*) : Token ID used for masked positions. When `None`, the scheduler runs in uniform corruption mode: it tracks committed positions internally (resetting at `timestep == 0`) and renoises the uncommitted ones with uniformly random tokens, matching DiffusionGemma's block refinement sampler.

temperature (`float`) : Sampling temperature.

top_p (`float`, *optional*) : Nucleus sampling cutoff.

top_k (`int`, *optional*) : Top-k sampling cutoff.

sampling_method (`str`) : Sampling method (`auto`, `greedy`, `multinomial`).

threshold (`float`, *optional*) : Confidence threshold for committing tokens. Defaults to config value.

editing_threshold (`float`, *optional*) : Confidence threshold for editing non-mask tokens; must be positive to enable editing. Defaults to config value.

minimal_topk (`int`, *optional*) : Minimum tokens to commit per step. Defaults to config value.

prompt_mask (`torch.BoolTensor`, *optional*) : Boolean mask of shape `(block_length,)` where `True` marks prompt (non-editable) positions.

generator (`torch.Generator`, *optional*) : RNG for sampling.

return_dict (`bool`) : Whether to return a `BlockRefinementSchedulerOutput` or a tuple.

Perform a single refinement step: sample from logits, commit confident tokens, and optionally edit existing
ones.

## BlockRefinementSchedulerOutput[[diffusers.BlockRefinementSchedulerOutput]]

#### diffusers.BlockRefinementSchedulerOutput[[diffusers.BlockRefinementSchedulerOutput]]

```python
diffusers.BlockRefinementSchedulerOutput(prev_sample: torch.LongTensor, transfer_index: torch.BoolTensor, editing_transfer_index: torch.BoolTensor, sampled_tokens: torch.LongTensor, sampled_probs: torch.Tensor, pred_logits: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/schedulers/scheduling_block_refinement.py#L27)

**Parameters:**

prev_sample (`torch.LongTensor` of shape `(batch_size, block_length)`) : Updated block tokens after the current refinement step.

transfer_index (`torch.BoolTensor` of shape `(batch_size, block_length)`) : Boolean mask indicating which tokens were committed (mask-filling).

editing_transfer_index (`torch.BoolTensor` of shape `(batch_size, block_length)`) : Boolean mask indicating which tokens were edited (non-mask replacement).

sampled_tokens (`torch.LongTensor` of shape `(batch_size, block_length)`) : Sampled token IDs from the model logits.

sampled_probs (`torch.Tensor` of shape `(batch_size, block_length)`) : Probabilities of the sampled tokens.

pred_logits (`torch.Tensor` of shape `(batch_size, block_length, vocab_size)`) : The denoiser logits, passed through for self-conditioning the next step.

Output class for block refinement scheduling.
