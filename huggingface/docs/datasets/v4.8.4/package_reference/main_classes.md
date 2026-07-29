# Main classes

## DatasetInfo[[datasets.DatasetInfo]]

#### datasets.DatasetInfo[[datasets.DatasetInfo]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/info.py#L92)

Information about a dataset.

`DatasetInfo` documents datasets, including its name, version, and features.
See the constructor arguments and properties for a full list.

Not all fields are known on construction and may be updated later.

from_directorydatasets.DatasetInfo.from_directoryhttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/info.py#L247[{"name": "dataset_info_dir", "val": ": str"}, {"name": "storage_options", "val": ": typing.Optional[dict] = None"}]- **dataset_info_dir** (`str`) --
  The directory containing the metadata file. This
  should be the root directory of a specific dataset version.
- **storage_options** (`dict`, *optional*) --
  Key/value pairs to be passed on to the file-system backend, if any.

  0
Create [DatasetInfo](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetInfo) from the JSON file in `dataset_info_dir`.

This function updates all the dynamically generated fields (num_examples,
hash, time of creation,...) of the [DatasetInfo](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetInfo).

This will overwrite all previous metadata.

Example:

```py
>>> from datasets import DatasetInfo
>>> ds_info = DatasetInfo.from_directory("/path/to/directory/")
```

**Parameters:**

description (`str`) : A description of the dataset.

citation (`str`) : A BibTeX citation of the dataset.

homepage (`str`) : A URL to the official homepage for the dataset.

license (`str`) : The dataset's license. It can be the name of the license or a paragraph containing the terms of the license.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : The features used to specify the dataset's column types.

post_processed (`PostProcessedInfo`, *optional*) : Information regarding the resources of a possible post-processing of a dataset. For example, it can contain the information of an index.

supervised_keys (`SupervisedKeysData`, *optional*) : Specifies the input feature and the label for supervised learning if applicable for the dataset (legacy from TFDS).

builder_name (`str`, *optional*) : The name of the `GeneratorBasedBuilder` subclass used to create the dataset. It is also the snake_case version of the dataset builder class name.

config_name (`str`, *optional*) : The name of the configuration derived from [BuilderConfig](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.BuilderConfig).

version (`str` or [Version](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.Version), *optional*) : The version of the dataset.

splits (`dict`, *optional*) : The mapping between split name and metadata.

download_checksums (`dict`, *optional*) : The mapping between the URL to download the dataset's checksums and corresponding metadata.

download_size (`int`, *optional*) : The size of the files to download to generate the dataset, in bytes.

post_processing_size (`int`, *optional*) : Size of the dataset in bytes after post-processing, if any.

dataset_size (`int`, *optional*) : The combined size in bytes of the Arrow tables for all splits.

size_in_bytes (`int`, *optional*) : The combined size in bytes of all files associated with the dataset (downloaded files + Arrow files).

- ****config_kwargs** (additional keyword arguments) : Keyword arguments to be passed to the [BuilderConfig](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.BuilderConfig) and used in the [DatasetBuilder](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.DatasetBuilder).
#### write_to_directory[[datasets.DatasetInfo.write_to_directory]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/info.py#L186)

Write `DatasetInfo` and license (if present) as JSON files to `dataset_info_dir`.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.info.write_to_directory("/path/to/directory/")
```

**Parameters:**

dataset_info_dir (`str`) : Destination directory.

pretty_print (`bool`, defaults to `False`) : If `True`, the JSON will be pretty-printed with the indent level of 4.

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  

## Dataset[[datasets.Dataset]]

The base class [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) implements a Dataset backed by an Apache Arrow table.

#### datasets.Dataset[[datasets.Dataset]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L716)

A Dataset backed by an Arrow table.

add_columndatasets.Dataset.add_columnhttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L6195[{"name": "name", "val": ": str"}, {"name": "column", "val": ": typing.Union[list, numpy.ndarray]"}, {"name": "new_fingerprint", "val": ": typing.Optional[str] = None"}, {"name": "feature", "val": ": typing.Union[dict, list, tuple, datasets.features.features.Value, datasets.features.features.ClassLabel, datasets.features.translation.Translation, datasets.features.translation.TranslationVariableLanguages, datasets.features.features.LargeList, datasets.features.features.List, datasets.features.features.Array2D, datasets.features.features.Array3D, datasets.features.features.Array4D, datasets.features.features.Array5D, datasets.features.audio.Audio, datasets.features.image.Image, datasets.features.video.Video, datasets.features.pdf.Pdf, datasets.features.nifti.Nifti, NoneType] = None"}]- **name** (`str`) --
  Column name.
- **column** (`list` or `np.array`) --
  Column data to be added.
- **feature** (`FeatureType` or `None`, defaults to `None`) --
  Column datatype.0[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
Add column to Dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> more_text = ds["text"]
>>> ds = ds.add_column(name="text_2", column=more_text)
>>> ds
Dataset({
    features: ['text', 'label', 'text_2'],
    num_rows: 1066
})
```

**Parameters:**

name (`str`) : Column name.

column (`list` or `np.array`) : Column data to be added.

feature (`FeatureType` or `None`, defaults to `None`) : Column datatype.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### add_item[[datasets.Dataset.add_item]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L6457)

Add item to Dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> new_review = {'label': 0, 'text': 'this movie is the absolute worst thing I have ever seen'}
>>> ds = ds.add_item(new_review)
>>> ds[-1]
{'label': 0, 'text': 'this movie is the absolute worst thing I have ever seen'}
```

**Parameters:**

item (`dict`) : Item data to be added.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### from_file[[datasets.Dataset.from_file]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L799)

Instantiate a Dataset backed by an Arrow table at filename.

**Parameters:**

filename (`str`) : File name of the dataset.

info (`DatasetInfo`, *optional*) : Dataset information, like description, citation, etc.

split (`NamedSplit`, *optional*) : Name of the dataset split.

indices_filename (`str`, *optional*) : File names of the indices.

in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### from_buffer[[datasets.Dataset.from_buffer]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L839)

Instantiate a Dataset backed by an Arrow buffer.

**Parameters:**

buffer (`pyarrow.Buffer`) : Arrow buffer.

info (`DatasetInfo`, *optional*) : Dataset information, like description, citation, etc.

split (`NamedSplit`, *optional*) : Name of the dataset split.

indices_buffer (`pyarrow.Buffer`, *optional*) : Indices Arrow buffer.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### from_pandas[[datasets.Dataset.from_pandas]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L871)

Convert `pandas.DataFrame` to a `pyarrow.Table` to create a [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset).

The column types in the resulting Arrow Table are inferred from the dtypes of the `pandas.Series` in the
DataFrame. In the case of non-object Series, the NumPy dtype is translated to its Arrow equivalent. In the
case of `object`, we need to guess the datatype by looking at the Python objects in this Series.

Be aware that Series of the `object` dtype don't carry enough information to always lead to a meaningful Arrow
type. In the case that we cannot infer a type, e.g. because the DataFrame is of length 0 or the Series only
contains `None/nan` objects, the type is set to `null`. This behavior can be avoided by constructing explicit
features and passing it to this function.

Important: a dataset created with from_pandas() lives in memory
and therefore doesn't have an associated cache directory.
This may change in the future, but in the meantime if you
want to reduce memory usage you should write it back on disk
and reload using e.g. save_to_disk / load_from_disk.

Example:

```py
>>> ds = Dataset.from_pandas(df)
```

**Parameters:**

df (`pandas.DataFrame`) : Dataframe that contains the dataset.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

info (`DatasetInfo`, *optional*) : Dataset information, like description, citation, etc.

split (`NamedSplit`, *optional*) : Name of the dataset split.

preserve_index (`bool`, *optional*) : Whether to store the index as an additional column in the resulting Dataset. The default of `None` will store the index as a column, except for `RangeIndex` which is stored as metadata only. Use `preserve_index=True` to force it to be stored as a column.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### from_dict[[datasets.Dataset.from_dict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L989)

Convert `dict` to a `pyarrow.Table` to create a [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset).

Important: a dataset created with from_dict() lives in memory
and therefore doesn't have an associated cache directory.
This may change in the future, but in the meantime if you
want to reduce memory usage you should write it back on disk
and reload using e.g. save_to_disk / load_from_disk.

Examples:

Get a Dataset from a dictionary containing one list per column:

```py
>>> ds = Dataset.from_dict({"text": ["hello there !", "general kenobi !"]})
```

Pass features to set the column types, e.g. for an image dataset:

```py
>>> features = Features({"image": Image()})
>>> ds = Dataset.from_dict({"image": ["path/to/image.png"]}, features=features)
```

Datasets are based on Arrow which is a columnar format, and therefore they expect every example to have the same
type and subtypes, and dictionaries to have the same keys and values types.
Loading a dataset errors out when fields have mismatching types, and fills missing fields in dictionaries with None so all dictionaries have the same keys and value types.

To avoid this and allow mixed-types without errors, you can use `on_mixed_types="use_json"` or specify `features=` with a [Json](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.Json) type:

```py
>>> ds = Dataset.from_dict({"a": [0, "foo", {"subfield": "bar"}]})
Traceback (most recent call last):
  ...
  File "pyarrow/error.pxi", line 92, in pyarrow.lib.check_status
pyarrow.lib.ArrowInvalid: Could not convert 'foo' with type str: tried to convert to int64

>>> ds = Dataset.from_dict({"a": [0, "foo", {"subfield": "bar"}]}, on_mixed_types="use_json")
>>> ds.features
{'a': Json()}
>>> list(ds["a"])
[0, "foo", {"subfield": "bar"}]

>>> features = Features({"a": Json()})
>>> ds = Dataset.from_dict({"a": [0, "foo", {"subfield": "bar"}]}, features=features)
>>> ds.features
{'a': Json()}
>>> list(ds["a"])
[0, "foo", {"subfield": "bar"}]
```

This is also useful for lists of dictionaries with arbitrary keys and values, to avoid filling missing fields with None:

```py
>>> ds = Dataset.from_dict({"a": [[{"b": 0}, {"c": 0}]]})
>>> ds.features
{'a': List({'b': Value('int64'), 'c': Value('int64')})}
>>> list(ds["a"])
[[{'b': 0, 'c': None}, {'b': None, 'c': 0}]]  # missing fields are filled with None

>>> features = Features({"a": List(Json())})
>>> ds = Dataset.from_dict({"a": [[{"b": 0}, {"c": 0}]]}, features=features)
>>> ds.features
{'a': List(Json())}
>>> list(ds["a"])
[[{'b': 0}, {'c': 0}]]  # OK

>>> ds = Dataset.from_dict({"a": [[{"b": 0}, {"c": 0}]]}, on_mixed_types="use_json")
>>> ds.features
{'a': List(Json())}
>>> list(ds["a"])
[[{'b': 0}, {'c': 0}]]  # OK
```

Another example with tool calling data:

```py
>>> messages = [
...     {"role": "user", "content": "Turn on the living room lights and play my electronic music playlist."},
...     {"role": "assistant", "tool_calls": [
...         {"type": "function", "function": {
...             "name": "control_light",
...             "arguments": {"room": "living room", "state": "on"}
...         }},
...         {"type": "function", "function": {
...             "name": "play_music",
...             "arguments": {"playlist": "electronic"}  # mixed-type here since keys ["playlist"] and ["room", "state"] are different
...         }}]
...     },
...     {"role": "tool", "name": "control_light", "content": "The lights in the living room are now on."},
...     {"role": "tool", "name": "play_music", "content": "The music is now playing."},
...     {"role": "assistant", "content": "Done!"}
... ]
>>> ds = Dataset.from_dict({"messages": [messages]}, on_mixed_types="use_json")
>>> ds.features
{'messages': List({'role': Value('string'), 'content': Value('string'), 'tool_calls': List(Json()), 'name': Value('string')})}
>>> ds[0]["messages"][1]["tool_calls"][0]["function"]["arguments"]
{"room": "living room", "state": "on"}
```

**Parameters:**

mapping (`Mapping`) : Mapping of strings to Arrays or Python lists.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

info (`DatasetInfo`, *optional*) : Dataset information, like description, citation, etc.

split (`NamedSplit`, *optional*) : Name of the dataset split.

on_mixed_types (`Literal["use_json"]`, *optional*, defaults to `None`) : If "use_json", use the Json() type for mixed-types fields, i.e. unstructured fields that contain data without a predefined schema. In this case, a field with mixed type is set to Json().  This allow loading lists with a mix of strings/integers/floats for example, or dictionaries with arbitrary value types.  

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### from_list[[datasets.Dataset.from_list]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L1155)

Convert a list of dicts to a `pyarrow.Table` to create a [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)`.

Note that the keys of the first entry will be used to determine the dataset columns,
regardless of what is passed to features.

Important: a dataset created with from_list() lives in memory
and therefore doesn't have an associated cache directory.
This may change in the future, but in the meantime if you
want to reduce memory usage you should write it back on disk
and reload using e.g. save_to_disk / load_from_disk.

Examples:

Get a Dataset from a list containing the examples:

```py
>>> ds = Dataset.from_list([{"text": "hello there !"}, {"text": "general kenobi !"}]})
```

Pass features to set the column types, e.g. for an image dataset:

```py
>>> features = Features({"image": Image()})
>>> ds = Dataset.from_list([{"image": "path/to/image.png"}], features=features)
```

Datasets are based on Arrow which is a columnar format, and therefore they expect every example to have the same
type and subtypes, and dictionaries to have the same keys and values types.
Loading a dataset errors out when fields have mismatching types, and fills missing fields in dictionaries with None so all dictionaries have the same keys and value types.

To avoid this and allow mixed-types without errors, you can use `on_mixed_types="use_json"` or specify `features=` with a [Json](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.Json) type:

```py
>>> ds = Dataset.from_list([{"a": 0}, {"a": "foo"}, {"a": {"subfield": "bar"}}])
Traceback (most recent call last):
  ...
  File "pyarrow/error.pxi", line 92, in pyarrow.lib.check_status
pyarrow.lib.ArrowInvalid: Could not convert 'foo' with type str: tried to convert to int64

>>> ds = Dataset.from_list([{"a": 0}, {"a": "foo"}, {"a": {"subfield": "bar"}}], on_mixed_types="use_json")
>>> ds.features
{'a': Json()}
>>> list(ds["a"])
[0, "foo", {"subfield": "bar"}]

>>> features = Features({"a": Json()})
>>> ds = Dataset.from_list([{"a": 0}, {"a": "foo"}, {"a": {"subfield": "bar"}}], features=features)
>>> ds.features
{'a': Json()}
>>> list(ds["a"])
[0, "foo", {"subfield": "bar"}]
```

This is also useful for lists of dictionaries with arbitrary keys and values, to avoid filling missing fields with None:

```py
>>> ds = Dataset.from_list([{"a": [{"b": 0}, {"c": 0}]}])
>>> ds.features
{'a': List({'b': Value('int64'), 'c': Value('int64')})}
>>> list(ds["a"])
[[{'b': 0, 'c': None}, {'b': None, 'c': 0}]]  # missing fields are filled with None

>>> features = Features({"a": List(Json())})
>>> ds = Dataset.from_list([{"a": [{"b": 0}, {"c": 0}]}], features=features)
>>> ds.features
{'a': List(Json())}
>>> list(ds["a"])
[[{'b': 0}, {'c': 0}]]  # OK

>>> ds = Dataset.from_list([{"a": [{"b": 0}, {"c": 0}]}], on_mixed_types="use_json")
>>> ds.features
{'a': List(Json())}
>>> list(ds["a"])
[[{'b': 0}, {'c': 0}]]  # OK
```

Another example with tool calling data:

```py
>>> messages = [
...     {"role": "user", "content": "Turn on the living room lights and play my electronic music playlist."},
...     {"role": "assistant", "tool_calls": [
...         {"type": "function", "function": {
...             "name": "control_light",
...             "arguments": {"room": "living room", "state": "on"}
...         }},
...         {"type": "function", "function": {
...             "name": "play_music",
...             "arguments": {"playlist": "electronic"}  # mixed-type here since keys ["playlist"] and ["room", "state"] are different
...         }}]
...     },
...     {"role": "tool", "name": "control_light", "content": "The lights in the living room are now on."},
...     {"role": "tool", "name": "play_music", "content": "The music is now playing."},
...     {"role": "assistant", "content": "Done!"}
... ]
>>> ds = Dataset.from_list([{"messages": messages}], on_mixed_types="use_json")
>>> ds.features
{'messages': List({'role': Value('string'), 'content': Value('string'), 'tool_calls': List(Json()), 'name': Value('string')})}
>>> ds[0]["messages"][1]["tool_calls"][0]["function"]["arguments"]
{"room": "living room", "state": "on"}
```

**Parameters:**

mapping (`List[dict]`) : A list of mappings of strings to row values.

features (`Features`, optional) : Dataset features.

info (`DatasetInfo`, optional) : Dataset information, like description, citation, etc.

split (`NamedSplit`, optional) : Name of the dataset split.

on_mixed_types (`Literal["use_json"]`, *optional*, defaults to `None`) : If "use_json", use the Json() type for mixed-types fields, i.e. unstructured fields that contain data without a predefined schema. In this case, a field with mixed type is set to Json().  This allow loading lists with a mix of strings/integers/floats for example, or dictionaries with arbitrary value types.  

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### from_generator[[datasets.Dataset.from_generator]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L1344)

Create a Dataset from a generator.

Load the data from the generator, cache the data in Arrow format on disk and return the Dataset from the memory-mapped Arrow data on disk.

Example:

```py
>>> def gen():
...     yield {"text": "Good", "label": 0}
...     yield {"text": "Bad", "label": 1}
...
>>> ds = Dataset.from_generator(gen)
```

```py
>>> def gen(shards):
...     for shard in shards:
...         with open(shard) as f:
...             for line in f:
...                 yield {"line": line}
...
>>> shards = [f"data{i}.txt" for i in range(32)]
>>> ds = Dataset.from_generator(gen, gen_kwargs={"shards": shards})
```

**Parameters:**

generator ( --`Callable`): A generator function that `yields` examples.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

cache_dir (`str`, *optional*, defaults to `"~/.cache/huggingface/datasets"`) : Directory to cache data.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

gen_kwargs(`dict`, *optional*) : Keyword arguments to be passed to the `generator` callable. You can define a sharded dataset by passing the list of shards in `gen_kwargs` and setting `num_proc` greater than 1.

num_proc (`int`, *optional*, defaults to `None`) : Number of processes when downloading and generating the dataset locally. This is helpful if the dataset is made of multiple files. Multiprocessing is disabled by default. If `num_proc` is greater than one, then all list values in `gen_kwargs` must be the same length. These values will be split between calls to the generator. The number of shards will be the minimum of the shortest list in `gen_kwargs` and `num_proc`.  

split ([NamedSplit](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.NamedSplit), defaults to `Split.TRAIN`) : Split name to be assigned to the dataset.  

fingerprint (`str`, *optional*) : Fingerprint that will be used to generate dataset ID. By default `fingerprint` is generated by hashing the generator function and all the args which can be slow if it uses large objects like AI models.  

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to :`GeneratorConfig`.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### data[[datasets.Dataset.data]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2105)

The Apache Arrow table backing the dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.data
MemoryMappedTable
text: string
label: int64
----
text: [["compassionately explores the seemingly irreconcilable situation between conservative christian parents and their estranged gay and lesbian children .","the soundtrack alone is worth the price of admission .","rodriguez does a splendid job of racial profiling hollywood style--casting excellent latin actors of all ages--a trend long overdue .","beneath the film's obvious determination to shock at any cost lies considerable skill and determination , backed by sheer nerve .","bielinsky is a filmmaker of impressive talent .","so beautifully acted and directed , it's clear that washington most certainly has a new career ahead of him if he so chooses .","a visual spectacle full of stunning images and effects .","a gentle and engrossing character study .","it's enough to watch huppert scheming , with her small , intelligent eyes as steady as any noir villain , and to enjoy the perfectly pitched web of tension that chabrol spins .","an engrossing portrait of uncompromising artists trying to create something original against the backdrop of a corporate music industry that only seems to care about the bottom line .",...,"ultimately , jane learns her place as a girl , softens up and loses some of the intensity that made her an interesting character to begin with .","ah-nuld's action hero days might be over .","it's clear why deuces wild , which was shot two years ago , has been gathering dust on mgm's shelf .","feels like nothing quite so much as a middle-aged moviemaker's attempt to surround himself with beautiful , half-naked women .","when the precise nature of matthew's predicament finally comes into sharp focus , the revelation fails to justify the build-up .","this picture is murder by numbers , and as easy to be bored by as your abc's , despite a few whopping shootouts .","hilarious musical comedy though stymied by accents thick as mud .","if you are into splatter movies , then you will probably have a reasonably good time with the salton sea .","a dull , simple-minded and stereotypical tale of drugs , death and mind-numbing indifference on the inner-city streets .","the feature-length stretch . . . strains the show's concept ."]]
label: [[1,1,1,1,1,1,1,1,1,1,...,0,0,0,0,0,0,0,0,0,0]]
```
#### cache_files[[datasets.Dataset.cache_files]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2125)

The cache files containing the Apache Arrow table backing the dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.cache_files
[{'filename': '/root/.cache/huggingface/datasets/rotten_tomatoes_movie_review/default/1.0.0/40d411e45a6ce3484deed7cc15b82a53dad9a72aafd9f86f8f227134bec5ca46/rotten_tomatoes_movie_review-validation.arrow'}]
```
#### num_columns[[datasets.Dataset.num_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2143)

Number of columns in the dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.num_columns
2
```
#### num_rows[[datasets.Dataset.num_rows]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2158)

Number of rows in the dataset (same as [Dataset.__len__()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.__len__)).

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.num_rows
1066
```
#### column_names[[datasets.Dataset.column_names]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2175)

Names of the columns in the dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.column_names
['text', 'label']
```
#### shape[[datasets.Dataset.shape]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2190)

Shape of the dataset (number of columns, number of rows).

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.shape
(1066, 2)
```
#### unique[[datasets.Dataset.unique]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2207)

Return a list of the unique elements in a column.

This is implemented in the low-level backend and as such, very fast.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.unique('label')
[1, 0]
```

**Parameters:**

column (`str`) : Column name (list all the column names with [column_names](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.column_names)).

**Returns:**

``list``

List of unique elements in the given column.
#### flatten[[datasets.Dataset.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2313)

Flatten the table.
Each column with a struct type is flattened into one column per struct field.
Other columns are left unchanged.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("rajpurkar/squad", split="train")
>>> ds.features
{'id': Value('string'),
 'title': Value('string'),
 'context': Value('string'),
 'question': Value('string'),
 'answers': {'text': List(Value('string')),
 'answer_start': List(Value('int32'))}}
>>> ds = ds.flatten()
>>> ds
Dataset({
    features: ['id', 'title', 'context', 'question', 'answers.text', 'answers.answer_start'],
    num_rows: 87599
})
```

**Parameters:**

new_fingerprint (`str`, *optional*) : The new fingerprint of the dataset after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments.

**Returns:**

`[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)`

A copy of the dataset with flattened columns.
#### cast[[datasets.Dataset.cast]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2360)

Cast the dataset to a new set of features.

Example:

```py
>>> from datasets import load_dataset, ClassLabel, Value
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.features
{'label': ClassLabel(names=['neg', 'pos']),
 'text': Value('string')}
>>> new_features = ds.features.copy()
>>> new_features['label'] = ClassLabel(names=['bad', 'good'])
>>> new_features['text'] = Value('large_string')
>>> ds = ds.cast(new_features)
>>> ds.features
{'label': ClassLabel(names=['bad', 'good']),
 'text': Value('large_string')}
```

**Parameters:**

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features)) : New features to cast the dataset to. The name of the fields in the features must match the current column names. The type of the data must also be convertible from one type to the other. For non-trivial conversion, e.g. `str`  `ClassLabel` you should use [map()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.map) to update the Dataset.

batch_size (`int`, defaults to `1000`) : Number of examples per batch provided to cast. If `batch_size >> from datasets import load_dataset, ClassLabel
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.features
{'label': ClassLabel(names=['neg', 'pos']),
 'text': Value('string')}
>>> ds = ds.cast_column('label', ClassLabel(names=['bad', 'good']))
>>> ds.features
{'label': ClassLabel(names=['bad', 'good']),
 'text': Value('string')}
```

**Parameters:**

column (`str`) : Column name.

feature (`FeatureType`) : Target feature.

new_fingerprint (`str`, *optional*) : The new fingerprint of the dataset after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### remove_columns[[datasets.Dataset.remove_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2487)

Remove one or several column(s) in the dataset and the features associated to them.

You can also remove a column using [map()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.map) with `remove_columns` but the present method
doesn't copy the data of the remaining columns and is thus faster.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds = ds.remove_columns('label')
Dataset({
    features: ['text'],
    num_rows: 1066
})
>>> ds = ds.remove_columns(column_names=ds.column_names) # Removing all the columns returns an empty dataset with the `num_rows` property set to 0
Dataset({
    features: [],
    num_rows: 0
})
```

**Parameters:**

column_names (`Union[str, List[str]]`) : Name of the column(s) to remove.

new_fingerprint (`str`, *optional*) : The new fingerprint of the dataset after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments.

**Returns:**

`[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)`

A copy of the dataset object without the columns to remove.
#### rename_column[[datasets.Dataset.rename_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2542)

Rename a column in the dataset, and move the features associated to the original column under the new column
name.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds = ds.rename_column('label', 'label_new')
Dataset({
    features: ['text', 'label_new'],
    num_rows: 1066
})
```

**Parameters:**

original_column_name (`str`) : Name of the column to rename.

new_column_name (`str`) : New name for the column.

new_fingerprint (`str`, *optional*) : The new fingerprint of the dataset after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments.

**Returns:**

`[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)`

A copy of the dataset with a renamed column.
#### rename_columns[[datasets.Dataset.rename_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2608)

Rename several columns in the dataset, and move the features associated to the original columns under
the new column names.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds = ds.rename_columns({'text': 'text_new', 'label': 'label_new'})
Dataset({
    features: ['text_new', 'label_new'],
    num_rows: 1066
})
```

**Parameters:**

column_mapping (`Dict[str, str]`) : A mapping of columns to rename to their new names

new_fingerprint (`str`, *optional*) : The new fingerprint of the dataset after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments.

**Returns:**

`[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)`

A copy of the dataset with renamed columns
#### select_columns[[datasets.Dataset.select_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2675)

Select one or several column(s) in the dataset and the features
associated to them.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds = ds.select_columns(['text'])
>>> ds
Dataset({
    features: ['text'],
    num_rows: 1066
})
```

**Parameters:**

column_names (`Union[str, List[str]]`) : Name of the column(s) to keep.

new_fingerprint (`str`, *optional*) : The new fingerprint of the dataset after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments.

**Returns:**

`[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)`

A copy of the dataset object which only consists of
selected columns.
#### class_encode_column[[datasets.Dataset.class_encode_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2238)

Casts the given column as [ClassLabel](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.ClassLabel) and updates the table.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("google/boolq", split="validation")
>>> ds.features
{'answer': Value('bool'),
 'passage': Value('string'),
 'question': Value('string')}
>>> ds = ds.class_encode_column('answer')
>>> ds.features
{'answer': ClassLabel(num_classes=2, names=['False', 'True']),
 'passage': Value('string'),
 'question': Value('string')}
```

**Parameters:**

column (`str`) : The name of the column to cast (list all the column names with [column_names](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.column_names))

include_nulls (`bool`, defaults to `False`) : Whether to include null values in the class labels. If `True`, the null values will be encoded as the `"None"` class label.  
#### __len__[[datasets.Dataset.__len__]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2731)

Number of rows in the dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.__len__

```
#### __iter__[[datasets.Dataset.__iter__]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2748)

Iterate through the examples.

If a formatting is set with [Dataset.set_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.set_format) rows will be returned with the
selected format.
#### iter[[datasets.Dataset.iter]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2777)

Iterate through the batches of size *batch_size*.

If a formatting is set with [*~datasets.Dataset.set_format*] rows will be returned with the
selected format.

**Parameters:**

batch_size (`int`) : size of each batch to yield.

drop_last_batch (`bool`, default *False*) : Whether a last batch smaller than the batch_size should be dropped
#### formatted_as[[datasets.Dataset.formatted_as]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2821)

To be used in a `with` statement. Set `__getitem__` return format (type and columns).

**Parameters:**

