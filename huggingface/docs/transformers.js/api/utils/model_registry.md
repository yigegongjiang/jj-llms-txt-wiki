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
console.log(output[0].generated_text.at(-1).content); // ...\n\nThe capital of France is **Paris**.

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

* [utils/model_registry](#module_utils/model_registry)
    * [.ModelRegistry](#module_utils/model_registry.ModelRegistry)
        * [`.get_files(modelId, [options])`](#module_utils/model_registry.ModelRegistry.get_files) ⇒ Promise.&lt;Array&gt;
        * [`.get_pipeline_files(task, modelId, [options])`](#module_utils/model_registry.ModelRegistry.get_pipeline_files) ⇒ Promise.&lt;Array&gt;
        * [`.get_model_files(modelId, [options])`](#module_utils/model_registry.ModelRegistry.get_model_files) ⇒ Promise.&lt;Array&gt;
        * [`.get_tokenizer_files(modelId)`](#module_utils/model_registry.ModelRegistry.get_tokenizer_files) ⇒ Promise.&lt;Array&gt;
        * [`.get_processor_files(modelId)`](#module_utils/model_registry.ModelRegistry.get_processor_files) ⇒ Promise.&lt;Array&gt;
        * [`.get_available_dtypes(modelId, [options])`](#module_utils/model_registry.ModelRegistry.get_available_dtypes) ⇒ Promise.&lt;Array&gt;
        * [`.is_cached(modelId, [options])`](#module_utils/model_registry.ModelRegistry.is_cached) ⇒ Promise.&lt;boolean&gt;
        * [`.is_cached_files(modelId, [options])`](#module_utils/model_registry.ModelRegistry.is_cached_files) ⇒ [Promise.&lt;CacheCheckResult&gt;](#CacheCheckResult)
        * [`.is_pipeline_cached(task, modelId, [options])`](#module_utils/model_registry.ModelRegistry.is_pipeline_cached) ⇒ Promise.&lt;boolean&gt;
        * [`.is_pipeline_cached_files(task, modelId, [options])`](#module_utils/model_registry.ModelRegistry.is_pipeline_cached_files) ⇒ [Promise.&lt;CacheCheckResult&gt;](#CacheCheckResult)
        * [`.get_file_metadata(path_or_repo_id, filename, [options])`](#module_utils/model_registry.ModelRegistry.get_file_metadata) ⇒ Promise.&lt;{exists: boolean, size: number, contentType: string, fromCache: boolean}&gt;
        * [`.clear_cache(modelId, [options])`](#module_utils/model_registry.ModelRegistry.clear_cache) ⇒ [Promise.&lt;CacheClearResult&gt;](#CacheClearResult)
        * [`.clear_pipeline_cache(task, modelId, [options])`](#module_utils/model_registry.ModelRegistry.clear_pipeline_cache) ⇒ [Promise.&lt;CacheClearResult&gt;](#CacheClearResult)

* * *

## utils/model_registry.ModelRegistry

Static class for cache and file management operations.

**Kind**: static class of [utils/model_registry](#module_utils/model_registry)  

* [.ModelRegistry](#module_utils/model_registry.ModelRegistry)
    * [`.get_files(modelId, [options])`](#module_utils/model_registry.ModelRegistry.get_files) ⇒ Promise.&lt;Array&gt;
    * [`.get_pipeline_files(task, modelId, [options])`](#module_utils/model_registry.ModelRegistry.get_pipeline_files) ⇒ Promise.&lt;Array&gt;
    * [`.get_model_files(modelId, [options])`](#module_utils/model_registry.ModelRegistry.get_model_files) ⇒ Promise.&lt;Array&gt;
    * [`.get_tokenizer_files(modelId)`](#module_utils/model_registry.ModelRegistry.get_tokenizer_files) ⇒ Promise.&lt;Array&gt;
    * [`.get_processor_files(modelId)`](#module_utils/model_registry.ModelRegistry.get_processor_files) ⇒ Promise.&lt;Array&gt;
    * [`.get_available_dtypes(modelId, [options])`](#module_utils/model_registry.ModelRegistry.get_available_dtypes) ⇒ Promise.&lt;Array&gt;
    * [`.is_cached(modelId, [options])`](#module_utils/model_registry.ModelRegistry.is_cached) ⇒ Promise.&lt;boolean&gt;
    * [`.is_cached_files(modelId, [options])`](#module_utils/model_registry.ModelRegistry.is_cached_files) ⇒ [Promise.&lt;CacheCheckResult&gt;](#CacheCheckResult)
    * [`.is_pipeline_cached(task, modelId, [options])`](#module_utils/model_registry.ModelRegistry.is_pipeline_cached) ⇒ Promise.&lt;boolean&gt;
    * [`.is_pipeline_cached_files(task, modelId, [options])`](#module_utils/model_registry.ModelRegistry.is_pipeline_cached_files) ⇒ [Promise.&lt;CacheCheckResult&gt;](#CacheCheckResult)
    * [`.get_file_metadata(path_or_repo_id, filename, [options])`](#module_utils/model_registry.ModelRegistry.get_file_metadata) ⇒ Promise.&lt;{exists: boolean, size: number, contentType: string, fromCache: boolean}&gt;
    * [`.clear_cache(modelId, [options])`](#module_utils/model_registry.ModelRegistry.clear_cache) ⇒ [Promise.&lt;CacheClearResult&gt;](#CacheClearResult)
    * [`.clear_pipeline_cache(task, modelId, [options])`](#module_utils/model_registry.ModelRegistry.clear_pipeline_cache) ⇒ [Promise.&lt;CacheClearResult&gt;](#CacheClearResult)

* * *

### `ModelRegistry.get_files(modelId, [options])` ⇒ Promise.&lt;Array&gt;

Get all files (model, tokenizer, processor) needed for a model.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: Promise.&lt;Array&gt; - Array of file paths  

  
    
      ParamTypeDefaultDescription
    
  
  

    modelIdstringThe model id (e.g., &quot;onnx-community/bert-base-uncased-ONNX&quot;)

    
    [options]ObjectOptional parameters

    
    [options.config]PretrainedConfigPre-loaded config

    
    [options.dtype]DataType | Record.&lt;string, DataType&gt;Override dtype

    
    [options.device]DeviceType | Record.&lt;string, DeviceType&gt;Override device

    
    [options.model_file_name]stringnullOverride the model file name (excluding .onnx suffix)

    
    [options.include_tokenizer]booleantrueWhether to check for tokenizer files

    
    [options.include_processor]booleantrueWhether to check for processor files

      

**Example**  
```js
const files = await ModelRegistry.get_files('onnx-community/gpt2-ONNX');
console.log(files); // ['config.json', 'tokenizer.json', 'onnx/model_q4.onnx', ...]
```

* * *

### `ModelRegistry.get_pipeline_files(task, modelId, [options])` ⇒ Promise.&lt;Array&gt;

Get all files needed for a specific pipeline task.
Automatically determines which components are needed based on the task.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: Promise.&lt;Array&gt; - Array of file paths  

  
    
      ParamTypeDefaultDescription
    
  
  

    taskstringThe pipeline task (e.g., &quot;text-generation&quot;, &quot;background-removal&quot;)

    
    modelIdstringThe model id (e.g., &quot;onnx-community/bert-base-uncased-ONNX&quot;)

    
    [options]ObjectOptional parameters

    
    [options.config]PretrainedConfigPre-loaded config

    
    [options.dtype]DataType | Record.&lt;string, DataType&gt;Override dtype

    
    [options.device]DeviceType | Record.&lt;string, DeviceType&gt;Override device

    
    [options.model_file_name]stringnullOverride the model file name (excluding .onnx suffix)

      

**Example**  
```js
const files = await ModelRegistry.get_pipeline_files('text-generation', 'onnx-community/gpt2-ONNX');
console.log(files); // ['config.json', 'tokenizer.json', 'onnx/model_q4.onnx', ...]
```

* * *

### `ModelRegistry.get_model_files(modelId, [options])` ⇒ Promise.&lt;Array&gt;

Get model files needed for a specific model.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: Promise.&lt;Array&gt; - Array of model file paths  

  
    
      ParamTypeDefaultDescription
    
  
  

    modelIdstringThe model id

    
    [options]ObjectOptional parameters

    
    [options.config]PretrainedConfigPre-loaded config

    
    [options.dtype]DataType | Record.&lt;string, DataType&gt;Override dtype

    
    [options.device]DeviceType | Record.&lt;string, DeviceType&gt;Override device

    
    [options.model_file_name]stringnullOverride the model file name (excluding .onnx suffix)

      

**Example**  
```js
const files = await ModelRegistry.get_model_files('onnx-community/bert-base-uncased-ONNX');
console.log(files); // ['config.json', 'onnx/model_q4.onnx', 'generation_config.json']
```

* * *

### `ModelRegistry.get_tokenizer_files(modelId)` ⇒ Promise.&lt;Array&gt;

Get tokenizer files needed for a specific model.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: Promise.&lt;Array&gt; - Array of tokenizer file paths  

  
    
      ParamTypeDescription
    
  
  

    modelIdstringThe model id

      

**Example**  
```js
const files = await ModelRegistry.get_tokenizer_files('onnx-community/gpt2-ONNX');
console.log(files); // ['tokenizer.json', 'tokenizer_config.json']
```

* * *

### `ModelRegistry.get_processor_files(modelId)` ⇒ Promise.&lt;Array&gt;

Get processor files needed for a specific model.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: Promise.&lt;Array&gt; - Array of processor file paths  

  
    
      ParamTypeDescription
    
  
  

    modelIdstringThe model id

      

**Example**  
```js
const files = await ModelRegistry.get_processor_files('onnx-community/vit-base-patch16-224-ONNX');
console.log(files); // ['preprocessor_config.json']
```

* * *

### `ModelRegistry.get_available_dtypes(modelId, [options])` ⇒ Promise.&lt;Array&gt;

Detects which quantization levels (dtypes) are available for a model
by checking which ONNX files exist on the hub or locally.

A dtype is considered available if all required model session files
exist for that dtype.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: Promise.&lt;Array&gt; - Array of available dtype strings (e.g., ['fp32', 'fp16', 'q4', 'q8'])  

  
    
      ParamTypeDefaultDescription
    
  
  

    modelIdstringThe model id (e.g., &quot;onnx-community/all-MiniLM-L6-v2-ONNX&quot;)

    
    [options]ObjectOptional parameters

    
    [options.config]PretrainedConfigPre-loaded config

    
    [options.model_file_name]stringnullOverride the model file name (excluding .onnx suffix)

    
    [options.revision]string&quot;&#x27;main&#x27;&quot;Model revision

    
    [options.cache_dir]stringnullCustom cache directory

    
    [options.local_files_only]booleanfalseOnly check local files

      

**Example**  
```js
const dtypes = await ModelRegistry.get_available_dtypes('onnx-community/all-MiniLM-L6-v2-ONNX');
console.log(dtypes); // ['fp32', 'fp16', 'int8', 'uint8', 'q8', 'q4']
```

* * *

### `ModelRegistry.is_cached(modelId, [options])` ⇒ Promise.&lt;boolean&gt;

Quickly checks if a model is fully cached by verifying `config.json` is present,
then confirming all required files are cached.
Returns a plain boolean — use `is_cached_files` if you need per-file detail.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: Promise.&lt;boolean&gt; - Whether all required files are cached  

  
    
      ParamTypeDefaultDescription
    
  
  

    modelIdstringThe model id

    
    [options]ObjectOptional parameters

    
    [options.cache_dir]stringCustom cache directory

    
    [options.revision]stringModel revision (default: &#39;main&#39;)

    
    [options.config]PretrainedConfigPre-loaded config

    
    [options.dtype]DataType | Record.&lt;string, DataType&gt;Override dtype

    
    [options.device]DeviceType | Record.&lt;string, DeviceType&gt;Override device

      

**Example**  
```js
const cached = await ModelRegistry.is_cached('onnx-community/bert-base-uncased-ONNX');
console.log(cached); // true or false
```

* * *

### `ModelRegistry.is_cached_files(modelId, [options])` ⇒ [Promise.&lt;CacheCheckResult&gt;](#CacheCheckResult)

Checks if all files for a given model are already cached, with per-file detail.
Automatically determines which files are needed using get_files().

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: [Promise.&lt;CacheCheckResult&gt;](#CacheCheckResult) - Object with allCached boolean and files array with cache status  

  
    
      ParamTypeDefaultDescription
    
  
  

    modelIdstringThe model id

    
    [options]ObjectOptional parameters

    
    [options.cache_dir]stringCustom cache directory

    
    [options.revision]stringModel revision (default: &#39;main&#39;)

    
    [options.config]PretrainedConfigPre-loaded config

    
    [options.dtype]DataType | Record.&lt;string, DataType&gt;Override dtype

    
    [options.device]DeviceType | Record.&lt;string, DeviceType&gt;Override device

      

**Example**  
```js
const status = await ModelRegistry.is_cached_files('onnx-community/bert-base-uncased-ONNX');
console.log(status.allCached); // true or false
console.log(status.files); // [{ file: 'config.json', cached: true }, ...]
```

* * *

### `ModelRegistry.is_pipeline_cached(task, modelId, [options])` ⇒ Promise.&lt;boolean&gt;

Quickly checks if all files for a specific pipeline task are cached by verifying
`config.json` is present, then confirming all required files are cached.
Returns a plain boolean — use `is_pipeline_cached_files` if you need per-file detail.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: Promise.&lt;boolean&gt; - Whether all required files are cached  

  
    
      ParamTypeDefaultDescription
    
  
  

    taskstringThe pipeline task (e.g., &quot;text-generation&quot;, &quot;background-removal&quot;)

    
    modelIdstringThe model id

    
    [options]ObjectOptional parameters

    
    [options.cache_dir]stringCustom cache directory

    
    [options.revision]stringModel revision (default: &#39;main&#39;)

    
    [options.config]PretrainedConfigPre-loaded config

    
    [options.dtype]DataType | Record.&lt;string, DataType&gt;Override dtype

    
    [options.device]DeviceType | Record.&lt;string, DeviceType&gt;Override device

      

**Example**  
```js
const cached = await ModelRegistry.is_pipeline_cached('text-generation', 'onnx-community/gpt2-ONNX');
console.log(cached); // true or false
```

* * *

### `ModelRegistry.is_pipeline_cached_files(task, modelId, [options])` ⇒ [Promise.&lt;CacheCheckResult&gt;](#CacheCheckResult)

Checks if all files for a specific pipeline task are already cached, with per-file detail.
Automatically determines which components are needed based on the task.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: [Promise.&lt;CacheCheckResult&gt;](#CacheCheckResult) - Object with allCached boolean and files array with cache status  

  
    
      ParamTypeDefaultDescription
    
  
  

    taskstringThe pipeline task (e.g., &quot;text-generation&quot;, &quot;background-removal&quot;)

    
    modelIdstringThe model id

    
    [options]ObjectOptional parameters

    
    [options.cache_dir]stringCustom cache directory

    
    [options.revision]stringModel revision (default: &#39;main&#39;)

    
    [options.config]PretrainedConfigPre-loaded config

    
    [options.dtype]DataType | Record.&lt;string, DataType&gt;Override dtype

    
    [options.device]DeviceType | Record.&lt;string, DeviceType&gt;Override device

      

**Example**  
```js
const status = await ModelRegistry.is_pipeline_cached_files('text-generation', 'onnx-community/gpt2-ONNX');
console.log(status.allCached); // true or false
console.log(status.files); // [{ file: 'config.json', cached: true }, ...]
```

* * *

### `ModelRegistry.get_file_metadata(path_or_repo_id, filename, [options])` ⇒ Promise.&lt;{exists: boolean, size: number, contentType: string, fromCache: boolean}&gt;

Get metadata for a specific file without downloading it.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: Promise.&lt;{exists: boolean, size: number, contentType: string, fromCache: boolean}&gt; - File metadata  

  
    
      ParamTypeDescription
    
  
  

    path_or_repo_idstringModel id or path

    
    filenamestringThe file name

    
    [options]PretrainedOptionsOptional parameters

      

**Example**  
```js
const metadata = await ModelRegistry.get_file_metadata('onnx-community/gpt2-ONNX', 'config.json');
console.log(metadata.exists, metadata.size); // true, 665
```

* * *

### `ModelRegistry.clear_cache(modelId, [options])` ⇒ [Promise.&lt;CacheClearResult&gt;](#CacheClearResult)

Clears all cached files for a given model.
Automatically determines which files are needed and removes them from the cache.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: [Promise.&lt;CacheClearResult&gt;](#CacheClearResult) - Object with deletion statistics and file status  

  
    
      ParamTypeDefaultDescription
    
  
  

    modelIdstringThe model id (e.g., &quot;onnx-community/gpt2-ONNX&quot;)

    
    [options]ObjectOptional parameters

    
    [options.cache_dir]stringCustom cache directory

    
    [options.revision]stringModel revision (default: &#39;main&#39;)

    
    [options.config]PretrainedConfigPre-loaded config

    
    [options.dtype]DataType | Record.&lt;string, DataType&gt;Override dtype

    
    [options.device]DeviceType | Record.&lt;string, DeviceType&gt;Override device

    
    [options.include_tokenizer]booleantrueWhether to clear tokenizer files

    
    [options.include_processor]booleantrueWhether to clear processor files

      

**Example**  
```js
const result = await ModelRegistry.clear_cache('onnx-community/bert-base-uncased-ONNX');
console.log(`Deleted ${result.filesDeleted} of ${result.filesCached} cached files`);
```

* * *

### `ModelRegistry.clear_pipeline_cache(task, modelId, [options])` ⇒ [Promise.&lt;CacheClearResult&gt;](#CacheClearResult)

Clears all cached files for a specific pipeline task.
Automatically determines which components are needed based on the task.

**Kind**: static method of [ModelRegistry](#module_utils/model_registry.ModelRegistry)  
**Returns**: [Promise.&lt;CacheClearResult&gt;](#CacheClearResult) - Object with deletion statistics and file status  

  
    
      ParamTypeDescription
    
  
  

    taskstringThe pipeline task (e.g., &quot;text-generation&quot;, &quot;image-classification&quot;)

    
    modelIdstringThe model id (e.g., &quot;onnx-community/gpt2-ONNX&quot;)

    
    [options]ObjectOptional parameters

    
    [options.cache_dir]stringCustom cache directory

    
    [options.revision]stringModel revision (default: &#39;main&#39;)

    
    [options.config]PretrainedConfigPre-loaded config

    
    [options.dtype]DataType | Record.&lt;string, DataType&gt;Override dtype

    
    [options.device]DeviceType | Record.&lt;string, DeviceType&gt;Override device

      

**Example**  
```js
const result = await ModelRegistry.clear_pipeline_cache('text-generation', 'onnx-community/gpt2-ONNX');
console.log(`Deleted ${result.filesDeleted} of ${result.filesCached} cached files`);
```

* * *
