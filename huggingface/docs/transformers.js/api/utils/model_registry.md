# utils/model_registry

Model registry for cache and file operations

Provides static methods for:
- Discovering which files a model needs
- Detecting available quantization levels (dtypes)
- Getting file metadata
- Checking cache status

**Example:** Get all files needed for a model
```javascript
const files = await ModelRegistry.get_files(
  "onnx-community/all-MiniLM-L6-v2-ONNX",
  { dtype: "fp16" },
);
console.log(files); // [ 'config.json', 'onnx/model_fp16.onnx', 'onnx/model_fp16.onnx_data', 'tokenizer.json', 'tokenizer_config.json' ]
```

**Example:** Get all files needed for a specific pipeline task
```javascript
const files = await ModelRegistry.get_pipeline_files(
  "text-generation",
  "onnx-community/Qwen3-0.6B-ONNX",
  { dtype: "q4" },
);
console.log(files); // [ 'config.json', 'onnx/model_q4.onnx', 'generation_config.json', 'tokenizer.json', 'tokenizer_config.json' ]
```

**Example:** Get specific component files
```javascript
const modelFiles = await ModelRegistry.get_model_files("onnx-community/all-MiniLM-L6-v2-ONNX", { dtype: "q4" });
const tokenizerFiles = await ModelRegistry.get_tokenizer_files("onnx-community/all-MiniLM-L6-v2-ONNX");
const processorFiles = await ModelRegistry.get_processor_files("onnx-community/all-MiniLM-L6-v2-ONNX");
console.log(modelFiles); // [ 'config.json', 'onnx/model_q4.onnx', 'onnx/model_q4.onnx_data' ]
console.log(tokenizerFiles); // [ 'tokenizer.json', 'tokenizer_config.json' ]
console.log(processorFiles); // [ ]
```

**Example:** Detect available quantization levels for a model
```javascript
const dtypes = await ModelRegistry.get_available_dtypes("onnx-community/all-MiniLM-L6-v2-ONNX");
console.log(dtypes); // [ 'fp32', 'fp16', 'int8', 'uint8', 'q8', 'q4' ]

// Use the result to pick the best available dtype
const preferredDtype = dtypes.includes("q4") ? "q4" : "fp32";
const files = await ModelRegistry.get_files("onnx-community/all-MiniLM-L6-v2-ONNX", { dtype: preferredDtype });
```

**Example:** Check file metadata without downloading
```javascript
const metadata = await ModelRegistry.get_file_metadata(
  "onnx-community/Qwen3-0.6B-ONNX",
  "config.json"
);
console.log(metadata); // { exists: true, size: 912, contentType: 'application/json', fromCache: true }
```

**Example:** Model cache management
```javascript
const modelId = "onnx-community/Qwen3-0.6B-ONNX";
const options = { dtype: "q4" };

// Quickly check if the model is cached (probably false)
let cached = await ModelRegistry.is_cached(modelId, options);
console.log(cached); // false

// Get per-file cache detail
let cacheStatus = await ModelRegistry.is_cached_files(modelId, options);
console.log(cacheStatus);
// {
//   allCached: false,
//   files: [ { file: 'config.json', cached: true }, { file: 'onnx/model_q4.onnx', cached: false }, { file: 'generation_config.json', cached: false }, { file: 'tokenizer.json', cached: false }, { file: 'tokenizer_config.json', cached: false } ]
// }

// Download the model by instantiating a pipeline
const generator = await pipeline("text-generation", modelId, options);
const output = await generator(
  [{ role: "user", content: "What is the capital of France?" }],
  { max_new_tokens: 256, do_sample: false },
);
console.log(output[0].generated_text.at(-1).content); // <think>...</think>\n\nThe capital of France is **Paris**.

// Check if the model is cached (should be true now)
cached = await ModelRegistry.is_cached(modelId, options);
console.log(cached); // true

// Clear the cache
const clearResult = await ModelRegistry.clear_cache(modelId, options);
console.log(clearResult);
// {
//   filesDeleted: 5,
//   filesCached: 5,
//   files: [ { file: 'config.json', deleted: true, wasCached: true }, { file: 'onnx/model_q4.onnx', deleted: true, wasCached: true }, { file: 'generation_config.json', deleted: true, wasCached: true }, { file: 'tokenizer.json', deleted: true, wasCached: true }, { file: 'tokenizer_config.json', deleted: true, wasCached: true } ]
// }

// Check if the model is cached (should be false again)
cached = await ModelRegistry.is_cached(modelId, options);
console.log(cached); // false
```

## Classes

### ModelRegistry

Static class for cache and file management operations.

#### `ModelRegistry.get_files(modelId, [options])`

Get all files (model, tokenizer, processor) needed for a model.

**Parameters**

