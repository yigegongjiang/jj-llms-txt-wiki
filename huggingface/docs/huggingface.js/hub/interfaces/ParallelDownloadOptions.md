# Interface: ParallelDownloadOptions

## Properties

### controllerTickMs

• `Optional` **controllerTickMs**: `number`

Concurrency controller tick interval, for tests and benchmarks.

#### Defined in[[controllertickms.defined-in]]

[packages/hub/src/utils/XetBlob.ts:69](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L69)

___

### maxConcurrency

• `Optional` **maxConcurrency**: `number`

Ceiling for the auto-tuned number of concurrent xorb requests.

**`Default`**

```ts
8
```

#### Defined in[[maxconcurrency.defined-in]]

[packages/hub/src/utils/XetBlob.ts:55](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L55)

___

### maxInFlightBytes

• `Optional` **maxInFlightBytes**: `number`

Budget of downloaded-but-not-yet-consumed bytes.

**`Default`**

```ts
derived from the file's reconstruction: 3x the largest xorb fetch, clamped to [64MB, 256MB]
```

#### Defined in[[maxinflightbytes.defined-in]]

[packages/hub/src/utils/XetBlob.ts:61](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L61)

___

### onStat

• `Optional` **onStat**: (`stat`: `Record`\<`string`, `unknown`\>) => `void`

Instrumentation callback for tests and benchmarks, called once per download.

#### Type declaration[[onstat.type-declaration]]

▸ (`stat`): `void`

##### Parameters[[onstat.parameters]]

| Name | Type |
| :------ | :------ |
| `stat` | `Record`\<`string`, `unknown`\> |

##### Returns[[onstat.returns]]

`void`

#### Defined in[[onstat.defined-in]]

[packages/hub/src/utils/XetBlob.ts:65](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L65)
