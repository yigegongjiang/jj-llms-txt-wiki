# Components and configs

## ComponentSpec[[diffusers.ComponentSpec]]

#### diffusers.ComponentSpec[[diffusers.ComponentSpec]]

```python
diffusers.ComponentSpec(name: str | None = None, type_hint: typing.Optional[typing.Type] = None, description: str | None = None, config: diffusers.configuration_utils.FrozenDict | None = None, pretrained_model_name_or_path: str | list[str] | None = None, subfolder: str | None = '', variant: str | None = None, revision: str | None = None, default_creation_method: typing.Literal['from_config', 'from_pretrained'] = 'from_pretrained', repo: str | list[str] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline_utils.py#L98)

**Parameters:**

name : Name of the component

type_hint : Type of the component (e.g. UNet2DConditionModel)

description : Optional description of the component

config : Optional config dict for __init__ creation

pretrained_model_name_or_path : Optional pretrained_model_name_or_path path for from_pretrained creation

subfolder : Optional subfolder in pretrained_model_name_or_path

variant : Optional variant in pretrained_model_name_or_path

revision : Optional revision in pretrained_model_name_or_path

default_creation_method : Preferred creation method - "from_config" or "from_pretrained"

Specification for a pipeline component.

A component can be created in two ways:
1. From scratch using __init__ with a config dict
2. using `from_pretrained`

#### create[[diffusers.ComponentSpec.create]]

```python
create(config: diffusers.configuration_utils.FrozenDict | dict[str, typing.Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline_utils.py#L267)

Create component using from_config with config.

#### decode_load_id[[diffusers.ComponentSpec.decode_load_id]]

```python
decode_load_id(load_id: str)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline_utils.py#L228)

**Parameters:**

load_id : The load_id string to decode, format: "pretrained_model_name_or_path|subfolder|variant|revision" where None values are represented as "null"

**Returns:**

Dict mapping loading field names to their values. e.g. {
"pretrained_model_name_or_path": "path/to/repo", "subfolder": "subfolder", "variant": "variant",
"revision": "revision"
} If a segment value is "null", it's replaced with None. Returns None if load_id is "null" (indicating
component not created with `load` method).

Decode a load_id string back into a dictionary of loading fields and values.

#### from_component[[diffusers.ComponentSpec.from_component]]

```python
from_component(name: str, component: typing.Any)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline_utils.py#L149)

**Parameters:**

name : Name of the component

component : Component object to create spec from

**Returns:**

ComponentSpec object

**Raises:** ``ValueError``

- ``ValueError`` -- If component is not supported (e.g. nn.Module without load_id, non-ConfigMixin)

Create a ComponentSpec from a Component.

Currently supports:
- Components created with `ComponentSpec.load()` method
- Components that are ConfigMixin subclasses but not nn.Modules (e.g. schedulers, guiders)

#### load[[diffusers.ComponentSpec.load]]

```python
load(**kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline_utils.py#L295)

Load component using from_pretrained.

#### loading_fields[[diffusers.ComponentSpec.loading_fields]]

```python
loading_fields()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline_utils.py#L209)

Return the names of all loading‐related fields (i.e. those whose field.metadata["loading"] is True).

## ConfigSpec[[diffusers.ConfigSpec]]

#### diffusers.ConfigSpec[[diffusers.ConfigSpec]]

```python
diffusers.ConfigSpec(name: str, default: typing.Any, description: str | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline_utils.py#L360)

Specification for a pipeline configuration parameter.

## ComponentsManager[[diffusers.ComponentsManager]]

#### diffusers.ComponentsManager[[diffusers.ComponentsManager]]

```python
diffusers.ComponentsManager()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L302)

A central registry and management system for model components across multiple pipelines.

[ComponentsManager](/docs/diffusers/v0.40.0/en/api/modular_diffusers/pipeline_components#diffusers.ComponentsManager) provides a unified way to register, track, and reuse model components (like UNet, VAE, text
encoders, etc.) across different modular pipelines. It includes features for duplicate detection, memory
management, and component organization.

> [!WARNING] > This is an experimental feature and is likely to change in the future.

Example:
```python
from diffusers import ComponentsManager

# Create a components manager
cm = ComponentsManager()

# Add components
cm.add("unet", unet_model, collection="sdxl")
cm.add("vae", vae_model, collection="sdxl")

# Enable auto offloading
cm.enable_auto_cpu_offload()

# Retrieve components
unet = cm.get_one(name="unet", collection="sdxl")
```

#### add[[diffusers.ComponentsManager.add]]

```python
add(name: str, component: Any, collection: str | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L399)

**Parameters:**

name (str) : The name of the component

component (Any) : The component to add

collection (str | None) : The collection to add the component to

**Returns:** `str`

The unique component ID, which is generated as "{name}_{id(component)}" where
id(component) is Python's built-in unique identifier for the object

Add a component to the ComponentsManager.

#### disable_auto_cpu_offload[[diffusers.ComponentsManager.disable_auto_cpu_offload]]

```python
disable_auto_cpu_offload()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L806)

