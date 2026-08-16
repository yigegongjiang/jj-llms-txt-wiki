# Downloading files

## Download a single file

### hf_hub_download[[huggingface_hub.hf_hub_download]]

#### huggingface_hub.hf_hub_download[[huggingface_hub.hf_hub_download]]

```python
huggingface_hub.hf_hub_download(repo_id: str, filename: str, subfolder: str | None = None, repo_type: str | None = None, revision: str | None = None, library_name: str | None = None, library_version: str | None = None, cache_dir: str | pathlib.Path | None = None, local_dir: str | pathlib.Path | None = None, user_agent: dict | str | None = None, force_download: bool = False, etag_timeout: float = 10, token: bool | str | None = None, local_files_only: bool = False, headers: dict[str, str] | None = None, endpoint: str | None = None, tqdm_class: type[tqdm.asyncio.tqdm_asyncio] | None = None, dry_run: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/file_download.py#L828)

**Parameters:**

repo_id (`str`) : A user or an organization name and a repo name separated by a `/`.

filename (`str`) : The name of the file in the repo.

subfolder (`str`, *optional*) : An optional value corresponding to a folder inside the model repo.

repo_type (`str`, *optional*) : Set to `"dataset"`, `"space"` or `"kernel"` if downloading from a dataset, space or kernel repo, `None` or `"model"` if downloading from a model. Default is `None`.

revision (`str`, *optional*) : An optional Git revision id which can be a branch name, a tag, or a commit hash.

library_name (`str`, *optional*) : The name of the library to which the object corresponds.

library_version (`str`, *optional*) : The version of the library.

cache_dir (`str`, `Path`, *optional*) : Path to the folder where cached files are stored.

local_dir (`str` or `Path`, *optional*) : If provided, the downloaded file will be placed under this directory.

user_agent (`dict`, `str`, *optional*) : The user-agent info in the form of a dictionary or a string.

force_download (`bool`, *optional*, defaults to `False`) : Whether the file should be downloaded even if it already exists in the local cache.

etag_timeout (`float`, *optional*, defaults to `10`) : When fetching ETag, how many seconds to wait for the server to send data before giving up, which is passed to `httpx.request`.

token (`str`, `bool`, *optional*) : A token to be used for the download. - If `True`, the token is read from the HuggingFace config folder. - If a string, it's used as the authentication token.

local_files_only (`bool`, *optional*, defaults to `False`) : If `True`, avoid downloading the file and return the path to the local cached file if it exists.

headers (`dict`, *optional*) : Additional headers to be sent with the request.

endpoint (`str`, *optional*) : The Hub endpoint to send the request to. Defaults to the value of `HF_ENDPOINT`.

tqdm_class (`tqdm`, *optional*) : If provided, overwrites the default behavior for the progress bar. Passed argument must inherit from `tqdm.auto.tqdm` or at least mimic its behavior. Defaults to the custom HF progress bar that can be disabled by setting `HF_HUB_DISABLE_PROGRESS_BARS` environment variable.

dry_run (`bool`, *optional*, defaults to `False`) : If `True`, perform a dry run without actually downloading the file. Returns a [DryRunFileInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.DryRunFileInfo) object containing information about what would be downloaded.

**Returns:** `str` or [DryRunFileInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.DryRunFileInfo)

- If `dry_run=False`: Local path of file or if networking is off, last version of file cached on disk.
- If `dry_run=True`: A [DryRunFileInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.DryRunFileInfo) object containing download information.

**Raises:** [RepositoryNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.RepositoryNotFoundError) or [RevisionNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.RevisionNotFoundError) or `~utils.RemoteEntryNotFoundError` or [LocalEntryNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.LocalEntryNotFoundError) or ``EnvironmentError`` or ``OSError`` or ``ValueError``

- [RepositoryNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.RepositoryNotFoundError) -- 
  If the repository to download from cannot be found. This may be because it doesn't exist,
  or because it is set to `private` and you do not have access.
- [RevisionNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.RevisionNotFoundError) -- 
  If the revision to download from cannot be found.
- `~utils.RemoteEntryNotFoundError` -- 
  If the file to download cannot be found.
- [LocalEntryNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.LocalEntryNotFoundError) -- 
  If network is disabled or unavailable and file is not found in cache.
- [`EnvironmentError`](https://docs.python.org/3/library/exceptions.html#EnvironmentError) -- 
  If `token=True` but the token cannot be found.
- [`OSError`](https://docs.python.org/3/library/exceptions.html#OSError) -- 
  If ETag cannot be determined.
- [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError) -- 
  If some parameter value is invalid.

Download a given file if it's not already present in the local cache.

The new cache file layout looks like this:
- The cache directory contains one subfolder per repo_id (namespaced by repo type)
- inside each repo folder:
  - refs is a list of the latest known revision => commit_hash pairs
  - blobs contains the actual file blobs (identified by their git-sha or sha256, depending on
    whether they're LFS files or not)
  - snapshots contains one subfolder per commit, each "commit" contains the subset of the files
    that have been resolved at that particular commit. Each filename is a symlink to the blob
    at that particular commit.

```
[  96]  .
└── [ 160]  models--julien-c--EsperBERTo-small
    ├── [ 160]  blobs
    │   ├── [321M]  403450e234d65943a7dcf7e05a771ce3c92faa84dd07db4ac20f592037a1e4bd
    │   ├── [ 398]  7cb18dc9bafbfcf74629a4b760af1b160957a83e
    │   └── [1.4K]  d7edf6bd2a681fb0175f7735299831ee1b22b812
    ├── [  96]  refs
    │   └── [  40]  main
    └── [ 128]  snapshots
        ├── [ 128]  2439f60ef33a0d46d85da5001d52aeda5b00ce9f
        │   ├── [  52]  README.md -> ../../blobs/d7edf6bd2a681fb0175f7735299831ee1b22b812
        │   └── [  76]  pytorch_model.bin -> ../../blobs/403450e234d65943a7dcf7e05a771ce3c92faa84dd07db4ac20f592037a1e4bd
        └── [ 128]  bbc77c8132af1cc5cf678da3f1ddf2de43606d48
            ├── [  52]  README.md -> ../../blobs/7cb18dc9bafbfcf74629a4b760af1b160957a83e
            └── [  76]  pytorch_model.bin -> ../../blobs/403450e234d65943a7dcf7e05a771ce3c92faa84dd07db4ac20f592037a1e4bd