- `modelId` (`string`) — The model id (e.g., "onnx-community/bert-base-uncased-ONNX")
- `options` (`Object`) _optional_ — Optional parameters
  - `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — defaults to `null` — Pre-loaded config
  - `dtype` ([`DataType`](../transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](../transformers#module_transformers.DataType)>) _optional_ — defaults to `null` — Override dtype
  - `device` ([`DeviceType`](../transformers#module_transformers.DeviceType) | `Record`<`string`, [`DeviceType`](../transformers#module_transformers.DeviceType)>) _optional_ — defaults to `null` — Override device
  - `model_file_name` (`string`) _optional_ — defaults to `null` — Override the model file name (excluding .onnx suffix)
  - `include_tokenizer` (`boolean`) _optional_ — defaults to `true` — Whether to check for tokenizer files
  - `include_processor` (`boolean`) _optional_ — defaults to `true` — Whether to check for processor files

**Returns:** `Promise`<`string[]`> — Array of file paths

**Example:**
```javascript
const files = await ModelRegistry.get_files('onnx-community/gpt2-ONNX');
console.log(files); // ['config.json', 'tokenizer.json', 'onnx/model_q4.onnx', ...]
```

#### `ModelRegistry.get_pipeline_files(task, modelId, [options])`

Get all files needed for a specific pipeline task.
Automatically determines which components are needed based on the task.

**Parameters**

- `task` (`string`) — The pipeline task (e.g., "text-generation", "background-removal")
- `modelId` (`string`) — The model id (e.g., "onnx-community/bert-base-uncased-ONNX")
- `options` (`Object`) _optional_ — Optional parameters
  - `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — defaults to `null` — Pre-loaded config
  - `dtype` ([`DataType`](../transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](../transformers#module_transformers.DataType)>) _optional_ — defaults to `null` — Override dtype
  - `device` ([`DeviceType`](../transformers#module_transformers.DeviceType) | `Record`<`string`, [`DeviceType`](../transformers#module_transformers.DeviceType)>) _optional_ — defaults to `null` — Override device
  - `model_file_name` (`string`) _optional_ — defaults to `null` — Override the model file name (excluding .onnx suffix)

**Returns:** `Promise`<`string[]`> — Array of file paths

**Example:**
```javascript
const files = await ModelRegistry.get_pipeline_files('text-generation', 'onnx-community/gpt2-ONNX');
console.log(files); // ['config.json', 'tokenizer.json', 'onnx/model_q4.onnx', ...]
```

#### `ModelRegistry.get_model_files(modelId, [options])`

Get model files needed for a specific model.

**Parameters**

- `modelId` (`string`) — The model id
- `options` (`Object`) _optional_ — Optional parameters
  - `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — defaults to `null` — Pre-loaded config
  - `dtype` ([`DataType`](../transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](../transformers#module_transformers.DataType)>) _optional_ — defaults to `null` — Override dtype
  - `device` ([`DeviceType`](../transformers#module_transformers.DeviceType) | `Record`<`string`, [`DeviceType`](../transformers#module_transformers.DeviceType)>) _optional_ — defaults to `null` — Override device
  - `model_file_name` (`string`) _optional_ — defaults to `null` — Override the model file name (excluding .onnx suffix)

**Returns:** `Promise`<`string[]`> — Array of model file paths

**Example:**
```javascript
const files = await ModelRegistry.get_model_files('onnx-community/bert-base-uncased-ONNX');
console.log(files); // ['config.json', 'onnx/model_q4.onnx', 'generation_config.json']
```

#### `ModelRegistry.get_tokenizer_files(modelId)`

Get tokenizer files needed for a specific model.

**Parameters**

- `modelId` (`string`) — The model id

**Returns:** `Promise`<`string[]`> — Array of tokenizer file paths

**Example:**
```javascript
const files = await ModelRegistry.get_tokenizer_files('onnx-community/gpt2-ONNX');
console.log(files); // ['tokenizer.json', 'tokenizer_config.json']
```

#### `ModelRegistry.get_processor_files(modelId)`

Get processor files needed for a specific model.

**Parameters**

- `modelId` (`string`) — The model id

**Returns:** `Promise`<`string[]`> — Array of processor file paths

**Example:**
```javascript
const files = await ModelRegistry.get_processor_files('onnx-community/vit-base-patch16-224-ONNX');
console.log(files); // ['preprocessor_config.json']
```

#### `ModelRegistry.get_available_dtypes(modelId, [options])`

Detects which quantization levels (dtypes) are available for a model
by checking which ONNX files exist on the hub or locally.

A dtype is considered available if all required model session files
exist for that dtype.

An empty array means the model is accessible but has no complete set of ONNX files for any dtype.

**Parameters**

- `modelId` (`string`) — The model id (e.g., "onnx-community/all-MiniLM-L6-v2-ONNX")
- `options` (`Object`) _optional_ — Optional parameters
  - `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — defaults to `null` — Pre-loaded config
  - `model_file_name` (`string`) _optional_ — defaults to `null` — Override the model file name (excluding .onnx suffix)
  - `revision` (`string`) _optional_ — defaults to `'main'` — Model revision
  - `cache_dir` (`string`) _optional_ — defaults to `null` — Custom cache directory
  - `local_files_only` (`boolean`) _optional_ — defaults to `false` — Only check local files

**Returns:** `Promise`<`string[]`> — Array of available dtype strings (e.g., ['fp32', 'fp16', 'q4', 'q8']). Empty if the model has no ONNX files.

**Throws**

- `ModelFileNotFoundError` — If the model is missing or inaccessible. The Hub returns 401 for nonexistent repositories.
- `Error` — On network or server failures.

**Example:**
```javascript
const dtypes = await ModelRegistry.get_available_dtypes('onnx-community/all-MiniLM-L6-v2-ONNX');
console.log(dtypes); // ['fp32', 'fp16', 'int8', 'uint8', 'q8', 'q4']
```

#### `ModelRegistry.is_cached(modelId, [options])`

Quickly checks if a model is fully cached by verifying `config.json` is present,
then confirming all required files are cached.
Returns a plain boolean — use `is_cached_files` if you need per-file detail.

**Parameters**

- `modelId` (`string`) — The model id
- `options` (`Object`) _optional_ — Optional parameters
  - `cache_dir` (`string`) _optional_ — Custom cache directory
  - `revision` (`string`) _optional_ — Model revision (default: 'main')
  - `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — Pre-loaded config
  - `dtype` ([`DataType`](../transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](../transformers#module_transformers.DataType)>) _optional_ — defaults to `null` — Override dtype
  - `device` ([`DeviceType`](../transformers#module_transformers.DeviceType) | `Record`<`string`, [`DeviceType`](../transformers#module_transformers.DeviceType)>) _optional_ — defaults to `null` — Override device

**Returns:** `Promise`<`boolean`> — Whether all required files are cached

**Example:**
```javascript
const cached = await ModelRegistry.is_cached('onnx-community/bert-base-uncased-ONNX');
console.log(cached); // true or false
```

#### `ModelRegistry.is_cached_files(modelId, [options])`

Checks if all files for a given model are already cached, with per-file detail.
Automatically determines which files are needed using get_files().

**Parameters**

- `modelId` (`string`) — The model id
- `options` (`Object`) _optional_ — Optional parameters
  - `cache_dir` (`string`) _optional_ — Custom cache directory
  - `revision` (`string`) _optional_ — Model revision (default: 'main')
  - `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — Pre-loaded config
  - `dtype` ([`DataType`](../transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](../transformers#module_transformers.DataType)>) _optional_ — defaults to `null` — Override dtype
  - `device` ([`DeviceType`](../transformers#module_transformers.DeviceType) | `Record`<`string`, [`DeviceType`](../transformers#module_transformers.DeviceType)>) _optional_ — defaults to `null` — Override device

**Returns:** `Promise`<`CacheCheckResult`> — Object with allCached boolean and files array with cache status

**Example:**
```javascript
const status = await ModelRegistry.is_cached_files('onnx-community/bert-base-uncased-ONNX');
console.log(status.allCached); // true or false
console.log(status.files); // [{ file: 'config.json', cached: true }, ...]
```

#### `ModelRegistry.is_pipeline_cached(task, modelId, [options])`

Quickly checks if all files for a specific pipeline task are cached by verifying
`config.json` is present, then confirming all required files are cached.
Returns a plain boolean — use `is_pipeline_cached_files` if you need per-file detail.

**Parameters**

- `task` (`string`) — The pipeline task (e.g., "text-generation", "background-removal")
- `modelId` (`string`) — The model id
- `options` (`Object`) _optional_ — Optional parameters
  - `cache_dir` (`string`) _optional_ — Custom cache directory
  - `revision` (`string`) _optional_ — Model revision (default: 'main')
  - `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — Pre-loaded config
  - `dtype` ([`DataType`](../transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](../transformers#module_transformers.DataType)>) _optional_ — defaults to `null` — Override dtype
  - `device` ([`DeviceType`](../transformers#module_transformers.DeviceType) | `Record`<`string`, [`DeviceType`](../transformers#module_transformers.DeviceType)>) _optional_ — defaults to `null` — Override device

**Returns:** `Promise`<`boolean`> — Whether all required files are cached

**Example:**
```javascript
const cached = await ModelRegistry.is_pipeline_cached('text-generation', 'onnx-community/gpt2-ONNX');
console.log(cached); // true or false
```

#### `ModelRegistry.is_pipeline_cached_files(task, modelId, [options])`

Checks if all files for a specific pipeline task are already cached, with per-file detail.
Automatically determines which components are needed based on the task.

**Parameters**

- `task` (`string`) — The pipeline task (e.g., "text-generation", "background-removal")
- `modelId` (`string`) — The model id
- `options` (`Object`) _optional_ — Optional parameters
  - `cache_dir` (`string`) _optional_ — Custom cache directory
  - `revision` (`string`) _optional_ — Model revision (default: 'main')
  - `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — Pre-loaded config
  - `dtype` ([`DataType`](../transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](../transformers#module_transformers.DataType)>) _optional_ — defaults to `null` — Override dtype
  - `device` ([`DeviceType`](../transformers#module_transformers.DeviceType) | `Record`<`string`, [`DeviceType`](../transformers#module_transformers.DeviceType)>) _optional_ — defaults to `null` — Override device

**Returns:** `Promise`<`CacheCheckResult`> — Object with allCached boolean and files array with cache status

**Example:**
```javascript
const status = await ModelRegistry.is_pipeline_cached_files('text-generation', 'onnx-community/gpt2-ONNX');
console.log(status.allCached); // true or false
console.log(status.files); // [{ file: 'config.json', cached: true }, ...]
```

#### `ModelRegistry.get_file_metadata(path_or_repo_id, filename, [options])`

Get metadata for a specific file without downloading it.

**Parameters**

- `path_or_repo_id` (`string`) — Model id or path
- `filename` (`string`) — The file name
- `options` ([`PretrainedOptions`](./hub#module_utils/hub.PretrainedOptions)) _optional_ — Optional parameters

**Returns:** `Promise`<`{ exists: boolean, size?: number, contentType?: string, fromCache?: boolean }`> — File metadata. `exists: false` means the file was not found.

**Throws**

- `ModelFileNotFoundError` — If the file is missing or inaccessible. The Hub returns 401 for nonexistent repositories.
- `Error` — On network or server failures.

**Example:**
```javascript
const metadata = await ModelRegistry.get_file_metadata('onnx-community/gpt2-ONNX', 'config.json');
console.log(metadata.exists, metadata.size); // true, 665
```

#### `ModelRegistry.clear_cache(modelId, [options])`

Clears all cached files for a given model.
Automatically determines which files are needed and removes them from the cache.

**Parameters**

- `modelId` (`string`) — The model id (e.g., "onnx-community/gpt2-ONNX")
- `options` (`Object`) _optional_ — Optional parameters
  - `cache_dir` (`string`) _optional_ — Custom cache directory
  - `revision` (`string`) _optional_ — Model revision (default: 'main')
  - `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — Pre-loaded config
  - `dtype` ([`DataType`](../transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](../transformers#module_transformers.DataType)>) _optional_ — Override dtype
  - `device` ([`DeviceType`](../transformers#module_transformers.DeviceType) | `Record`<`string`, [`DeviceType`](../transformers#module_transformers.DeviceType)>) _optional_ — Override device
  - `include_tokenizer` (`boolean`) _optional_ — defaults to `true` — Whether to clear tokenizer files
  - `include_processor` (`boolean`) _optional_ — defaults to `true` — Whether to clear processor files

**Returns:** `Promise`<`CacheClearResult`> — Object with deletion statistics and file status

**Example:**
```javascript
const result = await ModelRegistry.clear_cache('onnx-community/bert-base-uncased-ONNX');
console.log(`Deleted ${result.filesDeleted} of ${result.filesCached} cached files`);
```

#### `ModelRegistry.clear_pipeline_cache(task, modelId, [options])`

Clears all cached files for a specific pipeline task.
Automatically determines which components are needed based on the task.

**Parameters**

- `task` (`string`) — The pipeline task (e.g., "text-generation", "image-classification")
- `modelId` (`string`) — The model id (e.g., "onnx-community/gpt2-ONNX")
- `options` (`Object`) _optional_ — Optional parameters
  - `cache_dir` (`string`) _optional_ — Custom cache directory
  - `revision` (`string`) _optional_ — Model revision (default: 'main')
  - `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — Pre-loaded config
  - `dtype` ([`DataType`](../transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](../transformers#module_transformers.DataType)>) _optional_ — Override dtype
  - `device` ([`DeviceType`](../transformers#module_transformers.DeviceType) | `Record`<`string`, [`DeviceType`](../transformers#module_transformers.DeviceType)>) _optional_ — Override device

**Returns:** `Promise`<`CacheClearResult`> — Object with deletion statistics and file status

**Example:**
```javascript
const result = await ModelRegistry.clear_pipeline_cache('text-generation', 'onnx-community/gpt2-ONNX');
console.log(`Deleted ${result.filesDeleted} of ${result.filesCached} cached files`);
```
