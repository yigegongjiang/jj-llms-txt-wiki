# Scripts Utilities

## ScriptArguments[[trl.ScriptArguments]]

#### trl.ScriptArguments[[trl.ScriptArguments]]

```python
trl.ScriptArguments(dataset_name: str | None = None, dataset_config: str | None = None, dataset_train_split: str = 'train', dataset_test_split: str = 'test', dataset_streaming: bool = False, ignore_bias_buffers: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/scripts/utils.py#L146)

**Parameters:**

dataset_name (`str`,, *optional*) : Path or name of the dataset to load. If `datasets` is provided, this will be ignored.

dataset_config (`str`, *optional*) : Dataset configuration name. Corresponds to the `name` argument of the [load_dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/loading_methods#datasets.load_dataset) function. If `datasets` is provided, this will be ignored.

dataset_train_split (`str`, *optional*, defaults to `"train"`) : Dataset split to use for training. If `datasets` is provided, this will be ignored.

dataset_test_split (`str`, *optional*, defaults to `"test"`) : Dataset split to use for evaluation. If `datasets` is provided, this will be ignored.

dataset_streaming (`bool`, *optional*, defaults to `False`) : Whether to stream the dataset. If True, the dataset will be loaded in streaming mode. If `datasets` is provided, this will be ignored.

ignore_bias_buffers (`bool`, *optional*, defaults to `False`) : Debug argument for distributed training. Fix for DDP issues with LM bias/mask buffers - invalid scalar type, inplace operation. See https://github.com/huggingface/transformers/issues/22482#issuecomment-1595790992.

Arguments common to all scripts.

## TrlParser[[trl.TrlParser]]

#### trl.TrlParser[[trl.TrlParser]]

```python
trl.TrlParser(dataclass_types: typing.Union[trl.scripts._hf_argparser.DataClassType, collections.abc.Iterable[trl.scripts._hf_argparser.DataClassType], NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/scripts/utils.py#L234)

**Parameters:**

dataclass_types (`DataClassType | Iterable[DataClassType]`, *optional*) : Dataclass types to use for argument parsing.

- ****kwargs** : Additional keyword arguments passed to the [transformers.HfArgumentParser](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.HfArgumentParser) constructor.

A subclass of [transformers.HfArgumentParser](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.HfArgumentParser) designed for parsing command-line arguments with dataclass-backed
configurations, while also supporting configuration file loading and environment variable management.

Examples:

```yaml
# config.yaml
env:
    VAR1: value1
arg1: 23
```

```python
# main.py
import os
from dataclasses import dataclass
from trl import TrlParser

@dataclass
class MyArguments:
    arg1: int
    arg2: str = "alpha"

parser = TrlParser(dataclass_types=[MyArguments])
training_args = parser.parse_args_and_config()

print(training_args, os.environ.get("VAR1"))
```

```bash
$ python main.py --config config.yaml
(MyArguments(arg1=23, arg2='alpha'),) value1

$ python main.py --arg1 5 --arg2 beta
(MyArguments(arg1=5, arg2='beta'),) None
```

#### parse_args_and_config[[trl.TrlParser.parse_args_and_config]]

```python
parse_args_and_config(args: collections.abc.Iterable[str] | None = None, return_remaining_strings: bool = False, fail_with_unknown_args: bool = True, separate_remaining_strings: bool = False)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/scripts/utils.py#L303)

Parse command-line args and config file into instances of the specified dataclass types.

This method wraps [transformers.HfArgumentParser.parse_args_into_dataclasses](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.HfArgumentParser.parse_args_into_dataclasses) and also parses the config file
specified with the `--config` flag. The config file (in YAML format) provides argument values that replace the
default values in the dataclasses. Command line arguments can override values set by the config file. The
method also sets any environment variables specified in the `env` field of the config file.

#### parse_args_into_dataclasses[[trl.TrlParser.parse_args_into_dataclasses]]

```python
parse_args_into_dataclasses(args = None, return_remaining_strings = False, look_for_args_file = True, args_filename = None, args_file_flag = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/scripts/_hf_argparser.py#L298)

**Parameters:**

args : List of strings to parse. The default is taken from sys.argv. (same as argparse.ArgumentParser)

return_remaining_strings : If true, also return a list of remaining argument strings.

look_for_args_file : If true, will look for a ".args" file with the same base name as the entry point script for this process, and will append its potential content to the command line args.

args_filename : If not None, will uses this file instead of the ".args" file specified in the previous argument.

args_file_flag : If not None, will look for a file in the command-line args specified with this flag. The flag can be specified multiple times and precedence is determined by the order (last one wins).

**Returns:** `Tuple consisting of`

- the dataclass instances in the same order as they were passed to the initializer.abspath
- if applicable, an additional namespace for more (non-dataclass backed) arguments added to the parser
  after initialization.
- The potential list of remaining argument strings. (same as argparse.ArgumentParser.parse_known_args)

Parse command-line args into instances of the specified dataclass types.

This relies on argparse's `ArgumentParser.parse_known_args`. See the doc at:
docs.python.org/3/library/argparse.html#argparse.ArgumentParser.parse_args

#### set_defaults_with_config[[trl.TrlParser.set_defaults_with_config]]

```python
set_defaults_with_config(**kwargs)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/scripts/utils.py#L361)

Overrides the parser's default values with those provided via keyword arguments, including for subparsers.

Any argument with an updated default will also be marked as not required if it was previously required.

Returns a list of strings that were not consumed by the parser.

## get_dataset[[trl.get_dataset]]

#### trl.get_dataset[[trl.get_dataset]]

```python
trl.get_dataset(mixture_config: DatasetMixtureConfig)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/scripts/utils.py#L414)

**Parameters:**

mixture_config ([DatasetMixtureConfig](/docs/trl/v1.12.0/en/script_utils#trl.DatasetMixtureConfig)) : Script arguments containing dataset configuration.

**Returns:** [DatasetDict](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/main_classes#datasets.DatasetDict)

Combined dataset(s) from the mixture configuration, with optional train/test split if `test_split_size` is
set.

Load a mixture of datasets based on the configuration.

Example:
```python
>>> from trl import DatasetMixtureConfig, get_dataset
>>> from trl.scripts.utils import DatasetConfig

>>> mixture_config = DatasetMixtureConfig(datasets=[DatasetConfig(path="trl-lib/tldr")])
>>> get_dataset(mixture_config)
DatasetDict({
    train: Dataset({
        features: ['prompt', 'completion'],
        num_rows: 116722
    })
})
```

## DatasetConfig[[trl.scripts.utils.DatasetConfig]]

#### trl.scripts.utils.DatasetConfig[[trl.scripts.utils.DatasetConfig]]

```python
trl.scripts.utils.DatasetConfig(path: str, name: str | None = None, data_dir: str | None = None, data_files: str | list[str] | dict[str, str] | None = None, split: str = 'train', columns: list[str] | None = None, fraction: float | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/scripts/utils.py#L40)

**Parameters:**

path (`str`) : Path or name of the dataset.

name (`str`, *optional*) : Defining the name of the dataset configuration.

data_dir (`str`, *optional*) : Defining the `data_dir` of the dataset configuration. If specified for the generic builders(csv, text etc.) or the Hub datasets and `data_files` is `None`, the behavior is equal to passing `os.path.join(data_dir, **)` as `data_files` to reference all the files in a directory.

data_files (`str` or `Sequence` or `Mapping`, *optional*) : Path(s) to source data file(s).

split (`str`, *optional*, defaults to `"train"`) : Which split of the data to load.

columns (`list[str]`, *optional*) : List of column names to select from the dataset. If `None`, all columns are selected.

fraction (`float`, *optional*) : Target share of this dataset in the final mixture. Fractions are normalized to sum to one across all datasets, and the mixture size is capped so that no dataset is oversampled: the first `round(fraction * N)` rows of each dataset are kept, where `N` is the largest mixture size that avoids oversampling. Must be set for either all datasets in the mixture or none of them. Not supported for streaming datasets. When unset, the full datasets are concatenated.

Configuration for a dataset.

This class matches the signature of [load_dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/loading_methods#datasets.load_dataset) and the arguments are used directly in the
[load_dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/loading_methods#datasets.load_dataset) function. You can refer to the [load_dataset](https://huggingface.co/docs/datasets/v5.0.1/en/package_reference/loading_methods#datasets.load_dataset) documentation for more
details.

## DatasetMixtureConfig[[trl.DatasetMixtureConfig]]

#### trl.DatasetMixtureConfig[[trl.DatasetMixtureConfig]]

```python
trl.DatasetMixtureConfig(datasets: list = <factory>, streaming: bool = False, test_split_size: float | None = None)
```

[Source](https://github.com/huggingface/trl/blob/v1.12.0/trl/scripts/utils.py#L81)

**Parameters:**

datasets (`list[DatasetConfig]`) : List of dataset configurations to include in the mixture.

streaming (`bool`, *optional*, defaults to `False`) : Whether to stream the datasets. If `True`, the datasets will be loaded in streaming mode.

test_split_size (`float`, *optional*) : Size of the test split. Refer to the `test_size` parameter in the `train_test_split` function for more details. If `None`, the dataset will not be split into train and test sets.

Configuration class for a mixture of datasets.

Using [HfArgumentParser](https://huggingface.co/docs/transformers/v5.16.1/en/internal/trainer_utils#transformers.HfArgumentParser) we can turn this class into
[argparse](https://docs.python.org/3/library/argparse#module-argparse) arguments that can be specified on the
command line.

Usage:

When using the CLI, you can add the following section to your YAML config file:

```yaml
datasets:
- path: ...
    name: ...
    data_dir: ...
    data_files: ...
    split: ...
    columns: ...
    fraction: ...
- path: ...
    name: ...
    data_dir: ...
    data_files: ...
    split: ...
    columns: ...
streaming: ...
test_split_size: ...
```
