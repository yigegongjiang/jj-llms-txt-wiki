# Cache-system reference

The caching system was updated in v0.8.0 to become the central cache-system shared
across libraries that depend on the Hub. Read the [cache-system guide](../guides/manage-cache)
for a detailed presentation of caching at HF.

## Helpers

### try_to_load_from_cache[[huggingface_hub.try_to_load_from_cache]]

#### huggingface_hub.try_to_load_from_cache[[huggingface_hub.try_to_load_from_cache]]

```python
huggingface_hub.try_to_load_from_cache(repo_id: str, filename: str, cache_dir: str | pathlib.Path | None = None, revision: str | None = None, repo_type: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/file_download.py#L1474)

**Parameters:**

cache_dir (`str` or `os.PathLike`) : The folder where the cached files lie.

repo_id (`str`) : The ID of the repo on huggingface.co.

filename (`str`) : The filename to look for inside `repo_id`.

revision (`str`, *optional*) : The specific model version to use. Will default to `"main"` if it's not provided and no `commit_hash` is provided either.

repo_type (`str`, *optional*) : The type of the repository. Will default to `"model"`.

**Returns:** `Optional[str]` or `_CACHED_NO_EXIST`

Will return `None` if the file was not cached. Otherwise:
- The exact path to the cached file if it's found in the cache
- A special value `_CACHED_NO_EXIST` if the file does not exist at the given commit hash and this fact was
  cached.

Explores the cache to return the latest cached file for a given revision if found.

This function will not raise any exception if the file in not cached.

Example:

```python
from huggingface_hub import try_to_load_from_cache, _CACHED_NO_EXIST

filepath = try_to_load_from_cache()
if isinstance(filepath, str):
    # file exists and is cached
    ...
elif filepath is _CACHED_NO_EXIST:
    # non-existence of file is cached
    ...
else:
    # file is not cached
    ...