type (`str`, *optional*) : Either output type selected in `[None, 'numpy', 'torch', 'tensorflow', 'jax', 'arrow', 'pandas', 'polars']`. `None` means `__getitem__`` returns python objects (default).

columns (`List[str]`, *optional*) : Columns to format in the output. `None` means `__getitem__` returns all columns (default).

output_all_columns (`bool`, defaults to `False`) : Keep un-formatted columns as well in the output (as python objects).

- ****format_kwargs** (additional keyword arguments) : Keywords arguments passed to the convert function like `np.array`, `torch.tensor` or `tensorflow.ragged.constant`.
#### set_format[[datasets.Dataset.set_format]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2853)

Set `__getitem__` return format (type and columns). The data formatting is applied on-the-fly.
The format `type` (for example "numpy") is used to format batches when using `__getitem__`.
It's also possible to use custom transforms for formatting using [set_transform()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.set_transform).

It is possible to call [map()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.map) after calling `set_format`. Since `map` may add new columns, then the list of formatted columns

gets updated. In this case, if you apply `map` on a dataset to add a new column, then this column will be formatted as:

```
new formatted columns = (all columns - previously unformatted columns)
```

Example:

```py
>>> from datasets import load_dataset
>>> from transformers import AutoTokenizer
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> tokenizer = AutoTokenizer.from_pretrained("bert-base-cased")
>>> ds = ds.map(lambda x: tokenizer(x['text'], truncation=True, padding=True), batched=True)
>>> ds.set_format(type='numpy', columns=['text', 'label'])
>>> ds.format
{'type': 'numpy',
'format_kwargs': {},
'columns': ['text', 'label'],
'output_all_columns': False}
```

**Parameters:**

type (`str`, *optional*) : Either output type selected in `[None, 'numpy', 'torch', 'tensorflow', 'jax', 'arrow', 'pandas', 'polars']`. `None` means `__getitem__` returns python objects (default).

columns (`List[str]`, *optional*) : Columns to format in the output. `None` means `__getitem__` returns all columns (default).

output_all_columns (`bool`, defaults to `False`) : Keep un-formatted columns as well in the output (as python objects).

- ****format_kwargs** (additional keyword arguments) : Keywords arguments passed to the convert function like `np.array`, `torch.tensor` or `tensorflow.ragged.constant`.
#### set_transform[[datasets.Dataset.set_transform]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2961)

Set `__getitem__` return format using this transform. The transform is applied on-the-fly on batches when `__getitem__` is called.
As [set_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.set_format), this can be reset using [reset_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.reset_format).

Example:

```py
>>> from datasets import load_dataset
>>> from transformers import AutoTokenizer
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> tokenizer = AutoTokenizer.from_pretrained('bert-base-uncased')
>>> def encode(batch):
...     return tokenizer(batch['text'], padding=True, truncation=True, return_tensors='pt')
>>> ds.set_transform(encode)
>>> ds[0]
{'attention_mask': tensor([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
 1, 1]),
 'input_ids': tensor([  101, 29353,  2135, 15102,  1996,  9428, 20868,  2890,  8663,  6895,
         20470,  2571,  3663,  2090,  4603,  3017,  3008,  1998,  2037, 24211,
         5637,  1998, 11690,  2336,  1012,   102]),
 'token_type_ids': tensor([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0])}
```

**Parameters:**

transform (`Callable`, *optional*) : User-defined formatting transform, replaces the format defined by [set_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.set_format). A formatting function is a callable that takes a batch (as a `dict`) as input and returns a batch. This function is applied right before returning the objects in `__getitem__`.

columns (`List[str]`, *optional*) : Columns to format in the output. If specified, then the input batch of the transform only contains those columns.

output_all_columns (`bool`, defaults to `False`) : Keep un-formatted columns as well in the output (as python objects). If set to True, then the other un-formatted columns are kept with the output of the transform.
#### reset_format[[datasets.Dataset.reset_format]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L2932)

Reset `__getitem__` return format to python objects and all columns.

Same as `self.set_format()`

Example:

```py
>>> from datasets import load_dataset
>>> from transformers import AutoTokenizer
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> tokenizer = AutoTokenizer.from_pretrained("bert-base-cased")
>>> ds = ds.map(lambda x: tokenizer(x['text'], truncation=True, padding=True), batched=True)
>>> ds.set_format(type='numpy', columns=['input_ids', 'token_type_ids', 'attention_mask', 'label'])
>>> ds.format
{'columns': ['input_ids', 'token_type_ids', 'attention_mask', 'label'],
 'format_kwargs': {},
 'output_all_columns': False,
 'type': 'numpy'}
>>> ds.reset_format()
>>> ds.format
{'columns': ['text', 'label', 'input_ids', 'token_type_ids', 'attention_mask'],
 'format_kwargs': {},
 'output_all_columns': False,
 'type': None}
```
#### with_format[[datasets.Dataset.with_format]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L3004)

Set `__getitem__` return format (type and columns). The data formatting is applied on-the-fly.
The format `type` (for example "numpy") is used to format batches when using `__getitem__`.

It's also possible to use custom transforms for formatting using [with_transform()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.with_transform).

Contrary to [set_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.set_format), `with_format` returns a new [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) object.

Example:

```py
>>> from datasets import load_dataset
>>> from transformers import AutoTokenizer
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> tokenizer = AutoTokenizer.from_pretrained("bert-base-cased")
>>> ds = ds.map(lambda x: tokenizer(x['text'], truncation=True, padding=True), batched=True)
>>> ds.format
{'columns': ['text', 'label', 'input_ids', 'token_type_ids', 'attention_mask'],
 'format_kwargs': {},
 'output_all_columns': False,
 'type': None}
>>> ds = ds.with_format("torch")
>>> ds.format
{'columns': ['text', 'label', 'input_ids', 'token_type_ids', 'attention_mask'],
 'format_kwargs': {},
 'output_all_columns': False,
 'type': 'torch'}
