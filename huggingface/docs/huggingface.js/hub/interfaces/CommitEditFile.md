# Interface: CommitEditFile

Edit an existing file by replacing byte ranges of its original content.

When `originalContent` is a XetBlob (as returned by [downloadFile](../modules#downloadfile) for
xet-backed files), the unchanged parts of the file are neither downloaded nor re-chunked:
the CAS server provides chunk-aligned edit windows and partial merkle data
(`GET /v2/file-chunk-hashes`), so only the modified regions (plus their chunk-boundary
neighborhood) are fetched, re-chunked and hashed.

- For buckets (no sha256 involved), this means edits and appends require **no download of
  the unchanged data at all**.
- For models/datasets/spaces, the unchanged data is still streamed **once** to compute the
  new file's sha256 (required by the LFS protocol), instead of twice (sha256 + chunking).

With any other Blob as `originalContent`, the whole content is re-chunked locally and
unchanged chunks are deduplicated against the remote (upload is skipped, but the content
is read in full).

## Properties

### edits

• **edits**: \{ `content`: `Blob` ; `end`: `number` ; `start`: `number`  }[]

#### Defined in[[edits.defined-in]]

[packages/hub/src/lib/commit.ts:75](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L75)

___

### operation

• **operation**: ``"edit"``

#### Defined in[[operation.defined-in]]

[packages/hub/src/lib/commit.ts:71](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L71)

___

### originalContent

• **originalContent**: `Blob`

Later, will be ContentSource. For now simpler to just handle blobs

#### Defined in[[originalcontent.defined-in]]

[packages/hub/src/lib/commit.ts:74](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L74)

___

### path

• **path**: `string`

#### Defined in[[path.defined-in]]

[packages/hub/src/lib/commit.ts:72](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L72)
