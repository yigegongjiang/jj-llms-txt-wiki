# utils/tensor

Tensors and tensor operations.

`Tensor` is the typed n-dimensional array used throughout the library for model inputs
and outputs. This module also provides functions to create, transform, and combine tensors.

## On this page

**Classes** — [`Tensor`](#module_utils/tensor.Tensor)

**Functions** — [`permute`](#module_utils/tensor.permute) · [`interpolate`](#module_utils/tensor.interpolate) · [`interpolate_4d`](#module_utils/tensor.interpolate_4d) · [`matmul`](#module_utils/tensor.matmul) · [`rfft`](#module_utils/tensor.rfft) · [`topk`](#module_utils/tensor.topk) · [`slice`](#module_utils/tensor.slice) · [`mean_pooling`](#module_utils/tensor.mean_pooling) · [`layer_norm`](#module_utils/tensor.layer_norm) · [`cat`](#module_utils/tensor.cat) · [`stack`](#module_utils/tensor.stack) · [`std_mean`](#module_utils/tensor.std_mean) · [`mean`](#module_utils/tensor.mean) · [`full`](#module_utils/tensor.full) · [`full_like`](#module_utils/tensor.full_like) · [`ones`](#module_utils/tensor.ones) · [`ones_like`](#module_utils/tensor.ones_like) · [`zeros`](#module_utils/tensor.zeros) · [`zeros_like`](#module_utils/tensor.zeros_like) · [`rand`](#module_utils/tensor.rand) · [`randn`](#module_utils/tensor.randn) · [`quantize_embeddings`](#module_utils/tensor.quantize_embeddings)

## Classes

### Tensor

A typed multi-dimensional array.

```javascript
import { Tensor } from '@huggingface/transformers';
const tensor = new Tensor('float32', [1, 2, 3, 4, 5, 6], [2, 3]);
tensor.dims;    // [2, 3]
tensor.tolist(); // [[1, 2, 3], [4, 5, 6]]
```

#### `Tensor.dims` : `number[]`

Dimensions of the tensor.

#### `Tensor.type` : [`DataType`](../transformers#module_transformers.DataType)

Type of the tensor.

#### `Tensor.data` : [`DataArray`](./tensor#module_utils/tensor.DataArray)

The data stored in the tensor.

#### `Tensor.size` : `number`

The number of elements in the tensor.

#### `Tensor.location` : `string`

The location of the tensor data.

#### `Tensor.constructor(args)`

Create a new Tensor, either from raw data or by wrapping an `onnxruntime` tensor:
- `new Tensor(dataType, data, dims)`, e.g. `new Tensor('float32', new Float32Array([1, 2, 3]), [3])`.
- `new Tensor(ortTensor)`.

**Parameters**

- `args` (`[DataType, DataArray, number[]]` | `[ONNXTensor]`)

#### `Tensor.dispose()`

Releases the underlying ONNX Runtime tensor (e.g., GPU buffers). Do not use the tensor afterwards.

#### `Tensor.[Symbol.iterator]()`

Returns an iterator object for iterating over the tensor data in row-major order.
If the tensor has more than one dimension, the iterator will yield subarrays.

**Returns:** `Iterator<any>` — An iterator object for iterating over the tensor data in row-major order.

#### `Tensor.item()`

Returns the value of this tensor as a standard JavaScript Number. This only works
for tensors with one element. For other cases, see `Tensor.tolist()`.

**Returns:** `number` | `bigint` — The value of this tensor as a standard JavaScript Number.

**Throws**

- `Error` — If the tensor has more than one element.

#### `Tensor.tolist()`

Convert tensor data to a n-dimensional JS list

**Returns:** `any[]`

#### `Tensor.sigmoid()`

Return a new Tensor with the sigmoid function applied to each element.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The tensor with the sigmoid function applied.

#### `Tensor.sigmoid_()`

Applies the sigmoid function to the tensor in place.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — Returns `this`.

#### `Tensor.map(callback)`

Return a new Tensor with a callback function applied to each element.

**Parameters**

- `callback` (`Function`) — The function to apply to each element. It should take three arguments:
  the current element, its index, and the tensor's data array.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — A new Tensor with the callback function applied to each element.

#### `Tensor.map_(callback)`

Apply a callback function to each element of the tensor in place.

**Parameters**

- `callback` (`Function`) — The function to apply to each element. It should take three arguments:
  the current element, its index, and the tensor's data array.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — Returns `this`.

#### `Tensor.mul(val)`

Return a new Tensor with every element multiplied by a constant.

**Parameters**

- `val` (`number`) — The value to multiply by.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The new tensor.

#### `Tensor.mul_(val)`

Multiply the tensor by a constant in place.

**Parameters**

- `val` (`number`) — The value to multiply by.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — Returns `this`.

#### `Tensor.div(val)`

Return a new Tensor with every element divided by a constant.

**Parameters**

- `val` (`number`) — The value to divide by.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The new tensor.

#### `Tensor.div_(val)`

Divide the tensor by a constant in place.

**Parameters**

- `val` (`number`) — The value to divide by.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — Returns `this`.

#### `Tensor.add(val)`

Return a new Tensor with every element added by a constant.

**Parameters**

- `val` (`number`) — The value to add by.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The new tensor.

#### `Tensor.add_(val)`

Add the tensor by a constant in place.

**Parameters**

- `val` (`number`) — The value to add by.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — Returns `this`.

#### `Tensor.sub(val)`

Return a new Tensor with every element subtracted by a constant.

**Parameters**

- `val` (`number`) — The value to subtract by.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The new tensor.

#### `Tensor.sub_(val)`

Subtract the tensor by a constant in place.

**Parameters**

- `val` (`number`) — The value to subtract by.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — Returns `this`.

#### `Tensor.remainder(val)`

Return a new Tensor with the element-wise remainder of division by a constant.
Uses Python-style modulo signs (e.g. `-1 mod 2 = 1`) while preserving the tensor's dtype.
Negative divisors are unsupported for unsigned and boolean tensors.
This operation does not implement PyTorch's dtype promotion or scalar casting rules.

**Parameters**

- `val` (`number` | `bigint`) — The divisor.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The new tensor.

#### `Tensor.remainder_(val)`

In-place version of [`Tensor.remainder`](./tensor#module_utils/tensor.Tensor.remainder)

**Parameters**

- `val` (`number` | `bigint`) — The divisor.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — Returns `this`.

#### `Tensor.clone()`

Creates a deep copy of the current Tensor.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — A new Tensor with the same type, data, and dimensions as the original.

#### `Tensor.slice(slices)`

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

**Parameters**

- `slices` (`...(number|number[]|null)`) — The slice specifications for each dimension.
  - If a number is given, then a single element is selected.
  - If an array of two numbers is given, then a range of elements [start, end (exclusive)] is selected.
  - If null is given, then the entire dimension is selected.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — A new Tensor containing the selected elements.

**Throws**

- `Error` — If the slice input is invalid.

#### `Tensor.permute(dims)`

Return a permuted version of this Tensor, according to the provided dimensions.

**Parameters**

- `dims` (`...number`) — Dimensions to permute.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The permuted tensor.

#### `Tensor.transpose(dims)`

Return a permuted version of this Tensor, according to the provided dimensions.

**Parameters**

- `dims` (`...number`) — Dimensions to permute.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The permuted tensor.

#### `Tensor.sum([dim], keepdim)`

Returns the sum of each row of the input tensor in the given dimension dim.

**Parameters**

- `dim` (`number` | `null`) _optional_ — defaults to `null` — The dimension or dimensions to reduce. If `null`, all dimensions are reduced.
- `keepdim` (`boolean`) — Whether the output tensor has `dim` retained or not.

**Returns:** The summed tensor

#### `Tensor.norm([p], [dim], [keepdim])`

Returns the matrix norm or vector norm of a given tensor.

**Parameters**

- `p` (`number` | `string`) _optional_ — defaults to `'fro'` — The order of norm
- `dim` (`number` | `null`) _optional_ — defaults to `null` — Specifies which dimension of the tensor to calculate the norm across.
  If dim is None, the norm will be calculated across all dimensions of input.
- `keepdim` (`boolean`) _optional_ — defaults to `false` — Whether the output tensors have dim retained or not.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The norm of the tensor.

#### `Tensor.normalize_([p], [dim])`

Performs `L_p` normalization of inputs over specified dimension. Operates in place.

**Parameters**

- `p` (`number`) _optional_ — defaults to `2` — The exponent value in the norm formulation
- `dim` (`number`) _optional_ — defaults to `1` — The dimension to reduce

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — `this` for operation chaining.

#### `Tensor.normalize([p], [dim])`

Performs `L_p` normalization of inputs over specified dimension.

**Parameters**

- `p` (`number`) _optional_ — defaults to `2` — The exponent value in the norm formulation
- `dim` (`number`) _optional_ — defaults to `1` — The dimension to reduce

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The normalized tensor.

#### `Tensor.stride()`

Compute and return the stride of this tensor.
Stride is the jump necessary to go from one element to the next one in the specified dimension dim.

**Returns:** `number[]` — The stride of this tensor.

#### `Tensor.squeeze([dim])`

Returns a tensor with all specified dimensions of input of size 1 removed.

NOTE: The returned tensor shares the storage with the input tensor, so changing the contents of one will change the contents of the other.
If you would like a copy, use `tensor.clone()` before squeezing.

**Parameters**

- `dim` (`number` | `number[]` | `null`) _optional_ — defaults to `null` — If given, the input will be squeezed only in the specified dimensions.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The squeezed tensor

#### `Tensor.squeeze_([dim])`

In-place version of [`Tensor.squeeze`](./tensor#module_utils/tensor.Tensor.squeeze)

**Parameters**

- `dim` (`number` | `number[]` | `null`) _optional_ — defaults to `null` — If given, the input will be squeezed only in the specified dimensions.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — `this`, with the specified dimensions of size 1 removed.

#### `Tensor.unsqueeze(dim)`

Returns a new tensor with a dimension of size one inserted at the specified position.

NOTE: The returned tensor shares the same underlying data with this tensor.

**Parameters**

- `dim` (`number`) — The index at which to insert the singleton dimension

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The unsqueezed tensor

#### `Tensor.unsqueeze_(dim)`

In-place version of [`Tensor.unsqueeze`](./tensor#module_utils/tensor.Tensor.unsqueeze)

**Parameters**

- `dim` (`number`) — The index at which to insert the singleton dimension

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The unsqueezed tensor

#### `Tensor.flatten_([start_dim], [end_dim])`

In-place version of [`Tensor.flatten`](./tensor#module_utils/tensor.Tensor.flatten)

**Parameters**

- `start_dim` (`number`) _optional_ — defaults to `0` — the first dim to flatten
- `end_dim` (`number`) _optional_ — defaults to `-1` — the last dim to flatten

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — `this`, flattened along the specified dimensions.

#### `Tensor.flatten(start_dim, end_dim)`

Flattens input by reshaping it into a one-dimensional tensor.
If `start_dim` or `end_dim` are passed, only dimensions starting with `start_dim`
and ending with `end_dim` are flattened. The order of elements in input is unchanged.

**Parameters**

- `start_dim` (`number`) — the first dim to flatten
- `end_dim` (`number`) — the last dim to flatten

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The flattened tensor.

#### `Tensor.view(dims)`

Returns a new tensor with the same data as the `self` tensor but of a different `shape`.

**Parameters**

- `dims` (`...number`) — the desired size

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The tensor with the same data but different shape

#### `Tensor.neg_()`

In-place version of [`Tensor.neg`](./tensor#module_utils/tensor.Tensor.neg)

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — `this`, with every element negated.

#### `Tensor.neg()`

Returns a new tensor with the negative of the elements of this tensor.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — the output tensor.

#### `Tensor.gt(val)`

Computes input > val element-wise.

**Parameters**

- `val` (`number`) — The value to compare with.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — A boolean tensor that is `true` where input is greater than other and `false` elsewhere.

#### `Tensor.lt(val)`

Computes input < val element-wise.

**Parameters**

- `val` (`number`) — The value to compare with.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — A boolean tensor that is `true` where input is less than other and `false` elsewhere.

#### `Tensor.clamp_(min, max)`

In-place version of [`Tensor.clamp`](./tensor#module_utils/tensor.Tensor.clamp)

**Parameters**

- `min` (`number`) — lower-bound of the range to be clamped to
- `max` (`number`) — upper-bound of the range to be clamped to

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — the output tensor.

#### `Tensor.clamp(min, max)`

Clamps all elements in input into the range [ min, max ]

**Parameters**

- `min` (`number`) — lower-bound of the range to be clamped to
- `max` (`number`) — upper-bound of the range to be clamped to

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — the output tensor.

#### `Tensor.round_()`

In-place version of [`Tensor.round`](./tensor#module_utils/tensor.Tensor.round)

#### `Tensor.round()`

Rounds elements of input to the nearest integer.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — the output tensor.

#### `Tensor.mean([dim], [keepdim])`

Returns the mean value of each row of this tensor in the given dimension `dim`.

**Parameters**

- `dim` (`number` | `null`) _optional_ — defaults to `null` — the dimension to reduce. If `null`, the mean of all elements is computed.
- `keepdim` (`boolean`) _optional_ — defaults to `false` — whether the output tensor has `dim` retained or not.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — A new tensor with means taken along the specified dimension.

#### `Tensor.min([dim], [keepdim])`

Returns the minimum value of each row of this tensor in the given dimension `dim`.

**Parameters**

- `dim` (`number` | `null`) _optional_ — defaults to `null` — the dimension to reduce. If `null`, the minimum of all elements is computed.
- `keepdim` (`boolean`) _optional_ — defaults to `false` — whether the output tensor has `dim` retained or not.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — A new tensor with minimum values taken along the specified dimension.

#### `Tensor.max([dim], [keepdim])`

Returns the maximum value of each row of this tensor in the given dimension `dim`.

**Parameters**

- `dim` (`number` | `null`) _optional_ — defaults to `null` — the dimension to reduce. If `null`, the maximum of all elements is computed.
- `keepdim` (`boolean`) _optional_ — defaults to `false` — whether the output tensor has `dim` retained or not.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — A new tensor with maximum values taken along the specified dimension.

#### `Tensor.argmin([dim], [keepdim])`

Returns the index of the minimum value of all elements in this tensor.

**Parameters**

- `dim` (`number` | `null`) _optional_ — defaults to `null` — the dimension to reduce. Only `null` (reduce over all elements) is currently supported.
- `keepdim` (`boolean`) _optional_ — defaults to `false` — whether the output tensor has `dim` retained or not.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — An `int64` scalar tensor containing the index of the minimum value.

**Throws**

- `Error` — If `dim` is not `null`.

#### `Tensor.argmax([dim], [keepdim])`

Returns the index of the maximum value of all elements in this tensor.

**Parameters**

- `dim` (`number` | `null`) _optional_ — defaults to `null` — the dimension to reduce. Only `null` (reduce over all elements) is currently supported.
- `keepdim` (`boolean`) _optional_ — defaults to `false` — whether the output tensor has `dim` retained or not.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — An `int64` scalar tensor containing the index of the maximum value.

**Throws**

- `Error` — If `dim` is not `null`.

#### `Tensor.repeat(repeats)`

Repeats this tensor along the specified dimensions.

**Parameters**

- `repeats` (`...number`) — The number of times to repeat this tensor along each dimension.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The repeated tensor.

**Throws**

- `Error` — If the number of repeats is less than the number of dimensions.

#### `Tensor.tile(dims)`

Constructs a tensor by repeating the elements of input. The dims argument specifies the number of repetitions in each dimension.

**Parameters**

- `dims` (`...number`) — The number of repetitions per dimension.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The tiled tensor.

#### `Tensor.to(type)`

Performs Tensor dtype conversion.

**Parameters**

- `type` ([`DataType`](../transformers#module_transformers.DataType)) — The desired data type.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The converted tensor.

## Functions

### `permute(tensor, axes)`

Permutes a tensor according to the provided axes.

**Parameters**

- `tensor` (`any`) — The input tensor to permute.
- `axes` (`number[]`) — The axes to permute the tensor along.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The permuted tensor.

### `interpolate(input, size, mode, align_corners)`

Interpolates an Tensor to the given size.

**Parameters**

- `input` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — The input tensor to interpolate. Data must be channel-first (i.e., [c, h, w])
- `size` (`number[]`) — The output size of the image
- `mode` (`string`) — The interpolation mode
- `align_corners` (`boolean`) — Whether to align corners.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The interpolated tensor.

### `interpolate_4d(input, options)`

Down/up samples the input.
Inspired by https://pytorch.org/docs/stable/generated/torch.nn.functional.interpolate.html.

**Parameters**

- `input` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — the input tensor
- `options` (`Object`) — the options for the interpolation
  - `size` (`[number, number]` | `[number, number, number]` | `[number, number, number, number]`) _optional_ — defaults to `null` — output spatial size.
  - `mode` (`'nearest'` | `'bilinear'` | `'bicubic'`) _optional_ — defaults to `'bilinear'` — algorithm used for upsampling

**Returns:** `Promise`<[`Tensor`](./tensor#module_utils/tensor.Tensor)> — The interpolated tensor.

### `matmul(a, b)`

Matrix product of two tensors.
Inspired by https://pytorch.org/docs/stable/generated/torch.matmul.html

**Parameters**

- `a` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — the first tensor to be multiplied
- `b` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — the second tensor to be multiplied

**Returns:** `Promise`<[`Tensor`](./tensor#module_utils/tensor.Tensor)> — The matrix product of the two tensors.

### `rfft(x, a)`

Computes the one dimensional Fourier transform of real-valued input.
Inspired by https://pytorch.org/docs/stable/generated/torch.fft.rfft.html

**Parameters**

- `x` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — the real input tensor
- `a` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — The dimension along which to take the one dimensional real FFT.

**Returns:** `Promise`<[`Tensor`](./tensor#module_utils/tensor.Tensor)> — the output tensor.

### `topk(x, [k])`

Returns the k largest elements of the given input tensor.
Inspired by https://pytorch.org/docs/stable/generated/torch.topk.html

**Parameters**

- `x` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — the input tensor
- `k` (`number`) _optional_ — the k in "top-k"

**Returns:** `Promise`<`[Tensor, Tensor]`> — the output tuple of (Tensor, LongTensor) of top-k elements and their indices.

### `slice(data, starts, ends, axes, [steps])`

Slice a multidimensional float32 tensor.

**Parameters**

- `data` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — : Tensor of data to extract slices from
- `starts` (`number[]`) — : 1-D array of starting indices of corresponding axis in axes
- `ends` (`number[]`) — : 1-D array of ending indices (exclusive) of corresponding axis in axes
- `axes` (`number[]`) — : 1-D array of axes that starts and ends apply to
- `steps` (`number[]`) _optional_ — : 1-D array of slice step of corresponding axis in axes.

**Returns:** `Promise`<[`Tensor`](./tensor#module_utils/tensor.Tensor)> — Sliced data tensor.

### `mean_pooling(last_hidden_state, attention_mask)`

Perform mean pooling of the last hidden state followed by a normalization step.

**Parameters**

- `last_hidden_state` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — Tensor of shape [batchSize, seqLength, embedDim]
- `attention_mask` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — Tensor of shape [batchSize, seqLength]

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — Returns a new Tensor of shape [batchSize, embedDim].

### `layer_norm(input, normalized_shape, options)`

Apply Layer Normalization for last certain number of dimensions.

**Parameters**

- `input` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — The input tensor
- `normalized_shape` (`number[]`) — input shape from an expected input of size
- `options` (`Object`) — The options for the layer normalization
  - `eps` (`number`) _optional_ — defaults to `1e-5` — A value added to the denominator for numerical stability.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The normalized tensor.

### `cat(tensors, dim)`

Concatenates an array of tensors along a specified dimension.

**Parameters**

- `tensors` ([`Tensor`](./tensor#module_utils/tensor.Tensor)[]) — The array of tensors to concatenate.
- `dim` (`number`) — The dimension to concatenate along.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The concatenated tensor.

### `stack(tensors, dim)`

Stack an array of tensors along a specified dimension.

**Parameters**

- `tensors` ([`Tensor`](./tensor#module_utils/tensor.Tensor)[]) — The array of tensors to stack.
- `dim` (`number`) — The dimension to stack along.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The stacked tensor.

### `std_mean(input, dim, correction, keepdim)`

Calculates the standard deviation and mean over the dimensions specified by dim. dim can be a single dimension or `null` to reduce over all dimensions.

**Parameters**

- `input` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — the input tenso
- `dim` (`number` | `null`) — the dimension to reduce. If None, all dimensions are reduced.
- `correction` (`number`) — difference between the sample size and sample degrees of freedom. Defaults to Bessel's correction, correction=1.
- `keepdim` (`boolean`) — whether the output tensor has dim retained or not.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor)[] — A tuple of (std, mean) tensors.

### `mean(input, dim, keepdim)`

Returns the mean value of each row of the input tensor in the given dimension dim.

**Parameters**

- `input` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — the input tensor.
- `dim` (`number` | `null`) — the dimension to reduce.
- `keepdim` (`boolean`) — whether the output tensor has dim retained or not.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — A new tensor with means taken along the specified dimension.

### `full(size, fill_value)`

Creates a tensor of size size filled with fill_value. The tensor's dtype is inferred from fill_value.

**Parameters**

- `size` (`number[]`) — A sequence of integers defining the shape of the output tensor.
- `fill_value` (`number` | `bigint` | `boolean`) — The value to fill the output tensor with.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The filled tensor.

### `full_like(tensor, fill_value)`

Creates a tensor with the same size as `tensor`, filled with `fill_value`.
The tensor's dtype is inferred from `fill_value`.

**Parameters**

- `tensor` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — The size of input will determine size of the output tensor.
- `fill_value` (`number` | `bigint` | `boolean`) — The value to fill the output tensor with.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The filled tensor.

### `ones(size)`

Returns a tensor filled with the scalar value 1, with the shape defined by the variable argument size.

**Parameters**

- `size` (`number[]`) — A sequence of integers defining the shape of the output tensor.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The ones tensor.

### `ones_like(tensor)`

Returns a tensor filled with the scalar value 1, with the same size as input.

**Parameters**

- `tensor` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — The size of input will determine size of the output tensor.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The ones tensor.

### `zeros(size)`

Returns a tensor filled with the scalar value 0, with the shape defined by the variable argument size.

**Parameters**

- `size` (`number[]`) — A sequence of integers defining the shape of the output tensor.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The zeros tensor.

### `zeros_like(tensor)`

Returns a tensor filled with the scalar value 0, with the same size as input.

**Parameters**

- `tensor` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — The size of input will determine size of the output tensor.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The zeros tensor.

### `rand(size)`

Returns a tensor filled with random numbers from a uniform distribution on the interval [0, 1)

**Parameters**

- `size` (`number[]`) — A sequence of integers defining the shape of the output tensor.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The random tensor.

### `randn(size)`

Returns a tensor filled with random numbers from a normal distribution with mean 0 and variance 1 (also called the standard normal distribution).

**Parameters**

- `size` (`number[]`) — A sequence of integers defining the shape of the output tensor.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The random tensor.

### `quantize_embeddings(tensor, precision)`

Quantizes the embeddings tensor to binary or unsigned binary precision.

**Parameters**

- `tensor` ([`Tensor`](./tensor#module_utils/tensor.Tensor)) — The tensor to quantize.
- `precision` (`'binary'` | `'ubinary'`) — The precision to use for quantization.

**Returns:** [`Tensor`](./tensor#module_utils/tensor.Tensor) — The quantized tensor.

## Type Definitions

### DataType

_Type:_ `keyof typeof DataTypeMap`

### DataArray

_Type:_ [`AnyTypedArray`](./maths#module_utils/maths.AnyTypedArray) | `any[]`

### NestArray

This creates a nested array of a given type and depth (see examples).
