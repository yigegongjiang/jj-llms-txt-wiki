# generation/cache

Cache classes used by `generate()` to preserve and reuse decoder
past key/value tensors across calls. Pass a `DynamicCache` through
generation options when you need to persist KV state between turns.

## Type Definitions

### DynamicCache

Mutable cache of decoder past key/value tensors used by `generate()`.

Pass a `DynamicCache` through generation options when you need to preserve
cache tensors across calls or inspect them from `return_dict_in_generate`.

_Type:_ `Record`<`string`, [`Tensor`](../utils/tensor#module_utils/tensor.Tensor)>

#### `DynamicCache.constructor([entries])`

Create a DynamicCache, optionally pre-populated with entries.

**Parameters**

- `entries` (`Record`<`string`, [`Tensor`](../utils/tensor#module_utils/tensor.Tensor)>) _optional_ — Initial name→Tensor mappings.

#### `DynamicCache.get_seq_length()`

Get the cached sequence length. This requires at least one attention cache entry to be present.

**Returns:** `number` — The past sequence length.

#### `DynamicCache.update(newEntries)`

Update the cache in-place with new entries, disposing replaced GPU tensors.

**Parameters**

- `newEntries` (`Record`<`string`, [`Tensor`](../utils/tensor#module_utils/tensor.Tensor)>) — The new name → Tensor mappings.

#### `DynamicCache.dispose()`

Dispose all contained tensors whose data resides on the GPU.
Returns a promise that resolves when all disposals are complete.

**Returns:** `Promise`<`void`> — Promise that resolves when all GPU tensors are disposed.