Disable automatic CPU offloading for all components.

#### enable_auto_cpu_offload[[diffusers.ComponentsManager.enable_auto_cpu_offload]]

```python
enable_auto_cpu_offload(device: str | int | torch.device = None, memory_reserve_margin = '3GB', offload_strategy = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L708)

**Parameters:**

device (str | int | torch.device) : The execution device where models are moved for forward passes

memory_reserve_margin (str) : The memory reserve margin to use, default is 3GB. This is the amount of memory to keep free on the device to avoid running out of memory during model execution (e.g., for intermediate activations, gradients, etc.)

offload_strategy : Any callable with the signature `(hooks, model_id, model, execution_device) -> hooks`, returning which resident models to offload before the incoming one loads. Defaults to `AutoOffloadStrategy`, which frees the smallest sufficient combination.

Enable automatic CPU offloading for all components.

The algorithm works as follows:
1. All models start on CPU by default
2. When a model's forward pass is called, it's moved to the execution device
3. If there's insufficient memory, other models on the device are moved back to CPU
4. The system tries to offload the smallest combination of models that frees enough memory
5. Models stay on the execution device until another model needs memory and forces them off

A group offloaded model takes part in this but places itself: it can still make room by moving other models
aside, and is never moved to make room for them. Either order works — group offload before or after enabling
this. `AutoOffloadStrategy` sizes its decisions from model memory footprints, which do not describe a model
holding one group at a time, so pass an `offload_strategy` that decides from the workflow instead.

#### get_components_by_ids[[diffusers.ComponentsManager.get_components_by_ids]]

```python
get_components_by_ids(ids: list[str], return_dict_with_names: bool | None = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L1115)

**Parameters:**

ids (list[str]) : list of component IDs

return_dict_with_names (bool | None) : Whether to return a dictionary with component names as keys:

**Returns:** dict[str, Any]

Dictionary of components.
- If return_dict_with_names=True, keys are component names.
- If return_dict_with_names=False, keys are component IDs.

**Raises:** ``ValueError``

- ``ValueError`` -- If duplicate component names are found in the search results when return_dict_with_names=True

Get components by a list of IDs.

#### get_components_by_names[[diffusers.ComponentsManager.get_components_by_names]]

```python
get_components_by_names(names: list[str], collection: str | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L1148)

**Parameters:**

names (list[str]) : list of component names

collection (str | None) : Optional collection to filter by

**Returns:** dict[str, Any]

Dictionary of components with component names as keys

**Raises:** ``ValueError``

- ``ValueError`` -- If duplicate component names are found in the search results

Get components by a list of names, optionally filtered by collection.

#### get_ids[[diffusers.ComponentsManager.get_ids]]

```python
get_ids(names: str | list[str] = None, collection: str | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L1097)

