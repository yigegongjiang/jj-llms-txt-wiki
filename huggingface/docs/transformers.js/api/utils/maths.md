# utils/maths

Numerical helpers — softmax, dot product, cosine similarity, and the
typed-array utilities shared across the library.

## Functions

### `softmax(arr)`

Compute the softmax of an array of numbers.

**Parameters**

- `arr` ([`TypedArray`](./maths#module_utils/maths.TypedArray) | `number[]`) — The array of numbers to compute the softmax of.

**Returns:** [`TypedArray`](./maths#module_utils/maths.TypedArray) | `number[]` — The softmax array.

### `log_softmax(arr)`

Calculates the logarithm of the softmax function for the input array.

**Parameters**

- `arr` ([`TypedArray`](./maths#module_utils/maths.TypedArray) | `number[]`) — The input array to calculate the log_softmax function for.

**Returns:** [`TypedArray`](./maths#module_utils/maths.TypedArray) | `number[]` — The resulting log_softmax array.

### `dot(arr1, arr2)`

Calculates the dot product of two arrays.

**Parameters**

- `arr1` (`number[]`) — The first array.
- `arr2` (`number[]`) — The second array.

**Returns:** `number` — The dot product of arr1 and arr2.

### `cos_sim(arr1, arr2)`

Computes the cosine similarity between two arrays.

**Parameters**

- `arr1` (`number[]`) — The first array.
- `arr2` (`number[]`) — The second array.

**Returns:** `number` — The cosine similarity between the two arrays.

## Type Definitions

### TypedArray

_Type:_ `Int8Array` | `Uint8Array` | `Uint8ClampedArray` | `Int16Array` | `Uint16Array` | `Int32Array` | `Uint32Array` | `Float16Array` | `Float32Array` | `Float64Array`

### BigTypedArray

_Type:_ `BigInt64Array` | `BigUint64Array`

### AnyTypedArray

_Type:_ [`TypedArray`](./maths#module_utils/maths.TypedArray) | [`BigTypedArray`](./maths#module_utils/maths.BigTypedArray)
