# Interface: ListFileEntry

## Properties

### lastCommit

• `Optional` **lastCommit**: `Object`

Only fetched if `expand` is set to `true` in the `listFiles` call.

Not available for bucket repos, use [uploadedAt](ListFileEntry#uploadedat) instead.

#### Type declaration[[lastcommit.type-declaration]]

| Name | Type |
| :------ | :------ |
| `date` | `string` |
| `id` | `string` |
| `title` | `string` |

#### Defined in[[lastcommit.defined-in]]

[packages/hub/src/lib/list-files.ts:32](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-files.ts#L32)

___

### lfs

• `Optional` **lfs**: `Object`

#### Type declaration[[lfs.type-declaration]]

| Name | Type | Description |
| :------ | :------ | :------ |
| `oid` | `string` | - |
| `pointerSize` | `number` | Size of the raw pointer file, 100~200 bytes |
| `size` | `number` | - |

#### Defined in[[lfs.defined-in]]

[packages/hub/src/lib/list-files.ts:17](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-files.ts#L17)

___

### oid

• `Optional` **oid**: `string`

Not available for bucket repos.

#### Defined in[[oid.defined-in]]

[packages/hub/src/lib/list-files.ts:16](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-files.ts#L16)

___

### path

• **path**: `string`

#### Defined in[[path.defined-in]]

[packages/hub/src/lib/list-files.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-files.ts#L12)

___

### securityFileStatus

• `Optional` **securityFileStatus**: `unknown`

Only fetched if `expand` is set to `true` in the `listFiles` call.

#### Defined in[[securityfilestatus.defined-in]]

[packages/hub/src/lib/list-files.ts:46](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-files.ts#L46)

___

### size

• **size**: `number`

#### Defined in[[size.defined-in]]

[packages/hub/src/lib/list-files.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-files.ts#L11)

___

### type

• **type**: ``"unknown"`` \| ``"file"`` \| ``"directory"``

#### Defined in[[type.defined-in]]

[packages/hub/src/lib/list-files.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-files.ts#L10)

___

### uploadedAt

• `Optional` **uploadedAt**: `string`

Only fetched if `expand` is set to `true` in the `listFiles` call.

Only available for bucket repos.

#### Defined in[[uploadedat.defined-in]]

[packages/hub/src/lib/list-files.ts:42](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-files.ts#L42)

___

### xetHash

• `Optional` **xetHash**: `string`

Xet-backed hash, a new protocol replacing LFS for big files.

#### Defined in[[xethash.defined-in]]

[packages/hub/src/lib/list-files.ts:26](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-files.ts#L26)
