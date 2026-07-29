# utils/tensor

Helper module for `Tensor` processing.

These functions and classes are only used internally,
meaning an end-user shouldn't need to access anything here.

* [utils/tensor](#module_utils/tensor)
    * _static_
        * [.Tensor](#module_utils/tensor.Tensor)
            * [`new Tensor(...args)`](#new_module_utils/tensor.Tensor_new)
            * [`.dims`](#module_utils/tensor.Tensor+dims) : Array
            * [`.type`](#module_utils/tensor.Tensor+type) : [DataType](#DataType)
            * [`.data`](#module_utils/tensor.Tensor+data) : DataArray
            * [`.size`](#module_utils/tensor.Tensor+size) : number
            * [`.location`](#module_utils/tensor.Tensor+location) : string
            * [`.Symbol.iterator()`](#module_utils/tensor.Tensor+Symbol.iterator) ⇒ Iterator.&lt;any&gt;
            * [`._getitem(index)`](#module_utils/tensor.Tensor+_getitem) ⇒ [Tensor](#Tensor)
            * [`.indexOf(item)`](#module_utils/tensor.Tensor+indexOf) ⇒ number
            * [`._subarray(index, iterSize, iterDims)`](#module_utils/tensor.Tensor+_subarray) ⇒ [Tensor](#Tensor)
            * [`.item()`](#module_utils/tensor.Tensor+item) ⇒ number | bigint
            * [`.tolist()`](#module_utils/tensor.Tensor+tolist) ⇒ Array
            * [`.sigmoid()`](#module_utils/tensor.Tensor+sigmoid) ⇒ [Tensor](#Tensor)
            * [`.sigmoid_()`](#module_utils/tensor.Tensor+sigmoid_) ⇒ [Tensor](#Tensor)
            * [`.map(callback)`](#module_utils/tensor.Tensor+map) ⇒ [Tensor](#Tensor)
            * [`.map_(callback)`](#module_utils/tensor.Tensor+map_) ⇒ [Tensor](#Tensor)
            * [`.mul(val)`](#module_utils/tensor.Tensor+mul) ⇒ [Tensor](#Tensor)
            * [`.mul_(val)`](#module_utils/tensor.Tensor+mul_) ⇒ [Tensor](#Tensor)
            * [`.div(val)`](#module_utils/tensor.Tensor+div) ⇒ [Tensor](#Tensor)
            * [`.div_(val)`](#module_utils/tensor.Tensor+div_) ⇒ [Tensor](#Tensor)
            * [`.add(val)`](#module_utils/tensor.Tensor+add) ⇒ [Tensor](#Tensor)
            * [`.add_(val)`](#module_utils/tensor.Tensor+add_) ⇒ [Tensor](#Tensor)
            * [`.sub(val)`](#module_utils/tensor.Tensor+sub) ⇒ [Tensor](#Tensor)
            * [`.sub_(val)`](#module_utils/tensor.Tensor+sub_) ⇒ [Tensor](#Tensor)
            * [`.clone()`](#module_utils/tensor.Tensor+clone) ⇒ [Tensor](#Tensor)
            * [`.slice(...slices)`](#module_utils/tensor.Tensor+slice) ⇒ [Tensor](#Tensor)
            * [`.permute(...dims)`](#module_utils/tensor.Tensor+permute) ⇒ [Tensor](#Tensor)
            * [`.transpose()`](#module_utils/tensor.Tensor+transpose) : [Tensor](#Tensor)
            * [`.sum([dim], keepdim)`](#module_utils/tensor.Tensor+sum) ⇒
            * [`.norm([p], [dim], [keepdim])`](#module_utils/tensor.Tensor+norm) ⇒ [Tensor](#Tensor)
            * [`.normalize_([p], [dim])`](#module_utils/tensor.Tensor+normalize_) ⇒ [Tensor](#Tensor)
            * [`.normalize([p], [dim])`](#module_utils/tensor.Tensor+normalize) ⇒ [Tensor](#Tensor)
            * [`.stride()`](#module_utils/tensor.Tensor+stride) ⇒ Array
            * [`.squeeze([dim])`](#module_utils/tensor.Tensor+squeeze) ⇒ [Tensor](#Tensor)
            * [`.squeeze_()`](#module_utils/tensor.Tensor+squeeze_)
            * [`.unsqueeze(dim)`](#module_utils/tensor.Tensor+unsqueeze) ⇒ [Tensor](#Tensor)
            * [`.unsqueeze_()`](#module_utils/tensor.Tensor+unsqueeze_) : [Tensor](#Tensor)
            * [`.flatten_()`](#module_utils/tensor.Tensor+flatten_)
            * [`.flatten(start_dim, end_dim)`](#module_utils/tensor.Tensor+flatten) ⇒ [Tensor](#Tensor)
            * [`.view(...dims)`](#module_utils/tensor.Tensor+view) ⇒ [Tensor](#Tensor)
            * [`.gt(val)`](#module_utils/tensor.Tensor+gt) ⇒ [Tensor](#Tensor)
            * [`.lt(val)`](#module_utils/tensor.Tensor+lt) ⇒ [Tensor](#Tensor)
            * [`.clamp_()`](#module_utils/tensor.Tensor+clamp_) : [Tensor](#Tensor)
            * [`.clamp(min, max)`](#module_utils/tensor.Tensor+clamp) ⇒ [Tensor](#Tensor)
            * [`.round_()`](#module_utils/tensor.Tensor+round_)
            * [`.round()`](#module_utils/tensor.Tensor+round) ⇒ [Tensor](#Tensor)
            * [`.repeat(...repeats)`](#module_utils/tensor.Tensor+repeat) ⇒ [Tensor](#Tensor)
            * [`.tile(...dims)`](#module_utils/tensor.Tensor+tile) ⇒ [Tensor](#Tensor)
            * [`.to(type)`](#module_utils/tensor.Tensor+to) ⇒ [Tensor](#Tensor)
        * [`.permute(tensor, axes)`](#module_utils/tensor.permute) ⇒ [Tensor](#Tensor)
        * [`.interpolate(input, size, mode, align_corners)`](#module_utils/tensor.interpolate) ⇒ [Tensor](#Tensor)
        * [`.interpolate_4d(input, options)`](#module_utils/tensor.interpolate_4d) ⇒ [Promise.&lt;Tensor&gt;](#Tensor)
        * [`.matmul(a, b)`](#module_utils/tensor.matmul) ⇒ [Promise.&lt;Tensor&gt;](#Tensor)
        * [`.rfft(x, a)`](#module_utils/tensor.rfft) ⇒ [Promise.&lt;Tensor&gt;](#Tensor)
        * [`.topk(x, [k])`](#module_utils/tensor.topk) ⇒ Promise.&lt;Array&gt;
        * [`.slice(data:, starts:, ends:, axes:, [steps])`](#module_utils/tensor.slice) ⇒ [Promise.&lt;Tensor&gt;](#Tensor)
        * [`.mean_pooling(last_hidden_state, attention_mask)`](#module_utils/tensor.mean_pooling) ⇒ [Tensor](#Tensor)
        * [`.layer_norm(input, normalized_shape, options)`](#module_utils/tensor.layer_norm) ⇒ [Tensor](#Tensor)
        * [`.cat(tensors, dim)`](#module_utils/tensor.cat) ⇒ [Tensor](#Tensor)
        * [`.stack(tensors, dim)`](#module_utils/tensor.stack) ⇒ [Tensor](#Tensor)
        * [`.std_mean(input, dim, correction, keepdim)`](#module_utils/tensor.std_mean) ⇒ Array
        * [`.mean(input, dim, keepdim)`](#module_utils/tensor.mean) ⇒ [Tensor](#Tensor)
        * [`.full(size, fill_value)`](#module_utils/tensor.full) ⇒ [Tensor](#Tensor)
        * [`.ones(size)`](#module_utils/tensor.ones) ⇒ [Tensor](#Tensor)
        * [`.ones_like(tensor)`](#module_utils/tensor.ones_like) ⇒ [Tensor](#Tensor)
        * [`.zeros(size)`](#module_utils/tensor.zeros) ⇒ [Tensor](#Tensor)
        * [`.zeros_like(tensor)`](#module_utils/tensor.zeros_like) ⇒ [Tensor](#Tensor)
        * [`.rand(size)`](#module_utils/tensor.rand) ⇒ [Tensor](#Tensor)
        * [`.randn(size)`](#module_utils/tensor.randn) ⇒ [Tensor](#Tensor)
        * [`.quantize_embeddings(tensor, precision)`](#module_utils/tensor.quantize_embeddings) ⇒ [Tensor](#Tensor)
    * _inner_
        * [`~args[0]`](#module_utils/tensor..args[0]) : ONNXTensor
        * [`~reshape(data, dimensions)`](#module_utils/tensor..reshape) ⇒ NestArray.&lt;T, DIM&gt;
            * [`~reshapedArray`](#module_utils/tensor..reshape..reshapedArray) : any
        * [`~reduce_helper(callbackfn, input, dim, keepdim, [initialValue])`](#module_utils/tensor..reduce_helper) ⇒ Array
        * [`~DataArray`](#module_utils/tensor..DataArray) : string
        * [`~NestArray`](#module_utils/tensor..NestArray) : any

* * *

## utils/tensor.Tensor

**Kind**: static class of [utils/tensor](#module_utils/tensor)  

* [.Tensor](#module_utils/tensor.Tensor)
    * [`new Tensor(...args)`](#new_module_utils/tensor.Tensor_new)
    * [`.dims`](#module_utils/tensor.Tensor+dims) : Array
    * [`.type`](#module_utils/tensor.Tensor+type) : [DataType](#DataType)
    * [`.data`](#module_utils/tensor.Tensor+data) : DataArray
    * [`.size`](#module_utils/tensor.Tensor+size) : number
    * [`.location`](#module_utils/tensor.Tensor+location) : string
    * [`.Symbol.iterator()`](#module_utils/tensor.Tensor+Symbol.iterator) ⇒ Iterator.&lt;any&gt;
    * [`._getitem(index)`](#module_utils/tensor.Tensor+_getitem) ⇒ [Tensor](#Tensor)
    * [`.indexOf(item)`](#module_utils/tensor.Tensor+indexOf) ⇒ number
    * [`._subarray(index, iterSize, iterDims)`](#module_utils/tensor.Tensor+_subarray) ⇒ [Tensor](#Tensor)
    * [`.item()`](#module_utils/tensor.Tensor+item) ⇒ number | bigint
    * [`.tolist()`](#module_utils/tensor.Tensor+tolist) ⇒ Array
    * [`.sigmoid()`](#module_utils/tensor.Tensor+sigmoid) ⇒ [Tensor](#Tensor)
    * [`.sigmoid_()`](#module_utils/tensor.Tensor+sigmoid_) ⇒ [Tensor](#Tensor)
    * [`.map(callback)`](#module_utils/tensor.Tensor+map) ⇒ [Tensor](#Tensor)
    * [`.map_(callback)`](#module_utils/tensor.Tensor+map_) ⇒ [Tensor](#Tensor)
    * [`.mul(val)`](#module_utils/tensor.Tensor+mul) ⇒ [Tensor](#Tensor)
    * [`.mul_(val)`](#module_utils/tensor.Tensor+mul_) ⇒ [Tensor](#Tensor)
    * [`.div(val)`](#module_utils/tensor.Tensor+div) ⇒ [Tensor](#Tensor)
    * [`.div_(val)`](#module_utils/tensor.Tensor+div_) ⇒ [Tensor](#Tensor)
    * [`.add(val)`](#module_utils/tensor.Tensor+add) ⇒ [Tensor](#Tensor)
    * [`.add_(val)`](#module_utils/tensor.Tensor+add_) ⇒ [Tensor](#Tensor)
    * [`.sub(val)`](#module_utils/tensor.Tensor+sub) ⇒ [Tensor](#Tensor)
    * [`.sub_(val)`](#module_utils/tensor.Tensor+sub_) ⇒ [Tensor](#Tensor)
    * [`.clone()`](#module_utils/tensor.Tensor+clone) ⇒ [Tensor](#Tensor)
    * [`.slice(...slices)`](#module_utils/tensor.Tensor+slice) ⇒ [Tensor](#Tensor)
    * [`.permute(...dims)`](#module_utils/tensor.Tensor+permute) ⇒ [Tensor](#Tensor)
    * [`.transpose()`](#module_utils/tensor.Tensor+transpose) : [Tensor](#Tensor)
    * [`.sum([dim], keepdim)`](#module_utils/tensor.Tensor+sum) ⇒
    * [`.norm([p], [dim], [keepdim])`](#module_utils/tensor.Tensor+norm) ⇒ [Tensor](#Tensor)
    * [`.normalize_([p], [dim])`](#module_utils/tensor.Tensor+normalize_) ⇒ [Tensor](#Tensor)
    * [`.normalize([p], [dim])`](#module_utils/tensor.Tensor+normalize) ⇒ [Tensor](#Tensor)
    * [`.stride()`](#module_utils/tensor.Tensor+stride) ⇒ Array
    * [`.squeeze([dim])`](#module_utils/tensor.Tensor+squeeze) ⇒ [Tensor](#Tensor)
    * [`.squeeze_()`](#module_utils/tensor.Tensor+squeeze_)
    * [`.unsqueeze(dim)`](#module_utils/tensor.Tensor+unsqueeze) ⇒ [Tensor](#Tensor)
    * [`.unsqueeze_()`](#module_utils/tensor.Tensor+unsqueeze_) : [Tensor](#Tensor)
    * [`.flatten_()`](#module_utils/tensor.Tensor+flatten_)
    * [`.flatten(start_dim, end_dim)`](#module_utils/tensor.Tensor+flatten) ⇒ [Tensor](#Tensor)
    * [`.view(...dims)`](#module_utils/tensor.Tensor+view) ⇒ [Tensor](#Tensor)
    * [`.gt(val)`](#module_utils/tensor.Tensor+gt) ⇒ [Tensor](#Tensor)
    * [`.lt(val)`](#module_utils/tensor.Tensor+lt) ⇒ [Tensor](#Tensor)
    * [`.clamp_()`](#module_utils/tensor.Tensor+clamp_) : [Tensor](#Tensor)
    * [`.clamp(min, max)`](#module_utils/tensor.Tensor+clamp) ⇒ [Tensor](#Tensor)
    * [`.round_()`](#module_utils/tensor.Tensor+round_)
    * [`.round()`](#module_utils/tensor.Tensor+round) ⇒ [Tensor](#Tensor)
    * [`.repeat(...repeats)`](#module_utils/tensor.Tensor+repeat) ⇒ [Tensor](#Tensor)
    * [`.tile(...dims)`](#module_utils/tensor.Tensor+tile) ⇒ [Tensor](#Tensor)
    * [`.to(type)`](#module_utils/tensor.Tensor+to) ⇒ [Tensor](#Tensor)

* * *

### `new Tensor(...args)`

Create a new Tensor or copy an existing Tensor.

  
    
      ParamType
    
  
  

    ...argsArray | Array
      

* * *

### `tensor.dims` : Array

Dimensions of the tensor.

**Kind**: instance property of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.type` : [DataType](#DataType)

Type of the tensor.

**Kind**: instance property of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.data` : DataArray

The data stored in the tensor.

**Kind**: instance property of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.size` : number

The number of elements in the tensor.

**Kind**: instance property of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.location` : string

The location of the tensor data.

**Kind**: instance property of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.Symbol.iterator()` ⇒ Iterator.&lt;any&gt;

Returns an iterator object for iterating over the tensor data in row-major order.
If the tensor has more than one dimension, the iterator will yield subarrays.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: Iterator.&lt;any&gt; - An iterator object for iterating over the tensor data in row-major order.  

* * *

### `tensor._getitem(index)` ⇒ [Tensor](#Tensor)

Index into a Tensor object.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The data at the specified index.  

  
    
      ParamTypeDescription
    
  
  

    indexnumberThe index to access.

      

* * *

### `tensor.indexOf(item)` ⇒ number

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: number - The index of the first occurrence of item in the tensor data.  

  
    
      ParamTypeDescription
    
  
  

    itemnumber | bigintThe item to search for in the tensor

      

* * *

### `tensor._subarray(index, iterSize, iterDims)` ⇒ [Tensor](#Tensor)

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  

  
    
      ParamType
    
  
  

    indexnumber
    
    iterSizenumber
    
    iterDimsany
      

* * *

### `tensor.item()` ⇒ number | bigint

Returns the value of this tensor as a standard JavaScript Number. This only works
for tensors with one element. For other cases, see `Tensor.tolist()`.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: number | bigint - The value of this tensor as a standard JavaScript Number.  
**Throws**:

- Error If the tensor has more than one element.

* * *

### `tensor.tolist()` ⇒ Array

Convert tensor data to a n-dimensional JS list

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.sigmoid()` ⇒ [Tensor](#Tensor)

Return a new Tensor with the sigmoid function applied to each element.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The tensor with the sigmoid function applied.  

* * *

### `tensor.sigmoid_()` ⇒ [Tensor](#Tensor)

Applies the sigmoid function to the tensor in place.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - Returns `this`.  

* * *

### `tensor.map(callback)` ⇒ [Tensor](#Tensor)

Return a new Tensor with a callback function applied to each element.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - A new Tensor with the callback function applied to each element.  

  
    
      ParamTypeDescription
    
  
  

    callbackfunctionThe function to apply to each element. It should take three arguments:
                             the current element, its index, and the tensor&#39;s data array.

      

* * *

### `tensor.map_(callback)` ⇒ [Tensor](#Tensor)

Apply a callback function to each element of the tensor in place.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - Returns `this`.  

  
    
      ParamTypeDescription
    
  
  

    callbackfunctionThe function to apply to each element. It should take three arguments:
                             the current element, its index, and the tensor&#39;s data array.

      

* * *

### `tensor.mul(val)` ⇒ [Tensor](#Tensor)

Return a new Tensor with every element multiplied by a constant.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The new tensor.  

  
    
      ParamTypeDescription
    
  
  

    valnumberThe value to multiply by.

      

* * *

### `tensor.mul_(val)` ⇒ [Tensor](#Tensor)

Multiply the tensor by a constant in place.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - Returns `this`.  

  
    
      ParamTypeDescription
    
  
  

    valnumberThe value to multiply by.

      

* * *

### `tensor.div(val)` ⇒ [Tensor](#Tensor)

Return a new Tensor with every element divided by a constant.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The new tensor.  

  
    
      ParamTypeDescription
    
  
  

    valnumberThe value to divide by.

      

* * *

### `tensor.div_(val)` ⇒ [Tensor](#Tensor)

Divide the tensor by a constant in place.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - Returns `this`.  

  
    
      ParamTypeDescription
    
  
  

    valnumberThe value to divide by.

      

* * *

### `tensor.add(val)` ⇒ [Tensor](#Tensor)

Return a new Tensor with every element added by a constant.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The new tensor.  

  
    
      ParamTypeDescription
    
  
  

    valnumberThe value to add by.

      

* * *

### `tensor.add_(val)` ⇒ [Tensor](#Tensor)

Add the tensor by a constant in place.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - Returns `this`.  

  
    
      ParamTypeDescription
    
  
  

    valnumberThe value to add by.

      

* * *

### `tensor.sub(val)` ⇒ [Tensor](#Tensor)

Return a new Tensor with every element subtracted by a constant.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The new tensor.  

  
    
      ParamTypeDescription
    
  
  

    valnumberThe value to subtract by.

      

* * *

### `tensor.sub_(val)` ⇒ [Tensor](#Tensor)

Subtract the tensor by a constant in place.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - Returns `this`.  

  
    
      ParamTypeDescription
    
  
  

    valnumberThe value to subtract by.

      

* * *

### `tensor.clone()` ⇒ [Tensor](#Tensor)

Creates a deep copy of the current Tensor.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - A new Tensor with the same type, data, and dimensions as the original.  

* * *

### `tensor.slice(...slices)` ⇒ [Tensor](#Tensor)

Performs a slice operation on the Tensor along specified dimensions.

Consider a Tensor that has a dimension of [4, 7]:
```
[ 1,  2,  3,  4,  5,  6,  7]
[ 8,  9, 10, 11, 12, 13, 14]
[15, 16, 17, 18, 19, 20, 21]
[22, 23, 24, 25, 26, 27, 28]
```
We can slice against the two dims of row and column, for instance in this
case we can start at the second element, and return to the second last,
like this:
```
tensor.slice([1, -1], [1, -1]);
```
which would return:
```
[  9, 10, 11, 12, 13 ]
[ 16, 17, 18, 19, 20 ]
```

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - A new Tensor containing the selected elements.  
**Throws**:

- Error If the slice input is invalid.

  
    
      ParamTypeDescription
    
  
  

    ...slicesnumber | Array | nullThe slice specifications for each dimension.

If a number is given, then a single element is selected.
If an array of two numbers is given, then a range of elements [start, end (exclusive)] is selected.
If null is given, then the entire dimension is selected.

      

* * *

### `tensor.permute(...dims)` ⇒ [Tensor](#Tensor)

Return a permuted version of this Tensor, according to the provided dimensions.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The permuted tensor.  

  
    
      ParamTypeDescription
    
  
  

    ...dimsnumberDimensions to permute.

      

* * *

### `tensor.transpose()` : [Tensor](#Tensor)

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.sum([dim], keepdim)` ⇒

Returns the sum of each row of the input tensor in the given dimension dim.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: The summed tensor  

  
    
      ParamTypeDefaultDescription
    
  
  

    [dim]number | nullThe dimension or dimensions to reduce. If null, all dimensions are reduced.

    
    keepdimbooleanfalseWhether the output tensor has dim retained or not.

      

* * *

### `tensor.norm([p], [dim], [keepdim])` ⇒ [Tensor](#Tensor)

Returns the matrix norm or vector norm of a given tensor.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The norm of the tensor.  

  
    
      ParamTypeDefaultDescription
    
  
  

    [p]number | string&#x27;fro&#x27;The order of norm

    
    [dim]number | nullSpecifies which dimension of the tensor to calculate the norm across.
If dim is None, the norm will be calculated across all dimensions of input.

    
    [keepdim]booleanfalseWhether the output tensors have dim retained or not.

      

* * *

### `tensor.normalize_([p], [dim])` ⇒ [Tensor](#Tensor)

Performs `L_p` normalization of inputs over specified dimension. Operates in place.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - `this` for operation chaining.  

  
    
      ParamTypeDefaultDescription
    
  
  

    [p]number2The exponent value in the norm formulation

    
    [dim]number1The dimension to reduce

      

* * *

### `tensor.normalize([p], [dim])` ⇒ [Tensor](#Tensor)

Performs `L_p` normalization of inputs over specified dimension.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The normalized tensor.  

  
    
      ParamTypeDefaultDescription
    
  
  

    [p]number2The exponent value in the norm formulation

    
    [dim]number1The dimension to reduce

      

* * *

### `tensor.stride()` ⇒ Array

Compute and return the stride of this tensor.
Stride is the jump necessary to go from one element to the next one in the specified dimension dim.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: Array - The stride of this tensor.  

* * *

### `tensor.squeeze([dim])` ⇒ [Tensor](#Tensor)

Returns a tensor with all specified dimensions of input of size 1 removed.

NOTE: The returned tensor shares the storage with the input tensor, so changing the contents of one will change the contents of the other.
If you would like a copy, use `tensor.clone()` before squeezing.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The squeezed tensor  

  
    
      ParamTypeDefaultDescription
    
  
  

    [dim]number | Array | nullIf given, the input will be squeezed only in the specified dimensions.

      

* * *

### `tensor.squeeze_()`

In-place version of @see [Tensor.squeeze](Tensor.squeeze)

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.unsqueeze(dim)` ⇒ [Tensor](#Tensor)

Returns a new tensor with a dimension of size one inserted at the specified position.

NOTE: The returned tensor shares the same underlying data with this tensor.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The unsqueezed tensor  

  
    
      ParamTypeDescription
    
  
  

    dimnumberThe index at which to insert the singleton dimension

      

* * *

### `tensor.unsqueeze_()` : [Tensor](#Tensor)

In-place version of @see [Tensor.unsqueeze](Tensor.unsqueeze)

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.flatten_()`

In-place version of @see [Tensor.flatten](Tensor.flatten)

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.flatten(start_dim, end_dim)` ⇒ [Tensor](#Tensor)

Flattens input by reshaping it into a one-dimensional tensor.
If `start_dim` or `end_dim` are passed, only dimensions starting with `start_dim`
and ending with `end_dim` are flattened. The order of elements in input is unchanged.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The flattened tensor.  

  
    
      ParamTypeDefaultDescription
    
  
  

    start_dimnumber0the first dim to flatten

    
    end_dimnumberthe last dim to flatten

      

* * *

### `tensor.view(...dims)` ⇒ [Tensor](#Tensor)

Returns a new tensor with the same data as the `self` tensor but of a different `shape`.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The tensor with the same data but different shape  

  
    
      ParamTypeDescription
    
  
  

    ...dimsnumberthe desired size

      

* * *

### `tensor.gt(val)` ⇒ [Tensor](#Tensor)

Computes input > val element-wise.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - A boolean tensor that is `true` where input is greater than other and `false` elsewhere.  

  
    
      ParamTypeDescription
    
  
  

    valnumberThe value to compare with.

      

* * *

### `tensor.lt(val)` ⇒ [Tensor](#Tensor)

Computes input Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - A boolean tensor that is `true` where input is less than other and `false` elsewhere.  

  
    
      ParamTypeDescription
    
  
  

    valnumberThe value to compare with.

      

* * *

### `tensor.clamp_()` : [Tensor](#Tensor)

In-place version of @see [Tensor.clamp](Tensor.clamp)

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.clamp(min, max)` ⇒ [Tensor](#Tensor)

Clamps all elements in input into the range [ min, max ]

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - the output tensor.  

  
    
      ParamTypeDescription
    
  
  

    minnumberlower-bound of the range to be clamped to

    
    maxnumberupper-bound of the range to be clamped to

      

* * *

### `tensor.round_()`

In-place version of @see [Tensor.round](Tensor.round)

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  

* * *

### `tensor.round()` ⇒ [Tensor](#Tensor)

Rounds elements of input to the nearest integer.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - the output tensor.  

* * *

### `tensor.repeat(...repeats)` ⇒ [Tensor](#Tensor)

Repeats this tensor along the specified dimensions.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The repeated tensor.  
**Throws**:

- Error If the number of repeats is less than the number of dimensions.

  
    
      ParamTypeDescription
    
  
  

    ...repeatsnumberThe number of times to repeat this tensor along each dimension.

      

* * *

### `tensor.tile(...dims)` ⇒ [Tensor](#Tensor)

Constructs a tensor by repeating the elements of input. The dims argument specifies the number of repetitions in each dimension.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The tiled tensor.  

  
    
      ParamTypeDescription
    
  
  

    ...dimsnumberThe number of repetitions per dimension.

      

* * *

### `tensor.to(type)` ⇒ [Tensor](#Tensor)

Performs Tensor dtype conversion.

**Kind**: instance method of [Tensor](#module_utils/tensor.Tensor)  
**Returns**: [Tensor](#Tensor) - The converted tensor.  

  
    
      ParamTypeDescription
    
  
  

    typeDataTypeThe desired data type.

      

* * *

## `utils/tensor.permute(tensor, axes)` ⇒ [Tensor](#Tensor)

Permutes a tensor according to the provided axes.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The permuted tensor.  

  
    
      ParamTypeDescription
    
  
  

    tensoranyThe input tensor to permute.

    
    axesArrayThe axes to permute the tensor along.

      

* * *

## `utils/tensor.interpolate(input, size, mode, align_corners)` ⇒ [Tensor](#Tensor)

Interpolates an Tensor to the given size.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The interpolated tensor.  

  
    
      ParamTypeDescription
    
  
  

    inputTensorThe input tensor to interpolate. Data must be channel-first (i.e., [c, h, w])

    
    sizeArrayThe output size of the image

    
    modestringThe interpolation mode

    
    align_cornersbooleanWhether to align corners.

      

* * *

## `utils/tensor.interpolate_4d(input, options)` ⇒ [Promise.&lt;Tensor&gt;](#Tensor)

Down/up samples the input.
Inspired by https://pytorch.org/docs/stable/generated/torch.nn.functional.interpolate.html.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Promise.&lt;Tensor&gt;](#Tensor) - The interpolated tensor.  

  
    
      ParamTypeDefaultDescription
    
  
  

    inputTensorthe input tensor

    
    optionsObjectthe options for the interpolation

    
    [options.size]Array | Array | Arrayoutput spatial size.

    
    [options.mode]&quot;nearest&quot; | &quot;bilinear&quot; | &quot;bicubic&quot;&#x27;bilinear&#x27;algorithm used for upsampling

      

* * *

## `utils/tensor.matmul(a, b)` ⇒ [Promise.&lt;Tensor&gt;](#Tensor)

Matrix product of two tensors.
Inspired by https://pytorch.org/docs/stable/generated/torch.matmul.html

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Promise.&lt;Tensor&gt;](#Tensor) - The matrix product of the two tensors.  

  
    
      ParamTypeDescription
    
  
  

    aTensorthe first tensor to be multiplied

    
    bTensorthe second tensor to be multiplied

      

* * *

## `utils/tensor.rfft(x, a)` ⇒ [Promise.&lt;Tensor&gt;](#Tensor)

Computes the one dimensional Fourier transform of real-valued input.
Inspired by https://pytorch.org/docs/stable/generated/torch.fft.rfft.html

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Promise.&lt;Tensor&gt;](#Tensor) - the output tensor.  

  
    
      ParamTypeDescription
    
  
  

    xTensorthe real input tensor

    
    aTensorThe dimension along which to take the one dimensional real FFT.

      

* * *

## `utils/tensor.topk(x, [k])` ⇒ Promise.&lt;Array&gt;

Returns the k largest elements of the given input tensor.
Inspired by https://pytorch.org/docs/stable/generated/torch.topk.html

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: Promise.&lt;Array&gt; - the output tuple of (Tensor, LongTensor) of top-k elements and their indices.  

  
    
      ParamTypeDescription
    
  
  

    xTensorthe input tensor

    
    [k]numberthe k in &quot;top-k&quot;

      

* * *

## `utils/tensor.slice(data:, starts:, ends:, axes:, [steps])` ⇒ [Promise.&lt;Tensor&gt;](#Tensor)

Slice a multidimensional float32 tensor.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Promise.&lt;Tensor&gt;](#Tensor) - Sliced data tensor.  

  
    
      ParamTypeDescription
    
  
  

    data:TensorTensor of data to extract slices from

    
    starts:Array1-D array of starting indices of corresponding axis in axes

    
    ends:Array1-D array of ending indices (exclusive) of corresponding axis in axes

    
    axes:Array1-D array of axes that starts and ends apply to

    
    [steps]Array1-D array of slice step of corresponding axis in axes.

      

* * *

## `utils/tensor.mean_pooling(last_hidden_state, attention_mask)` ⇒ [Tensor](#Tensor)

Perform mean pooling of the last hidden state followed by a normalization step.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - Returns a new Tensor of shape [batchSize, embedDim].  

  
    
      ParamTypeDescription
    
  
  

    last_hidden_stateTensorTensor of shape [batchSize, seqLength, embedDim]

    
    attention_maskTensorTensor of shape [batchSize, seqLength]

      

* * *

## `utils/tensor.layer_norm(input, normalized_shape, options)` ⇒ [Tensor](#Tensor)

Apply Layer Normalization for last certain number of dimensions.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The normalized tensor.  

  
    
      ParamTypeDefaultDescription
    
  
  

    inputTensorThe input tensor

    
    normalized_shapeArrayinput shape from an expected input of size

    
    optionsObjectThe options for the layer normalization

    
    [options.eps]number1e-5A value added to the denominator for numerical stability.

      

* * *

## `utils/tensor.cat(tensors, dim)` ⇒ [Tensor](#Tensor)

Concatenates an array of tensors along a specified dimension.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The concatenated tensor.  

  
    
      ParamTypeDescription
    
  
  

    tensorsArrayThe array of tensors to concatenate.

    
    dimnumberThe dimension to concatenate along.

      

* * *

## `utils/tensor.stack(tensors, dim)` ⇒ [Tensor](#Tensor)

Stack an array of tensors along a specified dimension.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The stacked tensor.  

  
    
      ParamTypeDescription
    
  
  

    tensorsArrayThe array of tensors to stack.

    
    dimnumberThe dimension to stack along.

      

* * *

## `utils/tensor.std_mean(input, dim, correction, keepdim)` ⇒ Array

Calculates the standard deviation and mean over the dimensions specified by dim. dim can be a single dimension or `null` to reduce over all dimensions.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: Array - A tuple of (std, mean) tensors.  

  
    
      ParamTypeDescription
    
  
  

    inputTensorthe input tenso

    
    dimnumber | nullthe dimension to reduce. If None, all dimensions are reduced.

    
    correctionnumberdifference between the sample size and sample degrees of freedom. Defaults to Bessel&#39;s correction, correction=1.

    
    keepdimbooleanwhether the output tensor has dim retained or not.

      

* * *

## `utils/tensor.mean(input, dim, keepdim)` ⇒ [Tensor](#Tensor)

Returns the mean value of each row of the input tensor in the given dimension dim.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - A new tensor with means taken along the specified dimension.  

  
    
      ParamTypeDescription
    
  
  

    inputTensorthe input tensor.

    
    dimnumber | nullthe dimension to reduce.

    
    keepdimbooleanwhether the output tensor has dim retained or not.

      

* * *

## `utils/tensor.full(size, fill_value)` ⇒ [Tensor](#Tensor)

Creates a tensor of size size filled with fill_value. The tensor's dtype is inferred from fill_value.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The filled tensor.  

  
    
      ParamTypeDescription
    
  
  

    sizeArrayA sequence of integers defining the shape of the output tensor.

    
    fill_valuenumber | bigint | booleanThe value to fill the output tensor with.

      

* * *

## `utils/tensor.ones(size)` ⇒ [Tensor](#Tensor)

Returns a tensor filled with the scalar value 1, with the shape defined by the variable argument size.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The ones tensor.  

  
    
      ParamTypeDescription
    
  
  

    sizeArrayA sequence of integers defining the shape of the output tensor.

      

* * *

## `utils/tensor.ones_like(tensor)` ⇒ [Tensor](#Tensor)

Returns a tensor filled with the scalar value 1, with the same size as input.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The ones tensor.  

  
    
      ParamTypeDescription
    
  
  

    tensorTensorThe size of input will determine size of the output tensor.

      

* * *

## `utils/tensor.zeros(size)` ⇒ [Tensor](#Tensor)

Returns a tensor filled with the scalar value 0, with the shape defined by the variable argument size.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The zeros tensor.  

  
    
      ParamTypeDescription
    
  
  

    sizeArrayA sequence of integers defining the shape of the output tensor.

      

* * *

## `utils/tensor.zeros_like(tensor)` ⇒ [Tensor](#Tensor)

Returns a tensor filled with the scalar value 0, with the same size as input.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The zeros tensor.  

  
    
      ParamTypeDescription
    
  
  

    tensorTensorThe size of input will determine size of the output tensor.

      

* * *

## `utils/tensor.rand(size)` ⇒ [Tensor](#Tensor)

Returns a tensor filled with random numbers from a uniform distribution on the interval [0, 1)

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The random tensor.  

  
    
      ParamTypeDescription
    
  
  

    sizeArrayA sequence of integers defining the shape of the output tensor.

      

* * *

## `utils/tensor.randn(size)` ⇒ [Tensor](#Tensor)

Returns a tensor filled with random numbers from a normal distribution with mean 0 and variance 1 (also called the standard normal distribution).

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The random tensor.  

  
    
      ParamTypeDescription
    
  
  

    sizeArrayA sequence of integers defining the shape of the output tensor.

      

* * *

## `utils/tensor.quantize_embeddings(tensor, precision)` ⇒ [Tensor](#Tensor)

Quantizes the embeddings tensor to binary or unsigned binary precision.

**Kind**: static method of [utils/tensor](#module_utils/tensor)  
**Returns**: [Tensor](#Tensor) - The quantized tensor.  

  
    
      ParamTypeDescription
    
  
  

    tensorTensorThe tensor to quantize.

    
    precision&#x27;binary&#x27; | &#x27;ubinary&#x27;The precision to use for quantization.

      

* * *

## `utils/tensor~args[0]` : ONNXTensor

**Kind**: inner property of [utils/tensor](#module_utils/tensor)  

* * *

## `utils/tensor~reshape(data, dimensions)` ⇒ NestArray.&lt;T, DIM&gt;

Reshapes a 1-dimensional array into an n-dimensional array, according to the provided dimensions.

**Kind**: inner method of [utils/tensor](#module_utils/tensor)  
**Returns**: NestArray.&lt;T, DIM&gt; - The reshaped array.  

  
    
      ParamTypeDescription
    
  
  

    dataArray | DataArrayThe input array to reshape.

    
    dimensionsDIMThe target shape/dimensions.

      

**Example**  
```js
reshape([10                    ], [1      ]); // Type: number[]      Value: [10]
  reshape([1, 2, 3, 4            ], [2, 2   ]); // Type: number[][]    Value: [[1, 2], [3, 4]]
  reshape([1, 2, 3, 4, 5, 6, 7, 8], [2, 2, 2]); // Type: number[][][]  Value: [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
  reshape([1, 2, 3, 4, 5, 6, 7, 8], [4, 2   ]); // Type: number[][]    Value: [[1, 2], [3, 4], [5, 6], [7, 8]]
```

* * *

### `reshape~reshapedArray` : any

**Kind**: inner property of [reshape](#module_utils/tensor..reshape)  

* * *

## `utils/tensor~reduce_helper(callbackfn, input, dim, keepdim, [initialValue])` ⇒ Array

**Kind**: inner method of [utils/tensor](#module_utils/tensor)  
**Returns**: Array - The reduced tensor data.  

  
    
      ParamTypeDefaultDescription
    
  
  

    callbackfnfunction
    
    inputTensorthe input tensor.

    
    dimnumberthe dimension to reduce.

    
    keepdimbooleanfalsewhether the output tensor has dim retained or not.

    
    [initialValue]anythe initial value to start the reduction with.

      

* * *

## `utils/tensor~DataArray` : string

**Kind**: inner typedef of [utils/tensor](#module_utils/tensor)  

* * *

## `utils/tensor~NestArray` : any

This creates a nested array of a given type and depth (see examples).

**Kind**: inner typedef of [utils/tensor](#module_utils/tensor)  
**Example**  
```js
NestArray; // string[]
```
**Example**  
```js
NestArray; // number[][]
```
**Example**  
```js
NestArray; // string[][][] etc.
```

* * *
