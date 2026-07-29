# utils/hub

Utility functions to interact with the Hugging Face Hub (https://huggingface.co/models)

* [utils/hub](#module_utils/hub)
    * _static_
        * [`.getFile(urlOrPath)`](#module_utils/hub.getFile) ⇒ Promise.&lt;(FileResponse|Response)&gt;
        * [`.getFetchHeaders(urlOrPath)`](#module_utils/hub.getFetchHeaders) ⇒ Headers
        * [`.buildResourcePaths(path_or_repo_id, filename, [options], [cache])`](#module_utils/hub.buildResourcePaths) ⇒ Object
        * [`.checkCachedResource(cache, localPath, proposedCacheKey)`](#module_utils/hub.checkCachedResource) ⇒ Promise.&lt;(Response|FileResponse|undefined|string)&gt;
        * [`.storeCachedResource(path_or_repo_id, filename, cache, cacheKey, response, [result], [options])`](#module_utils/hub.storeCachedResource) ⇒ Promise.&lt;void&gt;
        * [`.loadResourceFile(path_or_repo_id, filename, [fatal], [options], [return_path], [cache])`](#module_utils/hub.loadResourceFile) ⇒ Promise.&lt;(string|Uint8Array|null)&gt;
            * [`~cacheKey`](#module_utils/hub.loadResourceFile..cacheKey) : string
            * [`~response`](#module_utils/hub.loadResourceFile..response) : Response | FileResponse | undefined | string
            * [`~buffer`](#module_utils/hub.loadResourceFile..buffer) : Uint8Array
        * [`.getModelFile(path_or_repo_id, filename, [fatal], [options], [return_path])`](#module_utils/hub.getModelFile) ⇒ Promise.&lt;(string|Uint8Array)&gt;
            * [`~cache`](#module_utils/hub.getModelFile..cache) : [CacheInterface](#CacheInterface) | null
        * [`.getModelText(modelPath, fileName, [fatal], [options])`](#module_utils/hub.getModelText) ⇒ Promise.&lt;(string|null)&gt;
        * [`.getModelJSON(modelPath, fileName, [fatal], [options])`](#module_utils/hub.getModelJSON) ⇒ Promise.&lt;Object&gt;
    * _inner_
        * [`~INFLIGHT_LOADS`](#module_utils/hub..INFLIGHT_LOADS) : Map.&lt;string, Promise.&lt;(string|Uint8Array)&gt;&gt;
        * [`~ExternalData`](#module_utils/hub..ExternalData) : boolean | number
        * [`~PretrainedOptions`](#module_utils/hub..PretrainedOptions) : Object
        * [`~ModelSpecificPretrainedOptions`](#module_utils/hub..ModelSpecificPretrainedOptions) : Object
        * [`~PretrainedModelOptions`](#module_utils/hub..PretrainedModelOptions) : [PretrainedOptions](#PretrainedOptions) | ModelSpecificPretrainedOptions

* * *

## `utils/hub.getFile(urlOrPath)` ⇒ Promise.&lt;(FileResponse|Response)&gt;

Helper function to get a file, using either the Fetch API or FileSystem API.

**Kind**: static method of [utils/hub](#module_utils/hub)  
**Returns**: Promise.&lt;(FileResponse|Response)&gt; - A promise that resolves to a FileResponse object (if the file is retrieved using the FileSystem API), or a Response object (if the file is retrieved using the Fetch API).  

  
    
      ParamTypeDescription
    
  
  

    urlOrPathURL | stringThe URL/path of the file to get.

      

* * *

## `utils/hub.getFetchHeaders(urlOrPath)` ⇒ Headers

Generates appropriate HTTP headers for fetching resources.
In Node.js environments, adds User-Agent and Authorization headers when applicable.
In browser environments, returns minimal headers for security.

**Kind**: static method of [utils/hub](#module_utils/hub)  
**Returns**: Headers - A Headers object with appropriate headers for the request.  

  
    
      ParamTypeDescription
    
  
  

    urlOrPathURL | stringThe URL or path being fetched.

      

* * *

## `utils/hub.buildResourcePaths(path_or_repo_id, filename, [options], [cache])` ⇒ Object

Builds the resource paths and URLs for a model file.
Can be used to get the resource URL or path without loading the file.

**Kind**: static method of [utils/hub](#module_utils/hub)  
**Returns**: Object - An object containing all the paths and URLs for the resource.  

  
    
      ParamTypeDescription
    
  
  

    path_or_repo_idstringThis can be either:

a string, the model id of a model repo on huggingface.co.
a path to a directory potentially containing the file.

    
    filenamestringThe name of the file to locate.

    
    [options]PretrainedOptionsAn object containing optional parameters.

    
    [cache]CacheInterface | nullThe cache instance to use for determining cache keys.

      

* * *

## `utils/hub.checkCachedResource(cache, localPath, proposedCacheKey)` ⇒ Promise.&lt;(Response|FileResponse|undefined|string)&gt;

Checks if a resource exists in cache.

**Kind**: static method of [utils/hub](#module_utils/hub)  
**Returns**: Promise.&lt;(Response|FileResponse|undefined|string)&gt; - The cached response if found, undefined otherwise.  

  
    
      ParamTypeDescription
    
  
  

    cacheCacheInterface | nullThe cache instance to check.

    
    localPathstringThe local path to try first.

    
    proposedCacheKeystringThe proposed cache key to try second.

      

* * *

## `utils/hub.storeCachedResource(path_or_repo_id, filename, cache, cacheKey, response, [result], [options])` ⇒ Promise.&lt;void&gt;

Stores a resource in the cache.

**Kind**: static method of [utils/hub](#module_utils/hub)  

  
    
      ParamTypeDescription
    
  
  

    path_or_repo_idstringThe path or repo ID of the model.

    
    filenamestringThe name of the file to cache.

    
    cacheCacheInterfaceThe cache instance to store in.

    
    cacheKeystringThe cache key to use.

    
    responseResponse | FileResponseThe response to cache.

    
    [result]Uint8ArrayThe result buffer if already read.

    
    [options]PretrainedOptionsOptions containing progress callback and context for progress updates.

      

* * *

## `utils/hub.loadResourceFile(path_or_repo_id, filename, [fatal], [options], [return_path], [cache])` ⇒ Promise.&lt;(string|Uint8Array|null)&gt;

Loads a resource file from local or remote sources.

**Kind**: static method of [utils/hub](#module_utils/hub)  
**Returns**: Promise.&lt;(string|Uint8Array|null)&gt; - A Promise that resolves with the file content as a Uint8Array if `return_path` is false, or the file path as a string if `return_path` is true.  
**Throws**:

- Will throw an error if the file is not found and `fatal` is true.

  
    
      ParamTypeDefaultDescription
    
  
  

    path_or_repo_idstringThis can be either:

a string, the model id of a model repo on huggingface.co.
a path to a directory potentially containing the file.

    
    filenamestringThe name of the file to locate.

    
    [fatal]booleantrueWhether to throw an error if the file is not found.

    
    [options]PretrainedOptionsAn object containing optional parameters.

    
    [return_path]booleanfalseWhether to return the path of the file instead of the file content.

    
    [cache]CacheInterface | nullThe cache instance to use.

      

* [`.loadResourceFile(path_or_repo_id, filename, [fatal], [options], [return_path], [cache])`](#module_utils/hub.loadResourceFile) ⇒ Promise.&lt;(string|Uint8Array|null)&gt;
    * [`~cacheKey`](#module_utils/hub.loadResourceFile..cacheKey) : string
    * [`~response`](#module_utils/hub.loadResourceFile..response) : Response | FileResponse | undefined | string
    * [`~buffer`](#module_utils/hub.loadResourceFile..buffer) : Uint8Array

* * *

### `loadResourceFile~cacheKey` : string

**Kind**: inner property of [loadResourceFile](#module_utils/hub.loadResourceFile)  

* * *

### `loadResourceFile~response` : Response | FileResponse | undefined | string

**Kind**: inner property of [loadResourceFile](#module_utils/hub.loadResourceFile)  

* * *

### `loadResourceFile~buffer` : Uint8Array

**Kind**: inner property of [loadResourceFile](#module_utils/hub.loadResourceFile)  

* * *

## `utils/hub.getModelFile(path_or_repo_id, filename, [fatal], [options], [return_path])` ⇒ Promise.&lt;(string|Uint8Array)&gt;

Retrieves a file from either a remote URL using the Fetch API or from the local file system using the FileSystem API.
If the filesystem is available and `env.useCache = true`, the file will be downloaded and cached.

**Kind**: static method of [utils/hub](#module_utils/hub)  
**Returns**: Promise.&lt;(string|Uint8Array)&gt; - A Promise that resolves with the file content as a Uint8Array if `return_path` is false, or the file path as a string if `return_path` is true.  
**Throws**:

- Will throw an error if the file is not found and `fatal` is true.

  
    
      ParamTypeDefaultDescription
    
  
  

    path_or_repo_idstringThis can be either:

a string, the model id of a model repo on huggingface.co.
a path to a directory potentially containing the file.

    
    filenamestringThe name of the file to locate in path_or_repo.

    
    [fatal]booleantrueWhether to throw an error if the file is not found.

    
    [options]PretrainedOptionsAn object containing optional parameters.

    
    [return_path]booleanfalseWhether to return the path of the file instead of the file content.

      

* * *

### `getModelFile~cache` : [CacheInterface](#CacheInterface) | null

**Kind**: inner constant of [getModelFile](#module_utils/hub.getModelFile)  

* * *

## `utils/hub.getModelText(modelPath, fileName, [fatal], [options])` ⇒ Promise.&lt;(string|null)&gt;

Fetches a text file from a given path and file name.

**Kind**: static method of [utils/hub](#module_utils/hub)  
**Returns**: Promise.&lt;(string|null)&gt; - The text content of the file.  
**Throws**:

- Will throw an error if the file is not found and `fatal` is true.

  
    
      ParamTypeDefaultDescription
    
  
  

    modelPathstringThe path to the directory containing the file.

    
    fileNamestringThe name of the file to fetch.

    
    [fatal]booleantrueWhether to throw an error if the file is not found.

    
    [options]PretrainedOptionsAn object containing optional parameters.

      

* * *

## `utils/hub.getModelJSON(modelPath, fileName, [fatal], [options])` ⇒ Promise.&lt;Object&gt;

Fetches a JSON file from a given path and file name.

**Kind**: static method of [utils/hub](#module_utils/hub)  
**Returns**: Promise.&lt;Object&gt; - The JSON data parsed into a JavaScript object.  
**Throws**:

- Will throw an error if the file is not found and `fatal` is true.

  
    
      ParamTypeDefaultDescription
    
  
  

    modelPathstringThe path to the directory containing the file.

    
    fileNamestringThe name of the file to fetch.

    
    [fatal]booleantrueWhether to throw an error if the file is not found.

    
    [options]PretrainedOptionsAn object containing optional parameters.

      

* * *

## `utils/hub~INFLIGHT_LOADS` : Map.&lt;string, Promise.&lt;(string|Uint8Array)&gt;&gt;

In-flight file loads keyed by repo+filename.

**Kind**: inner constant of [utils/hub](#module_utils/hub)  

* * *

## `utils/hub~ExternalData` : boolean | number

Specifies whether to load the model using the external data format.
- `false`: Do not use external data format
- `true`: Use external data format with 1 chunk
- `number`: Use external data format with the specified number of chunks

**Kind**: inner typedef of [utils/hub](#module_utils/hub)  

* * *

## `utils/hub~PretrainedOptions` : Object

Options for loading a pretrained model.

**Kind**: inner typedef of [utils/hub](#module_utils/hub)  
**Properties**

  
    
      NameTypeDefaultDescription
    
  
  

    [progress_callback]ProgressCallbackIf specified, this function will be called during model construction, to provide the user with progress updates.

    
    [config]PretrainedConfigConfiguration for the model to use instead of an automatically loaded configuration. Configuration can be automatically loaded when:

The model is a model provided by the library (loaded with the model id string of a pretrained model).
The model is loaded by supplying a local directory as pretrained_model_name_or_path and a configuration JSON file named config.json is found in the directory.

    
    [cache_dir]stringnullPath to a directory in which a downloaded pretrained model configuration should be cached if the standard cache should not be used.

    
    [local_files_only]booleanfalseWhether or not to only look at local files (e.g., not try downloading the model).

    
    [revision]string&quot;&#x27;main&#x27;&quot;The specific model version to use. It can be a branch name, a tag name, or a commit id,
since we use a git-based system for storing models and other artifacts on huggingface.co, so revision can be any identifier allowed by git.
NOTE: This setting is ignored for local requests.

      

* * *

## `utils/hub~ModelSpecificPretrainedOptions` : Object

Options for loading a pretrained model.

**Kind**: inner typedef of [utils/hub](#module_utils/hub)  
**Properties**

  
    
      NameTypeDefaultDescription
    
  
  

    [subfolder]string&quot;&#x27;onnx&#x27;&quot;In case the relevant files are located inside a subfolder of the model repo on huggingface.co,
you can specify the folder name here.

    
    [model_file_name]stringnullIf specified, load the model with this name (excluding the dtype and .onnx suffixes). Currently only valid for encoder- or decoder-only models.

    
    [device]DeviceType | Record.&lt;string, DeviceType&gt;The device to run the model on. If not specified, the device will be chosen from the environment settings.

    
    [dtype]DataType | Record.&lt;string, DataType&gt;The data type to use for the model. If not specified, the data type will be chosen from the environment settings.

    
    [use_external_data_format]ExternalData | Record.&lt;string, ExternalData&gt;falseWhether to load the model using the external data format (used for models &gt;= 2GB in size).

    
    [session_options]InferenceSession.SessionOptions(Optional) User-specified session options passed to the runtime. If not provided, suitable defaults will be chosen.

      

* * *

## `utils/hub~PretrainedModelOptions` : [PretrainedOptions](#PretrainedOptions) | ModelSpecificPretrainedOptions

Options for loading a pretrained model.

**Kind**: inner typedef of [utils/hub](#module_utils/hub)  

* * *
