# Exporters

New export backends can be added to Transformers by subclassing `HfExporter`.

Learn how to use the built-in exporters in the [Exporters](../exporters) guide.

## AutoHfExporter[[transformers.exporters.AutoHfExporter]]

#### transformers.exporters.AutoHfExporter[[transformers.exporters.AutoHfExporter]]

```python
transformers.exporters.AutoHfExporter()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/auto.py#L70)

The Auto-HF expoerter class that takes care of automatically instantiating to the correct
`HfExporter` given the `ExportConfig`.

#### from_pretrained[[transformers.exporters.AutoHfExporter.from_pretrained]]

```python
from_pretrained(pretrained_model_name_or_path, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/auto.py#L90)

Load an exporter instance from a pretrained model/checkpoint that ships an export config.

**Not implemented yet** — placeholder for a first-class "export recipe" workflow.

The idea: model owners publish an `export_config.json` (or an `export_config` field in
`config.json`) alongside their weights on the Hub. That file captures the settings the
owner has already validated for their architecture — the target format (`dynamo` /
`onnx` / `executorch`), exact dynamic-shape specs (e.g. `text_ids` dynamic to 4096,
image tiles fixed at 448, `batch=1` for edge deployment), `strict` flag, ONNX opset,
prefill vs. decode layout, ExecuTorch backend choice, and any other knob that today lives
as tribal knowledge in a README or a private notebook.

Consumers then get the owner-validated export in one call:

```python
exporter = AutoHfExporter.from_pretrained("org/model-name")
program = exporter.export(model, inputs)
```

        Composes with the [*register_export_input_preparer*] registry: the owner supplies the
        shape spec via `export_config.json`, transformers supplies the data-dependent
        precomputations (`cu_seqlens`, vision position ids, window indices, …) for that
        architecture. Together they cover the two hard parts of exporting new models — knowing
        the right shape contract and preparing the right inputs — so downstream users don't
        re-derive either from scratch (and don't break in production when they get it wrong).

#### supports_export_format[[transformers.exporters.AutoHfExporter.supports_export_format]]

```python
supports_export_format(export_config_dict: dict)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/auto.py#L122)

Return True if the provided dict describes an `export_format` that has both a
registered config class and a registered exporter class. Warns with an actionable message
when the format is missing entirely, unknown, or only half-registered.

## AutoExportConfig[[transformers.exporters.AutoExportConfig]]

#### transformers.exporters.AutoExportConfig[[transformers.exporters.AutoExportConfig]]

```python
transformers.exporters.AutoExportConfig()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/auto.py#L42)

The Auto-HF export config class that takes care of automatically dispatching to the correct
export config given an export config stored in a dictionary.

## HfExporter[[transformers.exporters.HfExporter]]

#### transformers.exporters.HfExporter[[transformers.exporters.HfExporter]]

```python
transformers.exporters.HfExporter()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/base.py#L43)

Abstract base class for all Transformers exporters.

Subclass and implement `~HfExporter.export` to add a new export backend.

#### export[[transformers.exporters.HfExporter.export]]

```python
export(model: PreTrainedModel, sample_inputs: MutableMapping[str, torch.Tensor | Cache], config: ExportConfigMixin)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/base.py#L101)

**Parameters:**

model ([PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel)) : The model to export.

sample_inputs (`dict[str, torch.Tensor | Cache]`) : **Forward** kwargs — what you'd pass to `model(**sample_inputs)`. These are used directly as the example inputs during tracing. For an autoregressive decode-step export, this means you need to include `past_key_values`, `cache_position`, etc. If you only have generation-style inputs, use `~HfExporter.export_for_generation` instead — it runs `model.generate` for you and exports each stage.

config (`ExportConfigMixin`) : Backend-specific configuration.

**Returns:**

Backend-specific export artifact.

Export the model and return the backend-specific program object.

#### export_for_generation[[transformers.exporters.HfExporter.export_for_generation]]

```python
export_for_generation(model: PreTrainedModel, sample_inputs: MutableMapping[str, torch.Tensor | Cache], config: ExportConfigMixin | dict[str, ExportConfigMixin], generation_config: GenerationConfig | None = None, multi_token_decode: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/base.py#L133)

**Parameters:**

model ([PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel)) : The generative model to export. Must support `model.generate(**sample_inputs)`.

sample_inputs (`dict[str, torch.Tensor | Cache]`) : **Generate** kwargs — what you'd pass to `model.generate(**sample_inputs)` (typically `input_ids` + `attention_mask`, plus any modality inputs like `pixel_values` / `input_features` for multi-modal models). Per-stage forward kwargs are captured internally.

config (`ExportConfigMixin` or `dict[str, ExportConfigMixin]`) : Backend-specific configuration. Pass a single config to apply to every component, or a `dict` keyed by component name (e.g. `"image_encoder"`, `"language_model"`, `"lm_head"`, `"decode"`) to override per-component — all component names must be present in the dict.

generation_config ([GenerationConfig](/docs/transformers/v5.15.1/en/main_classes/text_generation#transformers.GenerationConfig), *optional*) : Forwarded to the `generate()` capture (defaults to the model's own). Pass one with `cache_implementation="static"` to export against a fixed-size `StaticCache`.

multi_token_decode (`bool`, *optional*, defaults to `False`) : Whether the `decode` component processes multiple query tokens at once — a dynamic query axis (prefill on an empty cache, continuation-from-past otherwise) — vs the classic single-token step (see [decompose_for_generation()](/docs/transformers/v5.15.1/en/main_classes/exporters#transformers.exporters.utils.decompose_for_generation)). Only stays dynamic under a dynamic-shape export (`config.dynamic=True`).

**Returns:** `dict[str, Any]`

`{component_name: backend_specific_artifact}` — same keys as
[decompose_for_generation()](/docs/transformers/v5.15.1/en/main_classes/exporters#transformers.exporters.utils.decompose_for_generation). Values are whatever
`~HfExporter.export` returns for the concrete backend (`ExportedProgram`,
`ONNXProgram`, `ExecutorchProgramManager`).

Decompose a generative model and export each component independently.

Thin wrapper around [decompose_for_generation()](/docs/transformers/v5.15.1/en/main_classes/exporters#transformers.exporters.utils.decompose_for_generation) that calls
`~HfExporter.export` on every returned `(submodel, forward_inputs)` pair. If you need
the intermediate `(submodel, forward_inputs)` pairs (for verification, custom inputs,
skipping a stage, …), call [decompose_for_generation()](/docs/transformers/v5.15.1/en/main_classes/exporters#transformers.exporters.utils.decompose_for_generation) directly.

#### validate_environment[[transformers.exporters.HfExporter.validate_environment]]

```python
validate_environment(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/base.py#L59)

Check `required_packages` are installed and warn on version drift from `tested_versions`.

## DynamoExporter[[transformers.exporters.DynamoExporter]]

#### transformers.exporters.DynamoExporter[[transformers.exporters.DynamoExporter]]

```python
transformers.exporters.DynamoExporter()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/exporter_dynamo.py#L69)

Exporter that converts a [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel) to an `ExportedProgram`.

Example:

```python
>>> from transformers.exporters.exporter_dynamo import DynamoExporter, DynamoConfig

>>> exporter = DynamoExporter()
>>> exported = exporter.export(model, inputs, config=DynamoConfig(dynamic=True))
>>> outputs = exported.module()(**inputs)
```

#### export[[transformers.exporters.DynamoExporter.export]]

```python
export(model: PreTrainedModel, sample_inputs: MutableMapping[str, Any], config: DynamoConfig | dict[str, Any])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/exporter_dynamo.py#L87)

## OnnxExporter[[transformers.exporters.OnnxExporter]]

#### transformers.exporters.OnnxExporter[[transformers.exporters.OnnxExporter]]

```python
transformers.exporters.OnnxExporter()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/exporter_onnx.py#L87)

Exporter that converts a [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel) to an ONNX `ONNXProgram`.

Example:

```python
>>> from transformers.exporters.exporter_onnx import OnnxExporter, OnnxConfig

>>> exporter = OnnxExporter()
>>> onnx_program = exporter.export(model, inputs, config=OnnxConfig(dynamic=True))
>>> outputs = onnx_program(**inputs)  # run in-memory
>>> exporter.export(model, inputs, config=OnnxConfig(output_path="model.onnx"))  # save to disk
```

#### export[[transformers.exporters.OnnxExporter.export]]

```python
export(model: PreTrainedModel, sample_inputs: MutableMapping[str, Any], config: OnnxConfig | dict[str, Any])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/exporter_onnx.py#L105)

## ExecutorchExporter[[transformers.exporters.ExecutorchExporter]]

#### transformers.exporters.ExecutorchExporter[[transformers.exporters.ExecutorchExporter]]

```python
transformers.exporters.ExecutorchExporter()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/exporter_executorch.py#L109)

Exporter that converts a [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel) to an ExecuTorch `ExecutorchProgramManager`.

Example:

```python
>>> from transformers.exporters.exporter_executorch import ExecutorchExporter, ExecutorchConfig

>>> exporter = ExecutorchExporter()
>>> et_program = exporter.export(model, inputs, config=ExecutorchConfig(backend="xnnpack"))
>>> et_program.write_to_file("model.pte")
```

#### export[[transformers.exporters.ExecutorchExporter.export]]

```python
export(model: PreTrainedModel, sample_inputs: MutableMapping[str, Any], config: ExecutorchConfig | dict[str, Any])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/exporter_executorch.py#L126)

Export a model to ExecuTorch, applying backend preparation and torch op patches.

## DynamoConfig[[transformers.exporters.DynamoConfig]]

#### transformers.exporters.DynamoConfig[[transformers.exporters.DynamoConfig]]

```python
transformers.exporters.DynamoConfig(export_format: ExportFormat = <ExportFormat.DYNAMO: 'dynamo'>, dynamic: bool = False, strict: bool = False, dynamic_shapes: dict[str, typing.Any] | None = None, prefer_deferred_runtime_asserts_over_guards: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/configs.py#L76)

**Parameters:**

dynamic (*bool*, *optional*, defaults to *False*) : Whether to export with dynamic (symbolic) shapes. When *True* and *dynamic_shapes* is not set, all tensor dimensions are set to *Dim.AUTO* automatically.

strict (*bool*, *optional*, defaults to *False*) : Whether to enable strict mode in *torch.export*. Runs the full symbolic trace and catches more errors, but is slower and more likely to fail on complex models.

dynamic_shapes (*dict[str, Any]*, *optional*) : Explicit per-input dynamic shape specifications passed to *torch.export*. Takes precedence over *dynamic*.

prefer_deferred_runtime_asserts_over_guards (*bool*, *optional*, defaults to *False*) : When *True*, data-dependent shape guards are emitted as runtime asserts in the exported graph instead of failing the export at trace time when a guard wouldn't hold across the full symbolic shape range. Most transformer LLMs need this set to *True* when using fine-grained `Dim(min=, max=)` bounds. Not needed with `dynamic=True` / `Dim.AUTO`, where `torch.export` infers shape relations instead of verifying them against the user-stated bounds.

Configuration class for exporting models via *torch.export*.

## OnnxConfig[[transformers.exporters.OnnxConfig]]

#### transformers.exporters.OnnxConfig[[transformers.exporters.OnnxConfig]]

```python
transformers.exporters.OnnxConfig(export_format: ExportFormat = <ExportFormat.ONNX: 'onnx'>, dynamic: bool = False, strict: bool = False, dynamic_shapes: dict[str, typing.Any] | None = None, prefer_deferred_runtime_asserts_over_guards: bool = False, output_path: str | os.PathLike | None = None, opset_version: int | None = None, external_data: bool = True, optimize: bool = True, export_params: bool = True, keep_initializers_as_inputs: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/configs.py#L110)

**Parameters:**

output_path (`str` or `PathLike`, *optional*) : Output path for the `.onnx` file. When `None` (default) the exported model is kept in memory as an `ONNXProgram` and not written to disk.

opset_version (`int`, *optional*) : ONNX opset version to target. Defaults to the latest opset supported by the installed `onnxscript` version.

external_data (`bool`, *optional*, defaults to `True`) : Store large weight tensors in a separate `.onnx_data` sidecar file instead of embedding them in the protobuf. Required for models whose weights exceed the 2 GB protobuf limit.

optimize (`bool`, *optional*, defaults to `True`) : Run `onnxscript` optimisation passes (constant folding, dead-code elimination, …) on the exported graph. Disable for models that hit upstream `onnxscript` optimiser bugs.

export_params (`bool`, *optional*, defaults to `True`) : Embed model weights in the ONNX graph. Set to `False` to export a weight-free graph (weights must be supplied at runtime).

keep_initializers_as_inputs (`bool`, *optional*, defaults to `False`) : Expose weight initializers as explicit graph inputs. Required by some older ONNX runtimes (opset < 9).

Configuration class for exporting models to ONNX via `torch.onnx.export`.

Inherits all fields from `DynamoConfig` (`dynamic`, `strict`,
`dynamic_shapes`, `prefer_deferred_runtime_asserts_over_guards`).

## ExecutorchConfig[[transformers.exporters.ExecutorchConfig]]

#### transformers.exporters.ExecutorchConfig[[transformers.exporters.ExecutorchConfig]]

```python
transformers.exporters.ExecutorchConfig(export_format: ExportFormat = <ExportFormat.EXECUTORCH: 'executorch'>, dynamic: bool = False, strict: bool = False, dynamic_shapes: dict[str, typing.Any] | None = None, prefer_deferred_runtime_asserts_over_guards: bool = False, backend: str = 'xnnpack', alloc_graph_input: bool = True, alloc_graph_output: bool = True, alloc_mutable_buffers: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/configs.py#L153)

**Parameters:**

backend (`str`, *optional*, defaults to `"xnnpack"`) : Target ExecuTorch backend. Supported values:  - `"xnnpack"` — CPU inference via the XNNPACK library (default; runs anywhere). - `"cuda"` — GPU inference via the ExecuTorch CUDA backend.

alloc_graph_input (`bool`, *optional*, defaults to `True`) : Whether the memory-planning pass reserves arena memory for graph inputs. When `False`, the runtime uses the caller-provided input buffers directly instead of copying into the arena — so an in-place `USER_INPUT_MUTATION` (e.g. a `StaticCache` write) lands in the caller's tensor rather than an arena copy.

alloc_graph_output (`bool`, *optional*, defaults to `True`) : Whether the memory-planning pass reserves arena memory for graph outputs. When `False`, the caller must bind output buffers at runtime (`Method::set_output_data_ptr`); binding an output to its mutated input's buffer avoids the copy-out roundtrip.

alloc_mutable_buffers (`bool`, *optional*, defaults to `True`) : Whether the memory-planning pass reserves arena memory for mutable buffers (model-resident state). Passed through to the `MemoryPlanningPass`.

Configuration class for exporting models to ExecuTorch format.

Inherits all fields from `DynamoConfig` (`dynamic`, `strict`,
`dynamic_shapes`, `prefer_deferred_runtime_asserts_over_guards`).

## Utilities[[transformers.exporters.utils.get_leaf_tensors]]

Lower-level functions that power `export_for_generation`, useful when you need to intervene
between decomposing a model and exporting each component.

#### transformers.exporters.utils.get_leaf_tensors[[transformers.exporters.utils.get_leaf_tensors]]

```python
transformers.exporters.utils.get_leaf_tensors(obj: Any)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/utils.py#L277)

**Parameters:**

obj (`Any`) : A tensor, dataclass, dict, list, tuple, or any nesting thereof.

**Returns:** `dict[str, torch.Tensor]`

Flat mapping from dotted path strings to tensors.

Recursively retrieve all leaf tensors from a potentially nested structure.

#### transformers.exporters.utils.prepare_for_export[[transformers.exporters.utils.prepare_for_export]]

```python
transformers.exporters.utils.prepare_for_export(model: PreTrainedModel | torch.nn.Module, inputs: MutableMapping[str, Any])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/utils.py#L344)

Configure model and inputs for export. Mutates both `model` and `inputs` in place,
returning `(model, inputs, output_flags)` where `output_flags` holds the values popped
from `inputs` for `use_cache`, `return_dict`, etc. (to be applied reversibly onto
`model.config` by `patch_model_config` during the trace).

- Strips label inputs (`labels`, `future_values`) — loss computation is unsupported.
- Pops output flags (`use_cache`, `return_dict`, …) from `inputs` so they don't appear
  as traced kwargs; the values are returned for the trace block to apply onto
  `model.config`.
- Pre-computes data-dependent vision/audio kwargs registered via
  `@register_export_input_preparer` and writes them into `inputs`.
- Casts input tensors to match the model's `dtype` / `device`.

#### transformers.exporters.utils.decompose_prefill_decode[[transformers.exporters.utils.decompose_prefill_decode]]

```python
transformers.exporters.utils.decompose_prefill_decode(model: PreTrainedModel, inputs: dict[str, Any], generation_config: Any = None, multi_token_decode: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/utils.py#L715)

**Returns:** `dict[str, tuple[torch.nn.Module, dict]]`

`{"prefill": (model, prefill_inputs), "decode": (model, decode_inputs)}`.

Run `model.generate()` and capture prefill and decode inputs.

Reuses the full generation machinery so every architecture (decoder-only, SSM,
encoder-decoder, multi-modal, …) gets correct inputs without reimplementing the loop.

`generation_config` is forwarded to `generate()` (defaulting to the model's own), so the captured
inputs use whatever cache `generate()` would build. Pass one with `cache_implementation="static"`
and `max_cache_len=N` to capture a **statically sized** cache in the decode inputs — the basis for
a static-cache export. `max_cache_len` sizes the cache independently of the capture, so the
exported decode takes a fixed `[..., N, ...]` cache rather than a growing one.

When `multi_token_decode`, the `decode` component is captured as a **multi-token** decode — two
consecutive decode steps merged (see `_merge_decode_calls`) so its query-sequence axis stays
symbolic (a single-token decode would specialize that axis to 1). It then handles both one token
(ordinary decoding) and many (continuation-from-past, or a plain prefill when the cache is empty). Otherwise `decode` is the
classic single-token decode.

#### transformers.exporters.utils.decompose_multimodal[[transformers.exporters.utils.decompose_multimodal]]

```python
transformers.exporters.utils.decompose_multimodal(model: PreTrainedModel, inputs: dict[str, Any])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/utils.py#L838)

**Returns:** `dict[str, tuple[torch.nn.Module, dict]]`

One `name: (module, inputs)`
entry per detected submodule (image/audio encoder, projector, language model, lm_head).

**Raises:** ``ValueError``

- ``ValueError`` -- if no known multi-modal submodules are found on the model.

Capture inputs to each multi-modal submodule via a single forward pass.

Detects all known multi-modal submodules by attribute name (vision tower, projector,
language model, lm_head, …) and captures their forward kwargs during one
`model(**inputs)` call.

Each submodule is returned as a separate `name: (module, inputs)` entry for
independent export. The token-merge step (e.g. `masked_scatter` for multi-modal models)
is intentionally left outside the exported graphs — it is the caller's responsibility
to assemble `inputs_embeds` from the encoder outputs before running the decoder.

#### transformers.exporters.utils.decompose_for_generation[[transformers.exporters.utils.decompose_for_generation]]

```python
transformers.exporters.utils.decompose_for_generation(model: PreTrainedModel, inputs: dict[str, Any], generation_config: Any = None, multi_token_decode: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/utils.py#L882)

**Parameters:**

model : Generative model. Must support `model.generate(**inputs)`.

inputs : **Generate** kwargs — what you'd pass to `model.generate(**inputs)`.

generation_config : Optional `GenerationConfig` forwarded to `generate()` during capture. Pass one with `cache_implementation="static"` + `max_cache_len=N` to export against a statically sized cache (see `decompose_prefill_decode`).

multi_token_decode : When `True`, capture the `decode` component as a multi-token decode (dynamic query sequence axis: multiple tokens at once — continuation-from-past, or a plain prefill when the cache is empty); a single-token decode can't stay dynamic (see `decompose_prefill_decode`).

**Returns:** `{component_name

(submodel, forward_inputs)}`. Keys are `"prefill"` / `"decode"` for
plain generative models and `"<modality>_encoder"` / `"multi_modal_projector"` /
`"language_model"` / `"lm_head"` / `"decode"` for multi-modal generative models.

Decompose a generative model into independently exportable `(model, forward_inputs)` pairs.

Runs `decompose_prefill_decode` to capture prefill and decode forward kwargs from a real
`model.generate(**inputs, max_new_tokens=2)`. If the prefill is multi-modal (per `is_multimodal`),
further splits it into one entry per submodule (vision/audio encoder, projector, language model,
`lm_head`) via `decompose_multimodal`.

#### transformers.exporters.utils.is_multimodal[[transformers.exporters.utils.is_multimodal]]

```python
transformers.exporters.utils.is_multimodal(model: PreTrainedModel | torch.nn.Module)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/exporters/utils.py#L829)

Returns `True` if the model is multi-modal with modal encoders and a language model.

A non-`PreTrainedModel` (e.g. a bare `nn.Module`) has no canonical `get_encoder`/`get_decoder`
accessors and is trivially not multi-modal, so it short-circuits to `False`.
