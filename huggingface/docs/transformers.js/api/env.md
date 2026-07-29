# env

Module used to configure Transformers.js.

**Example:** Disable remote models.
```javascript
import { env } from '@huggingface/transformers';
env.allowRemoteModels = false;
```

**Example:** Set local model path.
```javascript
import { env } from '@huggingface/transformers';
env.localModelPath = '/path/to/local/models/';
```

**Example:** Set cache directory.
```javascript
import { env } from '@huggingface/transformers';
env.cacheDir = '/path/to/cache/directory/';
```

* [env](#module_env)
    * _static_
        * [`.apis`](#module_env.apis)
        * [`.LogLevel`](#module_env.LogLevel)
        * [`.env`](#module_env.env) : TransformersEnvironment
    * _inner_
        * [`~IS_BROWSER_ENV`](#module_env..IS_BROWSER_ENV)
        * [`~IS_WEBWORKER_ENV`](#module_env..IS_WEBWORKER_ENV)
        * [`~IS_WEB_ENV`](#module_env..IS_WEB_ENV)
        * [`~IS_SERVICE_WORKER_ENV`](#module_env..IS_SERVICE_WORKER_ENV)
        * [`~IS_DENO_WEB_RUNTIME`](#module_env..IS_DENO_WEB_RUNTIME)
        * [`~IS_WEB_CACHE_AVAILABLE`](#module_env..IS_WEB_CACHE_AVAILABLE)
        * [`~IS_WEBGPU_AVAILABLE`](#module_env..IS_WEBGPU_AVAILABLE)
        * [`~IS_WEBNN_AVAILABLE`](#module_env..IS_WEBNN_AVAILABLE)
        * [`~IS_SAFARI`](#module_env..IS_SAFARI)
        * [`~IS_PROCESS_AVAILABLE`](#module_env..IS_PROCESS_AVAILABLE)
        * [`~IS_NODE_ENV`](#module_env..IS_NODE_ENV)
        * [`~IS_FS_AVAILABLE`](#module_env..IS_FS_AVAILABLE)
        * [`~IS_PATH_AVAILABLE`](#module_env..IS_PATH_AVAILABLE)
        * [`~IS_CRYPTO_AVAILABLE`](#module_env..IS_CRYPTO_AVAILABLE)
        * [`~IS_CHROME_AVAILABLE`](#module_env..IS_CHROME_AVAILABLE)
        * [`~DEBUG`](#module_env..DEBUG)
        * [`~INFO`](#module_env..INFO)
        * [`~WARNING`](#module_env..WARNING)
        * [`~ERROR`](#module_env..ERROR)
        * [`~NONE`](#module_env..NONE)
        * [`~isSafari()`](#module_env..isSafari) ⇒ boolean
        * [`~TransformersEnvironment`](#module_env..TransformersEnvironment) : Object

* * *

## `env.apis`

A read-only object containing information about the APIs available in the current environment.

**Kind**: static constant of [env](#module_env)  

* * *

## `env.LogLevel`

Log levels for controlling output verbosity.

Each level is represented by a number, where higher numbers include all lower level messages.
Use these values to set `env.logLevel`.

**Kind**: static constant of [env](#module_env)  
**Example**  
```js
import { env, LogLevel } from '@huggingface/transformers';

// Set log level to show only errors
env.logLevel = LogLevel.ERROR;

// Set log level to show errors, warnings, and info
env.logLevel = LogLevel.INFO;

// Disable all logging
env.logLevel = LogLevel.NONE;
```

* * *

## `env.env` : TransformersEnvironment

**Kind**: static constant of [env](#module_env)  

* * *

## `env~IS_BROWSER_ENV`

Whether we are running in a browser environment (and not a web worker)

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_WEBWORKER_ENV`

Whether we are running in a web worker environment

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_WEB_ENV`

Whether we are running in a web-like environment (browser, web worker, or Deno web runtime)

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_SERVICE_WORKER_ENV`

Whether we are running in a service worker environment

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_DENO_WEB_RUNTIME`

Whether we are running in Deno's web runtime (CDN imports, Cache API available, no filesystem)

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_WEB_CACHE_AVAILABLE`

Whether the Cache API is available

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_WEBGPU_AVAILABLE`

Whether the WebGPU API is available

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_WEBNN_AVAILABLE`

Whether the WebNN API is available

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_SAFARI`

Whether we are running in a Safari browser

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_PROCESS_AVAILABLE`

Whether the Node.js process API is available

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_NODE_ENV`

Whether we are running in a Node.js-like environment (node, deno, bun)

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_FS_AVAILABLE`

Whether the filesystem API is available

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_PATH_AVAILABLE`

Whether the path API is available

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_CRYPTO_AVAILABLE`

Whether the crypto API is available

**Kind**: inner property of [env](#module_env)  

* * *

## `env~IS_CHROME_AVAILABLE`

Whether the Chrome runtime API is available

**Kind**: inner property of [env](#module_env)  

* * *

## `env~DEBUG`

All messages including debug output (value: 10)

**Kind**: inner property of [env](#module_env)  

* * *

## `env~INFO`

Errors, warnings, and info messages (value: 20)

**Kind**: inner property of [env](#module_env)  

* * *

## `env~WARNING`

Errors and warnings (value: 30)

**Kind**: inner property of [env](#module_env)  

* * *

## `env~ERROR`

Only error messages (value: 40)

**Kind**: inner property of [env](#module_env)  

* * *

## `env~NONE`

No logging output (value: 50)

**Kind**: inner property of [env](#module_env)  

* * *

## `env~isSafari()` ⇒ boolean

Check if the current environment is Safari browser.
Works in both browser and web worker contexts.

**Kind**: inner method of [env](#module_env)  
**Returns**: boolean - Whether the current environment is Safari.  

* * *

## `env~TransformersEnvironment` : Object

Global variable given visible to users to control execution. This provides users a simple way to configure Transformers.js.

**Kind**: inner typedef of [env](#module_env)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    versionstringThis version of Transformers.js.

    
    backendsObjectExpose environment variables of different backends,
allowing users to set these variables if they want to.

    
    logLevelnumberThe logging level. Use LogLevel enum values. Defaults to LogLevel.ERROR.

    
    allowRemoteModelsbooleanWhether to allow loading of remote files, defaults to true.
If set to false, it will have the same effect as setting local_files_only=true when loading pipelines, models, tokenizers, processors, etc.

    
    remoteHoststringHost URL to load models from. Defaults to the Hugging Face Hub.

    
    remotePathTemplatestringPath template to fill in and append to remoteHost when loading models.

    
    allowLocalModelsbooleanWhether to allow loading of local files, defaults to false if running in-browser, and true otherwise.
If set to false, it will skip the local file check and try to load the model from the remote host.

    
    localModelPathstringPath to load local models from. Defaults to /models/.

    
    useFSbooleanWhether to use the file system to load files. By default, it is true if available.

    
    useBrowserCachebooleanWhether to use Cache API to cache models. By default, it is true if available.

    
    useFSCachebooleanWhether to use the file system to cache files. By default, it is true if available.

    
    cacheDirstring | nullThe directory to use for caching files with the file system. By default, it is ./.cache.

    
    useCustomCachebooleanWhether to use a custom cache system (defined by customCache), defaults to false.

    
    customCacheCacheInterface | nullThe custom cache to use. Defaults to null. Note: this must be an object which
implements the match and put functions of the Web Cache API. For more information, see https://developer.mozilla.org/en-US/docs/Web/API/Cache.

    
    useWasmCachebooleanWhether to pre-load and cache WASM binaries and the WASM factory (.mjs) for ONNX Runtime.
Defaults to true when cache is available. This can improve performance and enables offline usage by avoiding repeated downloads.

    
    cacheKeystringThe cache key to use for storing models and WASM binaries. Defaults to &#39;transformers-cache&#39;.

    
    experimental_useCrossOriginStoragebooleanWhether to use the Cross-Origin Storage API to cache model files
across origins, allowing different sites to share the same cached model weights. Defaults to false.
Requires the Cross-Origin Storage Chrome extension: https://chromewebstore.google.com/detail/cross-origin-storage/denpnpcgjgikjpoglpjefakmdcbmlgih.
The experimental_ prefix indicates that the underlying browser API is not yet standardised and may change or be
removed without a major version bump. For more information, see https://github.com/WICG/cross-origin-storage.

    
    fetchfunctionThe fetch function to use. Defaults to fetch.

      

* * *
