# Utilities for pipelines

This page lists all the utility functions the library provides for pipelines.

Most of those are only useful if you are studying the code of the models in the library.

## Argument handling[[transformers.pipelines.ArgumentHandler]]

#### transformers.pipelines.ArgumentHandler[[transformers.pipelines.ArgumentHandler]]

```python
transformers.pipelines.ArgumentHandler()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L383)

Base interface for handling arguments for each [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline).

#### transformers.pipelines.ZeroShotClassificationArgumentHandler[[transformers.pipelines.ZeroShotClassificationArgumentHandler]]

```python
transformers.pipelines.ZeroShotClassificationArgumentHandler()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/zero_shot_classification.py#L13)

Handles arguments for zero-shot for text classification by turning each possible label into an NLI
premise/hypothesis pair.

## Data format[[transformers.PipelineDataFormat]]

#### transformers.PipelineDataFormat[[transformers.PipelineDataFormat]]

```python
transformers.PipelineDataFormat(output_path: str | None, input_path: str | None, column: str | None, overwrite: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L393)

**Parameters:**

output_path (`str`) : Where to save the outgoing data.

input_path (`str`) : Where to look for the input data.

column (`str`) : The column to read.

overwrite (`bool`, *optional*, defaults to `False`) : Whether or not to overwrite the `output_path`.

Base class for all the pipeline supported data format both for reading and writing. Supported data formats
currently includes:

- JSON
- CSV
- stdin/stdout (pipe)

`PipelineDataFormat` also includes some utilities to work with multi-columns like mapping from datasets columns to
pipelines keyword arguments through the `dataset_kwarg_1=dataset_column_1` format.

#### from_str[[transformers.PipelineDataFormat.from_str]]

```python
from_str(format: str, output_path: str | None, input_path: str | None, column: str | None, overwrite = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L470)

**Parameters:**

format (`str`) : The format of the desired pipeline. Acceptable values are `"json"`, `"csv"` or `"pipe"`.

output_path (`str`, *optional*) : Where to save the outgoing data.

input_path (`str`, *optional*) : Where to look for the input data.

column (`str`, *optional*) : The column to read.

overwrite (`bool`, *optional*, defaults to `False`) : Whether or not to overwrite the `output_path`.

**Returns:** [PipelineDataFormat](/docs/transformers/v5.15.1/en/internal/pipelines_utils#transformers.PipelineDataFormat)

The proper data format.

Creates an instance of the right subclass of [PipelineDataFormat](/docs/transformers/v5.15.1/en/internal/pipelines_utils#transformers.PipelineDataFormat) depending on `format`.

#### save[[transformers.PipelineDataFormat.save]]

```python
save(data: dict | list[dict])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L442)

**Parameters:**

data (`dict` or list of `dict`) : The data to store.

Save the provided data object with the representation for the current [PipelineDataFormat](/docs/transformers/v5.15.1/en/internal/pipelines_utils#transformers.PipelineDataFormat).

#### save_binary[[transformers.PipelineDataFormat.save_binary]]

```python
save_binary(data: dict | list[dict])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L452)

**Parameters:**

data (`dict` or list of `dict`) : The data to store.

**Returns:** `str`

Path where the data has been saved.

Save the provided data object as a pickle-formatted binary data on the disk.

#### transformers.CsvPipelineDataFormat[[transformers.CsvPipelineDataFormat]]

```python
transformers.CsvPipelineDataFormat(output_path: str | None, input_path: str | None, column: str | None, overwrite = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L506)

**Parameters:**

output_path (`str`) : Where to save the outgoing data.

input_path (`str`) : Where to look for the input data.

column (`str`) : The column to read.

overwrite (`bool`, *optional*, defaults to `False`) : Whether or not to overwrite the `output_path`.

Support for pipelines using CSV data format.

#### save[[transformers.CsvPipelineDataFormat.save]]

```python
save(data: list[dict])
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L536)

**Parameters:**

data (`list[dict]`) : The data to store.

Save the provided data object with the representation for the current [PipelineDataFormat](/docs/transformers/v5.15.1/en/internal/pipelines_utils#transformers.PipelineDataFormat).

#### transformers.JsonPipelineDataFormat[[transformers.JsonPipelineDataFormat]]

```python
transformers.JsonPipelineDataFormat(output_path: str | None, input_path: str | None, column: str | None, overwrite = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L550)

**Parameters:**

output_path (`str`) : Where to save the outgoing data.

input_path (`str`) : Where to look for the input data.

column (`str`) : The column to read.

overwrite (`bool`, *optional*, defaults to `False`) : Whether or not to overwrite the `output_path`.

Support for pipelines using JSON file format.

#### save[[transformers.JsonPipelineDataFormat.save]]

```python
save(data: dict)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L581)

**Parameters:**

data (`dict`) : The data to store.

Save the provided data object in a json file.

#### transformers.PipedPipelineDataFormat[[transformers.PipedPipelineDataFormat]]

```python
transformers.PipedPipelineDataFormat(output_path: str | None, input_path: str | None, column: str | None, overwrite: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L592)

**Parameters:**

output_path (`str`) : Where to save the outgoing data.

input_path (`str`) : Where to look for the input data.

column (`str`) : The column to read.

overwrite (`bool`, *optional*, defaults to `False`) : Whether or not to overwrite the `output_path`.

Read data from piped input to the python process. For multi columns data, columns should separated by 	

If columns are provided, then the output will be a dictionary with {column_x: value_x}

#### save[[transformers.PipedPipelineDataFormat.save]]

```python
save(data: dict)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L621)

**Parameters:**

data (`dict`) : The data to store.

Print the data.

## Utilities[[transformers.pipelines.PipelineException]]

#### transformers.pipelines.PipelineException[[transformers.pipelines.PipelineException]]

```python
transformers.pipelines.PipelineException(task: str, model: str, reason: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/pipelines/base.py#L366)

**Parameters:**

task (`str`) : The task of the pipeline.

model (`str`) : The model used by the pipeline.

reason (`str`) : The error message to display.

Raised by a [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) when handling __call__.