```

### cached_assets_path[[huggingface_hub.cached_assets_path]]

#### huggingface_hub.cached_assets_path[[huggingface_hub.cached_assets_path]]

```python
huggingface_hub.cached_assets_path(library_name: str, namespace: str = 'default', subfolder: str = 'default', assets_dir: str | pathlib.Path | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_assets.py#L19)

**Parameters:**

library_name (`str`) : Name of the library that will manage the cache folder. Example: `"dataset"`.

namespace (`str`, *optional*, defaults to "default") : Namespace to which the data belongs. Example: `"SQuAD"`.

subfolder (`str`, *optional*, defaults to "default") : Subfolder in which the data will be stored. Example: `extracted`.

assets_dir (`str`, `Path`, *optional*) : Path to the folder where assets are cached. This must not be the same folder where Hub files are cached. Defaults to `HF_HOME / "assets"` if not provided. Can also be set with `HF_ASSETS_CACHE` environment variable.

**Returns:**

Path to the cache folder (`Path`).

Return a folder path to cache arbitrary files.

`huggingface_hub` provides a canonical folder path to store assets. This is the
recommended way to integrate cache in a downstream library as it will benefit from
the builtins tools to scan and delete the cache properly.

The distinction is made between files cached from the Hub and assets. Files from the
Hub are cached in a git-aware manner and entirely managed by `huggingface_hub`. See
[related documentation](https://huggingface.co/docs/huggingface_hub/how-to-cache).
All other files that a downstream library caches are considered to be "assets"
(files downloaded from external sources, extracted from a .tar archive, preprocessed
for training,...).

Once the folder path is generated, it is guaranteed to exist and to be a directory.
The path is based on 3 levels of depth: the library name, a namespace and a
subfolder. Those 3 levels grants flexibility while allowing `huggingface_hub` to
expect folders when scanning/deleting parts of the assets cache. Within a library,
it is expected that all namespaces share the same subset of subfolder names but this
is not a mandatory rule. The downstream library has then full control on which file
structure to adopt within its cache. Namespace and subfolder are optional (would
default to a `"default/"` subfolder) but library name is mandatory as we want every
downstream library to manage its own cache.

Expected tree:
```text
    assets/
    └── datasets/
    │   ├── SQuAD/
    │   │   ├── downloaded/
    │   │   ├── extracted/
    │   │   └── processed/
    │   ├── Helsinki-NLP--tatoeba_mt/
    │       ├── downloaded/
    │       ├── extracted/
    │       └── processed/
    └── transformers/
        ├── default/
        │   ├── something/
        ├── bert-base-cased/
        │   ├── default/
        │   └── training/
    hub/
    └── models--julien-c--EsperBERTo-small/
        ├── blobs/
        │   ├── (...)
        │   ├── (...)
        ├── refs/
        │   └── (...)
        └── [ 128]  snapshots/
            ├── 2439f60ef33a0d46d85da5001d52aeda5b00ce9f/
            │   ├── (...)
            └── bbc77c8132af1cc5cf678da3f1ddf2de43606d48/
                └── (...)
```

Example:
```py
>>> from huggingface_hub import cached_assets_path

>>> cached_assets_path(library_name="datasets", namespace="SQuAD", subfolder="download")
PosixPath('/home/wauplin/.cache/huggingface/extra/datasets/SQuAD/download')

>>> cached_assets_path(library_name="datasets", namespace="SQuAD", subfolder="extracted")
PosixPath('/home/wauplin/.cache/huggingface/extra/datasets/SQuAD/extracted')

>>> cached_assets_path(library_name="datasets", namespace="Helsinki-NLP/tatoeba_mt")
PosixPath('/home/wauplin/.cache/huggingface/extra/datasets/Helsinki-NLP--tatoeba_mt/default')

>>> cached_assets_path(library_name="datasets", assets_dir="/tmp/tmp123456")
PosixPath('/tmp/tmp123456/datasets/default/default')
```

### scan_cache_dir[[huggingface_hub.scan_cache_dir]]

#### huggingface_hub.scan_cache_dir[[huggingface_hub.scan_cache_dir]]

```python
huggingface_hub.scan_cache_dir(cache_dir: str | pathlib.Path | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L597)

**Parameters:**

cache_dir (`str` or `Path`, `optional`) : Cache directory to cache. Defaults to the default HF cache directory.

Scan the entire HF cache-system and return a [~HFCacheInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.HFCacheInfo) structure.

Use `scan_cache_dir` in order to programmatically scan your cache-system. The cache
will be scanned repo by repo. If a repo is corrupted, a [~CorruptedCacheException](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.CorruptedCacheException)
will be thrown internally but captured and returned in the [~HFCacheInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.HFCacheInfo)
structure. Only valid repos get a proper report.

```py
>>> from huggingface_hub import scan_cache_dir

>>> hf_cache_info = scan_cache_dir()
HFCacheInfo(
    size_on_disk=3398085269,
    repos=frozenset({
        CachedRepoInfo(
            repo_id='t5-small',
            repo_type='model',
            repo_path=PosixPath(...),
            size_on_disk=970726914,
            nb_files=11,
            revisions=frozenset({
                CachedRevisionInfo(
                    commit_hash='d78aea13fa7ecd06c29e3e46195d6341255065d5',
                    size_on_disk=970726339,
                    snapshot_path=PosixPath(...),
                    files=frozenset({
                        CachedFileInfo(
                            file_name='config.json',
                            size_on_disk=1197
                            file_path=PosixPath(...),
                            blob_path=PosixPath(...),
                        ),
                        CachedFileInfo(...),
                        ...
                    }),
                ),
                CachedRevisionInfo(...),
                ...
            }),
        ),
        CachedRepoInfo(...),
        ...
    }),
    warnings=[
        CorruptedCacheException("Snapshots dir doesn't exist in cached repo: ..."),
        CorruptedCacheException(...),
        ...
    ],
)
```

You can also print a detailed report directly from the `hf` command line using:
```text
> hf cache ls
ID                          SIZE     LAST_ACCESSED LAST_MODIFIED REFS
--------------------------- -------- ------------- ------------- -----------
dataset/nyu-mll/glue          157.4M 2 days ago    2 days ago    main script
model/LiquidAI/LFM2-VL-1.6B     3.2G 4 days ago    4 days ago    main
model/microsoft/UserLM-8b      32.1G 4 days ago    4 days ago    main

Done in 0.0s. Scanned 6 repo(s) for a total of 3.4G.
Got 1 warning(s) while scanning. Use -vvv to print details.
```

> [!WARNING]
> Raises:
>
>     `CacheNotFound`
>       If the cache directory does not exist.
>
>     [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError)
>       If the cache directory is a file, instead of a directory.

Returns: a [~HFCacheInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.HFCacheInfo) object.

## Data structures

All structures are built and returned by [scan_cache_dir()](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.scan_cache_dir) and are immutable.

### HFCacheInfo[[huggingface_hub.HFCacheInfo]]

#### huggingface_hub.HFCacheInfo[[huggingface_hub.HFCacheInfo]]

```python
huggingface_hub.HFCacheInfo(size_on_disk: int, repos: frozenset, incomplete_files: frozenset, warnings: list)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L349)

**Parameters:**

size_on_disk (`int`) : Sum of all valid repo sizes in the cache-system.

repos (`frozenset[CachedRepoInfo]`) : Set of [~CachedRepoInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.CachedRepoInfo) describing all valid cached repos found on the cache-system while scanning.

incomplete_files (`frozenset[CachedIncompleteFileInfo]`) : Set of [~CachedIncompleteFileInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.CachedIncompleteFileInfo) describing orphaned `*.incomplete` files left behind by interrupted downloads.

warnings (`list[CorruptedCacheException]`) : List of [~CorruptedCacheException](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.CorruptedCacheException) that occurred while scanning the cache. Those exceptions are captured so that the scan can continue. Corrupted repos are skipped from the scan.

Frozen data structure holding information about the entire cache-system.

This data structure is returned by [scan_cache_dir()](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.scan_cache_dir) and is immutable.

> [!WARNING]
> Here `size_on_disk` is equal to the sum of all repo sizes (only blobs). However if
> some cached repos are corrupted, their sizes are not taken into account.

#### delete_revisions[[huggingface_hub.HFCacheInfo.delete_revisions]]

```python
delete_revisions(*revisions: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L393)

Prepare the strategy to delete one or more revisions cached locally.

Input revisions can be any revision hash. If a revision hash is not found in the
local cache, a warning is thrown but no error is raised. Revisions can be from
different cached repos since hashes are unique across repos,

Examples:
```py
>>> from huggingface_hub import scan_cache_dir
>>> cache_info = scan_cache_dir()
>>> delete_strategy = cache_info.delete_revisions(
...     "81fd1d6e7847c99f5862c9fb81387956d99ec7aa"
... )
>>> print(f"Will free {delete_strategy.expected_freed_size_str}.")
Will free 7.9K.
>>> delete_strategy.execute()
Cache deletion done. Saved 7.9K.
```

```py
>>> from huggingface_hub import scan_cache_dir
>>> scan_cache_dir().delete_revisions(
...     "81fd1d6e7847c99f5862c9fb81387956d99ec7aa",
...     "e2983b237dccf3ab4937c97fa717319a9ca1a96d",
...     "6c0e6080953db56375760c0471a8c5f2929baf11",
... ).execute()
Cache deletion done. Saved 8.6G.
```

> [!WARNING]
> `delete_revisions` returns a [DeleteCacheStrategy](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.DeleteCacheStrategy) object that needs to
> be executed. The [DeleteCacheStrategy](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.DeleteCacheStrategy) is not meant to be modified but
> allows having a dry run before actually executing the deletion.

#### export_as_table[[huggingface_hub.HFCacheInfo.export_as_table]]

```python
export_as_table(verbosity: int = 0)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L502)

