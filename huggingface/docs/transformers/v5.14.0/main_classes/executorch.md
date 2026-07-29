# ExecuTorch

[`ExecuTorch`](https://github.com/pytorch/executorch) is an end-to-end solution for enabling on-device inference capabilities across mobile and edge devices including wearables, embedded devices and microcontrollers. It is part of the PyTorch ecosystem and supports the deployment of PyTorch models with a focus on portability, productivity, and performance.

ExecuTorch introduces well defined entry points to perform model, device, and/or use-case specific optimizations such as backend delegation, user-defined compiler transformations, memory planning, and more. The first step in preparing a PyTorch model for execution on an edge device using ExecuTorch is to export the model. This is achieved through the use of a PyTorch API called [`torch.export`](https://pytorch.org/docs/stable/export.html).

## ExecuTorch Integration[[transformers.TorchExportableModuleWithStaticCache]]

An integration point is being developed to ensure that 🤗 Transformers can be exported using `torch.export`. The goal of this integration is not only to enable export but also to ensure that the exported artifact can be further lowered and optimized to run efficiently in `ExecuTorch`, particularly for mobile and edge use cases.

A recipe module designed to make a `PreTrainedModel` exportable with `torch.export`,
specifically for decoder-only LM to `StaticCache`. This module ensures that the
exported model is compatible with further lowering and execution in `ExecuTorch`.

Note:
This class is specifically designed to support export process using `torch.export`
in a way that ensures the model can be further lowered and run efficiently in `ExecuTorch`.

- **input_ids** (`torch.Tensor`) -- Tensor representing current input token id to the module.
- **inputs_embeds** (`torch.Tensor`) -- Tensor representing current input embeddings to the module.
- **cache_position** (`torch.Tensor`) -- Tensor representing current input position in the cache.torch.TensorLogits output from the model.

Forward pass of the module, which is compatible with the ExecuTorch runtime.

This forward adapter serves two primary purposes:

1. **Making the Model `torch.export`-Compatible**:
   The adapter hides unsupported objects, such as the `Cache`, from the graph inputs and outputs,
   enabling the model to be exportable using `torch.export` without encountering issues.

2. **Ensuring Compatibility with `ExecuTorch` runtime**:
   The adapter matches the model's forward signature with that in `executorch/extension/llm/runner`,
   ensuring that the exported model can be executed in `ExecuTorch` out-of-the-box.

- **model** (`PreTrainedModel`) -- The pretrained model to be exported.
- **example_input_ids** (`Optional[torch.Tensor]`) -- Example input token id used by `torch.export`.
- **example_cache_position** (`Optional[torch.Tensor]`) -- Example current cache position used by `torch.export`.
- **dynamic_shapes(`Optional[dict]`)** -- Dynamic shapes used by `torch.export`.
- **strict(`Optional[bool]`)** -- Flag to instruct `torch.export` to use `dynamo`.Exported program (`torch.export.ExportedProgram`)The exported program generated via `torch.export`.

Convert a `PreTrainedModel` into an exportable module and export it using `torch.export`,
ensuring the exported model is compatible with `ExecuTorch`.
