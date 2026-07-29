# Loading methods

Methods for listing and loading datasets:

## Datasets[[datasets.load_dataset]]

#### datasets.load_dataset[[datasets.load_dataset]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/load.py#L1467)

Load a dataset from the Hugging Face Hub, or a local dataset.

You can find the list of datasets on the [Hub](https://huggingface.co/datasets) or with `huggingface_hub.list_datasets`.

A dataset is a directory that contains some data files in generic formats (JSON, CSV, Parquet, etc.) and possibly
in a generic structure (Webdataset, ImageFolder, AudioFolder, VideoFolder, etc.)

This function does the following under the hood:

1. Load a dataset builder:

   * Find the most common data format in the dataset and pick its associated builder (JSON, CSV, Parquet, Webdataset, ImageFolder, AudioFolder, etc.)
   * Find which file goes into which split (e.g. train/test) based on file and directory names or on the YAML configuration
   * It is also possible to specify `data_files` manually, and which dataset builder to use (e.g. "parquet").

2. Run the dataset builder:

   In the general case:

   * Download the data files from the dataset if they are not already available locally or cached.
   * Process and cache the dataset in typed Arrow tables for caching.

     Arrow table are arbitrarily long, typed tables which can store nested objects and be mapped to numpy/pandas/python generic types.
     They can be directly accessed from disk, loaded in RAM or even streamed over the web.

   In the streaming case:

   * Don't download or cache anything. Instead, the dataset is lazily loaded and will be streamed on-the-fly when iterating on it.

3. Return a dataset built from the requested splits in `split` (default: all).

Example:

Load a dataset from the Hugging Face Hub:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset('cornell-movie-review-data/rotten_tomatoes', split='train')

# Load a subset or dataset configuration (here 'sst2')
>>> from datasets import load_dataset
>>> ds = load_dataset('nyu-mll/glue', 'sst2', split='train')

# Manual mapping of data files to splits
>>> data_files = {'train': 'train.csv', 'test': 'test.csv'}
>>> ds = load_dataset('namespace/your_dataset_name', data_files=data_files)

# Manual selection of a directory to load
>>> ds = load_dataset('namespace/your_dataset_name', data_dir='folder_name')
```

Load a dataset from a Storage Bucket on the Hugging Face Hub:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset('buckets/username/bucket_name/rotten_tomatoes', split='train')
```

Load a local dataset:

```py
# Load a CSV file
>>> from datasets import load_dataset
>>> ds = load_dataset('csv', data_files='path/to/local/my_dataset.csv')

# Load a JSON file
>>> from datasets import load_dataset
>>> ds = load_dataset('json', data_files='path/to/local/my_dataset.json')
```

Load an [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset):

```py
>>> from datasets import load_dataset
>>> ds = load_dataset('cornell-movie-review-data/rotten_tomatoes', split='train', streaming=True)
```

Load an image dataset with the `ImageFolder` dataset builder:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset('imagefolder', data_dir='/path/to/images', split='train')
```

**Parameters:**

path (`str`) : Path or name of the dataset.  - if `path` is a dataset repository on the HF hub (list all available datasets with `huggingface_hub.list_datasets`) -> load the dataset from supported files in the repository (csv, json, parquet, etc.) e.g. `'username/dataset_name'`, a dataset repository on the HF hub containing the data files.  - if `path` is a directory within a Storage Bucket on the HF Hub (list your buckets with `huggingface_hub.list_buckets`) -> load the dataset from supported files in the directory (csv, json, parquet, etc.) e.g. `'buckets/username/bucket_name/my_dataset'`.  - if `path` is a local directory -> load the dataset from supported files in the directory (csv, json, parquet, etc.) e.g. `'./path/to/directory/with/my/csv/data'`.  - if `path` is the name of a dataset builder and `data_files` or `data_dir` is specified (available builders are "json", "csv", "parquet", "arrow", "text", "xml", "webdataset", "imagefolder", "audiofolder", "videofolder") -> load the dataset from the files in `data_files` or `data_dir` e.g. `'parquet'`.  Use a `hf://` path like `'hf://datasets/username/dataset_name'` to allow remote only. Use an absolute path to allow local only. 

name (`str`, *optional*) : Defining the name of the dataset configuration.

data_dir (`str`, *optional*) : Defining the `data_dir` of the dataset configuration. If specified for the generic builders (csv, text etc.) or the Hub datasets and `data_files` is `None`, the behavior is equal to passing `os.path.join(data_dir, **)` as `data_files` to reference all the files in a directory.

data_files (`str` or `Sequence` or `Mapping`, *optional*) : Path(s) to source data file(s).

split (`Split` or `str`) : Which split of the data to load. If `None`, will return a `dict` with all splits (typically `datasets.Split.TRAIN` and `datasets.Split.TEST`). If given, will return a single Dataset. Splits can be combined and specified like in tensorflow-datasets.

cache_dir (`str`, *optional*) : Directory to read/write data. Defaults to `"~/.cache/huggingface/datasets"`.

features (`Features`, *optional*) : Set the features type to use for this dataset.

download_config ([DownloadConfig](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadConfig), *optional*) : Specific download configuration parameters.

download_mode ([DownloadMode](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadMode) or `str`, defaults to `REUSE_DATASET_IF_EXISTS`) : Download/generate mode.

verification_mode ([VerificationMode](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.VerificationMode) or `str`, defaults to `BASIC_CHECKS`) : Verification mode determining the checks to run on the downloaded/processed dataset information (checksums/size/splits/...).  

keep_in_memory (`bool`, defaults to `None`) : Whether to copy the dataset in-memory. If `None`, the dataset will not be copied in-memory unless explicitly enabled by setting `datasets.config.IN_MEMORY_MAX_SIZE` to nonzero. See more details in the [improve performance](../cache#improve-performance) section.

revision ([Version](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.Version) or `str`, *optional*) : Version of the dataset to load. As datasets have their own git repository on the Datasets Hub, the default version "main" corresponds to their "main" branch. You can specify a different version than the default "main" by using a commit SHA or a git tag of the dataset repository.

token (`str` or `bool`, *optional*) : Optional string or boolean to use as Bearer token for remote files on the Datasets Hub. If `True`, or not specified, will get token from `"~/.huggingface"`.

streaming (`bool`, defaults to `False`) : If set to `True`, don't download the data files. Instead, it streams the data progressively while iterating on the dataset. An [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset) or [IterableDatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDatasetDict) is returned instead in this case.  Note that streaming works for datasets that use data formats that support being iterated over like txt, csv, jsonl for example. Json files may be downloaded completely. Also streaming from remote zip or gzip files is supported but other compressed formats like rar and xz are not yet supported. The tgz format doesn't allow streaming.

num_proc (`int`, *optional*, defaults to `None`) : Number of processes when downloading and generating the dataset locally. Multiprocessing is disabled by default.  

storage_options (`dict`, *optional*, defaults to `None`) : **Experimental**. Key/value pairs to be passed on to the dataset file-system backend, if any.  

- ****config_kwargs** (additional keyword arguments) : Keyword arguments to be passed to the `BuilderConfig` and used in the [DatasetBuilder](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DatasetBuilder).

**Returns:**

`[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) or [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)`

- if `split` is not `None`: the dataset requested,
- if `split` is `None`, a [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) with each split.

or [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset) or [IterableDatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDatasetDict): if `streaming=True`

- if `split` is not `None`, the dataset is requested
- if `split` is `None`, a `~datasets.streaming.IterableDatasetDict` with each split.

#### datasets.load_from_disk[[datasets.load_from_disk]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/load.py#L1725)

Loads a dataset that was previously saved using [save_to_disk()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.save_to_disk) from a dataset directory, or
from a filesystem using any implementation of `fsspec.spec.AbstractFileSystem`.

Example:

```py
>>> from datasets import load_from_disk
>>> ds = load_from_disk('path/to/dataset/directory')
```

**Parameters:**

dataset_path (`path-like`) : Path (e.g. `"dataset/train"`) or remote URI (e.g. `"s3://my-bucket/dataset/train"`) of the [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) or [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) directory where the dataset/dataset-dict will be loaded from.

keep_in_memory (`bool`, defaults to `None`) : Whether to copy the dataset in-memory. If `None`, the dataset will not be copied in-memory unless explicitly enabled by setting `datasets.config.IN_MEMORY_MAX_SIZE` to nonzero. See more details in the [improve performance](../cache#improve-performance) section. 

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  

**Returns:**

`[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) or [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)`

- If `dataset_path` is a path of a dataset directory: the dataset requested.
- If `dataset_path` is a path of a dataset dict directory, a [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) with each split.

#### datasets.load_dataset_builder[[datasets.load_dataset_builder]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/load.py#L1212)

Load a dataset builder which can be used to:

- Inspect general information that is required to build a dataset (cache directory, config, dataset info, features, data files, etc.)
- Download and prepare the dataset as Arrow files in the cache
- Get a streaming dataset without downloading or caching anything

You can find the list of datasets on the [Hub](https://huggingface.co/datasets) or with `huggingface_hub.list_datasets`.

A dataset is a directory that contains some data files in generic formats (JSON, CSV, Parquet, etc.) and possibly
in a generic structure (Webdataset, ImageFolder, AudioFolder, VideoFolder, etc.)

Example:

```py
>>> from datasets import load_dataset_builder
>>> ds_builder = load_dataset_builder('cornell-movie-review-data/rotten_tomatoes')
>>> ds_builder.info.features
{'label': ClassLabel(names=['neg', 'pos']),
 'text': Value('string')}
```

**Parameters:**

path (`str`) : Path or name of the dataset.  - if `path` is a dataset repository on the HF hub (list all available datasets with `huggingface_hub.list_datasets`) -> load the dataset builder from supported files in the repository (csv, json, parquet, etc.) e.g. `'username/dataset_name'`, a dataset repository on the HF hub containing the data files.  - if `path` is a directory within a Storage Bucket on the HF Hub (list your buckets with `huggingface_hub.list_buckets`) -> load the dataset from supported files in the directory (csv, json, parquet, etc.) e.g. `'buckets/username/bucket_name/my_dataset'`.  - if `path` is a local directory -> load the dataset builder from supported files in the directory (csv, json, parquet, etc.) e.g. `'./path/to/directory/with/my/csv/data'`.  - if `path` is the name of a dataset builder and `data_files` or `data_dir` is specified (available builders are "json", "csv", "parquet", "arrow", "text", "xml", "webdataset", "imagefolder", "audiofolder", "videofolder") -> load the dataset builder from the files in `data_files` or `data_dir` e.g. `'parquet'`.  Use a `hf://` path like `'hf://datasets/username/dataset_name'` to allow remote only. Use an absolute path to allow local only. 

name (`str`, *optional*) : Defining the name of the dataset configuration.

data_dir (`str`, *optional*) : Defining the `data_dir` of the dataset configuration. If specified for the generic builders (csv, text etc.) or the Hub datasets and `data_files` is `None`, the behavior is equal to passing `os.path.join(data_dir, **)` as `data_files` to reference all the files in a directory.

data_files (`str` or `Sequence` or `Mapping`, *optional*) : Path(s) to source data file(s).

cache_dir (`str`, *optional*) : Directory to read/write data. Defaults to `"~/.cache/huggingface/datasets"`.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Set the features type to use for this dataset.

download_config ([DownloadConfig](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadConfig), *optional*) : Specific download configuration parameters.

download_mode ([DownloadMode](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadMode) or `str`, defaults to `REUSE_DATASET_IF_EXISTS`) : Download/generate mode.

revision ([Version](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.Version) or `str`, *optional*) : Version of the dataset to load. As datasets have their own git repository on the Datasets Hub, the default version "main" corresponds to their "main" branch. You can specify a different version than the default "main" by using a commit SHA or a git tag of the dataset repository.

token (`str` or `bool`, *optional*) : Optional string or boolean to use as Bearer token for remote files on the Datasets Hub. If `True`, or not specified, will get token from `"~/.huggingface"`.

storage_options (`dict`, *optional*, defaults to `None`) : **Experimental**. Key/value pairs to be passed on to the dataset file-system backend, if any.   

- ****config_kwargs** (additional keyword arguments) : Keyword arguments to be passed to the [BuilderConfig](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.BuilderConfig) and used in the [DatasetBuilder](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DatasetBuilder).

**Returns:**

[DatasetBuilder](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DatasetBuilder)

#### datasets.get_dataset_config_names[[datasets.get_dataset_config_names]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/inspect.py#L109)

Get the list of available config names for a particular dataset.

Example:

```py
>>> from datasets import get_dataset_config_names
>>> get_dataset_config_names("nyu-mll/glue")
['cola',
 'sst2',
 'mrpc',
 'qqp',
 'stsb',
 'mnli',
 'mnli_mismatched',
 'mnli_matched',
 'qnli',
 'rte',
 'wnli',
 'ax']
```

**Parameters:**

path (`str`) : path to the dataset repository. Can be either:  - a local path to the dataset directory containing the data files, e.g. `'./dataset/squad'` - a dataset identifier on the Hugging Face Hub (list all available datasets and ids with `huggingface_hub.list_datasets`), e.g. `'rajpurkar/squad'`, `'nyu-mll/glue'` or``'openai/webtext'`

revision (`Union[str, datasets.Version]`, *optional*) : If specified, the dataset module will be loaded from the datasets repository at this version. By default: - it is set to the local version of the lib. - it will also try to load it from the main branch if it's not available at the local version of the lib. Specifying a version that is different from your local version of the lib might cause compatibility issues.

download_config ([DownloadConfig](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadConfig), *optional*) : Specific download configuration parameters.

download_mode ([DownloadMode](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadMode) or `str`, defaults to `REUSE_DATASET_IF_EXISTS`) : Download/generate mode.

data_files (`Union[Dict, List, str]`, *optional*) : Defining the data_files of the dataset configuration.

- ****download_kwargs** (additional keyword arguments) : Optional attributes for [DownloadConfig](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadConfig) which will override the attributes in `download_config` if supplied, for example `token`.

#### datasets.get_dataset_infos[[datasets.get_dataset_infos]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/inspect.py#L42)

Get the meta information about a dataset, returned as a dict mapping config name to DatasetInfoDict.

Example:

```py
>>> from datasets import get_dataset_infos
>>> get_dataset_infos('cornell-movie-review-data/rotten_tomatoes')
{'default': DatasetInfo(description="Movie Review Dataset.
 is a dataset of containing 5,331 positive and 5,331 negative processed
ences from Rotten Tomatoes movie reviews...), ...}
```

**Parameters:**

path (`str`) : path to the dataset repository. Can be either:  - a local path to the dataset directory containing the data files, e.g. `'./dataset/squad'` - a dataset identifier on the Hugging Face Hub (list all available datasets and ids with `huggingface_hub.list_datasets`), e.g. `'rajpurkar/squad'`, `'nyu-mll/glue'` or``'openai/webtext'`

revision (`Union[str, datasets.Version]`, *optional*) : If specified, the dataset module will be loaded from the datasets repository at this version. By default: - it is set to the local version of the lib. - it will also try to load it from the main branch if it's not available at the local version of the lib. Specifying a version that is different from your local version of the lib might cause compatibility issues.

download_config ([DownloadConfig](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadConfig), *optional*) : Specific download configuration parameters.

download_mode ([DownloadMode](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadMode) or `str`, defaults to `REUSE_DATASET_IF_EXISTS`) : Download/generate mode.

data_files (`Union[Dict, List, str]`, *optional*) : Defining the data_files of the dataset configuration.

token (`str` or `bool`, *optional*) : Optional string or boolean to use as Bearer token for remote files on the Datasets Hub. If `True`, or not specified, will get token from `"~/.huggingface"`.

- ****config_kwargs** (additional keyword arguments) : Optional attributes for builder class which will override the attributes if supplied.

#### datasets.get_dataset_split_names[[datasets.get_dataset_split_names]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/inspect.py#L295)

Get the list of available splits for a particular config and dataset.

Example:

```py
>>> from datasets import get_dataset_split_names
>>> get_dataset_split_names('cornell-movie-review-data/rotten_tomatoes')
['train', 'validation', 'test']
```

**Parameters:**

path (`str`) : path to the dataset repository. Can be either:  - a local path to the dataset directory containing the data files, e.g. `'./dataset/squad'` - a dataset identifier on the Hugging Face Hub (list all available datasets and ids with `huggingface_hub.list_datasets`), e.g. `'rajpurkar/squad'`, `'nyu-mll/glue'` or``'openai/webtext'`

config_name (`str`, *optional*) : Defining the name of the dataset configuration.

data_files (`str` or `Sequence` or `Mapping`, *optional*) : Path(s) to source data file(s).

download_config ([DownloadConfig](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadConfig), *optional*) : Specific download configuration parameters.

download_mode ([DownloadMode](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DownloadMode) or `str`, defaults to `REUSE_DATASET_IF_EXISTS`) : Download/generate mode.

revision ([Version](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.Version) or `str`, *optional*) : Version of the dataset to load. As datasets have their own git repository on the Datasets Hub, the default version "main" corresponds to their "main" branch. You can specify a different version than the default "main" by using a commit SHA or a git tag of the dataset repository.

token (`str` or `bool`, *optional*) : Optional string or boolean to use as Bearer token for remote files on the Datasets Hub. If `True`, or not specified, will get token from `"~/.huggingface"`.

- ****config_kwargs** (additional keyword arguments) : Optional attributes for builder class which will override the attributes if supplied.

## From files

Configurations used to load data files.
They are used when loading local files or a dataset repository:

- local files: `load_dataset("parquet", data_dir="path/to/data/dir")`
- dataset repository: `load_dataset("allenai/c4")`

You can pass arguments to `load_dataset` to configure data loading.
For example you can specify the `sep` parameter to define the [CsvConfig](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.packaged_modules.csv.CsvConfig) that is used to load the data:

```python
load_dataset("csv", data_dir="path/to/data/dir", sep="\t")
```

### Text[[datasets.packaged_modules.text.TextConfig]]

#### datasets.packaged_modules.text.TextConfig[[datasets.packaged_modules.text.TextConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/text/text.py#L17)

BuilderConfig for text files.

**Parameters:**

features : (`Features`, *optional*): Cast the data to `features`.

encoding : (`str`, defaults to "utf-8"): Encoding to decode the file.

encoding_errors : (`str`, *optional*): Argument to define what to do in case of encoding error. This is the same as the `error` argument in `open()`.

chunksize : (`Features`, *optional*, defaults to "10MB"): Chunk size to read the data.

keep_linebreaks : (`bool`, defaults to False): Whether to keep line breaks.

sample_by (`Literal["line", "paragraph", "document"]`, defaults to "line") : Whether to load data per line, praragraph or document. By default one row in the dataset = one line.

#### datasets.packaged_modules.text.Text[[datasets.packaged_modules.text.Text]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/text/text.py#L45)

### CSV[[datasets.packaged_modules.csv.CsvConfig]]

#### datasets.packaged_modules.csv.CsvConfig[[datasets.packaged_modules.csv.CsvConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/csv/csv.py#L25)

BuilderConfig for CSV.

#### datasets.packaged_modules.csv.Csv[[datasets.packaged_modules.csv.Csv]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/csv/csv.py#L145)

### JSON[[datasets.packaged_modules.json.JsonConfig]]

#### datasets.packaged_modules.json.JsonConfig[[datasets.packaged_modules.json.JsonConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/json/json.py#L41)

BuilderConfig for JSON.

#### datasets.Json[[datasets.Json]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/json/json.py#L58)

### XML[[datasets.packaged_modules.xml.XmlConfig]]

#### datasets.packaged_modules.xml.XmlConfig[[datasets.packaged_modules.xml.XmlConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/xml/xml.py#L15)

BuilderConfig for xml files.

#### datasets.packaged_modules.xml.Xml[[datasets.packaged_modules.xml.Xml]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/xml/xml.py#L23)

### Parquet[[datasets.packaged_modules.parquet.ParquetConfig]]

#### datasets.packaged_modules.parquet.ParquetConfig[[datasets.packaged_modules.parquet.ParquetConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/parquet/parquet.py#L17)

BuilderConfig for Parquet.

Example:

Load a subset of columns:

```python
>>> ds = load_dataset(parquet_dataset_id, columns=["col_0", "col_1"])
```

Stream data and efficiently filter data, possibly skipping entire files or row groups:

```python
>>> filters = [("col_0", "==", 0)]
>>> ds = load_dataset(parquet_dataset_id, streaming=True, filters=filters)
```

Increase the minimum request size when streaming from 32MiB (default) to 128MiB and enable prefetching:

```python
>>> import pyarrow
>>> import pyarrow.dataset
>>> fragment_scan_options = pyarrow.dataset.ParquetFragmentScanOptions(
...     cache_options=pyarrow.CacheOptions(
...         prefetch_limit=1,
...         range_size_limit=128 >> ds = load_dataset(parquet_dataset_id, streaming=True, fragment_scan_options=fragment_scan_options)
```

**Parameters:**

batch_size (`int`, *optional*) : Size of the RecordBatches to iterate on. The default is the row group size (defined by the first row group).

columns (`list[str]`, *optional*) : List of columns to load, the other ones are ignored. All columns are loaded by default.

features : (`Features`, *optional*): Cast the data to `features`.

filters (`Union[pyarrow.dataset.Expression, list[tuple], list[list[tuple]]]`, *optional*) : Return only the rows matching the filter. If possible the predicate will be pushed down to exploit the partition information or internal metadata found in the data source, e.g. Parquet statistics. Otherwise filters the loaded RecordBatches before yielding them.

fragment_scan_options (`pyarrow.dataset.ParquetFragmentScanOptions`, *optional*) : Scan-specific options for Parquet fragments. This is especially useful to configure buffering and caching.  

on_bad_files (`Literal["error", "warn", "skip"]`, *optional*, defaults to "error") : Specify what to do upon encountering a bad file (a file that can't be read). Allowed values are : * 'error', raise an Exception when a bad file is encountered. * 'warn', raise a warning when a bad file is encountered and skip that file. * 'skip', skip bad files without raising or warning when they are encountered.  

#### datasets.packaged_modules.parquet.Parquet[[datasets.packaged_modules.parquet.Parquet]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/parquet/parquet.py#L90)

### Arrow[[datasets.packaged_modules.arrow.ArrowConfig]]

#### datasets.packaged_modules.arrow.ArrowConfig[[datasets.packaged_modules.arrow.ArrowConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/arrow/arrow.py#L15)

BuilderConfig for Arrow.

#### datasets.packaged_modules.arrow.Arrow[[datasets.packaged_modules.arrow.Arrow]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/arrow/arrow.py#L24)

### SQL[[datasets.packaged_modules.sql.SqlConfig]]

#### datasets.packaged_modules.sql.SqlConfig[[datasets.packaged_modules.sql.SqlConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/sql/sql.py#L25)

BuilderConfig for SQL.

#### datasets.packaged_modules.sql.Sql[[datasets.packaged_modules.sql.Sql]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/sql/sql.py#L92)

### Images[[datasets.packaged_modules.imagefolder.ImageFolderConfig]]

#### datasets.packaged_modules.imagefolder.ImageFolderConfig[[datasets.packaged_modules.imagefolder.ImageFolderConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/imagefolder/imagefolder.py#L9)

BuilderConfig for ImageFolder.

#### datasets.packaged_modules.imagefolder.ImageFolder[[datasets.packaged_modules.imagefolder.ImageFolder]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/imagefolder/imagefolder.py#L19)

### Audio[[datasets.packaged_modules.audiofolder.AudioFolderConfig]]

#### datasets.packaged_modules.audiofolder.AudioFolderConfig[[datasets.packaged_modules.audiofolder.AudioFolderConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/audiofolder/audiofolder.py#L9)

Builder Config for AudioFolder.

#### datasets.packaged_modules.audiofolder.AudioFolder[[datasets.packaged_modules.audiofolder.AudioFolder]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/audiofolder/audiofolder.py#L19)

### Videos[[datasets.packaged_modules.videofolder.VideoFolderConfig]]

#### datasets.packaged_modules.videofolder.VideoFolderConfig[[datasets.packaged_modules.videofolder.VideoFolderConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/videofolder/videofolder.py#L9)

BuilderConfig for ImageFolder.

#### datasets.packaged_modules.videofolder.VideoFolder[[datasets.packaged_modules.videofolder.VideoFolder]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/videofolder/videofolder.py#L19)

### HDF5[[datasets.packaged_modules.hdf5.HDF5Config]]

#### datasets.packaged_modules.hdf5.HDF5Config[[datasets.packaged_modules.hdf5.HDF5Config]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/hdf5/hdf5.py#L33)

BuilderConfig for HDF5.

#### datasets.packaged_modules.hdf5.HDF5[[datasets.packaged_modules.hdf5.HDF5]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/hdf5/hdf5.py#L40)

ArrowBasedBuilder that converts HDF5 files to Arrow tables using the HF extension types.

### Pdf[[datasets.packaged_modules.pdffolder.PdfFolderConfig]]

#### datasets.packaged_modules.pdffolder.PdfFolderConfig[[datasets.packaged_modules.pdffolder.PdfFolderConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/pdffolder/pdffolder.py#L9)

BuilderConfig for ImageFolder.

#### datasets.packaged_modules.pdffolder.PdfFolder[[datasets.packaged_modules.pdffolder.PdfFolder]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/pdffolder/pdffolder.py#L19)

### Nifti[[datasets.packaged_modules.niftifolder.NiftiFolderConfig]]

#### datasets.packaged_modules.niftifolder.NiftiFolderConfig[[datasets.packaged_modules.niftifolder.NiftiFolderConfig]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/niftifolder/niftifolder.py#L9)

BuilderConfig for NiftiFolder.

#### datasets.packaged_modules.niftifolder.NiftiFolder[[datasets.packaged_modules.niftifolder.NiftiFolder]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/niftifolder/niftifolder.py#L19)

### WebDataset[[datasets.packaged_modules.webdataset.WebDataset]]

#### datasets.packaged_modules.webdataset.WebDataset[[datasets.packaged_modules.webdataset.WebDataset]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/packaged_modules/webdataset/webdataset.py#L20)