```

If `local_dir` is provided, the file structure from the repo will be replicated in this location. When using this
option, the `cache_dir` will not be used and a `.cache/huggingface/` folder will be created at the root of `local_dir`
to store some metadata related to the downloaded files. While this mechanism is not as robust as the main
cache-system, it's optimized for regularly pulling the latest version of a repository.

### hf_hub_url[[huggingface_hub.hf_hub_url]]

#### huggingface_hub.hf_hub_url[[huggingface_hub.hf_hub_url]]

```python
huggingface_hub.hf_hub_url(repo_id: str, filename: str, subfolder: str | None = None, repo_type: str | None = None, revision: str | None = None, endpoint: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/file_download.py#L201)

**Parameters:**

repo_id (`str`) : A namespace (user or an organization) name and a repo name separated by a `/`.

filename (`str`) : The name of the file in the repo.

subfolder (`str`, *optional*) : An optional value corresponding to a folder inside the repo.

repo_type (`str`, *optional*) : Set to `"dataset"`, `"space"` or `"kernel"` if downloading from a dataset, space or kernel repo, `None` or `"model"` if downloading from a model. Default is `None`.

revision (`str`, *optional*) : An optional Git revision id which can be a branch name, a tag, or a commit hash.

endpoint (`str`, *optional*) : The Hub endpoint to send the request to. Defaults to the value of `HF_ENDPOINT`.

Construct the URL of a file from the given information.

The resolved address can either be a huggingface.co-hosted url, or a link to
Cloudfront (a Content Delivery Network, or CDN) for large files which are
more than a few MBs.

Example:

```python
>>> from huggingface_hub import hf_hub_url

>>> hf_hub_url(
...     repo_id="julien-c/EsperBERTo-small", filename="pytorch_model.bin"
... )
'https://huggingface.co/julien-c/EsperBERTo-small/resolve/main/pytorch_model.bin'
```

> [!TIP]
> Notes:
>
>     Cloudfront is replicated over the globe so downloads are way faster for
>     the end user (and it also lowers our bandwidth costs).
>
>     Cloudfront aggressively caches files by default (default TTL is 24
>     hours), however this is not an issue here because we implement a
>     git-based versioning system on huggingface.co, which means that we store
>     the files on S3/Cloudfront in a content-addressable way (i.e., the file
>     name is its hash). Using content-addressable filenames means cache can't
>     ever be stale.
>
>     In terms of client-side caching from this library, we base our caching
>     on the objects' entity tag (`ETag`), which is an identifier of a
>     specific version of a resource [1]_. An object's ETag is: its git-sha1
>     if stored in git, or its sha256 if stored in git-lfs.

References:

-  [1] https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag

## Download a snapshot of the repo[[huggingface_hub.snapshot_download]]

#### huggingface_hub.snapshot_download[[huggingface_hub.snapshot_download]]

```python
huggingface_hub.snapshot_download(repo_id: str, repo_type: str | None = None, revision: str | None = None, cache_dir: str | pathlib.Path | None = None, local_dir: str | pathlib.Path | None = None, library_name: str | None = None, library_version: str | None = None, user_agent: dict | str | None = None, etag_timeout: float = 10, force_download: bool = False, token: bool | str | None = None, local_files_only: bool = False, allow_patterns: list[str] | str | None = None, ignore_patterns: list[str] | str | None = None, max_workers: int = 8, tqdm_class: type[tqdm.asyncio.tqdm_asyncio] | None = None, headers: dict[str, str] | None = None, endpoint: str | None = None, dry_run: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_snapshot_download.py#L113)

**Parameters:**

repo_id (`str`) : A user or an organization name and a repo name separated by a `/`.

repo_type (`str`, *optional*) : Set to `"dataset"`, `"space"` or `"kernel"` if downloading from a dataset, space or kernel repo, `None` or `"model"` if downloading from a model. Default is `None`.

revision (`str`, *optional*) : An optional Git revision id, which can be a branch name, a tag, or a commit hash.

cache_dir (`str`, `Path`, *optional*) : Path to the folder where cached files are stored.

local_dir (`str` or `Path`, *optional*) : If provided, the downloaded files will be placed under this directory.

library_name (`str`, *optional*) : The name of the library to which the object corresponds.

library_version (`str`, *optional*) : The version of the library.

user_agent (`str`, `dict`, *optional*) : The user-agent info in the form of a dictionary or a string.

etag_timeout (`float`, *optional*, defaults to `10`) : When fetching ETag, how many seconds to wait for the server to send data before giving up, which is passed to `httpx.request`.

force_download (`bool`, *optional*, defaults to `False`) : Whether the file should be downloaded even if it already exists in the local cache.

token (`str`, `bool`, *optional*) : A token to be used for the download. - If `True`, the token is read from the HuggingFace config folder. - If a string, it's used as the authentication token.

headers (`dict`, *optional*) : Additional headers to include in the request. Those headers take precedence over the others.

endpoint (`str`, *optional*) : The Hub endpoint to send the request to. Defaults to the value of `HF_ENDPOINT`.

local_files_only (`bool`, *optional*, defaults to `False`) : If `True`, do not download any files even if they are not in `cache_dir` or `local_dir`.

allow_patterns (`list[str]` or `str`, *optional*) : If provided, only files matching at least one pattern are downloaded.

ignore_patterns (`list[str]` or `str`, *optional*) : If provided, files matching any of the patterns are not downloaded.

max_workers (`int`, *optional*) : Number of concurrent threads to download files (1 thread = 1 file download). Defaults to 8.

tqdm_class (`tqdm`, *optional*) : If provided, overwrites the default behavior for the progress bar. Passed argument must inherit from `tqdm.auto.tqdm` or at least mimic its behavior. Note that the `tqdm_class` is not passed to each individual download. Defaults to the custom HF progress bar that can be disabled by setting `HF_HUB_DISABLE_PROGRESS_BARS` environment variable.

dry_run (`bool`, *optional*, defaults to `False`) : If `True`, perform a dry run without actually downloading the files. Returns a list of [DryRunFileInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.DryRunFileInfo) objects containing information about what would be downloaded.

**Returns:** `str` or list of [DryRunFileInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.DryRunFileInfo)

- If `dry_run=False`: Local snapshot path.
- If `dry_run=True`: A list of [DryRunFileInfo](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.DryRunFileInfo) objects containing download information.

**Raises:** [RepositoryNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.RepositoryNotFoundError) or [RevisionNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.RevisionNotFoundError) or [IncompleteSnapshotError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.IncompleteSnapshotError) or ``EnvironmentError`` or ``OSError`` or ``ValueError``

- [RepositoryNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.RepositoryNotFoundError) -- 
  If the repository to download from cannot be found. This may be because it doesn't exist
  or because it is set to `private` and you do not have access.
- [RevisionNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.RevisionNotFoundError) -- 
  If the revision to download from cannot be found.
- [IncompleteSnapshotError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.IncompleteSnapshotError) -- 
  If the Hub cannot be reached (offline, connection issue, or `local_files_only=True`) and the
  cached snapshot is missing some of the requested files.
- [`EnvironmentError`](https://docs.python.org/3/library/exceptions.html#EnvironmentError) -- 
  If `token=True` and the token cannot be found.
- [`OSError`](https://docs.python.org/3/library/exceptions.html#OSError) -- if
  ETag cannot be determined.
- [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError) -- 
  If some parameter value is invalid.

Download repo files.

Download a whole snapshot of a repo's files at the specified revision. This is useful when you want all files from
a repo because you don't know which ones you will need _a priori_. All files are nested in a folder to keep their
path and filename relative to that folder. You can also filter which files to download by using `allow_patterns`
and `ignore_patterns`.

If `local_dir` is provided, the file structure from the repo will be replicated in this location. When using this
option, the `cache_dir` will not be used, and a `.cache/huggingface/` folder will be created at the root of `local_dir`
to store some metadata related to the downloaded files. While this mechanism is not as robust as the main
cache system, it's optimized for regularly pulling the latest version of a repository.

An alternative would be to clone the repo, but this requires git and git-lfs to be installed and properly
configured. It is also not possible to filter which files to download when cloning a repository using git.

## Read the cached repo tree[[huggingface_hub.get_cached_repo_tree]]

#### huggingface_hub.get_cached_repo_tree[[huggingface_hub.get_cached_repo_tree]]

```python
huggingface_hub.get_cached_repo_tree(repo_id: str, repo_type: str | None = None, revision: str | None = None, cache_dir: str | pathlib.Path | None = None, local_dir: str | pathlib.Path | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_snapshot_download.py#L584)

**Parameters:**

repo_id (`str`) : A user or an organization name and a repo name separated by a `/`.

repo_type (`str`, *optional*) : Set to `"dataset"`, `"space"` or `"kernel"` if listing from a dataset, space or kernel repo, `None` or `"model"` if listing from a model. Default is `None`.

revision (`str`, *optional*) : An optional Git revision id, which can be a branch name, a tag, or a commit hash. Defaults to the default branch. Branch/tag names are resolved to a commit hash using the local cache (`refs/`).

cache_dir (`str`, `Path`, *optional*) : Path to the folder where cached files are stored. Defaults to the value of `HF_HUB_CACHE`.

local_dir (`str` or `Path`, *optional*) : If provided, read the tree listing cached by a `local_dir` download (from `local_dir/.cache/huggingface/`) instead of the main cache. Branch/tag revisions are still resolved to a commit hash using the main cache (`cache_dir`).

**Returns:** `list[RepoFile]`

The list of [RepoFile](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.RepoFile) objects cached for this revision.

**Raises:** [CachedRepoTreeNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.CachedRepoTreeNotFoundError)

- [CachedRepoTreeNotFoundError](/docs/huggingface_hub/v1.27.0/en/package_reference/utilities#huggingface_hub.errors.CachedRepoTreeNotFoundError) -- 
  If no tree listing is cached for the requested revision (e.g. the repo was never downloaded at this revision).

Return the cached tree listing of a repo at a given revision, without any network call.

The tree listing is the set of files (with their download metadata) of a repo at a commit. It is populated
on disk as a side effect of [snapshot_download()](/docs/huggingface_hub/v1.27.0/en/package_reference/file_download#huggingface_hub.snapshot_download) (see the `trees/<commit_hash>.json` cache files) and is
used to skip network calls on subsequent downloads. This function exposes that cache directly.

If you need the current tree listing of a repo on the Hub, use [list_repo_tree()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.list_repo_tree) instead.

Example:
```py
>>> from huggingface_hub import get_cached_repo_tree
>>> files = get_cached_repo_tree("openai-community/gpt2")
>>> [f.path for f in files]
['.gitattributes', 'config.json', 'model.safetensors', ...]
```

## Resolve a revision

Resolve a branch/tag name to a commit hash once, then pass the result around to pin every download to the same commit. See [HfApi.resolve_revision()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.resolve_revision) and the [cache-system guide](../guides/manage-cache#pin-a-revision-advanced).

### ResolvedRevision[[huggingface_hub.ResolvedRevision]]

#### huggingface_hub.ResolvedRevision[[huggingface_hub.ResolvedRevision]]

```python
huggingface_hub.ResolvedRevision(resolved: str, initial: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/_revision.py#L4)

**Parameters:**

initial (`str` or `None`) : The revision initially requested by the user. If `None`, the string value defaults to `"main"`.

resolved (`str`) : The commit hash that `initial` resolves to.

A git revision that has already been resolved to a commit hash.

`ResolvedRevision` is a `str` subclass, so it can be passed to any `huggingface_hub` method taking a `revision`
argument. Its string value is the revision initially requested by the user (e.g. `"main"`, `"refs/pr/4"`),
which keeps URLs and error messages readable, while `.resolved` holds the commit hash it points to.

Instances are built by [HfApi.resolve_revision()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.resolve_revision), which also caches the `revision` -> `commit hash` mapping
in the local cache (`refs/` folder).

Example:
```python
>>> from huggingface_hub import resolve_revision
>>> revision = resolve_revision("openai-community/gpt2")
>>> revision
ResolvedRevision(initial=None, resolved='607a30d783dfa663caf39e06633721c8d4cfcd7e')
>>> revision == "main"  # it's a string
True
>>> revision.resolved
'607a30d783dfa663caf39e06633721c8d4cfcd7e'
```

## Get metadata about a file

### get_hf_file_metadata[[huggingface_hub.get_hf_file_metadata]]

#### huggingface_hub.get_hf_file_metadata[[huggingface_hub.get_hf_file_metadata]]

```python
huggingface_hub.get_hf_file_metadata(url: str, token: bool | str | None = None, timeout: float | None = 10, library_name: str | None = None, library_version: str | None = None, user_agent: dict | str | None = None, headers: dict[str, str] | None = None, endpoint: str | None = None, retry_on_errors: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/file_download.py#L1568)

**Parameters:**

url (`str`) : File url, for example returned by [hf_hub_url()](/docs/huggingface_hub/v1.27.0/en/package_reference/file_download#huggingface_hub.hf_hub_url).

token (`str` or `bool`, *optional*) : A token to be used for the download. - If `True`, the token is read from the HuggingFace config folder. - If `False` or `None`, no token is provided. - If a string, it's used as the authentication token.

timeout (`float`, *optional*, defaults to 10) : How many seconds to wait for the server to send metadata before giving up.

library_name (`str`, *optional*) : The name of the library to which the object corresponds.

library_version (`str`, *optional*) : The version of the library.

user_agent (`dict`, `str`, *optional*) : The user-agent info in the form of a dictionary or a string.

headers (`dict`, *optional*) : Additional headers to be sent with the request.

endpoint (`str`, *optional*) : Endpoint of the Hub. Defaults to .

retry_on_errors (`bool`, *optional*, defaults to `False`) : Whether to retry on errors (429, 5xx, timeout, network errors). If False, no retry for fast fallback to local cache.

**Returns:**

A [HfFileMetadata](/docs/huggingface_hub/v1.27.0/en/package_reference/file_download#huggingface_hub.HfFileMetadata) object containing metadata such as location, etag, size and
commit_hash.

Fetch metadata of a file versioned on the Hub for a given url.

### HfFileMetadata[[huggingface_hub.HfFileMetadata]]

#### huggingface_hub.HfFileMetadata[[huggingface_hub.HfFileMetadata]]

```python
huggingface_hub.HfFileMetadata(commit_hash: str | None, etag: str | None, location: str, size: int | None, xet_file_data: huggingface_hub.utils._xet.XetFileData | None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/file_download.py#L147)

**Parameters:**

commit_hash (`str`, *optional*) : The commit_hash related to the file.

etag (`str`, *optional*) : Etag of the file on the server.

location (`str`) : Location where to download the file. Can be a Hub url or not (CDN).

size (`size`) : Size of the file. In case of an LFS file, contains the size of the actual LFS file, not the pointer.

xet_file_data (`XetFileData`, *optional*) : Xet information for the file. This is only set if the file is stored using Xet storage.

Data structure containing information about a file versioned on the Hub.

Returned by [get_hf_file_metadata()](/docs/huggingface_hub/v1.27.0/en/package_reference/file_download#huggingface_hub.get_hf_file_metadata) based on a URL.

## Caching

The methods displayed above are designed to work with a caching system that prevents
re-downloading files. The caching system was updated in v0.8.0 to become the central
cache-system shared across libraries that depend on the Hub.

Read the [cache-system guide](../guides/manage-cache) for a detailed presentation of caching at HF.
