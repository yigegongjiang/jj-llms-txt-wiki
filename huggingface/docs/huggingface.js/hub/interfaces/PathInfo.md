# Interface: PathInfo

## Properties

### lastCommit

• `Optional` **lastCommit**: [`CommitInfo`](CommitInfo)

Not available for bucket repos, use [uploadedAt](PathInfo#uploadedat) instead.

#### Defined in[[lastcommit.defined-in]]

[packages/hub/src/lib/paths-info.ts:42](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L42)

___

### lfs

• `Optional` **lfs**: [`LfsPathInfo`](LfsPathInfo)

Only defined when path is LFS pointer. Not available for bucket repos.

#### Defined in[[lfs.defined-in]]

[packages/hub/src/lib/paths-info.ts:34](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L34)

___

### oid

• `Optional` **oid**: `string`

Not available for bucket repos.

#### Defined in[[oid.defined-in]]

[packages/hub/src/lib/paths-info.ts:29](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L29)

___

### path

• **path**: `string`

#### Defined in[[path.defined-in]]

[packages/hub/src/lib/paths-info.ts:24](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L24)

___

### securityFileStatus

• `Optional` **securityFileStatus**: [`SecurityFileStatus`](SecurityFileStatus)

#### Defined in[[securityfilestatus.defined-in]]

[packages/hub/src/lib/paths-info.ts:47](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L47)

___

### size

• **size**: `number`

#### Defined in[[size.defined-in]]

[packages/hub/src/lib/paths-info.ts:30](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L30)

___

### type

• **type**: `string`

#### Defined in[[type.defined-in]]

[packages/hub/src/lib/paths-info.ts:25](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L25)

___

### uploadedAt

• `Optional` **uploadedAt**: `string`

Only available for bucket repos.

#### Defined in[[uploadedat.defined-in]]

[packages/hub/src/lib/paths-info.ts:46](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L46)

___

### xetHash

• `Optional` **xetHash**: `string`

Xet-backed hash. Always present for bucket file entries.

#### Defined in[[xethash.defined-in]]

[packages/hub/src/lib/paths-info.ts:38](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L38)
