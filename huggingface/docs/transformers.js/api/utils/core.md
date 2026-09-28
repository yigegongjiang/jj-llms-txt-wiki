# utils/core

Shared types that describe model loading progress.

`ProgressInfo` and its discriminated-union variants describe the payload
passed to a `progress_callback` so callers can render download UIs, log
byte counts, and react to the `ready` event.

## Type Definitions

### InitiateProgressInfo

**Properties**

- `status` (`'initiate'`) — A file load is about to start.
- `name` (`string`) — The model ID or directory path.
- `file` (`string`) — The name of the file.

### DownloadProgressInfo

**Properties**

- `status` (`'download'`) — A file download has started.
- `name` (`string`) — The model ID or directory path.
- `file` (`string`) — The name of the file.

### ProgressStatusInfo

**Properties**

- `status` (`'progress'`) — A file download has reported byte progress.
- `name` (`string`) — The model ID or directory path.
- `file` (`string`) — The name of the file.
- `progress` (`number`) — A number between 0 and 100.
- `loaded` (`number`) — The number of bytes loaded.
- `total` (`number`) — The total number of bytes to be loaded.

### FileLoadingProgress

**Properties**

- `loaded` (`number`) — The number of bytes loaded for this file.
- `total` (`number`) — The total number of bytes for this file.

### FilesLoadingMap

A mapping of file names to their loading progress. Each key is a file path and each value contains
the loaded and total bytes for that file.

_Type:_ `Record`<`string`, [`FileLoadingProgress`](./core#module_utils/core.FileLoadingProgress)>

### TotalProgressInfo

**Properties**

- `status` (`'progress_total'`) — Aggregate progress across all files being loaded.
- `name` (`string`) — The model ID or directory path.
- `progress` (`number`) — A number between 0 and 100.
- `loaded` (`number`) — The number of bytes loaded.
- `total` (`number`) — The total number of bytes to be loaded.
- `files` ([`FilesLoadingMap`](./core#module_utils/core.FilesLoadingMap)) — A mapping of file names to their loading progress.

### DoneProgressInfo

**Properties**

- `status` (`'done'`) — A file has finished loading.
- `name` (`string`) — The model ID or directory path.
- `file` (`string`) — The name of the file.

### ReadyProgressInfo

**Properties**

- `status` (`'ready'`) — The requested pipeline is ready to use.
- `task` (`string`) — The loaded task.
- `model` (`string`) — The loaded model.

### ProgressInfo

_Type:_ [`InitiateProgressInfo`](./core#module_utils/core.InitiateProgressInfo) | [`DownloadProgressInfo`](./core#module_utils/core.DownloadProgressInfo) | [`ProgressStatusInfo`](./core#module_utils/core.ProgressStatusInfo) | [`DoneProgressInfo`](./core#module_utils/core.DoneProgressInfo) | [`ReadyProgressInfo`](./core#module_utils/core.ReadyProgressInfo) | [`TotalProgressInfo`](./core#module_utils/core.TotalProgressInfo)

## Callbacks

### ProgressCallback

A callback function that is called with progress information.

**Parameters**

- `progressInfo` ([`ProgressInfo`](./core#module_utils/core.ProgressInfo))

**Returns:** `void`
