## Kernels

This page documents the kernels configuration utilities.

### kernelize[[transformers.kernelize]]

#### transformers.kernelize[[transformers.kernelize]]

```python
transformers.kernelize(model: PreTrainedModel, mode: Mode | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/integrations/hub_kernels.py#L724)

Temporarily register hidden kernel wrappers so `kernelize` can discover and replace them.

### KernelConfig[[transformers.KernelConfig]]

#### transformers.KernelConfig[[transformers.KernelConfig]]

```python
transformers.KernelConfig(kernel_mapping = None, use_local_kernel = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/kernel_config.py#L99)

Kernel configuration class. This class is used to configure the kernel mapping for a model.

#### create_compatible_mapping[[transformers.KernelConfig.create_compatible_mapping]]

```python
create_compatible_mapping(model, compile = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/kernel_config.py#L248)

Transforms a simple kernel_mapping of the form:
{
"RMSNorm":
("kernels-community/layer_norm:LlamaRMSNorm", {"version": 1, "trust_remote_code": True}),
...
},

or for local path:

{
"RMSNorm":
"/home/user/liger_kernels:LigerRMSNorm",
...
},

into a nested mapping:

{
"RMSNorm": {
"cuda": {
Mode.INFERENCE: LayerRepository(
repo_id="kernels-community/layer_norm",
layer_name="LlamaRMSNorm",
version=1,
trust_remote_code=True,
)
}
}
}

or for local path:

{
"RMSNorm": {
"cuda": {
Mode.INFERENCE: LocalLayerRepository(
repo_path=Path("/home/user/liger_kernels"),
layer_name="LigerRMSNorm",
)
}
}
}

that's compatible with the kernels library.

The device is inferred from the model's parameters if not provided.
The Mode is inferred from the model's training state.

#### sanitize_kernel_mapping[[transformers.KernelConfig.sanitize_kernel_mapping]]

```python
sanitize_kernel_mapping(model)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/utils/kernel_config.py#L131)

**Parameters:**

model : The model instance whose modules are checked for registered kernel_layer_name attributes.

**Raises:** ``ValueError``

- ``ValueError`` -- If a layer_name is not registered in the model, if a device is not supported,
  or if a repo_name is not a valid 'org/repo:layer_name' string.

Validates the kernel_mapping to ensure that:
1. Each layer_name in the mapping is registered in the model (i.e., the model contains a module with a matching kernel_layer_name).
2. Each kernel value is
   - either a string of the form 'org/repo:layer_name' or a tuple with the same as string and a dict of {"revision"/"version/trust_remote_code": ...},
   - or a dict mapping device types ("cuda", "rocm", "xpu", "npu") to such values as above.
3. Each device key in a dict is one of "cuda", "rocm", "xpu", or "npu".
5. Each trust remote code key must be a bool.
6. Each revision or version key must exist mutually exclusive if it has been passed explicitly.
7. Each repo_name is a valid repository and layer name in the format 'org/repo:layer_name' (i.e., a string containing both a slash and a colon).
8. If a local path is detected, it should be in the format '/abs/path:layer_name', where the absolute path points to the kernel repository, like "/home/user/layer_norm".
