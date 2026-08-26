# Interface: RangeEditCacheEntry

## Properties

### lastTermChunks

• **lastTermChunks**: \{ `hash`: `string` ; `length`: `number`  }[]

The last term's chunks (hash + length)

#### Defined in[[lasttermchunks.defined-in]]

[packages/hub/src/utils/rangeEdit.ts:29](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/rangeEdit.ts#L29)

___

### openSubtree

• **openSubtree**: ``null`` \| `MerkleHashSubtreeJson`

Partial merkle state of all chunks except the last term's
(`at_start: true, at_end: false`); `null` when the file has a single term.

#### Defined in[[opensubtree.defined-in]]

[packages/hub/src/utils/rangeEdit.ts:27](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/rangeEdit.ts#L27)

___

### size

• **size**: `number`

Total file size in bytes

#### Defined in[[size.defined-in]]

[packages/hub/src/utils/rangeEdit.ts:20](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/rangeEdit.ts#L20)

___

### terms

• **terms**: `OriginalTerm` & \{ `rangeHash`: `string`  }[]

The file's terms, in order, with their verification range hashes

#### Defined in[[terms.defined-in]]

[packages/hub/src/utils/rangeEdit.ts:22](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/rangeEdit.ts#L22)
