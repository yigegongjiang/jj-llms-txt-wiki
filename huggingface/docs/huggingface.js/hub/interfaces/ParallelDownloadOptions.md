# Interface: ParallelDownloadOptions

## Properties

### controllerTickMs

• `Optional` **controllerTickMs**: `number`

Concurrency controller tick interval, for tests and benchmarks.

#### Defined in[[controllertickms.defined-in]]

[packages/hub/src/utils/XetBlob.ts:145](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L145)

___

### maxConcurrency

• `Optional` **maxConcurrency**: `number`

Ceiling for the auto-tuned number of concurrent xorb requests.

**`Default`**

```ts
8
```

#### Defined in[[maxconcurrency.defined-in]]

[packages/hub/src/utils/XetBlob.ts:131](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L131)

___

### maxInFlightBytes

• `Optional` **maxInFlightBytes**: `number`

Budget of downloaded-but-not-yet-consumed bytes.

**`Default`**

```ts
derived from the file's reconstruction: 3x the largest xorb fetch, clamped to [64MB, 256MB]
```

#### Defined in[[maxinflightbytes.defined-in]]

[packages/hub/src/utils/XetBlob.ts:137](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L137)

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

[packages/hub/src/utils/XetBlob.ts:141](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/XetBlob.ts#L141)
