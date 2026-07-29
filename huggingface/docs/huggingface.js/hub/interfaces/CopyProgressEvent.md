# Interface: CopyProgressEvent

Progress events yielded by [copyFileIter](../modules#copyfileiter) / [copyFilesIter](../modules#copyfilesiter) / [copyFolderIter](../modules#copyfolderiter).

Currently only `fileDownloaded` is emitted: one event per source file that had to be downloaded
(small git-stored files that can't be copied server-side). Xet-backed files are copied
server-side and do not produce events.

## Properties

### downloaded

• **downloaded**: `number`

Number of files downloaded so far (including this one).

#### Defined in[[downloaded.defined-in]]

[packages/hub/src/lib/copy-files.ts:27](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L27)

___

### event

• **event**: ``"fileDownloaded"``

#### Defined in[[event.defined-in]]

[packages/hub/src/lib/copy-files.ts:23](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L23)

___

### path

• **path**: `string`

Source path of the file that was just downloaded.

#### Defined in[[path.defined-in]]

[packages/hub/src/lib/copy-files.ts:25](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L25)

___

### total

• **total**: `number`

Total number of files that will be downloaded.

#### Defined in[[total.defined-in]]

[packages/hub/src/lib/copy-files.ts:29](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L29)