**Parameters:**

verbosity (`int`, *optional*) : The verbosity level. Defaults to 0.

**Returns:** `str`

The table as a string.

Generate a table from the [HFCacheInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.HFCacheInfo) object.

Pass `verbosity=0` to get a table with a single row per repo, with columns
"repo_id", "repo_type", "size_on_disk", "nb_files", "last_accessed", "last_modified", "refs", "local_path".

Pass `verbosity=1` to get a table with a row per repo and revision (thus multiple rows can appear for a single repo), with columns
"repo_id", "repo_type", "revision", "size_on_disk", "nb_files", "last_modified", "refs", "local_path".

Example:
```py
>>> from huggingface_hub.utils import scan_cache_dir

>>> hf_cache_info = scan_cache_dir()
HFCacheInfo(...)

>>> print(hf_cache_info.export_as_table())
REPO ID                                             REPO TYPE SIZE ON DISK NB FILES LAST_ACCESSED LAST_MODIFIED REFS LOCAL PATH
--------------------------------------------------- --------- ------------ -------- ------------- ------------- ---- --------------------------------------------------------------------------------------------------
roberta-base                                        model             2.7M        5 1 day ago     1 week ago    main ~/.cache/huggingface/hub/models--roberta-base
suno/bark                                           model             8.8K        1 1 week ago    1 week ago    main ~/.cache/huggingface/hub/models--suno--bark
t5-base                                             model           893.8M        4 4 days ago    7 months ago  main ~/.cache/huggingface/hub/models--t5-base
t5-large                                            model             3.0G        4 5 weeks ago   5 months ago  main ~/.cache/huggingface/hub/models--t5-large

>>> print(hf_cache_info.export_as_table(verbosity=1))
REPO ID                                             REPO TYPE REVISION                                 SIZE ON DISK NB FILES LAST_MODIFIED REFS LOCAL PATH
--------------------------------------------------- --------- ---------------------------------------- ------------ -------- ------------- ---- -----------------------------------------------------------------------------------------------------------------------------------------------------
roberta-base                                        model     e2da8e2f811d1448a5b465c236feacd80ffbac7b         2.7M        5 1 week ago    main ~/.cache/huggingface/hub/models--roberta-base/snapshots/e2da8e2f811d1448a5b465c236feacd80ffbac7b
suno/bark                                           model     70a8a7d34168586dc5d028fa9666aceade177992         8.8K        1 1 week ago    main ~/.cache/huggingface/hub/models--suno--bark/snapshots/70a8a7d34168586dc5d028fa9666aceade177992
t5-base                                             model     a9723ea7f1b39c1eae772870f3b547bf6ef7e6c1       893.8M        4 7 months ago  main ~/.cache/huggingface/hub/models--t5-base/snapshots/a9723ea7f1b39c1eae772870f3b547bf6ef7e6c1
t5-large                                            model     150ebc2c4b72291e770f58e6057481c8d2ed331a         3.0G        4 5 months ago  main ~/.cache/huggingface/hub/models--t5-large/snapshots/150ebc2c4b72291e770f58e6057481c8d2ed331a
```

