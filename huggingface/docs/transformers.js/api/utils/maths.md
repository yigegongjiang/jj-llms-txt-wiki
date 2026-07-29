# utils/maths

Helper module for mathematical processing.

These functions and classes are only used internally,
meaning an end-user shouldn't need to access anything here.

* [utils/maths](#module_utils/maths)
    * _static_
        * [`.uint16_to_float32`](#module_utils/maths.uint16_to_float32)
        * [`.interpolate_data(input)`](#module_utils/maths.interpolate_data)
        * [`.permute_data(array, dims, axes)`](#module_utils/maths.permute_data) ⇒ Array
        * [`.softmax(arr)`](#module_utils/maths.softmax) ⇒ T
        * [`.log_softmax(arr)`](#module_utils/maths.log_softmax) ⇒ T
        * [`.dot(arr1, arr2)`](#module_utils/maths.dot) ⇒ number
        * [`.cos_sim(arr1, arr2)`](#module_utils/maths.cos_sim) ⇒ number
        * [`.min(arr)`](#module_utils/maths.min) ⇒ any
        * [`.max(arr)`](#module_utils/maths.max) ⇒ any
        * [`.medianFilter(data, windowSize)`](#module_utils/maths.medianFilter)
        * [`.round(num, decimals)`](#module_utils/maths.round) ⇒ number
        * [`.bankers_round(x)`](#module_utils/maths.bankers_round) ⇒ number
        * [`.dynamic_time_warping(matrix)`](#module_utils/maths.dynamic_time_warping) ⇒ Array
    * _inner_
        * [~P2FFT](#module_utils/maths..P2FFT)
            * [`new P2FFT(size)`](#new_module_utils/maths..P2FFT_new)
            * [`.createComplexArray()`](#module_utils/maths..P2FFT+createComplexArray) ⇒ Float64Array
            * [`.fromComplexArray(complex, [storage])`](#module_utils/maths..P2FFT+fromComplexArray) ⇒ Array
            * [`.toComplexArray(input, [storage])`](#module_utils/maths..P2FFT+toComplexArray) ⇒ Float64Array
            * [`.transform(out, data)`](#module_utils/maths..P2FFT+transform) ⇒ void
            * [`.realTransform(out, data)`](#module_utils/maths..P2FFT+realTransform)
            * [`.inverseTransform(out, data)`](#module_utils/maths..P2FFT+inverseTransform) ⇒ void
            * [`._transform4(out, data, inv)`](#module_utils/maths..P2FFT+_transform4) ⇒ void
            * [`._singleTransform2(data, out, outOff, off, step)`](#module_utils/maths..P2FFT+_singleTransform2) ⇒ void
            * [`._singleTransform4(data, out, outOff, off, step, inv)`](#module_utils/maths..P2FFT+_singleTransform4) ⇒ void
            * [`._realTransform4(out, data, inv)`](#module_utils/maths..P2FFT+_realTransform4)
            * [`._singleRealTransform2(data, out, outOff, off, step)`](#module_utils/maths..P2FFT+_singleRealTransform2) ⇒ void
            * [`._singleRealTransform4(data, out, outOff, off, step, inv)`](#module_utils/maths..P2FFT+_singleRealTransform4)
        * [~NP2FFT](#module_utils/maths..NP2FFT)
            * [`new NP2FFT(fft_length)`](#new_module_utils/maths..NP2FFT_new)
        * [`~magnitude(arr)`](#module_utils/maths..magnitude) ⇒ number
        * [`~AnyTypedArray`](#module_utils/maths..AnyTypedArray) : Int8Array | Uint8Array | Uint8ClampedArray | Int16Array | Uint16Array | Int32Array | Uint32Array | Float16Array | Float32Array | Float64Array

* * *

## `utils/maths.uint16_to_float32`

Efficiently converts a Uint16Array of float16 values to a Float32Array.
This implementation uses a lazily initialized lookup table (LUT) for fast conversion.

**Kind**: static constant of [utils/maths](#module_utils/maths)  

* * *

## `utils/maths.interpolate_data(input)`

**Kind**: static method of [utils/maths](#module_utils/maths)  

  
    
      ParamType
    
  
  

    inputTypedArray
      

* * *

## `utils/maths.permute_data(array, dims, axes)` ⇒ Array

Helper method to permute a `AnyTypedArray` directly

**Kind**: static method of [utils/maths](#module_utils/maths)  
**Returns**: Array - The permuted array and the new shape.  

  
    
      ParamType
    
  
  

    arrayT
    
    dimsArray
    
    axesArray
      

* * *

## `utils/maths.softmax(arr)` ⇒ T

Compute the softmax of an array of numbers.

**Kind**: static method of [utils/maths](#module_utils/maths)  
**Returns**: T - The softmax array.  

  
    
      ParamTypeDescription
    
  
  

    arrTThe array of numbers to compute the softmax of.

      

* * *

## `utils/maths.log_softmax(arr)` ⇒ T

Calculates the logarithm of the softmax function for the input array.

**Kind**: static method of [utils/maths](#module_utils/maths)  
**Returns**: T - The resulting log_softmax array.  

  
    
      ParamTypeDescription
    
  
  

    arrTThe input array to calculate the log_softmax function for.

      

* * *

## `utils/maths.dot(arr1, arr2)` ⇒ number

Calculates the dot product of two arrays.

**Kind**: static method of [utils/maths](#module_utils/maths)  
**Returns**: number - The dot product of arr1 and arr2.  

  
    
      ParamTypeDescription
    
  
  

    arr1ArrayThe first array.

    
    arr2ArrayThe second array.

      

* * *

## `utils/maths.cos_sim(arr1, arr2)` ⇒ number

Computes the cosine similarity between two arrays.

**Kind**: static method of [utils/maths](#module_utils/maths)  
**Returns**: number - The cosine similarity between the two arrays.  

  
    
      ParamTypeDescription
    
  
  

    arr1ArrayThe first array.

    
    arr2ArrayThe second array.

      

* * *

## `utils/maths.min(arr)` ⇒ any

Returns the value and index of the minimum element in an array.

**Kind**: static method of [utils/maths](#module_utils/maths)  
**Returns**: any - the value and index of the minimum element, of the form: [valueOfMin, indexOfMin]  
**Throws**:

- Error If array is empty.

  
    
      ParamTypeDescription
    
  
  

    arrTarray of numbers.

      

* * *

## `utils/maths.max(arr)` ⇒ any

Returns the value and index of the maximum element in an array.

**Kind**: static method of [utils/maths](#module_utils/maths)  
**Returns**: any - the value and index of the maximum element, of the form: [valueOfMax, indexOfMax]  
**Throws**:

- Error If array is empty.

  
    
      ParamTypeDescription
    
  
  

    arrTarray of numbers.

      

* * *

## `utils/maths.medianFilter(data, windowSize)`

Performs median filter on the provided data. Padding is done by mirroring the data.

**Kind**: static method of [utils/maths](#module_utils/maths)  

  
    
      ParamTypeDescription
    
  
  

    dataAnyTypedArrayThe input array

    
    windowSizenumberThe window size

      

* * *

## `utils/maths.round(num, decimals)` ⇒ number

Helper function to round a number to a given number of decimals

**Kind**: static method of [utils/maths](#module_utils/maths)  
**Returns**: number - The rounded number  

  
    
      ParamTypeDescription
    
  
  

    numnumberThe number to round

    
    decimalsnumberThe number of decimals

      

* * *

## `utils/maths.bankers_round(x)` ⇒ number

Helper function to round a number to the nearest integer, with ties rounded to the nearest even number.
Also known as "bankers' rounding". This is the default rounding mode in python. For example:
1.5 rounds to 2 and 2.5 rounds to 2.

**Kind**: static method of [utils/maths](#module_utils/maths)  
**Returns**: number - The rounded number  

  
    
      ParamTypeDescription
    
  
  

    xnumberThe number to round

      

* * *

## `utils/maths.dynamic_time_warping(matrix)` ⇒ Array

Measures similarity between two temporal sequences (e.g., input audio and output tokens
to generate token-level timestamps).

**Kind**: static method of [utils/maths](#module_utils/maths)  

  
    
      ParamType
    
  
  

    matrixArray
      

* * *

## utils/maths~P2FFT

Implementation of Radix-4 FFT.

P2FFT class provides functionality for performing Fast Fourier Transform on arrays
which are a power of two in length.
Code adapted from https://www.npmjs.com/package/fft.js

**Kind**: inner class of [utils/maths](#module_utils/maths)  

* [~P2FFT](#module_utils/maths..P2FFT)
    * [`new P2FFT(size)`](#new_module_utils/maths..P2FFT_new)
    * [`.createComplexArray()`](#module_utils/maths..P2FFT+createComplexArray) ⇒ Float64Array
    * [`.fromComplexArray(complex, [storage])`](#module_utils/maths..P2FFT+fromComplexArray) ⇒ Array
    * [`.toComplexArray(input, [storage])`](#module_utils/maths..P2FFT+toComplexArray) ⇒ Float64Array
    * [`.transform(out, data)`](#module_utils/maths..P2FFT+transform) ⇒ void
    * [`.realTransform(out, data)`](#module_utils/maths..P2FFT+realTransform)
    * [`.inverseTransform(out, data)`](#module_utils/maths..P2FFT+inverseTransform) ⇒ void
    * [`._transform4(out, data, inv)`](#module_utils/maths..P2FFT+_transform4) ⇒ void
    * [`._singleTransform2(data, out, outOff, off, step)`](#module_utils/maths..P2FFT+_singleTransform2) ⇒ void
    * [`._singleTransform4(data, out, outOff, off, step, inv)`](#module_utils/maths..P2FFT+_singleTransform4) ⇒ void
    * [`._realTransform4(out, data, inv)`](#module_utils/maths..P2FFT+_realTransform4)
    * [`._singleRealTransform2(data, out, outOff, off, step)`](#module_utils/maths..P2FFT+_singleRealTransform2) ⇒ void
    * [`._singleRealTransform4(data, out, outOff, off, step, inv)`](#module_utils/maths..P2FFT+_singleRealTransform4)

* * *

### `new P2FFT(size)`

**Throws**:

- Error FFT size must be a power of two larger than 1.

  
    
      ParamTypeDescription
    
  
  

    sizenumberThe size of the input array. Must be a power of two larger than 1.

      

* * *

### `p2FFT.createComplexArray()` ⇒ Float64Array

Create a complex number array with size `2 * size`

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  
**Returns**: Float64Array - A complex number array with size `2 * size`  

* * *

### `p2FFT.fromComplexArray(complex, [storage])` ⇒ Array

Converts a complex number representation stored in a Float64Array to an array of real numbers.

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  
**Returns**: Array - An array of real numbers representing the input complex number representation.  

  
    
      ParamTypeDescription
    
  
  

    complexFloat64ArrayThe complex number representation to be converted.

    
    [storage]ArrayAn optional array to store the result in.

      

* * *

### `p2FFT.toComplexArray(input, [storage])` ⇒ Float64Array

Convert a real-valued input array to a complex-valued output array.

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  
**Returns**: Float64Array - The complex-valued output array.  

  
    
      ParamTypeDescription
    
  
  

    inputFloat64ArrayThe real-valued input array.

    
    [storage]Float64ArrayOptional buffer to store the output array.

      

* * *

### `p2FFT.transform(out, data)` ⇒ void

Performs a Fast Fourier Transform (FFT) on the given input data and stores the result in the output buffer.

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  
**Throws**:

- Error Input and output buffers must be different.

  
    
      ParamTypeDescription
    
  
  

    outFloat64ArrayThe output buffer to store the result.

    
    dataFloat64ArrayThe input data to transform.

      

* * *

### `p2FFT.realTransform(out, data)`

Performs a real-valued forward FFT on the given input buffer and stores the result in the given output buffer.
The input buffer must contain real values only, while the output buffer will contain complex values. The input and
output buffers must be different.

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  
**Throws**:

- Error If the input and output buffers are the same.

  
    
      ParamTypeDescription
    
  
  

    outFloat64ArrayThe output buffer.

    
    dataFloat64ArrayThe input buffer containing real values.

      

* * *

### `p2FFT.inverseTransform(out, data)` ⇒ void

Performs an inverse FFT transformation on the given `data` array, and stores the result in `out`.
The `out` array must be a different buffer than the `data` array. The `out` array will contain the
result of the transformation. The `data` array will not be modified.

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  
**Throws**:

- Error If `out` and `data` refer to the same buffer.

  
    
      ParamTypeDescription
    
  
  

    outFloat64ArrayThe output buffer for the transformed data.

    
    dataFloat64ArrayThe input data to transform.

      

* * *

### `p2FFT._transform4(out, data, inv)` ⇒ void

Performs a radix-4 implementation of a discrete Fourier transform on a given set of data.

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  

  
    
      ParamTypeDescription
    
  
  

    outFloat64ArrayThe output buffer for the transformed data.

    
    dataFloat64ArrayThe input buffer of data to be transformed.

    
    invnumberA scaling factor to apply to the transform.

      

* * *

### `p2FFT._singleTransform2(data, out, outOff, off, step)` ⇒ void

Performs a radix-2 implementation of a discrete Fourier transform on a given set of data.

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  

  
    
      ParamTypeDescription
    
  
  

    dataFloat64ArrayThe input buffer of data to be transformed.

    
    outFloat64ArrayThe output buffer for the transformed data.

    
    outOffnumberThe offset at which to write the output data.

    
    offnumberThe offset at which to begin reading the input data.

    
    stepnumberThe step size for indexing the input data.

      

* * *

### `p2FFT._singleTransform4(data, out, outOff, off, step, inv)` ⇒ void

Performs radix-4 transformation on input data of length 8

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  

  
    
      ParamTypeDescription
    
  
  

    dataFloat64ArrayInput data array of length 8

    
    outFloat64ArrayOutput data array of length 8

    
    outOffnumberIndex of output array to start writing from

    
    offnumberIndex of input array to start reading from

    
    stepnumberStep size between elements in input array

    
    invnumberScaling factor for inverse transform

      

* * *

### `p2FFT._realTransform4(out, data, inv)`

Real input radix-4 implementation

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  

  
    
      ParamTypeDescription
    
  
  

    outFloat64ArrayOutput array for the transformed data

    
    dataFloat64ArrayInput array of real data to be transformed

    
    invnumberThe scale factor used to normalize the inverse transform

      

* * *

### `p2FFT._singleRealTransform2(data, out, outOff, off, step)` ⇒ void

Performs a single real input radix-2 transformation on the provided data

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  

  
    
      ParamTypeDescription
    
  
  

    dataFloat64ArrayThe input data array

    
    outFloat64ArrayThe output data array

    
    outOffnumberThe output offset

    
    offnumberThe input offset

    
    stepnumberThe step

      

* * *

### `p2FFT._singleRealTransform4(data, out, outOff, off, step, inv)`

Computes a single real-valued transform using radix-4 algorithm.
This method is only called for len=8.

**Kind**: instance method of [P2FFT](#module_utils/maths..P2FFT)  

  
    
      ParamTypeDescription
    
  
  

    dataFloat64ArrayThe input data array.

    
    outFloat64ArrayThe output data array.

    
    outOffnumberThe offset into the output array.

    
    offnumberThe offset into the input array.

    
    stepnumberThe step size for the input array.

    
    invnumberThe value of inverse.

      

* * *

## utils/maths~NP2FFT

NP2FFT class provides functionality for performing Fast Fourier Transform on arrays
which are not a power of two in length. In such cases, the chirp-z transform is used.

For more information, see: https://math.stackexchange.com/questions/77118/non-power-of-2-ffts/77156#77156

**Kind**: inner class of [utils/maths](#module_utils/maths)  

* * *

### `new NP2FFT(fft_length)`

Constructs a new NP2FFT object.

  
    
      ParamTypeDescription
    
  
  

    fft_lengthnumberThe length of the FFT

      

* * *

## `utils/maths~magnitude(arr)` ⇒ number

Calculates the magnitude of a given array.

**Kind**: inner method of [utils/maths](#module_utils/maths)  
**Returns**: number - The magnitude of the array.  

  
    
      ParamTypeDescription
    
  
  

    arrArrayThe array to calculate the magnitude of.

      

* * *

## `utils/maths~AnyTypedArray` : Int8Array | Uint8Array | Uint8ClampedArray | Int16Array | Uint16Array | Int32Array | Uint32Array | Float16Array | Float32Array | Float64Array

**Kind**: inner typedef of [utils/maths](#module_utils/maths)  

* * *