>>> ds[0]
{'text': 'compassionately explores the seemingly irreconcilable situation between conservative christian parents and their estranged gay and lesbian children .',
 'label': tensor(1),
 'input_ids': tensor([  101, 18027, 16310, 16001,  1103,  9321,   178, 11604,  7235,  6617,
        1742,  2165,  2820,  1206,  6588, 22572, 12937,  1811,  2153,  1105,
        1147, 12890, 19587,  6463,  1105, 15026,  1482,   119,   102,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0]),
 'token_type_ids': tensor([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
 'attention_mask': tensor([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])}
```

**Parameters:**

type (`str`, *optional*) : Either output type selected in `[None, 'numpy', 'torch', 'tensorflow', 'jax', 'arrow', 'pandas', 'polars']`. `None` means `__getitem__` returns python objects (default).

columns (`List[str]`, *optional*) : Columns to format in the output. `None` means `__getitem__` returns all columns (default).

output_all_columns (`bool`, defaults to `False`) : Keep un-formatted columns as well in the output (as python objects).

- ****format_kwargs** (additional keyword arguments) : Keywords arguments passed to the convert function like `np.array`, `torch.tensor` or `tensorflow.ragged.constant`.
#### with_transform[[datasets.Dataset.with_transform]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L3075)

Set `__getitem__` return format using this transform. The transform is applied on-the-fly on batches when `__getitem__` is called.

As [set_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.set_format), this can be reset using [reset_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.reset_format).

Contrary to [set_transform()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.set_transform), `with_transform` returns a new [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) object.

Example:

```py
>>> from datasets import load_dataset
>>> from transformers import AutoTokenizer
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> tokenizer = AutoTokenizer.from_pretrained("bert-base-cased")
>>> def encode(example):
...     return tokenizer(example["text"], padding=True, truncation=True, return_tensors='pt')
>>> ds = ds.with_transform(encode)
>>> ds[0]
{'attention_mask': tensor([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
 1, 1, 1, 1, 1]),
 'input_ids': tensor([  101, 18027, 16310, 16001,  1103,  9321,   178, 11604,  7235,  6617,
         1742,  2165,  2820,  1206,  6588, 22572, 12937,  1811,  2153,  1105,
         1147, 12890, 19587,  6463,  1105, 15026,  1482,   119,   102]),
 'token_type_ids': tensor([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0])}
```

**Parameters:**

transform (`Callable`, `optional`) : User-defined formatting transform, replaces the format defined by [set_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.set_format). A formatting function is a callable that takes a batch (as a `dict`) as input and returns a batch. This function is applied right before returning the objects in `__getitem__`.

columns (`List[str]`, `optional`) : Columns to format in the output. If specified, then the input batch of the transform only contains those columns.

output_all_columns (`bool`, defaults to `False`) : Keep un-formatted columns as well in the output (as python objects). If set to `True`, then the other un-formatted columns are kept with the output of the transform.
#### __getitem__[[datasets.Dataset.__getitem__]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L3151)

Can be used to index columns (by string names) or rows (by integer index or iterable of indices or bools).
#### cleanup_cache_files[[datasets.Dataset.cleanup_cache_files]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L3164)

Clean up all cache files in the dataset cache directory, excepted the currently used cache file if there is
one.

Be careful when running this command that no other process is currently using other cache files.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds.cleanup_cache_files()
10
```

**Returns:**

``int``

Number of removed files.
#### map[[datasets.Dataset.map]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L3211)

Apply a function to all the examples in the table (individually or in batches) and update the table.
If your function returns a column that already exists, then it overwrites it.

You can specify whether the function should be batched or not with the `batched` parameter:

- If batched is `False`, then the function takes 1 example in and should return 1 example.
  An example is a dictionary, e.g. `{"text": "Hello there !"}`.
- If batched is `True` and `batch_size` is 1, then the function takes a batch of 1 example as input and can return a batch with 1 or more examples.
  A batch is a dictionary, e.g. a batch of 1 example is `{"text": ["Hello there !"]}`.
- If batched is `True` and `batch_size` is `n > 1`, then the function takes a batch of `n` examples as input and can return a batch with `n` examples, or with an arbitrary number of examples.
  Note that the last batch may have less than `n` examples.
  A batch is a dictionary, e.g. a batch of `n` examples is `{"text": ["Hello there !"] * n}`.

If the function is asynchronous, then `map` will run your function in parallel, with up to one thousand simultaneous calls.
It is recommended to use a `asyncio.Semaphore` in your function if you want to set a maximum number of operations that can run at the same time.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> def add_prefix(example):
...     example["text"] = "Review: " + example["text"]
...     return example
>>> ds = ds.map(add_prefix)
>>> ds[0:3]["text"]
['Review: compassionately explores the seemingly irreconcilable situation between conservative christian parents and their estranged gay and lesbian children .',
 'Review: the soundtrack alone is worth the price of admission .',
 'Review: rodriguez does a splendid job of racial profiling hollywood style--casting excellent latin actors of all ages--a trend long overdue .']

# process a batch of examples
>>> ds = ds.map(lambda example: tokenizer(example["text"]), batched=True)
# set number of processors
>>> ds = ds.map(add_prefix, num_proc=4)
```

**Parameters:**

function (`Callable`) : Function with one of the following signatures:  - `function(example: Dict[str, Any]) -> Dict[str, Any]` if `batched=False` and `with_indices=False` and `with_rank=False` - `function(example: Dict[str, Any], *extra_args) -> Dict[str, Any]` if `batched=False` and `with_indices=True` and/or `with_rank=True` (one extra arg for each) - `function(batch: Dict[str, List]) -> Dict[str, List]` if `batched=True` and `with_indices=False` and `with_rank=False` - `function(batch: Dict[str, List], *extra_args) -> Dict[str, List]` if `batched=True` and `with_indices=True` and/or `with_rank=True` (one extra arg for each)  For advanced usage, the function can also return a `pyarrow.Table`. If the function is asynchronous, then `map` will run your function in parallel. Moreover if your function returns nothing (`None`), then `map` will run your function and return the dataset unchanged. If no function is provided, default to identity function: `lambda x: x`.

with_indices (`bool`, defaults to `False`) : Provide example indices to `function`. Note that in this case the signature of `function` should be `def function(example, idx[, rank]): ...`.

with_rank (`bool`, defaults to `False`) : Provide process rank to `function`. Note that in this case the signature of `function` should be `def function(example[, idx], rank): ...`.

input_columns (`Optional[Union[str, List[str]]]`, defaults to `None`) : The columns to be passed into `function` as positional arguments. If `None`, a `dict` mapping to all formatted columns is passed as one argument.

batched (`bool`, defaults to `False`) : Provide batch of examples to `function`.

batch_size (`int`, *optional*, defaults to `1000`) : Number of examples per batch provided to `function` if `batched=True`. If `batch_size  int32). Set to False if you want to always infer new types.

on_mixed_types (`Literal["use_json"]`, *optional*, defaults to `None`) : If "use_json", use the Json() type for mixed-types fields, i.e. unstructured fields that contain data without a predefined schema. In this case, a field with mixed type is set to Json().  This allow loading lists with a mix of strings/integers/floats for example, or dictionaries with arbitrary value types.  
#### filter[[datasets.Dataset.filter]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L4112)

Apply a filter function to all the elements in the table in batches
and update the table so that the dataset only includes examples according to the filter function.

If the function is asynchronous, then `filter` will run your function in parallel, with up to one thousand simultaneous calls (configurable).
It is recommended to use a `asyncio.Semaphore` in your function if you want to set a maximum number of operations that can run at the same time.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds = ds.filter(lambda x: x["label"] == 1)
>>> ds
Dataset({
    features: ['text', 'label'],
    num_rows: 533
})
```

**Parameters:**

function (`Callable`) : Callable with one of the following signatures:  - `function(example: Dict[str, Any]) -> bool` if `batched=False` and `with_indices=False` and `with_rank=False` - `function(example: Dict[str, Any], *extra_args) -> bool` if `batched=False` and `with_indices=True` and/or `with_rank=True` (one extra arg for each) - `function(batch: Dict[str, List]) -> List[bool]` if `batched=True` and `with_indices=False` and `with_rank=False` - `function(batch: Dict[str, List], *extra_args) -> List[bool]` if `batched=True` and `with_indices=True` and/or `with_rank=True` (one extra arg for each)  If the function is asynchronous, then `filter` will run your function in parallel. If no function is provided, defaults to an always `True` function: `lambda x: True`.

with_indices (`bool`, defaults to `False`) : Provide example indices to `function`. Note that in this case the signature of `function` should be `def function(example, idx[, rank]): ...`.

with_rank (`bool`, defaults to `False`) : Provide process rank to `function`. Note that in this case the signature of `function` should be `def function(example[, idx], rank): ...`.

input_columns (`str` or `List[str]`, *optional*) : The columns to be passed into `function` as positional arguments. If `None`, a `dict` mapping to all formatted columns is passed as one argument.

batched (`bool`, defaults to `False`) : Provide batch of examples to `function`.

batch_size (`int`, *optional*, defaults to `1000`) : Number of examples per batch provided to `function` if `batched = True`. If `batched = False`, one example per batch is passed to `function`. If `batch_size >> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds = ds.select(range(4))
>>> ds
Dataset({
    features: ['text', 'label'],
    num_rows: 4
})
```

**Parameters:**

indices (`range`, `list`, `iterable`, `ndarray` or `Series`) : Range, list or 1D-array of integer indices for indexing. If the indices correspond to a contiguous range, the Arrow table is simply sliced. However passing a list of indices that are not contiguous creates indices mapping, which is much less efficient, but still faster than recreating an Arrow table made of the requested rows.

keep_in_memory (`bool`, defaults to `False`) : Keep the indices mapping in memory instead of writing it to a cache file.

indices_cache_file_name (`str`, *optional*, defaults to `None`) : Provide the name of a path for the cache file. It is used to store the indices mapping instead of the automatically generated cache file name.

writer_batch_size (`int`, defaults to `1000`) : Number of rows per write operation for the cache file writer. This value is a good trade-off between memory usage during the processing, and processing speed. Higher value makes the processing do fewer lookups, lower value consume less temporary memory while running `map`.

new_fingerprint (`str`, *optional*, defaults to `None`) : The new fingerprint of the dataset after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments.
#### sort[[datasets.Dataset.sort]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L4677)

Create a new dataset sorted according to a single or multiple columns.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset('cornell-movie-review-data/rotten_tomatoes', split='validation')
>>> ds['label'][:10]
[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
>>> sorted_ds = ds.sort('label')
>>> sorted_ds['label'][:10]
[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
>>> another_sorted_ds = ds.sort(['label', 'text'], reverse=[True, False])
>>> another_sorted_ds['label'][:10]
[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
```

**Parameters:**

column_names (`Union[str, Sequence[str]]`) : Column name(s) to sort by.

reverse (`Union[bool, Sequence[bool]]`, defaults to `False`) : If `True`, sort by descending order rather than ascending. If a single bool is provided, the value is applied to the sorting of all column names. Otherwise a list of bools with the same length and order as column_names must be provided.

null_placement (`str`, defaults to `at_end`) : Put `None` values at the beginning if `at_start` or `first` or at the end if `at_end` or `last`  

keep_in_memory (`bool`, defaults to `False`) : Keep the sorted indices in memory instead of writing it to a cache file.

load_from_cache_file (`Optional[bool]`, defaults to `True` if caching is enabled) : If a cache file storing the sorted indices can be identified, use it instead of recomputing.

indices_cache_file_name (`str`, *optional*, defaults to `None`) : Provide the name of a path for the cache file. It is used to store the sorted indices instead of the automatically generated cache file name.

writer_batch_size (`int`, defaults to `1000`) : Number of rows per write operation for the cache file writer. Higher value gives smaller cache files, lower value consume less temporary memory.

new_fingerprint (`str`, *optional*, defaults to `None`) : The new fingerprint of the dataset after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments
#### shuffle[[datasets.Dataset.shuffle]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L4805)

Create a new Dataset where the rows are shuffled.

Currently shuffling uses numpy random generators.
You can either supply a NumPy BitGenerator to use, or a seed to initiate NumPy's default random generator (PCG64).

Shuffling takes the list of indices `[0:len(my_dataset)]` and shuffles it to create an indices mapping.
However as soon as your [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) has an indices mapping, the speed can become 10x slower.
This is because there is an extra step to get the row index to read using the indices mapping, and most importantly, you aren't reading contiguous chunks of data anymore.
To restore the speed, you'd need to rewrite the entire dataset on your disk again using [Dataset.flatten_indices()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.flatten_indices), which removes the indices mapping.

This may take a lot of time depending of the size of your dataset though:

```python
my_dataset[0]  # fast
my_dataset = my_dataset.shuffle(seed=42)
my_dataset[0]  # up to 10x slower
my_dataset = my_dataset.flatten_indices()  # rewrite the shuffled dataset on disk as contiguous chunks of data
my_dataset[0]  # fast again
```

In this case, we recommend switching to an [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset) and leveraging its fast approximate shuffling method [IterableDataset.shuffle()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset.shuffle).

It only shuffles the shards order and adds a shuffle buffer to your dataset, which keeps the speed of your dataset optimal:

```python
my_iterable_dataset = my_dataset.to_iterable_dataset(num_shards=128)
for example in enumerate(my_iterable_dataset):  # fast
    pass

shuffled_iterable_dataset = my_iterable_dataset.shuffle(seed=42, buffer_size=100)

for example in enumerate(shuffled_iterable_dataset):  # as fast as before
    pass
```

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds['label'][:10]
[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]

# set a seed
>>> shuffled_ds = ds.shuffle(seed=42)
>>> shuffled_ds['label'][:10]
[1, 0, 1, 1, 0, 0, 0, 0, 0, 0]
```

**Parameters:**

seed (`int`, *optional*) : A seed to initialize the default BitGenerator if `generator=None`. If `None`, then fresh, unpredictable entropy will be pulled from the OS. If an `int` or `array_like[ints]` is passed, then it will be passed to SeedSequence to derive the initial BitGenerator state.

generator (`numpy.random.Generator`, *optional*) : Numpy random Generator to use to compute the permutation of the dataset rows. If `generator=None` (default), uses `np.random.default_rng` (the default BitGenerator (PCG64) of NumPy).

keep_in_memory (`bool`, default `False`) : Keep the shuffled indices in memory instead of writing it to a cache file.

load_from_cache_file (`Optional[bool]`, defaults to `True` if caching is enabled) : If a cache file storing the shuffled indices can be identified, use it instead of recomputing.

indices_cache_file_name (`str`, *optional*) : Provide the name of a path for the cache file. It is used to store the shuffled indices instead of the automatically generated cache file name.

writer_batch_size (`int`, defaults to `1000`) : Number of rows per write operation for the cache file writer. This value is a good trade-off between memory usage during the processing, and processing speed. Higher value makes the processing do fewer lookups, lower value consume less temporary memory while running `map`.

new_fingerprint (`str`, *optional*, defaults to `None`) : The new fingerprint of the dataset after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments.
#### skip[[datasets.Dataset.skip]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L4592)

Create a new [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) that skips the first `n` elements.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train")
>>> list(ds.take(3))
[{'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'effective but too-tepid biopic'}]
>>> ds = ds.skip(1)
>>> list(ds.take(3))
[{'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'effective but too-tepid biopic'},
 {'label': 1,
 'text': 'if you sometimes like to go to the movies to have fun , wasabi is a good place to start .'}]
```

**Parameters:**

n (`int`) : Number of elements to skip.
#### take[[datasets.Dataset.take]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L4654)

Create a new [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) with only the first `n` elements.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train")
>>> small_ds = ds.take(2)
>>> list(small_ds)
[{'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'}]
```

**Parameters:**

n (`int`) : Number of elements to take.
#### train_test_split[[datasets.Dataset.train_test_split]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L4937)

Return a dictionary ([datasets.DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)) with two random train and test subsets (`train` and `test` `Dataset` splits).
Splits are created from the dataset according to `test_size`, `train_size` and `shuffle`.

This method is similar to scikit-learn `train_test_split`.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds = ds.train_test_split(test_size=0.2, shuffle=True)
DatasetDict({
    train: Dataset({
        features: ['text', 'label'],
        num_rows: 852
    })
    test: Dataset({
        features: ['text', 'label'],
        num_rows: 214
    })
})

# set a seed
>>> ds = ds.train_test_split(test_size=0.2, seed=42)

# stratified split
>>> ds = load_dataset("stanfordnlp/imdb",split="train")
Dataset({
    features: ['text', 'label'],
    num_rows: 25000
})
>>> ds = ds.train_test_split(test_size=0.2, stratify_by_column="label")
DatasetDict({
    train: Dataset({
        features: ['text', 'label'],
        num_rows: 20000
    })
    test: Dataset({
        features: ['text', 'label'],
        num_rows: 5000
    })
})
```

**Parameters:**

test_size (`Union[float, int, None]`, *optional*) : Size of the test split If `float`, should be between `0.0` and `1.0` and represent the proportion of the dataset to include in the test split. If `int`, represents the absolute number of test samples. If `None`, the value is set to the complement of the train size. If `train_size` is also `None`, it will be set to `0.25`.

train_size (`Union[float, int, None]`, *optional*) : Size of the train split If `float`, should be between `0.0` and `1.0` and represent the proportion of the dataset to include in the train split. If `int`, represents the absolute number of train samples. If `None`, the value is automatically set to the complement of the test size.

shuffle (`bool`, *optional*, defaults to `True`) : Whether or not to shuffle the data before splitting.

stratify_by_column (`str`, *optional*, defaults to `None`) : The column name of labels to be used to perform stratified split of data.

seed (`int`, *optional*) : A seed to initialize the default BitGenerator if `generator=None`. If `None`, then fresh, unpredictable entropy will be pulled from the OS. If an `int` or `array_like[ints]` is passed, then it will be passed to SeedSequence to derive the initial BitGenerator state.

generator (`numpy.random.Generator`, *optional*) : Numpy random Generator to use to compute the permutation of the dataset rows. If `generator=None` (default), uses `np.random.default_rng` (the default BitGenerator (PCG64) of NumPy).

keep_in_memory (`bool`, defaults to `False`) : Keep the splits indices in memory instead of writing it to a cache file.

load_from_cache_file (`Optional[bool]`, defaults to `True` if caching is enabled) : If a cache file storing the splits indices can be identified, use it instead of recomputing.

train_cache_file_name (`str`, *optional*) : Provide the name of a path for the cache file. It is used to store the train split indices instead of the automatically generated cache file name.

test_cache_file_name (`str`, *optional*) : Provide the name of a path for the cache file. It is used to store the test split indices instead of the automatically generated cache file name.

writer_batch_size (`int`, defaults to `1000`) : Number of rows per write operation for the cache file writer. This value is a good trade-off between memory usage during the processing, and processing speed. Higher value makes the processing do fewer lookups, lower value consume less temporary memory while running `map`.

train_new_fingerprint (`str`, *optional*, defaults to `None`) : The new fingerprint of the train set after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments

test_new_fingerprint (`str`, *optional*, defaults to `None`) : The new fingerprint of the test set after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments
#### shard[[datasets.Dataset.shard]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L5220)

Return the `index`-nth shard from dataset split into `num_shards` pieces.

This shards deterministically. `dataset.shard(n, i)` splits the dataset into contiguous chunks,
so it can be easily concatenated back together after processing. If `len(dataset) % n == l`, then the
first `l` dataset each have length `(len(dataset) // n) + 1`, and the remaining dataset have length `(len(dataset) // n)`.
`datasets.concatenate_datasets([dset.shard(n, i) for i in range(n)])` returns a dataset with the same order as the original.

Note: n should be less or equal to the number of elements in the dataset `len(dataset)`.

On the other hand, `dataset.shard(n, i, contiguous=False)` contains all elements of the dataset whose index mod `n = i`.

Be sure to shard before using any randomizing operator (such as `shuffle`).
It is best if the shard operator is used early in the dataset pipeline.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation")
>>> ds
Dataset({
    features: ['text', 'label'],
    num_rows: 1066
})
>>> ds = ds.shard(num_shards=2, index=0)
>>> ds
Dataset({
    features: ['text', 'label'],
    num_rows: 533
})
```

**Parameters:**

num_shards (`int`) : How many shards to split the dataset into.

index (`int`) : Which shard to select and return.

contiguous : (`bool`, defaults to `True`): Whether to select contiguous blocks of indices for shards.

keep_in_memory (`bool`, defaults to `False`) : Keep the dataset in memory instead of writing it to a cache file.

indices_cache_file_name (`str`, *optional*) : Provide the name of a path for the cache file. It is used to store the indices of each shard instead of the automatically generated cache file name.

writer_batch_size (`int`, defaults to `1000`) : This only concerns the indices mapping. Number of indices per write operation for the cache file writer. This value is a good trade-off between memory usage during the processing, and processing speed. Higher value makes the processing do fewer lookups, lower value consume less temporary memory while running `map`.
#### repeat[[datasets.Dataset.repeat]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L4622)

Create a new [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) that repeats the underlying dataset `num_times` times.

Like itertools.repeat, repeating once just returns the full dataset.

Example:
```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train")
>>> ds = ds.take(2).repeat(2)
>>> list(ds)
[{'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'effective but too-tepid biopic'},
 {'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'effective but too-tepid biopic'}]
```

**Parameters:**

num_times (`int`) : Number of times to repeat the dataset.
#### to_tf_dataset[[datasets.Dataset.to_tf_dataset]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L343)

Create a `tf.data.Dataset` from the underlying Dataset. This `tf.data.Dataset` will load and collate batches from
the Dataset, and is suitable for passing to methods like `model.fit()` or `model.predict()`. The dataset will yield
`dicts` for both inputs and labels unless the `dict` would contain only a single key, in which case a raw
`tf.Tensor` is yielded instead.

Example:

```py
>>> ds_train = ds["train"].to_tf_dataset(
...    columns=['input_ids', 'token_type_ids', 'attention_mask', 'label'],
...    shuffle=True,
...    batch_size=16,
...    collate_fn=data_collator,
... )
```

**Parameters:**

batch_size (`int`, *optional*) : Size of batches to load from the dataset. Defaults to `None`, which implies that the dataset won't be batched, but the returned dataset can be batched later with `tf_dataset.batch(batch_size)`.

columns (`List[str]` or `str`, *optional*) : Dataset column(s) to load in the `tf.data.Dataset`. Column names that are created by the `collate_fn` and that do not exist in the original dataset can be used.

shuffle(`bool`, defaults to `False`) : Shuffle the dataset order when loading. Recommended `True` for training, `False` for validation/evaluation.

drop_remainder(`bool`, defaults to `False`) : Drop the last incomplete batch when loading. Ensures that all batches yielded by the dataset will have the same length on the batch dimension.

collate_fn(`Callable`, *optional*) : A function or callable object (such as a `DataCollator`) that will collate lists of samples into a batch.

collate_fn_args (`Dict`, *optional*) : An optional `dict` of keyword arguments to be passed to the `collate_fn`.

label_cols (`List[str]` or `str`, defaults to `None`) : Dataset column(s) to load as labels. Note that many models compute loss internally rather than letting Keras do it, in which case passing the labels here is optional, as long as they're in the input `columns`.

prefetch (`bool`, defaults to `True`) : Whether to run the dataloader in a separate thread and maintain a small buffer of batches for training. Improves performance by allowing data to be loaded in the background while the model is training.

num_workers (`int`, defaults to `0`) : Number of workers to use for loading the dataset.

num_test_batches (`int`, defaults to `20`) : Number of batches to use to infer the output signature of the dataset. The higher this number, the more accurate the signature will be, but the longer it will take to create the dataset.

**Returns:**

`tf.data.Dataset`
#### push_to_hub[[datasets.Dataset.push_to_hub]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L6001)

Pushes the dataset to the hub as a Parquet dataset.
The dataset is pushed using HTTP requests and does not need to have neither git or git-lfs installed.

The resulting Parquet files are self-contained by default. If your dataset contains [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image), [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio) or [Video](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Video)
data, the Parquet files will store the bytes of your images or audio files.
You can disable this by setting `embed_external_files` to `False`.

Example:

```python
>>> dataset.push_to_hub("/")
>>> dataset_dict.push_to_hub("/", private=True)
>>> dataset.push_to_hub("/", max_shard_size="1GB")
>>> dataset.push_to_hub("/", num_shards=1024)
```

If your dataset has multiple splits (e.g. train/validation/test):

```python
>>> train_dataset.push_to_hub("/", split="train")
>>> val_dataset.push_to_hub("/", split="validation")
>>> # later
>>> dataset = load_dataset("/")
>>> train_dataset = dataset["train"]
>>> val_dataset = dataset["validation"]
```

If you want to add a new configuration (or subset) to a dataset (e.g. if the dataset has multiple tasks/versions/languages):

```python
>>> english_dataset.push_to_hub("/", "en")
>>> french_dataset.push_to_hub("/", "fr")
>>> # later
>>> english_dataset = load_dataset("/", "en")
>>> french_dataset = load_dataset("/", "fr")
```

**Parameters:**

repo_id (`str`) : The ID of the repository to push to in the following format: `/` or `/`. Also accepts ``, which will default to the namespace of the logged-in user.  It could also be a location inside a bucket, e.g. `buckets///...`

config_name (`str`, defaults to "default") : The configuration name (or subset) of a dataset. Defaults to "default".

set_default (`bool`, *optional*) : Whether to set this configuration as the default one. Otherwise, the default configuration is the one named "default".

split (`str`, *optional*) : The name of the split that will be given to that dataset. Defaults to `self.split`.

data_dir (`str`, *optional*) : Directory name that will contain the uploaded data files. Defaults to the `config_name` if different from "default", else "data".  

commit_message (`str`, *optional*) : Message to commit while pushing. Will default to `"Upload dataset"`.

commit_description (`str`, *optional*) : Description of the commit that will be created. Additionally, description of the PR if a PR is created (`create_pr` is True).  

private (`bool`, *optional*) : Whether to make the repo private. If `None` (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

token (`str`, *optional*) : An optional authentication token for the Hugging Face Hub. If no token is passed, will default to the token saved locally when logging in with `huggingface-cli login`. Will raise an error if no token is passed and the user is not logged-in.

revision (`str`, *optional*) : Branch to push the uploaded files to. Defaults to the `"main"` branch.  

create_pr (`bool`, *optional*, defaults to `False`) : Whether to create a PR with the uploaded files or directly commit.  

max_shard_size (`int` or `str`, *optional*, defaults to `"500MB"`) : The maximum size of the dataset shards to be uploaded to the hub. If expressed as a string, needs to be digits followed by a unit (like `"5MB"`).

num_shards (`int`, *optional*) : Number of shards to write. By default, the number of shards depends on `max_shard_size`.  

embed_external_files (`bool`, defaults to `True`) : Whether to embed file bytes in the shards. In particular, this will do the following before the push for the fields of type:  - [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio) and [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image): remove local path information and embed file content in the Parquet files.

num_proc (`int`, *optional*, defaults to `None`) : Number of processes when preparing and uploading the dataset. This is helpful if the dataset is made of many samples or media files to embed. I uses "spawn" context to work with hf_xet, the rust client for fast uploads to HF. Multiprocessing is disabled by default.  

**Returns:**

huggingface_hub.CommitInfo
#### save_to_disk[[datasets.Dataset.save_to_disk]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L1788)

Saves a dataset to a dataset directory, or in a filesystem using any implementation of `fsspec.spec.AbstractFileSystem`.

For [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image), [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio) and [Video](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Video) data:

All the Image(), Audio() and Video() data are stored in the arrow files.
If you want to store paths or urls, please use the Value("string") type.

Example:

```py
>>> ds.save_to_disk("path/to/dataset/directory")
>>> ds.save_to_disk("path/to/dataset/directory", max_shard_size="1GB")
>>> ds.save_to_disk("path/to/dataset/directory", num_shards=1024)
```

**Parameters:**

dataset_path (`path-like`) : Path (e.g. `dataset/train`) or remote URI (e.g. `s3://my-bucket/dataset/train`) of the dataset directory where the dataset will be saved to.

max_shard_size (`int` or `str`, *optional*, defaults to `"500MB"`) : The maximum size of the dataset shards to be saved to the filesystem. If expressed as a string, needs to be digits followed by a unit (like `"50MB"`).

num_shards (`int`, *optional*) : Number of shards to write. By default the number of shards depends on `max_shard_size` and `num_proc`.  

num_proc (`int`, *optional*) : Number of processes when downloading and generating the dataset locally. Multiprocessing is disabled by default.  

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  
#### load_from_disk[[datasets.Dataset.load_from_disk]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L1984)

Loads a dataset that was previously saved using `save_to_disk` from a dataset directory, or from a
filesystem using any implementation of `fsspec.spec.AbstractFileSystem`.

Example:

```py
>>> ds = load_from_disk("path/to/dataset/directory")
```

**Parameters:**

dataset_path (`path-like`) : Path (e.g. `"dataset/train"`) or remote URI (e.g. `"s3//my-bucket/dataset/train"`) of the dataset directory where the dataset will be loaded from.

keep_in_memory (`bool`, defaults to `None`) : Whether to copy the dataset in-memory. If `None`, the dataset will not be copied in-memory unless explicitly enabled by setting `datasets.config.IN_MEMORY_MAX_SIZE` to nonzero. See more details in the [improve performance](../cache#improve-performance) section.

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  

**Returns:**

`[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) or [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)`

- If `dataset_path` is a path of a dataset directory, the dataset requested.
- If `dataset_path` is a path of a dataset dict directory, a `datasets.DatasetDict` with each split.
#### flatten_indices[[datasets.Dataset.flatten_indices]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L4260)

Create and cache a new Dataset by flattening the indices mapping.

**Parameters:**

keep_in_memory (`bool`, defaults to `False`) : Keep the dataset in memory instead of writing it to a cache file.

cache_file_name (`str`, *optional*, default `None`) : Provide the name of a path for the cache file. It is used to store the results of the computation instead of the automatically generated cache file name.

writer_batch_size (`int`, defaults to `1000`) : Number of rows per write operation for the cache file writer. This value is a good trade-off between memory usage during the processing, and processing speed. Higher value makes the processing do fewer lookups, lower value consume less temporary memory while running `map`.

features (`Optional[datasets.Features]`, defaults to `None`) : Use a specific [Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features) to store the cache file instead of the automatically generated one.

disable_nullable (`bool`, defaults to `False`) : Allow null values in the table.

num_proc (`int`, optional, default `None`) : Max number of processes when generating cache. Already cached shards are loaded sequentially

new_fingerprint (`str`, *optional*, defaults to `None`) : The new fingerprint of the dataset after transform. If `None`, the new fingerprint is computed using a hash of the previous fingerprint, and the transform arguments
#### to_csv[[datasets.Dataset.to_csv]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L5297)

Exports the dataset to csv

Example:

```py
>>> ds.to_csv("path/to/dataset/directory")
```

**Parameters:**

path_or_buf (`PathLike` or `FileOrBuffer`) : Either a path to a file (e.g. `file.csv`), a remote URI (e.g. `hf://datasets/username/my_dataset_name/data.csv`), or a BinaryIO, where the dataset will be saved to in the specified format.

batch_size (`int`, *optional*) : Size of the batch to load in memory and write at once. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

num_proc (`int`, *optional*) : Number of processes for multiprocessing. By default it doesn't use multiprocessing. `batch_size` in this case defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE` but feel free to make it 5x or 10x of the default value if you have sufficient compute power.

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  

- ****to_csv_kwargs** (additional keyword arguments) : Parameters to pass to pandas's [`pandas.DataFrame.to_csv`](https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.to_csv.html).    Now, `index` defaults to `False` if not specified.  If you would like to write the index, pass `index=True` and also set a name for the index column by passing `index_label`.  

**Returns:**

``int``

The number of characters or bytes written.
#### to_pandas[[datasets.Dataset.to_pandas]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L5461)

Returns the dataset as a `pandas.DataFrame`. Can also return a generator for large datasets.

Example:

```py
>>> ds.to_pandas()
```

**Parameters:**

batch_size (`int`, *optional*) : The size (number of rows) of the batches if `batched` is `True`. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

batched (`bool`) : Set to `True` to return a generator that yields the dataset as batches of `batch_size` rows. Defaults to `False` (returns the whole datasets once).

**Returns:**

`pandas.DataFrame` or `Iterator[pandas.DataFrame]`
#### to_dict[[datasets.Dataset.to_dict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L5356)

Returns the dataset as a Python dict. Can also return a generator for large datasets.

Example:

```py
>>> ds.to_dict()
```

**Parameters:**

batch_size (`int`, *optional*) : The size (number of rows) of the batches if `batched` is `True`. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

batched (`bool`) : Set to `True` to return a generator that yields the dataset as batches of `batch_size` rows. Defaults to `False` (returns the whole datasets once).

**Returns:**

`dict` or `Iterator[dict]`
#### to_json[[datasets.Dataset.to_json]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L5399)

Export the dataset to JSON Lines or JSON.

The default output format is [JSON Lines](https://jsonlines.org/).
To export to [JSON](https://www.json.org), pass `lines=False` argument and the desired `orient`.

Example:

```py
>>> ds.to_json("path/to/dataset/directory/filename.jsonl")
```

**Parameters:**

path_or_buf (`PathLike` or `FileOrBuffer`) : Either a path to a file (e.g. `file.json`), a remote URI (e.g. `hf://datasets/username/my_dataset_name/data.json`), or a BinaryIO, where the dataset will be saved to in the specified format.

batch_size (`int`, *optional*) : Size of the batch to load in memory and write at once. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

num_proc (`int`, *optional*) : Number of processes for multiprocessing. By default, it doesn't use multiprocessing. `batch_size` in this case defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE` but feel free to make it 5x or 10x of the default value if you have sufficient compute power.

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  

- ****to_json_kwargs** (additional keyword arguments) : Parameters to pass to pandas's [`pandas.DataFrame.to_json`](https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.to_json.html). Default arguments are `lines=True` and `orient="records".    The parameter `index` defaults to `False` if `orient` is `"split"` or `"table"`.  If you would like to write the index, pass `index=True`.  

**Returns:**

``int``

The number of characters or bytes written.
#### to_parquet[[datasets.Dataset.to_parquet]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L5560)

Exports the dataset to parquet

Example:

```py
>>> ds.to_parquet("path/to/dataset/directory")
```

**Parameters:**

path_or_buf (`PathLike` or `FileOrBuffer`) : Either a path to a file (e.g. `file.parquet`), a remote URI (e.g. `hf://datasets/username/my_dataset_name/data.parquet`), or a BinaryIO, where the dataset will be saved to in the specified format.

batch_size (`int`, *optional*) : Size of the batch to load in memory and write at once. By default it aims for row groups with maximum uncompressed byte size of "100MB", defined by `datasets.config.MAX_ROW_GROUP_SIZE`.

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  

- ****parquet_writer_kwargs** (additional keyword arguments) : Parameters to pass to PyArrow's `pyarrow.parquet.ParquetWriter`.

**Returns:**

``int``

The number of characters or bytes written.
#### to_sql[[datasets.Dataset.to_sql]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L5600)

Exports the dataset to a SQL database.

Example:

```py
>>> # con provided as a connection URI string
>>> ds.to_sql("data", "sqlite:///my_own_db.sql")
>>> # con provided as a sqlite3 connection object
>>> import sqlite3
>>> con = sqlite3.connect("my_own_db.sql")
>>> with con:
...     ds.to_sql("data", con)
```

**Parameters:**

name (`str`) : Name of SQL table.

con (`str` or `sqlite3.Connection` or `sqlalchemy.engine.Connection` or `sqlalchemy.engine.Connection`) : A [URI string](https://docs.sqlalchemy.org/en/13/core/engines.html#database-urls) or a SQLite3/SQLAlchemy connection object used to write to a database.

batch_size (`int`, *optional*) : Size of the batch to load in memory and write at once. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

- ****sql_writer_kwargs** (additional keyword arguments) : Parameters to pass to pandas's [`pandas.DataFrame.to_sql`](https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.to_sql.html).    Now, `index` defaults to `False` if not specified.  If you would like to write the index, pass `index=True` and also set a name for the index column by passing `index_label`.  

**Returns:**

``int``

The number of records written.
#### to_iterable_dataset[[datasets.Dataset.to_iterable_dataset]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L5692)

Get an [datasets.IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset) from a map-style [datasets.Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset).
This is equivalent to loading a dataset in streaming mode with [datasets.load_dataset()](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.load_dataset), but much faster since the data is streamed from local files.

Contrary to map-style datasets, iterable datasets are lazy and can only be iterated over (e.g. using a for loop).
Since they are read sequentially in training loops, iterable datasets are much faster than map-style datasets.
All the transformations applied to iterable datasets like filtering or processing are done on-the-fly when you start iterating over the dataset.

Still, it is possible to shuffle an iterable dataset using [datasets.IterableDataset.shuffle()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset.shuffle).
This is a fast approximate shuffling that works best if you have multiple shards and if you specify a buffer size that is big enough.

To get the best speed performance, make sure your dataset doesn't have an indices mapping.
If this is the case, the data are not read contiguously, which can be slow sometimes.
You can use `ds = ds.flatten_indices()` to write your dataset in contiguous chunks of data and have optimal speed before switching to an iterable dataset.

Example:

Basic usage:
```python
>>> ids = ds.to_iterable_dataset()
>>> for example in ids:
...     pass
```

With lazy filtering and processing:
```python
>>> ids = ds.to_iterable_dataset()
>>> ids = ids.filter(filter_fn).map(process_fn)  # will filter and process on-the-fly when you start iterating over the iterable dataset
>>> for example in ids:
...     pass
```

With sharding to enable efficient shuffling:
```python
>>> ids = ds.to_iterable_dataset(num_shards=64)  # the dataset is split into 64 shards to be iterated over
>>> ids = ids.shuffle(buffer_size=10_000)  # will shuffle the shards order and use a shuffle buffer for fast approximate shuffling when you start iterating
>>> for example in ids:
...     pass
```

With a PyTorch DataLoader:
```python
>>> import torch
>>> ids = ds.to_iterable_dataset(num_shards=64)
>>> ids = ids.filter(filter_fn).map(process_fn)
>>> dataloader = torch.utils.data.DataLoader(ids, num_workers=4)  # will assign 64 / 4 = 16 shards to each worker to load, filter and process when you start iterating
>>> for example in ids:
...     pass
```

With a PyTorch DataLoader and shuffling:
```python
>>> import torch
>>> ids = ds.to_iterable_dataset(num_shards=64)
>>> ids = ids.shuffle(buffer_size=10_000)  # will shuffle the shards order and use a shuffle buffer when you start iterating
>>> dataloader = torch.utils.data.DataLoader(ids, num_workers=4)  # will assign 64 / 4 = 16 shards from the shuffled list of shards to each worker when you start iterating
>>> for example in ids:
...     pass
```

In a distributed setup like PyTorch DDP with a PyTorch DataLoader and shuffling

```python
>>> from datasets.distributed import split_dataset_by_node
>>> ids = ds.to_iterable_dataset(num_shards=512)
>>> ids = ids.shuffle(buffer_size=10_000, seed=42)  # will shuffle the shards order and use a shuffle buffer when you start iterating
>>> ids = split_dataset_by_node(ds, world_size=8, rank=0)  # will keep only 512 / 8 = 64 shards from the shuffled lists of shards when you start iterating
>>> dataloader = torch.utils.data.DataLoader(ids, num_workers=4)  # will assign 64 / 4 = 16 shards from this node's list of shards to each worker when you start iterating
>>> for example in ids:
...     pass
```

With shuffling and multiple epochs:
```python
>>> ids = ds.to_iterable_dataset(num_shards=64)
>>> ids = ids.shuffle(buffer_size=10_000, seed=42)  # will shuffle the shards order and use a shuffle buffer when you start iterating
>>> for epoch in range(n_epochs):
...     ids.set_epoch(epoch)  # will use effective_seed = seed + epoch to shuffle the shards and for the shuffle buffer when you start iterating
...     for example in ids:
...         pass
```

Feel free to also use `IterableDataset.set_epoch()` when using a PyTorch DataLoader or in distributed setups.

**Parameters:**

num_shards (`int`, default to `1`) : Number of shards to define when instantiating the iterable dataset. This is especially useful for big datasets to be able to shuffle properly, and also to enable fast parallel loading using a PyTorch DataLoader or in distributed setups for example. Shards are defined using [datasets.Dataset.shard()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.shard): it simply slices the data without writing anything on disk.

**Returns:**

[datasets.IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)
#### add_faiss_index[[datasets.Dataset.add_faiss_index]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L6250)

Add a dense index using Faiss for fast retrieval.
By default the index is done over the vectors of the specified column.
You can specify `device` if you want to run it on GPU (`device` must be the GPU index).
You can find more information about Faiss here:

- For [string factory](https://github.com/facebookresearch/faiss/wiki/The-index-factory)

Example:

```python
>>> ds = datasets.load_dataset('community-datasets/crime_and_punish', split='train')
>>> ds_with_embeddings = ds.map(lambda example: {'embeddings': embed(example['line']}))
>>> ds_with_embeddings.add_faiss_index(column='embeddings')
>>> # query
>>> scores, retrieved_examples = ds_with_embeddings.get_nearest_examples('embeddings', embed('my new query'), k=10)
>>> # save index
>>> ds_with_embeddings.save_faiss_index('embeddings', 'my_index.faiss')

>>> ds = datasets.load_dataset('community-datasets/crime_and_punish', split='train')
>>> # load index
>>> ds.load_faiss_index('embeddings', 'my_index.faiss')
>>> # query
>>> scores, retrieved_examples = ds.get_nearest_examples('embeddings', embed('my new query'), k=10)
```

**Parameters:**

column (`str`) : The column of the vectors to add to the index.

index_name (`str`, *optional*) : The `index_name`/identifier of the index. This is the `index_name` that is used to call [get_nearest_examples()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.get_nearest_examples) or [search()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.search). By default it corresponds to `column`.

device (`Union[int, List[int]]`, *optional*) : If positive integer, this is the index of the GPU to use. If negative integer, use all GPUs. If a list of positive integers is passed in, run only on those GPUs. By default it uses the CPU.

string_factory (`str`, *optional*) : This is passed to the index factory of Faiss to create the index. Default index class is `IndexFlat`.

metric_type (`int`, *optional*) : Type of metric. Ex: `faiss.METRIC_INNER_PRODUCT` or `faiss.METRIC_L2`.

custom_index (`faiss.Index`, *optional*) : Custom Faiss index that you already have instantiated and configured for your needs.

batch_size (`int`) : Size of the batch to use while adding vectors to the `FaissIndex`. Default value is `1000`. 

train_size (`int`, *optional*) : If the index needs a training step, specifies how many vectors will be used to train the index.

faiss_verbose (`bool`, defaults to `False`) : Enable the verbosity of the Faiss index.

dtype (`data-type`) : The dtype of the numpy arrays that are indexed. Default is `np.float32`.
#### add_faiss_index_from_external_arrays[[datasets.Dataset.add_faiss_index_from_external_arrays]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L6330)

Add a dense index using Faiss for fast retrieval.
The index is created using the vectors of `external_arrays`.
You can specify `device` if you want to run it on GPU (`device` must be the GPU index).
You can find more information about Faiss here:

- For [string factory](https://github.com/facebookresearch/faiss/wiki/The-index-factory)

**Parameters:**

external_arrays (`np.array`) : If you want to use arrays from outside the lib for the index, you can set `external_arrays`. It will use `external_arrays` to create the Faiss index instead of the arrays in the given `column`.

index_name (`str`) : The `index_name`/identifier of the index. This is the `index_name` that is used to call [get_nearest_examples()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.get_nearest_examples) or [search()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.search).

device (Optional `Union[int, List[int]]`, *optional*) : If positive integer, this is the index of the GPU to use. If negative integer, use all GPUs. If a list of positive integers is passed in, run only on those GPUs. By default it uses the CPU.

string_factory (`str`, *optional*) : This is passed to the index factory of Faiss to create the index. Default index class is `IndexFlat`.

metric_type (`int`, *optional*) : Type of metric. Ex: `faiss.faiss.METRIC_INNER_PRODUCT` or `faiss.METRIC_L2`.

custom_index (`faiss.Index`, *optional*) : Custom Faiss index that you already have instantiated and configured for your needs.

batch_size (`int`, *optional*) : Size of the batch to use while adding vectors to the FaissIndex. Default value is 1000. 

train_size (`int`, *optional*) : If the index needs a training step, specifies how many vectors will be used to train the index.

faiss_verbose (`bool`, defaults to False) : Enable the verbosity of the Faiss index.

dtype (`numpy.dtype`) : The dtype of the numpy arrays that are indexed. Default is np.float32.
#### save_faiss_index[[datasets.Dataset.save_faiss_index]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/search.py#L535)

Save a FaissIndex on disk.

**Parameters:**

index_name (`str`) : The index_name/identifier of the index. This is the index_name that is used to call `.get_nearest` or `.search`.

file (`str`) : The path to the serialized faiss index on disk or remote URI (e.g. `"s3://my-bucket/index.faiss"`).

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  
#### load_faiss_index[[datasets.Dataset.load_faiss_index]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/search.py#L553)

Load a FaissIndex from disk.

If you want to do additional configurations, you can have access to the faiss index object by doing
`.get_index(index_name).faiss_index` to make it fit your needs.

**Parameters:**

index_name (`str`) : The index_name/identifier of the index. This is the index_name that is used to call `.get_nearest` or `.search`.

file (`str`) : The path to the serialized faiss index on disk or remote URI (e.g. `"s3://my-bucket/index.faiss"`).

device (Optional `Union[int, List[int]]`) : If positive integer, this is the index of the GPU to use. If negative integer, use all GPUs. If a list of positive integers is passed in, run only on those GPUs. By default it uses the CPU.

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  
#### add_elasticsearch_index[[datasets.Dataset.add_elasticsearch_index]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L6389)

Add a text index using ElasticSearch for fast retrieval. This is done in-place.

Example:

```python
>>> es_client = elasticsearch.Elasticsearch()
>>> ds = datasets.load_dataset('community-datasets/crime_and_punish', split='train')
>>> ds.add_elasticsearch_index(column='line', es_client=es_client, es_index_name="my_es_index")
>>> scores, retrieved_examples = ds.get_nearest_examples('line', 'my new query', k=10)
```

**Parameters:**

column (`str`) : The column of the documents to add to the index.

index_name (`str`, *optional*) : The `index_name`/identifier of the index. This is the index name that is used to call [get_nearest_examples()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.get_nearest_examples) or [search()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.search). By default it corresponds to `column`.

host (`str`, *optional*, defaults to `localhost`) : Host of where ElasticSearch is running.

port (`str`, *optional*, defaults to `9200`) : Port of where ElasticSearch is running.

es_client (`elasticsearch.Elasticsearch`, *optional*) : The elasticsearch client used to create the index if host and port are `None`.

es_index_name (`str`, *optional*) : The elasticsearch index name used to create the index.

es_index_config (`dict`, *optional*) : The configuration of the elasticsearch index. Default config is: ``` { "settings": { "number_of_shards": 1, "analysis": {"analyzer": {"stop_standard": {"type": "standard", " stopwords": "_english_"}}}, }, "mappings": { "properties": { "text": { "type": "text", "analyzer": "standard", "similarity": "BM25" }, } }, } ```
#### load_elasticsearch_index[[datasets.Dataset.load_elasticsearch_index]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/search.py#L637)

Load an existing text index using ElasticSearch for fast retrieval.

**Parameters:**

index_name (`str`) : The `index_name`/identifier of the index. This is the index name that is used to call `get_nearest` or `search`.

es_index_name (`str`) : The name of elasticsearch index to load.

host (`str`, *optional*, defaults to `localhost`) : Host of where ElasticSearch is running.

port (`str`, *optional*, defaults to `9200`) : Port of where ElasticSearch is running.

es_client (`elasticsearch.Elasticsearch`, *optional*) : The elasticsearch client used to create the index if host and port are `None`.

es_index_config (`dict`, *optional*) : The configuration of the elasticsearch index. Default config is: ``` { "settings": { "number_of_shards": 1, "analysis": {"analyzer": {"stop_standard": {"type": "standard", " stopwords": "_english_"}}}, }, "mappings": { "properties": { "text": { "type": "text", "analyzer": "standard", "similarity": "BM25" }, } }, } ```
#### list_indexes[[datasets.Dataset.list_indexes]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/search.py#L438)

List the `colindex_nameumns`/identifiers of all the attached indexes.
#### get_index[[datasets.Dataset.get_index]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/search.py#L442)

List the `index_name`/identifiers of all the attached indexes.

**Parameters:**

index_name (`str`) : Index name.

**Returns:**

`BaseIndex`
#### drop_index[[datasets.Dataset.drop_index]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/search.py#L684)

Drop the index with the specified column.

**Parameters:**

index_name (`str`) : The `index_name`/identifier of the index.
#### search[[datasets.Dataset.search]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/search.py#L693)

Find the nearest examples indices in the dataset to the query.

**Parameters:**

index_name (`str`) : The name/identifier of the index.

query (`Union[str, np.ndarray]`) : The query as a string if `index_name` is a text index or as a numpy array if `index_name` is a vector index.

k (`int`) : The number of examples to retrieve.

**Returns:**

``(scores, indices)``

A tuple of `(scores, indices)` where:
- **scores** (`List[List[float]`): the retrieval scores from either FAISS (`IndexFlatL2` by default) or ElasticSearch of the retrieved examples
- **indices** (`List[List[int]]`): the indices of the retrieved examples
#### search_batch[[datasets.Dataset.search_batch]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/search.py#L713)

Find the nearest examples indices in the dataset to the query.

**Parameters:**

index_name (`str`) : The `index_name`/identifier of the index.

queries (`Union[List[str], np.ndarray]`) : The queries as a list of strings if `index_name` is a text index or as a numpy array if `index_name` is a vector index.

k (`int`) : The number of examples to retrieve per query.

**Returns:**

``(total_scores, total_indices)``

A tuple of `(total_scores, total_indices)` where:
- **total_scores** (`List[List[float]`): the retrieval scores from either FAISS (`IndexFlatL2` by default) or ElasticSearch of the retrieved examples per query
- **total_indices** (`List[List[int]]`): the indices of the retrieved examples per query
#### get_nearest_examples[[datasets.Dataset.get_nearest_examples]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/search.py#L735)

Find the nearest examples in the dataset to the query.

**Parameters:**

index_name (`str`) : The index_name/identifier of the index.

query (`Union[str, np.ndarray]`) : The query as a string if `index_name` is a text index or as a numpy array if `index_name` is a vector index.

k (`int`) : The number of examples to retrieve.

**Returns:**

``(scores, examples)``

A tuple of `(scores, examples)` where:
- **scores** (`List[float]`): the retrieval scores from either FAISS (`IndexFlatL2` by default) or ElasticSearch of the retrieved examples
- **examples** (`dict`): the retrieved examples
#### get_nearest_examples_batch[[datasets.Dataset.get_nearest_examples_batch]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/search.py#L759)

Find the nearest examples in the dataset to the query.

**Parameters:**

index_name (`str`) : The `index_name`/identifier of the index.

queries (`Union[List[str], np.ndarray]`) : The queries as a list of strings if `index_name` is a text index or as a numpy array if `index_name` is a vector index.

k (`int`) : The number of examples to retrieve per query.

**Returns:**

``(total_scores, total_examples)``

A tuple of `(total_scores, total_examples)` where:
- **total_scores** (`List[List[float]`): the retrieval scores from either FAISS (`IndexFlatL2` by default) or ElasticSearch of the retrieved examples per query
- **total_examples** (`List[dict]`): the retrieved examples per query
#### info[[datasets.Dataset.info]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L179)

[DatasetInfo](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetInfo) object containing all the metadata in the dataset.
#### split[[datasets.Dataset.split]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L184)

[NamedSplit](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.NamedSplit) object corresponding to a named dataset split.
#### builder_name[[datasets.Dataset.builder_name]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L189)
#### citation[[datasets.Dataset.citation]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L193)
#### config_name[[datasets.Dataset.config_name]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L197)
#### dataset_size[[datasets.Dataset.dataset_size]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L201)
#### description[[datasets.Dataset.description]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L205)
#### download_checksums[[datasets.Dataset.download_checksums]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L209)
#### download_size[[datasets.Dataset.download_size]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L213)
#### features[[datasets.Dataset.features]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L792)
#### homepage[[datasets.Dataset.homepage]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L221)
#### license[[datasets.Dataset.license]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L225)
#### size_in_bytes[[datasets.Dataset.size_in_bytes]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L229)
#### supervised_keys[[datasets.Dataset.supervised_keys]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L233)
#### version[[datasets.Dataset.version]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L237)
#### from_csv[[datasets.Dataset.from_csv]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L1289)

Create Dataset from CSV file(s).

Read the CSV files, cache the data in Arrow format on disk and return the Dataset from the memory-mapped Arrow data on disk.

Example:

```py
>>> ds = Dataset.from_csv('path/to/dataset.csv')
```

**Parameters:**

path_or_paths (`path-like` or list of `path-like`) : Path(s) of the CSV file(s).

split ([NamedSplit](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.NamedSplit), *optional*) : Split name to be assigned to the dataset.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

cache_dir (`str`, *optional*, defaults to `"~/.cache/huggingface/datasets"`) : Directory to cache data.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

num_proc (`int`, *optional*, defaults to `None`) : Number of processes when downloading and generating the dataset locally. This is helpful if the dataset is made of multiple files. Multiprocessing is disabled by default.  

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `pandas.read_csv`.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### from_json[[datasets.Dataset.from_json]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L1429)

Create Dataset from JSON or JSON Lines file(s).

Read the JSON files, cache the data in Arrow format on disk and return the Dataset from the memory-mapped Arrow data on disk.

Example:

```py
>>> ds = Dataset.from_json('path/to/dataset.json')
```

**Parameters:**

path_or_paths (`path-like` or list of `path-like`) : Path(s) of the JSON or JSON Lines file(s).

split ([NamedSplit](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.NamedSplit), *optional*) : Split name to be assigned to the dataset.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

cache_dir (`str`, *optional*, defaults to `"~/.cache/huggingface/datasets"`) : Directory to cache data.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

field (`str`, *optional*) : Field name of the JSON file where the dataset is contained in.

num_proc (`int`, *optional* defaults to `None`) : Number of processes when downloading and generating the dataset locally. This is helpful if the dataset is made of multiple files. Multiprocessing is disabled by default.  

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `JsonConfig`.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### from_parquet[[datasets.Dataset.from_parquet]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L1488)

Create Dataset from Parquet file(s).

Read the Parquet files, cache the data in Arrow format on disk and return the Dataset from the memory-mapped Arrow data on disk.

Example:

```py
>>> ds = Dataset.from_parquet('path/to/dataset.parquet')
```

Load a subset of columns:

```python
>>> ds = Dataset.from_parquet('path/to/dataset.parquet', columns=["col_0", "col_1"])
```

Efficiently filter data, possibly skipping entire files or row groups:

```python
>>> filters = [("col_0", "==", 0)]
>>> ds = Dataset.from_parquet(parquet_files_list, filters=filters)
```

**Parameters:**

path_or_paths (`path-like` or list of `path-like`) : Path(s) of the Parquet file(s).

split (`NamedSplit`, *optional*) : Split name to be assigned to the dataset.

features (`Features`, *optional*) : Dataset features.

cache_dir (`str`, *optional*, defaults to `"~/.cache/huggingface/datasets"`) : Directory to cache data.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

columns (`List[str]`, *optional*) : If not `None`, only these columns will be read from the file. A column name may be a prefix of a nested field, e.g. 'a' will select 'a.b', 'a.c', and 'a.d.e'.

num_proc (`int`, *optional*, defaults to `None`) : Number of processes when downloading and generating the dataset locally. This is helpful if the dataset is made of multiple files. Multiprocessing is disabled by default.  

filters (`Union[pyarrow.dataset.Expression, list[tuple], list[list[tuple]]]`, *optional*) : Return only the rows matching the filter. If possible the predicate will be pushed down to exploit the partition information or internal metadata found in the data source, e.g. Parquet statistics. Otherwise filters the loaded RecordBatches before yielding them.

fragment_scan_options (`pyarrow.dataset.ParquetFragmentScanOptions`, *optional*) : Scan-specific options for Parquet fragments. This is especially useful to configure buffering and caching.  

on_bad_files (`Literal["error", "warn", "skip"]`, *optional*, defaults to "error") : Specify what to do upon encountering a bad file (a file that can't be read). Allowed values are : * 'error', raise an Exception when a bad file is encountered. * 'warn', raise a warning when a bad file is encountered and skip that file. * 'skip', skip bad files without raising or warning when they are encountered.  

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `ParquetConfig`.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### from_text[[datasets.Dataset.from_text]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L1585)

Create Dataset from text file(s).

Read the text files, cache the data in Arrow format on disk and return the Dataset from the memory-mapped Arrow data on disk.

Example:

```py
>>> ds = Dataset.from_text('path/to/dataset.txt')
```

**Parameters:**

path_or_paths (`path-like` or list of `path-like`) : Path(s) of the text file(s).

split (`NamedSplit`, *optional*) : Split name to be assigned to the dataset.

features (`Features`, *optional*) : Dataset features.

cache_dir (`str`, *optional*, defaults to `"~/.cache/huggingface/datasets"`) : Directory to cache data.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

num_proc (`int`, *optional*, defaults to `None`) : Number of processes when downloading and generating the dataset locally. This is helpful if the dataset is made of multiple files. Multiprocessing is disabled by default.  

keep_linebreaks : (`bool`, defaults to False): Whether to keep line breaks.

sample_by (`Literal["line", "paragraph", "document"]`, defaults to "line") : Whether to load data per line, praragraph or document. By default one row in the dataset = one line.

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `TextConfig`.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### from_sql[[datasets.Dataset.from_sql]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L1713)

Create Dataset from SQL query or database table.

Query the SQL database, cache the data in Arrow format on disk and return the Dataset from the memory-mapped Arrow data on disk.

Example:

```py
>>> # Fetch a database table
>>> ds = Dataset.from_sql("test_data", "postgres:///db_name")
>>> # Execute a SQL query on the table
>>> ds = Dataset.from_sql("SELECT sentence FROM test_data", "postgres:///db_name")
>>> # Use a Selectable object to specify the query
>>> from sqlalchemy import select, text
>>> stmt = select([text("sentence")]).select_from(text("test_data"))
>>> ds = Dataset.from_sql(stmt, "postgres:///db_name")
```

> [!TIP]
> The returned dataset can only be cached if `con` is specified as URI string.

**Parameters:**

sql (`str` or `sqlalchemy.sql.Selectable`) : SQL query to be executed or a table name.

con (`str` or `sqlite3.Connection` or `sqlalchemy.engine.Connection` or `sqlalchemy.engine.Connection`) : A [URI string](https://docs.sqlalchemy.org/en/13/core/engines.html#database-urls) used to instantiate a database connection or a SQLite3/SQLAlchemy connection object.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

cache_dir (`str`, *optional*, defaults to `"~/.cache/huggingface/datasets"`) : Directory to cache data.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `SqlConfig`.

**Returns:**

[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset)
#### align_labels_with_mapping[[datasets.Dataset.align_labels_with_mapping]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L6511)

Align the dataset's label ID and label name mapping to match an input `label2id` mapping.
This is useful when you want to ensure that a model's predicted labels are aligned with the dataset.
The alignment in done using the lowercase label names.

Example:

```python
>>> # dataset with mapping {'entailment': 0, 'neutral': 1, 'contradiction': 2}
>>> ds = load_dataset("nyu-mll/glue", "mnli", split="train")
>>> # mapping to align with
>>> label2id = {'CONTRADICTION': 0, 'NEUTRAL': 1, 'ENTAILMENT': 2}
>>> ds_aligned = ds.align_labels_with_mapping(label2id, "label")
```

**Parameters:**

label2id (`dict`) : The label name to ID mapping to align the dataset with.

label_column (`str`) : The column name of labels to align on.

#### datasets.concatenate_datasets[[datasets.concatenate_datasets]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/combine.py#L168)

Concatenate several datasets (sources) into a single dataset.

Use axis=0 to concatenate vertically (default), or axis=1 to concatenate horizontally.

Note for iterable datasets:

* if axis=0, the resulting dataset's `num_shards` is the sum of each dataset's `num_shards`.
* if axis=1, the resulting dataset has one (1) shard to not misalign data.

Example:

```py
>>> ds3 = concatenate_datasets([ds1, ds2])
```

**Parameters:**

dsets (`List[datasets.Dataset]` or `List[datasets.IterableDataset]`) : List of Datasets to concatenate.

info (`DatasetInfo`, *optional*) : Dataset information, like description, citation, etc.

split (`NamedSplit`, *optional*) : Name of the dataset split.

axis (`{0, 1}`, defaults to `0`) : Axis to concatenate over, where `0` means over rows (vertically) and `1` means over columns (horizontally).  

#### datasets.interleave_datasets[[datasets.interleave_datasets]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/combine.py#L18)

Interleave several datasets (sources) into a single dataset.
The new dataset is constructed by alternating between the sources to get the examples.

You can use this function on a list of [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) objects, or on a list of [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset) objects.

- If `probabilities` is `None` (default) the new dataset is constructed by cycling between each source to get the examples.
- If `probabilities` is not `None`, the new dataset is constructed by getting examples from a random source at a time according to the provided probabilities.

The resulting dataset ends when one of the source datasets runs out of examples except when `oversampling` is `True`,
in which case, the resulting dataset ends when all datasets have ran out of examples at least one time.

Note for iterable datasets:

* The resulting dataset's `num_shards` is the minimum of each dataset's `num_shards` to ensure good parallelism.
  If some of your datasets have a very low number of shards, you may use [IterableDataset.reshard()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset.reshard).
* In a distributed setup or in PyTorch DataLoader workers, the stopping strategy is applied per process.
  Therefore the "first_exhausted" strategy on an sharded iterable dataset can generate less samples in total (up to 1 missing sample per subdataset per worker).

Example:

For regular datasets (map-style):

```python
>>> from datasets import Dataset, interleave_datasets
>>> d1 = Dataset.from_dict({"a": [0, 1, 2]})
>>> d2 = Dataset.from_dict({"a": [10, 11, 12]})
>>> d3 = Dataset.from_dict({"a": [20, 21, 22]})
>>> dataset = interleave_datasets([d1, d2, d3], probabilities=[0.7, 0.2, 0.1], seed=42, stopping_strategy="all_exhausted")
>>> dataset["a"]
[10, 0, 11, 1, 2, 20, 12, 10, 0, 1, 2, 21, 0, 11, 1, 2, 0, 1, 12, 2, 10, 0, 22]
>>> dataset = interleave_datasets([d1, d2, d3], probabilities=[0.7, 0.2, 0.1], seed=42)
>>> dataset["a"]
[10, 0, 11, 1, 2]
>>> dataset = interleave_datasets([d1, d2, d3])
>>> dataset["a"]
[0, 10, 20, 1, 11, 21, 2, 12, 22]
>>> dataset = interleave_datasets([d1, d2, d3], stopping_strategy="all_exhausted")
>>> dataset["a"]
[0, 10, 20, 1, 11, 21, 2, 12, 22]
>>> d1 = Dataset.from_dict({"a": [0, 1, 2]})
>>> d2 = Dataset.from_dict({"a": [10, 11, 12, 13]})
>>> d3 = Dataset.from_dict({"a": [20, 21, 22, 23, 24]})
>>> dataset = interleave_datasets([d1, d2, d3])
>>> dataset["a"]
[0, 10, 20, 1, 11, 21, 2, 12, 22]
>>> dataset = interleave_datasets([d1, d2, d3], stopping_strategy="all_exhausted")
>>> dataset["a"]
[0, 10, 20, 1, 11, 21, 2, 12, 22, 0, 13, 23, 1, 10, 24]
>>> dataset = interleave_datasets([d1, d2, d3], probabilities=[0.7, 0.2, 0.1], seed=42)
>>> dataset["a"]
[10, 0, 11, 1, 2]
>>> dataset = interleave_datasets([d1, d2, d3], probabilities=[0.7, 0.2, 0.1], seed=42, stopping_strategy="all_exhausted")
>>> dataset["a"]
[10, 0, 11, 1, 2, 20, 12, 13, ..., 0, 1, 2, 0, 24]
For datasets in streaming mode (iterable):

>>> from datasets import interleave_datasets
>>> d1 = load_dataset('allenai/c4', 'es', split='train', streaming=True)
>>> d2 = load_dataset('allenai/c4', 'fr', split='train', streaming=True)
>>> dataset = interleave_datasets([d1, d2])
>>> iterator = iter(dataset)
>>> next(iterator)
{'text': 'Comprar Zapatillas para niña en chancla con goma por...'}
>>> next(iterator)
{'text': 'Le sacre de philippe ier, 23 mai 1059 - Compte Rendu...'
```

**Parameters:**

datasets (`List[Dataset]` or `List[IterableDataset]`) : List of datasets to interleave.

probabilities (`List[float]`, *optional*, defaults to `None`) : If specified, the new dataset is constructed by sampling examples from one source at a time according to these probabilities.

seed (`int`, *optional*, defaults to `None`) : The random seed used to choose a source for each example.

info ([DatasetInfo](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetInfo), *optional*) : Dataset information, like description, citation, etc. 

split ([NamedSplit](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.NamedSplit), *optional*) : Name of the dataset split. 

stopping_strategy (`str`, defaults to `first_exhausted`) : Three strategies are proposed right now, `first_exhausted`, `all_exhausted` and `all_exhausted_without_replacement`. By default, `first_exhausted` is an undersampling strategy, i.e the dataset construction is stopped as soon as one dataset has ran out of samples. If the strategy is `all_exhausted`,  we use an oversampling strategy, i.e the dataset construction is stopped as soon as every samples of every dataset has been added at least once. When strategy is `all_exhausted_without_replacement` we make sure that each sample in each dataset is sampled only once. Note that if the strategy is `all_exhausted`, the interleaved dataset size can get enormous: - with no probabilities, the resulting dataset will have `max_length_datasets*nb_dataset` samples. - with given probabilities, the resulting dataset will have more samples if some datasets have really low probability of visiting.

**Returns:**

`[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) or [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)`

Return type depends on the input `datasets`
parameter. `Dataset` if the input is a list of `Dataset`, `IterableDataset` if the input is a list of
`IterableDataset`.

#### datasets.distributed.split_dataset_by_node[[datasets.distributed.split_dataset_by_node]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/distributed.py#L10)

Split a dataset for the node at rank `rank` in a pool of nodes of size `world_size`.

For map-style datasets:

Each node is assigned a chunk of data, e.g. rank 0 is given the first chunk of the dataset.
To maximize data loading throughput, chunks are made of contiguous data on disk if possible.

For iterable datasets:

If the dataset has a number of shards that is a factor of `world_size` (i.e. if `dataset.num_shards % world_size == 0`),
then the shards are evenly assigned across the nodes, which is the most optimized.
Otherwise, each node keeps 1 example out of `world_size`, skipping the other examples.

> [!WARNING]
> If you shuffle your iterable dataset in a distributed setup, make sure to set a fixed `seed` in [IterableDataset.shuffle()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset.shuffle)
so the same shuffled list of shards is used on every node to know which shards the node should skip.

**Parameters:**

dataset ([Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) or [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)) : The dataset to split by node.

rank (`int`) : Rank of the current node.

world_size (`int`) : Total number of nodes.

**Returns:**

`[Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) or [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)`

The dataset to be used on the node at rank `rank`.

#### datasets.enable_caching[[datasets.enable_caching]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/fingerprint.py#L120)

When applying transforms on a dataset, the data are stored in cache files.
The caching mechanism allows to reload an existing cache file if it's already been computed.

Reloading a dataset is possible since the cache files are named using the dataset fingerprint, which is updated
after each transform.

If disabled, the library will no longer reload cached datasets files when applying transforms to the datasets.
More precisely, if the caching is disabled:
- cache files are always recreated
- cache files are written to a temporary directory that is deleted when session closes
- cache files are named using a random hash instead of the dataset fingerprint
- use [save_to_disk()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.save_to_disk) to save a transformed dataset or it will be deleted when session closes
- caching doesn't affect [load_dataset()](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.load_dataset). If you want to regenerate a dataset from scratch you should use
the `download_mode` parameter in [load_dataset()](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.load_dataset).

#### datasets.disable_caching[[datasets.disable_caching]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/fingerprint.py#L141)

When applying transforms on a dataset, the data are stored in cache files.
The caching mechanism allows to reload an existing cache file if it's already been computed.

Reloading a dataset is possible since the cache files are named using the dataset fingerprint, which is updated
after each transform.

If disabled, the library will no longer reload cached datasets files when applying transforms to the datasets.
More precisely, if the caching is disabled:
- cache files are always recreated
- cache files are written to a temporary directory that is deleted when session closes
- cache files are named using a random hash instead of the dataset fingerprint
- use [save_to_disk()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.save_to_disk) to save a transformed dataset or it will be deleted when session closes
- caching doesn't affect [load_dataset()](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.load_dataset). If you want to regenerate a dataset from scratch you should use
the `download_mode` parameter in [load_dataset()](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.load_dataset).

#### datasets.is_caching_enabled[[datasets.is_caching_enabled]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/fingerprint.py#L162)

When applying transforms on a dataset, the data are stored in cache files.
The caching mechanism allows to reload an existing cache file if it's already been computed.

Reloading a dataset is possible since the cache files are named using the dataset fingerprint, which is updated
after each transform.

If disabled, the library will no longer reload cached datasets files when applying transforms to the datasets.
More precisely, if the caching is disabled:
- cache files are always recreated
- cache files are written to a temporary directory that is deleted when session closes
- cache files are named using a random hash instead of the dataset fingerprint
- use [save_to_disk()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.save_to_disk)] to save a transformed dataset or it will be deleted when session closes
- caching doesn't affect [load_dataset()](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.load_dataset). If you want to regenerate a dataset from scratch you should use
the `download_mode` parameter in [load_dataset()](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.load_dataset).

#### datasets.Column[[datasets.Column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L645)

An iterable for a specific column of a [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset).

Example:

Iterate on the texts of the "text" column of a dataset:

```python
for text in dataset["text"]:
    ...
```

It also works with nested columns:

```python
for source in dataset["metadata"]["source"]:
    ...
```

## DatasetDict[[datasets.DatasetDict]]

Dictionary with split names as keys ('train', 'test' for example), and `Dataset` objects as values.
It also has dataset transform methods like map or filter, to process all the splits at once.

#### datasets.DatasetDict[[datasets.DatasetDict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L63)

A dictionary (dict of str: datasets.Dataset) with dataset transforms methods (map, filter, etc.)

datadatasets.DatasetDict.datahttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L104[]
The Apache Arrow tables backing each split.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.data
```
#### cache_files[[datasets.DatasetDict.cache_files]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L119)

The cache files containing the Apache Arrow table backing each split.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.cache_files
{'test': [{'filename': '/root/.cache/huggingface/datasets/rotten_tomatoes_movie_review/default/1.0.0/40d411e45a6ce3484deed7cc15b82a53dad9a72aafd9f86f8f227134bec5ca46/rotten_tomatoes_movie_review-test.arrow'}],
 'train': [{'filename': '/root/.cache/huggingface/datasets/rotten_tomatoes_movie_review/default/1.0.0/40d411e45a6ce3484deed7cc15b82a53dad9a72aafd9f86f8f227134bec5ca46/rotten_tomatoes_movie_review-train.arrow'}],
 'validation': [{'filename': '/root/.cache/huggingface/datasets/rotten_tomatoes_movie_review/default/1.0.0/40d411e45a6ce3484deed7cc15b82a53dad9a72aafd9f86f8f227134bec5ca46/rotten_tomatoes_movie_review-validation.arrow'}]}
```
#### num_columns[[datasets.DatasetDict.num_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L137)

Number of columns in each split of the dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.num_columns
{'test': 2, 'train': 2, 'validation': 2}
```
#### num_rows[[datasets.DatasetDict.num_rows]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L153)

Number of rows in each split of the dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.num_rows
{'test': 1066, 'train': 8530, 'validation': 1066}
```
#### column_names[[datasets.DatasetDict.column_names]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L169)

Names of the columns in each split of the dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.column_names
{'test': ['text', 'label'],
 'train': ['text', 'label'],
 'validation': ['text', 'label']}
```
#### shape[[datasets.DatasetDict.shape]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L187)

Shape of each split of the dataset (number of rows, number of columns).

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.shape
{'test': (1066, 2), 'train': (8530, 2), 'validation': (1066, 2)}
```
#### unique[[datasets.DatasetDict.unique]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L236)

Return a list of the unique elements in a column for each split.

This is implemented in the low-level backend and as such, very fast.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.unique("label")
{'test': [1, 0], 'train': [1, 0], 'validation': [1, 0]}
```

**Parameters:**

column (`str`) : column name (list all the column names with [column_names](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict.column_names))

**Returns:**

`Dict[`str`, `list`]`

Dictionary of unique elements in the given column.
#### cleanup_cache_files[[datasets.DatasetDict.cleanup_cache_files]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L260)

Clean up all cache files in the dataset cache directory, excepted the currently used cache file if there is one.
Be careful when running this command that no other process is currently using other cache files.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.cleanup_cache_files()
{'test': 0, 'train': 0, 'validation': 0}
```

**Returns:**

`Dict` with the number of removed files for each split
#### map[[datasets.DatasetDict.map]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L824)

Apply a function to all the examples in the table (individually or in batches) and update the table.
If your function returns a column that already exists, then it overwrites it.
The transformation is applied to all the datasets of the dataset dictionary.

You can specify whether the function should be batched or not with the `batched` parameter:

- If batched is `False`, then the function takes 1 example in and should return 1 example.
  An example is a dictionary, e.g. `{"text": "Hello there !"}`.
- If batched is `True` and `batch_size` is 1, then the function takes a batch of 1 example as input and can return a batch with 1 or more examples.
  A batch is a dictionary, e.g. a batch of 1 example is `{"text": ["Hello there !"]}`.
- If batched is `True` and `batch_size` is `n > 1`, then the function takes a batch of `n` examples as input and can return a batch with `n` examples, or with an arbitrary number of examples.
  Note that the last batch may have less than `n` examples.
  A batch is a dictionary, e.g. a batch of `n` examples is `{"text": ["Hello there !"] * n}`.

If the function is asynchronous, then `map` will run your function in parallel, with up to one thousand simultaneous calls.
It is recommended to use a `asyncio.Semaphore` in your function if you want to set a maximum number of operations that can run at the same time.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> def add_prefix(example):
...     example["text"] = "Review: " + example["text"]
...     return example
>>> ds = ds.map(add_prefix)
>>> ds["train"][0:3]["text"]
['Review: the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .',
 'Review: the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .',
 'Review: effective but too-tepid biopic']

# process a batch of examples
>>> ds = ds.map(lambda example: tokenizer(example["text"]), batched=True)
# set number of processors
>>> ds = ds.map(add_prefix, num_proc=4)
```

**Parameters:**

function (`callable`) : with one of the following signature: - `function(example: Dict[str, Any]) -> Dict[str, Any]` if `batched=False` and `with_indices=False` - `function(example: Dict[str, Any], indices: int) -> Dict[str, Any]` if `batched=False` and `with_indices=True` - `function(batch: Dict[str, list]) -> Dict[str, list]` if `batched=True` and `with_indices=False` - `function(batch: Dict[str, list], indices: list[int]) -> Dict[str, list]` if `batched=True` and `with_indices=True`  For advanced usage, the function can also return a `pyarrow.Table`. If the function is asynchronous, then `map` will run your function in parallel. Moreover if your function returns nothing (`None`), then `map` will run your function and return the dataset unchanged. If no function is provided, default to identity function: `lambda x: x`.

with_indices (`bool`, defaults to `False`) : Provide example indices to `function`. Note that in this case the signature of `function` should be `def function(example, idx): ...`.

with_rank (`bool`, defaults to `False`) : Provide process rank to `function`. Note that in this case the signature of `function` should be `def function(example[, idx], rank): ...`.

with_split (`bool`, defaults to `False`) : Provide process split to `function`. Note that in this case the signature of `function` should be `def function(example[, idx], split): ...`.

input_columns (`[Union[str, list[str]]]`, *optional*, defaults to `None`) : The columns to be passed into `function` as positional arguments. If `None`, a dict mapping to all formatted columns is passed as one argument.

batched (`bool`, defaults to `False`) : Provide batch of examples to `function`.

batch_size (`int`, *optional*, defaults to `1000`) : Number of examples per batch provided to `function` if `batched=True`, `batch_size  int32). Set to False if you want to always infer new types.

on_mixed_types (`Literal["use_json"]`, *optional*, defaults to `None`) : If "use_json", use the Json() type for mixed-types fields, i.e. unstructured fields that contain data without a predefined schema. In this case, a field with mixed type is set to Json().  This allow loading lists with a mix of strings/integers/floats for example, or dictionaries with arbitrary value types.  
#### filter[[datasets.DatasetDict.filter]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L996)

Apply a filter function to all the elements in the table in batches
and update the table so that the dataset only includes examples according to the filter function.
The transformation is applied to all the datasets of the dataset dictionary.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.filter(lambda x: x["label"] == 1)
DatasetDict({
    train: Dataset({
        features: ['text', 'label'],
        num_rows: 4265
    })
    validation: Dataset({
        features: ['text', 'label'],
        num_rows: 533
    })
    test: Dataset({
        features: ['text', 'label'],
        num_rows: 533
    })
})
```

**Parameters:**

function (`Callable`) : Callable with one of the following signatures:  - `function(example: Dict[str, Any]) -> bool` if `batched=False` and `with_indices=False` and `with_rank=False` - `function(example: Dict[str, Any], *extra_args) -> bool` if `batched=False` and `with_indices=True` and/or `with_rank=True` (one extra arg for each) - `function(batch: Dict[str, list]) -> list[bool]` if `batched=True` and `with_indices=False` and `with_rank=False` - `function(batch: Dict[str, list], *extra_args) -> list[bool]` if `batched=True` and `with_indices=True` and/or `with_rank=True` (one extra arg for each)  If no function is provided, defaults to an always `True` function: `lambda x: True`.

with_indices (`bool`, defaults to `False`) : Provide example indices to `function`. Note that in this case the signature of `function` should be `def function(example, idx[, rank]): ...`.

with_rank (`bool`, defaults to `False`) : Provide process rank to `function`. Note that in this case the signature of `function` should be `def function(example[, idx], rank): ...`.

input_columns (`[Union[str, list[str]]]`, *optional*, defaults to `None`) : The columns to be passed into `function` as positional arguments. If `None`, a dict mapping to all formatted columns is passed as one argument.

batched (`bool`, defaults to `False`) : Provide batch of examples to `function`.

batch_size (`int`, *optional*, defaults to `1000`) : Number of examples per batch provided to `function` if `batched=True` `batch_size >> from datasets import load_dataset
>>> ds = load_dataset('cornell-movie-review-data/rotten_tomatoes')
>>> ds['train']['label'][:10]
[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
>>> sorted_ds = ds.sort('label')
>>> sorted_ds['train']['label'][:10]
[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
>>> another_sorted_ds = ds.sort(['label', 'text'], reverse=[True, False])
>>> another_sorted_ds['train']['label'][:10]
[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
```

**Parameters:**

column_names (`Union[str, Sequence[str]]`) : Column name(s) to sort by.

reverse (`Union[bool, Sequence[bool]]`, defaults to `False`) : If `True`, sort by descending order rather than ascending. If a single bool is provided, the value is applied to the sorting of all column names. Otherwise a list of bools with the same length and order as column_names must be provided.

null_placement (`str`, defaults to `at_end`) : Put `None` values at the beginning if `at_start` or `first` or at the end if `at_end` or `last`

keep_in_memory (`bool`, defaults to `False`) : Keep the sorted indices in memory instead of writing it to a cache file.

load_from_cache_file (`Optional[bool]`, defaults to `True` if caching is enabled) : If a cache file storing the sorted indices can be identified, use it instead of recomputing.

indices_cache_file_names (`[Dict[str, str]]`, *optional*, defaults to `None`) : Provide the name of a path for the cache file. It is used to store the indices mapping instead of the automatically generated cache file name. You have to provide one `cache_file_name` per dataset in the dataset dictionary.

writer_batch_size (`int`, defaults to `1000`) : Number of rows per write operation for the cache file writer. Higher value gives smaller cache files, lower value consume less temporary memory.
#### shuffle[[datasets.DatasetDict.shuffle]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1228)

Create a new Dataset where the rows are shuffled.

The transformation is applied to all the datasets of the dataset dictionary.

Currently shuffling uses numpy random generators.
You can either supply a NumPy BitGenerator to use, or a seed to initiate NumPy's default random generator (PCG64).

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds["train"]["label"][:10]
[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]

# set a seed
>>> shuffled_ds = ds.shuffle(seed=42)
>>> shuffled_ds["train"]["label"][:10]
[0, 1, 0, 1, 0, 0, 0, 0, 0, 0]
```

**Parameters:**

seeds (`Dict[str, int]` or `int`, *optional*) : A seed to initialize the default BitGenerator if `generator=None`. If `None`, then fresh, unpredictable entropy will be pulled from the OS. If an `int` or `array_like[ints]` is passed, then it will be passed to SeedSequence to derive the initial BitGenerator state. You can provide one `seed` per dataset in the dataset dictionary.

seed (`int`, *optional*) : A seed to initialize the default BitGenerator if `generator=None`. Alias for seeds (a `ValueError` is raised if both are provided).

generators (`Dict[str, *optional*, np.random.Generator]`) : Numpy random Generator to use to compute the permutation of the dataset rows. If `generator=None` (default), uses `np.random.default_rng` (the default BitGenerator (PCG64) of NumPy). You have to provide one `generator` per dataset in the dataset dictionary.

keep_in_memory (`bool`, defaults to `False`) : Keep the dataset in memory instead of writing it to a cache file.

load_from_cache_file (`Optional[bool]`, defaults to `True` if caching is enabled) : If a cache file storing the current computation from `function` can be identified, use it instead of recomputing.

indices_cache_file_names (`Dict[str, str]`, *optional*) : Provide the name of a path for the cache file. It is used to store the indices mappings instead of the automatically generated cache file name. You have to provide one `cache_file_name` per dataset in the dataset dictionary.

writer_batch_size (`int`, defaults to `1000`) : Number of rows per write operation for the cache file writer. This value is a good trade-off between memory usage during the processing, and processing speed. Higher value makes the processing do fewer lookups, lower value consume less temporary memory while running `map`.
#### set_format[[datasets.DatasetDict.set_format]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L581)

Set `__getitem__` return format (type and columns).
The format is set for every dataset in the dataset dictionary.

It is possible to call `map` after calling `set_format`. Since `map` may add new columns, then the list of formatted columns
gets updated. In this case, if you apply `map` on a dataset to add a new column, then this column will be formatted:

`new formatted columns = (all columns - previously unformatted columns)`

Example:

```py
>>> from datasets import load_dataset
>>> from transformers import AutoTokenizer
>>> tokenizer = AutoTokenizer.from_pretrained("bert-base-cased")
>>> ds = ds.map(lambda x: tokenizer(x["text"], truncation=True, padding=True), batched=True)
>>> ds.set_format(type="numpy", columns=['input_ids', 'token_type_ids', 'attention_mask', 'label'])
>>> ds["train"].format
{'columns': ['input_ids', 'token_type_ids', 'attention_mask', 'label'],
 'format_kwargs': {},
 'output_all_columns': False,
 'type': 'numpy'}
```

**Parameters:**

type (`str`, *optional*) : Either output type selected in `[None, 'numpy', 'torch', 'tensorflow', 'jax', 'arrow', 'pandas', 'polars']`. `None` means `__getitem__` returns python objects (default).

columns (`list[str]`, *optional*) : Columns to format in the output. `None` means `__getitem__` returns all columns (default).

output_all_columns (`bool`, defaults to False) : Keep un-formatted columns as well in the output (as python objects),

- ****format_kwargs** (additional keyword arguments) : Keywords arguments passed to the convert function like `np.array`, `torch.tensor` or `tensorflow.ragged.constant`.
#### reset_format[[datasets.DatasetDict.reset_format]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L632)

Reset `__getitem__` return format to python objects and all columns.
The transformation is applied to all the datasets of the dataset dictionary.

Same as `self.set_format()`

Example:

```py
>>> from datasets import load_dataset
>>> from transformers import AutoTokenizer
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> tokenizer = AutoTokenizer.from_pretrained("bert-base-cased")
>>> ds = ds.map(lambda x: tokenizer(x["text"], truncation=True, padding=True), batched=True)
>>> ds.set_format(type="numpy", columns=['input_ids', 'token_type_ids', 'attention_mask', 'label'])
>>> ds["train"].format
{'columns': ['input_ids', 'token_type_ids', 'attention_mask', 'label'],
 'format_kwargs': {},
 'output_all_columns': False,
 'type': 'numpy'}
>>> ds.reset_format()
>>> ds["train"].format
{'columns': ['text', 'label', 'input_ids', 'token_type_ids', 'attention_mask'],
 'format_kwargs': {},
 'output_all_columns': False,
 'type': None}
```
#### formatted_as[[datasets.DatasetDict.formatted_as]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L541)

To be used in a `with` statement. Set `__getitem__` return format (type and columns).
The transformation is applied to all the datasets of the dataset dictionary.

**Parameters:**

type (`str`, *optional*) : Either output type selected in `[None, 'numpy', 'torch', 'tensorflow', 'jax', 'arrow', 'pandas', 'polars']`. `None` means `__getitem__` returns python objects (default).

columns (`list[str]`, *optional*) : Columns to format in the output. `None` means `__getitem__` returns all columns (default).

output_all_columns (`bool`, defaults to False) : Keep un-formatted columns as well in the output (as python objects).

- ****format_kwargs** (additional keyword arguments) : Keywords arguments passed to the convert function like `np.array`, `torch.tensor` or `tensorflow.ragged.constant`.
#### with_format[[datasets.DatasetDict.with_format]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L693)

Set `__getitem__` return format (type and columns). The data formatting is applied on-the-fly.
The format `type` (for example "numpy") is used to format batches when using `__getitem__`.
The format is set for every dataset in the dataset dictionary.

It's also possible to use custom transforms for formatting using [with_transform()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.with_transform).

Contrary to [set_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict.set_format), `with_format` returns a new [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) object with new [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) objects.

Example:

```py
>>> from datasets import load_dataset
>>> from transformers import AutoTokenizer
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> tokenizer = AutoTokenizer.from_pretrained("bert-base-cased")
>>> ds = ds.map(lambda x: tokenizer(x['text'], truncation=True, padding=True), batched=True)
>>> ds["train"].format
{'columns': ['text', 'label', 'input_ids', 'token_type_ids', 'attention_mask'],
 'format_kwargs': {},
 'output_all_columns': False,
 'type': None}
>>> ds = ds.with_format("torch")
>>> ds["train"].format
{'columns': ['text', 'label', 'input_ids', 'token_type_ids', 'attention_mask'],
 'format_kwargs': {},
 'output_all_columns': False,
 'type': 'torch'}
>>> ds["train"][0]
{'text': 'compassionately explores the seemingly irreconcilable situation between conservative christian parents and their estranged gay and lesbian children .',
 'label': tensor(1),
 'input_ids': tensor([  101, 18027, 16310, 16001,  1103,  9321,   178, 11604,  7235,  6617,
        1742,  2165,  2820,  1206,  6588, 22572, 12937,  1811,  2153,  1105,
        1147, 12890, 19587,  6463,  1105, 15026,  1482,   119,   102,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0]),
 'token_type_ids': tensor([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
 'attention_mask': tensor([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])}
```

**Parameters:**

type (`str`, *optional*) : Either output type selected in `[None, 'numpy', 'torch', 'tensorflow', 'jax', 'arrow', 'pandas', 'polars']`. `None` means `__getitem__` returns python objects (default).

columns (`list[str]`, *optional*) : Columns to format in the output. `None` means `__getitem__` returns all columns (default).

output_all_columns (`bool`, defaults to `False`) : Keep un-formatted columns as well in the output (as python objects).

- ****format_kwargs** (additional keyword arguments) : Keywords arguments passed to the convert function like `np.array`, `torch.tensor` or `tensorflow.ragged.constant`.
#### with_transform[[datasets.DatasetDict.with_transform]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L770)

Set `__getitem__` return format using this transform. The transform is applied on-the-fly on batches when `__getitem__` is called.
The transform is set for every dataset in the dataset dictionary

As [set_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.set_format), this can be reset using [reset_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.reset_format).

Contrary to `set_transform()`, `with_transform` returns a new [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) object with new [Dataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset) objects.

Example:

```py
>>> from datasets import load_dataset
>>> from transformers import AutoTokenizer
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> tokenizer = AutoTokenizer.from_pretrained("bert-base-cased")
>>> def encode(example):
...     return tokenizer(example['text'], truncation=True, padding=True, return_tensors="pt")
>>> ds = ds.with_transform(encode)
>>> ds["train"][0]
{'attention_mask': tensor([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
 1, 1, 1, 1, 1, 1, 1, 1, 1]),
 'input_ids': tensor([  101,  1103,  2067,  1110, 17348,  1106,  1129,  1103,  6880,  1432,
        112,   188,  1207,   107, 14255,  1389,   107,  1105,  1115,  1119,
        112,   188,  1280,  1106,  1294,   170, 24194,  1256,  3407,  1190,
        170, 11791,  5253,   188,  1732,  7200, 10947, 12606,  2895,   117,
        179,  7766,   118,   172, 15554,  1181,  3498,  6961,  3263,  1137,
        188,  1566,  7912, 14516,  6997,   119,   102]),
 'token_type_ids': tensor([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0])}
```

**Parameters:**

transform (`Callable`, *optional*) : User-defined formatting transform, replaces the format defined by [set_format()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.set_format). A formatting function is a callable that takes a batch (as a dict) as input and returns a batch. This function is applied right before returning the objects in `__getitem__`.

columns (`list[str]`, *optional*) : Columns to format in the output. If specified, then the input batch of the transform only contains those columns.

output_all_columns (`bool`, defaults to False) : Keep un-formatted columns as well in the output (as python objects). If set to `True`, then the other un-formatted columns are kept with the output of the transform.
#### flatten[[datasets.DatasetDict.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L203)

Flatten the Apache Arrow Table of each split (nested features are flatten).
Each column with a struct type is flattened into one column per struct field.
Other columns are left unchanged.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("rajpurkar/squad")
>>> ds["train"].features
{'id': Value('string'),
 'title': Value('string'),
 'context': Value('string'),
 'question': Value('string'),
 'answers.text': List(Value('string')),
 'answers.answer_start': List(Value('int32'))}
>>> ds.flatten()
DatasetDict({
    train: Dataset({
        features: ['id', 'title', 'context', 'question', 'answers.text', 'answers.answer_start'],
        num_rows: 87599
    })
    validation: Dataset({
        features: ['id', 'title', 'context', 'question', 'answers.text', 'answers.answer_start'],
        num_rows: 10570
    })
})
```
#### cast[[datasets.DatasetDict.cast]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L284)

Cast the dataset to a new set of features.
The transformation is applied to all the datasets of the dataset dictionary.

Example:

```py
>>> from datasets import load_dataset, ClassLabel, Value
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds["train"].features
{'label': ClassLabel(names=['neg', 'pos']),
 'text': Value('string')}
>>> new_features = ds["train"].features.copy()
>>> new_features['label'] = ClassLabel(names=['bad', 'good'])
>>> new_features['text'] = Value('large_string')
>>> ds = ds.cast(new_features)
>>> ds["train"].features
{'label': ClassLabel(names=['bad', 'good']),
 'text': Value('large_string')}
```

**Parameters:**

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features)) : New features to cast the dataset to. The name and order of the fields in the features must match the current column names. The type of the data must also be convertible from one type to the other. For non-trivial conversion, e.g. `string`  `ClassLabel` you should use [map()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict.map) to update the dataset.
#### cast_column[[datasets.DatasetDict.cast_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L316)

Cast column to feature for decoding.

Example:

```py
>>> from datasets import load_dataset, ClassLabel
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds["train"].features
{'label': ClassLabel(names=['neg', 'pos']),
 'text': Value('string')}
>>> ds = ds.cast_column('label', ClassLabel(names=['bad', 'good']))
>>> ds["train"].features
{'label': ClassLabel(names=['bad', 'good']),
 'text': Value('string')}
```

**Parameters:**

column (`str`) : Column name.

feature (`Feature`) : Target feature.

**Returns:**

[DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)
#### remove_columns[[datasets.DatasetDict.remove_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L345)

Remove one or several column(s) from each split in the dataset
and the features associated to the column(s).

The transformation is applied to all the splits of the dataset dictionary.

You can also remove a column using [map()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict.map) with `remove_columns` but the present method
doesn't copy the data of the remaining columns and is thus faster.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds = ds.remove_columns("label")
DatasetDict({
    train: Dataset({
        features: ['text'],
        num_rows: 8530
    })
    validation: Dataset({
        features: ['text'],
        num_rows: 1066
    })
    test: Dataset({
        features: ['text'],
        num_rows: 1066
    })
})
```

**Parameters:**

column_names (`Union[str, list[str]]`) : Name of the column(s) to remove.

**Returns:**

`[DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)`

A copy of the dataset object without the columns to remove.
#### rename_column[[datasets.DatasetDict.rename_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L387)

Rename a column in the dataset and move the features associated to the original column under the new column name.
The transformation is applied to all the datasets of the dataset dictionary.

You can also rename a column using [map()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict.map) with `remove_columns` but the present method:
- takes care of moving the original features under the new column name.
- doesn't copy the data to a new dataset and is thus much faster.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds = ds.rename_column("label", "label_new")
DatasetDict({
    train: Dataset({
        features: ['text', 'label_new'],
        num_rows: 8530
    })
    validation: Dataset({
        features: ['text', 'label_new'],
        num_rows: 1066
    })
    test: Dataset({
        features: ['text', 'label_new'],
        num_rows: 1066
    })
})
```

**Parameters:**

original_column_name (`str`) : Name of the column to rename.

new_column_name (`str`) : New name for the column.
#### rename_columns[[datasets.DatasetDict.rename_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L435)

Rename several columns in the dataset, and move the features associated to the original columns under
the new column names.
The transformation is applied to all the datasets of the dataset dictionary.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.rename_columns({'text': 'text_new', 'label': 'label_new'})
DatasetDict({
    train: Dataset({
        features: ['text_new', 'label_new'],
        num_rows: 8530
    })
    validation: Dataset({
        features: ['text_new', 'label_new'],
        num_rows: 1066
    })
    test: Dataset({
        features: ['text_new', 'label_new'],
        num_rows: 1066
    })
})
```

**Parameters:**

column_mapping (`Dict[str, str]`) : A mapping of columns to rename to their new names.

**Returns:**

`[DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)`

A copy of the dataset with renamed columns.
#### select_columns[[datasets.DatasetDict.select_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L473)

Select one or several column(s) from each split in the dataset and
the features associated to the column(s).

The transformation is applied to all the splits of the dataset
dictionary.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes")
>>> ds.select_columns("text")
DatasetDict({
    train: Dataset({
        features: ['text'],
        num_rows: 8530
    })
    validation: Dataset({
        features: ['text'],
        num_rows: 1066
    })
    test: Dataset({
        features: ['text'],
        num_rows: 1066
    })
})
```

**Parameters:**

column_names (`Union[str, list[str]]`) : Name of the column(s) to keep.
#### class_encode_column[[datasets.DatasetDict.class_encode_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L509)

Casts the given column as [ClassLabel](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.ClassLabel) and updates the tables.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("google/boolq")
>>> ds["train"].features
{'answer': Value('bool'),
 'passage': Value('string'),
 'question': Value('string')}
>>> ds = ds.class_encode_column("answer")
>>> ds["train"].features
{'answer': ClassLabel(num_classes=2, names=['False', 'True']),
 'passage': Value('string'),
 'question': Value('string')}
```

**Parameters:**

column (`str`) : The name of the column to cast.

include_nulls (`bool`, defaults to `False`) : Whether to include null values in the class labels. If `True`, the null values will be encoded as the `"None"` class label.  
#### push_to_hub[[datasets.DatasetDict.push_to_hub]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1633)

Pushes the [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) to the hub as a Parquet dataset.
The [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) is pushed using HTTP requests and does not need to have neither git or git-lfs installed.

Each dataset split will be pushed independently. The pushed dataset will keep the original split names.

The resulting Parquet files are self-contained by default: if your dataset contains [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image) or [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio)
data, the Parquet files will store the bytes of your images or audio files.
You can disable this by setting `embed_external_files` to False.

Example:

```python
>>> dataset_dict.push_to_hub("/")
>>> dataset_dict.push_to_hub("/", private=True)
>>> dataset_dict.push_to_hub("/", max_shard_size="1GB")
>>> dataset_dict.push_to_hub("/", num_shards={"train": 1024, "test": 8})
```

If you want to add a new configuration (or subset) to a dataset (e.g. if the dataset has multiple tasks/versions/languages):

```python
>>> english_dataset.push_to_hub("/", "en")
>>> french_dataset.push_to_hub("/", "fr")
>>> # later
>>> english_dataset = load_dataset("/", "en")
>>> french_dataset = load_dataset("/", "fr")
```

**Parameters:**

repo_id (`str`) : The ID of the repository to push to in the following format: `/` or `/`. Also accepts ``, which will default to the namespace of the logged-in user.  It could also be a location inside a bucket, e.g. `buckets///...`

config_name (`str`) : Configuration name of a dataset. Defaults to "default".

set_default (`bool`, *optional*) : Whether to set this configuration as the default one. Otherwise, the default configuration is the one named "default".

data_dir (`str`, *optional*) : Directory name that will contain the uploaded data files. Defaults to the `config_name` if different from "default", else "data".  

commit_message (`str`, *optional*) : Message to commit while pushing. Will default to `"Upload dataset"`.

commit_description (`str`, *optional*) : Description of the commit that will be created. Additionally, description of the PR if a PR is created (`create_pr` is True).  

private (`bool`, *optional*) : Whether to make the repo private. If `None` (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

token (`str`, *optional*) : An optional authentication token for the Hugging Face Hub. If no token is passed, will default to the token saved locally when logging in with `huggingface-cli login`. Will raise an error if no token is passed and the user is not logged-in.

revision (`str`, *optional*) : Branch to push the uploaded files to. Defaults to the `"main"` branch.  

create_pr (`bool`, *optional*, defaults to `False`) : Whether to create a PR with the uploaded files or directly commit.  

max_shard_size (`int` or `str`, *optional*, defaults to `"500MB"`) : The maximum size of the dataset shards to be uploaded to the hub. If expressed as a string, needs to be digits followed by a unit (like `"500MB"` or `"1GB"`).

num_shards (`Dict[str, int]`, *optional*) : Number of shards to write. By default, the number of shards depends on `max_shard_size`. Use a dictionary to define a different num_shards for each split.  

embed_external_files (`bool`, defaults to `True`) : Whether to embed file bytes in the shards. In particular, this will do the following before the push for the fields of type:  - [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio) and [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image) removes local path information and embed file content in the Parquet files.

num_proc (`int`, *optional*, defaults to `None`) : Number of processes when preparing and uploading the dataset. This is helpful if the dataset is made of many samples or media files to embed. I uses "spawn" context to work with hf_xet, the rust client for fast uploads to HF. Multiprocessing is disabled by default.  

**Returns:**

huggingface_hub.CommitInfo
#### save_to_disk[[datasets.DatasetDict.save_to_disk]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1311)

Saves a dataset dict to a filesystem using `fsspec.spec.AbstractFileSystem`.

For [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image), [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio) and [Video](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Video) data:

All the Image(), Audio() and Video() data are stored in the arrow files.
If you want to store paths or urls, please use the Value("string") type.

Example:

```python
>>> dataset_dict.save_to_disk("path/to/dataset/directory")
>>> dataset_dict.save_to_disk("path/to/dataset/directory", max_shard_size="1GB")
>>> dataset_dict.save_to_disk("path/to/dataset/directory", num_shards={"train": 1024, "test": 8})
```

**Parameters:**

dataset_dict_path (`path-like`) : Path (e.g. `dataset/train`) or remote URI (e.g. `s3://my-bucket/dataset/train`) of the dataset dict directory where the dataset dict will be saved to.

max_shard_size (`int` or `str`, *optional*, defaults to `"500MB"`) : The maximum size of the dataset shards to be saved to the filesystem. If expressed as a string, needs to be digits followed by a unit (like `"50MB"`).

num_shards (`Dict[str, int]`, *optional*) : Number of shards to write. By default the number of shards depends on `max_shard_size` and `num_proc`. You need to provide the number of shards for each dataset in the dataset dictionary. Use a dictionary to define a different num_shards for each split.  

num_proc (`int`, *optional*, default `None`) : Number of processes when downloading and generating the dataset locally. Multiprocessing is disabled by default.  

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  
#### load_from_disk[[datasets.DatasetDict.load_from_disk]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1385)

Load a dataset that was previously saved using `save_to_disk` from a filesystem using `fsspec.spec.AbstractFileSystem`.

Example:

```py
>>> ds = load_from_disk('path/to/dataset/directory')
```

**Parameters:**

dataset_dict_path (`path-like`) : Path (e.g. `"dataset/train"`) or remote URI (e.g. `"s3//my-bucket/dataset/train"`) of the dataset dict directory where the dataset dict will be loaded from.

keep_in_memory (`bool`, defaults to `None`) : Whether to copy the dataset in-memory. If `None`, the dataset will not be copied in-memory unless explicitly enabled by setting `datasets.config.IN_MEMORY_MAX_SIZE` to nonzero. See more details in the [improve performance](../cache#improve-performance) section.

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  

**Returns:**

[DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)
#### from_csv[[datasets.DatasetDict.from_csv]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1445)

Create [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) from CSV file(s).

Example:

```py
>>> from datasets import DatasetDict
>>> ds = DatasetDict.from_csv({'train': 'path/to/dataset.csv'})
```

**Parameters:**

path_or_paths (`dict` of path-like) : Path(s) of the CSV file(s).

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

cache_dir (str, *optional*, defaults to `"~/.cache/huggingface/datasets"`) : Directory to cache data.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `pandas.read_csv`.

**Returns:**

[DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)
#### from_json[[datasets.DatasetDict.from_json]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1488)

Create [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) from JSON Lines file(s).

Example:

```py
>>> from datasets import DatasetDict
>>> ds = DatasetDict.from_json({'train': 'path/to/dataset.json'})
```

**Parameters:**

path_or_paths (`path-like` or list of `path-like`) : Path(s) of the JSON Lines file(s).

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

cache_dir (str, *optional*, defaults to `"~/.cache/huggingface/datasets"`) : Directory to cache data.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `JsonConfig`.

**Returns:**

[DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)
#### from_parquet[[datasets.DatasetDict.from_parquet]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1531)

Create [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) from Parquet file(s).

Example:

```py
>>> from datasets import DatasetDict
>>> ds = DatasetDict.from_parquet({'train': 'path/to/dataset/parquet'})
```

**Parameters:**

path_or_paths (`dict` of path-like) : Path(s) of the CSV file(s).

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

cache_dir (`str`, *optional*, defaults to `"~/.cache/huggingface/datasets"`) : Directory to cache data.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

columns (`list[str]`, *optional*) : If not `None`, only these columns will be read from the file. A column name may be a prefix of a nested field, e.g. 'a' will select 'a.b', 'a.c', and 'a.d.e'.

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `ParquetConfig`.

**Returns:**

[DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)
#### from_text[[datasets.DatasetDict.from_text]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1580)

Create [DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict) from text file(s).

Example:

```py
>>> from datasets import DatasetDict
>>> ds = DatasetDict.from_text({'train': 'path/to/dataset.txt'})
```

**Parameters:**

path_or_paths (`dict` of path-like) : Path(s) of the text file(s).

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

cache_dir (`str`, *optional*, defaults to `"~/.cache/huggingface/datasets"`) : Directory to cache data.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `TextConfig`.

**Returns:**

[DatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetDict)

## IterableDataset[[datasets.IterableDataset]]

The base class [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset) implements an iterable Dataset backed by python generators.

#### datasets.IterableDataset[[datasets.IterableDataset]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2357)

A Dataset backed by an iterable.

from_filedatasets.IterableDataset.from_filehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2826[{"name": "filename", "val": ": str"}]- **filename** (`str`) --
  File name of the dataset.0[IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)
Instantiate a IterableDataset from Arrow table at filename.

**Parameters:**

filename (`str`) : File name of the dataset.

**Returns:**

[IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)
#### from_pandas[[datasets.IterableDataset.from_pandas]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2842)

Convert `pandas.DataFrame` to a `pyarrow.Table` to create an [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset).

The column types in the resulting Arrow Table are inferred from the dtypes of the `pandas.Series` in the
DataFrame. In the case of non-object Series, the NumPy dtype is translated to its Arrow equivalent. In the
case of `object`, we need to guess the datatype by looking at the Python objects in this Series.

Be aware that Series of the `object` dtype don't carry enough information to always lead to a meaningful Arrow
type. In the case that we cannot infer a type, e.g. because the DataFrame is of length 0 or the Series only
contains `None/nan` objects, the type is set to `null`. This behavior can be avoided by constructing explicit
features and passing it to this function.

Important: a dataset created with from_pandas() lives in memory.
This may change in the future, but in the meantime if you
want to reduce memory usage you should write it on disk
and reload using e.g. to_parquet / from_parquet.

Example:

```py
>>> ds = IterableDataset.from_pandas(df)
```

**Parameters:**

df (`pandas.DataFrame`) : Dataframe that contains the dataset.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

info (`DatasetInfo`, *optional*) : Dataset information, like description, citation, etc.

split (`NamedSplit`, *optional*) : Name of the dataset split.

preserve_index (`bool`, *optional*) : Whether to store the index as an additional column in the resulting Dataset. The default of `None` will store the index as a column, except for `RangeIndex` which is stored as metadata only. Use `preserve_index=True` to force it to be stored as a column.

num_shards (`int`, default to `1`) : Number of shards to define when instantiating the iterable dataset. This is especially useful for big datasets to be able to shuffle properly, and also to enable fast parallel loading using a PyTorch DataLoader or in distributed setups for example.

**Returns:**

[IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)
#### from_dict[[datasets.IterableDataset.from_dict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2954)

Convert `dict` to a `pyarrow.Table` to create an [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset).

Important: a dataset created with from_dict() lives in memory.
This may change in the future, but in the meantime if you
want to reduce memory usage you should write it back on disk
and reload using e.g. to_parquet / from_parquet.

**Parameters:**

mapping (`Mapping`) : Mapping of strings to Arrays or Python lists.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

info (`DatasetInfo`, *optional*) : Dataset information, like description, citation, etc.

split (`NamedSplit`, *optional*) : Name of the dataset split.

num_shards (`int`, default to `1`) : Number of shards to define when instantiating the iterable dataset. This is especially useful for big datasets to be able to shuffle properly, and also to enable fast parallel loading using a PyTorch DataLoader or in distributed setups for example.

**Returns:**

[IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)
#### from_list[[datasets.IterableDataset.from_list]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2991)

Convert a list of dicts to a `pyarrow.Table` to create an [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)`.

Note that the keys of the first entry will be used to determine the dataset columns,
regardless of what is passed to features.

Important: a dataset created with from_list() lives in memory.
This may change in the future, but in the meantime if you
want to reduce memory usage you should write it back on disk
and reload using e.g. from_parquet / to_parquet.

**Parameters:**

mapping (`List[dict]`) : A list of mappings of strings to row values.

features (`Features`, optional) : Dataset features.

info (`DatasetInfo`, optional) : Dataset information, like description, citation, etc.

split (`NamedSplit`, optional) : Name of the dataset split.

num_shards (`int`, default to `1`) : Number of shards to define when instantiating the iterable dataset. This is especially useful for big datasets to be able to shuffle properly, and also to enable fast parallel loading using a PyTorch DataLoader or in distributed setups for example.

**Returns:**

[IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)
#### from_generator[[datasets.IterableDataset.from_generator]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2728)

Create an Iterable Dataset from a generator.

Example:

```py
>>> def gen():
...     yield {"text": "Good", "label": 0}
...     yield {"text": "Bad", "label": 1}
...
>>> ds = IterableDataset.from_generator(gen)
```

```py
>>> def gen(shards):
...     for shard in shards:
...         with open(shard) as f:
...             for line in f:
...                 yield {"line": line}
...
>>> shards = [f"data{i}.txt" for i in range(32)]
>>> ds = IterableDataset.from_generator(gen, gen_kwargs={"shards": shards})
>>> ds = ds.shuffle(seed=42, buffer_size=10_000)  # shuffles the shards order + uses a shuffle buffer
>>> from torch.utils.data import DataLoader
>>> dataloader = DataLoader(ds.with_format("torch"), num_workers=4)  # give each worker a subset of 32/4=8 shards
```

**Parameters:**

generator (`Callable`) : A generator function that `yields` examples.

features (`Features`, *optional*) : Dataset features.

gen_kwargs(`dict`, *optional*) : Keyword arguments to be passed to the `generator` callable. You can define a sharded iterable dataset by passing the list of shards in `gen_kwargs`. This can be used to improve shuffling and when iterating over the dataset with multiple workers.

split ([NamedSplit](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.NamedSplit), defaults to `Split.TRAIN`) : Split name to be assigned to the dataset.  

**Returns:**

[IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)
#### remove_columns[[datasets.IterableDataset.remove_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3908)

Remove one or several column(s) in the dataset and the features associated to them.
The removal is done on-the-fly on the examples when iterating over the dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train", streaming=True)
>>> next(iter(ds))
{'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .', 'label': 1}
>>> ds = ds.remove_columns("label")
>>> next(iter(ds))
{'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'}
```

**Parameters:**

column_names (`Union[str, List[str]]`) : Name of the column(s) to remove.

**Returns:**

``IterableDataset``

A copy of the dataset object without the columns to remove.
#### select_columns[[datasets.IterableDataset.select_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3943)

Select one or several column(s) in the dataset and the features
associated to them. The selection is done on-the-fly on the examples
when iterating over the dataset.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train", streaming=True)
>>> next(iter(ds))
{'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .', 'label': 1}
>>> ds = ds.select_columns("text")
>>> next(iter(ds))
{'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'}
```

**Parameters:**

column_names (`Union[str, List[str]]`) : Name of the column(s) to select.

**Returns:**

``IterableDataset``

A copy of the dataset object with selected columns.
#### cast_column[[datasets.IterableDataset.cast_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3993)

Cast column to feature for decoding.

Example:

```py
>>> from datasets import load_dataset, Audio
>>> ds = load_dataset("PolyAI/minds14", name="en-US", split="train", streaming=True)
>>> ds.features
{'audio': Audio(sampling_rate=8000, mono=True, decode=True, id=None),
 'english_transcription': Value('string'),
 'intent_class': ClassLabel(num_classes=14, names=['abroad', 'address', 'app_error', 'atm_limit', 'balance', 'business_loan',  'card_issues', 'cash_deposit', 'direct_debit', 'freeze', 'high_value_payment', 'joint_account', 'latest_transactions', 'pay_bill']),
 'lang_id': ClassLabel(num_classes=14, names=['cs-CZ', 'de-DE', 'en-AU', 'en-GB', 'en-US', 'es-ES', 'fr-FR', 'it-IT', 'ko-KR',  'nl-NL', 'pl-PL', 'pt-PT', 'ru-RU', 'zh-CN']),
 'path': Value('string'),
 'transcription': Value('string')}
>>> ds = ds.cast_column("audio", Audio(sampling_rate=16000))
>>> ds.features
{'audio': Audio(sampling_rate=16000, mono=True, decode=True, id=None),
 'english_transcription': Value('string'),
 'intent_class': ClassLabel(num_classes=14, names=['abroad', 'address', 'app_error', 'atm_limit', 'balance', 'business_loan',  'card_issues', 'cash_deposit', 'direct_debit', 'freeze', 'high_value_payment', 'joint_account', 'latest_transactions', 'pay_bill']),
 'lang_id': ClassLabel(num_classes=14, names=['cs-CZ', 'de-DE', 'en-AU', 'en-GB', 'en-US', 'es-ES', 'fr-FR', 'it-IT', 'ko-KR',  'nl-NL', 'pl-PL', 'pt-PT', 'ru-RU', 'zh-CN']),
 'path': Value('string'),
 'transcription': Value('string')}
```

**Parameters:**

column (`str`) : Column name.

feature (`Feature`) : Target feature.

**Returns:**

``IterableDataset``
#### cast[[datasets.IterableDataset.cast]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L4039)

Cast the dataset to a new set of features.

Example:

```py
>>> from datasets import load_dataset, ClassLabel, Value
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train", streaming=True)
>>> ds.features
{'label': ClassLabel(names=['neg', 'pos']),
 'text': Value('string')}
>>> new_features = ds.features.copy()
>>> new_features["label"] = ClassLabel(names=["bad", "good"])
>>> new_features["text"] = Value("large_string")
>>> ds = ds.cast(new_features)
>>> ds.features
{'label': ClassLabel(names=['bad', 'good']),
 'text': Value('large_string')}
```

**Parameters:**

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features)) : New features to cast the dataset to. The name of the fields in the features must match the current column names. The type of the data must also be convertible from one type to the other. For non-trivial conversion, e.g. `string`  `ClassLabel` you should use [map()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Dataset.map) to update the Dataset.

**Returns:**

``IterableDataset``

A copy of the dataset with casted features.
#### decode[[datasets.IterableDataset.decode]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L4085)

Enable or disable the dataset features decoding for audio, image, video.

When enabled (default), media types are decoded:

* audio -> dict of "array" and "sampling_rate" and "path"
* image -> PIL.Image
* video -> torchcodec.decoders.VideoDecoder

You can enable multithreading using `num_threads`. This is especially useful to speed up remote
data streaming. However it can be slower than `num_threads=0` for local data on fast disks.

Disabling decoding is useful if you want to iterate on the paths or bytes of the media files
without actually decoding their content. To disable decoding you can use `.decode(False)`, which
is equivalent to calling `.cast()` or `.cast_column()` with all the Audio, Image and Video types
set to `decode=False`.

Examples:

Disable decoding:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("sshh12/planet-textures", split="train", streaming=True)
>>> next(iter(ds))
{'image': ,
'text': 'A distant celestial object with an icy crust, displaying a light blue shade, covered with round pits and rugged terrains.'}
>>> ds = ds.decode(False)
>>> ds.features
{'image': Image(mode=None, decode=False, id=None),
'text': Value('string')}
>>> next(iter(ds))
{
  'image': {
    'path': 'hf://datasets/sshh12/planet-textures@69dc4cef7a5c4b2cfe387727ec8ea73d4bff7302/train/textures/0000.png',
    'bytes': None
  },
  'text': 'A distant celestial object with an icy crust, displaying a light blue shade, covered with round pits and rugged terrains.'
}
```

Speed up streaming with multithreading:

```py
>>> import os
>>> from datasets import load_dataset
>>> from tqdm import tqdm
>>> ds = load_dataset("sshh12/planet-textures", split="train", streaming=True)
>>> num_threads = min(32, (os.cpu_count() or 1) + 4)
>>> ds = ds.decode(num_threads=num_threads)
>>> for _ in tqdm(ds):  # 20 times faster !
...     ...
```

**Parameters:**

enable (`bool`, defaults to `True`) : Enable or disable features decoding.

num_threads (`int`, defaults to `0`) : Enable multithreading for features decoding.

**Returns:**

``IterableDataset``

A copy of the dataset with casted features.
#### __iter__[[datasets.IterableDataset.__iter__]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2673)
#### iter[[datasets.IterableDataset.iter]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2694)

Iterate through the batches of size *batch_size*.

**Parameters:**

batch_size (`int`) : size of each batch to yield.

drop_last_batch (`bool`, default *False*) : Whether a last batch smaller than the batch_size should be dropped
#### map[[datasets.IterableDataset.map]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3314)

Apply a function to all the examples in the iterable dataset (individually or in batches) and update them.
If your function returns a column that already exists, then it overwrites it.
The function is applied on-the-fly on the examples when iterating over the dataset.

You can specify whether the function should be batched or not with the `batched` parameter:

- If batched is `False`, then the function takes 1 example in and should return 1 example.
  An example is a dictionary, e.g. `{"text": "Hello there !"}`.
- If batched is `True` and `batch_size` is 1, then the function takes a batch of 1 example as input and can return a batch with 1 or more examples.
  A batch is a dictionary, e.g. a batch of 1 example is {"text": ["Hello there !"]}.
- If batched is `True` and `batch_size` is `n` > 1, then the function takes a batch of `n` examples as input and can return a batch with `n` examples, or with an arbitrary number of examples.
  Note that the last batch may have less than `n` examples.
  A batch is a dictionary, e.g. a batch of `n` examples is `{"text": ["Hello there !"] * n}`.

If the function is asynchronous, then `map` will run your function in parallel, with up to one thousand simulatenous calls.
It is recommended to use a `asyncio.Semaphore` in your function if you want to set a maximum number of operations that can run at the same time.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train", streaming=True)
>>> def add_prefix(example):
...     example["text"] = "Review: " + example["text"]
...     return example
>>> ds = ds.map(add_prefix)
>>> list(ds.take(3))
[{'label': 1,
 'text': 'Review: the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'Review: the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'Review: effective but too-tepid biopic'}]
```

**Parameters:**

function (`Callable`, *optional*, defaults to `None`) : Function applied on-the-fly on the examples when you iterate on the dataset. It must have one of the following signatures:  - `function(example: Dict[str, Any]) -> Dict[str, Any]` if `batched=False` and `with_indices=False` - `function(example: Dict[str, Any], idx: int) -> Dict[str, Any]` if `batched=False` and `with_indices=True` - `function(batch: Dict[str, List]) -> Dict[str, List]` if `batched=True` and `with_indices=False` - `function(batch: Dict[str, List], indices: List[int]) -> Dict[str, List]` if `batched=True` and `with_indices=True`  For advanced usage, the function can also return a `pyarrow.Table`. If the function is asynchronous, then `map` will run your function in parallel. Moreover if your function returns nothing (`None`), then `map` will run your function and return the dataset unchanged. If no function is provided, default to identity function: `lambda x: x`.

with_indices (`bool`, defaults to `False`) : Provide example indices to `function`. Note that in this case the signature of `function` should be `def function(example, idx[, rank]): ...`.

input_columns (`Optional[Union[str, List[str]]]`, defaults to `None`) : The columns to be passed into `function` as positional arguments. If `None`, a dict mapping to all formatted columns is passed as one argument.

batched (`bool`, defaults to `False`) : Provide batch of examples to `function`.

batch_size (`int`, *optional*, defaults to `1000`) : Number of examples per batch provided to `function` if `batched=True`. `batch_size >> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train", streaming=True)
>>> next(iter(ds))
{'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'}
>>> ds = ds.rename_column("text", "movie_review")
>>> next(iter(ds))
{'label': 1,
 'movie_review': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'}
```

**Parameters:**

original_column_name (`str`) : Name of the column to rename.

new_column_name (`str`) : New name for the column.

**Returns:**

``IterableDataset``

A copy of the dataset with a renamed column.
#### filter[[datasets.IterableDataset.filter]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3469)

Apply a filter function to all the elements so that the dataset only includes examples according to the filter function.
The filtering is done on-the-fly when iterating over the dataset.

If the function is asynchronous, then `filter` will run your function in parallel, with up to one thousand simulatenous calls (configurable).
It is recommended to use a `asyncio.Semaphore` in your function if you want to set a maximum number of operations that can run at the same time.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train", streaming=True)
>>> ds = ds.filter(lambda x: x["label"] == 0)
>>> list(ds.take(3))
[{'label': 0, 'movie_review': 'simplistic , silly and tedious .'},
 {'label': 0,
 'movie_review': "it's so laddish and juvenile , only teenage boys could possibly find it funny ."},
 {'label': 0,
 'movie_review': 'exploitative and largely devoid of the depth or sophistication that would make watching such a graphic treatment of the crimes bearable .'}]
```

**Parameters:**

function (`Callable`) : Callable with one of the following signatures:  - `function(example: Dict[str, Any]) -> bool` if `with_indices=False, batched=False` - `function(example: Dict[str, Any], indices: int) -> bool` if `with_indices=True, batched=False` - `function(example: Dict[str, List]) -> List[bool]` if `with_indices=False, batched=True` - `function(example: Dict[str, List], indices: List[int]) -> List[bool]` if `with_indices=True, batched=True`  If the function is asynchronous, then `filter` will run your function in parallel. If no function is provided, defaults to an always True function: `lambda x: True`.

with_indices (`bool`, defaults to `False`) : Provide example indices to `function`. Note that in this case the signature of `function` should be `def function(example, idx): ...`.

input_columns (`str` or `List[str]`, *optional*) : The columns to be passed into `function` as positional arguments. If `None`, a dict mapping to all formatted columns is passed as one argument.

batched (`bool`, defaults to `False`) : Provide batch of examples to `function`.

batch_size (`int`, *optional*, default `1000`) : Number of examples per batch provided to `function` if `batched=True`.

fn_kwargs (`Dict`, *optional*, default `None`) : Keyword arguments to be passed to `function`.
#### shuffle[[datasets.IterableDataset.shuffle]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3554)

Randomly shuffles the elements of this dataset.

This dataset fills a buffer with `buffer_size` elements, then randomly samples elements from this buffer,
replacing the selected elements with new elements. For perfect shuffling, a buffer size greater than or
equal to the full size of the dataset is required.

For instance, if your dataset contains 10,000 elements but `buffer_size` is set to 1000, then `shuffle` will
initially select a random element from only the first 1000 elements in the buffer. Once an element is
selected, its space in the buffer is replaced by the next (i.e. 1,001-st) element,
maintaining the 1000 element buffer.

If the dataset is made of several shards, it also does shuffle the order of the shards.
However if the order has been fixed by using [skip()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset.skip) or [take()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset.take)
then the order of the shards is kept unchanged.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train", streaming=True)
>>> list(ds.take(3))
[{'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'effective but too-tepid biopic'}]
>>> shuffled_ds = ds.shuffle(seed=42)
>>> list(shuffled_ds.take(3))
[{'label': 1,
 'text': "a sports movie with action that's exciting on the field and a story you care about off it ."},
 {'label': 1,
 'text': 'at its best , the good girl is a refreshingly adult take on adultery . . .'},
 {'label': 1,
 'text': "sam jones became a very lucky filmmaker the day wilco got dropped from their record label , proving that one man's ruin may be another's fortune ."}]
```

**Parameters:**

seed (`int`, *optional*, defaults to `None`) : Random seed that will be used to shuffle the dataset. It is used to sample from the shuffle buffer and also to shuffle the data shards.

generator (`numpy.random.Generator`, *optional*) : Numpy random Generator to use to compute the permutation of the dataset rows. If `generator=None` (default), uses `np.random.default_rng` (the default BitGenerator (PCG64) of NumPy).

buffer_size (`int`, defaults to `1000`) : Size of the buffer.
#### batch[[datasets.IterableDataset.batch]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L4207)

Group samples from the dataset into batches.

Example:
```py
>>> ds = load_dataset("some_dataset", streaming=True)
>>> batched_ds = ds.batch(batch_size=32)
```

**Parameters:**

batch_size (`int`) : The number of samples in each batch.

drop_last_batch (`bool`, defaults to `False`) : Whether to drop the last incomplete batch.
#### skip[[datasets.IterableDataset.skip]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3626)

Create a new [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset) that skips the first `n` elements.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train", streaming=True)
>>> list(ds.take(3))
[{'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'effective but too-tepid biopic'}]
>>> ds = ds.skip(1)
>>> list(ds.take(3))
[{'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'effective but too-tepid biopic'},
 {'label': 1,
 'text': 'if you sometimes like to go to the movies to have fun , wasabi is a good place to start .'}]
```

**Parameters:**

n (`int`) : Number of elements to skip.
#### take[[datasets.IterableDataset.take]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3710)

Create a new [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset) with only the first `n` elements.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train", streaming=True)
>>> small_ds = ds.take(2)
>>> list(small_ds)
[{'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'}]
```

**Parameters:**

n (`int`) : Number of elements to take.
#### shard[[datasets.IterableDataset.shard]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3745)

Return the `index`-nth shard from dataset split into `num_shards` pieces.

This shards deterministically. `dataset.shard(n, i)` splits the dataset into contiguous chunks,
so it can be easily concatenated back together after processing. If `dataset.num_shards % n == l`, then the
first `l` datasets each have `(dataset.num_shards // n) + 1` shards, and the remaining datasets have `(dataset.num_shards // n)` shards.
`datasets.concatenate_datasets([dset.shard(n, i) for i in range(n)])` returns a dataset with the same order as the original.
In particular, `dataset.shard(dataset.num_shards, i)` returns a dataset with 1 shard.

Note: n should be less or equal to the number of shards in the dataset `dataset.num_shards`.

On the other hand, `dataset.shard(n, i, contiguous=False)` contains all the shards of the dataset whose index mod `n = i`.

Be sure to shard before using any randomizing operator (such as `shuffle`).
It is best if the shard operator is used early in the dataset pipeline.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("fancyzhx/amazon_polarity", split="train", streaming=True)
>>> ds
IterableDataset({
    features: ['label', 'title', 'content'],
    num_shards: 4
})
>>> ds.shard(num_shards=2, index=0)
IterableDataset({
    features: ['label', 'title', 'content'],
    num_shards: 2
})
```

**Parameters:**

num_shards (`int`) : How many shards to split the dataset into.

index (`int`) : Which shard to select and return.

contiguous : (`bool`, defaults to `True`): Whether to select contiguous blocks of indices for shards.
#### reshard[[datasets.IterableDataset.reshard]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3801)

Reshard the dataset if possible, i.e. split the current shards further into more shards.
This increases the number of shards and the resulting dataset has num_shards >= previous_num_shards.
Equality may happen if no shard can be split further.

The resharding mechanism depends on the dataset file format:

* Parquet: shard per row group instead of per file
* Other: not implemented yet (contributions are welcome !)

Be sure to reshard/shard before using any randomizing operator (such as `shuffle`).
It is best if the shard operator is used early in the dataset pipeline.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("fancyzhx/amazon_polarity", split="train", streaming=True)
>>> ds
IterableDataset({
    features: ['label', 'title', 'content'],
    num_shards: 4
})
>>> ds.reshard()
IterableDataset({
    features: ['label', 'title', 'content'],
    num_shards: 3600
})
```
#### repeat[[datasets.IterableDataset.repeat]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3668)

Create a new [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset) that repeats the underlying dataset `num_times` times.

N.B. The effect of calling shuffle after repeat depends significantly on buffer size.
With buffer_size 1, duplicate data is never seen in the same iteration, even after shuffling:
ds.repeat(n).shuffle(seed=42, buffer_size=1) is equivalent to ds.shuffle(seed=42, buffer_size=1).repeat(n),
and only shuffles shard orders within each iteration.
With buffer size >= (num samples in the dataset * num_times), we get full shuffling of the repeated data, i.e. we can observe duplicates in
the same iteration.

Example:
```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train")
>>> ds = ds.take(2).repeat(2)
>>> list(ds)
[{'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'effective but too-tepid biopic'},
 {'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'effective but too-tepid biopic'}]
```

**Parameters:**

num_times (`int`) or (`None`) : Number of times to repeat the dataset. If `None`, the dataset will be repeated indefinitely.
#### to_csv[[datasets.IterableDataset.to_csv]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L4345)

Exports the dataset to csv.

This iterates on the dataset and loads it completely in memory before writing it.

Example:

```py
>>> ds.to_csv("path/to/dataset/directory")
```

**Parameters:**

path_or_buf (`PathLike` or `FileOrBuffer`) : Either a path to a file (e.g. `file.csv`), a remote URI (e.g. `hf://datasets/username/my_dataset_name/data.csv`), or a BinaryIO, where the dataset will be saved to in the specified format.

batch_size (`int`, *optional*) : Size of the batch to load in memory and write at once. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.

- ****to_csv_kwargs** (additional keyword arguments) : Parameters to pass to pandas's [`pandas.DataFrame.to_csv`](https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.to_csv.html). The parameter `index` defaults to `False` if not specified. If you would like to write the index, pass `index=True` and also set a name for the index column by passing `index_label`.

**Returns:**

``int``

The number of characters or bytes written.
#### to_pandas[[datasets.IterableDataset.to_pandas]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L4268)

Returns the dataset as a `pandas.DataFrame`. Can also return a generator for large datasets.

Example:

```py
>>> ds.to_pandas()
```

**Parameters:**

batch_size (`int`, *optional*) : The size (number of rows) of the batches if `batched` is `True`. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

batched (`bool`) : Set to `True` to return a generator that yields the dataset as batches of `batch_size` rows. Defaults to `False` (returns the whole datasets once).

**Returns:**

`pandas.DataFrame` or `Iterator[pandas.DataFrame]`
#### to_dict[[datasets.IterableDataset.to_dict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L4230)

Returns the dataset as a Python dict. Can also return a generator for large datasets.

Example:

```py
>>> ds.to_dict()
```

**Parameters:**

batch_size (`int`, *optional*) : The size (number of rows) of the batches if `batched` is `True`. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

**Returns:**

`dict` or `Iterator[dict]`
#### to_json[[datasets.IterableDataset.to_json]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L4388)

Export the dataset to JSON Lines or JSON.

This iterates on the dataset and loads it completely in memory before writing it.

The default output format is [JSON Lines](https://jsonlines.org/).
To export to [JSON](https://www.json.org), pass `lines=False` argument and the desired `orient`.

Example:

```py
>>> ds.to_json("path/to/dataset/directory/filename.jsonl")
```

```py
>>> num_shards = dataset.num_shards
>>> for index in range(num_shards):
...     shard = dataset.shard(index, num_shards)
...     shard.to_json(f"path/of/my/dataset/data-{index:05d}.jsonl")
```

**Parameters:**

path_or_buf (`PathLike` or `FileOrBuffer`) : Either a path to a file (e.g. `file.json`), a remote URI (e.g. `hf://datasets/username/my_dataset_name/data.json`), or a BinaryIO, where the dataset will be saved to in the specified format.

batch_size (`int`, *optional*) : Size of the batch to load in memory and write at once. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.

- ****to_json_kwargs** (additional keyword arguments) : Parameters to pass to pandas's [`pandas.DataFrame.to_json`](https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.to_json.html). Default arguments are `lines=True` and `orient="records". The parameter `index` defaults to `False` if `orient` is `"split"` or `"table"`. If you would like to write the index, pass `index=True`.

**Returns:**

``int``

The number of characters or bytes written.
#### to_parquet[[datasets.IterableDataset.to_parquet]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L4484)

Exports the dataset to parquet

Example:

```py
>>> ds.to_parquet("path/to/dataset/directory")
```

```py
>>> num_shards = dataset.num_shards
>>> for index in range(num_shards):
...     shard = dataset.shard(index, num_shards)
...     shard.to_parquet(f"path/of/my/dataset/data-{index:05d}.parquet")
```

**Parameters:**

path_or_buf (`PathLike` or `FileOrBuffer`) : Either a path to a file (e.g. `file.parquet`), a remote URI (e.g. `hf://datasets/username/my_dataset_name/data.parquet`), or a BinaryIO, where the dataset will be saved to in the specified format.

batch_size (`int`, *optional*) : Size of the batch to load in memory and write at once. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

storage_options (`dict`, *optional*) : Key/value pairs to be passed on to the file-system backend, if any.  

- ****parquet_writer_kwargs** (additional keyword arguments) : Parameters to pass to PyArrow's `pyarrow.parquet.ParquetWriter`.

**Returns:**

``int``

The number of characters or bytes written.
#### to_sql[[datasets.IterableDataset.to_sql]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L4442)

Exports the dataset to a SQL database.

Example:

```py
>>> # con provided as a connection URI string
>>> ds.to_sql("data", "sqlite:///my_own_db.sql")
>>> # con provided as a sqlite3 connection object
>>> import sqlite3
>>> con = sqlite3.connect("my_own_db.sql")
>>> with con:
...     ds.to_sql("data", con)
```

**Parameters:**

name (`str`) : Name of SQL table.

con (`str` or `sqlite3.Connection` or `sqlalchemy.engine.Connection` or `sqlalchemy.engine.Connection`) : A [URI string](https://docs.sqlalchemy.org/en/13/core/engines.html#database-urls) or a SQLite3/SQLAlchemy connection object used to write to a database.

batch_size (`int`, *optional*) : Size of the batch to load in memory and write at once. Defaults to `datasets.config.DEFAULT_MAX_BATCH_SIZE`.

- ****sql_writer_kwargs** (additional keyword arguments) : Parameters to pass to pandas's [`pandas.DataFrame.to_sql`](https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.to_sql.html). The parameter `index` defaults to `False` if not specified. If you would like to write the index, pass `index=True` and also set a name for the index column by passing `index_label`.

**Returns:**

``int``

The number of records written.
#### push_to_hub[[datasets.IterableDataset.push_to_hub]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L4743)

Pushes the dataset to the hub as a Parquet dataset.
The dataset is pushed using HTTP requests and does not need to have neither git or git-lfs installed.

The resulting Parquet files are self-contained by default. If your dataset contains [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image), [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio) or [Video](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Video)
data, the Parquet files will store the bytes of your images or audio files.
You can disable this by setting `embed_external_files` to `False`.

Example:

```python
>>> dataset.push_to_hub("/")
>>> dataset_dict.push_to_hub("/", private=True)
>>> dataset.push_to_hub("/", max_shard_size="1GB")
>>> dataset.push_to_hub("/", num_shards=1024)
```

If your dataset has multiple splits (e.g. train/validation/test):

```python
>>> train_dataset.push_to_hub("/", split="train")
>>> val_dataset.push_to_hub("/", split="validation")
>>> # later
>>> dataset = load_dataset("/")
>>> train_dataset = dataset["train"]
>>> val_dataset = dataset["validation"]
```

If you want to add a new configuration (or subset) to a dataset (e.g. if the dataset has multiple tasks/versions/languages):

```python
>>> english_dataset.push_to_hub("/", "en")
>>> french_dataset.push_to_hub("/", "fr")
>>> # later
>>> english_dataset = load_dataset("/", "en")
>>> french_dataset = load_dataset("/", "fr")
```

**Parameters:**

repo_id (`str`) : The ID of the repository to push to in the following format: `/` or `/`. Also accepts ``, which will default to the namespace of the logged-in user.  It could also be a location inside a bucket, e.g. `buckets///...`

config_name (`str`, defaults to "default") : The configuration name (or subset) of a dataset. Defaults to "default".

set_default (`bool`, *optional*) : Whether to set this configuration as the default one. Otherwise, the default configuration is the one named "default".

split (`str`, *optional*) : The name of the split that will be given to that dataset. Defaults to `self.split`.

data_dir (`str`, *optional*) : Directory name that will contain the uploaded data files. Defaults to the `config_name` if different from "default", else "data".

commit_message (`str`, *optional*) : Message to commit while pushing. Will default to `"Upload dataset"`.

commit_description (`str`, *optional*) : Description of the commit that will be created. Additionally, description of the PR if a PR is created (`create_pr` is True).

private (`bool`, *optional*) : Whether to make the repo private. If `None` (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

token (`str`, *optional*) : An optional authentication token for the Hugging Face Hub. If no token is passed, will default to the token saved locally when logging in with `huggingface-cli login`. Will raise an error if no token is passed and the user is not logged-in.

revision (`str`, *optional*) : Branch to push the uploaded files to. Defaults to the `"main"` branch.

create_pr (`bool`, *optional*, defaults to `False`) : Whether to create a PR with the uploaded files or directly commit.

max_shard_size (`int` or `str`, *optional*) : Optional maximum size of the dataset shards to be uploaded to the hub. If expressed as a string, needs to be digits followed by a unit (like `"5MB"`). If not provided, shard count defaults to this dataset's `.num_shards`.

num_shards (`int`, *optional*) : Number of shards to write. If `max_shard_size` is provided and `num_shards` is not, then the number of shards is estimated from `max_shard_size`.

embed_external_files (`bool`, defaults to `True`) : Whether to embed file bytes in the shards. In particular, this will do the following before the push for the fields of type:  - [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio) and [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image): remove local path information and embed file content in the Parquet files.

num_proc (`int`, *optional*, defaults to `None`) : Number of processes when preparing and uploading the dataset. This is helpful if the dataset is made of many samples and transformations. I uses "spawn" context to work with hf_xet, the rust client for fast uploads to HF. Multiprocessing is disabled by default.

**Returns:**

huggingface_hub.CommitInfo
#### load_state_dict[[datasets.IterableDataset.load_state_dict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2467)

Load the state_dict of the dataset.
The iteration will restart at the next example from when the state was saved.

Resuming returns exactly where the checkpoint was saved except in two cases:

1. examples from shuffle buffers are lost when resuming and the buffers are refilled with new data
2. combinations of `.with_format(arrow)` and batched `.map()` may skip one batch.

Example:

```py
>>> from datasets import Dataset, concatenate_datasets
>>> ds = Dataset.from_dict({"a": range(6)}).to_iterable_dataset(num_shards=3)
>>> for idx, example in enumerate(ds):
...     print(example)
...     if idx == 2:
...         state_dict = ds.state_dict()
...         print("checkpoint")
...         break
>>> ds.load_state_dict(state_dict)
>>> print(f"restart from checkpoint")
>>> for example in ds:
...     print(example)
```

which returns:
```
{'a': 0}
{'a': 1}
{'a': 2}
checkpoint
restart from checkpoint
{'a': 3}
{'a': 4}
{'a': 5}
```

```py
>>> from torchdata.stateful_dataloader import StatefulDataLoader
>>> ds = load_dataset("deepmind/code_contests", streaming=True, split="train")
>>> dataloader = StatefulDataLoader(ds, batch_size=32, num_workers=4)
>>> # checkpoint
>>> state_dict = dataloader.state_dict()  # uses ds.state_dict() under the hood
>>> # resume from checkpoint
>>> dataloader.load_state_dict(state_dict)  # uses ds.load_state_dict() under the hood
```
#### state_dict[[datasets.IterableDataset.state_dict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2414)

Get the current state_dict of the dataset.
It corresponds to the state at the latest example it yielded.

Resuming returns exactly where the checkpoint was saved except in two cases:

1. examples from shuffle buffers are lost when resuming and the buffers are refilled with new data
2. combinations of `.with_format(arrow)` and batched `.map()` may skip one batch.

Example:

```py
>>> from datasets import Dataset, concatenate_datasets
>>> ds = Dataset.from_dict({"a": range(6)}).to_iterable_dataset(num_shards=3)
>>> for idx, example in enumerate(ds):
...     print(example)
...     if idx == 2:
...         state_dict = ds.state_dict()
...         print("checkpoint")
...         break
>>> ds.load_state_dict(state_dict)
>>> print(f"restart from checkpoint")
>>> for example in ds:
...     print(example)
```

which returns:
```
{'a': 0}
{'a': 1}
{'a': 2}
checkpoint
restart from checkpoint
{'a': 3}
{'a': 4}
{'a': 5}
```

```py
>>> from torchdata.stateful_dataloader import StatefulDataLoader
>>> ds = load_dataset("deepmind/code_contests", streaming=True, split="train")
>>> dataloader = StatefulDataLoader(ds, batch_size=32, num_workers=4)
>>> # checkpoint
>>> state_dict = dataloader.state_dict()  # uses ds.state_dict() under the hood
>>> # resume from checkpoint
>>> dataloader.load_state_dict(state_dict)  # uses ds.load_state_dict() under the hood
```

**Returns:**

``dict``
#### info[[datasets.IterableDataset.info]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L179)

[DatasetInfo](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.DatasetInfo) object containing all the metadata in the dataset.
#### split[[datasets.IterableDataset.split]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L184)

[NamedSplit](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.NamedSplit) object corresponding to a named dataset split.
#### builder_name[[datasets.IterableDataset.builder_name]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L189)
#### citation[[datasets.IterableDataset.citation]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L193)
#### config_name[[datasets.IterableDataset.config_name]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L197)
#### dataset_size[[datasets.IterableDataset.dataset_size]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L201)
#### description[[datasets.IterableDataset.description]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L205)
#### download_checksums[[datasets.IterableDataset.download_checksums]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L209)
#### download_size[[datasets.IterableDataset.download_size]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L213)
#### features[[datasets.IterableDataset.features]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L217)
#### homepage[[datasets.IterableDataset.homepage]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L221)
#### license[[datasets.IterableDataset.license]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L225)
#### size_in_bytes[[datasets.IterableDataset.size_in_bytes]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L229)
#### supervised_keys[[datasets.IterableDataset.supervised_keys]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L233)
#### version[[datasets.IterableDataset.version]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/arrow_dataset.py#L237)
#### from_csv[[datasets.IterableDataset.from_csv]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3030)

Create an IterableDataset from CSV file(s).

Example:

```py
>>> ds = IterableDataset.from_csv('path/to/dataset.csv')
```

**Parameters:**

path_or_paths (`path-like` or list of `path-like`) : Path(s) of the CSV file(s).

split ([NamedSplit](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.NamedSplit), *optional*) : Split name to be assigned to the dataset.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `pandas.read_csv`.

**Returns:**

[IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)
#### from_json[[datasets.IterableDataset.from_json]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3073)

Create an IterableDataset from JSON or JSON Lines file(s).

Example:

```py
>>> ds = IterableDataset.from_json('path/to/dataset.json')
```

**Parameters:**

path_or_paths (`path-like` or list of `path-like`) : Path(s) of the JSON or JSON Lines file(s).

split ([NamedSplit](/docs/datasets/v4.8.4/en/package_reference/builder_classes#datasets.NamedSplit), *optional*) : Split name to be assigned to the dataset.

features ([Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features), *optional*) : Dataset features.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

field (`str`, *optional*) : Field name of the JSON file where the dataset is contained in.

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `JsonConfig`.

**Returns:**

[IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)
#### from_parquet[[datasets.IterableDataset.from_parquet]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3120)

Create an IterableDataset from Parquet file(s).

Example:

```py
>>> ds = IterableDataset.from_parquet('path/to/dataset.parquet')
```

Load a subset of columns:

```python
>>> ds = IterableDataset.from_parquet('path/to/dataset.parquet', columns=["col_0", "col_1"])
```

Efficiently filter data, possibly skipping entire files or row groups:

```python
>>> filters = [("col_0", "==", 0)]
>>> ds = IterableDataset.from_parquet(parquet_files_list, filters=filters)
```

**Parameters:**

path_or_paths (`path-like` or list of `path-like`) : Path(s) of the Parquet file(s).

split (`NamedSplit`, *optional*) : Split name to be assigned to the dataset.

features (`Features`, *optional*) : Dataset features.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

columns (`List[str]`, *optional*) : If not `None`, only these columns will be read from the file. A column name may be a prefix of a nested field, e.g. 'a' will select 'a.b', 'a.c', and 'a.d.e'.

filters (`Union[pyarrow.dataset.Expression, list[tuple], list[list[tuple]]]`, *optional*) : Return only the rows matching the filter. If possible the predicate will be pushed down to exploit the partition information or internal metadata found in the data source, e.g. Parquet statistics. Otherwise filters the loaded RecordBatches before yielding them.

fragment_scan_options (`pyarrow.dataset.ParquetFragmentScanOptions`, *optional*) : Scan-specific options for Parquet fragments. This is especially useful to configure buffering and caching.  

on_bad_files (`Literal["error", "warn", "skip"]`, *optional*, defaults to "error") : Specify what to do upon encountering a bad file (a file that can't be read). Allowed values are : * 'error', raise an Exception when a bad file is encountered. * 'warn', raise a warning when a bad file is encountered and skip that file. * 'skip', skip bad files without raising or warning when they are encountered.  

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `ParquetConfig`.

**Returns:**

[IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)
#### from_text[[datasets.IterableDataset.from_text]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L3205)

Create an IterableDataset from text file(s).

Example:

```py
>>> ds = IterableDataset.from_text('path/to/dataset.txt')
```

**Parameters:**

path_or_paths (`path-like` or list of `path-like`) : Path(s) of the text file(s).

split (`NamedSplit`, *optional*) : Split name to be assigned to the dataset.

features (`Features`, *optional*) : Dataset features.

keep_in_memory (`bool`, defaults to `False`) : Whether to copy the data in-memory.

keep_linebreaks : (`bool`, defaults to False): Whether to keep line breaks.

sample_by (`Literal["line", "paragraph", "document"]`, defaults to "line") : Whether to load data per line, praragraph or document. By default one row in the dataset = one line.

- ****kwargs** (additional keyword arguments) : Keyword arguments to be passed to `TextConfig`.

**Returns:**

[IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset)

#### datasets.IterableColumn[[datasets.IterableColumn]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/iterable_dataset.py#L2324)

An iterable for a specific column of an [IterableDataset](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset).

Example:

Iterate on the texts of the "text" column of a dataset:

```python
for text in dataset["text"]:
    ...
```

It also works with nested columns:

```python
for source in dataset["metadata"]["source"]:
    ...
```

## IterableDatasetDict[[datasets.IterableDatasetDict]]

Dictionary with split names as keys ('train', 'test' for example), and `IterableDataset` objects as values.

#### datasets.IterableDatasetDict[[datasets.IterableDatasetDict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1822)

mapdatasets.IterableDatasetDict.maphttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1923[{"name": "function", "val": ": typing.Optional[typing.Callable] = None"}, {"name": "with_indices", "val": ": bool = False"}, {"name": "with_split", "val": ": bool = False"}, {"name": "input_columns", "val": ": typing.Union[str, list[str], NoneType] = None"}, {"name": "batched", "val": ": bool = False"}, {"name": "batch_size", "val": ": int = 1000"}, {"name": "drop_last_batch", "val": ": bool = False"}, {"name": "remove_columns", "val": ": typing.Union[str, list[str], NoneType] = None"}, {"name": "fn_kwargs", "val": ": typing.Optional[dict] = None"}]- **function** (`Callable`, *optional*, defaults to `None`) --
  Function applied on-the-fly on the examples when you iterate on the dataset.
  It must have one of the following signatures:

  - `function(example: Dict[str, Any]) -> Dict[str, Any]` if `batched=False` and `with_indices=False`
  - `function(example: Dict[str, Any], idx: int) -> Dict[str, Any]` if `batched=False` and `with_indices=True`
  - `function(batch: Dict[str, list]) -> Dict[str, list]` if `batched=True` and `with_indices=False`
  - `function(batch: Dict[str, list], indices: list[int]) -> Dict[str, list]` if `batched=True` and `with_indices=True`

  For advanced usage, the function can also return a `pyarrow.Table`.
  If the function is asynchronous, then `map` will run your function in parallel.
  Moreover if your function returns nothing (`None`), then `map` will run your function and return the dataset unchanged.
  If no function is provided, default to identity function: `lambda x: x`.
- **with_indices** (`bool`, defaults to `False`) --
  Provide example indices to `function`. Note that in this case the signature of `function` should be `def function(example, idx[, rank]): ...`.
- **input_columns** (`[Union[str, list[str]]]`, *optional*, defaults to `None`) --
  The columns to be passed into `function`
  as positional arguments. If `None`, a dict mapping to all formatted columns is passed as one argument.
- **batched** (`bool`, defaults to `False`) --
  Provide batch of examples to `function`.
- **batch_size** (`int`, *optional*, defaults to `1000`) --
  Number of examples per batch provided to `function` if `batched=True`.
- **drop_last_batch** (`bool`, defaults to `False`) --
  Whether a last batch smaller than the `batch_size` should be
  dropped instead of being processed by the function.
- **remove_columns** (`[list[str]]`, *optional*, defaults to `None`) --
  Remove a selection of columns while doing the mapping.
  Columns will be removed before updating the examples with the output of `function`, i.e. if `function` is adding
  columns with names in `remove_columns`, these columns will be kept.
- **fn_kwargs** (`Dict`, *optional*, defaults to `None`) --
  Keyword arguments to be passed to `function`0

Apply a function to all the examples in the iterable dataset (individually or in batches) and update them.
If your function returns a column that already exists, then it overwrites it.
The function is applied on-the-fly on the examples when iterating over the dataset.
The transformation is applied to all the datasets of the dataset dictionary.

You can specify whether the function should be batched or not with the `batched` parameter:

- If batched is `False`, then the function takes 1 example in and should return 1 example.
  An example is a dictionary, e.g. `{"text": "Hello there !"}`.
- If batched is `True` and `batch_size` is 1, then the function takes a batch of 1 example as input and can return a batch with 1 or more examples.
  A batch is a dictionary, e.g. a batch of 1 example is `{"text": ["Hello there !"]}`.
- If batched is `True` and `batch_size` is `n` > 1, then the function takes a batch of `n` examples as input and can return a batch with `n` examples, or with an arbitrary number of examples.
  Note that the last batch may have less than `n` examples.
  A batch is a dictionary, e.g. a batch of `n` examples is `{"text": ["Hello there !"] * n}`.

If the function is asynchronous, then `map` will run your function in parallel, with up to one thousand simultaneous calls.
It is recommended to use a `asyncio.Semaphore` in your function if you want to set a maximum number of operations that can run at the same time.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", streaming=True)
>>> def add_prefix(example):
...     example["text"] = "Review: " + example["text"]
...     return example
>>> ds = ds.map(add_prefix)
>>> next(iter(ds["train"]))
{'label': 1,
 'text': 'Review: the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'}
```

**Parameters:**

function (`Callable`, *optional*, defaults to `None`) : Function applied on-the-fly on the examples when you iterate on the dataset. It must have one of the following signatures:  - `function(example: Dict[str, Any]) -> Dict[str, Any]` if `batched=False` and `with_indices=False` - `function(example: Dict[str, Any], idx: int) -> Dict[str, Any]` if `batched=False` and `with_indices=True` - `function(batch: Dict[str, list]) -> Dict[str, list]` if `batched=True` and `with_indices=False` - `function(batch: Dict[str, list], indices: list[int]) -> Dict[str, list]` if `batched=True` and `with_indices=True`  For advanced usage, the function can also return a `pyarrow.Table`. If the function is asynchronous, then `map` will run your function in parallel. Moreover if your function returns nothing (`None`), then `map` will run your function and return the dataset unchanged. If no function is provided, default to identity function: `lambda x: x`.

with_indices (`bool`, defaults to `False`) : Provide example indices to `function`. Note that in this case the signature of `function` should be `def function(example, idx[, rank]): ...`.

input_columns (`[Union[str, list[str]]]`, *optional*, defaults to `None`) : The columns to be passed into `function` as positional arguments. If `None`, a dict mapping to all formatted columns is passed as one argument.

batched (`bool`, defaults to `False`) : Provide batch of examples to `function`.

batch_size (`int`, *optional*, defaults to `1000`) : Number of examples per batch provided to `function` if `batched=True`.

drop_last_batch (`bool`, defaults to `False`) : Whether a last batch smaller than the `batch_size` should be dropped instead of being processed by the function.

remove_columns (`[list[str]]`, *optional*, defaults to `None`) : Remove a selection of columns while doing the mapping. Columns will be removed before updating the examples with the output of `function`, i.e. if `function` is adding columns with names in `remove_columns`, these columns will be kept.

fn_kwargs (`Dict`, *optional*, defaults to `None`) : Keyword arguments to be passed to `function`
#### filter[[datasets.IterableDatasetDict.filter]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L2023)

Apply a filter function to all the elements so that the dataset only includes examples according to the filter function.
The filtering is done on-the-fly when iterating over the dataset.
The filtering is applied to all the datasets of the dataset dictionary.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", streaming=True)
>>> ds = ds.filter(lambda x: x["label"] == 0)
>>> list(ds["train"].take(3))
[{'label': 0, 'text': 'Review: simplistic , silly and tedious .'},
 {'label': 0,
 'text': "Review: it's so laddish and juvenile , only teenage boys could possibly find it funny ."},
 {'label': 0,
 'text': 'Review: exploitative and largely devoid of the depth or sophistication that would make watching such a graphic treatment of the crimes bearable .'}]
```

**Parameters:**

function (`Callable`) : Callable with one of the following signatures:  - `function(example: Dict[str, Any]) -> bool` if `with_indices=False, batched=False` - `function(example: Dict[str, Any], indices: int) -> bool` if `with_indices=True, batched=False` - `function(example: Dict[str, list]) -> list[bool]` if `with_indices=False, batched=True` - `function(example: Dict[str, list], indices: list[int]) -> list[bool]` if `with_indices=True, batched=True`  If no function is provided, defaults to an always True function: `lambda x: True`.

with_indices (`bool`, defaults to `False`) : Provide example indices to `function`. Note that in this case the signature of `function` should be `def function(example, idx): ...`.

input_columns (`str` or `list[str]`, *optional*) : The columns to be passed into `function` as positional arguments. If `None`, a dict mapping to all formatted columns is passed as one argument.

batched (`bool`, defaults to `False`) : Provide batch of examples to `function`

batch_size (`int`, *optional*, defaults to `1000`) : Number of examples per batch provided to `function` if `batched=True`.

fn_kwargs (`Dict`, *optional*, defaults to `None`) : Keyword arguments to be passed to `function`
#### shuffle[[datasets.IterableDatasetDict.shuffle]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L2086)

Randomly shuffles the elements of this dataset.
The shuffling is applied to all the datasets of the dataset dictionary.

This dataset fills a buffer with buffer_size elements, then randomly samples elements from this buffer,
replacing the selected elements with new elements. For perfect shuffling, a buffer size greater than or
equal to the full size of the dataset is required.

For instance, if your dataset contains 10,000 elements but `buffer_size` is set to 1000, then `shuffle` will
initially select a random element from only the first 1000 elements in the buffer. Once an element is
selected, its space in the buffer is replaced by the next (i.e. 1,001-st) element,
maintaining the 1000 element buffer.

If the dataset is made of several shards, it also does `shuffle` the order of the shards.
However if the order has been fixed by using [skip()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset.skip) or [take()](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDataset.take)
then the order of the shards is kept unchanged.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", streaming=True)
>>> list(ds["train"].take(3))
[{'label': 1,
 'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'},
 {'label': 1,
 'text': 'the gorgeously elaborate continuation of " the lord of the rings " trilogy is so huge that a column of words cannot adequately describe co-writer/director peter jackson's expanded vision of j . r . r . tolkien's middle-earth .'},
 {'label': 1, 'text': 'effective but too-tepid biopic'}]
>>> ds = ds.shuffle(seed=42)
>>> list(ds["train"].take(3))
[{'label': 1,
 'text': "a sports movie with action that's exciting on the field and a story you care about off it ."},
 {'label': 1,
 'text': 'at its best , the good girl is a refreshingly adult take on adultery . . .'},
 {'label': 1,
 'text': "sam jones became a very lucky filmmaker the day wilco got dropped from their record label , proving that one man's ruin may be another's fortune ."}]
```

**Parameters:**

seed (`int`, *optional*, defaults to `None`) : Random seed that will be used to shuffle the dataset. It is used to sample from the shuffle buffer and also to shuffle the data shards.

generator (`numpy.random.Generator`, *optional*) : Numpy random Generator to use to compute the permutation of the dataset rows. If `generator=None` (default), uses `np.random.default_rng` (the default BitGenerator (PCG64) of NumPy).

buffer_size (`int`, defaults to `1000`) : Size of the buffer.
#### with_format[[datasets.IterableDatasetDict.with_format]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L1877)

Return a dataset with the specified format.

Example:

```py
>>> from datasets import load_dataset
>>> from transformers import AutoTokenizer
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="validation", streaming=True)
>>> tokenizer = AutoTokenizer.from_pretrained("bert-base-cased")
>>> ds = ds.map(lambda x: tokenizer(x['text'], truncation=True, padding=True), batched=True)
>>> ds = ds.with_format("torch")
>>> next(iter(ds))
{'text': 'compassionately explores the seemingly irreconcilable situation between conservative christian parents and their estranged gay and lesbian children .',
 'label': tensor(1),
 'input_ids': tensor([  101, 18027, 16310, 16001,  1103,  9321,   178, 11604,  7235,  6617,
        1742,  2165,  2820,  1206,  6588, 22572, 12937,  1811,  2153,  1105,
        1147, 12890, 19587,  6463,  1105, 15026,  1482,   119,   102,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
            0,     0,     0,     0]),
 'token_type_ids': tensor([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
 'attention_mask': tensor([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])}
```

**Parameters:**

type (`str`, *optional*) : Either output type selected in `[None, 'numpy', 'torch', 'tensorflow', 'jax', 'arrow', 'pandas', 'polars']`. `None` means it returns python objects (default).
#### cast[[datasets.IterableDatasetDict.cast]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L2294)

Cast the dataset to a new set of features.
The type casting is applied to all the datasets of the dataset dictionary.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", streaming=True)
>>> ds["train"].features
{'label': ClassLabel(names=['neg', 'pos']),
 'text': Value('string')}
>>> new_features = ds["train"].features.copy()
>>> new_features['label'] = ClassLabel(names=['bad', 'good'])
>>> new_features['text'] = Value('large_string')
>>> ds = ds.cast(new_features)
>>> ds["train"].features
{'label': ClassLabel(names=['bad', 'good']),
 'text': Value('large_string')}
```

**Parameters:**

features (`Features`) : New features to cast the dataset to. The name of the fields in the features must match the current column names. The type of the data must also be convertible from one type to the other. For non-trivial conversion, e.g. `string`  `ClassLabel` you should use `map` to update the Dataset.

**Returns:**

`[IterableDatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDatasetDict)`

A copy of the dataset with casted features.
#### cast_column[[datasets.IterableDatasetDict.cast_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L2263)

Cast column to feature for decoding.
The type casting is applied to all the datasets of the dataset dictionary.

Example:

```py
>>> from datasets import load_dataset, ClassLabel
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", streaming=True)
>>> ds["train"].features
{'label': ClassLabel(names=['neg', 'pos']),
 'text': Value('string')}
>>> ds = ds.cast_column('label', ClassLabel(names=['bad', 'good']))
>>> ds["train"].features
{'label': ClassLabel(names=['bad', 'good']),
 'text': Value('string')}
```

**Parameters:**

column (`str`) : Column name.

feature (`Feature`) : Target feature.

**Returns:**

[IterableDatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDatasetDict)
#### remove_columns[[datasets.IterableDatasetDict.remove_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L2211)

Remove one or several column(s) in the dataset and the features associated to them.
The removal is done on-the-fly on the examples when iterating over the dataset.
The removal is applied to all the datasets of the dataset dictionary.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", streaming=True)
>>> ds = ds.remove_columns("label")
>>> next(iter(ds["train"]))
{'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'}
```

**Parameters:**

column_names (`Union[str, list[str]]`) : Name of the column(s) to remove.

**Returns:**

`[IterableDatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDatasetDict)`

A copy of the dataset object without the columns to remove.
#### rename_column[[datasets.IterableDatasetDict.rename_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L2147)

Rename a column in the dataset, and move the features associated to the original column under the new column
name.
The renaming is applied to all the datasets of the dataset dictionary.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", streaming=True)
>>> ds = ds.rename_column("text", "movie_review")
>>> next(iter(ds["train"]))
{'label': 1,
 'movie_review': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'}
```

**Parameters:**

original_column_name (`str`) : Name of the column to rename.

new_column_name (`str`) : New name for the column.

**Returns:**

`[IterableDatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDatasetDict)`

A copy of the dataset with a renamed column.
#### rename_columns[[datasets.IterableDatasetDict.rename_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L2183)

Rename several columns in the dataset, and move the features associated to the original columns under
the new column names.
The renaming is applied to all the datasets of the dataset dictionary.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", streaming=True)
>>> ds = ds.rename_columns({"text": "movie_review", "label": "rating"})
>>> next(iter(ds["train"]))
{'movie_review': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .',
 'rating': 1}
```

**Parameters:**

column_mapping (`Dict[str, str]`) : A mapping of columns to rename to their new names.

**Returns:**

`[IterableDatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDatasetDict)`

A copy of the dataset with renamed columns
#### select_columns[[datasets.IterableDatasetDict.select_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L2237)

Select one or several column(s) in the dataset and the features
associated to them. The selection is done on-the-fly on the examples
when iterating over the dataset. The selection is applied to all the
datasets of the dataset dictionary.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", streaming=True)
>>> ds = ds.select("text")
>>> next(iter(ds["train"]))
{'text': 'the rock is destined to be the 21st century's new " conan " and that he's going to make a splash even greater than arnold schwarzenegger , jean-claud van damme or steven segal .'}
```

**Parameters:**

column_names (`Union[str, list[str]]`) : Name of the column(s) to keep.

**Returns:**

`[IterableDatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDatasetDict)`

A copy of the dataset object with only selected columns.
#### push_to_hub[[datasets.IterableDatasetDict.push_to_hub]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/dataset_dict.py#L2331)

Pushes the [IterableDatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDatasetDict) to the hub as a Parquet dataset.
The [IterableDatasetDict](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.IterableDatasetDict) is pushed using HTTP requests and does not need to have neither git or git-lfs installed.

Each dataset split will be pushed independently. The pushed dataset will keep the original split names.

The resulting Parquet files are self-contained by default: if your dataset contains [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image) or [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio)
data, the Parquet files will store the bytes of your images or audio files.
You can disable this by setting `embed_external_files` to False.

Example:

```python
>>> dataset_dict.push_to_hub("/")
>>> dataset_dict.push_to_hub("/", private=True)
>>> dataset_dict.push_to_hub("/", max_shard_size="1GB")
>>> dataset_dict.push_to_hub("/", num_shards={"train": 1024, "test": 8})
```

If you want to add a new configuration (or subset) to a dataset (e.g. if the dataset has multiple tasks/versions/languages):

```python
>>> english_dataset.push_to_hub("/", "en")
>>> french_dataset.push_to_hub("/", "fr")
>>> # later
>>> english_dataset = load_dataset("/", "en")
>>> french_dataset = load_dataset("/", "fr")
```

**Parameters:**

repo_id (`str`) : The ID of the repository to push to in the following format: `/` or `/`. Also accepts ``, which will default to the namespace of the logged-in user.  It could also be a location inside a bucket, e.g. `buckets///...`

config_name (`str`) : Configuration name of a dataset. Defaults to "default".

set_default (`bool`, *optional*) : Whether to set this configuration as the default one. Otherwise, the default configuration is the one named "default".

data_dir (`str`, *optional*) : Directory name that will contain the uploaded data files. Defaults to the `config_name` if different from "default", else "data".  

commit_message (`str`, *optional*) : Message to commit while pushing. Will default to `"Upload dataset"`.

commit_description (`str`, *optional*) : Description of the commit that will be created. Additionally, description of the PR if a PR is created (`create_pr` is True).  

private (`bool`, *optional*) : Whether to make the repo private. If `None` (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

token (`str`, *optional*) : An optional authentication token for the Hugging Face Hub. If no token is passed, will default to the token saved locally when logging in with `huggingface-cli login`. Will raise an error if no token is passed and the user is not logged-in.

revision (`str`, *optional*) : Branch to push the uploaded files to. Defaults to the `"main"` branch.

create_pr (`bool`, *optional*, defaults to `False`) : Whether to create a PR with the uploaded files or directly commit.

max_shard_size (`int` or `str`, *optional*) : Optional maximum size of the dataset shards to be uploaded to the hub. If expressed as a string, needs to be digits followed by a unit (like `"500MB"` or `"1GB"`). If not provided, each split keeps its dataset-native shard count.

num_shards (`Dict[str, int]`, *optional*) : Number of shards to write. Use a dictionary to define a different `num_shards` for each split. If `max_shard_size` is provided and a split's `num_shards` is not, then the number of shards for that split is estimated from `max_shard_size`.

embed_external_files (`bool`, defaults to `True`) : Whether to embed file bytes in the shards. In particular, this will do the following before the push for the fields of type:  - [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio) and [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image) removes local path information and embed file content in the Parquet files.

num_proc (`int`, *optional*, defaults to `None`) : Number of processes when preparing and uploading the dataset. This is helpful if the dataset is made of many samples or media files to embed. I uses "spawn" context to work with hf_xet, the rust client for fast uploads to HF. Multiprocessing is disabled by default.  

**Returns:**

huggingface_hub.CommitInfo

## Features[[datasets.Features]]

#### datasets.Features[[datasets.Features]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L1837)

A special dictionary that defines the internal structure of a dataset.

Instantiated with a dictionary of type `dict[str, FieldType]`, where keys are the desired column names,
and values are the type of that column.

`FieldType` can be one of the following:
- [Value](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Value) feature specifies a single data type value, e.g. `int64` or `string`.
- [ClassLabel](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.ClassLabel) feature specifies a predefined set of classes which can have labels associated to them and
  will be stored as integers in the dataset.
- Python `dict` specifies a composite feature containing a mapping of sub-fields to sub-features.
  It's possible to have nested fields of nested fields in an arbitrary manner.
- [List](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.List) or [LargeList](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.LargeList) specifies a composite feature containing a sequence of
  sub-features, all of the same feature type.
- [Array2D](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Array2D), [Array3D](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Array3D), [Array4D](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Array4D) or [Array5D](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Array5D) feature for multidimensional arrays.
- [Audio](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Audio) feature to store the absolute path to an audio file or a dictionary with the relative path
  to an audio file ("path" key) and its bytes content ("bytes" key).
  This feature loads the audio lazily with a decoder.
- [Image](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Image) feature to store the absolute path to an image file, an `np.ndarray` object, a `PIL.Image.Image` object
  or a dictionary with the relative path to an image file ("path" key) and its bytes content ("bytes" key).
  This feature extracts the image data.
- [Video](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Video) feature to store the absolute path to a video file, a `torchcodec.decoders.VideoDecoder` object
  or a dictionary with the relative path to a video file ("path" key) and its bytes content ("bytes" key).
  This feature loads the video lazily with a decoder.
- [Pdf](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Pdf) feature to store the absolute path to a PDF file, a `pdfplumber.pdf.PDF` object
  or a dictionary with the relative path to a PDF file ("path" key) and its bytes content ("bytes" key).
  This feature loads the PDF lazily with a PDF reader.
- [Nifti](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Nifti) feature to store the absolute path to a NIfTI neuroimaging file, a `nibabel.Nifti1Image` object
  or a dictionary with the relative path to a NIfTI file ("path" key) and its bytes content ("bytes" key).
  This feature loads the NIfTI file lazily with nibabel.
- [Translation](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Translation) or [TranslationVariableLanguages](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.TranslationVariableLanguages) feature specific to Machine Translation.
- [Json](/docs/datasets/v4.8.4/en/package_reference/loading_methods#datasets.Json) feature to store unstructred data, e.g. containing mixed/abritrary types. Under the hood

copydatasets.Features.copyhttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L2264[][Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features)

Make a deep copy of [Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features).

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train")
>>> copy_of_features = ds.features.copy()
>>> copy_of_features
{'label': ClassLabel(names=['neg', 'pos']),
 'text': Value('string')}
```

**Returns:**

[Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features)
#### decode_batch[[datasets.Features.decode_batch]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L2237)

Decode batch with custom feature decoding.

**Parameters:**

batch (`dict[str, list[Any]]`) : Dataset batch data.

token_per_repo_id (`dict`, *optional*) : To access and decode audio or image files from private repositories on the Hub, you can pass a dictionary repo_id (str) -> token (bool or str)

**Returns:**

`dict[str, list[Any]]`
#### decode_column[[datasets.Features.decode_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L2212)

Decode column with custom feature decoding.

**Parameters:**

column (`list[Any]`) : Dataset column data.

column_name (`str`) : Dataset column name.

**Returns:**

`list[Any]`
#### decode_example[[datasets.Features.decode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L2189)

Decode example with custom feature decoding.

**Parameters:**

example (`dict[str, Any]`) : Dataset row data.

token_per_repo_id (`dict`, *optional*) : To access and decode audio or image files from private repositories on the Hub, you can pass a dictionary `repo_id (str) -> token (bool or str)`.

**Returns:**

`dict[str, Any]`
#### encode_batch[[datasets.Features.encode_batch]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L2170)

Encode batch into a format for Arrow.

**Parameters:**

batch (`dict[str, list[Any]]`) : Data in a Dataset batch.

**Returns:**

`dict[str, list[Any]]`
#### encode_column[[datasets.Features.encode_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L2154)

Encode column into a format for Arrow.

**Parameters:**

column (`list[Any]`) : Data in a Dataset column.

column_name (`str`) : Dataset column name.

**Returns:**

`list[Any]`
#### encode_example[[datasets.Features.encode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L2140)

Encode example into a format for Arrow.

**Parameters:**

example (`dict[str, Any]`) : Data in a Dataset row.

**Returns:**

`dict[str, Any]`
#### flatten[[datasets.Features.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L2337)

Flatten the features. Every dictionary column is removed and is replaced by
all the subfields it contains. The new fields are named by concatenating the
name of the original column and the subfield name like this: `.`.

If a column contains nested dictionaries, then all the lower-level subfields names are
also concatenated to form new columns: `..`, etc.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("rajpurkar/squad", split="train")
>>> ds.features.flatten()
{'answers.answer_start': List(Value('int32'), id=None),
 'answers.text': List(Value('string'), id=None),
 'context': Value('string'),
 'id': Value('string'),
 'question': Value('string'),
 'title': Value('string')}
```

**Returns:**

`[Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features)`

The flattened features.
#### from_arrow_schema[[datasets.Features.from_arrow_schema]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L1923)

Construct [Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features) from Arrow Schema.
It also checks the schema metadata for Hugging Face Datasets features.
Non-nullable fields are not supported and set to nullable.

Also, pa.dictionary is not supported and it uses its underlying type instead.
Therefore datasets convert DictionaryArray objects to their actual values.

**Parameters:**

pa_schema (`pyarrow.Schema`) : Arrow Schema.

**Returns:**

[Features](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Features)
#### from_dict[[datasets.Features.from_dict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L1957)

Construct [*Features*] from dict.

Regenerate the nested feature object from a deserialized dict.
We use the *_type* key to infer the dataclass name of the feature *FieldType*.

It allows for a convenient constructor syntax
to define features from deserialized JSON dictionaries. This function is used in particular when deserializing
a [*DatasetInfo*] that was dumped to a JSON object. This acts as an analogue to
[*Features.from_arrow_schema*] and handles the recursive field-by-field instantiation, but doesn't require
any mapping to/from pyarrow, except for the fact that it takes advantage of the mapping of pyarrow primitive
dtypes that [*Value*] automatically performs.

Example:

```python
>>> Features.from_dict(&amp;lcub;'_type': &amp;lcub;'dtype': 'string', 'id': None, '_type': 'Value'}})
&amp;lcub;'_type': Value('string')}
```

**Parameters:**

dic (*dict[str, Any]*) : Python dictionary.

**Returns:**

`*Features*`
#### reorder_fields_as[[datasets.Features.reorder_fields_as]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L2284)

Reorder Features fields to match the field order of other [*Features*].

The order of the fields is important since it matters for the underlying arrow data.
Re-ordering the fields allows to make the underlying arrow data type match.

Example:

```python
>>> from datasets import Features, List, Value
>>> # let's say we have two features with a different order of nested fields (for a and b for example)
>>> f1 = Features(&amp;lcub;"root": &amp;lcub;"a": Value("string"), "b": Value("string")}})
>>> f2 = Features(&amp;lcub;"root": &amp;lcub;"b": Value("string"), "a": Value("string")}})
>>> assert f1.type != f2.type
>>> # re-ordering keeps the base structure (here List is defined at the root level), but makes the fields order match
>>> f1.reorder_fields_as(f2)
&amp;lcub;'root': List(&amp;lcub;'b': Value('string'), 'a': Value('string')})}
>>> assert f1.reorder_fields_as(f2).type == f2.type
```

**Parameters:**

other ([*Features*]) : The other [*Features*] to align with.

**Returns:**

[*Features*]

### Scalar[[datasets.Value]]

#### datasets.Value[[datasets.Value]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L493)

Scalar feature value of a particular data type.

The possible dtypes of `Value` are as follows:
- `null`
- `bool`
- `int8`
- `int16`
- `int32`
- `int64`
- `uint8`
- `uint16`
- `uint32`
- `uint64`
- `float16`
- `float32` (alias float)
- `float64` (alias double)
- `time32[(s|ms)]`
- `time64[(us|ns)]`
- `timestamp[(s|ms|us|ns)]`
- `timestamp[(s|ms|us|ns), tz=(tzstring)]`
- `date32`
- `date64`
- `duration[(s|ms|us|ns)]`
- `decimal128(precision, scale)`
- `decimal256(precision, scale)`
- `binary`
- `large_binary`
- `binary_view`
- `string`
- `large_string`
- `string_view`

Example:

```py
>>> from datasets import Features
>>> features = Features({'stars': Value('int32')})
>>> features
{'stars': Value('int32')}
```

**Parameters:**

dtype (`str`) : Name of the data type.

#### datasets.ClassLabel[[datasets.ClassLabel]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L985)

Feature type for integer class labels.

There are 3 ways to define a `ClassLabel`, which correspond to the 3 arguments:

* `num_classes`: Create 0 to (num_classes-1) labels.
* `names`: List of label strings.
* `names_file`: File containing the list of labels.

Under the hood the labels are stored as integers.
You can use negative integers to represent unknown/missing labels.

Example:

```py
>>> from datasets import Features, ClassLabel
>>> features = Features({'label': ClassLabel(num_classes=3, names=['bad', 'ok', 'good'])})
>>> features
{'label': ClassLabel(names=['bad', 'ok', 'good'])}
```

cast_storagedatasets.ClassLabel.cast_storagehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L1150[{"name": "storage", "val": ": typing.Union[pyarrow.lib.StringArray, pyarrow.lib.IntegerArray]"}]- **storage** (`Union[pa.StringArray, pa.IntegerArray]`) --
  PyArrow array to cast.0`pa.Int64Array`Array in the `ClassLabel` arrow storage type.
Cast an Arrow array to the `ClassLabel` arrow storage type.
The Arrow types that can be converted to the `ClassLabel` pyarrow storage type are:

- `pa.string()`
- `pa.int()`

**Parameters:**

num_classes (`int`, *optional*) : Number of classes. All labels must be  class name `string`.

Regarding unknown/missing labels: passing negative integers raises `ValueError`.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train")
>>> ds.features["label"].int2str(0)
'neg'
```
#### str2int[[datasets.ClassLabel.str2int]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L1059)

Conversion class name `string` => `integer`.

Example:

```py
>>> from datasets import load_dataset
>>> ds = load_dataset("cornell-movie-review-data/rotten_tomatoes", split="train")
>>> ds.features["label"].str2int('neg')
0
```

### Composite[[datasets.LargeList]]

#### datasets.LargeList[[datasets.LargeList]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L1328)

Feature type for large list data composed of child feature data type.

It is backed by `pyarrow.LargeListType`, which is like `pyarrow.ListType` but with 64-bit rather than 32-bit offsets.

**Parameters:**

feature (`FeatureType`) : Child feature data type of each item within the large list.

#### datasets.List[[datasets.List]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L1300)

Feature type for large list data composed of child feature data type.

It is backed by `pyarrow.ListType`, which uses 32-bit offsets or a fixed length.

**Parameters:**

feature (`FeatureType`) : Child feature data type of each item within the large list.

length (optional `int`, default to -1) : Length of the list if it is fixed. Defaults to -1 which means an arbitrary length.

#### datasets.Sequence[[datasets.Sequence]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L1266)

A `Sequence` is a utility that automatically converts internal dictionary feature into a dictionary of
lists. This behavior is implemented to have a compatibility layer with the TensorFlow Datasets library but may be
un-wanted in some cases. If you don't want this behavior, you can use a [List](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.List) or a [LargeList](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.LargeList)
instead of the [Sequence](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.Sequence).

**Parameters:**

feature (`FeatureType`) : Child feature data type of each item within the large list.

length (optional `int`, default to -1) : Length of the list if it is fixed. Defaults to -1 which means an arbitrary length.

**Returns:**

[List](/docs/datasets/v4.8.4/en/package_reference/main_classes#datasets.List) of the specified feature, except `dict` of sub-features
which are converted to `dict` of lists of sub-features for compatibility with TFDS.

### Translation[[datasets.Translation]]

#### datasets.Translation[[datasets.Translation]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/translation.py#L12)

`Feature` for translations with fixed languages per example.
Here for compatibility with tfds.

Example:

```python
>>> # At construction time:
>>> datasets.features.Translation(languages=['en', 'fr', 'de'])
>>> # During data generation:
>>> yield {
...         'en': 'the cat',
...         'fr': 'le chat',
...         'de': 'die katze'
... }
```

flattendatasets.Translation.flattenhttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/translation.py#L44[]
Flatten the Translation feature into a dictionary.

**Parameters:**

languages (`dict`) : A dictionary for each example mapping string language codes to string translations.

#### datasets.TranslationVariableLanguages[[datasets.TranslationVariableLanguages]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/translation.py#L52)

`Feature` for translations with variable languages per example.
Here for compatibility with tfds.

Example:

```python
>>> # At construction time:
>>> datasets.features.TranslationVariableLanguages(languages=['en', 'fr', 'de'])
>>> # During data generation:
>>> yield {
...         'en': 'the cat',
...         'fr': ['le chat', 'la chatte,']
...         'de': 'die katze'
... }
>>> # Tensor returned :
>>> {
...         'language': ['en', 'de', 'fr', 'fr'],
...         'translation': ['the cat', 'die katze', 'la chatte', 'le chat'],
... }
```

flattendatasets.TranslationVariableLanguages.flattenhttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/translation.py#L122[]
Flatten the TranslationVariableLanguages feature into a dictionary.

**Parameters:**

languages (`dict`) : A dictionary for each example mapping string language codes to one or more string translations. The languages present may vary from example to example.

**Returns:**

`- `language` or `translation` (variable-length 1D `tf.Tensor` of `tf.string`)`

Language codes sorted in ascending order or plain text translations, sorted to align with language codes.

### Arrays[[datasets.Array2D]]

#### datasets.Array2D[[datasets.Array2D]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L590)

Create a two-dimensional array.

Example:

```py
>>> from datasets import Features
>>> features = Features({'x': Array2D(shape=(1, 3), dtype='int32')})
```

**Parameters:**

shape (`tuple`) : Size of each dimension.

dtype (`str`) : Name of the data type.

#### datasets.Array3D[[datasets.Array3D]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L615)

Create a three-dimensional array.

Example:

```py
>>> from datasets import Features
>>> features = Features({'x': Array3D(shape=(1, 2, 3), dtype='int32')})
```

**Parameters:**

shape (`tuple`) : Size of each dimension.

dtype (`str`) : Name of the data type.

#### datasets.Array4D[[datasets.Array4D]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L640)

Create a four-dimensional array.

Example:

```py
>>> from datasets import Features
>>> features = Features({'x': Array4D(shape=(1, 2, 2, 3), dtype='int32')})
```

**Parameters:**

shape (`tuple`) : Size of each dimension.

dtype (`str`) : Name of the data type.

#### datasets.Array5D[[datasets.Array5D]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L665)

Create a five-dimensional array.

Example:

```py
>>> from datasets import Features
>>> features = Features({'x': Array5D(shape=(1, 2, 2, 3, 3), dtype='int32')})
```

**Parameters:**

shape (`tuple`) : Size of each dimension.

dtype (`str`) : Name of the data type.

### Audio[[datasets.Audio]]

#### datasets.Audio[[datasets.Audio]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/audio.py#L24)

Audio `Feature` to extract audio data from an audio file.

Input: The Audio feature accepts as input:
- A `str`: Absolute path to the audio file (i.e. random access is allowed).
- A `pathlib.Path`: path to the audio file (i.e. random access is allowed).
- A `dict` with the keys:

  - `path`: String with relative path of the audio file to the archive file.
  - `bytes`: Bytes content of the audio file.

  This is useful for parquet or webdataset files which embed audio files.

- A `dict` with the keys:

  - `array`: Array containing the audio sample
  - `sampling_rate`: Integer corresponding to the sampling rate of the audio sample.

- A `torchcodec.decoders.AudioDecoder`: torchcodec audio decoder object.

Output: The Audio features output data as `torchcodec.decoders.AudioDecoder` objects, with additional keys:

- `array`: Array containing the audio sample
- `sampling_rate`: Integer corresponding to the sampling rate of the audio sample.

Example:

```py
>>> from datasets import load_dataset, Audio
>>> ds = load_dataset("PolyAI/minds14", name="en-US", split="train")
>>> ds = ds.cast_column("audio", Audio(sampling_rate=44100, num_channels=2))
>>> ds[0]["audio"]

>>> audio = ds[0]["audio"]
>>> audio.get_samples_played_in_range(0, 10)
AudioSamples:
    data (shape): torch.Size([2, 110592])
    pts_seconds: 0.0
    duration_seconds: 2.507755102040816
    sample_rate: 44100
```

cast_storagedatasets.Audio.cast_storagehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/audio.py#L234[{"name": "storage", "val": ": typing.Union[pyarrow.lib.StringArray, pyarrow.lib.StructArray]"}]- **storage** (`Union[pa.StringArray, pa.StructArray]`) --
  PyArrow array to cast.0`pa.StructArray`Array in the Audio arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`
Cast an Arrow array to the Audio arrow storage type.
The Arrow types that can be converted to the Audio pyarrow storage type are:

- `pa.string()` - it must contain the "path" data
- `pa.binary()` - it must contain the audio bytes
- `pa.struct({"bytes": pa.binary()})`
- `pa.struct({"path": pa.string()})`
- `pa.struct({"bytes": pa.binary(), "path": pa.string()})`  - order doesn't matter

**Parameters:**

sampling_rate (`int`, *optional*) : Target sampling rate. If `None`, the native sampling rate is used.

num_channels (`int`, *optional*) : The desired number of channels of the samples. By default, the number of channels of the source is used. Audio decoding will return samples with shape (num_channels, num_samples) Currently `None` (number of channels of the source, default), `1` (mono) or `2` (stereo) channels are supported. The `num_channels` argument is passed to `torchcodec.decoders.AudioDecoder`.  

decode (`bool`, defaults to `True`) : Whether to decode the audio data. If `False`, returns the underlying dictionary in the format `{"path": audio_path, "bytes": audio_bytes}`.

stream_index (`int`, *optional*) : The streaming index to use from the file. If `None` defaults to the "best" index.

**Returns:**

``pa.StructArray``

Array in the Audio arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`
#### decode_example[[datasets.Audio.decode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/audio.py#L164)

Decode example audio file into audio data.

**Parameters:**

value (`dict`) : A dictionary with keys:  - `path`: String with relative audio file path. - `bytes`: Bytes of the audio file.

token_per_repo_id (`dict`, *optional*) : To access and decode audio files from private repositories on the Hub, you can pass a dictionary repo_id (`str`) -> token (`bool` or `str`)

**Returns:**

`torchcodec.decoders.AudioDecoder`
#### embed_storage[[datasets.Audio.embed_storage]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/audio.py#L280)

Embed audio files into the Arrow array.

**Parameters:**

storage (`pa.StructArray`) : PyArrow array to embed.

**Returns:**

``pa.StructArray``

Array in the Audio arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
#### encode_example[[datasets.Audio.encode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/audio.py#L96)

Encode example into a format for Arrow.

**Parameters:**

value (`str`, `bytes`,`bytearray`,`dict`, `AudioDecoder`) : Data passed as input to Audio feature.

**Returns:**

``dict``
#### flatten[[datasets.Audio.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/audio.py#L223)

If in the decodable state, raise an error, otherwise flatten the feature into a dictionary.

### Image[[datasets.Image]]

#### datasets.Image[[datasets.Image]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/image.py#L47)

Image `Feature` to read image data from an image file.

Input: The Image feature accepts as input:
- A `str`: Absolute path to the image file (i.e. random access is allowed).
- A `pathlib.Path`: path to the image file (i.e. random access is allowed).
- A `dict` with the keys:

  - `path`: String with relative path of the image file to the archive file.
  - `bytes`: Bytes of the image file.

  This is useful for parquet or webdataset files which embed image files.

- An `np.ndarray`: NumPy array representing an image.
- A `PIL.Image.Image`: PIL image object.

Output: The Image features output data as `PIL.Image.Image` objects.

Examples:

```py
>>> from datasets import load_dataset, Image
>>> ds = load_dataset("AI-Lab-Makerere/beans", split="train")
>>> ds.features["image"]
Image(decode=True, id=None)
>>> ds[0]["image"]

>>> ds = ds.cast_column('image', Image(decode=False))
{'bytes': None,
 'path': '/root/.cache/huggingface/datasets/downloads/extracted/b0a21163f78769a2cf11f58dfc767fb458fc7cea5c05dccc0144a2c0f0bc1292/train/healthy/healthy_train.85.jpg'}
```

cast_storagedatasets.Image.cast_storagehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/image.py#L213[{"name": "storage", "val": ": typing.Union[pyarrow.lib.StringArray, pyarrow.lib.StructArray, pyarrow.lib.ListArray]"}]- **storage** (`Union[pa.StringArray, pa.StructArray, pa.ListArray]`) --
  PyArrow array to cast.0`pa.StructArray`Array in the Image arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
Cast an Arrow array to the Image arrow storage type.
The Arrow types that can be converted to the Image pyarrow storage type are:

- `pa.string()` - it must contain the "path" data
- `pa.large_string()` - it must contain the "path" data (will be cast to string if possible)
- `pa.binary()` - it must contain the image bytes
- `pa.struct({"bytes": pa.binary()})`
- `pa.struct({"path": pa.string()})`
- `pa.struct({"bytes": pa.binary(), "path": pa.string()})`  - order doesn't matter
- `pa.list(*)` - it must contain the image array data

**Parameters:**

mode (`str`, *optional*) : The mode to convert the image to. If `None`, the native mode of the image is used.

decode (`bool`, defaults to `True`) : Whether to decode the image data. If `False`, returns the underlying dictionary in the format `{"path": image_path, "bytes": image_bytes}`.

**Returns:**

``pa.StructArray``

Array in the Image arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
#### decode_example[[datasets.Image.decode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/image.py#L139)

Decode example image file into image data.

**Parameters:**

value (`str` or `dict`) : A string with the absolute image file path, a dictionary with keys:  - `path`: String with absolute or relative image file path. - `bytes`: The bytes of the image file.

token_per_repo_id (`dict`, *optional*) : To access and decode image files from private repositories on the Hub, you can pass a dictionary repo_id (`str`) -> token (`bool` or `str`).

**Returns:**

`PIL.Image.Image`
#### embed_storage[[datasets.Image.embed_storage]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/image.py#L275)

Embed image files into the Arrow array.

**Parameters:**

storage (`pa.StructArray`) : PyArrow array to embed.

**Returns:**

``pa.StructArray``

Array in the Image arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
#### encode_example[[datasets.Image.encode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/image.py#L98)

Encode example into a format for Arrow.

**Parameters:**

value (`str`, `np.ndarray`, `PIL.Image.Image` or `dict`) : Data passed as input to Image feature.

**Returns:**

`dict` with "path" and "bytes" fields
#### flatten[[datasets.Image.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/image.py#L200)

If in the decodable state, return the feature itself, otherwise flatten the feature into a dictionary.

### Video[[datasets.Video]]

#### datasets.Video[[datasets.Video]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/video.py#L29)

Video `Feature` to read video data from a video file.

Input: The Video feature accepts as input:
- A `str`: Absolute path to the video file (i.e. random access is allowed).
- A `pathlib.Path`: path to the video file (i.e. random access is allowed).
- A `dict` with the keys:

  - `path`: String with relative path of the video file in a dataset repository.
  - `bytes`: Bytes of the video file.

  This is useful for parquet or webdataset files which embed video files.

- A `torchcodec.decoders.VideoDecoder`: torchcodec video decoder object.

Output: The Video features output data as `torchcodec.decoders.VideoDecoder` objects.

Examples:

```py
>>> from datasets import Dataset, Video
>>> ds = Dataset.from_dict({"video":["path/to/Screen Recording.mov"]}).cast_column("video", Video())
>>> ds.features["video"]
Video(decode=True, id=None)
>>> ds[0]["video"]

>>> video = ds[0]["video"]
>>> video.get_frames_in_range(0, 10)
FrameBatch:
data (shape): torch.Size([10, 3, 50, 66])
pts_seconds: tensor([0.4333, 0.4333, 0.4333, 0.4333, 0.4333, 0.4333, 0.4333, 0.4333, 0.4333,
        0.4333], dtype=torch.float64)
duration_seconds: tensor([0.0167, 0.0167, 0.0167, 0.0167, 0.0167, 0.0167, 0.0167, 0.0167, 0.0167,
        0.0167], dtype=torch.float64)
>>> ds.cast_column('video', Video(decode=False))[0]["video]
{'bytes': None,
 'path': 'path/to/Screen Recording.mov'}
```

cast_storagedatasets.Video.cast_storagehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/video.py#L239[{"name": "storage", "val": ": typing.Union[pyarrow.lib.StringArray, pyarrow.lib.StructArray, pyarrow.lib.ListArray]"}]- **storage** (`Union[pa.StringArray, pa.StructArray, pa.ListArray]`) --
  PyArrow array to cast.0`pa.StructArray`Array in the Video arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
Cast an Arrow array to the Video arrow storage type.
The Arrow types that can be converted to the Video pyarrow storage type are:

- `pa.string()` - it must contain the "path" data
- `pa.binary()` - it must contain the video bytes
- `pa.struct({"bytes": pa.binary()})`
- `pa.struct({"path": pa.string()})`
- `pa.struct({"bytes": pa.binary(), "path": pa.string()})`  - order doesn't matter
- `pa.list(*)` - it must contain the video array data

**Parameters:**

decode (`bool`, defaults to `True`) : Whether to decode the video data. If `False`, returns the underlying dictionary in the format `{"path": video_path, "bytes": video_bytes}`.

stream_index (`int`, *optional*) : The streaming index to use from the file. If `None` defaults to the "best" index.

dimension_order (`str`, defaults to `NCHW`) : The dimension order of the decoded frames. where N is the batch size, C is the number of channels, H is the height, and W is the width of the frames.

num_ffmpeg_threads (`int`, defaults to `1`) : The number of threads to use for decoding the video. (Recommended to keep this at 1)

device (`str` or `torch.device`, defaults to `cpu`) : The device to use for decoding the video.

seek_mode (`str`, defaults to `exact`) : Determines if frame access will be “exact” or “approximate”. Exact guarantees that requesting frame i will always return frame i, but doing so requires an initial scan of the file. Approximate is faster as it avoids scanning the file, but less accurate as it uses the file's metadata to calculate where i probably is. read more [here](https://docs.pytorch.org/torchcodec/stable/generated_examples/approximate_mode.html#sphx-glr-generated-examples-approximate-mode-py)

**Returns:**

``pa.StructArray``

Array in the Video arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
#### decode_example[[datasets.Video.decode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/video.py#L153)

Decode example video file into video data.

**Parameters:**

value (`str` or `dict`) : A string with the absolute video file path, a dictionary with keys:  - `path`: String with absolute or relative video file path. - `bytes`: The bytes of the video file.

token_per_repo_id (`dict`, *optional*) : To access and decode video files from private repositories on the Hub, you can pass a dictionary repo_id (`str`) -> token (`bool` or `str`).

**Returns:**

`torchcodec.decoders.VideoDecoder`
#### embed_storage[[datasets.Video.embed_storage]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/video.py#L291)

Embed image files into the Arrow array.

**Parameters:**

storage (`pa.StructArray`) : PyArrow array to embed.

**Returns:**

``pa.StructArray``

Array in the Video arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
#### encode_example[[datasets.Video.encode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/video.py#L105)

Encode example into a format for Arrow.

**Parameters:**

value (`str`, `np.ndarray`, `bytes`, `bytearray`, `VideoDecoder` or `dict`) : Data passed as input to Video feature.

**Returns:**

`dict` with "path" and "bytes" fields
#### flatten[[datasets.Video.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/video.py#L226)

If in the decodable state, return the feature itself, otherwise flatten the feature into a dictionary.

### Json[[datasets.Json]]

#### datasets.Json[[datasets.Json]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L1183)

Feature type for JSON objects.

Under the hood the objects are stored as JSON-encoded strings.

Example:

```py
>>> from datasets import Features, Json
>>> features = Features({'json': Json()})
>>> features
{'json': Json()}
```

```py
>>> from datasets import Dataset, Features, Json, List
>>> features = Features({"a": List(Json())})
>>> ds = Dataset.from_dict({"a": [[{"b": 0}, {"c": 0}]]}, features=features)
>>> # OR
>>> ds = Dataset.from_dict({"a": [[{"b": 0}, {"c": 0}]]}, on_mixed_types="use_json")
>>> ds.features
{'a': List(Json())}
>>> ds[0]
{'a': [{'b': 0}, {'c': 0}]}
>>> def f(x):
...     for y in x["a"]:
...         y["d"] = "foo"
...     return x
>>> ds = ds.map(f)
>>> ds.features
>>> ds[0]
{'a': [{'b': 0, 'd': 'foo'}, {'c': 0, 'd': 'foo'}]}
```

cast_storagedatasets.Json.cast_storagehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/features.py#L1242[{"name": "storage", "val": ": Array"}]- **storage** (`Union[pa.StringArray, pa.IntegerArray]`) --
  PyArrow array to cast.0`pa.JsonArray`Array in the `Json` arrow storage type.
Cast an Arrow array to the `Json` arrow storage type.

**Parameters:**

storage (`Union[pa.StringArray, pa.IntegerArray]`) : PyArrow array to cast.

**Returns:**

``pa.JsonArray``

Array in the `Json` arrow storage type.

### Pdf[[datasets.Pdf]]

#### datasets.Pdf[[datasets.Pdf]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/pdf.py#L31)

**Experimental.**
Pdf `Feature` to read pdf documents from a pdf file.

Input: The Pdf feature accepts as input:
- A `str`: Absolute path to the pdf file (i.e. random access is allowed).
- A `pathlib.Path`: path to the pdf file (i.e. random access is allowed).
- A `dict` with the keys:
  - `path`: String with relative path of the pdf file in a dataset repository.
  - `bytes`: Bytes of the pdf file.
  This is useful for archived files with sequential access.

- A `pdfplumber.pdf.PDF`: pdfplumber pdf object.

Examples:

```py
>>> from datasets import Dataset, Pdf
>>> ds = Dataset.from_dict({"pdf": ["path/to/pdf/file.pdf"]}).cast_column("pdf", Pdf())
>>> ds.features["pdf"]
Pdf(decode=True, id=None)
>>> ds[0]["pdf"]

>>> ds = ds.cast_column("pdf", Pdf(decode=False))
>>> ds[0]["pdf"]
{'bytes': None,
'path': 'path/to/pdf/file.pdf'}
```

cast_storagedatasets.Pdf.cast_storagehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/pdf.py#L184[{"name": "storage", "val": ": typing.Union[pyarrow.lib.StringArray, pyarrow.lib.StructArray, pyarrow.lib.ListArray]"}]- **storage** (`Union[pa.StringArray, pa.StructArray, pa.ListArray]`) --
  PyArrow array to cast.0`pa.StructArray`Array in the Pdf arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
Cast an Arrow array to the Pdf arrow storage type.
The Arrow types that can be converted to the Pdf pyarrow storage type are:

- `pa.string()` - it must contain the "path" data
- `pa.binary()` - it must contain the image bytes
- `pa.struct({"bytes": pa.binary()})`
- `pa.struct({"path": pa.string()})`
- `pa.struct({"bytes": pa.binary(), "path": pa.string()})`  - order doesn't matter
- `pa.list(*)` - it must contain the pdf array data

**Parameters:**

decode (`bool`, defaults to `True`) : Whether to decode the pdf data. If `False`, returns the underlying dictionary in the format `{"path": pdf_path, "bytes": pdf_bytes}`.

**Returns:**

``pa.StructArray``

Array in the Pdf arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
#### decode_example[[datasets.Pdf.decode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/pdf.py#L113)

Decode example pdf file into pdf data.

**Parameters:**

value (`str` or `dict`) : A string with the absolute pdf file path, a dictionary with keys:  - `path`: String with absolute or relative pdf file path. - `bytes`: The bytes of the pdf file. 

token_per_repo_id (`dict`, *optional*) : To access and decode pdf files from private repositories on the Hub, you can pass a dictionary repo_id (`str`) -> token (`bool` or `str`).

**Returns:**

`pdfplumber.pdf.PDF`
#### embed_storage[[datasets.Pdf.embed_storage]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/pdf.py#L221)

Embed PDF files into the Arrow array.

**Parameters:**

storage (`pa.StructArray`) : PyArrow array to embed.

**Returns:**

``pa.StructArray``

Array in the PDF arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
#### encode_example[[datasets.Pdf.encode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/pdf.py#L78)

Encode example into a format for Arrow.

**Parameters:**

value (`str`, `bytes`, `pdfplumber.pdf.PDF` or `dict`) : Data passed as input to Pdf feature.

**Returns:**

`dict` with "path" and "bytes" fields
#### flatten[[datasets.Pdf.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/pdf.py#L171)

If in the decodable state, return the feature itself, otherwise flatten the feature into a dictionary.

### Nifti[[datasets.Nifti]]

#### datasets.Nifti[[datasets.Nifti]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/nifti.py#L64)

**Experimental.**
Nifti `Feature` to read NIfTI neuroimaging files.

Input: The Nifti feature accepts as input:
- A `str`: Absolute path to the NIfTI file (i.e. random access is allowed).
- A `pathlib.Path`: path to the NIfTI file (i.e. random access is allowed).
- A `dict` with the keys:
  - `path`: String with relative path of the NIfTI file in a dataset repository.
  - `bytes`: Bytes of the NIfTI file.
  This is useful for archived files with sequential access.

- A `nibabel` image object (e.g., `nibabel.nifti1.Nifti1Image`).

Examples:

```py
>>> from datasets import Dataset, Nifti
>>> ds = Dataset.from_dict({"nifti": ["path/to/file.nii.gz"]}).cast_column("nifti", Nifti())
>>> ds.features["nifti"]
Nifti(decode=True, id=None)
>>> ds[0]["nifti"]

>>> ds = ds.cast_column("nifti", Nifti(decode=False))
>>> ds[0]["nifti"]
{'bytes': None,
'path': 'path/to/file.nii.gz'}
```

cast_storagedatasets.Nifti.cast_storagehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/nifti.py#L266[{"name": "storage", "val": ": typing.Union[pyarrow.lib.StringArray, pyarrow.lib.StructArray, pyarrow.lib.BinaryArray]"}]- **storage** (`Union[pa.StringArray, pa.StructArray, pa.BinaryArray]`) --
  PyArrow array to cast.0`pa.StructArray`Array in the Nifti arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
Cast an Arrow array to the Nifti arrow storage type.
The Arrow types that can be converted to the Nifti pyarrow storage type are:

- `pa.string()` - it must contain the "path" data
- `pa.binary()` - it must contain the NIfTI bytes
- `pa.struct({"bytes": pa.binary()})`
- `pa.struct({"path": pa.string()})`
- `pa.struct({"bytes": pa.binary(), "path": pa.string()})`  - order doesn't matter

**Parameters:**

decode (`bool`, defaults to `True`) : Whether to decode the NIfTI data. If `False` a string with the bytes is returned. `decode=False` is not supported when decoding examples.

**Returns:**

``pa.StructArray``

Array in the Nifti arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
#### decode_example[[datasets.Nifti.decode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/nifti.py#L150)

Decode example NIfTI file into nibabel image object.

**Parameters:**

value (`str` or `dict`) : A string with the absolute NIfTI file path, a dictionary with keys:  - `path`: String with absolute or relative NIfTI file path. - `bytes`: The bytes of the NIfTI file. 

token_per_repo_id (`dict`, *optional*) : To access and decode NIfTI files from private repositories on the Hub, you can pass a dictionary repo_id (`str`) -> token (`bool` or `str`).

**Returns:**

`nibabel.Nifti1Image` objects
#### embed_storage[[datasets.Nifti.embed_storage]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/nifti.py#L213)

Embed NifTI files into the Arrow array.

**Parameters:**

storage (`pa.StructArray`) : PyArrow array to embed.

**Returns:**

``pa.StructArray``

Array in the NifTI arrow storage type, that is
`pa.struct({"bytes": pa.binary(), "path": pa.string()})`.
#### encode_example[[datasets.Nifti.encode_example]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/nifti.py#L110)

Encode example into a format for Arrow.

**Parameters:**

value (`str`, `bytes`, `nibabel.Nifti1Image` or `dict`) : Data passed as input to Nifti feature.

**Returns:**

`dict` with "path" and "bytes" fields
#### flatten[[datasets.Nifti.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/features/nifti.py#L253)

If in the decodable state, return the feature itself, otherwise flatten the feature into a dictionary.

## Filesystems[[datasets.filesystems.is_remote_filesystem]]

#### datasets.filesystems.is_remote_filesystem[[datasets.filesystems.is_remote_filesystem]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/filesystems/__init__.py#L28)

Checks if `fs` is a remote filesystem.

**Parameters:**

fs (`fsspec.spec.AbstractFileSystem`) : An abstract super-class for pythonic file-systems, e.g. `fsspec.filesystem('file')` or `s3fs.S3FileSystem`.

## Fingerprint[[datasets.fingerprint.Hasher]]

#### datasets.fingerprint.Hasher[[datasets.fingerprint.Hasher]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/fingerprint.py#L196)

Hasher that accepts python objects as inputs.
