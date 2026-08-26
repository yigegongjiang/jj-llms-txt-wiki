# Pipeline

## ModularPipeline[[diffusers.ModularPipeline]]

#### diffusers.ModularPipeline[[diffusers.ModularPipeline]]

```python
diffusers.ModularPipeline(blocks: diffusers.modular_pipelines.modular_pipeline.ModularPipelineBlocks | None = None, pretrained_model_name_or_path: str | os.PathLike | None = None, components_manager: diffusers.modular_pipelines.components_manager.ComponentsManager | None = None, collection: str | None = None, workflow: str | None = None, modular_config_dict: dict[str, typing.Any] | None = None, config_dict: dict[str, typing.Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline.py#L1621)

**Parameters:**

blocks : ModularPipelineBlocks, the blocks to be used in the pipeline

Base class for all Modular pipelines.

#### from_pretrained[[diffusers.ModularPipeline.from_pretrained]]

```python
from_pretrained(pretrained_model_name_or_path: str | os.PathLike | None, trust_remote_code: bool | None = None, components_manager: diffusers.modular_pipelines.components_manager.ComponentsManager | None = None, collection: str | None = None, workflow: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline.py#L1840)

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`, optional) : Path to a pretrained pipeline configuration. It will first try to load config from `modular_model_index.json`, then fallback to `model_index.json` for compatibility with standard non-modular repositories. If the pretrained_model_name_or_path does not contain any pipeline config, it will be set to None during initialization.

trust_remote_code (`bool`, optional) : Whether to trust remote code when loading the pipeline, need to be set to True if you want to create pipeline blocks based on the custom code in `pretrained_model_name_or_path`

components_manager (`ComponentsManager`, optional) : ComponentsManager instance for managing multiple component cross different pipelines and apply offloading strategies.

collection (`str`, optional) --` Collection name for organizing components in the ComponentsManager.

workflow (`str`, optional) : Name of a workflow declared by the pipeline blocks. If provided, the blocks are pruned to that workflow's execution blocks, so the pipeline only expects — and `load_components()` only loads — the components that workflow uses.

Load a ModularPipeline from a huggingface hub repo.

#### get_component_spec[[diffusers.ModularPipeline.get_component_spec]]

```python
get_component_spec(name: str)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline.py#L2301)

**Returns:**

- a copy of the ComponentSpec object for the given component name

#### load_components[[diffusers.ModularPipeline.load_components]]

```python
load_components(names: list[str] | str | None = None, workflow: str | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline.py#L2389)

**Parameters:**

names : list of component names to load. If None, will load all components with default_creation_method == "from_pretrained". If provided as a list or string, will load only the specified components.

workflow : name of a workflow declared by the pipeline blocks. If provided, only the components that workflow's execution blocks use are loaded. Cannot be combined with `names`.

- ****kwargs** : additional kwargs to be passed to `from_pretrained()`.Can be: - a single value to be applied to all components to be loaded, e.g. dtype=torch.bfloat16 - a dict, e.g. dtype={"unet": torch.bfloat16, "default": torch.float32} - if potentially override ComponentSpec if passed a different loading field in kwargs, e.g. `pretrained_model_name_or_path`, `variant`, `revision`, etc. - if potentially override ComponentSpec if passed a different loading field in kwargs, e.g. `pretrained_model_name_or_path`, `variant`, `revision`, etc.

Load selected components from specs.

#### register_components[[diffusers.ModularPipeline.register_components]]

```python
register_components(**kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline.py#L2086)

**Parameters:**

- ****kwargs** : Keyword arguments where keys are component names and values are component objects. E.g., register_components(unet=unet_model, text_encoder=encoder_model)

Register components with their corresponding specifications.

This method is responsible for:
1. Sets component objects as attributes on the loader (e.g., self.unet = unet)
2. Updates the config dict, which will be saved as `modular_model_index.json` during `save_pretrained` (only
   for from_pretrained components)
3. Adds components to the component manager if one is attached (only for from_pretrained components)

This method is called when:
- Components are first initialized in __init__:
  - from_pretrained components not loaded during __init__ so they are registered as None;
  - non from_pretrained components are created during __init__ and registered as the object itself
- Components are updated with the `update_components()` method: e.g. loader.update_components(unet=unet) or
  loader.update_components(guider=guider_spec)
- (from_pretrained) Components are loaded with the `load_components()` method: e.g.
  loader.load_components(names=["unet"]) or loader.load_components() to load all default components

Notes:
- When registering None for a component, it sets attribute to None but still syncs specs with the config
  dict, which will be saved as `modular_model_index.json` during `save_pretrained`
- component_specs are updated to match the new component outside of this method, e.g. in
  `update_components()` method

#### save_pretrained[[diffusers.ModularPipeline.save_pretrained]]

```python
save_pretrained(save_directory: str | os.PathLike, safe_serialization: bool = True, variant: str | None = None, max_shard_size: int | str | None = None, push_to_hub: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline.py#L1932)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory to save the pipeline to. Will be created if it doesn't exist.

safe_serialization (`bool`, *optional*, defaults to `True`) : Whether to save the model using `safetensors` or the traditional PyTorch way with `pickle`.

variant (`str`, *optional*) : If specified, weights are saved in the format `pytorch_model.<variant>.bin`.

max_shard_size (`int` or `str`, defaults to `None`) : The maximum size for a checkpoint before being sharded. Checkpoints shard will then be each of size lower than this size. If expressed as a string, needs to be digits followed by a unit (like `"5GB"`). If expressed as an integer, the unit is bytes.

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether to push the pipeline to the Hugging Face model hub after saving it.

- ****kwargs** : Additional keyword arguments: - `overwrite_modular_index` (`bool`, *optional*, defaults to `False`): When saving a Modular Pipeline, its components in `modular_model_index.json` may reference repos different from the destination repo. Setting this to `True` updates all component references in `modular_model_index.json` so they point to the repo specified by `repo_id`. - `repo_id` (`str`, *optional*): The repository ID to push the pipeline to. Defaults to the last component of `save_directory`. - `commit_message` (`str`, *optional*): Commit message for the push to hub operation. - `private` (`bool`, *optional*): Whether the repository should be private. - `create_pr` (`bool`, *optional*, defaults to `False`): Whether to create a pull request instead of pushing directly. - `token` (`str`, *optional*): The Hugging Face token to use for authentication.

Save the pipeline and all its components to a directory, so that it can be re-loaded using the
[from_pretrained()](/docs/diffusers/v0.40.0/en/api/modular_diffusers/pipeline#diffusers.ModularPipeline.from_pretrained) class method.

#### to[[diffusers.ModularPipeline.to]]

```python
to(*args, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline.py#L2551)

**Parameters:**

dtype (`torch.dtype`, *optional*) : Returns a pipeline with the specified [`dtype`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.dtype)

device (`torch.Device`, *optional*) : Returns a pipeline with the specified [`device`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.device)

silence_dtype_warnings (`str`, *optional*, defaults to `False`) : Whether to omit warnings if the target `dtype` is not compatible with the target `device`.

**Returns:** [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline)

The pipeline converted to specified `dtype` and/or `dtype`.

Performs Pipeline dtype and/or device conversion. A torch.dtype and torch.device are inferred from the
arguments of `self.to(*args, **kwargs).`

> [!TIP] > If the pipeline already has the correct torch.dtype and torch.device, then it is returned as is.
Otherwise, > the returned pipeline is a copy of self with the desired torch.dtype and torch.device.

Here are the ways to call `to`:

- `to(dtype, silence_dtype_warnings=False) → DiffusionPipeline` to return a pipeline with the specified
  [`dtype`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.dtype)
- `to(device, silence_dtype_warnings=False) → DiffusionPipeline` to return a pipeline with the specified
  [`device`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.device)
- `to(device=None, dtype=None, silence_dtype_warnings=False) → DiffusionPipeline` to return a pipeline with the
  specified [`device`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.device) and
  [`dtype`](https://pytorch.org/docs/stable/tensor_attributes.html#torch.dtype)

#### unload_components[[diffusers.ModularPipeline.unload_components]]

```python
unload_components(names: list[str] | str)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline.py#L2495)

**Parameters:**

names : component name or list of component names to unload.

Unload selected components, freeing their memory.

The component attribute is set back to `None` and, if a ComponentsManager is attached, the component is removed
from it. The component spec is untouched, so the component can be loaded again later with `load_components()`.

#### update_components[[diffusers.ModularPipeline.update_components]]

```python
update_components(**kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline.py#L2308)

**Parameters:**

- ****kwargs** : Component objects or configuration values to update: - Component objects: Models loaded with `AutoModel.from_pretrained()` or `ComponentSpec.load()` are automatically tagged with loading information. ConfigMixin objects without weights (e.g., schedulers, guiders) can be passed directly. - Configuration values: Simple values to update configuration settings (e.g., `requires_safety_checker=False`)

Update components and configuration values and specs after the pipeline has been instantiated.

This method allows you to:
1. Replace existing components with new ones (e.g., updating `self.unet` or `self.text_encoder`)
2. Update configuration values (e.g., changing `self.requires_safety_checker` flag)

In addition to updating the components and configuration values as pipeline attributes, the method also
updates:
- the corresponding specs in `_component_specs` and `_config_specs`
- the `config` dict, which will be saved as `modular_model_index.json` during `save_pretrained`

Examples:
```python
# Update pre-trained model
pipeline.update_components(unet=new_unet_model, text_encoder=new_text_encoder)

# Update configuration values
pipeline.update_components(requires_safety_checker=False)
```

Notes:
- Components loaded with `AutoModel.from_pretrained()` or `ComponentSpec.load()` will have
loading specs preserved for serialization. Custom or locally loaded components without Hub references will
have their `modular_model_index.json` entries updated automatically during `save_pretrained()`.
- ConfigMixin objects without weights (e.g., schedulers, guiders) can be passed directly.
