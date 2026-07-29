# Interface: CopySource

Source location of a file in [copyFile](../modules#copyfile) / [copyFiles](../modules#copyfiles) / [copyFolder](../modules#copyfolder).

## Properties

### path

• **path**: `string`

Path of the file (or folder, for [copyFolder](../modules#copyfolder)) inside the source repo.
Leave empty in [copyFolder](../modules#copyfolder) to copy the whole repo.

#### Defined in[[path.defined-in]]

[packages/hub/src/lib/copy-files.ts:44](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L44)

___

### repo

• **repo**: [`RepoDesignation`](../modules#repodesignation)

#### Defined in[[repo.defined-in]]

[packages/hub/src/lib/copy-files.ts:39](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L39)

___

### revision

• `Optional` **revision**: `string`

Git revision to read the source from. Ignored for bucket sources.

**`Default`**

```ts
"main"
```

#### Defined in[[revision.defined-in]]

[packages/hub/src/lib/copy-files.ts:50](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L50)
