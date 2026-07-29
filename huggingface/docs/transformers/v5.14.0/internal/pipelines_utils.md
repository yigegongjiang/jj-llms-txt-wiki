# Utilities for pipelines

This page lists all the utility functions the library provides for pipelines.

Most of those are only useful if you are studying the code of the models in the library.

## Argument handling[[transformers.pipelines.ArgumentHandler]]

Base interface for handling arguments for each [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline).

Handles arguments for zero-shot for text classification by turning each possible label into an NLI
premise/hypothesis pair.

## Data format[[transformers.PipelineDataFormat]]

- **output_path** (`str`) -- Where to save the outgoing data.
- **input_path** (`str`) -- Where to look for the input data.
- **column** (`str`) -- The column to read.
- **overwrite** (`bool`, *optional*, defaults to `False`) --
  Whether or not to overwrite the `output_path`.

Base class for all the pipeline supported data format both for reading and writing. Supported data formats
currently includes:

- JSON
- CSV
- stdin/stdout (pipe)

`PipelineDataFormat` also includes some utilities to work with multi-columns like mapping from datasets columns to
pipelines keyword arguments through the `dataset_kwarg_1=dataset_column_1` format.

- **format** (`str`) --
  The format of the desired pipeline. Acceptable values are `"json"`, `"csv"` or `"pipe"`.
- **output_path** (`str`, *optional*) --
  Where to save the outgoing data.
- **input_path** (`str`, *optional*) --
  Where to look for the input data.
- **column** (`str`, *optional*) --
  The column to read.
- **overwrite** (`bool`, *optional*, defaults to `False`) --
  Whether or not to overwrite the `output_path`.[PipelineDataFormat](/docs/transformers/v5.14.0/en/internal/pipelines_utils#transformers.PipelineDataFormat)The proper data format.

Creates an instance of the right subclass of [PipelineDataFormat](/docs/transformers/v5.14.0/en/internal/pipelines_utils#transformers.PipelineDataFormat) depending on `format`.

- **data** (`dict` or list of `dict`) -- The data to store.

Save the provided data object with the representation for the current [PipelineDataFormat](/docs/transformers/v5.14.0/en/internal/pipelines_utils#transformers.PipelineDataFormat).

- **data** (`dict` or list of `dict`) -- The data to store.`str`Path where the data has been saved.

Save the provided data object as a pickle-formatted binary data on the disk.

- **output_path** (`str`) -- Where to save the outgoing data.
- **input_path** (`str`) -- Where to look for the input data.
- **column** (`str`) -- The column to read.
- **overwrite** (`bool`, *optional*, defaults to `False`) --
  Whether or not to overwrite the `output_path`.

Support for pipelines using CSV data format.

- **data** (`list[dict]`) -- The data to store.

Save the provided data object with the representation for the current [PipelineDataFormat](/docs/transformers/v5.14.0/en/internal/pipelines_utils#transformers.PipelineDataFormat).

- **output_path** (`str`) -- Where to save the outgoing data.
- **input_path** (`str`) -- Where to look for the input data.
- **column** (`str`) -- The column to read.
- **overwrite** (`bool`, *optional*, defaults to `False`) --
  Whether or not to overwrite the `output_path`.

Support for pipelines using JSON file format.

- **data** (`dict`) -- The data to store.

Save the provided data object in a json file.

- **output_path** (`str`) -- Where to save the outgoing data.
- **input_path** (`str`) -- Where to look for the input data.
- **column** (`str`) -- The column to read.
- **overwrite** (`bool`, *optional*, defaults to `False`) --
  Whether or not to overwrite the `output_path`.

Read data from piped input to the python process. For multi columns data, columns should separated by 	

If columns are provided, then the output will be a dictionary with {column_x: value_x}

- **data** (`dict`) -- The data to store.

Print the data.

## Utilities[[transformers.pipelines.PipelineException]]

- **task** (`str`) -- The task of the pipeline.
- **model** (`str`) -- The model used by the pipeline.
- **reason** (`str`) -- The error message to display.

Raised by a [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) when handling __call__.
