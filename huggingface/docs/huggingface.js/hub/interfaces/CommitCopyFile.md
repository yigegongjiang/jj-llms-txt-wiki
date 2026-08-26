# Interface: CommitCopyFile

Server-side copy of a file from a source repo/bucket to the destination repo.

Only supported when the destination repo is a bucket. The source file must be xet-backed,
so the caller is responsible for resolving the source path to its [sourceXetHash](CommitCopyFile#sourcexethash)
(typically via [pathsInfo](../modules#pathsinfo) or [listFiles](../modules#listfiles)).

For higher-level helpers that perform the resolution and handle non-xet source files,
see [copyFile](../modules#copyfile), [copyFiles](../modules#copyfiles) and [copyFolder](../modules#copyfolder).

## Properties

### operation

• **operation**: ``"copy"``

#### Defined in[[operation.defined-in]]

[packages/hub/src/lib/commit.ts:116](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L116)

___

### path

• **path**: `string`

#### Defined in[[path.defined-in]]

[packages/hub/src/lib/commit.ts:117](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L117)

___

### sourceRepo

• **sourceRepo**: [`RepoDesignation`](../modules#repodesignation)

#### Defined in[[sourcerepo.defined-in]]

[packages/hub/src/lib/commit.ts:119](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L119)

___

### sourceXetHash

• **sourceXetHash**: `string`

#### Defined in[[sourcexethash.defined-in]]

[packages/hub/src/lib/commit.ts:118](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L118)
