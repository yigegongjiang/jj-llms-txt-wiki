# Interface: JobVolume

## Properties

### mountPath

• **mountPath**: `string`

Mount path inside the container, e.g. "/data"

#### Defined in[[mountpath.defined-in]]

[packages/hub/src/types/api/api-jobs.ts:16](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/api/api-jobs.ts#L16)

___

### path

• `Optional` **path**: `string`

Subfolder prefix inside the bucket/repo to mount, e.g. "path/to/dir"

#### Defined in[[path.defined-in]]

[packages/hub/src/types/api/api-jobs.ts:22](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/api/api-jobs.ts#L22)

___

### readOnly

• `Optional` **readOnly**: `boolean`

Read-only mount (forced true for repos, defaults to false for buckets)

#### Defined in[[readonly.defined-in]]

[packages/hub/src/types/api/api-jobs.ts:20](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/api/api-jobs.ts#L20)

___

### revision

• `Optional` **revision**: `string`

Git revision (only for repos, defaults to "main")

#### Defined in[[revision.defined-in]]

[packages/hub/src/types/api/api-jobs.ts:18](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/api/api-jobs.ts#L18)

___

### source

• **source**: [`RepoDesignation`](../modules#repodesignation)

Source repo, e.g. "datasets/user/my-dataset", "user/my-model", or { type: "dataset", name: "user/my-dataset" }

#### Defined in[[source.defined-in]]

[packages/hub/src/types/api/api-jobs.ts:14](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/api/api-jobs.ts#L14)
