# DiffusionGemma

DiffusionGemma is a block-diffusion encoder-decoder language model. A causal encoder reads the clean prompt (and any
previously generated blocks) into a KV cache, and a bidirectional decoder denoises a fixed-size "canvas" of
`canvas_length` tokens by cross-attending to that cache. Generation alternates an outer autoregressive loop over
canvases with an inner denoising loop, where each step samples candidate tokens, commits the most confident ones via
[BlockRefinementScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/block_refinement#diffusers.BlockRefinementScheduler) in uniform corruption mode, and renoises the rest. The model itself lives in
`transformers` as `DiffusionGemmaForBlockDiffusion`; the released checkpoint is
[`google/diffusiongemma-26B-A4B-it`](https://huggingface.co/google/diffusiongemma-26B-A4B-it).

## Usage

```py
import torch
from transformers import AutoProcessor, DiffusionGemmaForBlockDiffusion

from diffusers import BlockRefinementScheduler, DiffusionGemmaPipeline

model_id = "google/diffusiongemma-26B-A4B-it"
model = DiffusionGemmaForBlockDiffusion.from_pretrained(model_id, dtype=torch.bfloat16, device_map="auto")
processor = AutoProcessor.from_pretrained(model_id)
scheduler = BlockRefinementScheduler()

pipe = DiffusionGemmaPipeline(model=model, scheduler=scheduler, processor=processor)
pipe.model.model.decoder = torch.compile(pipe.model.model.decoder, mode="reduce-overhead", fullgraph=True)
output = pipe(
    prompt="Why is the sky blue?",
    gen_length=256,
    num_inference_steps=48,
    cache_implementation="static",
)
print(output.texts[0])
```

`num_inference_steps` is the number of denoising steps per canvas (48 matches the released checkpoint); fewer steps are
faster but lower quality. `cache_implementation="static"` lets the decoder be `torch.compile`-d with cudagraphs (see
[Static cache and compilation](#static-cache-and-compilation)); drop both for a simpler dynamic-cache run.

For multi-turn or multimodal inputs, pass a raw `messages` conversation instead of `prompt`. It is a list of
`{"role", "content"}` dicts in the usual chat format, which the processor runs through its chat template:

```py
messages = [
    {"role": "user", "content": "Why is the sky blue?"},
]
# or with an image:
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "image": image},
            {"type": "text", "text": "Describe this image."},
        ],
    },
]
output = pipe(messages=messages, gen_length=256)
```

For a single user turn you can skip `messages` and pass an `image` alongside the `prompt`; the processor turns it into
the model's image inputs automatically.

## Schedulers

The scheduler is the sampler that denoises each canvas, and it is interchangeable: swap it to change the sampling
strategy without touching anything else. Three schedulers are available:

- [BlockRefinementScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/block_refinement#diffusers.BlockRefinementScheduler) (default): commits the most confident tokens each step (above `threshold`, plus an even
  per-step quota) and renoises the rest. `editing_threshold` additionally lets it re-edit already committed tokens.
- [DiscreteDDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/discrete_ddim#diffusers.DiscreteDDIMScheduler): samples each position from the exact discrete posterior of the uniform corruption process
  (D3PM). It is parameter free, and the final step deterministically commits the predicted tokens.
- [EntropyBoundScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/entropy_bound#diffusers.EntropyBoundScheduler): commits the lowest-entropy positions whose joint entropy stays under `entropy_bound`, so
  roughly independent tokens are accepted together. It anneals its sampling temperature from `t_max` (`0.8`) on the
  first step down to `t_min` (`0.4`) on the last, matching the released checkpoint's sampler.

```py
from diffusers import DiscreteDDIMScheduler, EntropyBoundScheduler

pipe.scheduler = DiscreteDDIMScheduler()
# or: pipe.scheduler = EntropyBoundScheduler(entropy_bound=0.1)
output = pipe(prompt="Why is the sky blue?", gen_length=256, num_inference_steps=48)
print(output.texts[0])
```

Scheduler-specific sampling knobs (the block-refinement `threshold`/`top_k`, the entropy bound, ...) are set on the
scheduler config:

```py
from diffusers import BlockRefinementScheduler

pipe.scheduler = BlockRefinementScheduler.from_config(pipe.scheduler.config, threshold=0.9)
```

`EntropyBoundScheduler` anneals its sampling temperature (`t_max`/`t_min`) internally over the denoising steps;
`DiscreteDDIMScheduler` and `BlockRefinementScheduler` use the flat `temperature` passed to the pipeline (`0.0` for
greedy).

### Predictor-corrector sampling

`DiscreteDDIMScheduler` supports the leave-one-out predictor-corrector of [Uniform Diffusion Models Revisited: Leave-One-Out Denoiser and Absorbing State Reformulation](https://huggingface.co/papers/2605.22765). It refines the canvas with `corrector_steps` Gibbs sweeps that resample the least-confident positions from the one-coordinate conditional of the noisy marginal, which leaves that marginal invariant and improves generation at no extra training cost. It works directly on the released checkpoint: for uniform diffusion the denoiser and the leave-one-out posterior are interchangeable in closed form, so the corrector recovers the leave-one-out quantities it needs without any retraining.

The corrector sweeps are folded into the `num_inference_steps` budget rather than added on top: the pipeline runs fewer predictor steps and spends the freed forwards on correctors, so the total number of model forwards stays `num_inference_steps` and the predictor-corrector costs the same as plain ancestral sampling.

```py
from diffusers import DiscreteDDIMScheduler

pipe.scheduler = DiscreteDDIMScheduler(corrector_steps=2, corrector_k=12)
output = pipe(prompt="Why is the sky blue?", gen_length=256, num_inference_steps=48)
print(output.texts[0])
```

## PEFT adapters

The denoiser is a 🤗 Transformers model, so adapters are loaded through its native [PEFT](https://huggingface.co/docs/peft) integration rather than the diffusers `load_lora_weights` API. Because that integration is adapter-type-agnostic, the same calls load LoRA, DoRA, or any other PEFT adapter (e.g. the output of TRL's `SFTTrainer`). Manage adapters on the model component directly:

```py
pipe.model.load_adapter("path/to/adapter", adapter_name="sft")  # LoRA, DoRA, ...
pipe.model.set_adapter("sft")
output = pipe(prompt="Why is the sky blue?", gen_length=256)

pipe.model.disable_adapters()  # run the base model
pipe.model.delete_adapter("sft")
```

Adapters stay active and unmerged: DiffusionGemma ties the encoder and decoder base weights, so fusing an adapter into them would corrupt both branches.

## Static cache and compilation

The pipeline prefills the encoder once per block into a reusable cache (a `DynamicCache` by default). Passing
`cache_implementation="static"` uses a fixed-shape `StaticCache` instead, whose shapes let you `torch.compile` the
decoder with cudagraphs for a further speedup (the pipeline marks each step and clones the logits so cudagraph memory
is not overwritten); this is the setup shown in [Usage](#usage). Drop both the `torch.compile` call and
`cache_implementation="static"` for a simpler dynamic-cache run.

## Adaptive stopping

A block usually converges before all `num_inference_steps` are spent, so by default the pipeline freezes each batch
example once its argmax prediction is stable for `stability_threshold` steps and its mean per-token entropy falls below
`confidence_threshold` (`0.005`, the value used by the released checkpoint). The denoising loop ends once every example
is frozen. This roughly halves the number of decoder forwards at matched quality and is the largest single throughput
lever. Pass `confidence_threshold=None` to always run the full `num_inference_steps`:

```py
output = pipe(prompt="Why is the sky blue?", gen_length=256, confidence_threshold=None)  # disable adaptive stopping
```

## Callbacks

Callbacks run after each denoising step. Pass `callback_on_step_end_tensor_inputs` to select which tensors are
included in `callback_kwargs`; `canvas` (the current block tokens) and `logits` are available. Return `{"canvas": ...}`
from the callback to replace the canvas.

```py
def on_step_end(pipe, step, timestep, callback_kwargs):
    canvas = callback_kwargs["canvas"]
    # Inspect or modify `canvas` here.
    return {"canvas": canvas}

out = pipe(
    prompt="Why is the sky blue?",
    callback_on_step_end=on_step_end,
    callback_on_step_end_tensor_inputs=["canvas"],
)
```

## DiffusionGemmaPipeline[[diffusers.DiffusionGemmaPipeline]]

#### diffusers.DiffusionGemmaPipeline[[diffusers.DiffusionGemmaPipeline]]

```python
diffusers.DiffusionGemmaPipeline(model: Any, scheduler: BlockRefinementScheduler | DiscreteDDIMScheduler | EntropyBoundScheduler, processor: Any)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/diffusion_gemma/pipeline_diffusion_gemma.py#L53)

**Parameters:**

model ([DiffusionGemmaForBlockDiffusion](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/diffusion_gemma#transformers.DiffusionGemmaForBlockDiffusion)) : The block-diffusion denoiser (causal encoder + bidirectional decoder with tied weights).

scheduler ([BlockRefinementScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/block_refinement#diffusers.BlockRefinementScheduler), [DiscreteDDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/discrete_ddim#diffusers.DiscreteDDIMScheduler) or [EntropyBoundScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/entropy_bound#diffusers.EntropyBoundScheduler)) : The sampler that commits and renoises canvas tokens each denoising step.

processor ([ProcessorMixin](https://huggingface.co/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin)) : The processor used to apply the chat template and decode the generated tokens.

Pipeline for DiffusionGemma block-diffusion text generation.

DiffusionGemma is a block-diffusion encoder-decoder model: a causal encoder reads the clean prompt (and any
previously generated blocks) into a KV cache, and a bidirectional decoder denoises a fixed-size "canvas" of
`canvas_length` tokens by cross-attending to that cache. Generation alternates an outer autoregressive loop over
canvases with an inner denoising loop, where each step samples candidate tokens, commits the most confident ones
via [BlockRefinementScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/block_refinement#diffusers.BlockRefinementScheduler) (uniform corruption mode, `mask_token_id=None`), and renoises the rest.

The model is expected to be a `DiffusionGemmaForBlockDiffusion` instance exposing `forward(input_ids,
decoder_input_ids=..., self_conditioning_logits=..., ...)` and returning logits of shape `[batch, canvas_length,
vocab_size]` over the canvas. See the model card at https://huggingface.co/google/diffusiongemma-26B-A4B-it.

#### __call__[[diffusers.DiffusionGemmaPipeline.__call__]]

```python
__call__(prompt: str | list[str] | None = None, messages: list[dict] | None = None, image: Any | list[Any] | None = None, add_generation_prompt: bool = True, gen_length: int = 256, num_inference_steps: int = 48, temperature: float = 0.0, cache_implementation: str | None = None, eos_early_stop: bool = True, eos_token_id: int | None = None, stability_threshold: int = 1, confidence_threshold: float | None = 0.005, generator: torch.Generator | None = None, output_type: str = 'text', return_dict: bool = True, callback_on_step_end: Callable[[Any, int, int, dict], dict] | PipelineCallback | MultiPipelineCallbacks | None = None, callback_on_step_end_tensor_inputs: list[str] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/diffusion_gemma/pipeline_diffusion_gemma.py#L163)

**Parameters:**

prompt (`str` or `List[str]`, *optional*) : Prompt text, wrapped in a chat template and tokenized by the processor. Provide either this or `messages`.

messages (`List[Dict]`, *optional*) : A raw chat conversation to encode, e.g. `[{"role": "user", "content": "Hello"}]` or a multi-turn / multimodal conversation. Use this instead of `prompt` for anything beyond a single user turn.

image (`PIL.Image.Image` or `List`, *optional*) : Image(s) to pair with `prompt` for multimodal generation; the processor turns them into the model's image inputs. For richer layouts, put the image content directly in `messages`.

add_generation_prompt (`bool`, defaults to `True`) : Whether to add the generation prompt when applying the chat template.

gen_length (`int`, defaults to `256`) : Number of tokens to generate, rounded up to a multiple of the model's `canvas_length`.

num_inference_steps (`int`, defaults to `48`) : Number of denoising steps per canvas.

temperature (`float`, defaults to `0.0`) : Sampling temperature for `DiscreteDDIMScheduler`/`BlockRefinementScheduler` (`0.0` is greedy); `EntropyBoundScheduler` ignores it and anneals its own temperature. Other sampling knobs (e.g. `top_k`, `threshold`, `t_min`/`t_max`) are scheduler config; set them on the scheduler, e.g. `pipe.scheduler = BlockRefinementScheduler.from_config(pipe.scheduler.config, top_k=...)`.

cache_implementation (`str`, *optional*) : Set to `"static"` to prefill the encoder once per block into a persistent `StaticCache` and run the decoder against it with fixed shapes, instead of re-encoding the full sequence on every step. The fixed shapes also let you compile the decoder, e.g. `pipe.model.model.decoder = torch.compile(pipe.model.model.decoder, fullgraph=True)`.

eos_early_stop (`bool`, defaults to `True`) : Whether to stop generating further canvases once every sequence has emitted EOS.

eos_token_id (`int`, *optional*) : EOS token ID for early stopping. Falls back to the processor's tokenizer.

stability_threshold (`int`, defaults to `1`) : Number of consecutive steps an example's argmax prediction must remain unchanged for that example to count as stable. Only used when `confidence_threshold` is set.

confidence_threshold (`float`, *optional*, defaults to `0.005`) : Freeze each example once it is stable (see `stability_threshold`) and the mean per-token entropy of its scheduler-shaped prediction logits is below this value. The block's denoising loop ends once every example is frozen. Speeds up generation at matched quality; the default matches the released checkpoint. Set to `None` to always run all `num_inference_steps`.

generator (`torch.Generator`, *optional*) : RNG for sampling.

output_type (`str`, defaults to `"text"`) : `"text"` decodes sequences into strings (requires a processor); `"seq"` returns token IDs only.

return_dict (`bool`, defaults to `True`) : Whether to return a [DiffusionGemmaPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/diffusion_gemma#diffusers.DiffusionGemmaPipelineOutput) instead of a tuple.

callback_on_step_end (`Callable` or `PipelineCallback`, *optional*) : Callback run after each denoising step with signature `callback_on_step_end(self, step, timestep, callback_kwargs)`. Allowed tensor keys: `canvas`, `logits`.

callback_on_step_end_tensor_inputs (`List[str]`, *optional*) : Tensor keys to pass to the callback.

**Returns:** [DiffusionGemmaPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/diffusion_gemma#diffusers.DiffusionGemmaPipelineOutput) or `tuple`

The generated token IDs (`sequences`) and, for `output_type="text"`, the decoded `texts`.

Generate text with block diffusion.

Examples:
```python
>>> import torch
>>> from transformers import AutoProcessor, DiffusionGemmaForBlockDiffusion
>>> from diffusers import BlockRefinementScheduler, DiffusionGemmaPipeline

>>> model_id = "google/diffusiongemma-26B-A4B-it"
>>> model = DiffusionGemmaForBlockDiffusion.from_pretrained(model_id, dtype=torch.bfloat16, device_map="auto")
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> scheduler = BlockRefinementScheduler()

>>> pipe = DiffusionGemmaPipeline(model=model, scheduler=scheduler, processor=processor)
>>> output = pipe(prompt="Why is the sky blue?", gen_length=256)
>>> print(output.texts[0])
```

## DiffusionGemmaPipelineOutput[[diffusers.DiffusionGemmaPipelineOutput]]

#### diffusers.DiffusionGemmaPipelineOutput[[diffusers.DiffusionGemmaPipelineOutput]]

```python
diffusers.DiffusionGemmaPipelineOutput(sequences: torch.LongTensor, texts: list[str] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/diffusion_gemma/pipeline_output.py#L25)

**Parameters:**

sequences (`torch.LongTensor` of shape `(batch_size, gen_length)`) : The generated token IDs (the prompt is stripped off).

texts (`list[str]`, *optional*) : The decoded text, one string per sequence. Only set for `output_type="text"`.

Output class for DiffusionGemma block-diffusion generation.
