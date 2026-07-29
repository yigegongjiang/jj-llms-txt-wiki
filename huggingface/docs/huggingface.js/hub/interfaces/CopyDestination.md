# Interface: CopyDestination

Destination location for [copyFile](../modules#copyfile) / [copyFolder](../modules#copyfolder).

The destination repo must be a bucket — server-side copy is currently only supported
towards buckets.

## Properties

### path

• **path**: `string`

Exact destination path within the destination bucket. For [copyFolder](../modules#copyfolder),
acts as a prefix; leave empty to copy under the bucket root.

#### Defined in[[path.defined-in]]

[packages/hub/src/lib/copy-files.ts:65](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L65)

___

### repo

• **repo**: `BucketDesignation`

#### Defined in[[repo.defined-in]]

[packages/hub/src/lib/copy-files.ts:60](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L60)
