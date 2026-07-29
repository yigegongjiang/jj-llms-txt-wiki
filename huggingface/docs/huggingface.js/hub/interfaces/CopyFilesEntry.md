# Interface: CopyFilesEntry

One file to copy in a [copyFiles](../modules#copyfiles) call.

## Properties

### destinationPath

• **destinationPath**: `string`

Exact path within the destination bucket. The bucket itself is shared with the
other entries via the top-level [copyFiles](../modules#copyfiles) `destination` parameter.

#### Defined in[[destinationpath.defined-in]]

[packages/hub/src/lib/copy-files.ts:77](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L77)

___

### source

• **source**: [`CopySource`](CopySource)

#### Defined in[[source.defined-in]]

[packages/hub/src/lib/copy-files.ts:72](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L72)
