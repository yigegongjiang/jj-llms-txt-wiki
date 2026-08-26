# API Reference

## Run[[trackio.Run]]

#### trackio.Run[[trackio.Run]]

```python
trackio.Run(url: str | None, project: str, client: typing.Optional[typing.Any], name: str | None = None, run_id: str | None = None, group: str | None = None, config: dict | None = None, space_id: str | None = None, bucket_id: str | None = None, server_base_url: str | None = None, write_token: str | None = None, existing_runs: list[str] | None = None, initial_last_step: int | None = None, auto_log_gpu: bool = False, gpu_log_interval: float = 10.0, auto_log_cpu: bool = False, cpu_log_interval: float = 10.0, webhook_url: str | None = None, webhook_min_level: trackio.alerts.AlertLevel | str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/run.py#L43)

#### link_artifact[[trackio.Run.link_artifact]]

```python
link_artifact(artifact: Artifact, target_path: str, aliases: list[str] | None = None, bucket_id: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/run.py#L1467)

**Parameters:**

artifact (`Artifact`) : The artifact version to link. Must be an `Artifact` that has been logged or fetched. To link a specific existing version, fetch it first with `use_artifact("my-model:v3")` and link the result. An artifact that was never logged is logged to this run's project first.

target_path (`str`) : The collection to link into, as `"registry-<registry>/<collection>"`, e.g. `"registry-models/my-model"`. The registry must already exist (see `trackio.Api().create_registry`). The collection is created on first link and adopts the artifact's type; later links must match it.

aliases (`list[str]`, *optional*) : Aliases to place on the linked version, e.g. `["staging"]`. An alias points at one version per collection, so assigning an alias that another version holds moves it. `latest` is managed automatically and always follows the newest linked version; passing it here is a no-op.

bucket_id (`str`, *optional*) : Hugging Face bucket holding the registry, e.g. `"my-org/models-registry"`. Required to link from a run that logs to a Space or a self-hosted server. Omit for a local registry; defaults to `TRACKIO_REGISTRY_BUCKET_ID` when that is set.

**Returns:**

The linked artifact at its registry location. Its `name` is the
collection, its `project` is the registry's project, and its
`version` and `aliases` are the collection's. Its content
(`manifest`, `manifest_digest`, `size`, `metadata`) is the
source version's, and the `source_project`, `source_name`,
`source_version`, and `source_qualified_name` properties point
back at it. Calling `download()` on it is not supported yet;
download the source version instead.

Link an artifact version into a registry collection.

A link is a pointer to the source version: no files are copied. The
link gets the next version number in the collection (`v0`, `v1`,
...), independent of the source artifact's own version numbers.
Linking a version that is already in the collection does not create
a new one: you get the existing version back, and the `aliases` you
pass still move.

A local registry records the link alongside the run's data, so it is
only available to runs on this machine. Pass `bucket_id` to publish
into a bucket-backed registry instead — that is also how a run that
logs to a Space or a self-hosted server publishes, since the bucket is
reachable from both sides.

#### use_artifact[[trackio.Run.use_artifact]]

```python
use_artifact(artifact_or_name: trackio.artifact.Artifact | str, type: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/run.py#L1380)

Resolve an artifact and record this run as a consumer of it.

## init[[trackio.init]]

#### trackio.init[[trackio.init]]

```python
trackio.init(project: str, name: str | None = None, group: str | None = None, space_id: str | None = None, server_url: str | None = None, space_storage: huggingface_hub._space_api.SpaceStorage | None = None, dataset_id: str | None = None, bucket_id: str | None = None, config: dict | None = None, resume: str = 'never', settings: typing.Any = None, private: bool | None = None, embed: bool = True, auto_log_gpu: bool | None = None, gpu_log_interval: float = 10.0, auto_log_cpu: bool | None = None, cpu_log_interval: float = 10.0, webhook_url: str | None = None, webhook_min_level: trackio.alerts.AlertLevel | str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/__init__.py#L241)

**Parameters:**

project (*str*) : The name of the project (can be an existing project to continue tracking or a new project to start tracking from scratch).

name (*str*, *optional*) : The name of the run (if not provided, a default name will be generated).

group (*str*, *optional*) : The name of the group which this run belongs to in order to help organize related runs together. You can toggle the entire group's visibility in the dashboard.

space_id (*str*, *optional*) : If provided, the project will be logged to a Hugging Face Space instead of a local directory. Should be a complete Space name like *"username/reponame"* or *"orgname/reponame"*, or just *"reponame"* in which case the Space will be created in the currently-logged-in Hugging Face user's namespace. If the Space does not exist, it will be created. If the Space already exists, the project will be logged to it. Can also be set via the *TRACKIO_SPACE_ID* environment variable. You cannot log to a Space that has been **frozen** (converted to the static SDK); use `trackio.sync(..., sdk="static")` only after you are done logging. Takes precedence over *server_url* and *TRACKIO_SERVER_URL* when more than one is set.

server_url (*str*, *optional*) : Base URL of a self-hosted Trackio server (`http://` or `https://`), or the write-access URL from `trackio.show()` which may include a `write_token` query parameter. The client sends that token on each request (`X-Trackio-Write-Token`); you can also set `TRACKIO_WRITE_TOKEN` instead of embedding the token in the URL. When set, metrics are sent to that server over HTTP instead of creating or syncing to a Hugging Face Space. Can also be set via the `TRACKIO_SERVER_URL` environment variable. Ignored when `space_id` or `TRACKIO_SPACE_ID` is set.

space_storage ([*~huggingface_hub.SpaceStorage*], *optional*) : Choice of persistent storage tier.

dataset_id (*str*, *optional*) : Deprecated: persisting trackio data to a Hugging Face Dataset will be removed in a future version of trackio. Use *bucket_id* (a Hugging Face Bucket) instead.

bucket_id (*str*, *optional*) : The ID of the Hugging Face Bucket to use for metric persistence. By default, when a *space_id* is provided and *bucket_id* is not explicitly set, a bucket is auto-generated from the space_id. Buckets provide S3-like storage without git overhead - the SQLite database is stored directly via *hf-mount* in the Space. Specify a Bucket with name like *"username/bucketname"* or just *"bucketname"*.

config (*dict*, *optional*) : A dictionary of configuration options. Provided for compatibility with *wandb.init()*.

resume (*str*, *optional*, defaults to *"never"*) : Controls how to handle resuming a run. Can be one of:  - *"must"*: Must resume the run with the given name, raises error if run doesn't exist - *"allow"*: Resume the run if it exists, otherwise create a new run - *"never"*: Never resume a run, always create a new one

private (*bool*, *optional*) : Whether to make the Space private. If None (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

settings (*Any*, *optional*) : Not used. Provided for compatibility with *wandb.init()*.

embed (*bool*, *optional*, defaults to *True*) : If running inside a Jupyter/Colab notebook, whether the dashboard should automatically be embedded in the cell when trackio.init() is called. For local runs, this launches a local Trackio dashboard and embeds it. For Space runs, this embeds the Space URL. In Colab, the local dashboard will be accessible via a public share URL when *share=True*.

auto_log_gpu (*bool* or *None*, *optional*, defaults to *None*) : Controls automatic GPU metrics logging. If *None* (default), GPU logging is automatically enabled when *nvidia-ml-py* is installed and an NVIDIA GPU or Apple M series is detected. Set to *True* to force enable or *False* to disable.

gpu_log_interval (*float*, *optional*, defaults to *10.0*) : The interval in seconds between automatic GPU metric logs. Only used when *auto_log_gpu=True*.

auto_log_cpu (*bool* or *None*, *optional*, defaults to *None*) : Controls automatic CPU and RAM metrics logging (utilization, memory, disk I/O, network I/O, and sensors). If *None* (default), CPU logging is automatically enabled when *psutil* is installed. Set to *True* to force enable or *False* to disable.

cpu_log_interval (*float*, *optional*, defaults to *10.0*) : The interval in seconds between automatic CPU metric logs. Only used when CPU auto-logging is enabled.

webhook_url (*str*, *optional*) : A webhook URL to POST alert payloads to when *trackio.alert()* is called. Supports Slack and Discord webhook URLs natively (payloads are formatted automatically). Can also be set via the *TRACKIO_WEBHOOK_URL* environment variable. Individual alerts can override this URL by passing *webhook_url* to *trackio.alert()*.

webhook_min_level (*AlertLevel* or *str*, *optional*) : Minimum alert level that should trigger webhook delivery. For example, *AlertLevel.WARN* sends only *WARN* and *ERROR* alerts to the webhook destination. Can also be set via *TRACKIO_WEBHOOK_MIN_LEVEL*.

**Returns:** `*Run*`

A [*Run*] object that can be used to log metrics and finish the run.

Creates a new Trackio project and returns a [*Run*] object.

## log[[trackio.log]]

#### trackio.log[[trackio.log]]

```python
trackio.log(metrics: dict, step: int | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/__init__.py#L687)

**Parameters:**

metrics (`dict`) : A dictionary of metrics to log.

step (`int`, *optional*) : The step number. If not provided, the step will be incremented automatically.

Logs metrics to the current run.

## log_gpu[[trackio.log_gpu]]

#### trackio.log_gpu[[trackio.log_gpu]]

```python
trackio.log_gpu(run: trackio.run.Run | None = None, device: int | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/__init__.py#L783)

**Parameters:**

run : Optional Run instance. If None, uses current run from context.

device : CUDA device index to collect metrics from (NVIDIA GPUs only). If None, collects from all GPUs visible to this process. This parameter is ignored for Apple GPUs.

**Returns:** `dict`

The GPU metrics that were logged.

Log GPU metrics to the current or specified run as system metrics.
Automatically detects whether an NVIDIA or Apple GPU is available and calls
the appropriate logging method.

Example:
```python
import trackio

run = trackio.init(project="my-project")
trackio.log({"loss": 0.5})
trackio.log_gpu()
trackio.log_gpu(device=0)
```

## log_system[[trackio.log_system]]

#### trackio.log_system[[trackio.log_system]]

```python
trackio.log_system(metrics: dict)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/__init__.py#L707)

**Parameters:**

metrics (`dict`) : A dictionary of system metrics to log.

Logs system metrics (GPU, etc.) to the current run using timestamps instead of steps.

## log_artifact[[trackio.log_artifact]]

#### trackio.log_artifact[[trackio.log_artifact]]

```python
trackio.log_artifact(artifact_or_path: trackio.artifact.Artifact | str | pathlib.Path, name: str | None = None, type: str | None = None, aliases: list[str] | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/__init__.py#L721)

**Parameters:**

artifact_or_path (`Artifact`, `str`, or `Path`) : The artifact to log (must have at least one file added via `add_file` or `add_dir`), or a path to a file or directory to log as a new artifact.

name (`str`, *optional*) : Artifact name when logging a path. Defaults to `run-<run_id>-<basename>`. Must not be passed with an `Artifact`.

type (`str`, *optional*) : Artifact type when logging a path (e.g. `"model"`, `"dataset"`). Defaults to `"unspecified"`. Must not be passed with an `Artifact`.

aliases (`list[str]`, *optional*) : Aliases to rotate onto the resulting version, alongside `latest` (assigned automatically whenever a new version is created). Your aliases rotate onto the version even when identical content is de-duplicated.

**Returns:**

The logged `Artifact` instance, hydrated with `version`, `aliases`,
`size`, `manifest`, and `project` set.

Logs an artifact as an output of the current run.

## use_artifact[[trackio.use_artifact]]

#### trackio.use_artifact[[trackio.use_artifact]]

```python
trackio.use_artifact(artifact_or_name: trackio.artifact.Artifact | str, type: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/__init__.py#L758)

**Parameters:**

artifact_or_name (`Artifact` or `str`) : An already-logged `Artifact`, or an artifact name. A bare name (`"my-model"`) resolves to `:latest`; you can also pin a version (`"my-model:v3"`) or resolve an alias (`"my-model:prod"`).

type (`str`, *optional*) : If given, checked against the stored artifact type, raising if it does not match.

**Returns:**

The fetched `Artifact`, hydrated and ready to `download()`.

Fetches an artifact and records it as an input to the current run.

## finish[[trackio.finish]]

#### trackio.finish[[trackio.finish]]

```python
trackio.finish()
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/__init__.py#L853)

Finishes the current run.

## show[[trackio.show]]

#### trackio.show[[trackio.show]]

```python
trackio.show(project: str | None = None, dashboard: str | None = None, theme: typing.Any = None, mcp_server: bool | None = None, footer: bool = True, color_palette: list[str] | None = None, open_browser: bool = True, block_thread: bool | None = None, host: str | None = None, share: bool | None = None, server_port: int | None = None, frontend_dir: str | pathlib.Path | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/__init__.py#L1105)

**Parameters:**

project (`str`, *optional*) : The name of the project whose runs to show. If not provided, all projects will be shown and the user can select one.

dashboard (`str`, *optional*) : Set to `"registry"` to show the standalone registry dashboard. The default is the project dashboard.

theme (`Any`, *optional*) : Ignored. Kept for backward compatibility; Trackio no longer uses Gradio themes.

mcp_server (`bool`, *optional*) : If `True`, the dashboard exposes an MCP server at `/mcp` when the optional `trackio[mcp]` dependency is installed. If `None` (default), the `GRADIO_MCP_SERVER` environment variable is used (e.g. on Spaces).

footer (`bool`, *optional*, defaults to `True`) : Whether to include `footer=false` in the write-token URL when `False`. This can also be controlled via the `footer` query parameter in the URL.

color_palette (`list[str]`, *optional*) : A list of hex color codes to use for plot lines. If not provided, the `TRACKIO_COLOR_PALETTE` environment variable will be used (comma-separated hex codes), or if that is not set, the default color palette will be used. Example: `['#FF0000', '#00FF00', '#0000FF']`

open_browser (`bool`, *optional*, defaults to `True`) : If `True` and not in a notebook, a new browser tab will be opened with the dashboard. If `False`, the browser will not be opened.

block_thread (`bool`, *optional*) : If `True`, the main thread will be blocked until the dashboard is closed. If `None` (default behavior), then the main thread will not be blocked if the dashboard is launched in a notebook, otherwise the main thread will be blocked.

host (`str`, *optional*) : The host to bind the server to. If not provided, defaults to `'127.0.0.1'` (localhost only). Set to `'0.0.0.0'` to allow remote access.

share (`bool`, *optional*) : If `True`, creates a temporary public URL (Gradio-compatible tunnel). On Colab or hosted notebooks, defaults to `True` unless overridden.

server_port (`int`, *optional*) : Port to bind. If not set, scans from `GRADIO_SERVER_PORT` (default 7860).

frontend_dir (`str | Path`, *optional*) : Directory containing a custom static frontend. Must contain `index.html`. If not provided, Trackio checks `TRACKIO_FRONTEND_DIR`, then the persistent Trackio config, then the bundled frontend. If an explicit `frontend_dir` points to a missing or empty directory, Trackio copies in the starter template and serves that directory.

**Returns:** `app`

The dashboard handle (`.close()` stops the server).
`url`: The local URL of the dashboard.
`share_url`: The public share URL, if any.
`full_url`: The full URL including the write token (share URL when sharing, else local).

Launches the Trackio dashboard.

## sync[[trackio.sync]]

#### trackio.sync[[trackio.sync]]

```python
trackio.sync(project: str, space_id: str | None = None, private: bool | None = None, force: bool = False, run_in_background: bool = False, sdk: str = 'gradio', dataset_id: str | None = None, bucket_id: str | None = None, frontend_dir: str | pathlib.Path | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/deploy.py#L1075)

**Parameters:**

project (*str*) : The name of the project to upload.

space_id (*str*, *optional*) : The ID of the Space to upload to (e.g., *"username/space_id"*). If not provided, checks project metadata first, then generates a random space_id.

private (*bool*, *optional*) : Whether to make the Space private. If None (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists. Not supported with `sdk="static"` because static Trackio dashboards read snapshot data directly from the browser.

force (*bool*, *optional*, defaults to *False*) : If *True*, overwrite the existing database without prompting for confirmation. If *False*, prompt the user before overwriting an existing database.

run_in_background (*bool*, *optional*, defaults to *False*) : If *True*, the Space creation and database upload will be run in a background thread. If *False*, all the steps will be run synchronously.

sdk (*str*, *optional*, defaults to *"gradio"*) : The type of Space to deploy. *"gradio"* deploys a Gradio Space with a live server. *"static"* freezes the Space: deploys a static Space that reads from an HF Bucket (no server needed).

dataset_id (*str*, *optional*) : Deprecated: persisting trackio data to a Hugging Face Dataset will be removed in a future version of trackio. Use *bucket_id* (a Hugging Face Bucket) instead.

bucket_id (*str*, *optional*) : The ID of the HF Bucket to sync to. By default, a bucket is auto-generated from the space_id.

**Returns:** `*str*`

The Space ID of the synced project.

Syncs a local Trackio project's database to a Hugging Face Space.
If the Space does not exist, it will be created. Local data is never deleted.

**Freezing:** Passing `sdk="static"` deploys a static Space backed by an HF Bucket
(read-only dashboard, no Gradio server). You can sync the same project again later to
refresh that static Space. If you want a one-time snapshot of an existing Gradio Space,
use `freeze()` instead.

## freeze[[trackio.freeze]]

#### trackio.freeze[[trackio.freeze]]

```python
trackio.freeze(space_id: str, project: str, new_space_id: str | None = None, private: bool | None = None, bucket_id: str | None = None, frontend_dir: str | pathlib.Path | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/deploy.py#L1230)

**Parameters:**

space_id (`str`) : The ID of the source Gradio Space (e.g., `"username/my-space"` or a short repo name with the logged-in namespace inferred, like `init()`). Must be a Gradio Space with a bucket mounted at `/data`.

project (`str`) : The name of the project whose data to include in the frozen Space.

new_space_id (`str`, *optional*) : The ID for the new static Space. If not provided, defaults to `"{space_id}_static"`.

private (`bool`, *optional*) : Not supported. Frozen static dashboards read snapshot data directly from the browser, so the destination snapshot must be public.

bucket_id (`str`, *optional*) : The ID of the HF Bucket for the new static Space's data storage. If not provided, one is auto-generated from the new Space ID.

**Returns:** `str`

The Space ID of the newly created static Space.

Creates a new static Hugging Face Space containing a read-only snapshot of
the data for the specified project from the source Gradio Space. The data is
read from the bucket attached to the source Space at freeze time. The original
Space is not modified, and the new static Space does not automatically reflect
metrics uploaded to the original Gradio Space after the freeze completes.

## delete_project[[trackio.delete_project]]

#### trackio.delete_project[[trackio.delete_project]]

```python
trackio.delete_project(project: str, force: bool = False)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/__init__.py#L899)

**Parameters:**

project (`str`) : The name of the project to delete.

force (`bool`, *optional*, defaults to `False`) : If `True`, deletes the project without prompting for confirmation. If `False`, prompts the user to confirm before deleting.

**Returns:** `bool`

`True` if the project was deleted, `False` otherwise.

Deletes a project by removing its local SQLite database.

## import_csv[[trackio.import_csv]]

#### trackio.import_csv[[trackio.import_csv]]

```python
trackio.import_csv(csv_path: str | pathlib.Path, project: str, name: str | None = None, space_id: str | None = None, dataset_id: str | None = None, private: bool | None = None, force: bool = False)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/imports.py#L9)

**Parameters:**

csv_path (`str` or `Path`) : The str or Path to the CSV file to import.

project (`str`) : The name of the project to import the CSV file into. Must not be an existing project.

name (`str`, *optional*) : The name of the Run to import the CSV file into. If not provided, a default name will be generated.

name (`str`, *optional*) : The name of the run (if not provided, a default name will be generated).

space_id (`str`, *optional*) : If provided, the project will be logged to a Hugging Face Space instead of a local directory. Should be a complete Space name like `"username/reponame"` or `"orgname/reponame"`, or just `"reponame"` in which case the Space will be created in the currently-logged-in Hugging Face user's namespace. If the Space does not exist, it will be created. If the Space already exists, the project will be logged to it.

dataset_id (`str`, *optional*) : Deprecated: persisting trackio data to a Hugging Face Dataset will be removed in a future version of trackio. Use `bucket_id` (a Hugging Face Bucket) instead.

private (`bool`, *optional*) : Whether to make the Space private. If None (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

Imports a CSV file into a Trackio project. The CSV file must contain a `"step"`
column, may optionally contain a `"timestamp"` column, and any other columns will be
treated as metrics. It should also include a header row with the column names.

TODO: call init() and return a Run object so that the user can continue to log metrics to it.

## import_tf_events[[trackio.import_tf_events]]

#### trackio.import_tf_events[[trackio.import_tf_events]]

```python
trackio.import_tf_events(log_dir: str | pathlib.Path, project: str, name: str | None = None, space_id: str | None = None, dataset_id: str | None = None, private: bool | None = None, force: bool = False)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/imports.py#L163)

**Parameters:**

log_dir (`str` or `Path`) : The str or Path to the directory containing TensorFlow Events files.

project (`str`) : The name of the project to import the TensorFlow Events files into. Must not be an existing project.

name (`str`, *optional*) : The name prefix for runs (if not provided, will use directory names). Each subdirectory will create a separate run.

space_id (`str`, *optional*) : If provided, the project will be logged to a Hugging Face Space instead of a local directory. Should be a complete Space name like `"username/reponame"` or `"orgname/reponame"`, or just `"reponame"` in which case the Space will be created in the currently-logged-in Hugging Face user's namespace. If the Space does not exist, it will be created. If the Space already exists, the project will be logged to it.

dataset_id (`str`, *optional*) : Deprecated: persisting trackio data to a Hugging Face Dataset will be removed in a future version of trackio. Use `bucket_id` (a Hugging Face Bucket) instead.

private (`bool`, *optional*) : Whether to make the Space private. If None (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.

Imports TensorFlow Events files from a directory into a Trackio project. Each
subdirectory in the log directory will be imported as a separate run.

## save[[trackio.save]]

#### trackio.save[[trackio.save]]

```python
trackio.save(glob_str: str | pathlib.Path, project: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/__init__.py#L954)

**Parameters:**

glob_str (`str` or `Path`) : The file path or glob pattern to save. Can be a single file or a pattern matching multiple files (e.g., `"*.py"`, `"models/**/*.pth"`).

project (`str`, *optional*) : The name of the project to save files to. If not provided, uses the current project from `trackio.init()`. If no project is initialized, raises an error.

**Returns:** `str`

The path where the file(s) were saved (project's files directory).

Saves files to a project (not linked to a specific run). If Trackio is running
locally, the file(s) will be copied to the project's files directory. If Trackio is
running in a Space, the file(s) will be uploaded to the Space's files directory.

Example:
```python
import trackio

trackio.init(project="my-project")
trackio.save("config.yaml")
trackio.save("models/*.pth")
```

## Artifact[[trackio.Artifact]]

#### trackio.Artifact[[trackio.Artifact]]

```python
trackio.Artifact(name: str, type: str, description: str | None = None, metadata: dict | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/artifact.py#L114)

**Parameters:**

name (`str`) : Artifact name, unique within the project. Must match `^[A-Za-z0-9._-]+$` (letters, digits, dot, underscore, hyphen).

type (`str`) : Free-form category such as `"model"` or `"dataset"`, used to group artifacts.

description (`str`, *optional*) : Human-readable description of the artifact.

metadata (`dict`, *optional*) : Arbitrary JSON-serializable metadata stored alongside the version.

A versioned, named bundle of files and references attached to a project.

Construct an `Artifact`, add files to it with `add_file` / `add_dir`
(bytes staged into content-addressed storage) or `add_reference` (an
external URI recorded without copying any bytes), then persist it with
`trackio.log_artifact`. Logging freezes the artifact and populates its
read-only `version`, `aliases`, `size`, and `manifest`.

#### add_dir[[trackio.Artifact.add_dir]]

```python
add_dir(local_dir: str | pathlib.Path, name: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/artifact.py#L316)

**Parameters:**

local_dir (`str` or `Path`) : Path to an existing directory whose files are added.

name (`str`, *optional*) : Logical path prefix for the added files. Defaults to no prefix, so files keep their paths relative to `local_dir`.

Stage every regular file under a directory, recursively.

Symlinks are skipped.

#### add_file[[trackio.Artifact.add_file]]

```python
add_file(local_path: str | pathlib.Path, name: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/artifact.py#L296)

**Parameters:**

local_path (`str` or `Path`) : Path to an existing regular file to add.

name (`str`, *optional*) : Logical path the file is stored under inside the artifact. Defaults to the file's basename.

Stage a single file for inclusion in the artifact.

#### add_reference[[trackio.Artifact.add_reference]]

```python
add_reference(uri: str, name: str | None = None, checksum: bool = True, max_objects: int | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/artifact.py#L346)

**Parameters:**

uri (`str`) : Location of the referenced data, percent-encoded (build local URIs with `pathlib.Path.as_uri()`). `file://`, `http(s)://`, and `hf://` are resolved and fetched natively; any other scheme (`s3://`, `gs://`, Azure blob URLs, ...) requires a custom `ReferenceHandler` registered via `trackio.register_reference_handler` — see the artifacts documentation for complete examples — and raises otherwise.

name (`str`, *optional*) : Logical path the reference is stored under. Defaults to the last segment of `uri`; for an expanded prefix (ending with /) it is the prefix under which each object's relative path is nested.

checksum (`bool`, *optional*, defaults to `True`) : When `True`, probes the source to derive its size and a checksum (and to expand a directory/prefix): `file://` is stream-hashed to a sha256, `hf://` uses its LFS sha256 or git blob id, and `http(s)://` uses the server `ETag`. This may read the filesystem or make network requests. When no checksum can be obtained for a *remote* source (e.g. no ETag), the URI itself is used as the `digest` and a warning is emitted. A local `file://` path that does not exist instead raises. Pass `checksum=False` to skip remote probing entirely.

max_objects (`int`, *optional*) : Cap on how many objects a directory/prefix expands into (default 10,000); exceeding it raises.

Stage a reference to external data, without copying any bytes.

Unlike `add_file`/`add_dir`, a reference records the data's URI (plus,
when available, a size and checksum) *without staging the bytes into
the content-addressed store*. This is how you version and track
lineage over large or already-durably-stored datasets that must not be
duplicated.

A URI that denotes a directory (`file://`, `hf://`, or a prefix
expanded by a custom handler) is expanded into one entry per object,
capped by `max_objects`. Reference bytes are never uploaded to a Space
or bucket.

The URI is stored verbatim in the artifact manifest, and manifests may
be synced to a shared Hugging Face Dataset — so a signed URI triggers a
warning for security reasons; please prefer the object's canonical
unsigned URI (e.g. `s3://bucket/key`).

#### download[[trackio.Artifact.download]]

```python
download(root: str | pathlib.Path | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/artifact.py#L497)

**Parameters:**

root (`str` or `Path`, *optional*) : Directory to write the files into. Defaults to `./.trackio/artifact-downloads/<project>/<name>_v<version>/`, keyed by project so same-named artifacts from different projects never collide, and by the resolved version so a moving alias like `latest` never leaves behind stale files from a previous version.

**Returns:**

The absolute path to the download directory, as a string.

Materialize the artifact's files into a local directory.

Files are copied from Trackio's content-addressed cache (and fetched
from the remote when missing locally), so repeated calls are cheap and
idempotent.

Reference entries added via `add_reference` are fetched here too: the
object at each reference URI is downloaded into the directory (via the
bundled `httpx` for `http(s)://`, `huggingface_hub` for `hf://`, and
the registered custom `ReferenceHandler` for any other scheme). Each
object is written atomically and calls are idempotent. Because reference digests
are opaque (a provider ETag or the URI itself), a present file is assumed
current and is not re-verified against the source, and a source whose
re-probed checksum no longer matches the recorded one is downloaded
anyway with a warning. Downloading can transfer a large amount of data;
to work with the URIs without downloading, read them from the `references`
property or `get_entry_uri` instead. A missing source, an unreachable
server, or a scheme with no registered handler raises.

#### get_entry_uri[[trackio.Artifact.get_entry_uri]]

```python
get_entry_uri(name: str)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/artifact.py#L762)

Return the URI recorded for the reference entry at logical path
`name`, or None if there is no reference entry with that name.

#### link[[trackio.Artifact.link]]

```python
link(target_path: str, aliases: list[str] | None = None, bucket_id: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/artifact.py#L580)

**Parameters:**

target_path (`str`) : The collection to link into, as `"registry-<registry>/<collection>"`. The registry must already exist.

aliases (`list[str]`, *optional*) : Aliases to place on the linked version. `latest` is managed automatically; passing it is a no-op.

bucket_id (`str`, *optional*) : Bucket holding the registry, e.g. `"my-org/models-registry"`. Omit for a local registry; defaults to `TRACKIO_REGISTRY_BUCKET_ID` when that is set.

**Returns:**

The linked artifact at its registry location.

Link this artifact version into a registry collection.

A link is a pointer to this version: no files are copied. See
`Run.link_artifact` for the full semantics (collection versioning,
aliases, and the returned linked artifact).

The artifact must already be logged or fetched; unlike
`Run.link_artifact`, this does not log a draft artifact for you, and
the link records no publishing run. An artifact fetched from a Space or
a self-hosted server can only be linked into a bucket-backed registry,
since a local registry is not reachable from where its bytes live.

#### unlink[[trackio.Artifact.unlink]]

```python
unlink()
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/artifact.py#L625)

Remove this registry link.

Call this on the linked artifact returned by `link` or
`Run.link_artifact`, not on a source artifact version. The source
artifact and its files are untouched; only the collection membership
is removed. Aliases pointing at the link go with it, and the
collection version number is never reused. If the version held
`latest`, that alias moves to the highest remaining version.

#### wait[[trackio.Artifact.wait]]

```python
wait(timeout: int | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/artifact.py#L286)

No-op: trackio logs artifacts synchronously, so the artifact is
already committed by the time `log_artifact` returns.

## Registry[[trackio.Registry]]

#### trackio.Registry[[trackio.Registry]]

```python
trackio.Registry(name: str, bucket_id: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/registry.py#L51)

**Parameters:**

name (`str`) : Registry name, e.g. `"models"`. Must match `^[A-Za-z0-9_-]+$` (letters, digits, underscore, hyphen).

bucket_id (`str`, *optional*) : Hugging Face bucket holding the registry, e.g. `"my-org/models-registry"`. Omit for a local registry. Defaults to `TRACKIO_REGISTRY_BUCKET_ID` when that is set.

A handle on one artifact registry.

A registry is a shared catalog of artifact versions, organized into
collections. You publish into it with `Run.link_artifact` and promote
versions by moving aliases such as `"staging"` or `"production"`. See
the Registry guide for the full picture.

You do not construct a `Registry` yourself. Create a registry with
`trackio.Api().create_registry(name)` or fetch an existing one with
`trackio.Api().registry(name)`; both return this handle.

Example:
```python
import trackio

registry = trackio.Api().create_registry("models")
registry.create_collection(
    "my-model", artifact_type="model", description="My favorite model"
)

run = trackio.init(project="my-experiments")
artifact = trackio.log_artifact("model.pt", name="resnet", type="model")
run.link_artifact(artifact, "registry-models/my-model", aliases=["staging"])
trackio.finish()

registry.collection("my-model").links
```

#### collection[[trackio.Registry.collection]]

```python
collection(name: str)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/registry.py#L145)

Describe one collection.

Returns None if the registry or the collection does not exist.

#### collections[[trackio.Registry.collections]]

```python
collections()
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/registry.py#L163)

List the registry's collections, links included.

Returns an empty list if the registry does not exist.

#### create_collection[[trackio.Registry.create_collection]]

```python
create_collection(name: str, artifact_type: str, description: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/registry.py#L113)

**Parameters:**

name (`str`) : Collection name, unique within the registry. Must match `^[A-Za-z0-9._-]+$`.

artifact_type (`str`) : The artifact type the collection accepts, e.g. `"model"` or `"dataset"`. The type is fixed at creation; linking an artifact of a different type raises.

description (`str`, *optional*) : Human-readable description of the collection.

**Returns:**

The [Collection](/docs/trackio/v0.36.0/en/api#trackio.Collection).

Create a collection in this registry.

Collections are also created automatically on first link, so this
is mainly useful to set a description up front. If the collection
already exists, it is returned as-is and a non-None `description`
is applied.

#### events[[trackio.Registry.events]]

```python
events()
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/registry.py#L172)

Return the registry's audit log, oldest event first.

Each event has an `id`, a `ts` timestamp, a `kind` (`"create"`,
`"link"`, `"promote"`, `"update"`, or `"unlink"`), and a `payload`
describing the mutation. A `create` payload without a `collection`
key records the creation of the registry itself.

## Collection[[trackio.Collection]]

#### trackio.Collection[[trackio.Collection]]

```python
trackio.Collection(name: str, type: str, description: str | None, created_at: str, num_links: int, latest_version: int | None, links: list)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/registry.py#L20)

A read-only snapshot of one registry collection.

Returned by `Registry.create_collection`, `Registry.collection`, and
`Registry.collections`.

## Image[[trackio.TrackioImage]]

#### trackio.TrackioImage[[trackio.TrackioImage]]

```python
trackio.TrackioImage(value: str | pathlib.Path | numpy.ndarray | PIL.Image.Image, caption: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/media/image.py#L13)

**Parameters:**

value (`str`, `Path`, `numpy.ndarray`, or `PIL.Image`, *optional*) : A path to an image, a PIL Image, or a numpy array of shape (height, width, channels). If numpy array, should be of type `np.uint8` with RGB values in the range `[0, 255]`.

caption (`str`, *optional*) : A string caption for the image.

Initializes an Image object.

Example:
```python
import trackio
import numpy as np
from PIL import Image

# Create an image from numpy array
image_data = np.random.randint(0, 255, (64, 64, 3), dtype=np.uint8)
image = trackio.Image(image_data, caption="Random image")
trackio.log({"my_image": image})

# Create an image from PIL Image
pil_image = Image.new('RGB', (100, 100), color='red')
image = trackio.Image(pil_image, caption="Red square")
trackio.log({"red_image": image})

# Create an image from file path
image = trackio.Image("path/to/image.jpg", caption="Photo from file")
trackio.log({"file_image": image})
```

## Video[[trackio.TrackioVideo]]

#### trackio.TrackioVideo[[trackio.TrackioVideo]]

```python
trackio.TrackioVideo(value: str | pathlib.Path | numpy.ndarray, caption: str | None = None, fps: int | None = None, format: typing.Optional[typing.Literal['gif', 'mp4', 'webm']] = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/media/video.py#L17)

**Parameters:**

value (`str`, `Path`, or `numpy.ndarray`, *optional*) : A path to a video file, or a numpy array. If numpy array, should be of type `np.uint8` with RGB values in the range `[0, 255]`. It is expected to have shape of either (frames, channels, height, width) or (batch, frames, channels, height, width). For the latter, the videos will be tiled into a grid.

caption (`str`, *optional*) : A string caption for the video.

fps (`int`, *optional*) : Frames per second for the video. Only used when value is an ndarray. Default is `24`.

format (`Literal["gif", "mp4", "webm"]`, *optional*) : Video format ("gif", "mp4", or "webm"). Only used when value is an ndarray. Default is "gif".

Initializes a Video object.

Example:
```python
import trackio
import numpy as np

# Create a simple video from numpy array
frames = np.random.randint(0, 255, (10, 3, 64, 64), dtype=np.uint8)
video = trackio.Video(frames, caption="Random video", fps=30)

# Create a batch of videos
batch_frames = np.random.randint(0, 255, (3, 10, 3, 64, 64), dtype=np.uint8)
batch_video = trackio.Video(batch_frames, caption="Batch of videos", fps=15)

# Create video from file path
video = trackio.Video("path/to/video.mp4", caption="Video from file")
```

#### write_video[[trackio.TrackioVideo.write_video]]

```python
write_video(file_path: str | pathlib.Path, video: ndarray, fps: float, codec: typing.Literal['h264', 'vp9', 'gif'])
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/media/video.py#L93)

RGB uint8 only, shape (F, H, W, 3).

## Audio[[trackio.TrackioAudio]]

#### trackio.TrackioAudio[[trackio.TrackioAudio]]

```python
trackio.TrackioAudio(value: str | pathlib.Path | numpy.ndarray, caption: str | None = None, sample_rate: int | None = None, format: typing.Optional[typing.Literal['wav', 'mp3']] = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/media/audio.py#L19)

**Parameters:**

value (`str`, `Path`, or `numpy.ndarray`, *optional*) : A path to an audio file, or a numpy array. The array should be shaped `(samples,)` for mono or `(samples, 2)` for stereo. Float arrays will be peak-normalized and converted to 16-bit PCM; integer arrays will be converted to 16-bit PCM as needed.

caption (`str`, *optional*) : A string caption for the audio.

sample_rate (`int`, *optional*) : Sample rate in Hz. Required when `value` is a numpy array.

format (`Literal["wav", "mp3"]`, *optional*) : Audio format used when `value` is a numpy array. Default is "wav".

Initializes an Audio object.

Example:
```python
import trackio
import numpy as np

# Generate a 1-second 440 Hz sine wave (mono)
sr = 16000
t = np.linspace(0, 1, sr, endpoint=False)
wave = 0.2 * np.sin(2 * np.pi * 440 * t)
audio = trackio.Audio(wave, caption="A4 sine", sample_rate=sr, format="wav")
trackio.log({"tone": audio})

# Stereo from numpy array (shape: samples, 2)
stereo = np.stack([wave, wave], axis=1)
audio = trackio.Audio(stereo, caption="Stereo", sample_rate=sr, format="mp3")
trackio.log({"stereo": audio})

# From an existing file
audio = trackio.Audio("path/to/audio.wav", caption="From file")
trackio.log({"file_audio": audio})
```

#### ensure_int16_pcm[[trackio.TrackioAudio.ensure_int16_pcm]]

```python
ensure_int16_pcm(data: ndarray)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/media/audio.py#L90)

Convert input audio array to contiguous int16 PCM.
Peak normalization is applied to floating inputs.

## Object3D[[trackio.TrackioObject3D]]

#### trackio.TrackioObject3D[[trackio.TrackioObject3D]]

```python
trackio.TrackioObject3D(path_or_array: str | pathlib.Path | numpy.ndarray, caption: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/media/object3d.py#L207)

## Html[[trackio.TrackioHtml]]

#### trackio.TrackioHtml[[trackio.TrackioHtml]]

```python
trackio.TrackioHtml(data: typing.Any, inject: bool = True, data_is_not_path: bool = False, caption: str | None = None)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/media/html.py#L17)

**Parameters:**

data (`str`, `Path`, file-like, Plotly figure, or Matplotlib figure) : Raw HTML, a path to an `.html` file, a file-like object, or a Plotly or Matplotlib figure.

inject (`bool`, *optional*) : Wrap HTML fragments in a styled document. Default is `True`.

data_is_not_path (`bool`, *optional*) : Accepted for `wandb.Html` compatibility but ignored.

caption (`str`, *optional*) : A string caption for the HTML.

Initializes an Html object.

Example:
```python
import trackio

trackio.init(project="my-project")

# Raw HTML
trackio.log({"report": trackio.Html("<h1>Results</h1>")})

# Plotly figure
import plotly.express as px

fig = px.line(x=[1, 2, 3], y=[4, 5, 6])
trackio.log({"plot": trackio.Html(fig)})

# Matplotlib figure
import matplotlib.pyplot as plt

plt.plot([1, 2, 3, 4])
trackio.log({"chart": trackio.Html(plt)})
```

## Table[[trackio.Table]]

#### trackio.Table[[trackio.Table]]

```python
trackio.Table(columns: list[str] | None = None, data: list[list[typing.Any]] | None = None, dataframe: typing.Optional[typing.Any] = None, rows: list[list[typing.Any]] | None = None, optional: bool | list[bool] = True, allow_mixed_types: bool = False, log_mode: typing.Optional[typing.Literal['IMMUTABLE', 'MUTABLE', 'INCREMENTAL']] = 'IMMUTABLE')
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/table.py#L9)

**Parameters:**

columns (`list[str]`, *optional*) : Names of the columns in the table. Optional if `data` is provided. Not expected if `dataframe` is provided. Currently ignored.

data (`list[list[Any]]`, *optional*) : 2D row-oriented array of values. Each value can be a number, a string (treated as Markdown and truncated if too long), or a `Trackio.Image` or list of `Trackio.Image` objects.

dataframe (`pandas.DataFrame`, *optional*) : DataFrame used to create the table. When set, `data` and `columns` arguments are ignored.

rows (`list[list[Any]]`, *optional*) : Currently ignored.

optional (`bool` or `list[bool]`, *optional*, defaults to `True`) : Currently ignored.

allow_mixed_types (`bool`, *optional*, defaults to `False`) : Currently ignored.

log_mode : (`Literal["IMMUTABLE", "MUTABLE", "INCREMENTAL"]` or `None`, *optional*, defaults to `"IMMUTABLE"`): Currently ignored.

Initializes a Table object.

Tables can be used to log tabular data including images, numbers, and text.

#### to_display_format[[trackio.Table.to_display_format]]

```python
to_display_format(table_data: list)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/table.py#L124)

**Parameters:**

table_data (`list[dict]`) : List of dictionaries representing table rows (from stored `_value`).

**Returns:** `list[dict]`

Table data with images converted to markdown syntax and long
text truncated.

Converts stored table data to display format for UI rendering.

Note:
This does not use the `self.data` attribute, but instead uses the
`table_data` parameter, which is what the UI receives.

## Histogram[[trackio.Histogram]]

#### trackio.Histogram[[trackio.Histogram]]

```python
trackio.Histogram(sequence: typing.Union[numpy.ndarray, typing.Sequence[float], typing.Sequence[int], NoneType] = None, np_histogram: tuple | None = None, num_bins: int = 64)
```

[Source](https://github.com/gradio-app/trackio/blob/v0.36.0/trackio/histogram.py#L6)

**Parameters:**

sequence (`np.ndarray` or `Sequence[float]` or `Sequence[int]`, *optional*) : Sequence of values to create the histogram from.

np_histogram (`tuple`, *optional*) : Pre-computed NumPy histogram as a `(hist, bins)` tuple.

num_bins (`int`, *optional*, defaults to `64`) : Number of bins for the histogram (maximum `512`).

Histogram data type for Trackio, compatible with wandb.Histogram.

Example:
```python
import trackio
import numpy as np

# Create histogram from sequence
data = np.random.randn(1000)
trackio.log({"distribution": trackio.Histogram(data)})

# Create histogram from numpy histogram
hist, bins = np.histogram(data, bins=30)
trackio.log({"distribution": trackio.Histogram(np_histogram=(hist, bins))})

# Specify custom number of bins
trackio.log({"distribution": trackio.Histogram(data, num_bins=50)})
```