### CachedRepoInfo[[huggingface_hub.CachedRepoInfo]]

#### huggingface_hub.CachedRepoInfo[[huggingface_hub.CachedRepoInfo]]

```python
huggingface_hub.CachedRepoInfo(repo_id: str, repo_type: typing.Literal['model', 'dataset', 'space'], repo_path: Path, size_on_disk: int, nb_files: int, revisions: frozenset, last_accessed: float, last_modified: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L175)

**Parameters:**

repo_id (`str`) : Repo id of the repo on the Hub. Example: `"google/fleurs"`.

repo_type (`Literal["dataset", "model", "space"]`) : Type of the cached repo.

repo_path (`Path`) : Local path to the cached repo.

size_on_disk (`int`) : Sum of the blob file sizes in the cached repo.

nb_files (`int`) : Total number of blob files in the cached repo.

revisions (`frozenset[CachedRevisionInfo]`) : Set of [~CachedRevisionInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.CachedRevisionInfo) describing all revisions cached in the repo.

last_accessed (`float`) : Timestamp of the last time a blob file of the repo has been accessed.

last_modified (`float`) : Timestamp of the last time a blob file of the repo has been modified/created.

Frozen data structure holding information about a cached repository.

> [!WARNING]
> `size_on_disk` is not necessarily the sum of all revisions sizes because of
> duplicated files. Besides, only blobs are taken into account, not the (negligible)
> size of folders and symlinks.

> [!WARNING]
> `last_accessed` and `last_modified` reliability can depend on the OS you are using.
> See [python documentation](https://docs.python.org/3/library/os.html#os.stat_result)
> for more details.

#### size_on_disk_str[[huggingface_hub.CachedRepoInfo.size_on_disk_str]]

```python
size_on_disk_str()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L237)

(property) Sum of the blob file sizes as a human-readable string.

Example: "42.2K".

#### refs[[huggingface_hub.CachedRepoInfo.refs]]

```python
refs()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L251)

(property) Mapping between `refs` and revision data structures.

### CachedRevisionInfo[[huggingface_hub.CachedRevisionInfo]]

#### huggingface_hub.CachedRevisionInfo[[huggingface_hub.CachedRevisionInfo]]

```python
huggingface_hub.CachedRevisionInfo(commit_hash: str, snapshot_path: Path, size_on_disk: int, files: frozenset, refs: frozenset, last_modified: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L104)

**Parameters:**

commit_hash (`str`) : Hash of the revision (unique). Example: `"9338f7b671827df886678df2bdd7cc7b4f36dffd"`.

snapshot_path (`Path`) : Path to the revision directory in the `snapshots` folder. It contains the exact tree structure as the repo on the Hub.

files : (`frozenset[CachedFileInfo]`): Set of [~CachedFileInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.CachedFileInfo) describing all files contained in the snapshot.

refs (`frozenset[str]`) : Set of `refs` pointing to this revision. If the revision has no `refs`, it is considered detached. Example: `{"main", "2.4.0"}` or `{"refs/pr/1"}`.

size_on_disk (`int`) : Sum of the blob file sizes that are symlink-ed by the revision.

last_modified (`float`) : Timestamp of the last time the revision has been created/modified.

Frozen data structure holding information about a revision.

A revision correspond to a folder in the `snapshots` folder and is populated with
the exact tree structure as the repo on the Hub but contains only symlinks. A
revision can be either referenced by 1 or more `refs` or be "detached" (no refs).

