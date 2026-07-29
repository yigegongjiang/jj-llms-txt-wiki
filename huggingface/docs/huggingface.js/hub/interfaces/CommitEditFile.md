# Interface: CommitEditFile

Opitmized when only the beginning or the end of the file is replaced

todo: handle other cases

## Properties

### edits

• **edits**: \{ `content`: `Blob` ; `end`: `number` ; `start`: `number`  }[]

#### Defined in[[edits.defined-in]]

[packages/hub/src/lib/commit.ts:59](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L59)

___

### operation

• **operation**: ``"edit"``

#### Defined in[[operation.defined-in]]

[packages/hub/src/lib/commit.ts:55](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L55)

___

### originalContent

• **originalContent**: `Blob`

Later, will be ContentSource. For now simpler to just handle blobs

#### Defined in[[originalcontent.defined-in]]

[packages/hub/src/lib/commit.ts:58](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L58)

___

### path

• **path**: `string`

#### Defined in[[path.defined-in]]

[packages/hub/src/lib/commit.ts:56](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L56)
