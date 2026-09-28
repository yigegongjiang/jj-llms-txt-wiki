# env

Global configuration for the library. Mutate fields on the exported
`env` object at startup to change where models are loaded from, how files
are cached, and how verbose logging is.

**Example:** Load models from your own server and disable remote downloads.
```javascript
import { env } from '@huggingface/transformers';
env.allowRemoteModels = false;
env.localModelPath = '/path/to/local/models/';
```

**Example:** Point the filesystem cache at a custom directory (Node.js).
```javascript
import { env } from '@huggingface/transformers';
env.cacheDir = '/path/to/cache/directory/';
```

## Constants

### `LogLevel`

Log-level enum. Assign to `env.logLevel` to control how verbose the library
is. Higher values silence more: `DEBUG` (10) surfaces everything,
`NONE` (50) suppresses all output. Default is `WARNING` (30).

| Level     | Value | Shows                                     |
|-----------|-------|-------------------------------------------|
| `DEBUG`   | 10    | Every message, including debug traces.    |
| `INFO`    | 20    | Errors, warnings, and info messages.      |
| `WARNING` | 30    | Errors and warnings.                      |
| `ERROR`   | 40    | Only errors.                              |
| `NONE`    | 50    | Nothing.                                  |

```javascript
import { env, LogLevel } from '@huggingface/transformers';
env.logLevel = LogLevel.ERROR;
```

### `env` : [`TransformersEnvironment`](./env#module_env.TransformersEnvironment)

The global configuration object. See `TransformersEnvironment` below for the
full set of fields.

## Type Definitions

### TransformersEnvironment

Shape of the `env` object. Every field is mutable.

**Properties**

- `version` (`string`) — This version of Transformers.js.
- `backends` (`{ onnx: Partial<Env> & { setLogLevel?: (logLevel: number) => void } }`) — Exposes backend environment settings that users can override.
- `logLevel` (`number`) — The logging level. Use LogLevel enum values. Defaults to LogLevel.WARNING.
- `allowRemoteModels` (`boolean`) — Whether to allow loading of remote files, defaults to `true`.
  If set to `false`, it will have the same effect as setting `local_files_only=true` when loading pipelines, models, tokenizers, processors, etc.
- `remoteHost` (`string`) — Host URL to load models from. Defaults to the Hugging Face Hub.
- `remotePathTemplate` (`string`) — Path template to fill in and append to `remoteHost` when loading models.
- `allowLocalModels` (`boolean`) — Whether to allow loading of local files, defaults to `false` if running in-browser, and `true` otherwise.
  If set to `false`, it will skip the local file check and try to load the model from the remote host.
- `localModelPath` (`string`) — Path to load local models from. By default, it is `/models/` relative to the library's installed location when a file system is available (e.g., Node.js), and the `/models/` URL path otherwise (e.g., browsers).
- `useFS` (`boolean`) — Whether to use the file system to load files. By default, it is `true` if available.
- `useBrowserCache` (`boolean`) — Whether to use Cache API to cache models. By default, it is `true` if available.
- `useFSCache` (`boolean`) — Whether to use the file system to cache files. By default, it is `true` if available.
- `cacheDir` (`string` | `null`) — The directory to use for caching files with the file system. By default, it is `.cache` relative to the library's installed location when a file system is available (e.g., Node.js), and `null` otherwise (e.g., browsers).
- `useCustomCache` (`boolean`) — Whether to use a custom cache system (defined by `customCache`), defaults to `false`.
- `customCache` (`CacheInterface` | `null`) — The custom cache to use. Defaults to `null`. This must be an object that
  implements the `match` and `put` functions of the Web Cache API. For more information, see https://developer.mozilla.org/en-US/docs/Web/API/Cache.
- `useWasmCache` (`boolean`) — Whether to pre-load and cache WASM binaries and the WASM factory (.mjs) for ONNX Runtime.
  Defaults to `true` when cache is available. This can improve performance and enables offline usage by avoiding repeated downloads.
- `cacheKey` (`string`) — The cache key to use for storing models and WASM binaries. Defaults to `transformers-cache`.
- `experimental_useCrossOriginStorage` (`boolean`) — Whether to use the Cross-Origin Storage API to cache model files
  across origins, allowing different sites to share the same cached model weights. Defaults to `false`.
  Requires the Cross-Origin Storage Chrome extension: [https://chromewebstore.google.com/detail/cross-origin-storage/denpnpcgjgikjpoglpjefakmdcbmlgih](https://chromewebstore.google.com/detail/cross-origin-storage/denpnpcgjgikjpoglpjefakmdcbmlgih).
  The `experimental_` prefix indicates that the underlying browser API is not yet standardized and may change or be
  removed without a major version bump. For more information, see [https://github.com/WICG/cross-origin-storage](https://github.com/WICG/cross-origin-storage).
- `fetch` (`(input: string | URL, init?: any) => Promise<any>`) — The fetch function to use. Defaults to `fetch`.