> [!WARNING]
> `last_accessed` cannot be determined correctly on a single revision as blob files
> are shared across revisions.

> [!WARNING]
> `size_on_disk` is not necessarily the sum of all file sizes because of possible
> duplicated files. Besides, only blobs are taken into account, not the (negligible)
> size of folders and symlinks.

#### size_on_disk_str[[huggingface_hub.CachedRevisionInfo.size_on_disk_str]]

```python
size_on_disk_str()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L157)

(property) Sum of the blob file sizes as a human-readable string.

Example: "42.2K".

#### nb_files[[huggingface_hub.CachedRevisionInfo.nb_files]]

```python
nb_files()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L166)

(property) Total number of files in the revision.

### CachedFileInfo[[huggingface_hub.CachedFileInfo]]

#### huggingface_hub.CachedFileInfo[[huggingface_hub.CachedFileInfo]]

```python
huggingface_hub.CachedFileInfo(file_name: str, file_path: Path, blob_path: Path, size_on_disk: int, blob_last_accessed: float, blob_last_modified: float)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L40)

**Parameters:**

file_name (`str`) : Name of the file. Example: `config.json`.

file_path (`Path`) : Path of the file in the `snapshots` directory. The file path is a symlink referring to a blob in the `blobs` folder.

blob_path (`Path`) : Path of the blob file. This is equivalent to `file_path.resolve()`.

size_on_disk (`int`) : Size of the blob file in bytes.

blob_last_accessed (`float`) : Timestamp of the last time the blob file has been accessed (from any revision).

blob_last_modified (`float`) : Timestamp of the last time the blob file has been modified/created.

Frozen data structure holding information about a single cached file.

> [!WARNING]
> `blob_last_accessed` and `blob_last_modified` reliability can depend on the OS you
> are using. See [python documentation](https://docs.python.org/3/library/os.html#os.stat_result)
> for more details.

#### size_on_disk_str[[huggingface_hub.CachedFileInfo.size_on_disk_str]]

```python
size_on_disk_str()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L93)

(property) Size of the blob file as a human-readable string.

Example: "42.2K".

### CachedIncompleteFileInfo[[huggingface_hub.CachedIncompleteFileInfo]]

#### huggingface_hub.CachedIncompleteFileInfo[[huggingface_hub.CachedIncompleteFileInfo]]

```python
huggingface_hub.CachedIncompleteFileInfo(file_path: Path, size_on_disk: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L330)

**Parameters:**

file_path (`Path`) : Path of the `.incomplete` file in the `blobs` folder.

size_on_disk (`int`) : Size of the partially-downloaded file in bytes.

Frozen data structure holding information about a single incomplete download.

Interrupted downloads leave `<cache>/<repo>/blobs/<etag>.incomplete` files behind.
These are not part of any committed revision, so they are surfaced separately by
[scan_cache_dir()](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.scan_cache_dir).

### DeleteCacheStrategy[[huggingface_hub.DeleteCacheStrategy]]

#### huggingface_hub.DeleteCacheStrategy[[huggingface_hub.DeleteCacheStrategy]]

```python
huggingface_hub.DeleteCacheStrategy(expected_freed_size: int, blobs: frozenset, refs: frozenset, repos: frozenset, snapshots: frozenset)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L260)

**Parameters:**

expected_freed_size (`float`) : Expected freed size once strategy is executed.

blobs (`frozenset[Path]`) : Set of blob file paths to be deleted.

refs (`frozenset[Path]`) : Set of reference file paths to be deleted.

repos (`frozenset[Path]`) : Set of entire repo paths to be deleted.

snapshots (`frozenset[Path]`) : Set of snapshots to be deleted (directory of symlinks).

Frozen data structure holding the strategy to delete cached revisions.

This object is not meant to be instantiated programmatically but to be returned by
[delete_revisions()](/docs/huggingface_hub/v1.27.0/en/package_reference/cache#huggingface_hub.HFCacheInfo.delete_revisions). See documentation for usage example.

#### expected_freed_size_str[[huggingface_hub.DeleteCacheStrategy.expected_freed_size_str]]

```python
expected_freed_size_str()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/utils/_cache_manager.py#L285)

(property) Expected size that will be freed as a human-readable string.

Example: "42.2K".

## Exceptions

### CorruptedCacheException[[huggingface_hub.CorruptedCacheException]]

#### huggingface_hub.CorruptedCacheException[[huggingface_hub.CorruptedCacheException]]

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/errors.py#L22)

Exception for any unexpected structure in the Huggingface cache-system.
