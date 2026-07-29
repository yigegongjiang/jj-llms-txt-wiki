# Fusion mapping (experimental feature)

Fusion mapping provides an opt-in way to replace model submodules at load time while preserving the original checkpoint format.

It builds on:

- [Monkey patching](./monkey_patching) to swap module classes before model instantiation.
- [Dynamic weight loading](./weightconverter) to map weights between the original and fused runtime layouts.

> [!WARNING]
> Fusion mapping is an experimental loading feature. It changes the runtime module structure and may affect model behavior. Use it only when you explicitly want a fused runtime layout.

## Quick start

Fusion is enabled through [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) with `fusion_config`:

```python
from transformers import AutoModelForImageTextToText

model = AutoModelForImageTextToText.from_pretrained(
    "Qwen/Qwen2-VL-2B-Instruct",
    fusion_config={"patch_embeddings": True},
)
```

By default, no fusion is applied.
If `fusion_config` is stored in the model config, `from_pretrained()` will reuse it automatically.

## How it works

Fusion registration happens before the model is instantiated:

1. [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) uses the explicit `fusion_config` argument or falls back to `config.fusion_config`.
2. The fusion registry validates the requested fusion names.
3. Each enabled fusion meta-initializes the target model class, optionally filters candidate modules by name, and uses `is_fusable(...)` to discover compatible module classes.
4. Fused replacement classes are registered through [register_patch_mapping()](/docs/transformers/v5.14.0/en/monkey_patching#transformers.monkey_patching.register_patch_mapping).
5. Matching `~WeightTransform` rules are generated from the config so checkpoint loading can map weights into the fused runtime layout.
6. By default, [save_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.save_pretrained) uses the reverse conversion path to restore the original checkpoint layout. Pass `save_original_format=False` to keep the converted runtime layout instead.

This lets a fusion use a different runtime module structure while still loading from the original checkpoint format, and by default saving back to it as well.

Note: With the current monkey-patching mechanism, fusion registration is class-level: one compatible module class maps to one fused replacement class.

## Current fusion families

Currently, `fusion_config` supports one fusion family:

- `patch_embeddings`
  Enable with:

  ```python
  fusion_config = {"patch_embeddings": True}
  ```

  Effect:
  Replaces compatible `nn.Conv3d` patch embedding projections with equivalent flattened `nn.Linear` projections at runtime.

## Extending fusion mapping

To add a new fusion family:

1. Add an `is_fusable` predicate.
   This decides whether a discovered module is compatible with the fusion.
2. Optionally add `target_modules_patterns`.
   This makes the discovery step more explicit by pre-filtering candidate module names before `is_fusable(...)`.
3. Add a `make_fused_class` factory.
   This returns the runtime replacement class for a compatible module class.
4. Add a `make_transforms` factory if the fused layout needs checkpoint conversion.
   This returns the `~WeightTransform` rules that map weights between the original and fused layouts for a given config.
5. Register the new `ModuleFusionSpec` in [`fusion_mapping.py`](https://github.com/huggingface/transformers/blob/main/src/transformers/fusion_mapping.py).

Once registered, the new fusion becomes available through `fusion_config`.

## Internal API[[transformers.fusion_mapping.ModuleFusionSpec]]

Base recipe for a fusion family.

A fusion spec decides which modules are eligible for a fusion, how to build
the runtime replacement class, and which weight transforms are needed to map
checkpoints between the original and fused layouts.

Return the log message emitted when no compatible modules are found.

Return whether `module` is compatible with this fusion family.

Build the runtime replacement class for a compatible module class.

Build the weight transforms needed to load and save the fused runtime layout.

Fuse compatible Conv3d patch embeddings into flattened Linear projections.

Register one fusion family for `cls`.

This function updates the two global registries used by fused loading:
- the monkey-patching registry, so compatible module classes are replaced before initialization
- the checkpoint conversion mapping, so fused runtime modules still load from the original checkpoint layout

Notes:
- conflicting checkpoint transforms fail fast

Register requested runtime fusions for `cls`.

This function:
- validates `fusion_config` against `_FUSION_REGISTRY`
- resolves the enabled fusion families in user order
- registers monkey patches and checkpoint transforms before model instantiation