**Parameters:**

names (str | list[str]) : list of component names

collection (str | None) : Optional collection to filter by

**Returns:** list[str]

list of component IDs

Get component IDs by a list of names, optionally filtered by collection.

#### get_model_info[[diffusers.ComponentsManager.get_model_info]]

```python
get_model_info(component_id: str, fields: str | list[str] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L823)

**Parameters:**

component_id (str) : Name of the component to get info for

fields (str | list[str] | None) : Field(s) to return. Can be a string for single field or list of fields. If None, uses the available_info_fields setting.

**Returns:**

Dictionary containing requested component metadata. If fields is specified, returns only those fields.
Otherwise, returns all fields.

Get comprehensive information about a component.

#### get_one[[diffusers.ComponentsManager.get_one]]

```python
get_one(component_id: str | None = None, name: str | None = None, collection: str | None = None, load_id: str | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L1052)

**Parameters:**

component_id (str | None) : Optional component ID to get

name (str | None) : Component name or pattern

collection (str | None) : Optional collection to filter by

load_id (str | None) : Optional load_id to filter by

**Returns:**

A single component

**Raises:** ``ValueError``

- ``ValueError`` -- If no components match or multiple components match

Get a single component by either:
- searching name (pattern matching), collection, or load_id.
- passing in a component_id
Raises an error if multiple components match or none are found.

#### remove[[diffusers.ComponentsManager.remove]]

```python
remove(component_id: str = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L489)

**Parameters:**

component_id (str) : The ID of the component to remove

Remove a component from the ComponentsManager.

#### remove_from_collection[[diffusers.ComponentsManager.remove_from_collection]]

```python
remove_from_collection(component_id: str, collection: str)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L471)

Remove a component from a collection.

#### search_components[[diffusers.ComponentsManager.search_components]]

```python
search_components(names: str | None = None, collection: str | None = None, load_id: str | None = None, return_dict_with_names: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L522)

**Parameters:**

names : Component name(s) or pattern(s) Patterns: - "unet" : match any component with base name "unet" (e.g., unet_123abc) - "!unet" : everything except components with base name "unet" - "unet*" : anything with base name starting with "unet" - "!unet*" : anything with base name NOT starting with "unet" - "*unet*" : anything with base name containing "unet" - "!*unet*" : anything with base name NOT containing "unet" - "refiner|vae|unet" : anything with base name exactly matching "refiner", "vae", or "unet" - "!refiner|vae|unet" : anything with base name NOT exactly matching "refiner", "vae", or "unet" - "unet*|vae*" : anything with base name starting with "unet" OR starting with "vae"

collection : Optional collection to filter by

load_id : Optional load_id to filter by

return_dict_with_names : If True, returns a dictionary with component names as keys, throw an error if multiple components with the same name are found If False, returns a dictionary with component IDs as keys

**Returns:**

Dictionary mapping component names to components if return_dict_with_names=True, or a dictionary mapping
component IDs to components if return_dict_with_names=False

Search components by name with simple pattern matching. Optionally filter by collection or load_id.

#### set_offload_strategy[[diffusers.ComponentsManager.set_offload_strategy]]

```python
set_offload_strategy(offload_strategy)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/components_manager.py#L790)

**Parameters:**

offload_strategy : Any callable with the signature `(hooks, model_id, model, execution_device) -> hooks`: it receives the hooks of the models currently on the device and returns the ones to offload before the incoming model loads. The default is `AutoOffloadStrategy`, which frees the smallest sufficient combination.

Replace the offload strategy on all managed models. Only valid while auto CPU offloading is enabled.

## InsertableDict[[diffusers.modular_pipelines.InsertableDict]]

#### diffusers.modular_pipelines.InsertableDict[[diffusers.modular_pipelines.InsertableDict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/modular_pipeline_utils.py#L60)
