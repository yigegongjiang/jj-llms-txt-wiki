# Exporters

New export backends can be added to Transformers by subclassing `HfExporter`.

Learn how to use the built-in exporters in the [Exporters](../exporters) guide.

## AutoHfExporter[[transformers.exporters.AutoHfExporter]]

The Auto-HF expoerter class that takes care of automatically instantiating to the correct
`HfExporter` given the `ExportConfig`.

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

Return True if the provided dict describes an `export_format` that has both a
registered config class and a registered exporter class. Warns with an actionable message
when the format is missing entirely, unknown, or only half-registered.

## AutoExportConfig[[transformers.exporters.AutoExportConfig]]

The Auto-HF export config class that takes care of automatically dispatching to the correct
export config given an export config stored in a dictionary.

## HfExporter[[transformers.exporters.HfExporter]]

Abstract base class for all Transformers exporters.

Subclass and implement `~HfExporter.export` to add a new export backend.

- **model** ([PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel)) --
  The model to export.
- **sample_inputs** (`dict[str, torch.Tensor | Cache]`) --
  **Forward** kwargs — what you'd pass to `model(**sample_inputs)`. These are used
  directly as the example inputs during tracing. For an autoregressive decode-step
  export, this means you need to include `past_key_values`, `cache_position`, etc.
  If you only have generation-style inputs, use `~HfExporter.export_for_generation`
  instead — it runs `model.generate` for you and exports each stage.
- **config** (`ExportConfigMixin`) --
  Backend-specific configuration.Backend-specific export artifact.

Export the model and return the backend-specific program object.

- **model** ([PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel)) --
  The generative model to export. Must support `model.generate(**sample_inputs)`.
- **sample_inputs** (`dict[str, torch.Tensor | Cache]`) --
  **Generate** kwargs — what you'd pass to `model.generate(**sample_inputs)`
  (typically `input_ids` + `attention_mask`, plus any modality inputs like
  `pixel_values` / `input_features` for multi-modal models). Per-stage forward
  kwargs are captured internally.
- **config** (`ExportConfigMixin` or `dict[str, ExportConfigMixin]`) --
  Backend-specific configuration. Pass a single config to apply to every
  component, or a `dict` keyed by component name (e.g. `"image_encoder"`,
  `"language_model"`, `"lm_head"`, `"decode"`) to override per-component —
  all component names must be present in the dict.`dict[str, Any]``{component_name: backend_specific_artifact}` — same keys as
