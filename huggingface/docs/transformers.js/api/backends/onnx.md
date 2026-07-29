# backends/onnx

Handler file for choosing the correct version of ONNX Runtime, based on the environment.
Ideally, we could import the `onnxruntime-web` and `onnxruntime-node` packages only when needed,
but dynamic imports don't seem to work with the current webpack version and/or configuration.
This is possibly due to the experimental nature of top-level await statements.
So, we just import both packages, and use the appropriate one based on the environment:
  - When running in node, we use `onnxruntime-node`.
  - When running in the browser, we use `onnxruntime-web` (`onnxruntime-node` is not bundled).

This module is not directly exported, but can be accessed through the environment variables:
```javascript
import { env } from '@huggingface/transformers';
console.log(env.backends.onnx);
```

* [backends/onnx](#module_backends/onnx)
    * _static_
        * [`.deviceToExecutionProviders([device])`](#module_backends/onnx.deviceToExecutionProviders) ⇒ Array
        * [`.createInferenceSession(buffer_or_path, session_options, session_config)`](#module_backends/onnx.createInferenceSession) ⇒ Promise.&lt;(InferenceSession|{config: Object})&gt;
        * [`.runInferenceSession(session, ortFeed)`](#module_backends/onnx.runInferenceSession) ⇒ Promise.&lt;Record.&lt;string, Tensor&gt;&gt;
        * [`.isONNXTensor(x)`](#module_backends/onnx.isONNXTensor) ⇒ boolean
        * [`.isONNXProxy()`](#module_backends/onnx.isONNXProxy) ⇒ boolean
    * _inner_
        * [`~defaultDevices`](#module_backends/onnx..defaultDevices) : Array
        * [`~webInitChain`](#module_backends/onnx..webInitChain) : Promise.&lt;any&gt;
        * [`~wasmLoadPromise`](#module_backends/onnx..wasmLoadPromise) : Promise.&lt;void&gt; | null
        * [`~webInferenceChain`](#module_backends/onnx..webInferenceChain) : Promise.&lt;any&gt;
        * [`~DEVICE_TO_EXECUTION_PROVIDER_MAPPING`](#module_backends/onnx..DEVICE_TO_EXECUTION_PROVIDER_MAPPING) : Record.&lt;DeviceType, ONNXExecutionProviders&gt;
        * [`~ONNX_LOG_LEVEL_NAMES`](#module_backends/onnx..ONNX_LOG_LEVEL_NAMES) : Record.&lt;(0|1|2|3|4), (&#x27;verbose&#x27;|&#x27;info&#x27;|&#x27;warning&#x27;|&#x27;error&#x27;|&#x27;fatal&#x27;)&gt;
        * [`~supportedDevices`](#module_backends/onnx..supportedDevices) : Array
        * [`~ONNX_ENV`](#module_backends/onnx..ONNX_ENV) : Env
        * [`~getOnnxLogSeverityLevel(logLevel)`](#module_backends/onnx..getOnnxLogSeverityLevel) ⇒ number
        * [`~ensureWasmLoaded()`](#module_backends/onnx..ensureWasmLoaded) ⇒ Promise.&lt;void&gt;
        * [`~setLogLevel(logLevel)`](#module_backends/onnx..setLogLevel)
        * [`~ONNXExecutionProviders`](#module_backends/onnx..ONNXExecutionProviders) : InferenceSession.ExecutionProviderConfig

* * *

## `backends/onnx.deviceToExecutionProviders([device])` ⇒ Array

Map a device to the execution providers to use for the given device.

**Kind**: static method of [backends/onnx](#module_backends/onnx)  
**Returns**: Array - The execution providers to use for the given device.  

  
    
      ParamTypeDefaultDescription
    
  
  

    [device]DeviceType | &quot;auto&quot; | null(Optional) The device to run the inference on.

      

* * *

## `backends/onnx.createInferenceSession(buffer_or_path, session_options, session_config)` ⇒ Promise.&lt;(InferenceSession|{config: Object})&gt;

Create an ONNX inference session.

**Kind**: static method of [backends/onnx](#module_backends/onnx)  
**Returns**: Promise.&lt;(InferenceSession|{config: Object})&gt; - The ONNX inference session.  

  
    
      ParamTypeDescription
    
  
  

    buffer_or_pathUint8Array | stringThe ONNX model buffer or path.

    
    session_optionsInferenceSession.SessionOptionsONNX inference session options.

    
    session_configObjectONNX inference session configuration.

      

* * *

## `backends/onnx.runInferenceSession(session, ortFeed)` ⇒ Promise.&lt;Record.&lt;string, Tensor&gt;&gt;

Run an inference session.

**Kind**: static method of [backends/onnx](#module_backends/onnx)  
**Returns**: Promise.&lt;Record.&lt;string, Tensor&gt;&gt; - The output tensors.  

  
    
      ParamTypeDescription
    
  
  

    sessionInferenceSessionThe ONNX inference session.

    
    ortFeedRecord.&lt;string, Tensor&gt;The input tensors.

      

* * *

## `backends/onnx.isONNXTensor(x)` ⇒ boolean

Check if an object is an ONNX tensor.

**Kind**: static method of [backends/onnx](#module_backends/onnx)  
**Returns**: boolean - Whether the object is an ONNX tensor.  

  
    
      ParamTypeDescription
    
  
  

    xanyThe object to check

      

* * *

## `backends/onnx.isONNXProxy()` ⇒ boolean

Check if ONNX's WASM backend is being proxied.

**Kind**: static method of [backends/onnx](#module_backends/onnx)  
**Returns**: boolean - Whether ONNX's WASM backend is being proxied.  

* * *

## `backends/onnx~defaultDevices` : Array

**Kind**: inner property of [backends/onnx](#module_backends/onnx)  

* * *

## `backends/onnx~webInitChain` : Promise.&lt;any&gt;

Currently, Transformers.js doesn't support simultaneous loading of sessions in WASM/WebGPU.
For this reason, we need to chain the loading calls.

**Kind**: inner property of [backends/onnx](#module_backends/onnx)  

* * *

## `backends/onnx~wasmLoadPromise` : Promise.&lt;void&gt; | null

Promise that resolves when WASM binary has been loaded (if caching is enabled).
This ensures we only attempt to load the WASM binary once.

**Kind**: inner property of [backends/onnx](#module_backends/onnx)  

* * *

## `backends/onnx~webInferenceChain` : Promise.&lt;any&gt;

Currently, Transformers.js doesn't support simultaneous execution of sessions in WASM/WebGPU.
For this reason, we need to chain the inference calls (otherwise we get "Error: Session already started").

**Kind**: inner property of [backends/onnx](#module_backends/onnx)  

* * *

## `backends/onnx~DEVICE_TO_EXECUTION_PROVIDER_MAPPING` : Record.&lt;DeviceType, ONNXExecutionProviders&gt;

**Kind**: inner constant of [backends/onnx](#module_backends/onnx)  

* * *

## `backends/onnx~ONNX_LOG_LEVEL_NAMES` : Record.&lt;(0|1|2|3|4), (&#x27;verbose&#x27;|&#x27;info&#x27;|&#x27;warning&#x27;|&#x27;error&#x27;|&#x27;fatal&#x27;)&gt;

Maps ONNX Runtime numeric severity levels to string log levels.

**Kind**: inner constant of [backends/onnx](#module_backends/onnx)  

* * *

## `backends/onnx~supportedDevices` : Array

The list of supported devices, sorted by priority/performance.

**Kind**: inner constant of [backends/onnx](#module_backends/onnx)  

* * *

## `backends/onnx~ONNX_ENV` : Env

**Kind**: inner constant of [backends/onnx](#module_backends/onnx)  

* * *

## `backends/onnx~getOnnxLogSeverityLevel(logLevel)` ⇒ number

Converts any LogLevel value to ONNX Runtime's numeric severity level (0-4).
This handles both standard LogLevel values (10, 20, 30, 40, 50) and custom intermediate values.

**Kind**: inner method of [backends/onnx](#module_backends/onnx)  
**Returns**: number - ONNX Runtime severity level (0-4)  

  
    
      ParamTypeDescription
    
  
  

    logLevelnumberThe LogLevel value to convert

      

* * *

## `backends/onnx~ensureWasmLoaded()` ⇒ Promise.&lt;void&gt;

Ensures the WASM binary is loaded and cached before creating an inference session.
Only runs once, even if called multiple times.

**Kind**: inner method of [backends/onnx](#module_backends/onnx)  

* * *

## `backends/onnx~setLogLevel(logLevel)`

A function to map Transformers.js log levels to ONNX Runtime log severity
levels, and set the log level environment variable in ONNX Runtime.

**Kind**: inner method of [backends/onnx](#module_backends/onnx)  

  
    
      ParamTypeDescription
    
  
  

    logLevelnumberThe log level to set.

      

* * *

## `backends/onnx~ONNXExecutionProviders` : InferenceSession.ExecutionProviderConfig

**Kind**: inner typedef of [backends/onnx](#module_backends/onnx)  

* * *
