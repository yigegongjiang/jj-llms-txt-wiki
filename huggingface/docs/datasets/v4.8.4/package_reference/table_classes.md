# Table Classes

Each `Dataset` object is backed by a PyArrow Table.
A Table can be loaded from either the disk (memory mapped) or in memory.
Several Table types are available, and they all inherit from [table.Table](/docs/datasets/v4.8.4/en/package_reference/table_classes#datasets.table.Table).

## Table[[datasets.table.Table]]

#### datasets.table.Table[[datasets.table.Table]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L153)

Wraps a pyarrow Table by using composition.
This is the base class for `InMemoryTable`, `MemoryMappedTable` and `ConcatenationTable`.

It implements all the basic attributes/methods of the pyarrow Table class except
the Table transforms: `slice, filter, flatten, combine_chunks, cast, add_column,
append_column, remove_column, set_column, rename_columns` and `drop`.

The implementation of these methods differs for the subclasses.

validatedatasets.table.Table.validatehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L178[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]- **full** (`bool`, defaults to `False`) --
  If `True`, run expensive checks, otherwise cheap checks only.0- ``pa.lib.ArrowInvalid`` -- if validation fails``pa.lib.ArrowInvalid``

Perform validation checks.  An exception is raised if validation fails.

By default only cheap validation checks are run.  Pass `full=True`
for thorough validation checks (potentially `O(n)`).

**Parameters:**

full (`bool`, defaults to `False`) : If `True`, run expensive checks, otherwise cheap checks only.
#### equals[[datasets.table.Table.equals]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L194)

Check if contents of two tables are equal.

**Parameters:**

other ([Table](/docs/datasets/v4.8.4/en/package_reference/table_classes#datasets.table.Table)) : Table to compare against.

check_metadata `bool`, defaults to `False`) : Whether schema metadata equality should be checked as well.

**Returns:**

``bool``
#### to_batches[[datasets.table.Table.to_batches]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L211)

Convert Table to list of (contiguous) `RecordBatch` objects.

**Parameters:**

max_chunksize (`int`, defaults to `None`) : Maximum size for `RecordBatch` chunks. Individual chunks may be smaller depending on the chunk layout of individual columns.

**Returns:**

`List[pyarrow.RecordBatch]`
#### to_pydict[[datasets.table.Table.to_pydict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L225)

Convert the Table to a `dict` or `OrderedDict`.

**Returns:**

``dict``
#### to_pandas[[datasets.table.Table.to_pandas]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L243)

Convert to a pandas-compatible NumPy array or DataFrame, as appropriate.

**Parameters:**

memory_pool (`MemoryPool`, defaults to `None`) : Arrow MemoryPool to use for allocations. Uses the default memory pool is not passed.

strings_to_categorical (`bool`, defaults to `False`) : Encode string (UTF8) and binary types to `pandas.Categorical`.

categories (`list`, defaults to `empty`) : List of fields that should be returned as `pandas.Categorical`. Only applies to table-like data structures.

zero_copy_only (`bool`, defaults to `False`) : Raise an `ArrowException` if this function call would require copying the underlying data.

integer_object_nulls (`bool`, defaults to `False`) : Cast integers with nulls to objects.

date_as_object (`bool`, defaults to `True`) : Cast dates to objects. If `False`, convert to `datetime64[ns]` dtype.

timestamp_as_object (`bool`, defaults to `False`) : Cast non-nanosecond timestamps (`np.datetime64`) to objects. This is useful if you have timestamps that don't fit in the normal date range of nanosecond timestamps (1678 CE-2262 CE). If `False`, all timestamps are converted to `datetime64[ns]` dtype.

use_threads (`bool`, defaults to `True`) : Whether to parallelize the conversion using multiple threads.

deduplicate_objects (`bool`, defaults to `False`) : Do not create multiple copies Python objects when created, to save on memory use. Conversion will be slower.

ignore_metadata (`bool`, defaults to `False`) : If `True`, do not use the 'pandas' metadata to reconstruct the DataFrame index, if present.

safe (`bool`, defaults to `True`) : For certain data types, a cast is needed in order to store the data in a pandas DataFrame or Series (e.g. timestamps are always stored as nanoseconds in pandas). This option controls whether it is a safe cast or not.

split_blocks (`bool`, defaults to `False`) : If `True`, generate one internal "block" for each column when creating a pandas.DataFrame from a `RecordBatch` or `Table`. While this can temporarily reduce memory note that various pandas operations can trigger "consolidation" which may balloon memory use.

self_destruct (`bool`, defaults to `False`) : EXPERIMENTAL: If `True`, attempt to deallocate the originating Arrow memory while converting the Arrow object to pandas. If you use the object after calling `to_pandas` with this option it will crash your program.

types_mapper (`function`, defaults to `None`) : A function mapping a pyarrow DataType to a pandas `ExtensionDtype`. This can be used to override the default pandas type for conversion of built-in pyarrow types or in absence of `pandas_metadata` in the Table schema. The function receives a pyarrow DataType and is expected to return a pandas `ExtensionDtype` or `None` if the default conversion should be used for that type. If you have a dictionary mapping, you can pass `dict.get` as function.

**Returns:**

``pandas.Series` or `pandas.DataFrame``

`pandas.Series` or `pandas.DataFrame` depending on type of object
#### to_string[[datasets.table.Table.to_string]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L305)
#### field[[datasets.table.Table.field]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L324)

Select a schema field by its column name or numeric index.

**Parameters:**

i (`Union[int, str]`) : The index or name of the field to retrieve.

**Returns:**

`pyarrow.Field`
#### column[[datasets.table.Table.column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L337)

Select a column by its column name, or numeric index.

**Parameters:**

i (`Union[int, str]`) : The index or name of the column to retrieve.

**Returns:**

`pyarrow.ChunkedArray`
#### itercolumns[[datasets.table.Table.itercolumns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L350)

Iterator over all columns in their numerical order.
#### schema[[datasets.table.Table.schema]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L359)

Schema of the table and its columns.

**Returns:**

`pyarrow.Schema`
#### columns[[datasets.table.Table.columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L369)

List of all columns in numerical order.

**Returns:**

`List[pa.ChunkedArray]`
#### num_columns[[datasets.table.Table.num_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L379)

Number of columns in this table.

**Returns:**

int
#### num_rows[[datasets.table.Table.num_rows]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L389)

Number of rows in this table.

Due to the definition of a table, all columns have the same number of
rows.

**Returns:**

int
#### shape[[datasets.table.Table.shape]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L402)

Dimensions of the table: (#rows, #columns).

**Returns:**

``(int, int)``

Number of rows and number of columns.
#### nbytes[[datasets.table.Table.nbytes]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L412)

Total number of bytes consumed by the elements of the table.

## InMemoryTable[[datasets.table.InMemoryTable]]

#### datasets.table.InMemoryTable[[datasets.table.InMemoryTable]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L638)

The table is said in-memory when it is loaded into the user's RAM.

Pickling it does copy all the data using memory.
Its implementation is simple and uses the underlying pyarrow Table methods directly.

This is different from the `MemoryMapped` table, for which pickling doesn't copy all the
data in memory. For a `MemoryMapped`, unpickling instead reloads the table from the disk.

`InMemoryTable` must be used when data fit in memory, while `MemoryMapped` are reserved for
data bigger than memory or when you want the memory footprint of your application to
stay low.

validatedatasets.table.InMemoryTable.validatehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L178[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]- **full** (`bool`, defaults to `False`) --
  If `True`, run expensive checks, otherwise cheap checks only.0- ``pa.lib.ArrowInvalid`` -- if validation fails``pa.lib.ArrowInvalid``

Perform validation checks.  An exception is raised if validation fails.

By default only cheap validation checks are run.  Pass `full=True`
for thorough validation checks (potentially `O(n)`).

**Parameters:**

full (`bool`, defaults to `False`) : If `True`, run expensive checks, otherwise cheap checks only.
#### equals[[datasets.table.InMemoryTable.equals]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L194)

Check if contents of two tables are equal.

**Parameters:**

other ([Table](/docs/datasets/v4.8.4/en/package_reference/table_classes#datasets.table.Table)) : Table to compare against.

check_metadata `bool`, defaults to `False`) : Whether schema metadata equality should be checked as well.

**Returns:**

``bool``
#### to_batches[[datasets.table.InMemoryTable.to_batches]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L211)

Convert Table to list of (contiguous) `RecordBatch` objects.

**Parameters:**

max_chunksize (`int`, defaults to `None`) : Maximum size for `RecordBatch` chunks. Individual chunks may be smaller depending on the chunk layout of individual columns.

**Returns:**

`List[pyarrow.RecordBatch]`
#### to_pydict[[datasets.table.InMemoryTable.to_pydict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L225)

Convert the Table to a `dict` or `OrderedDict`.

**Returns:**

``dict``
#### to_pandas[[datasets.table.InMemoryTable.to_pandas]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L243)

Convert to a pandas-compatible NumPy array or DataFrame, as appropriate.

**Parameters:**

memory_pool (`MemoryPool`, defaults to `None`) : Arrow MemoryPool to use for allocations. Uses the default memory pool is not passed.

strings_to_categorical (`bool`, defaults to `False`) : Encode string (UTF8) and binary types to `pandas.Categorical`.

categories (`list`, defaults to `empty`) : List of fields that should be returned as `pandas.Categorical`. Only applies to table-like data structures.

zero_copy_only (`bool`, defaults to `False`) : Raise an `ArrowException` if this function call would require copying the underlying data.

integer_object_nulls (`bool`, defaults to `False`) : Cast integers with nulls to objects.

date_as_object (`bool`, defaults to `True`) : Cast dates to objects. If `False`, convert to `datetime64[ns]` dtype.

timestamp_as_object (`bool`, defaults to `False`) : Cast non-nanosecond timestamps (`np.datetime64`) to objects. This is useful if you have timestamps that don't fit in the normal date range of nanosecond timestamps (1678 CE-2262 CE). If `False`, all timestamps are converted to `datetime64[ns]` dtype.

use_threads (`bool`, defaults to `True`) : Whether to parallelize the conversion using multiple threads.

deduplicate_objects (`bool`, defaults to `False`) : Do not create multiple copies Python objects when created, to save on memory use. Conversion will be slower.

ignore_metadata (`bool`, defaults to `False`) : If `True`, do not use the 'pandas' metadata to reconstruct the DataFrame index, if present.

safe (`bool`, defaults to `True`) : For certain data types, a cast is needed in order to store the data in a pandas DataFrame or Series (e.g. timestamps are always stored as nanoseconds in pandas). This option controls whether it is a safe cast or not.

split_blocks (`bool`, defaults to `False`) : If `True`, generate one internal "block" for each column when creating a pandas.DataFrame from a `RecordBatch` or `Table`. While this can temporarily reduce memory note that various pandas operations can trigger "consolidation" which may balloon memory use.

self_destruct (`bool`, defaults to `False`) : EXPERIMENTAL: If `True`, attempt to deallocate the originating Arrow memory while converting the Arrow object to pandas. If you use the object after calling `to_pandas` with this option it will crash your program.

types_mapper (`function`, defaults to `None`) : A function mapping a pyarrow DataType to a pandas `ExtensionDtype`. This can be used to override the default pandas type for conversion of built-in pyarrow types or in absence of `pandas_metadata` in the Table schema. The function receives a pyarrow DataType and is expected to return a pandas `ExtensionDtype` or `None` if the default conversion should be used for that type. If you have a dictionary mapping, you can pass `dict.get` as function.

**Returns:**

``pandas.Series` or `pandas.DataFrame``

`pandas.Series` or `pandas.DataFrame` depending on type of object
#### to_string[[datasets.table.InMemoryTable.to_string]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L305)
#### field[[datasets.table.InMemoryTable.field]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L324)

Select a schema field by its column name or numeric index.

**Parameters:**

i (`Union[int, str]`) : The index or name of the field to retrieve.

**Returns:**

`pyarrow.Field`
#### column[[datasets.table.InMemoryTable.column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L337)

Select a column by its column name, or numeric index.

**Parameters:**

i (`Union[int, str]`) : The index or name of the column to retrieve.

**Returns:**

`pyarrow.ChunkedArray`
#### itercolumns[[datasets.table.InMemoryTable.itercolumns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L350)

Iterator over all columns in their numerical order.
#### schema[[datasets.table.InMemoryTable.schema]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L359)

Schema of the table and its columns.

**Returns:**

`pyarrow.Schema`
#### columns[[datasets.table.InMemoryTable.columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L369)

List of all columns in numerical order.

**Returns:**

`List[pa.ChunkedArray]`
#### num_columns[[datasets.table.InMemoryTable.num_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L379)

Number of columns in this table.

**Returns:**

int
#### num_rows[[datasets.table.InMemoryTable.num_rows]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L389)

Number of rows in this table.

Due to the definition of a table, all columns have the same number of
rows.

**Returns:**

int
#### shape[[datasets.table.InMemoryTable.shape]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L402)

Dimensions of the table: (#rows, #columns).

**Returns:**

``(int, int)``

Number of rows and number of columns.
#### nbytes[[datasets.table.InMemoryTable.nbytes]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L412)

Total number of bytes consumed by the elements of the table.
#### column_names[[datasets.table.InMemoryTable.column_names]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L419)

Names of the table's columns.
#### slice[[datasets.table.InMemoryTable.slice]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L793)

Compute zero-copy slice of this Table.

**Parameters:**

offset (`int`, defaults to `0`) : Offset from start of table to slice.

length (`int`, defaults to `None`) : Length of slice (default is until end of table starting from offset).

**Returns:**

`datasets.table.Table`
#### filter[[datasets.table.InMemoryTable.filter]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L810)

Select records from a Table. See `pyarrow.compute.filter` for full usage.
#### flatten[[datasets.table.InMemoryTable.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L816)

Flatten this Table.  Each column with a struct type is flattened
into one column per struct field.  Other columns are left unchanged.

**Parameters:**

memory_pool (`MemoryPool`, defaults to `None`) : For memory allocations, if required, otherwise use default pool.

**Returns:**

`datasets.table.Table`
#### combine_chunks[[datasets.table.InMemoryTable.combine_chunks]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L830)

Make a new table by combining the chunks this table has.

All the underlying chunks in the `ChunkedArray` of each column are
concatenated into zero or one chunk.

**Parameters:**

memory_pool (`MemoryPool`, defaults to `None`) : For memory allocations, if required, otherwise use default pool.

**Returns:**

`datasets.table.Table`
#### cast[[datasets.table.InMemoryTable.cast]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L846)

Cast table values to another schema.

**Parameters:**

target_schema (`Schema`) : Schema to cast to, the names and order of fields must match.

safe (`bool`, defaults to `True`) : Check for overflows or other unsafe conversions.

**Returns:**

`datasets.table.Table`
#### replace_schema_metadata[[datasets.table.InMemoryTable.replace_schema_metadata]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L861)

EXPERIMENTAL: Create shallow copy of table by replacing schema
key-value metadata with the indicated new metadata (which may be `None`,
which deletes any existing metadata).

**Parameters:**

metadata (`dict`, defaults to `None`) --

**Returns:**

``datasets.table.Table``

shallow_copy
#### add_column[[datasets.table.InMemoryTable.add_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L875)

Add column to Table at position.

A new table is returned with the column added, the original table
object is left unchanged.

**Parameters:**

i (`int`) : Index to place the column at.

field_ (`Union[str, pyarrow.Field]`) : If a string is passed then the type is deduced from the column data.

column (`Union[pyarrow.Array, List[pyarrow.Array]]`) : Column data.

**Returns:**

``datasets.table.Table``

New table with the passed column added.
#### append_column[[datasets.table.InMemoryTable.append_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L896)

Append column at end of columns.

**Parameters:**

field_ (`Union[str, pyarrow.Field]`) : If a string is passed then the type is deduced from the column data.

column (`Union[pyarrow.Array, List[pyarrow.Array]]`) : Column data.

**Returns:**

``datasets.table.Table``

New table with the passed column added.
#### remove_column[[datasets.table.InMemoryTable.remove_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L913)

Create new Table with the indicated column removed.

**Parameters:**

i (`int`) : Index of column to remove.

**Returns:**

``datasets.table.Table``

New table without the column.
#### set_column[[datasets.table.InMemoryTable.set_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L927)

Replace column in Table at position.

**Parameters:**

i (`int`) : Index to place the column at.

field_ (`Union[str, pyarrow.Field]`) : If a string is passed then the type is deduced from the column data.

column (`Union[pyarrow.Array, List[pyarrow.Array]]`) : Column data.

**Returns:**

``datasets.table.Table``

New table with the passed column set.
#### rename_columns[[datasets.table.InMemoryTable.rename_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L946)

Create new table with columns renamed to provided names.
#### select[[datasets.table.InMemoryTable.select]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L969)

Select columns of the table.

Returns a new table with the specified columns, and metadata preserved.

**Parameters:**

columns (`Union[List[str], List[int]]`) : The column names or integer indices to select.

**Returns:**

`[datasets.table.Table](/docs/datasets/v4.8.4/en/package_reference/table_classes#datasets.table.Table)`

New table with the specified columns, and metadata preserved.
#### drop[[datasets.table.InMemoryTable.drop]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L952)

Drop one or more columns and return a new table.

**Parameters:**

columns (`List[str]`) : List of field names referencing existing columns.

**Returns:**

``datasets.table.Table``

New table without the columns.
#### from_file[[datasets.table.InMemoryTable.from_file]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L653)
#### from_buffer[[datasets.table.InMemoryTable.from_buffer]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L658)
#### from_pandas[[datasets.table.InMemoryTable.from_pandas]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L663)

Convert pandas.DataFrame to an Arrow Table.

The column types in the resulting Arrow Table are inferred from the
dtypes of the pandas.Series in the DataFrame. In the case of non-object
Series, the NumPy dtype is translated to its Arrow equivalent. In the
case of `object`, we need to guess the datatype by looking at the
Python objects in this Series.

Be aware that Series of the `object` dtype don't carry enough
information to always lead to a meaningful Arrow type. In the case that
we cannot infer a type, e.g. because the DataFrame is of length 0 or
the Series only contains `None/nan` objects, the type is set to
null. This behavior can be avoided by constructing an explicit schema
and passing it to this function.

Examples:
```python
>>> import pandas as pd
>>> import pyarrow as pa
>>> df = pd.DataFrame({
    ...     'int': [1, 2],
    ...     'str': ['a', 'b']
    ... })
>>> pa.Table.from_pandas(df)

```

**Parameters:**

df (`pandas.DataFrame`) --

schema (`pyarrow.Schema`, *optional*) : The expected schema of the Arrow Table. This can be used to indicate the type of columns if we cannot infer it automatically. If passed, the output will have exactly this schema. Columns specified in the schema that are not found in the DataFrame columns or its index will raise an error. Additional columns or index levels in the DataFrame which are not specified in the schema will be ignored.

preserve_index (`bool`, *optional*) : Whether to store the index as an additional column in the resulting `Table`. The default of None will store the index as a column, except for RangeIndex which is stored as metadata only. Use `preserve_index=True` to force it to be stored as a column.

nthreads (`int`, defaults to `None` (may use up to system CPU count threads)) : If greater than 1, convert columns to Arrow in parallel using indicated number of threads.

columns (`List[str]`, *optional*) : List of column to be converted. If `None`, use all columns.

safe (`bool`, defaults to `True`) : Check for overflows or other unsafe conversions,

**Returns:**

``datasets.table.Table``
#### from_arrays[[datasets.table.InMemoryTable.from_arrays]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L721)

Construct a Table from Arrow arrays.

**Parameters:**

arrays (`List[Union[pyarrow.Array, pyarrow.ChunkedArray]]`) : Equal-length arrays that should form the table.

names (`List[str]`, *optional*) : Names for the table columns. If not passed, schema must be passed.

schema (`Schema`, defaults to `None`) : Schema for the created table. If not passed, names must be passed.

metadata (`Union[dict, Mapping]`, defaults to `None`) : Optional metadata for the schema (if inferred).

**Returns:**

`datasets.table.Table`
#### from_pydict[[datasets.table.InMemoryTable.from_pydict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L741)

Construct a Table from Arrow arrays or columns.

**Parameters:**

mapping (`Union[dict, Mapping]`) : A mapping of strings to Arrays or Python lists.

schema (`Schema`, defaults to `None`) : If not passed, will be inferred from the Mapping values

metadata (`Union[dict, Mapping]`, defaults to `None`) : Optional metadata for the schema (if inferred).

**Returns:**

`datasets.table.Table`
#### from_batches[[datasets.table.InMemoryTable.from_batches]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L777)

Construct a Table from a sequence or iterator of Arrow `RecordBatches`.

**Parameters:**

batches (`Union[Sequence[pyarrow.RecordBatch], Iterator[pyarrow.RecordBatch]]`) : Sequence of `RecordBatch` to be converted, all schemas must be equal.

schema (`Schema`, defaults to `None`) : If not passed, will be inferred from the first `RecordBatch`.

**Returns:**

``datasets.table.Table``

## MemoryMappedTable[[datasets.table.MemoryMappedTable]]

#### datasets.table.MemoryMappedTable[[datasets.table.MemoryMappedTable]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L989)

The table is said memory mapped when it doesn't use the user's RAM but loads the data
from the disk instead.

Pickling it doesn't copy the data into memory.
Instead, only the path to the memory mapped arrow file is pickled, as well as the list
of transforms to "replay" when reloading the table from the disk.

Its implementation requires to store an history of all the transforms that were applied
to the underlying pyarrow Table, so that they can be "replayed" when reloading the Table
from the disk.

This is different from the `InMemoryTable` table, for which pickling does copy all the
data in memory.

`InMemoryTable` must be used when data fit in memory, while `MemoryMapped` are reserved for
data bigger than memory or when you want the memory footprint of your application to
stay low.

validatedatasets.table.MemoryMappedTable.validatehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L178[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]- **full** (`bool`, defaults to `False`) --
  If `True`, run expensive checks, otherwise cheap checks only.0- ``pa.lib.ArrowInvalid`` -- if validation fails``pa.lib.ArrowInvalid``

Perform validation checks.  An exception is raised if validation fails.

By default only cheap validation checks are run.  Pass `full=True`
for thorough validation checks (potentially `O(n)`).

**Parameters:**

full (`bool`, defaults to `False`) : If `True`, run expensive checks, otherwise cheap checks only.
#### equals[[datasets.table.MemoryMappedTable.equals]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L194)

Check if contents of two tables are equal.

**Parameters:**

other ([Table](/docs/datasets/v4.8.4/en/package_reference/table_classes#datasets.table.Table)) : Table to compare against.

check_metadata `bool`, defaults to `False`) : Whether schema metadata equality should be checked as well.

**Returns:**

``bool``
#### to_batches[[datasets.table.MemoryMappedTable.to_batches]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L211)

Convert Table to list of (contiguous) `RecordBatch` objects.

**Parameters:**

max_chunksize (`int`, defaults to `None`) : Maximum size for `RecordBatch` chunks. Individual chunks may be smaller depending on the chunk layout of individual columns.

**Returns:**

`List[pyarrow.RecordBatch]`
#### to_pydict[[datasets.table.MemoryMappedTable.to_pydict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L225)

Convert the Table to a `dict` or `OrderedDict`.

**Returns:**

``dict``
#### to_pandas[[datasets.table.MemoryMappedTable.to_pandas]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L243)

Convert to a pandas-compatible NumPy array or DataFrame, as appropriate.

**Parameters:**

memory_pool (`MemoryPool`, defaults to `None`) : Arrow MemoryPool to use for allocations. Uses the default memory pool is not passed.

strings_to_categorical (`bool`, defaults to `False`) : Encode string (UTF8) and binary types to `pandas.Categorical`.

categories (`list`, defaults to `empty`) : List of fields that should be returned as `pandas.Categorical`. Only applies to table-like data structures.

zero_copy_only (`bool`, defaults to `False`) : Raise an `ArrowException` if this function call would require copying the underlying data.

integer_object_nulls (`bool`, defaults to `False`) : Cast integers with nulls to objects.

date_as_object (`bool`, defaults to `True`) : Cast dates to objects. If `False`, convert to `datetime64[ns]` dtype.

timestamp_as_object (`bool`, defaults to `False`) : Cast non-nanosecond timestamps (`np.datetime64`) to objects. This is useful if you have timestamps that don't fit in the normal date range of nanosecond timestamps (1678 CE-2262 CE). If `False`, all timestamps are converted to `datetime64[ns]` dtype.

use_threads (`bool`, defaults to `True`) : Whether to parallelize the conversion using multiple threads.

deduplicate_objects (`bool`, defaults to `False`) : Do not create multiple copies Python objects when created, to save on memory use. Conversion will be slower.

ignore_metadata (`bool`, defaults to `False`) : If `True`, do not use the 'pandas' metadata to reconstruct the DataFrame index, if present.

safe (`bool`, defaults to `True`) : For certain data types, a cast is needed in order to store the data in a pandas DataFrame or Series (e.g. timestamps are always stored as nanoseconds in pandas). This option controls whether it is a safe cast or not.

split_blocks (`bool`, defaults to `False`) : If `True`, generate one internal "block" for each column when creating a pandas.DataFrame from a `RecordBatch` or `Table`. While this can temporarily reduce memory note that various pandas operations can trigger "consolidation" which may balloon memory use.

self_destruct (`bool`, defaults to `False`) : EXPERIMENTAL: If `True`, attempt to deallocate the originating Arrow memory while converting the Arrow object to pandas. If you use the object after calling `to_pandas` with this option it will crash your program.

types_mapper (`function`, defaults to `None`) : A function mapping a pyarrow DataType to a pandas `ExtensionDtype`. This can be used to override the default pandas type for conversion of built-in pyarrow types or in absence of `pandas_metadata` in the Table schema. The function receives a pyarrow DataType and is expected to return a pandas `ExtensionDtype` or `None` if the default conversion should be used for that type. If you have a dictionary mapping, you can pass `dict.get` as function.

**Returns:**

``pandas.Series` or `pandas.DataFrame``

`pandas.Series` or `pandas.DataFrame` depending on type of object
#### to_string[[datasets.table.MemoryMappedTable.to_string]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L305)
#### field[[datasets.table.MemoryMappedTable.field]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L324)

Select a schema field by its column name or numeric index.

**Parameters:**

i (`Union[int, str]`) : The index or name of the field to retrieve.

**Returns:**

`pyarrow.Field`
#### column[[datasets.table.MemoryMappedTable.column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L337)

Select a column by its column name, or numeric index.

**Parameters:**

i (`Union[int, str]`) : The index or name of the column to retrieve.

**Returns:**

`pyarrow.ChunkedArray`
#### itercolumns[[datasets.table.MemoryMappedTable.itercolumns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L350)

Iterator over all columns in their numerical order.
#### schema[[datasets.table.MemoryMappedTable.schema]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L359)

Schema of the table and its columns.

**Returns:**

`pyarrow.Schema`
#### columns[[datasets.table.MemoryMappedTable.columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L369)

List of all columns in numerical order.

**Returns:**

`List[pa.ChunkedArray]`
#### num_columns[[datasets.table.MemoryMappedTable.num_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L379)

Number of columns in this table.

**Returns:**

int
#### num_rows[[datasets.table.MemoryMappedTable.num_rows]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L389)

Number of rows in this table.

Due to the definition of a table, all columns have the same number of
rows.

**Returns:**

int
#### shape[[datasets.table.MemoryMappedTable.shape]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L402)

Dimensions of the table: (#rows, #columns).

**Returns:**

``(int, int)``

Number of rows and number of columns.
#### nbytes[[datasets.table.MemoryMappedTable.nbytes]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L412)

Total number of bytes consumed by the elements of the table.
#### column_names[[datasets.table.MemoryMappedTable.column_names]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L419)

Names of the table's columns.
#### slice[[datasets.table.MemoryMappedTable.slice]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1048)

Compute zero-copy slice of this Table.

**Parameters:**

offset (`int`, defaults to `0`) : Offset from start of table to slice.

length (`int`, defaults to `None`) : Length of slice (default is until end of table starting from offset).

**Returns:**

`datasets.table.Table`
#### filter[[datasets.table.MemoryMappedTable.filter]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1067)

Select records from a Table. See `pyarrow.compute.filter` for full usage.
#### flatten[[datasets.table.MemoryMappedTable.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1075)

Flatten this Table.  Each column with a struct type is flattened
into one column per struct field.  Other columns are left unchanged.

**Parameters:**

memory_pool (`MemoryPool`, defaults to `None`) : For memory allocations, if required, otherwise use default pool.

**Returns:**

`datasets.table.Table`
#### combine_chunks[[datasets.table.MemoryMappedTable.combine_chunks]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1091)

Make a new table by combining the chunks this table has.

All the underlying chunks in the ChunkedArray of each column are
concatenated into zero or one chunk.

**Parameters:**

memory_pool (`MemoryPool`, defaults to `None`) : For memory allocations, if required, otherwise use default pool.

**Returns:**

`datasets.table.Table`
#### cast[[datasets.table.MemoryMappedTable.cast]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1109)

Cast table values to another schema

**Parameters:**

target_schema (`Schema`) : Schema to cast to, the names and order of fields must match.

safe (`bool`, defaults to `True`) : Check for overflows or other unsafe conversions.

**Returns:**

`datasets.table.Table`
#### replace_schema_metadata[[datasets.table.MemoryMappedTable.replace_schema_metadata]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1126)

EXPERIMENTAL: Create shallow copy of table by replacing schema
key-value metadata with the indicated new metadata (which may be None,
which deletes any existing metadata.

**Parameters:**

metadata (`dict`, defaults to `None`) --

**Returns:**

``datasets.table.Table``

shallow_copy
#### add_column[[datasets.table.MemoryMappedTable.add_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1142)

Add column to Table at position.

A new table is returned with the column added, the original table
object is left unchanged.

**Parameters:**

i (`int`) : Index to place the column at.

field_ (`Union[str, pyarrow.Field]`) : If a string is passed then the type is deduced from the column data.

column (`Union[pyarrow.Array, List[pyarrow.Array]]`) : Column data.

**Returns:**

``datasets.table.Table``

New table with the passed column added.
#### append_column[[datasets.table.MemoryMappedTable.append_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1165)

Append column at end of columns.

**Parameters:**

field_ (`Union[str, pyarrow.Field]`) : If a string is passed then the type is deduced from the column data.

column (`Union[pyarrow.Array, List[pyarrow.Array]]`) : Column data.

**Returns:**

``datasets.table.Table``

New table with the passed column added.
#### remove_column[[datasets.table.MemoryMappedTable.remove_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1184)

Create new Table with the indicated column removed.

**Parameters:**

i (`int`) : Index of column to remove.

**Returns:**

``datasets.table.Table``

New table without the column.
#### set_column[[datasets.table.MemoryMappedTable.set_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1200)

Replace column in Table at position.

**Parameters:**

i (`int`) : Index to place the column at.

field_ (`Union[str, pyarrow.Field]`) : If a string is passed then the type is deduced from the column data.

column (`Union[pyarrow.Array, List[pyarrow.Array]]`) : Column data.

**Returns:**

``datasets.table.Table``

New table with the passed column set.
#### rename_columns[[datasets.table.MemoryMappedTable.rename_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1221)

Create new table with columns renamed to provided names.
#### select[[datasets.table.MemoryMappedTable.select]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1248)

Select columns of the table.

Returns a new table with the specified columns, and metadata preserved.

**Parameters:**

columns (`Union[List[str], List[int]]`) : The column names or integer indices to select.

**Returns:**

`[datasets.table.Table](/docs/datasets/v4.8.4/en/package_reference/table_classes#datasets.table.Table)`

New table with the specified columns, and metadata preserved.
#### drop[[datasets.table.MemoryMappedTable.drop]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1229)

Drop one or more columns and return a new table.

**Parameters:**

columns (`List[str]`) : List of field names referencing existing columns.

**Returns:**

``datasets.table.Table``

New table without the columns.
#### from_file[[datasets.table.MemoryMappedTable.from_file]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1015)

## ConcatenationTable[[datasets.table.ConcatenationTable]]

#### datasets.table.ConcatenationTable[[datasets.table.ConcatenationTable]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1273)

The table comes from the concatenation of several tables called blocks.
It enables concatenation on both axis 0 (append rows) and axis 1 (append columns).

The underlying tables are called "blocks" and can be either `InMemoryTable`
or `MemoryMappedTable` objects.
This allows to combine tables that come from memory or that are memory mapped.
When a `ConcatenationTable` is pickled, then each block is pickled:
- the `InMemoryTable` objects are pickled by copying all the data in memory.
- the MemoryMappedTable objects are pickled without copying the data into memory.
Instead, only the path to the memory mapped arrow file is pickled, as well as the list
of transforms to "replays" when reloading the table from the disk.

Its implementation requires to store each block separately.
The `blocks` attributes stores a list of list of blocks.
The first axis concatenates the tables along the axis 0 (it appends rows),
while the second axis concatenates tables along the axis 1 (it appends columns).

If some columns are missing when concatenating on axis 0, they are filled with null values.
This is done using `pyarrow.concat_tables(tables, promote=True)`.

You can access the fully combined table by accessing the `ConcatenationTable.table` attribute,
and the blocks by accessing the `ConcatenationTable.blocks` attribute.

validatedatasets.table.ConcatenationTable.validatehttps://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L178[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]- **full** (`bool`, defaults to `False`) --
  If `True`, run expensive checks, otherwise cheap checks only.0- ``pa.lib.ArrowInvalid`` -- if validation fails``pa.lib.ArrowInvalid``

Perform validation checks.  An exception is raised if validation fails.

By default only cheap validation checks are run.  Pass `full=True`
for thorough validation checks (potentially `O(n)`).

**Parameters:**

full (`bool`, defaults to `False`) : If `True`, run expensive checks, otherwise cheap checks only.
#### equals[[datasets.table.ConcatenationTable.equals]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L194)

Check if contents of two tables are equal.

**Parameters:**

other ([Table](/docs/datasets/v4.8.4/en/package_reference/table_classes#datasets.table.Table)) : Table to compare against.

check_metadata `bool`, defaults to `False`) : Whether schema metadata equality should be checked as well.

**Returns:**

``bool``
#### to_batches[[datasets.table.ConcatenationTable.to_batches]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L211)

Convert Table to list of (contiguous) `RecordBatch` objects.

**Parameters:**

max_chunksize (`int`, defaults to `None`) : Maximum size for `RecordBatch` chunks. Individual chunks may be smaller depending on the chunk layout of individual columns.

**Returns:**

`List[pyarrow.RecordBatch]`
#### to_pydict[[datasets.table.ConcatenationTable.to_pydict]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L225)

Convert the Table to a `dict` or `OrderedDict`.

**Returns:**

``dict``
#### to_pandas[[datasets.table.ConcatenationTable.to_pandas]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L243)

Convert to a pandas-compatible NumPy array or DataFrame, as appropriate.

**Parameters:**

memory_pool (`MemoryPool`, defaults to `None`) : Arrow MemoryPool to use for allocations. Uses the default memory pool is not passed.

strings_to_categorical (`bool`, defaults to `False`) : Encode string (UTF8) and binary types to `pandas.Categorical`.

categories (`list`, defaults to `empty`) : List of fields that should be returned as `pandas.Categorical`. Only applies to table-like data structures.

zero_copy_only (`bool`, defaults to `False`) : Raise an `ArrowException` if this function call would require copying the underlying data.

integer_object_nulls (`bool`, defaults to `False`) : Cast integers with nulls to objects.

date_as_object (`bool`, defaults to `True`) : Cast dates to objects. If `False`, convert to `datetime64[ns]` dtype.

timestamp_as_object (`bool`, defaults to `False`) : Cast non-nanosecond timestamps (`np.datetime64`) to objects. This is useful if you have timestamps that don't fit in the normal date range of nanosecond timestamps (1678 CE-2262 CE). If `False`, all timestamps are converted to `datetime64[ns]` dtype.

use_threads (`bool`, defaults to `True`) : Whether to parallelize the conversion using multiple threads.

deduplicate_objects (`bool`, defaults to `False`) : Do not create multiple copies Python objects when created, to save on memory use. Conversion will be slower.

ignore_metadata (`bool`, defaults to `False`) : If `True`, do not use the 'pandas' metadata to reconstruct the DataFrame index, if present.

safe (`bool`, defaults to `True`) : For certain data types, a cast is needed in order to store the data in a pandas DataFrame or Series (e.g. timestamps are always stored as nanoseconds in pandas). This option controls whether it is a safe cast or not.

split_blocks (`bool`, defaults to `False`) : If `True`, generate one internal "block" for each column when creating a pandas.DataFrame from a `RecordBatch` or `Table`. While this can temporarily reduce memory note that various pandas operations can trigger "consolidation" which may balloon memory use.

self_destruct (`bool`, defaults to `False`) : EXPERIMENTAL: If `True`, attempt to deallocate the originating Arrow memory while converting the Arrow object to pandas. If you use the object after calling `to_pandas` with this option it will crash your program.

types_mapper (`function`, defaults to `None`) : A function mapping a pyarrow DataType to a pandas `ExtensionDtype`. This can be used to override the default pandas type for conversion of built-in pyarrow types or in absence of `pandas_metadata` in the Table schema. The function receives a pyarrow DataType and is expected to return a pandas `ExtensionDtype` or `None` if the default conversion should be used for that type. If you have a dictionary mapping, you can pass `dict.get` as function.

**Returns:**

``pandas.Series` or `pandas.DataFrame``

`pandas.Series` or `pandas.DataFrame` depending on type of object
#### to_string[[datasets.table.ConcatenationTable.to_string]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L305)
#### field[[datasets.table.ConcatenationTable.field]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L324)

Select a schema field by its column name or numeric index.

**Parameters:**

i (`Union[int, str]`) : The index or name of the field to retrieve.

**Returns:**

`pyarrow.Field`
#### column[[datasets.table.ConcatenationTable.column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L337)

Select a column by its column name, or numeric index.

**Parameters:**

i (`Union[int, str]`) : The index or name of the column to retrieve.

**Returns:**

`pyarrow.ChunkedArray`
#### itercolumns[[datasets.table.ConcatenationTable.itercolumns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L350)

Iterator over all columns in their numerical order.
#### schema[[datasets.table.ConcatenationTable.schema]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L359)

Schema of the table and its columns.

**Returns:**

`pyarrow.Schema`
#### columns[[datasets.table.ConcatenationTable.columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L369)

List of all columns in numerical order.

**Returns:**

`List[pa.ChunkedArray]`
#### num_columns[[datasets.table.ConcatenationTable.num_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L379)

Number of columns in this table.

**Returns:**

int
#### num_rows[[datasets.table.ConcatenationTable.num_rows]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L389)

Number of rows in this table.

Due to the definition of a table, all columns have the same number of
rows.

**Returns:**

int
#### shape[[datasets.table.ConcatenationTable.shape]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L402)

Dimensions of the table: (#rows, #columns).

**Returns:**

``(int, int)``

Number of rows and number of columns.
#### nbytes[[datasets.table.ConcatenationTable.nbytes]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L412)

Total number of bytes consumed by the elements of the table.
#### column_names[[datasets.table.ConcatenationTable.column_names]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L419)

Names of the table's columns.
#### slice[[datasets.table.ConcatenationTable.slice]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1482)

Compute zero-copy slice of this Table.

**Parameters:**

offset (`int`, defaults to `0`) : Offset from start of table to slice.

length (`int`, defaults to `None`) : Length of slice (default is until end of table starting from offset).

**Returns:**

`datasets.table.Table`
#### filter[[datasets.table.ConcatenationTable.filter]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1513)

Select records from a Table. See `pyarrow.compute.filter` for full usage.
#### flatten[[datasets.table.ConcatenationTable.flatten]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1524)

Flatten this Table.  Each column with a struct type is flattened
into one column per struct field.  Other columns are left unchanged.

**Parameters:**

memory_pool (`MemoryPool`, defaults to `None`) : For memory allocations, if required, otherwise use default pool.

**Returns:**

`datasets.table.Table`
#### combine_chunks[[datasets.table.ConcatenationTable.combine_chunks]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1542)

Make a new table by combining the chunks this table has.

All the underlying chunks in the `ChunkedArray` of each column are
concatenated into zero or one chunk.

**Parameters:**

memory_pool (`MemoryPool`, defaults to `None`) : For memory allocations, if required, otherwise use default pool.

**Returns:**

`datasets.table.Table`
#### cast[[datasets.table.ConcatenationTable.cast]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1562)

Cast table values to another schema.

**Parameters:**

target_schema (`Schema`) : Schema to cast to, the names and order of fields must match.

safe (`bool`, defaults to `True`) : Check for overflows or other unsafe conversions.

**Returns:**

`datasets.table.Table`
#### replace_schema_metadata[[datasets.table.ConcatenationTable.replace_schema_metadata]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1593)

EXPERIMENTAL: Create shallow copy of table by replacing schema
key-value metadata with the indicated new metadata (which may be `None`,
which deletes any existing metadata).

**Parameters:**

metadata (`dict`, defaults to `None`) --

**Returns:**

``datasets.table.Table``

shallow_copy
#### add_column[[datasets.table.ConcatenationTable.add_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1611)

Add column to Table at position.

A new table is returned with the column added, the original table
object is left unchanged.

**Parameters:**

i (`int`) : Index to place the column at.

field_ (`Union[str, pyarrow.Field]`) : If a string is passed then the type is deduced from the column data.

column (`Union[pyarrow.Array, List[pyarrow.Array]]`) : Column data.

**Returns:**

``datasets.table.Table``

New table with the passed column added.
#### append_column[[datasets.table.ConcatenationTable.append_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1632)

Append column at end of columns.

**Parameters:**

field_ (`Union[str, pyarrow.Field]`) : If a string is passed then the type is deduced from the column data.

column (`Union[pyarrow.Array, List[pyarrow.Array]]`) : Column data.

**Returns:**

``datasets.table.Table``

New table with the passed column added.
#### remove_column[[datasets.table.ConcatenationTable.remove_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1649)

Create new Table with the indicated column removed.

**Parameters:**

i (`int`) : Index of column to remove.

**Returns:**

``datasets.table.Table``

New table without the column.
#### set_column[[datasets.table.ConcatenationTable.set_column]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1673)

Replace column in Table at position.

**Parameters:**

i (`int`) : Index to place the column at.

field_ (`Union[str, pyarrow.Field]`) : If a string is passed then the type is deduced from the column data.

column (`Union[pyarrow.Array, List[pyarrow.Array]]`) : Column data.

**Returns:**

``datasets.table.Table``

New table with the passed column set.
#### rename_columns[[datasets.table.ConcatenationTable.rename_columns]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1692)

Create new table with columns renamed to provided names.
#### select[[datasets.table.ConcatenationTable.select]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1726)

Select columns of the table.

Returns a new table with the specified columns, and metadata preserved.

**Parameters:**

columns (`Union[List[str], List[int]]`) : The column names or integer indices to select.

**Returns:**

`[datasets.table.Table](/docs/datasets/v4.8.4/en/package_reference/table_classes#datasets.table.Table)`

New table with the specified columns, and metadata preserved.
#### drop[[datasets.table.ConcatenationTable.drop]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1705)

Drop one or more columns and return a new table.

**Parameters:**

columns (`List[str]`) : List of field names referencing existing columns.

**Returns:**

``datasets.table.Table``

New table without the columns.
#### from_blocks[[datasets.table.ConcatenationTable.from_blocks]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1378)
#### from_tables[[datasets.table.ConcatenationTable.from_tables]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1392)

Create `ConcatenationTable` from list of tables.

**Parameters:**

tables (list of `Table` or list of `pyarrow.Table`) : List of tables.

axis (`{0, 1}`, defaults to `0`, meaning over rows) : Axis to concatenate over, where `0` means over rows (vertically) and `1` means over columns (horizontally).  

## Utils[[datasets.table.concat_tables]]

#### datasets.table.concat_tables[[datasets.table.concat_tables]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1746)

Concatenate tables.

**Parameters:**

tables (list of `Table`) : List of tables to be concatenated.

axis (`{0, 1}`, defaults to `0`, meaning over rows) : Axis to concatenate over, where `0` means over rows (vertically) and `1` means over columns (horizontally).  

**Returns:**

``datasets.table.Table``

If the number of input tables is > 1, then the returned table is a `datasets.table.ConcatenationTable`.
Otherwise if there's only one table, it is returned as is.

#### datasets.table.list_table_cache_files[[datasets.table.list_table_cache_files]]

[Source](https://github.com/huggingface/datasets/blob/4.8.4/src/datasets/table.py#L1769)

Get the cache files that are loaded by the table.
Cache file are used when parts of the table come from the disk via memory mapping.

**Returns:**

``List[str]``

A list of paths to the cache files loaded by the table.
