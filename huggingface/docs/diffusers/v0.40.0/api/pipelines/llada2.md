# LLaDA2

[LLaDA2](https://huggingface.co/collections/inclusionAI/llada21) is a family of discrete diffusion language models
that generate text through block-wise iterative refinement. Instead of autoregressive token-by-token generation,
LLaDA2 starts with a fully masked sequence and progressively unmasks tokens by confidence over multiple refinement
steps.

## Usage

```py
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer

from diffusers import BlockRefinementScheduler, LLaDA2Pipeline

model_id = "inclusionAI/LLaDA2.1-mini"
model = AutoModelForCausalLM.from_pretrained(
    model_id, trust_remote_code=True, dtype=torch.bfloat16, device_map="auto"
)
tokenizer = AutoTokenizer.from_pretrained(model_id, trust_remote_code=True)
scheduler = BlockRefinementScheduler()

pipe = LLaDA2Pipeline(model=model, scheduler=scheduler, tokenizer=tokenizer)
output = pipe(
    prompt="Write a short poem about the ocean.",
    gen_length=256,
    block_length=32,
    num_inference_steps=32,
    threshold=0.7,
    editing_threshold=0.5,
    max_post_steps=16,
    temperature=0.0,
)
print(output.texts[0])
```

## Callbacks

Callbacks run after each refinement step. Pass `callback_on_step_end_tensor_inputs` to select which tensors are
included in `callback_kwargs`. In the current implementation, `block_x` (the sequence window being refined) and
`transfer_index` (mask-filling commit mask) are provided; return `{"block_x": ...}` from the callback to replace the
window.

```py
def on_step_end(pipe, step, timestep, callback_kwargs):
    block_x = callback_kwargs["block_x"]
    # Inspect or modify `block_x` here.
    return {"block_x": block_x}

out = pipe(
    prompt="Write a short poem.",
    callback_on_step_end=on_step_end,
    callback_on_step_end_tensor_inputs=["block_x"],
)
```

## Recommended parameters

LLaDA2.1 models support two modes:

| Mode | `threshold` | `editing_threshold` | `max_post_steps` |
|------|-------------|---------------------|------------------|
| Quality | 0.7 | 0.5 | 16 |
| Speed | 0.5 | `None` | 16 |

Pass `editing_threshold=None`, `0.0`, or a negative value to turn off post-mask editing.

For LLaDA2.0 models, disable editing by passing `editing_threshold=None` or `0.0`.

For all models: `block_length=32`, `temperature=0.0`, `num_inference_steps=32`.

## LLaDA2Pipeline[[diffusers.LLaDA2Pipeline]]

#### diffusers.LLaDA2Pipeline[[diffusers.LLaDA2Pipeline]]

```python
diffusers.LLaDA2Pipeline(model: Any, scheduler: BlockRefinementScheduler, tokenizer: Any | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/llada2/pipeline_llada2.py#L59)

Pipeline for LLaDA2-style discrete diffusion text generation via block-wise iterative refinement.

This pipeline maintains a template sequence filled with a `mask_token_id` and refines it in blocks. In each
refinement step, it samples candidate tokens for the active block and commits a subset based on confidence.

The model is expected to accept an attention mask and `position_ids`, and to return logits of shape `[batch, seq,
vocab_size]`.

#### __call__[[diffusers.LLaDA2Pipeline.__call__]]

```python
__call__(prompt: str | list[str] | None = None, messages: list[dict[str, str]] | None = None, input_ids: torch.LongTensor | None = None, attention_mask: torch.LongTensor | None = None, use_chat_template: bool = True, add_generation_prompt: bool = True, gen_length: int = 2048, block_length: int | None = None, num_inference_steps: int = 32, temperature: float = 0.0, top_p: float | None = None, top_k: int | None = None, sampling_method: str = 'multinomial', threshold: float = 0.7, editing_threshold: float | None = 0.5, max_post_steps: int = 16, minimal_topk: int = 1, eos_early_stop: bool = True, eos_token_id: int | None = None, mask_token_id: int | None = None, generator: torch.Generator | None = None, output_type: str = 'text', return_dict: bool = True, callback_on_step_end: Callable[[int, int, dict], None] | PipelineCallback | MultiPipelineCallbacks | None = None, callback_on_step_end_tensor_inputs: list[str] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/llada2/pipeline_llada2.py#L242)

**Parameters:**

prompt (`str` or `List[str]`, *optional*) : Prompt text. When `use_chat_template` is `True` (default) and a tokenizer with a chat template is available, the prompt is wrapped in a chat message before tokenization.

messages (`List[Dict[str, str]]`, *optional*) : Chat messages to encode (e.g. `[{"role": "user", "content": "Hello"}]`). Takes precedence over `prompt` when provided. Requires a tokenizer with `apply_chat_template`.

input_ids (`torch.LongTensor`, *optional*) : Pre-tokenized input IDs. Takes precedence over `prompt` and `messages`.

attention_mask (`torch.LongTensor`, *optional*) : Per-token mask (1 for valid prompt tokens, 0 for padding) matching the shape of `input_ids`. Only used when `input_ids` is provided. When omitted (and `input_ids` is given), all positions are treated as valid. When constructing inputs from `prompt` / `messages`, the tokenizer's mask is carried through automatically.

use_chat_template (`bool`, defaults to `True`) : Whether to wrap the prompt in a chat template.

add_generation_prompt (`bool`, defaults to `True`) : Whether to add the generation prompt when using chat templates.

gen_length (`int`) : Number of tokens to generate.

block_length (`int`, *optional*) : Block size for refinement. If not provided, the scheduler's configured `block_length` is used.

num_inference_steps (`int`) : Number of refinement steps per block.

temperature (`float`) : Sampling temperature.

top_p (`float`, *optional*) : Nucleus sampling cutoff.

top_k (`int`, *optional*) : Top-k sampling cutoff.

sampling_method (`str`) : Sampling method (`auto`, `greedy`, `multinomial`).

threshold (`float`) : Confidence threshold for committing tokens.

editing_threshold (`float`, *optional*) : Confidence threshold for editing already-committed (non-mask) tokens. When positive, after all mask tokens in a block are resolved, the pipeline continues refining: if the model predicts a different token with confidence above this threshold, the existing token is replaced. Set to `None`, `0.0`, or a negative value to disable editing. Defaults to `0.5`.

max_post_steps (`int`) : Maximum number of additional refinement iterations after all mask tokens in a block are resolved. Only used when `editing_threshold` is enabled. Defaults to `16`.

minimal_topk (`int`) : Minimum number of tokens to commit per step.

eos_early_stop (`bool`) : Whether to stop after committing EOS in a block.

eos_token_id (`int`, *optional*) : EOS token ID to use for early stopping.

mask_token_id (`int`, *optional*) : Mask token ID to use for the template.

generator (`torch.Generator`, *optional*) : RNG for sampling.

output_type (`str`, defaults to `"text"`) : Output format. `"text"` decodes sequences into strings (requires a tokenizer). `"seq"` returns raw token ID sequences only.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a [LLaDA2PipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/llada2#diffusers.LLaDA2PipelineOutput) instead of a tuple.

callback_on_step_end (`Callable` or `PipelineCallback`, *optional*) : Callback executed after each refinement step with signature `callback_on_step_end(self, step: int, timestep: int, callback_kwargs: Dict)`.

callback_on_step_end_tensor_inputs (`List[str]`, *optional*) : Tensor keys to pass to the callback. Allowed keys: `block_x`, `transfer_index`, `editing_transfer_index`, `sampled_tokens`, `sampled_probs`, `active_block`.

**Returns:** [LLaDA2PipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/llada2#diffusers.LLaDA2PipelineOutput) or `tuple`

If `return_dict` is `True`, [LLaDA2PipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/llada2#diffusers.LLaDA2PipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is the generated token IDs (`torch.LongTensor`)
and the second element is the decoded texts (`list[str]`), or `None` when `output_type` is `"seq"`.

Generate text with block-wise refinement.

Examples:
```python
>>> import torch
>>> from transformers import AutoModelForCausalLM, AutoTokenizer
>>> from diffusers import BlockRefinementScheduler, LLaDA2Pipeline

>>> model_id = "inclusionAI/LLaDA2.1-mini"
>>> model = AutoModelForCausalLM.from_pretrained(
...     model_id, trust_remote_code=True, dtype=torch.bfloat16, device_map="auto"
... )
>>> tokenizer = AutoTokenizer.from_pretrained(model_id, trust_remote_code=True)
>>> scheduler = BlockRefinementScheduler()

>>> pipe = LLaDA2Pipeline(model=model, scheduler=scheduler, tokenizer=tokenizer)
>>> output = pipe(prompt="What is the meaning of life?", gen_length=256)
>>> print(output.texts[0])
```

## LLaDA2PipelineOutput[[diffusers.LLaDA2PipelineOutput]]

#### diffusers.LLaDA2PipelineOutput[[diffusers.LLaDA2PipelineOutput]]

```python
diffusers.LLaDA2PipelineOutput(sequences: torch.LongTensor, texts: list[str] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/llada2/pipeline_llada2.py#L54)
