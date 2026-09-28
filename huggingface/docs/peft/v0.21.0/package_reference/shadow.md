# ShadowPEFT

[ShadowPEFT](https://arxiv.org/abs/2604.19254) augments a frozen base decoder-only model with a **lightweight, pretrainable** *shadow* network that runs in parallel with the backbone. A small shadow backbone produces an initial shadow state `s^(0)`, which then rides the base model's decoder loop: at every targeted block the discrepancy between the base hidden states and the shadow state is injected back into the block input (a low-rank correction), and the shadow state is advanced by a gated residual update computed from the block output. Only the shadow components are trained; the base model stays frozen.

```
Input
  ├──► Shadow backbone (small, trainable) ──► s^(0)
  └──► Base model (frozen, large)
         block_0 ◄── inject(h, s) ─► h_0 ──► update ─► s_1
         block_1 ◄── inject(h, s) ─► h_1 ──► update ─► s_2
         ...                       (the (hidden, shadow) pair rides the loop together)
```

Because the adaptation is an **input-dependent trajectory in layer space** (the shadow state evolves with the data) rather than a static weight-space delta, ShadowPEFT **cannot be merged** into the base weights. Calling `merge`, `merge_adapter`, or `merge_and_unload` raises an explicit error. For Transformers language models, you can obtain the lightweight shadow network on its own with `model.base_model.unload_shadow()`, which returns a standalone `DetachedShadowModel`. Standalone unloading is not supported for Diffusers models because reconstructing a complete denoiser is architecture-specific.

Adding multiple adapters, switching between them with `set_adapter`, deleting them, and enabling/disabling them all work as with other PEFT methods. Only **one** adapter can be active at a time, because the shadow state is a single trajectory through the network.

The shadow backbone can be built in two ways, controlled by `ShadowConfig.shadow_model`:

- `"mirror"` (default): a smaller shadow backbone is created automatically. Language models use a reduced copy of the base architecture. Diffusers architectures with a registered backend use a reduced architecture-alike model initialized from selected base weights; compatible architectures without a backend fall back to a token-wise residual MLP. When the shadow hidden size differs from the base, a trained projection bridges the gap.
- a model id or local path: the backbone is loaded as a smaller pre-trained model. Transformers models use `AutoModel`; registered Diffusers backends define their own compatible checkpoint loading.

Architecture-aware Diffusers support is selected automatically from the model class. Flux2 currently has a registered backend; other compatible transformer-based Diffusers models use the generic MLP fallback. Standalone `unload_shadow()` remains unsupported for all Diffusers models because reconstructing a complete denoiser is architecture-specific.

Compared to LoRA-style methods, ShadowPEFT adds more parameters and compute (it runs a parallel network and wraps whole decoder blocks), but the adapter is a self-contained network that can be trained centrally, reused across tasks, and initialized from a pre-trained small model. An optional auxiliary loss (`auxiliary_loss_weight`) applies the task head to the initial shadow state `s^(0)` and adds it to the task loss, encouraging the detachable shadow path to solve the task on its own. For causal LM, the base output head is reused; include `"lm_head"` in `modules_to_save` to train and save it through the standard PEFT mechanism.

## KV cache

ShadowPEFT supports incremental decoding with a **dual** KV cache: one for the frozen base model and one for the
shadow backbone. Inject/update are token-local, so a new token only needs its own shadow state `s`; causality keeps
prefix base keys/values (computed under injection) valid. The paired object is a [ShadowCache](/docs/peft/v0.21.0/en/package_reference/shadow#peft.tuners.shadow.ShadowCache),
returned as `past_key_values` when `use_cache=True`. You can pass `use_cache=True` to `generate()` as usual.

```py
out = model.generate(input_ids, max_new_tokens=32)  # dual KV cache enabled by default
# or explicitly:
out = model.generate(input_ids, use_cache=True, max_new_tokens=32)
```

`use_cache=False` still works and reprocesses the full sequence each step (useful for debugging).

## Usage

```py
from transformers import AutoModelForCausalLM
from peft import ShadowConfig, get_peft_model

model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen3-0.6B")
config = ShadowConfig(r=8, shadow_num_hidden_layers=1, task_type="CAUSAL_LM")
model = get_peft_model(model, config)
model.print_trainable_parameters()

out = model.generate(input_ids, max_new_tokens=32)
```

To initialize the shadow backbone from a smaller pre-trained model, pass its id or path as `shadow_model`:

```py
config = ShadowConfig(shadow_model="Qwen/Qwen3-0.6B", task_type="CAUSAL_LM")
model = get_peft_model(base_model, config)
```

## Evaluating the shadow path

By default the model output (`logits`) is the **shadow-adapted base model**: the shadow corrections are injected into
the base model's hidden states at every layer, so `logits` already reflects ShadowPEFT (use `model.disable_adapter()`
to get the plain base model for comparison). The auxiliary loss additionally trains the shadow path to solve the task
on its own.

To evaluate the **standalone shadow network** for a Transformers language model (the detachable, lightweight model —
the ShadowPEFT analogue of `merge_and_unload`), use `unload_shadow()`. It returns
`head(projection(backbone(x)))` as a normal task model that you can evaluate like any other: for a causal-LM task it is
a generation-capable causal LM (supports `generate()` and KV caching), and for a sequence-classification task it pools
the last token and returns class logits. Calling this method for a Diffusers model raises `NotImplementedError`.

```py
shadow = model.base_model.unload_shadow()  # a DetachedShadowModel (a PreTrainedModel)
shadow.eval()
# causal LM:
out = shadow.generate(input_ids, max_new_tokens=32)
# sequence classification:
logits = shadow(input_ids=input_ids, attention_mask=attention_mask).logits  # (batch, num_labels)
```

By default (`copy=False`) the returned model shares its modules with the PEFT model, and a shadow backbone that shares the frozen base input embeddings reaches them through a reference that is not a submodule. That is fine for evaluation, but it means `save_pretrained` would write a checkpoint without the embedding table. Pass `copy=True` when you want to save or push the standalone model:

```py
shadow = model.base_model.unload_shadow(copy=True)
shadow.save_pretrained("standalone-shadow")
```

# API

## ShadowConfig[[peft.ShadowConfig]]

#### peft.ShadowConfig[[peft.ShadowConfig]]

```python
peft.ShadowConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, target_modules: typing.Union[str, list[str], NoneType] = None, exclude_modules: typing.Union[str, list[str], NoneType] = None, r: int = 8, shadow_alpha: float = 0.1, shadow_dropout: float = 0.2, init_weights: bool = True, shadow_model: str = 'mirror', shadow_num_hidden_layers: typing.Optional[int] = None, shadow_hidden_size: typing.Optional[int] = None, shadow_num_attention_heads: typing.Optional[int] = None, shadow_intermediate_size: typing.Optional[int] = None, share_embeddings: bool = True, update_hidden_size: typing.Optional[int] = None, auxiliary_loss_weight: float = 0.05, layers_to_transform: typing.Union[list[int], int, NoneType] = None, layers_pattern: typing.Union[str, list[str], NoneType] = None, modules_to_save: typing.Optional[list[str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/tuners/shadow/config.py#L23)

**Parameters:**

target_modules (`Optional[Union[list[str], str]]`) : The transformer blocks to wrap with the shadow mechanism (whole decoder blocks, not linear layers). Can be a list of module names, or a regex, e.g. `r'.*\.layers\.\d+$'` to target every decoder block. Defaults to `None`, in which case every decoder block of the base model is wrapped. Note that the wrapped blocks must be contiguous, because the shadow state rides the decoder loop from the first wrapped block to the last.

exclude_modules (`Optional[Union[list[str], str]]`) : The names of the modules to not wrap with the shadow mechanism, given as a list or a regex. Defaults to `None`.

r (`int`) : The rank of the low-rank injection bottleneck `W_down` / `W_up`. Defaults to `8`.

shadow_alpha (`float`) : The strength of the injected correction added onto the block input (Eq. 4). Defaults to `0.1`.

shadow_dropout (`float`) : The dropout probability applied to the discrepancy signal before the bottleneck (Eq. 3). Defaults to `0.2`.

init_weights (`bool`) : Whether to zero-initialize `W_up` so the injection is a no-op at the start of training (mirroring LoRA's `B=0` convention). Don't change this unless you know what you are doing. Defaults to `True`.

shadow_model (`str`) : How to build the shadow backbone. `"mirror"` (default) builds a fresh backbone of the same architecture as the base model but with fewer/smaller layers (see the `shadow_*` overrides below). Diffusers architectures with a registered Shadow backend use a reduced architecture-alike model; other compatible diffusion transformers fall back to a generic token-wise MLP. Any other string is treated as a model id or local path. A "projected" Transformers shadow checkpoint (`model_type == "causal_lm_with_hidden_projection"`, e.g. `shadow-llm/Qwen3-0.6B-H8B`) loads its pretrained backbone together with its trained shadow-hidden -> base-hidden projection. Explicit Diffusers checkpoints require a registered backend; currently Flux2 is supported.

shadow_num_hidden_layers (`Optional[int]`) : The depth of the auto-built (`"mirror"`) shadow backbone. For Flux2 this creates the requested number of double-stream blocks and twice as many single-stream blocks; for the generic Diffusers fallback, it is the number of residual MLP blocks. Defaults to `None` (`1` layer).

shadow_hidden_size (`Optional[int]`) : The hidden size of the auto-built shadow backbone; may differ from the base hidden size (a projection is inserted automatically). Architecture backends may impose additional constraints; Flux2 reductions must preserve its base attention head dimension. Defaults to `None` (same as the base model).

shadow_num_attention_heads (`Optional[int]`) : The number of attention heads of the auto-built shadow backbone. Defaults to `None` (same as the base model). This option is used by architecture-aware backends but not by the generic Diffusers MLP fallback.

shadow_intermediate_size (`Optional[int]`) : The feed-forward width of the auto-built shadow backbone. Defaults to `None` (same as its hidden size for the generic Diffusers MLP fallback).

share_embeddings (`bool`) : Whether to reuse the frozen base input embeddings to feed the shadow backbone (via `inputs_embeds`) instead of the shadow backbone's own embedding table. Defaults to `True`.

update_hidden_size (`Optional[int]`) : The hidden width of the `T` (candidate) and `G` (gate) update MLPs. Defaults to `None` (uses `r`).

auxiliary_loss_weight (`float`) : The weight `lambda` of the auxiliary shadow loss (Eq. 8-9) that is added to the task loss when `labels` are passed. Diffusion training loops can use the same value for the detached denoising loss. Set to `0` to disable it. Defaults to `0.05`.

layers_to_transform (`Optional[Union[list[int], int]]`) : The block indices to transform. If a list is passed, the shadow mechanism is applied to the blocks at those indices. If a single integer is passed, it is applied at that index only. Defaults to `None` (every matched block is transformed).

layers_pattern (`Optional[Union[list[str], str]]`) : The layer pattern name, used only if `layers_to_transform` is different from `None`. This is the name of the `nn.ModuleList` that holds the decoder blocks (often `"layers"` or `"h"`). Defaults to `None`.

modules_to_save (`Optional[list[str]]`) : The extra modules to set as trainable and save in the final checkpoint (e.g. `"lm_head"` or a classifier head). Defaults to `None`.

Configuration class for [ShadowModel](/docs/peft/v0.21.0/en/package_reference/shadow#peft.ShadowModel) (ShadowPEFT).

ShadowPEFT augments a frozen base decoder-only model with a small, trainable parallel *shadow* network. A shadow
backbone produces an initial shadow state `s^(0)` that then rides the base model's decoder loop: at every targeted
block the discrepancy between the base hidden states and the shadow state is injected back into the block input
(Eq. 2-4), and the shadow state is advanced by a gated residual update from the block output (Eq. 5-7). Only the
shadow components are trained; the base model stays frozen. Because the adaptation is an input-dependent trajectory
in layer space rather than a static weight delta, ShadowPEFT cannot be merged into the base weights.

## ShadowModel[[peft.ShadowModel]]

#### peft.ShadowModel[[peft.ShadowModel]]

```python
peft.ShadowModel(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/tuners/shadow/model.py#L232)

Creates a ShadowPEFT model from a pretrained transformers model.

ShadowPEFT augments a frozen base decoder-only model with a small, trainable parallel *shadow* network. A shadow
backbone produces an initial shadow state that rides the base decoder loop; at every targeted block the discrepancy
between the base hidden states and the shadow state is injected into the block, and the shadow state is advanced by
a gated residual update. Only the shadow components are trained. See [ShadowConfig](/docs/peft/v0.21.0/en/package_reference/shadow#peft.ShadowConfig) for the configuration.

The method cannot be merged into the base weights (the adaptation is an input-dependent trajectory, not a static
weight delta); use [ShadowModel.unload_shadow()](/docs/peft/v0.21.0/en/package_reference/shadow#peft.ShadowModel.unload_shadow) to obtain the standalone shadow network instead.

#### shadow_auxiliary_loss[[peft.ShadowModel.shadow_auxiliary_loss]]

```python
shadow_auxiliary_loss(labels: Tensor, attention_mask: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/tuners/shadow/model.py#L792)

The shadow path's own task loss, `CE(shadow_head(s^(0)), labels)` (unweighted; `forward` applies the weight).

The loss is computed on the *initial* shadow state `s^(0)` (the shadow backbone output, projected) -- exactly
what the standalone `unload_shadow()` model computes as `head(projection(backbone(x)))`. This is what makes the
detached shadow network usable on its own; training it on `s^(L)` (the final state, which depends on the base
model's per-layer outputs and does not exist standalone) would leave the detached model untrained.

#### unload_shadow[[peft.ShadowModel.unload_shadow]]

```python
unload_shadow(adapter_name: typing.Optional[str] = None, copy: bool = False)
```

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/tuners/shadow/model.py#L875)

**Parameters:**

adapter_name (`str`, *optional*) : The adapter whose shadow network to unload. Defaults to the active adapter.

copy (`bool`, *optional*, defaults to `False`) : If `True`, deep-copy the returned model so it is independent of this one (uses more memory). If `False` (default), share modules -- similar to `merge_and_unload`, which reuses modules rather than cloning them. Mutating one model then affects the other.

Return the shadow backbone (+ head) as a standalone model, *without* the base model.

The ShadowPEFT analogue of `merge_and_unload`: where that would hand back the base model with the adaptation
baked in, this hands back only the lightweight shadow network for high-efficiency / edge inference. It runs
`head(projection(backbone(x)))` -- the per-block updates require the base outputs and so do not exist
standalone. For language models, the result behaves like a normal causal LM (supports `generate()` and KV
caching). Diffusers models are not supported because reconstructing a complete standalone denoiser is specific
to each diffusion architecture.

Assign the result to a variable and use it; with `copy=False` the modules remain shared with this model.

Pass `copy=True` when you intend to `save_pretrained` the standalone model. A shadow backbone that shares the
frozen base input embeddings (the default `"mirror"` setup) reaches them through a reference that is not a
submodule of the returned model, so a `copy=False` checkpoint is missing the embedding table; `copy=True`
re-attaches a private copy and saves a complete checkpoint.

## ShadowCache[[peft.tuners.shadow.ShadowCache]]

#### peft.tuners.shadow.ShadowCache[[peft.tuners.shadow.ShadowCache]]

```python
peft.tuners.shadow.ShadowCache(base: typing.Any = None, shadow: typing.Any = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.21.0/src/peft/tuners/shadow/layers.py#L59)

Paired KV caches for incremental ShadowPEFT decoding.

Autoregressive generation needs a base-model cache (keys/values computed under shadow injection) *and* a separate
shadow model cache (to advance `s^(0)` token-by-token). This is a Transformers `Cache` subclass: operations that
affect the batch or sequence layout (reset, reorder, crop, repeat, and select) are applied to both caches, while
attention metadata and updates delegate to the base cache. Unknown cache attributes also delegate to the base cache
for compatibility with architecture-specific cache implementations.

The wrapper is intentionally not compileable because the two caches can have different layer layouts and hidden
sizes. Legacy tuple conversion is not supported, since a single legacy tuple cannot represent both paths. The
shadow half is unpacked before the base forward and re-packed on the way out -- see [ShadowModel](/docs/peft/v0.21.0/en/package_reference/shadow#peft.ShadowModel) hooks.
