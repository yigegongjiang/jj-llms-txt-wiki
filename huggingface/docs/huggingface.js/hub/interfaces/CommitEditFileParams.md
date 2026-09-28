# Interface: CommitEditFileParams

Edit an existing file by replacing byte ranges of its original content, without
re-uploading the whole file.

When `originalContent` comes from [downloadFile](../modules#downloadfile) on a xet-backed file, the
unchanged parts are neither downloaded nor re-uploaded: only the modified byte ranges
(and their chunk-boundary neighborhood) are transferred. This works fully remotely —
without downloading the unchanged data at all — when uploading to a bucket.

**`Example`**

```ts
const original = await downloadFile({ repo: "buckets/me/repo", path: "model.gguf" });
await uploadFile({
  repo: "buckets/me/repo",
  file: {
    path: "model.gguf",
    originalContent: original,
    edits: [{ start: 0, end: 100, content: new Blob([patchedHeader]) }],
  },
});
```

## Properties

### edits

• **edits**: \{ `content`: `Blob` ; `end`: `number` ; `start`: `number`  }[]

#### Defined in[[edits.defined-in]]

[packages/hub/src/lib/upload-file.ts:29](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/upload-file.ts#L29)

___

### originalContent

• **originalContent**: `Blob`

The file's current content, eg as returned by [downloadFile](../modules#downloadfile)

#### Defined in[[originalcontent.defined-in]]

[packages/hub/src/lib/upload-file.ts:28](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/upload-file.ts#L28)

___

### path

• **path**: `string`

#### Defined in[[path.defined-in]]

[packages/hub/src/lib/upload-file.ts:26](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/upload-file.ts#L26)
