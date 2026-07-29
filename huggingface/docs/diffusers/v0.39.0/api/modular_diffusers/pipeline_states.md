# Pipeline states

## PipelineState[[diffusers.modular_pipelines.PipelineState]]

#### diffusers.modular_pipelines.PipelineState[[diffusers.modular_pipelines.PipelineState]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L146)

`PipelineState` stores the state of a pipeline. It is used to pass data between pipeline blocks.

getdiffusers.modular_pipelines.PipelineState.gethttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L171[{"name": "keys", "val": ": str | list[str]"}, {"name": "default", "val": ": typing.Any = None"}]- **keys** (str | list[str]) -- Key or list of keys for the values
- **default** (Any) -- The default value to return if not found0Any | dict[str, Any]Single value if keys is str, dictionary of values if keys is list

Get one or multiple values from the pipeline state.

**Parameters:**

keys (str | list[str]) : Key or list of keys for the values

default (Any) : The default value to return if not found

**Returns:**

`Any | dict[str, Any]`

Single value if keys is str, dictionary of values if keys is list
#### get_by_kwargs[[diffusers.modular_pipelines.PipelineState.get_by_kwargs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L186)

Get all values with matching kwargs_type.

**Parameters:**

kwargs_type (str) : The kwargs_type to filter by

**Returns:**

`dict[str, Any]`

Dictionary of values with matching kwargs_type
#### set[[diffusers.modular_pipelines.PipelineState.set]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L154)

Add a value to the pipeline state.

**Parameters:**

key (str) : The key for the value

value (Any) : The value to store

kwargs_type (str) : The kwargs_type with which the value is associated
#### to_dict[[diffusers.modular_pipelines.PipelineState.to_dict]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L199)

Convert PipelineState to a dictionary.

## BlockState[[diffusers.modular_pipelines.BlockState]]

#### diffusers.modular_pipelines.BlockState[[diffusers.modular_pipelines.BlockState]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L236)

Container for block state data with attribute access and formatted representation.

as_dictdiffusers.modular_pipelines.BlockState.as_dicthttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/modular_pipelines/modular_pipeline.py#L253[]dict[str, Any]Dictionary containing all attributes of the BlockState

Convert BlockState to a dictionary.

**Returns:**

`dict[str, Any]`

Dictionary containing all attributes of the BlockState
