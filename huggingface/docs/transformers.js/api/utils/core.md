# utils/core

Core utility functions/classes for Transformers.js.

These are only used internally, meaning an end-user shouldn't
need to access anything here.

* [utils/core](#module_utils/core)
    * _static_
        * [.DefaultProgressCallback](#module_utils/core.DefaultProgressCallback)
            * [`new DefaultProgressCallback(callback, files_loading)`](#new_module_utils/core.DefaultProgressCallback_new)
            * [`._call(info)`](#module_utils/core.DefaultProgressCallback+_call)
        * [`.reverseDictionary(data)`](#module_utils/core.reverseDictionary) ⇒ Object
        * [`.escapeRegExp(string)`](#module_utils/core.escapeRegExp) ⇒ string
        * [`.isTypedArray(val)`](#module_utils/core.isTypedArray) ⇒ boolean
        * [`.isIntegralNumber(x)`](#module_utils/core.isIntegralNumber) ⇒ boolean
        * [`.isNullishDimension(x)`](#module_utils/core.isNullishDimension) ⇒ boolean
        * [`.calculateDimensions(arr)`](#module_utils/core.calculateDimensions) ⇒ Array
        * [`.pop(obj, key, defaultValue)`](#module_utils/core.pop) ⇒ *
        * [`.mergeArrays(arrs)`](#module_utils/core.mergeArrays) ⇒ Array
        * [`.calculateReflectOffset(i, w)`](#module_utils/core.calculateReflectOffset) ⇒ number
        * [`.pick(o, props)`](#module_utils/core.pick) ⇒ Object
        * [`.len(s)`](#module_utils/core.len) ⇒ number
        * [`.count(arr, value)`](#module_utils/core.count)
    * _inner_
        * [`~InitiateProgressInfo`](#module_utils/core..InitiateProgressInfo) : Object
        * [`~DownloadProgressInfo`](#module_utils/core..DownloadProgressInfo) : Object
        * [`~ProgressStatusInfo`](#module_utils/core..ProgressStatusInfo) : Object
        * [`~FileLoadingProgress`](#module_utils/core..FileLoadingProgress) : Object
        * [`~FilesLoadingMap`](#module_utils/core..FilesLoadingMap) : Record.&lt;string, FileLoadingProgress&gt;
        * [`~TotalProgressInfo`](#module_utils/core..TotalProgressInfo) : Object
        * [`~DoneProgressInfo`](#module_utils/core..DoneProgressInfo) : Object
        * [`~ReadyProgressInfo`](#module_utils/core..ReadyProgressInfo) : Object
        * [`~ProgressInfo`](#module_utils/core..ProgressInfo) : InitiateProgressInfo | DownloadProgressInfo | ProgressStatusInfo | DoneProgressInfo | ReadyProgressInfo | TotalProgressInfo
        * [`~ProgressCallback`](#module_utils/core..ProgressCallback) ⇒ void

* * *

## utils/core.DefaultProgressCallback

A callable progress callback that wraps an original callback and emits
aggregate `progress_total` events. Because it extends `Callable`, instances
can be passed directly wherever a plain callback function is expected.

Callers can check `callback instanceof DefaultProgressCallback` to avoid
double-wrapping when both `pipeline()` and `from_pretrained()` would
otherwise each add their own wrapper.

**Kind**: static class of [utils/core](#module_utils/core)  

* [.DefaultProgressCallback](#module_utils/core.DefaultProgressCallback)
    * [`new DefaultProgressCallback(callback, files_loading)`](#new_module_utils/core.DefaultProgressCallback_new)
    * [`._call(info)`](#module_utils/core.DefaultProgressCallback+_call)

* * *

### `new DefaultProgressCallback(callback, files_loading)`

  
    
      ParamTypeDescription
    
  
  

    callbackProgressCallbackThe original callback.

    
    files_loadingFilesLoadingMapMutable map storing per-file progress.

      

* * *

### `defaultProgressCallback._call(info)`

**Kind**: instance method of [DefaultProgressCallback](#module_utils/core.DefaultProgressCallback)  

  
    
      ParamType
    
  
  

    infoProgressInfo
      

* * *

## `utils/core.reverseDictionary(data)` ⇒ Object

Reverses the keys and values of an object.

**Kind**: static method of [utils/core](#module_utils/core)  
**Returns**: Object - The reversed object.  
**See**: https://ultimatecourses.com/blog/reverse-object-keys-and-values-in-javascript  

  
    
      ParamTypeDescription
    
  
  

    dataObjectThe object to reverse.

      

* * *

## `utils/core.escapeRegExp(string)` ⇒ string

Escapes regular expression special characters from a string by replacing them with their escaped counterparts.

**Kind**: static method of [utils/core](#module_utils/core)  
**Returns**: string - The escaped string.  

  
    
      ParamTypeDescription
    
  
  

    stringstringThe string to escape.

      

* * *

## `utils/core.isTypedArray(val)` ⇒ boolean

Check if a value is a typed array.

**Kind**: static method of [utils/core](#module_utils/core)  
**Returns**: boolean - True if the value is a `TypedArray`, false otherwise.

Adapted from https://stackoverflow.com/a/71091338/13989043  

  
    
      ParamTypeDescription
    
  
  

    val*The value to check.

      

* * *

## `utils/core.isIntegralNumber(x)` ⇒ boolean

Check if a value is an integer.

**Kind**: static method of [utils/core](#module_utils/core)  
**Returns**: boolean - True if the value is a string, false otherwise.  

  
    
      ParamTypeDescription
    
  
  

    x*The value to check.

      

* * *

## `utils/core.isNullishDimension(x)` ⇒ boolean

Determine if a provided width or height is nullish.

**Kind**: static method of [utils/core](#module_utils/core)  
**Returns**: boolean - True if the value is `null`, `undefined` or `-1`, false otherwise.  

  
    
      ParamTypeDescription
    
  
  

    x*The value to check.

      

* * *

## `utils/core.calculateDimensions(arr)` ⇒ Array

Calculates the dimensions of a nested array.

**Kind**: static method of [utils/core](#module_utils/core)  
**Returns**: Array - An array containing the dimensions of the input array.  

  
    
      ParamTypeDescription
    
  
  

    arrArrayThe nested array to calculate dimensions for.

      

* * *

## `utils/core.pop(obj, key, defaultValue)` ⇒ *

Replicate python's .pop() method for objects.

**Kind**: static method of [utils/core](#module_utils/core)  
**Returns**: * - The value of the popped key.  
**Throws**:

- Error If the key does not exist and no default value is provided.

  
    
      ParamTypeDescription
    
  
  

    objObjectThe object to pop from.

    
    keystringThe key to pop.

    
    defaultValue*The default value to return if the key does not exist.

      

* * *

## `utils/core.mergeArrays(arrs)` ⇒ Array

Efficiently merge arrays, creating a new copy.
Adapted from https://stackoverflow.com/a/6768642/13989043

**Kind**: static method of [utils/core](#module_utils/core)  
**Returns**: Array - The merged array.  

  
    
      ParamTypeDescription
    
  
  

    arrsArrayArrays to merge.

      

* * *

## `utils/core.calculateReflectOffset(i, w)` ⇒ number

Calculates the index offset for a given index and window size.

**Kind**: static method of [utils/core](#module_utils/core)  
**Returns**: number - The index offset.  

  
    
      ParamTypeDescription
    
  
  

    inumberThe index.

    
    wnumberThe window size.

      

* * *

## `utils/core.pick(o, props)` ⇒ Object

**Kind**: static method of [utils/core](#module_utils/core)  

  
    
      ParamType
    
  
  

    oObject
    
    propsArray
      

* * *

## `utils/core.len(s)` ⇒ number

Calculate the length of a string, taking multi-byte characters into account.
This mimics the behavior of Python's `len` function.

**Kind**: static method of [utils/core](#module_utils/core)  
**Returns**: number - The length of the string.  

  
    
      ParamTypeDescription
    
  
  

    sstringThe string to calculate the length of.

      

* * *

## `utils/core.count(arr, value)`

Count the occurrences of a value in an array or string.
This mimics the behavior of Python's `count` method.

**Kind**: static method of [utils/core](#module_utils/core)  

  
    
      ParamTypeDescription
    
  
  

    arrArray | stringThe array or string to search.

    
    valueanyThe value to count.

      

* * *

## `utils/core~InitiateProgressInfo` : Object

**Kind**: inner typedef of [utils/core](#module_utils/core)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    status&#x27;initiate&#x27;
    
    namestringThe model id or directory path.

    
    filestringThe name of the file.

      

* * *

## `utils/core~DownloadProgressInfo` : Object

**Kind**: inner typedef of [utils/core](#module_utils/core)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    status&#x27;download&#x27;
    
    namestringThe model id or directory path.

    
    filestringThe name of the file.

      

* * *

## `utils/core~ProgressStatusInfo` : Object

**Kind**: inner typedef of [utils/core](#module_utils/core)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    status&#x27;progress&#x27;
    
    namestringThe model id or directory path.

    
    filestringThe name of the file.

    
    progressnumberA number between 0 and 100.

    
    loadednumberThe number of bytes loaded.

    
    totalnumberThe total number of bytes to be loaded.

      

* * *

## `utils/core~FileLoadingProgress` : Object

**Kind**: inner typedef of [utils/core](#module_utils/core)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    loadednumberThe number of bytes loaded for this file.

    
    totalnumberThe total number of bytes for this file.

      

* * *

## `utils/core~FilesLoadingMap` : Record.&lt;string, FileLoadingProgress&gt;

A mapping of file names to their loading progress. Each key is a file path and each value contains
the loaded and total bytes for that file.

**Kind**: inner typedef of [utils/core](#module_utils/core)  

* * *

## `utils/core~TotalProgressInfo` : Object

**Kind**: inner typedef of [utils/core](#module_utils/core)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    status&#x27;progress_total&#x27;
    
    namestringThe model id or directory path.

    
    progressnumberA number between 0 and 100.

    
    loadednumberThe number of bytes loaded.

    
    totalnumberThe total number of bytes to be loaded.

    
    filesFilesLoadingMapA mapping of file names to their loading progress.

      

* * *

## `utils/core~DoneProgressInfo` : Object

**Kind**: inner typedef of [utils/core](#module_utils/core)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    status&#x27;done&#x27;
    
    namestringThe model id or directory path.

    
    filestringThe name of the file.

      

* * *

## `utils/core~ReadyProgressInfo` : Object

**Kind**: inner typedef of [utils/core](#module_utils/core)  
**Properties**

  
    
      NameTypeDescription
    
  
  

    status&#x27;ready&#x27;
    
    taskstringThe loaded task.

    
    modelstringThe loaded model.

      

* * *

## `utils/core~ProgressInfo` : InitiateProgressInfo | DownloadProgressInfo | ProgressStatusInfo | DoneProgressInfo | ReadyProgressInfo | TotalProgressInfo

**Kind**: inner typedef of [utils/core](#module_utils/core)  

* * *

## `utils/core~ProgressCallback` ⇒ void

A callback function that is called with progress information.

**Kind**: inner typedef of [utils/core](#module_utils/core)  

  
    
      ParamType
    
  
  

    progressInfoProgressInfo
      

* * *