[decompose_for_generation()](/docs/transformers/v5.14.0/en/exporters#transformers.exporters.utils.decompose_for_generation). Values are whatever
`~HfExporter.export` returns for the concrete backend (`ExportedProgram`,
`ONNXProgram`, `ExecutorchProgramManager`).

Decompose a generative model and export each component independently.

Thin wrapper around [decompose_for_generation()](/docs/transformers/v5.14.0/en/exporters#transformers.exporters.utils.decompose_for_generation) that calls
`~HfExporter.export` on every returned `(submodel, forward_inputs)` pair. If you need
the intermediate `(submodel, forward_inputs)` pairs (for verification, custom inputs,
skipping a stage, …), call [decompose_for_generation()](/docs/transformers/v5.14.0/en/exporters#transformers.exporters.utils.decompose_for_generation) directly.

Check `required_packages` are installed and warn on version drift from `tested_versions`.

## DynamoConfig[[transformers.exporters.DynamoConfig]]

"}, {"name": "dynamic", "val": ": bool = False"}, {"name": "strict", "val": ": bool = False"}, {"name": "dynamic_shapes", "val": ": dict[str, typing.Any] | None = None"}, {"name": "prefer_deferred_runtime_asserts_over_guards", "val": ": bool = False"}]}>
- **dynamic** (*bool*, *optional*, defaults to *False*) --
  Whether to export with dynamic (symbolic) shapes. When *True* and
  *dynamic_shapes* is not set, all tensor dimensions are set to
  *Dim.AUTO* automatically.
- **strict** (*bool*, *optional*, defaults to *False*) --
  Whether to enable strict mode in *torch.export*. Runs the full
  symbolic trace and catches more errors, but is slower and more
  likely to fail on complex models.
- **dynamic_shapes** (*dict[str, Any]*, *optional*) --
  Explicit per-input dynamic shape specifications passed to
  *torch.export*. Takes precedence over *dynamic*.
- **prefer_deferred_runtime_asserts_over_guards** (*bool*, *optional*, defaults to *False*) --
  When *True*, data-dependent shape guards are emitted as runtime asserts in the exported
  graph instead of failing the export at trace time when a guard wouldn't hold across the
  full symbolic shape range. Most transformer LLMs need this set to *True* when using
  fine-grained `Dim(min=, max=)` bounds. Not needed with `dynamic=True` / `Dim.AUTO`,
  where `torch.export` infers shape relations instead of verifying them against the
  user-stated bounds.

Configuration class for exporting models via *torch.export*.

## OnnxConfig[[transformers.exporters.OnnxConfig]]

"}, {"name": "dynamic", "val": ": bool = False"}, {"name": "strict", "val": ": bool = False"}, {"name": "dynamic_shapes", "val": ": dict[str, typing.Any] | None = None"}, {"name": "prefer_deferred_runtime_asserts_over_guards", "val": ": bool = False"}, {"name": "output_path", "val": ": str | os.PathLike | None = None"}, {"name": "opset_version", "val": ": int | None = None"}, {"name": "external_data", "val": ": bool = True"}, {"name": "optimize", "val": ": bool = True"}, {"name": "export_params", "val": ": bool = True"}, {"name": "keep_initializers_as_inputs", "val": ": bool = False"}]}>
- **output_path** (`str` or `PathLike`, *optional*) --
  Output path for the `.onnx` file. When `None` (default) the
  exported model is kept in memory as an `ONNXProgram` and not
  written to disk.
- **opset_version** (`int`, *optional*) --
  ONNX opset version to target. Defaults to the latest opset
  supported by the installed `onnxscript` version.
- **external_data** (`bool`, *optional*, defaults to `True`) --
  Store large weight tensors in a separate `.onnx_data` sidecar
  file instead of embedding them in the protobuf. Required for
  models whose weights exceed the 2 GB protobuf limit.
- **optimize** (`bool`, *optional*, defaults to `True`) --
  Run `onnxscript` optimisation passes (constant folding, dead-code
  elimination, …) on the exported graph. Disable for models that
  hit upstream `onnxscript` optimiser bugs.
- **export_params** (`bool`, *optional*, defaults to `True`) --
  Embed model weights in the ONNX graph. Set to `False` to export
  a weight-free graph (weights must be supplied at runtime).
- **keep_initializers_as_inputs** (`bool`, *optional*, defaults to `False`) --
  Expose weight initializers as explicit graph inputs. Required by
  some older ONNX runtimes (opset < 9).

Configuration class for exporting models to ONNX via `torch.onnx.export`.

Inherits all fields from `DynamoConfig` (`dynamic`, `strict`,
`dynamic_shapes`, `prefer_deferred_runtime_asserts_over_guards`).

## ExecutorchConfig[[transformers.exporters.ExecutorchConfig]]

"}, {"name": "dynamic", "val": ": bool = False"}, {"name": "strict", "val": ": bool = False"}, {"name": "dynamic_shapes", "val": ": dict[str, typing.Any] | None = None"}, {"name": "prefer_deferred_runtime_asserts_over_guards", "val": ": bool = False"}, {"name": "backend", "val": ": str = 'xnnpack'"}]}>
- **backend** (`str`, *optional*, defaults to `"xnnpack"`) --
  Target ExecuTorch backend. Supported values:

  - `"xnnpack"` — CPU inference via the XNNPACK library (default; runs anywhere).
  - `"cuda"` — GPU inference via the ExecuTorch CUDA backend.

Configuration class for exporting models to ExecuTorch format.

Inherits all fields from `DynamoConfig` (`dynamic`, `strict`,
`dynamic_shapes`, `prefer_deferred_runtime_asserts_over_guards`).
