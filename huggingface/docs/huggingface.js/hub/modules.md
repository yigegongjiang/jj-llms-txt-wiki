# @huggingface/hub

## Classes

- [HubApiError](classes/HubApiError)
- [InvalidApiResponseFormatError](classes/InvalidApiResponseFormatError)
- [SafetensorParseError](classes/SafetensorParseError)
- [\_\_internal\_XetBlob](classes/_internal_XetBlob)

## Interfaces

- [AuthInfo](interfaces/AuthInfo)
- [CachedFileInfo](interfaces/CachedFileInfo)
- [CachedRepoInfo](interfaces/CachedRepoInfo)
- [CachedRevisionInfo](interfaces/CachedRevisionInfo)
- [CommitCopyFile](interfaces/CommitCopyFile)
- [CommitData](interfaces/CommitData)
- [CommitDeletedEntry](interfaces/CommitDeletedEntry)
- [CommitEditFile](interfaces/CommitEditFile)
- [CommitFile](interfaces/CommitFile)
- [CommitInfo](interfaces/CommitInfo)
- [CommitOutput](interfaces/CommitOutput)
- [CopyDestination](interfaces/CopyDestination)
- [CopyFilesEntry](interfaces/CopyFilesEntry)
- [CopyProgressEvent](interfaces/CopyProgressEvent)
- [CopySource](interfaces/CopySource)
- [Credentials](interfaces/Credentials)
- [DatasetEntry](interfaces/DatasetEntry)
- [FileDownloadInfoOutput](interfaces/FileDownloadInfoOutput)
- [HFCacheInfo](interfaces/HFCacheInfo)
- [JobVolume](interfaces/JobVolume)
- [LfsPathInfo](interfaces/LfsPathInfo)
- [ListFileEntry](interfaces/ListFileEntry)
- [ModelConfig](interfaces/ModelConfig)
- [ModelDerivedFields](interfaces/ModelDerivedFields)
- [ModelEntry](interfaces/ModelEntry)
- [OAuthResult](interfaces/OAuthResult)
- [ParallelDownloadOptions](interfaces/ParallelDownloadOptions)
- [PathInfo](interfaces/PathInfo)
- [QuantizationConfig](interfaces/QuantizationConfig)
- [RepoId](interfaces/RepoId)
- [SafetensorsIndexJson](interfaces/SafetensorsIndexJson)
- [SafetensorsShardFileInfo](interfaces/SafetensorsShardFileInfo)
- [SecurityFileStatus](interfaces/SecurityFileStatus)
- [SpaceEntry](interfaces/SpaceEntry)
- [SpaceResourceConfig](interfaces/SpaceResourceConfig)
- [SpaceResourceRequirement](interfaces/SpaceResourceRequirement)
- [SpaceRuntime](interfaces/SpaceRuntime)
- [TensorInfo](interfaces/TensorInfo)
- [UserInfo](interfaces/UserInfo)
- [WhoAmIApp](interfaces/WhoAmIApp)
- [WhoAmIOrg](interfaces/WhoAmIOrg)
- [WhoAmIUser](interfaces/WhoAmIUser)
- [XetFileInfo](interfaces/XetFileInfo)
- [XetReadToken](interfaces/XetReadToken)

## Type Aliases

### AccessToken

Ƭ **AccessToken**: `string`

Actually `hf_${string}`, but for convenience, using the string type

#### Defined in[[accesstoken.defined-in]]

[packages/hub/src/types/public.ts:28](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L28)

___

### AccessTokenRole

Ƭ **AccessTokenRole**: ``"admin"`` \| ``"write"`` \| ``"contributor"`` \| ``"read"``

#### Defined in[[accesstokenrole.defined-in]]

[packages/hub/src/types/public.ts:90](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L90)

___

### AuthType

Ƭ **AuthType**: ``"access_token"`` \| ``"app_token"`` \| ``"app_token_as_user"``

#### Defined in[[authtype.defined-in]]

[packages/hub/src/types/public.ts:92](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L92)

___

### CommitOperation

Ƭ **CommitOperation**: [`CommitDeletedEntry`](interfaces/CommitDeletedEntry) \| [`CommitFile`](interfaces/CommitFile) \| [`CommitEditFile`](interfaces/CommitEditFile) \| [`CommitCopyFile`](interfaces/CommitCopyFile)

#### Defined in[[commitoperation.defined-in]]

[packages/hub/src/lib/commit.ts:106](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L106)

___

### CommitParams

Ƭ **CommitParams**: \{ `abortSignal?`: `AbortSignal` ; `branch?`: `string` ; `description?`: `string` ; `fetch?`: typeof [`__type`](classes/_internal_XetBlob#__type) ; `hubUrl?`: `string` ; `isPullRequest?`: `boolean` ; `maxFolderDepth?`: `number` ; `operations`: [`CommitOperation`](modules#commitoperation)[] ; `parentCommit?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `title`: `string` ; `useWebWorkers?`: `boolean` \| \{ `minSize?`: `number` ; `poolSize?`: `number`  } ; `useXet?`: `boolean`  } & `Partial`\<`CredentialsParams`\>

#### Defined in[[commitparams.defined-in]]

[packages/hub/src/lib/commit.ts:113](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L113)

___

### CommitProgressEvent

Ƭ **CommitProgressEvent**: \{ `event`: ``"phase"`` ; `phase`: ``"preuploading"`` \| ``"uploadingLargeFiles"`` \| ``"committing"``  } \| \{ `event`: ``"fileProgress"`` ; `path`: `string` ; `progress`: `number` ; `state`: ``"hashing"`` \| ``"uploading"`` \| ``"error"``  }

#### Defined in[[commitprogressevent.defined-in]]

[packages/hub/src/lib/commit.ts:174](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L174)

___

### ContentSource

Ƭ **ContentSource**: `Blob` \| `URL`

#### Defined in[[contentsource.defined-in]]

[packages/hub/src/lib/commit.ts:40](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L40)

___

### Dtype

Ƭ **Dtype**: ``"F64"`` \| ``"F32"`` \| ``"C64"`` \| ``"F16"`` \| ``"F8_E4M3"`` \| ``"F8_E4M3FNUZ"`` \| ``"F8_E5M2"`` \| ``"F8_E5M2FNUZ"`` \| ``"F8_E8M0"`` \| ``"E8M0"`` \| ``"F6_E3M2"`` \| ``"F6_E2M3"`` \| ``"F4"`` \| ``"FP4"`` \| ``"BF16"`` \| ``"I64"`` \| ``"U64"`` \| ``"I32"`` \| ``"U32"`` \| ``"I16"`` \| ``"I8"`` \| ``"U16"`` \| ``"U8"`` \| ``"UE8"`` \| ``"BOOL"``

#### Defined in[[dtype.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:246](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L246)

___

### ModelAdditionalField

Ƭ **ModelAdditionalField**: `Exclude`\ \| keyof [`ModelDerivedFields`](interfaces/ModelDerivedFields)

#### Defined in[[modeladditionalfield.defined-in]]

[packages/hub/src/lib/list-models.ts:50](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-models.ts#L50)

___

### PipelineType

Ƭ **PipelineType**: keyof typeof `PIPELINE_DATA`

#### Defined in[[pipelinetype.defined-in]]

packages/tasks/dist/commonjs/pipelines.d.ts:380

___

### RepoDesignation

Ƭ **RepoDesignation**: [`RepoId`](interfaces/RepoId) \| [`RepoFullName`](modules#repofullname)

#### Defined in[[repodesignation.defined-in]]

[packages/hub/src/types/public.ts:17](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L17)

___

### RepoFullName

Ƭ **RepoFullName**: `string` \| \`spaces/$\{string}\` \| \`datasets/$\{string}\` \| \`buckets/$\{string}\` \| \`kernels/$\{string}\`

#### Defined in[[repofullname.defined-in]]

[packages/hub/src/types/public.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L10)

___

### RepoType

Ƭ **RepoType**: ``"space"`` \| ``"dataset"`` \| ``"model"`` \| ``"bucket"`` \| ``"kernel"``

#### Defined in[[repotype.defined-in]]

[packages/hub/src/types/public.ts:3](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L3)

___

### ResolveModelAdditionalFields

Ƭ **ResolveModelAdditionalFields**\<`T`\>: `Pick`\<`ApiModelInfo`, `T` & keyof `ApiModelInfo`\> & `Pick`\<[`ModelDerivedFields`](interfaces/ModelDerivedFields), `T` & keyof [`ModelDerivedFields`](interfaces/ModelDerivedFields)\>

#### Type parameters[[resolvemodeladditionalfields.type-parameters]]

| Name | Type |
| :------ | :------ |
| `T` | extends [`ModelAdditionalField`](modules#modeladditionalfield) |

#### Defined in[[resolvemodeladditionalfields.defined-in]]

[packages/hub/src/lib/list-models.ts:54](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-models.ts#L54)

___

### SafetensorsFileHeader

Ƭ **SafetensorsFileHeader**: `Record`\<[`TensorName`](modules#tensorname), [`TensorInfo`](interfaces/TensorInfo)\> & \{ `__metadata__?`: \{ `total_parameters?`: `string` \| `number`  } & `Record`\<`string`, `string`\>  }

#### Defined in[[safetensorsfileheader.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:279](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L279)

___

### SafetensorsParseFromRepo

Ƭ **SafetensorsParseFromRepo**: \{ `filepaths`: `string`[] ; `header`: [`SafetensorsFileHeader`](modules#safetensorsfileheader) ; `parameterCount?`: `Partial`\<`Record`\<[`Dtype`](modules#dtype), `number`\>\> ; `parameterTotal?`: `number` ; `sharded`: ``false``  } \| \{ `filepaths`: `string`[] ; `headers`: [`SafetensorsShardedHeaders`](modules#safetensorsshardedheaders) ; `index`: [`SafetensorsIndexJson`](interfaces/SafetensorsIndexJson) ; `parameterCount?`: `Partial`\<`Record`\<[`Dtype`](modules#dtype), `number`\>\> ; `parameterTotal?`: `number` ; `sharded`: ``true``  }

#### Defined in[[safetensorsparsefromrepo.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:303](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L303)

___

### SafetensorsShardedHeaders

Ƭ **SafetensorsShardedHeaders**: `Record`\<`FileName`, [`SafetensorsFileHeader`](modules#safetensorsfileheader)\>

#### Defined in[[safetensorsshardedheaders.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:301](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L301)

___

### SpaceHardwareFlavor

Ƭ **SpaceHardwareFlavor**: ``"cpu-basic"`` \| ``"cpu-upgrade"`` \| ``"cpu-performance"`` \| ``"cpu-xl"`` \| ``"sprx8"`` \| ``"zero-a10g"`` \| ``"inf2x6"`` \| ``"t4-small"`` \| ``"t4-medium"`` \| ``"l4x1"`` \| ``"l4x4"`` \| ``"l40sx1"`` \| ``"l40sx4"`` \| ``"l40sx8"`` \| ``"a10g-small"`` \| ``"a10g-large"`` \| ``"a10g-largex2"`` \| ``"a10g-largex4"`` \| ``"a100-large"`` \| ``"a100x4"`` \| ``"a100x8"``

#### Defined in[[spacehardwareflavor.defined-in]]

[packages/hub/src/types/public.ts:53](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L53)

___

### SpaceSdk

Ƭ **SpaceSdk**: ``"streamlit"`` \| ``"gradio"`` \| ``"docker"`` \| ``"static"``

#### Defined in[[spacesdk.defined-in]]

[packages/hub/src/types/public.ts:76](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L76)

___

### SpaceStage

Ƭ **SpaceStage**: ``"NO_APP_FILE"`` \| ``"CONFIG_ERROR"`` \| ``"BUILDING"`` \| ``"BUILD_ERROR"`` \| ``"RUNNING"`` \| ``"RUNNING_BUILDING"`` \| ``"RUNTIME_ERROR"`` \| ``"DELETING"`` \| ``"PAUSED"`` \| ``"SLEEPING"``

#### Defined in[[spacestage.defined-in]]

[packages/hub/src/types/public.ts:78](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L78)

___

### TensorName

Ƭ **TensorName**: `string`

#### Defined in[[tensorname.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:245](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L245)

___

### WhoAmI

Ƭ **WhoAmI**: [`WhoAmIApp`](interfaces/WhoAmIApp) \| [`WhoAmIOrg`](interfaces/WhoAmIOrg) \| [`WhoAmIUser`](interfaces/WhoAmIUser)

#### Defined in[[whoami.defined-in]]

[packages/hub/src/lib/who-am-i.ts:51](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/who-am-i.ts#L51)

## Variables

### DATASET\_EXPANDABLE\_KEYS

• `Const` **DATASET\_EXPANDABLE\_KEYS**: readonly [``"author"``, ``"cardData"``, ``"citation"``, ``"createdAt"``, ``"disabled"``, ``"description"``, ``"downloads"``, ``"downloadsAllTime"``, ``"gated"``, ``"gitalyUid"``, ``"lastModified"``, ``"likes"``, ``"paperswithcode_id"``, ``"private"``, ``"sha"``, ``"tags"``]

#### Defined in[[datasetexpandablekeys.defined-in]]

[packages/hub/src/lib/list-datasets.ts:17](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-datasets.ts#L17)

___

### DATASET\_EXPAND\_KEYS

• `Const` **DATASET\_EXPAND\_KEYS**: readonly [``"private"``, ``"downloads"``, ``"gated"``, ``"likes"``, ``"lastModified"``]

#### Defined in[[datasetexpandkeys.defined-in]]

[packages/hub/src/lib/list-datasets.ts:9](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-datasets.ts#L9)

___

### DEFAULT\_REVISION

• `Const` **DEFAULT\_REVISION**: ``"main"``

#### Defined in[[defaultrevision.defined-in]]

[packages/hub/src/lib/snapshot-download.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/snapshot-download.ts#L12)

___

### DIFFUSERS\_SAFETENSORS\_FILE

• `Const` **DIFFUSERS\_SAFETENSORS\_FILE**: ``"diffusion_pytorch_model.safetensors"``

#### Defined in[[diffuserssafetensorsfile.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:16](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L16)

___

### DIFFUSERS\_SAFETENSORS\_INDEX\_FILE

• `Const` **DIFFUSERS\_SAFETENSORS\_INDEX\_FILE**: ``"diffusion_pytorch_model.safetensors.index.json"``

#### Defined in[[diffuserssafetensorsindexfile.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:17](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L17)

___

### DIFFUSERS\_WEIGHTS\_SUBFOLDERS

• `Const` **DIFFUSERS\_WEIGHTS\_SUBFOLDERS**: readonly [``"transformer"``, ``"unet"``]

#### Defined in[[diffusersweightssubfolders.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:18](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L18)

___

### HUB\_URL

• `Const` **HUB\_URL**: ``"https://huggingface.co"``

Base URL of the Hugging Face Hub.

#### Defined in[[huburl.defined-in]]

[packages/hub/src/consts.ts:4](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/consts.ts#L4)

___

### MODEL\_DERIVED\_FIELD\_TO\_API\_KEY

• `Const` **MODEL\_DERIVED\_FIELD\_TO\_API\_KEY**: `Record`\

#### Defined in[[modelderivedfieldtoapikey.defined-in]]

[packages/hub/src/lib/list-models.ts:46](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-models.ts#L46)

___

### MODEL\_EXPANDABLE\_KEYS

• `Const` **MODEL\_EXPANDABLE\_KEYS**: readonly [``"author"``, ``"cardData"``, ``"config"``, ``"createdAt"``, ``"disabled"``, ``"downloads"``, ``"downloadsAllTime"``, ``"gated"``, ``"gitalyUid"``, ``"inferenceProviderMapping"``, ``"lastModified"``, ``"library_name"``, ``"likes"``, ``"model-index"``, ``"pipeline_tag"``, ``"private"``, ``"safetensors"``, ``"sha"``, ``"spaces"``, ``"tags"``, ``"transformersInfo"``]

#### Defined in[[modelexpandablekeys.defined-in]]

[packages/hub/src/lib/list-models.ts:18](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-models.ts#L18)

___

### MODEL\_EXPAND\_KEYS

• `Const` **MODEL\_EXPAND\_KEYS**: readonly [``"pipeline_tag"``, ``"private"``, ``"gated"``, ``"downloads"``, ``"likes"``, ``"lastModified"``]

#### Defined in[[modelexpandkeys.defined-in]]

[packages/hub/src/lib/list-models.ts:9](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-models.ts#L9)

___

### REGEX\_COMMIT\_HASH

• `Const` **REGEX\_COMMIT\_HASH**: `RegExp`

#### Defined in[[regexcommithash.defined-in]]

[packages/hub/src/lib/download-file-to-cache-dir.ts:15](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/download-file-to-cache-dir.ts#L15)

___

### REPO\_ID\_SEPARATOR

• `Const` **REPO\_ID\_SEPARATOR**: `string` = `"--"`

#### Defined in[[repoidseparator.defined-in]]

[packages/hub/src/lib/cache-management.ts:25](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/cache-management.ts#L25)

___

### RE\_SAFETENSORS\_FILE

• `Const` **RE\_SAFETENSORS\_FILE**: `RegExp`

#### Defined in[[resafetensorsfile.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:21](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L21)

___

### RE\_SAFETENSORS\_INDEX\_FILE

• `Const` **RE\_SAFETENSORS\_INDEX\_FILE**: `RegExp`

#### Defined in[[resafetensorsindexfile.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:22](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L22)

___

### RE\_SAFETENSORS\_SHARD\_FILE

• `Const` **RE\_SAFETENSORS\_SHARD\_FILE**: `RegExp`

#### Defined in[[resafetensorsshardfile.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:23](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L23)

___

### SAFETENSORS\_FILE

• `Const` **SAFETENSORS\_FILE**: ``"model.safetensors"``

#### Defined in[[safetensorsfile.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L12)

___

### SAFETENSORS\_INDEX\_FILE

• `Const` **SAFETENSORS\_INDEX\_FILE**: ``"model.safetensors.index.json"``

#### Defined in[[safetensorsindexfile.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L13)

___

### SPACE\_EXPANDABLE\_KEYS

• `Const` **SPACE\_EXPANDABLE\_KEYS**: readonly [``"author"``, ``"cardData"``, ``"datasets"``, ``"disabled"``, ``"gitalyUid"``, ``"lastModified"``, ``"createdAt"``, ``"likes"``, ``"private"``, ``"runtime"``, ``"sdk"``, ``"sha"``, ``"subdomain"``, ``"tags"``, ``"models"``]

#### Defined in[[spaceexpandablekeys.defined-in]]

[packages/hub/src/lib/list-spaces.ts:15](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-spaces.ts#L15)

___

### SPACE\_EXPAND\_KEYS

• `Const` **SPACE\_EXPAND\_KEYS**: readonly [``"sdk"``, ``"likes"``, ``"private"``, ``"lastModified"``]

#### Defined in[[spaceexpandkeys.defined-in]]

[packages/hub/src/lib/list-spaces.ts:9](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-spaces.ts#L9)

## Functions

### \_\_internal\_sha256

▸ **__internal_sha256**(`buffer`, `opts?`): `AsyncGenerator`\<`number`, `string`\>

#### Parameters[[internalsha256.parameters]]

| Name | Type |
| :------ | :------ |
| `buffer` | `Blob` |
| `opts?` | `Object` |
| `opts.abortSignal?` | `AbortSignal` |
| `opts.useWebWorker?` | `boolean` \| \{ `minSize?`: `number` ; `poolSize?`: `number`  } |

#### Returns[[internalsha256.returns]]

`AsyncGenerator`\<`number`, `string`\>

hex-encoded sha

**`Yields`**

progress (0-1)

#### Defined in[[internalsha256.defined-in]]

[packages/hub/src/utils/sha256.ts:72](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/utils/sha256.ts#L72)

___

### cancelJob

▸ **cancelJob**(`params`): `Promise`\<`ApiJob`\>

Cancel a job.

#### Parameters[[canceljob.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[canceljob.returns]]

`Promise`\<`ApiJob`\>

#### Defined in[[canceljob.defined-in]]

[packages/hub/src/lib/jobs/cancel-job.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/cancel-job.ts#L10)

___

### checkRepoAccess

▸ **checkRepoAccess**(`params`): `Promise`\<`void`\>

Check if we have read access to a repository.

Throw a [HubApiError](classes/HubApiError) error if we don't have access. HubApiError.statusCode will be 401, 403 or 404.

#### Parameters[[checkrepoaccess.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation)  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[checkrepoaccess.returns]]

`Promise`\<`void`\>

#### Defined in[[checkrepoaccess.defined-in]]

[packages/hub/src/lib/check-repo-access.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/check-repo-access.ts#L13)

___

### commit

▸ **commit**(`params`): `Promise`\<[`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Parameters[[commit.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | [`CommitParams`](modules#commitparams) |

#### Returns[[commit.returns]]

`Promise`\<[`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

undefined for bucket uploads, CommitOutput otherwise

#### Defined in[[commit.defined-in]]

[packages/hub/src/lib/commit.ts:1003](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L1003)

___

### commitIter

▸ **commitIter**(`params`): `AsyncGenerator`\<[`CommitProgressEvent`](modules#commitprogressevent), [`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

Internal function for now, used by commit.

Can be exposed later to offer fine-tuned progress info

CommitOutput is not present for bucket commits

#### Parameters[[commititer.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | [`CommitParams`](modules#commitparams) |

#### Returns[[commititer.returns]]

`AsyncGenerator`\<[`CommitProgressEvent`](modules#commitprogressevent), [`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Defined in[[commititer.defined-in]]

[packages/hub/src/lib/commit.ts:193](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L193)

___

### commitIterBucket

▸ **commitIterBucket**(`params`): `AsyncGenerator`\<[`CommitProgressEvent`](modules#commitprogressevent)\>

#### Parameters[[commititerbucket.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | [`CommitParams`](modules#commitparams) |

#### Returns[[commititerbucket.returns]]

`AsyncGenerator`\<[`CommitProgressEvent`](modules#commitprogressevent)\>

#### Defined in[[commititerbucket.defined-in]]

[packages/hub/src/lib/commit.ts:748](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/commit.ts#L748)

___

### computeNumOfParamsByDtypeSingleFile

▸ **computeNumOfParamsByDtypeSingleFile**(`header`, `quantConfig?`, `expertDtype?`): `Partial`\<`Record`\<[`Dtype`](modules#dtype), `number`\>\>

Sums parameters per dtype for one file's header, applying the quantization packing factors and
skipping bookkeeping tensors.

#### Parameters[[computenumofparamsbydtypesinglefile.parameters]]

| Name | Type |
| :------ | :------ |
| `header` | [`SafetensorsFileHeader`](modules#safetensorsfileheader) |
| `quantConfig?` | [`QuantizationConfig`](interfaces/QuantizationConfig) |
| `expertDtype?` | `string` |

#### Returns[[computenumofparamsbydtypesinglefile.returns]]

`Partial`\<`Record`\<[`Dtype`](modules#dtype), `number`\>\>

#### Defined in[[computenumofparamsbydtypesinglefile.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:908](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L908)

___

### copyFile

▸ **copyFile**(`params`): `Promise`\<`undefined`\>

Copy a single file from a source repo/bucket to the destination bucket.

The copy is server-side (no data transfer) when the source file is xet-backed.
For small non-xet repo files (e.g. `config.json`) the file is downloaded and
re-uploaded to the destination bucket in the same commit.

LFS pointer files that have not been migrated to xet are rejected up front
(they would otherwise require downloading the full LFS blob).

#### Parameters[[copyfile.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `destination`: [`CopyDestination`](interfaces/CopyDestination) ; `source`: [`CopySource`](interfaces/CopySource)  } & `SharedParams` |

#### Returns[[copyfile.returns]]

`Promise`\<`undefined`\>

**`Example`**

```ts
await copyFile({
  source: {
    repo: { type: "model", name: "username/my-model" },
    path: "model.safetensors",
  },
  destination: {
    repo: { type: "bucket", name: "username/my-bucket" },
    path: "models/my-model/model.safetensors",
  },
  accessToken: "hf_...",
});
```

#### Defined in[[copyfile.defined-in]]

[packages/hub/src/lib/copy-files.ts:111](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L111)

___

### copyFileIter

▸ **copyFileIter**(`params`): `AsyncGenerator`\<[`CopyProgressEvent`](interfaces/CopyProgressEvent), `undefined`\>

Async-iterator variant of [copyFile](modules#copyfile) that yields [CopyProgressEvent](interfaces/CopyProgressEvent)s while
downloading non-xet source files (xet-backed files are copied server-side and do not
emit events). See [copyFile](modules#copyfile) for the semantics.

#### Parameters[[copyfileiter.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `destination`: [`CopyDestination`](interfaces/CopyDestination) ; `source`: [`CopySource`](interfaces/CopySource)  } & `SharedParams` |

#### Returns[[copyfileiter.returns]]

`AsyncGenerator`\<[`CopyProgressEvent`](interfaces/CopyProgressEvent), `undefined`\>

**`Example`**

```ts
for await (const event of copyFileIter({ source, destination, accessToken })) {
  console.log(`downloaded ${event.path} (${event.downloaded}/${event.total})`);
}
```

#### Defined in[[copyfileiter.defined-in]]

[packages/hub/src/lib/copy-files.ts:144](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L144)

___

### copyFiles

▸ **copyFiles**(`params`): `Promise`\<`undefined`\>

Copy multiple files (potentially from different source repos/buckets) to the destination
bucket in a single commit.

For xet-backed source files, the copy is performed server-side with no data transfer.
For non-xet source files (typically small git-stored repo files), the file is
downloaded and re-uploaded as part of the same commit.

LFS pointer files that have not been migrated to xet are rejected up front.

#### Parameters[[copyfiles.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `destination`: `BucketDesignation` ; `files`: [`CopyFilesEntry`](interfaces/CopyFilesEntry)[]  } & `SharedParams` |

#### Returns[[copyfiles.returns]]

`Promise`\<`undefined`\>

**`Example`**

```ts
await copyFiles({
  destination: { type: "bucket", name: "username/my-bucket" },
  files: [
    {
      source: {
        repo: { type: "bucket", name: "username/other-bucket" },
        path: "data.bin",
      },
      destinationPath: "data.bin",
    },
    {
      source: {
        repo: { type: "model", name: "username/my-model" },
        path: "model.safetensors",
      },
      destinationPath: "models/my-model/model.safetensors",
    },
  ],
  accessToken: "hf_...",
});
```

#### Defined in[[copyfiles.defined-in]]

[packages/hub/src/lib/copy-files.ts:199](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L199)

___

### copyFilesIter

▸ **copyFilesIter**(`params`): `AsyncGenerator`\<[`CopyProgressEvent`](interfaces/CopyProgressEvent), `undefined`\>

Async-iterator variant of [copyFiles](modules#copyfiles) that yields [CopyProgressEvent](interfaces/CopyProgressEvent)s while
downloading non-xet source files (xet-backed files are copied server-side and do not
emit events). See [copyFiles](modules#copyfiles) for the semantics.

#### Parameters[[copyfilesiter.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `destination`: `BucketDesignation` ; `files`: [`CopyFilesEntry`](interfaces/CopyFilesEntry)[]  } & `SharedParams` |

#### Returns[[copyfilesiter.returns]]

`AsyncGenerator`\<[`CopyProgressEvent`](interfaces/CopyProgressEvent), `undefined`\>

#### Defined in[[copyfilesiter.defined-in]]

[packages/hub/src/lib/copy-files.ts:219](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L219)

___

### copyFolder

▸ **copyFolder**(`params`): `Promise`\<`undefined`\>

Copy a folder (recursively) from a source repo/bucket to the destination bucket
in a single commit.

Per-file paths are resolved relative to [CopySource.path](interfaces/CopySource#path); the source folder
itself is not preserved in the destination unless [CopyDestination.path](interfaces/CopyDestination#path)
keeps it.

#### Parameters[[copyfolder.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `destination`: `Omit`\<[`CopyDestination`](interfaces/CopyDestination), ``"path"``\> & \{ `path?`: `string`  } ; `source`: `Omit`\<[`CopySource`](interfaces/CopySource), ``"path"``\> & \{ `path?`: `string`  }  } & `SharedParams` |

#### Returns[[copyfolder.returns]]

`Promise`\<`undefined`\>

**`Example`**

```ts
// Copy an entire dataset under "datasets/my-dataset/" in the bucket
await copyFolder({
  source: { repo: { type: "dataset", name: "username/my-dataset" } },
  destination: {
    repo: { type: "bucket", name: "username/my-bucket" },
    path: "datasets/my-dataset/",
  },
  accessToken: "hf_...",
});

// Copy a subfolder
await copyFolder({
  source: {
    repo: { type: "bucket", name: "username/src-bucket" },
    path: "models/",
  },
  destination: {
    repo: { type: "bucket", name: "username/dst-bucket" },
    path: "backup/",
  },
  accessToken: "hf_...",
});
```

#### Defined in[[copyfolder.defined-in]]

[packages/hub/src/lib/copy-files.ts:277](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L277)

___

### copyFolderIter

▸ **copyFolderIter**(`params`): `AsyncGenerator`\<[`CopyProgressEvent`](interfaces/CopyProgressEvent), `undefined`\>

Async-iterator variant of [copyFolder](modules#copyfolder) that yields [CopyProgressEvent](interfaces/CopyProgressEvent)s while
downloading non-xet source files (xet-backed files are copied server-side and do not
emit events). See [copyFolder](modules#copyfolder) for the semantics.

#### Parameters[[copyfolderiter.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `destination`: `Omit`\<[`CopyDestination`](interfaces/CopyDestination), ``"path"``\> & \{ `path?`: `string`  } ; `source`: `Omit`\<[`CopySource`](interfaces/CopySource), ``"path"``\> & \{ `path?`: `string`  }  } & `SharedParams` |

#### Returns[[copyfolderiter.returns]]

`AsyncGenerator`\<[`CopyProgressEvent`](interfaces/CopyProgressEvent), `undefined`\>

#### Defined in[[copyfolderiter.defined-in]]

[packages/hub/src/lib/copy-files.ts:297](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L297)

___

### countCommits

▸ **countCommits**(`params`): `Promise`\<`number`\>

#### Parameters[[countcommits.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[countcommits.returns]]

`Promise`\<`number`\>

#### Defined in[[countcommits.defined-in]]

[packages/hub/src/lib/count-commits.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/count-commits.ts#L7)

___

### createBranch

▸ **createBranch**(`params`): `Promise`\<`void`\>

#### Parameters[[createbranch.parameters]]

| Name | Type | Description |
| :------ | :------ | :------ |
| `params` | `Object` | - |
| `params.accessToken?` | `string` | - |
| `params.branch` | `string` | The name of the branch to create |
| `params.empty?` | `boolean` | Use this to create an empty branch, with no commits. |
| `params.fetch?` | (`input`: `URL` \| `RequestInfo`, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> | [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch) |
| `params.hubUrl?` | `string` | - |
| `params.overwrite?` | `boolean` | Use this to overwrite the branch if it already exists. If you only specify `overwrite` and no `revision`/`empty`, and the branch already exists, it will be a no-op. |
| `params.repo` | [`RepoDesignation`](modules#repodesignation) | - |
| `params.revision?` | `string` | Revision to create the branch from. Defaults to the default branch. Use empty: true to create an empty branch. |

#### Returns[[createbranch.returns]]

`Promise`\<`void`\>

#### Defined in[[createbranch.defined-in]]

[packages/hub/src/lib/create-branch.ts:6](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/create-branch.ts#L6)

___

### createCollection

▸ **createCollection**(`params`): `Promise`\<\{ `slug`: `string`  }\>

#### Parameters[[createcollection.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `collection`: `ApiCreateCollectionPayload` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[createcollection.returns]]

`Promise`\<\{ `slug`: `string`  }\>

#### Defined in[[createcollection.defined-in]]

[packages/hub/src/lib/create-collection.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/create-collection.ts#L7)

___

### createRepo

▸ **createRepo**(`params`): `Promise`\<\{ `id`: `string` ; `repoUrl`: `string`  }\>

#### Parameters[[createrepo.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `files?`: \{ `content`: `ArrayBuffer` \| `Blob` ; `path`: `string`  }[] ; `hubUrl?`: `string` ; `license?`: `string` ; `private?`: `boolean` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `resourceGroupId?`: `string` ; `sdk?`: SpaceSdk \| undefined ; `visibility?`: ``"public"`` \| ``"private"`` \| ``"protected"``  } & `CredentialsParams` |

#### Returns[[createrepo.returns]]

`Promise`\<\{ `id`: `string` ; `repoUrl`: `string`  }\>

#### Defined in[[createrepo.defined-in]]

[packages/hub/src/lib/create-repo.ts:9](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/create-repo.ts#L9)

___

### createScheduledJob

▸ **createScheduledJob**(`params`): `Promise`\<`ApiScheduledJob`\>

Create a scheduled job.

#### Parameters[[createscheduledjob.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `namespace`: `string`  } & CreateScheduledJobOptions & CredentialsParams |

#### Returns[[createscheduledjob.returns]]

`Promise`\<`ApiScheduledJob`\>

#### Defined in[[createscheduledjob.defined-in]]

[packages/hub/src/lib/jobs/create-scheduled-job.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/create-scheduled-job.ts#L11)

___

### datasetInfo

▸ **datasetInfo**\<`T`\>(`params`): `Promise`\<[`DatasetEntry`](interfaces/DatasetEntry) & `Pick`\<`ApiDatasetInfo`, `T`\>\>

#### Type parameters[[datasetinfo.type-parameters]]

| Name | Type |
| :------ | :------ |
| `T` | extends ``"author"`` \| ``"cardData"`` \| ``"disabled"`` \| ``"gitalyUid"`` \| ``"createdAt"`` \| ``"tags"`` \| ``"paperswithcode_id"`` \| ``"sha"`` \| ``"citation"`` \| ``"description"`` \| ``"downloadsAllTime"`` = `never` |

#### Parameters[[datasetinfo.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `additionalFields?`: `T`[] ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `name`: `string` ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[datasetinfo.returns]]

`Promise`\<[`DatasetEntry`](interfaces/DatasetEntry) & `Pick`\<`ApiDatasetInfo`, `T`\>\>

#### Defined in[[datasetinfo.defined-in]]

[packages/hub/src/lib/dataset-info.ts:9](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/dataset-info.ts#L9)

___

### deleteBranch

▸ **deleteBranch**(`params`): `Promise`\<`void`\>

#### Parameters[[deletebranch.parameters]]

| Name | Type | Description |
| :------ | :------ | :------ |
| `params` | `Object` | - |
| `params.accessToken?` | `string` | - |
| `params.branch` | `string` | The name of the branch to delete |
| `params.fetch?` | (`input`: `URL` \| `RequestInfo`, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> | [MDN Reference](https://developer.mozilla.org/docs/Web/API/Window/fetch) |
| `params.hubUrl?` | `string` | - |
| `params.repo` | [`RepoDesignation`](modules#repodesignation) | - |

#### Returns[[deletebranch.returns]]

`Promise`\<`void`\>

#### Defined in[[deletebranch.defined-in]]

[packages/hub/src/lib/delete-branch.ts:6](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/delete-branch.ts#L6)

___

### deleteCollection

▸ **deleteCollection**(`params`): `Promise`\<`void`\>

#### Parameters[[deletecollection.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `slug`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[deletecollection.returns]]

`Promise`\<`void`\>

#### Defined in[[deletecollection.defined-in]]

[packages/hub/src/lib/delete-collection.ts:6](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/delete-collection.ts#L6)

___

### deleteFile

▸ **deleteFile**(`params`): `Promise`\<[`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Parameters[[deletefile.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `branch?`: `string` ; `commitDescription?`: `string` ; `commitTitle?`: `string` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `isPullRequest?`: `boolean` ; `parentCommit?`: `string` ; `path`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation)  } & `CredentialsParams` |

#### Returns[[deletefile.returns]]

`Promise`\<[`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Defined in[[deletefile.defined-in]]

[packages/hub/src/lib/delete-file.ts:5](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/delete-file.ts#L5)

___

### deleteFiles

▸ **deleteFiles**(`params`): `Promise`\<[`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Parameters[[deletefiles.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `branch?`: `string` ; `commitDescription?`: `string` ; `commitTitle?`: `string` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `isPullRequest?`: `boolean` ; `parentCommit?`: `string` ; `paths`: `string`[] ; `repo`: [`RepoDesignation`](modules#repodesignation)  } & `CredentialsParams` |

#### Returns[[deletefiles.returns]]

`Promise`\<[`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Defined in[[deletefiles.defined-in]]

[packages/hub/src/lib/delete-files.ts:5](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/delete-files.ts#L5)

___

### deleteRepo

▸ **deleteRepo**(`params`): `Promise`\<`void`\>

#### Parameters[[deleterepo.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation)  } & `CredentialsParams` |

#### Returns[[deleterepo.returns]]

`Promise`\<`void`\>

#### Defined in[[deleterepo.defined-in]]

[packages/hub/src/lib/delete-repo.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/delete-repo.ts#L7)

___

### deleteScheduledJob

▸ **deleteScheduledJob**(`params`): `Promise`\<`void`\>

Delete a scheduled job.

#### Parameters[[deletescheduledjob.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[deletescheduledjob.returns]]

`Promise`\<`void`\>

#### Defined in[[deletescheduledjob.defined-in]]

[packages/hub/src/lib/jobs/delete-scheduled-job.ts:9](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/delete-scheduled-job.ts#L9)

___

### downloadFile

▸ **downloadFile**(`params`): `Promise`\<`Blob` \| ``null``\>

#### Parameters[[downloadfile.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `downloadInfo?`: [`FileDownloadInfoOutput`](interfaces/FileDownloadInfoOutput) ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `parallelDownloads?`: `boolean` \| [`ParallelDownloadOptions`](interfaces/ParallelDownloadOptions) ; `path`: `string` ; `raw?`: `boolean` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string` ; `xet?`: `boolean` \| \{ `readToken`: [`XetReadToken`](interfaces/XetReadToken)  }  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[downloadfile.returns]]

`Promise`\<`Blob` \| ``null``\>

null when the file doesn't exist

#### Defined in[[downloadfile.defined-in]]

[packages/hub/src/lib/download-file.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/download-file.ts#L13)

___

### downloadFileToCacheDir

▸ **downloadFileToCacheDir**(`params`): `Promise`\<`string`\>

Download a given file if it's not already present in the local cache.

#### Parameters[[downloadfiletocachedir.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `cacheDir?`: `string` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `path`: `string` ; `raw?`: `boolean` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[downloadfiletocachedir.returns]]

`Promise`\<`string`\>

the symlink to the blob object

#### Defined in[[downloadfiletocachedir.defined-in]]

[packages/hub/src/lib/download-file-to-cache-dir.ts:45](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/download-file-to-cache-dir.ts#L45)

___

### duplicateJob

▸ **duplicateJob**(`params`): `Promise`\<`ApiJob`\>

Duplicate a job (re-run with the same spec).

#### Parameters[[duplicatejob.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[duplicatejob.returns]]

`Promise`\<`ApiJob`\>

#### Defined in[[duplicatejob.defined-in]]

[packages/hub/src/lib/jobs/duplicate-job.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/duplicate-job.ts#L10)

___

### fileDownloadInfo

▸ **fileDownloadInfo**(`params`): `Promise`\<[`FileDownloadInfoOutput`](interfaces/FileDownloadInfoOutput) \| ``null``\>

#### Parameters[[filedownloadinfo.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `noContentDisposition?`: `boolean` ; `path`: `string` ; `raw?`: `boolean` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[filedownloadinfo.returns]]

`Promise`\<[`FileDownloadInfoOutput`](interfaces/FileDownloadInfoOutput) \| ``null``\>

null when the file doesn't exist

#### Defined in[[filedownloadinfo.defined-in]]

[packages/hub/src/lib/file-download-info.ts:27](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/file-download-info.ts#L27)

___

### fileExists

▸ **fileExists**(`params`): `Promise`\<`boolean`\>

#### Parameters[[fileexists.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `path`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[fileexists.returns]]

`Promise`\<`boolean`\>

#### Defined in[[fileexists.defined-in]]

[packages/hub/src/lib/file-exists.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/file-exists.ts#L7)

___

### getBlobStat

▸ **getBlobStat**(`blobPath`, `blobStats`): `Promise`\<`Stats`\>

#### Parameters[[getblobstat.parameters]]

| Name | Type |
| :------ | :------ |
| `blobPath` | `string` |
| `blobStats` | `Map`\<`string`, `Stats`\> |

#### Returns[[getblobstat.returns]]

`Promise`\<`Stats`\>

#### Defined in[[getblobstat.defined-in]]

[packages/hub/src/lib/cache-management.ts:254](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/cache-management.ts#L254)

___

### getHFHubCachePath

▸ **getHFHubCachePath**(): `string`

#### Returns[[gethfhubcachepath.returns]]

`string`

#### Defined in[[gethfhubcachepath.defined-in]]

[packages/hub/src/lib/cache-management.ts:19](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/cache-management.ts#L19)

___

### getJob

▸ **getJob**(`params`): `Promise`\<`ApiJob`\>

Get a specific job by ID.

#### Parameters[[getjob.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[getjob.returns]]

`Promise`\<`ApiJob`\>

#### Defined in[[getjob.defined-in]]

[packages/hub/src/lib/jobs/get-job.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/get-job.ts#L10)

___

### getQuantizationMultiplier

▸ **getQuantizationMultiplier**(`tensorName`, `dtype`, `quantConfig?`, `expertDtype?`): `number`

Gets the parameter multiplier for a quantized tensor based on quantization method.

May be fractional — see `packingFactor`.

#### Parameters[[getquantizationmultiplier.parameters]]

| Name | Type |
| :------ | :------ |
| `tensorName` | `string` |
| `dtype` | [`Dtype`](modules#dtype) |
| `quantConfig?` | [`QuantizationConfig`](interfaces/QuantizationConfig) |
| `expertDtype?` | `string` |

#### Returns[[getquantizationmultiplier.returns]]

`number`

#### Defined in[[getquantizationmultiplier.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:811](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L811)

___

### getRepoFolderName

▸ **getRepoFolderName**(`«destructured»`): `string`

#### Parameters[[getrepofoldername.parameters]]

| Name | Type |
| :------ | :------ |
| `«destructured»` | [`RepoId`](interfaces/RepoId) |

#### Returns[[getrepofoldername.returns]]

`string`

#### Defined in[[getrepofoldername.defined-in]]

[packages/hub/src/lib/cache-management.ts:27](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/cache-management.ts#L27)

___

### getScheduledJob

▸ **getScheduledJob**(`params`): `Promise`\<`ApiScheduledJob`\>

Get a specific scheduled job by ID.

#### Parameters[[getscheduledjob.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[getscheduledjob.returns]]

`Promise`\<`ApiScheduledJob`\>

#### Defined in[[getscheduledjob.defined-in]]

[packages/hub/src/lib/jobs/get-scheduled-job.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/get-scheduled-job.ts#L10)

___

### globMatch

▸ **globMatch**(`pattern`, `str`): `boolean`

Glob match without RegExp: splits pattern on `*` and checks that each literal
segment appears in order within `str`. Avoids RegExp entirely (no ReDoS risk,
no SyntaxError from attacker-controlled patterns in config.json).

#### Parameters[[globmatch.parameters]]

| Name | Type |
| :------ | :------ |
| `pattern` | `string` |
| `str` | `string` |

#### Returns[[globmatch.returns]]

`boolean`

#### Defined in[[globmatch.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:715](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L715)

___

### isQuantizedTensor

▸ **isQuantizedTensor**(`tensorName`, `quantConfig?`): `boolean`

Determines if a tensor is quantized based on quantization config and tensor name.

Python's transformers uses plain substring matching for `modules_to_not_convert`,
so bare names like `"lm_head"` must match `"model.lm_head.weight"`. When the
pattern contains a `*` we fall back to proper glob matching for flexibility.

#### Parameters[[isquantizedtensor.parameters]]

| Name | Type |
| :------ | :------ |
| `tensorName` | `string` |
| `quantConfig?` | [`QuantizationConfig`](interfaces/QuantizationConfig) |

#### Returns[[isquantizedtensor.returns]]

`boolean`

#### Defined in[[isquantizedtensor.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:751](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L751)

___

### listCollections

▸ **listCollections**(`params?`): `AsyncGenerator`\<`ApiCollectionInfo`\>

#### Parameters[[listcollections.parameters]]

| Name | Type |
| :------ | :------ |
| `params?` | \{ search?: \{ owner?: string[] \| undefined; item?: string[] \| undefined; q?: string \| undefined; } \| undefined; sort?: "lastModified" \| "trending" \| "upvotes" \| undefined; limit?: number \| undefined; hubUrl?: string \| undefined; fetch?: \{ ...; } \| undefined; } & Partial\<...\> |

#### Returns[[listcollections.returns]]

`AsyncGenerator`\<`ApiCollectionInfo`\>

#### Defined in[[listcollections.defined-in]]

[packages/hub/src/lib/list-collections.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-collections.ts#L12)

___

### listCommits

▸ **listCommits**(`params`): `AsyncGenerator`\<[`CommitData`](interfaces/CommitData)\>

#### Parameters[[listcommits.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `batchSize?`: `number` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[listcommits.returns]]

`AsyncGenerator`\<[`CommitData`](interfaces/CommitData)\>

#### Defined in[[listcommits.defined-in]]

[packages/hub/src/lib/list-commits.ts:17](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-commits.ts#L17)

___

### listDatasets

▸ **listDatasets**\<`T`\>(`params?`): `AsyncGenerator`\<[`DatasetEntry`](interfaces/DatasetEntry) & `Pick`\<`ApiDatasetInfo`, `T`\>\>

#### Type parameters[[listdatasets.type-parameters]]

| Name | Type |
| :------ | :------ |
| `T` | extends ``"author"`` \| ``"cardData"`` \| ``"disabled"`` \| ``"gitalyUid"`` \| ``"createdAt"`` \| ``"tags"`` \| ``"paperswithcode_id"`` \| ``"sha"`` \| ``"citation"`` \| ``"description"`` \| ``"downloadsAllTime"`` = `never` |

#### Parameters[[listdatasets.parameters]]

| Name | Type |
| :------ | :------ |
| `params?` | \{ search?: \{ query?: string \| undefined; owner?: string \| undefined; tags?: string[] \| undefined; } \| undefined; hubUrl?: string \| undefined; additionalFields?: T[] \| undefined; limit?: number \| undefined; sort?: "id" \| ... 8 more ... \| undefined; fetch?: \{ ...; } \| undefined; } & Partial\<...\> |

#### Returns[[listdatasets.returns]]

`AsyncGenerator`\<[`DatasetEntry`](interfaces/DatasetEntry) & `Pick`\<`ApiDatasetInfo`, `T`\>\>

#### Defined in[[listdatasets.defined-in]]

[packages/hub/src/lib/list-datasets.ts:47](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-datasets.ts#L47)

___

### listFiles

▸ **listFiles**(`params`): `AsyncGenerator`\<[`ListFileEntry`](interfaces/ListFileEntry)\>

List files in a folder. To list ALL files in the directory, call it
with params.recursive set to `true`.

#### Parameters[[listfiles.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `expand?`: `boolean` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `path?`: `string` ; `recursive?`: `boolean` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[listfiles.returns]]

`AsyncGenerator`\<[`ListFileEntry`](interfaces/ListFileEntry)\>

#### Defined in[[listfiles.defined-in]]

[packages/hub/src/lib/list-files.ts:53](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-files.ts#L53)

___

### listJobHardware

▸ **listJobHardware**(`params?`): `Promise`\<`ApiJobHardware`[]\>

Get the list of available hardware for jobs.
This endpoint is public and does not require authentication, but authentication is optional.

#### Parameters[[listjobhardware.parameters]]

| Name | Type |
| :------ | :------ |
| `params?` | \{ hubUrl?: string \| undefined; fetch?: \{ (input: URL \| RequestInfo, init?: RequestInit \| undefined): Promise\; (input: string \| ... 1 more ... \| Request, init?: RequestInit \| undefined): Promise\<...\>; } \| undefined; } & Partial\<...\> |

#### Returns[[listjobhardware.returns]]

`Promise`\<`ApiJobHardware`[]\>

#### Defined in[[listjobhardware.defined-in]]

[packages/hub/src/lib/jobs/list-job-hardware.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/list-job-hardware.ts#L11)

___

### listJobs

▸ **listJobs**(`params`): `Promise`\<`ApiJob`[]\>

List jobs for a namespace (user or organization).

#### Parameters[[listjobs.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[listjobs.returns]]

`Promise`\<`ApiJob`[]\>

#### Defined in[[listjobs.defined-in]]

[packages/hub/src/lib/jobs/list-jobs.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/list-jobs.ts#L10)

___

### listModels

▸ **listModels**\<`T`\>(`params?`): `AsyncGenerator`\<[`ModelEntry`](interfaces/ModelEntry) & [`ResolveModelAdditionalFields`](modules#resolvemodeladditionalfields)\<`T`\>\>

#### Type parameters[[listmodels.type-parameters]]

| Name | Type |
| :------ | :------ |
| `T` | extends [`ModelAdditionalField`](modules#modeladditionalfield) = `never` |

#### Parameters[[listmodels.parameters]]

| Name | Type |
| :------ | :------ |
| `params?` | \{ search?: \{ query?: string \| undefined; owner?: string \| undefined; task?: "other" \| "text-classification" \| "token-classification" \| "table-question-answering" \| "question-answering" \| ... 52 more ... \| undefined; tags?: string[] \| undefined; inferenceProviders?: string[] \| undefined; apps?: string[] \| undefined; ... |

#### Returns[[listmodels.returns]]

`AsyncGenerator`\<[`ModelEntry`](interfaces/ModelEntry) & [`ResolveModelAdditionalFields`](modules#resolvemodeladditionalfields)\<`T`\>\>

#### Defined in[[listmodels.defined-in]]

[packages/hub/src/lib/list-models.ts:68](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-models.ts#L68)

___

### listScheduledJobs

▸ **listScheduledJobs**(`params`): `Promise`\<`ApiScheduledJob`[]\>

List scheduled jobs for a namespace.

#### Parameters[[listscheduledjobs.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[listscheduledjobs.returns]]

`Promise`\<`ApiScheduledJob`[]\>

#### Defined in[[listscheduledjobs.defined-in]]

[packages/hub/src/lib/jobs/list-scheduled-jobs.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/list-scheduled-jobs.ts#L10)

___

### listSpaces

▸ **listSpaces**\<`T`\>(`params?`): `AsyncGenerator`\<[`SpaceEntry`](interfaces/SpaceEntry) & `Pick`\<`ApiSpaceInfo`, `T`\>\>

#### Type parameters[[listspaces.type-parameters]]

| Name | Type |
| :------ | :------ |
| `T` | extends ``"models"`` \| ``"datasets"`` \| ``"author"`` \| ``"cardData"`` \| ``"disabled"`` \| ``"gitalyUid"`` \| ``"createdAt"`` \| ``"tags"`` \| ``"sha"`` \| ``"subdomain"`` \| ``"runtime"`` = `never` |

#### Parameters[[listspaces.parameters]]

| Name | Type |
| :------ | :------ |
| `params?` | \{ search?: \{ query?: string \| undefined; owner?: string \| undefined; tags?: string[] \| undefined; } \| undefined; hubUrl?: string \| undefined; fetch?: \{ (input: URL \| RequestInfo, init?: RequestInit \| undefined): Promise\<...\>; (input: string \| ... 1 more ... \| Request, init?: RequestInit \| undefined): Promise\<...\>; }... |

#### Returns[[listspaces.returns]]

`AsyncGenerator`\<[`SpaceEntry`](interfaces/SpaceEntry) & `Pick`\<`ApiSpaceInfo`, `T`\>\>

#### Defined in[[listspaces.defined-in]]

[packages/hub/src/lib/list-spaces.ts:44](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/list-spaces.ts#L44)

___

### matchesCompressedTensorsTarget

▸ **matchesCompressedTensorsTarget**(`target`, `moduleName`): `boolean`

Matches a module name against a compressed-tensors target.

Targets are either exact module names, class names (e.g. `"Linear"`, which we
cannot resolve from tensor names and therefore ignore), or Python regexes
prefixed with `re:`. To avoid evaluating attacker-controlled RegExp from
config.json (ReDoS, SyntaxError — see globMatch), we translate the common
regex subset (`.*` wildcard, `^`/`$` anchors, `\.` escapes) to globMatch and
treat targets using any other regex syntax as non-matching.

#### Parameters[[matchescompressedtensorstarget.parameters]]

| Name | Type |
| :------ | :------ |
| `target` | `string` |
| `moduleName` | `string` |

#### Returns[[matchescompressedtensorstarget.returns]]

`boolean`

#### Defined in[[matchescompressedtensorstarget.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:783](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L783)

___

### modelInfo

▸ **modelInfo**\<`T`\>(`params`): `Promise`\<[`ModelEntry`](interfaces/ModelEntry) & [`ResolveModelAdditionalFields`](modules#resolvemodeladditionalfields)\<`T`\>\>

#### Type parameters[[modelinfo.type-parameters]]

| Name | Type |
| :------ | :------ |
| `T` | extends [`ModelAdditionalField`](modules#modeladditionalfield) = `never` |

#### Parameters[[modelinfo.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `additionalFields?`: `T`[] ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `name`: `string` ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[modelinfo.returns]]

`Promise`\<[`ModelEntry`](interfaces/ModelEntry) & [`ResolveModelAdditionalFields`](modules#resolvemodeladditionalfields)\<`T`\>\>

#### Defined in[[modelinfo.defined-in]]

[packages/hub/src/lib/model-info.ts:16](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/model-info.ts#L16)

___

### oauthHandleRedirect

▸ **oauthHandleRedirect**(`opts?`): `Promise`\<[`OAuthResult`](interfaces/OAuthResult)\>

To call after the OAuth provider redirects back to the app.

There is also a helper function [oauthHandleRedirectIfPresent](modules#oauthhandleredirectifpresent), which will call `oauthHandleRedirect` if the URL contains an oauth code
in the query parameters and return `false` otherwise.

#### Parameters[[oauthhandleredirect.parameters]]

| Name | Type | Description |
| :------ | :------ | :------ |
| `opts?` | `Object` | - |
| `opts.codeVerifier?` | `string` | codeVerifier generated by oauthLoginUrl **`Default`** ```ts localStorage.getItem("huggingface.co:oauth:code_verifier") ``` |
| `opts.hubUrl?` | `string` | The URL of the hub. Defaults to [HUB_URL](modules#hub_url). |
| `opts.nonce?` | `string` | nonce generated by oauthLoginUrl **`Default`** ```ts localStorage.getItem("huggingface.co:oauth:nonce") ``` |
| `opts.redirectedUrl?` | `string` | The URL to analyze. **`Default`** ```ts window.location.href ``` |

#### Returns[[oauthhandleredirect.returns]]

`Promise`\<[`OAuthResult`](interfaces/OAuthResult)\>

#### Defined in[[oauthhandleredirect.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:123](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L123)

___

### oauthHandleRedirectIfPresent

▸ **oauthHandleRedirectIfPresent**(`opts?`): `Promise`\<[`OAuthResult`](interfaces/OAuthResult) \| ``false``\>

To call after the OAuth provider redirects back to the app.

It returns false if the URL does not contain an oauth code in the query parameters, otherwise
it calls [oauthHandleRedirect](modules#oauthhandleredirect).

Depending on your app, you may want to call [oauthHandleRedirect](modules#oauthhandleredirect) directly instead.

#### Parameters[[oauthhandleredirectifpresent.parameters]]

| Name | Type | Description |
| :------ | :------ | :------ |
| `opts?` | `Object` | - |
| `opts.codeVerifier?` | `string` | codeVerifier generated by oauthLoginUrl **`Default`** ```ts localStorage.getItem("huggingface.co:oauth:code_verifier") ``` |
| `opts.hubUrl?` | `string` | The URL of the hub. Defaults to [HUB_URL](modules#hub_url). |
| `opts.nonce?` | `string` | nonce generated by oauthLoginUrl **`Default`** ```ts localStorage.getItem("huggingface.co:oauth:nonce") ``` |
| `opts.redirectedUrl?` | `string` | The URL to analyze. **`Default`** ```ts window.location.href ``` |

#### Returns[[oauthhandleredirectifpresent.returns]]

`Promise`\<[`OAuthResult`](interfaces/OAuthResult) \| ``false``\>

#### Defined in[[oauthhandleredirectifpresent.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:293](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L293)

___

### oauthLoginUrl

▸ **oauthLoginUrl**(`opts?`): `Promise`\<`string`\>

Use "Sign in with Hub" to authenticate a user, and get oauth user info / access token.

Returns an url to redirect to. After the user is redirected back to your app, call `oauthHandleRedirect` to get the oauth user info / access token.

When called from inside a static Space with OAuth enabled, it will load the config from the space, otherwise you need to at least specify
the client ID of your OAuth App.

#### Parameters[[oauthloginurl.parameters]]

| Name | Type | Description |
| :------ | :------ | :------ |
| `opts?` | `Object` | - |
| `opts.clientId?` | `string` | OAuth client ID. For static Spaces, you can omit this and it will be loaded from the Space config, as long as `hf_oauth: true` is present in the README.md's metadata. For other Spaces, it is available to the backend in the OAUTH_CLIENT_ID environment variable, as long as `hf_oauth: true` is present in the README.md's metadata. You can also create a Developer Application at https://huggingface.co/settings/connected-applications and use its client ID. |
| `opts.hubUrl?` | `string` | - |
| `opts.localStorage?` | `Object` | If provided, will be filled with the code verifier and nonce used for the OAuth flow, instead of using localStorage. When calling `oauthHandleRedirectIfPresent` or `oauthHandleRedirect` you will need to provide the same values. |
| `opts.localStorage.codeVerifier?` | `string` | - |
| `opts.localStorage.nonce?` | `string` | - |
| `opts.redirectUrl?` | `string` | Redirect URI, defaults to the current URL. For Spaces, any URL within the Space is allowed. For Developer Applications, you can add any URL you want to the list of allowed redirect URIs at https://huggingface.co/settings/connected-applications. |
| `opts.scopes?` | `string` | OAuth scope, a list of space-separated scopes. For static Spaces, you can omit this and it will be loaded from the Space config, as long as `hf_oauth: true` is present in the README.md's metadata. For other Spaces, it is available to the backend in the OAUTH_SCOPES environment variable, as long as `hf_oauth: true` is present in the README.md's metadata. Defaults to "openid profile". You can also create a Developer Application at https://huggingface.co/settings/connected-applications and use its scopes. See https://huggingface.co/docs/hub/oauth for a list of available scopes. |
| `opts.state?` | `string` | State to pass to the OAuth provider, which will be returned in the call to `oauthLogin` after the redirect. |

#### Returns[[oauthloginurl.returns]]

`Promise`\<`string`\>

**`Example`**

```ts
import { oauthLoginUrl, oauthHandleRedirectIfPresent } from "@huggingface/hub";

const oauthResult = await oauthHandleRedirectIfPresent();

if (!oauthResult) {
  // If the user is not logged in, redirect to the login page
  window.location.href = await oauthLoginUrl();
}

// You can use oauthResult.accessToken, oauthResult.accessTokenExpiresAt and oauthResult.userInfo
console.log(oauthResult);
```

(Theoretically, this function could be used to authenticate a user for any OAuth provider supporting PKCE and OpenID Connect by changing `hubUrl`,
but it is currently only tested with the Hugging Face Hub.)

#### Defined in[[oauthloginurl.defined-in]]

[packages/hub/src/lib/oauth-login-url.ts:31](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-login-url.ts#L31)

___

### parseRepoType

▸ **parseRepoType**(`type`): [`RepoType`](modules#repotype)

#### Parameters[[parserepotype.parameters]]

| Name | Type |
| :------ | :------ |
| `type` | `string` |

#### Returns[[parserepotype.returns]]

[`RepoType`](modules#repotype)

#### Defined in[[parserepotype.defined-in]]

[packages/hub/src/lib/cache-management.ts:264](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/cache-management.ts#L264)

___

### parseSafetensorsMetadata

▸ **parseSafetensorsMetadata**(`params`): `Promise`\<`SetRequired`\<[`SafetensorsParseFromRepo`](modules#safetensorsparsefromrepo), ``"parameterCount"``\>\>

Analyze model.safetensors.index.json or model.safetensors from a model hosted
on Hugging Face using smart range requests to extract its metadata.

#### Parameters[[parsesafetensorsmetadata.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `computeParametersCount`: ``true`` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `library?`: `string` ; `path?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[parsesafetensorsmetadata.returns]]

`Promise`\<`SetRequired`\<[`SafetensorsParseFromRepo`](modules#safetensorsparsefromrepo), ``"parameterCount"``\>\>

#### Defined in[[parsesafetensorsmetadata.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:522](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L522)

▸ **parseSafetensorsMetadata**(`params`): `Promise`\<[`SafetensorsParseFromRepo`](modules#safetensorsparsefromrepo)\>

#### Parameters[[parsesafetensorsmetadata.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `computeParametersCount?`: `boolean` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `library?`: `string` ; `path?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[parsesafetensorsmetadata.returns]]

`Promise`\<[`SafetensorsParseFromRepo`](modules#safetensorsparsefromrepo)\>

#### Defined in[[parsesafetensorsmetadata.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:552](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L552)

___

### parseSafetensorsShardFilename

▸ **parseSafetensorsShardFilename**(`filename`): [`SafetensorsShardFileInfo`](interfaces/SafetensorsShardFileInfo) \| ``null``

#### Parameters[[parsesafetensorsshardfilename.parameters]]

| Name | Type |
| :------ | :------ |
| `filename` | `string` |

#### Returns[[parsesafetensorsshardfilename.returns]]

[`SafetensorsShardFileInfo`](interfaces/SafetensorsShardFileInfo) \| ``null``

#### Defined in[[parsesafetensorsshardfilename.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:31](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L31)

___

### parseTotalParameters

▸ **parseTotalParameters**(`value`, `computedTotal?`): `number` \| `undefined`

Reads the `total_parameters` shortcut from the header/index metadata. It's self-reported and
the Hub displays it verbatim, bypassing the header validation above — so cap it at the
computed count when we have one (a no-op for well-formed files, where the two agree).

#### Parameters[[parsetotalparameters.parameters]]

| Name | Type |
| :------ | :------ |
| `value` | `undefined` \| `string` \| `number` |
| `computedTotal?` | `number` |

#### Returns[[parsetotalparameters.returns]]

`number` \| `undefined`

#### Defined in[[parsetotalparameters.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:483](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L483)

___

### pathsInfo

▸ **pathsInfo**(`params`): `Promise`\<[`PathInfo`](interfaces/PathInfo) & \{ `lastCommit`: [`CommitInfo`](interfaces/CommitInfo) ; `securityFileStatus`: [`SecurityFileStatus`](interfaces/SecurityFileStatus)  }[]\>

#### Parameters[[pathsinfo.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `expand`: ``true`` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `paths`: `string`[] ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[pathsinfo.returns]]

`Promise`\<[`PathInfo`](interfaces/PathInfo) & \{ `lastCommit`: [`CommitInfo`](interfaces/CommitInfo) ; `securityFileStatus`: [`SecurityFileStatus`](interfaces/SecurityFileStatus)  }[]\>

#### Defined in[[pathsinfo.defined-in]]

[packages/hub/src/lib/paths-info.ts:51](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L51)

▸ **pathsInfo**(`params`): `Promise`\<[`PathInfo`](interfaces/PathInfo)[]\>

#### Parameters[[pathsinfo.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `expand?`: `boolean` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `paths`: `string`[] ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[pathsinfo.returns]]

`Promise`\<[`PathInfo`](interfaces/PathInfo)[]\>

#### Defined in[[pathsinfo.defined-in]]

[packages/hub/src/lib/paths-info.ts:64](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/paths-info.ts#L64)

___

### relativeUnderFolder

▸ **relativeUnderFolder**(`filePath`, `folderPath`): `string`

Compute the path of `filePath` relative to `folderPath`. Used to map source paths
under a folder being copied to destination paths under the new prefix.

#### Parameters[[relativeunderfolder.parameters]]

| Name | Type |
| :------ | :------ |
| `filePath` | `string` |
| `folderPath` | `string` |

#### Returns[[relativeunderfolder.returns]]

`string`

#### Defined in[[relativeunderfolder.defined-in]]

[packages/hub/src/lib/copy-files.ts:558](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/copy-files.ts#L558)

___

### repoExists

▸ **repoExists**(`params`): `Promise`\<`boolean`\>

#### Parameters[[repoexists.parameters]]

| Name | Type | Description |
| :------ | :------ | :------ |
| `params` | `Object` | - |
| `params.accessToken?` | `string` | - |
| `params.fetch?` | (`input`: `URL` \| `RequestInfo`, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> | Custom fetch function to use instead of the default one, for example to use a proxy or edit headers. |
| `params.hubUrl?` | `string` | - |
| `params.repo` | [`RepoDesignation`](modules#repodesignation) | - |
| `params.revision?` | `string` | An optional Git revision id which can be a branch name, a tag, or a commit hash. |

#### Returns[[repoexists.returns]]

`Promise`\<`boolean`\>

#### Defined in[[repoexists.defined-in]]

[packages/hub/src/lib/repo-exists.ts:6](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/repo-exists.ts#L6)

___

### resumeScheduledJob

▸ **resumeScheduledJob**(`params`): `Promise`\<`void`\>

Resume a scheduled job.

#### Parameters[[resumescheduledjob.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[resumescheduledjob.returns]]

`Promise`\<`void`\>

#### Defined in[[resumescheduledjob.defined-in]]

[packages/hub/src/lib/jobs/resume-scheduled-job.ts:9](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/resume-scheduled-job.ts#L9)

___

### runJob

▸ **runJob**(`params`): `Promise`\<`ApiJob`\>

Run a new job.

#### Parameters[[runjob.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `namespace`: `string`  } & CreateJobOptions & CredentialsParams |

#### Returns[[runjob.returns]]

`Promise`\<`ApiJob`\>

#### Defined in[[runjob.defined-in]]

[packages/hub/src/lib/jobs/run-job.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/run-job.ts#L12)

___

### runScheduledJob

▸ **runScheduledJob**(`params`): `Promise`\<`ApiJob` \| ``null``\>

Trigger a scheduled job to run immediately.
Returns the job that was triggered, or null if another instance is already running.

#### Parameters[[runscheduledjob.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[runscheduledjob.returns]]

`Promise`\<`ApiJob` \| ``null``\>

#### Defined in[[runscheduledjob.defined-in]]

[packages/hub/src/lib/jobs/run-scheduled-job.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/run-scheduled-job.ts#L11)

___

### scanCacheDir

▸ **scanCacheDir**(`cacheDir?`): `Promise`\<[`HFCacheInfo`](interfaces/HFCacheInfo)\>

#### Parameters[[scancachedir.parameters]]

| Name | Type | Default value |
| :------ | :------ | :------ |
| `cacheDir` | `undefined` \| `string` | `undefined` |

#### Returns[[scancachedir.returns]]

`Promise`\<[`HFCacheInfo`](interfaces/HFCacheInfo)\>

#### Defined in[[scancachedir.defined-in]]

[packages/hub/src/lib/cache-management.ts:72](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/cache-management.ts#L72)

___

### scanCachedRepo

▸ **scanCachedRepo**(`repoPath`): `Promise`\<[`CachedRepoInfo`](interfaces/CachedRepoInfo)\>

#### Parameters[[scancachedrepo.parameters]]

| Name | Type |
| :------ | :------ |
| `repoPath` | `string` |

#### Returns[[scancachedrepo.returns]]

`Promise`\<[`CachedRepoInfo`](interfaces/CachedRepoInfo)\>

#### Defined in[[scancachedrepo.defined-in]]

[packages/hub/src/lib/cache-management.ts:118](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/cache-management.ts#L118)

___

### scanRefsDir

▸ **scanRefsDir**(`refsPath`, `refsByHash`): `Promise`\<`void`\>

#### Parameters[[scanrefsdir.parameters]]

| Name | Type |
| :------ | :------ |
| `refsPath` | `string` |
| `refsByHash` | `Map`\<`string`, `string`[]\> |

#### Returns[[scanrefsdir.returns]]

`Promise`\<`void`\>

#### Defined in[[scanrefsdir.defined-in]]

[packages/hub/src/lib/cache-management.ts:210](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/cache-management.ts#L210)

___

### scanSnapshotDir

▸ **scanSnapshotDir**(`revisionPath`, `cachedFiles`, `blobStats`): `Promise`\<`void`\>

#### Parameters[[scansnapshotdir.parameters]]

| Name | Type |
| :------ | :------ |
| `revisionPath` | `string` |
| `cachedFiles` | [`CachedFileInfo`](interfaces/CachedFileInfo)[] |
| `blobStats` | `Map`\<`string`, `Stats`\> |

#### Returns[[scansnapshotdir.returns]]

`Promise`\<`void`\>

#### Defined in[[scansnapshotdir.defined-in]]

[packages/hub/src/lib/cache-management.ts:227](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/cache-management.ts#L227)

___

### snapshotDownload

▸ **snapshotDownload**(`params`): `Promise`\<`string`\>

Downloads an entire repository at a given revision in the cache directory [getHFHubCachePath](modules#gethfhubcachepath).
You can list all cached repositories using [scanCachedRepo](modules#scancachedrepo)

#### Parameters[[snapshotdownload.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `cacheDir?`: `string` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[snapshotdownload.returns]]

`Promise`\<`string`\>

**`Remarks`**

It uses internally [downloadFileToCacheDir](modules#downloadfiletocachedir).

#### Defined in[[snapshotdownload.defined-in]]

[packages/hub/src/lib/snapshot-download.ts:19](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/snapshot-download.ts#L19)

___

### spaceInfo

▸ **spaceInfo**\<`T`\>(`params`): `Promise`\<[`SpaceEntry`](interfaces/SpaceEntry) & `Pick`\<`ApiSpaceInfo`, `T`\>\>

#### Type parameters[[spaceinfo.type-parameters]]

| Name | Type |
| :------ | :------ |
| `T` | extends ``"models"`` \| ``"datasets"`` \| ``"author"`` \| ``"cardData"`` \| ``"disabled"`` \| ``"gitalyUid"`` \| ``"createdAt"`` \| ``"tags"`` \| ``"sha"`` \| ``"subdomain"`` \| ``"runtime"`` = `never` |

#### Parameters[[spaceinfo.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `additionalFields?`: `T`[] ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `name`: `string` ; `revision?`: `string`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[spaceinfo.returns]]

`Promise`\<[`SpaceEntry`](interfaces/SpaceEntry) & `Pick`\<`ApiSpaceInfo`, `T`\>\>

#### Defined in[[spaceinfo.defined-in]]

[packages/hub/src/lib/space-info.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/space-info.ts#L10)

___

### streamJobEvents

▸ **streamJobEvents**(`params`): `AsyncGenerator`\<`string`, `void`, `unknown`\>

Stream job events using Server-Sent Events (SSE).
Returns an async iterable of event chunks.

#### Parameters[[streamjobevents.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[streamjobevents.returns]]

`AsyncGenerator`\<`string`, `void`, `unknown`\>

#### Defined in[[streamjobevents.defined-in]]

[packages/hub/src/lib/jobs/stream-job-events.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/stream-job-events.ts#L10)

___

### streamJobLogs

▸ **streamJobLogs**(`params`): `AsyncGenerator`\<\{ `message`: `string` ; `timestamp`: `Date`  }, `void`, `unknown`\>

Stream job logs using Server-Sent Events (SSE).
Returns an async iterable of log chunks.

#### Parameters[[streamjoblogs.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[streamjoblogs.returns]]

`AsyncGenerator`\<\{ `message`: `string` ; `timestamp`: `Date`  }, `void`, `unknown`\>

#### Defined in[[streamjoblogs.defined-in]]

[packages/hub/src/lib/jobs/stream-job-logs.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/stream-job-logs.ts#L10)

___

### streamJobMetrics

▸ **streamJobMetrics**(`params`): `AsyncGenerator`\<`string`, `void`, `unknown`\>

Stream job metrics using Server-Sent Events (SSE).
Returns an async iterable of metric chunks.

#### Parameters[[streamjobmetrics.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[streamjobmetrics.returns]]

`AsyncGenerator`\<`string`, `void`, `unknown`\>

#### Defined in[[streamjobmetrics.defined-in]]

[packages/hub/src/lib/jobs/stream-job-metrics.ts:10](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/stream-job-metrics.ts#L10)

___

### suspendScheduledJob

▸ **suspendScheduledJob**(`params`): `Promise`\<`void`\>

Suspend (pause) a scheduled job.

#### Parameters[[suspendscheduledjob.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string` ; `jobId`: `string` ; `namespace`: `string`  } & `CredentialsParams` |

#### Returns[[suspendscheduledjob.returns]]

`Promise`\<`void`\>

#### Defined in[[suspendscheduledjob.defined-in]]

[packages/hub/src/lib/jobs/suspend-scheduled-job.ts:9](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/jobs/suspend-scheduled-job.ts#L9)

___

### uploadFile

▸ **uploadFile**(`params`): `Promise`\<[`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Parameters[[uploadfile.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `abortSignal?`: `AbortSignal` ; `branch?`: `string` ; `commitDescription?`: `string` ; `commitTitle?`: `string` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `file`: `URL` \| `File` \| \{ `content`: [`ContentSource`](modules#contentsource) ; `path`: `string`  } ; `hubUrl?`: `string` ; `isPullRequest?`: `boolean` ; `parentCommit?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `useWebWorkers?`: `boolean` \| \{ `minSize?`: `number` ; `poolSize?`: `number`  } ; `useXet?`: `boolean`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[uploadfile.returns]]

`Promise`\<[`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Defined in[[uploadfile.defined-in]]

[packages/hub/src/lib/upload-file.ts:5](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/upload-file.ts#L5)

___

### uploadFiles

▸ **uploadFiles**(`params`): `Promise`\<[`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Parameters[[uploadfiles.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `abortSignal?`: `AbortSignal` ; `branch?`: `string` ; `commitDescription?`: `string` ; `commitTitle?`: `string` ; `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `files`: (`URL` \| `File` \| \{ `content`: [`ContentSource`](modules#contentsource) ; `path`: `string`  })[] ; `hubUrl?`: `string` ; `isPullRequest?`: `boolean` ; `maxFolderDepth?`: `number` ; `parentCommit?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `useWebWorkers?`: `boolean` \| \{ `minSize?`: `number` ; `poolSize?`: `number`  } ; `useXet?`: `boolean`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[uploadfiles.returns]]

`Promise`\<[`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Defined in[[uploadfiles.defined-in]]

[packages/hub/src/lib/upload-files.ts:5](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/upload-files.ts#L5)

___

### uploadFilesWithProgress

▸ **uploadFilesWithProgress**(`params`): `AsyncGenerator`\<[`CommitProgressEvent`](modules#commitprogressevent), [`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

Uploads with progress

Needs XMLHttpRequest to be available for progress events for uploads on models, datasets and spaces.
Set useWebWorkers to true in order to have progress events for hashing for models, datasets and spaces.

#### Parameters[[uploadfileswithprogress.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `abortSignal?`: `AbortSignal` ; `branch?`: `string` ; `commitDescription?`: `string` ; `commitTitle?`: `string` ; `files`: (`URL` \| `File` \| \{ `content`: [`ContentSource`](modules#contentsource) ; `path`: `string`  })[] ; `hubUrl?`: `string` ; `isPullRequest?`: `boolean` ; `maxFolderDepth?`: `number` ; `parentCommit?`: `string` ; `repo`: [`RepoDesignation`](modules#repodesignation) ; `useWebWorkers?`: `boolean` \| \{ `minSize?`: `number` ; `poolSize?`: `number`  } ; `useXet?`: `boolean`  } & `Partial`\<`CredentialsParams`\> |

#### Returns[[uploadfileswithprogress.returns]]

`AsyncGenerator`\<[`CommitProgressEvent`](modules#commitprogressevent), [`CommitOutput`](interfaces/CommitOutput) \| `undefined`\>

#### Defined in[[uploadfileswithprogress.defined-in]]

[packages/hub/src/lib/upload-files-with-progress.ts:20](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/upload-files-with-progress.ts#L20)

___

### validateTensorEntry

▸ **validateTensorEntry**(`path`, `tensorName`, `info`, `fileSizeBytes`): `void`

Validates one tensor entry from a header: dims are finite integers under `MAX_TENSOR_DIM`,
`data_offsets` are sane, and — when the file size is known — the declared byte span fits in
the file. `data_offsets` is checked rather than `shape * dtype size` because the two may
legitimately disagree (padding), so offsets are the only ground truth the format gives us.

#### Parameters[[validatetensorentry.parameters]]

| Name | Type |
| :------ | :------ |
| `path` | `string` |
| `tensorName` | `string` |
| `info` | [`TensorInfo`](interfaces/TensorInfo) |
| `fileSizeBytes` | `undefined` \| `number` |

#### Returns[[validatetensorentry.returns]]

`void`

#### Defined in[[validatetensorentry.defined-in]]

[packages/hub/src/lib/parse-safetensors-metadata.ts:201](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/parse-safetensors-metadata.ts#L201)

___

### whoAmI

▸ **whoAmI**(`params`): `Promise`\<[`WhoAmI`](modules#whoami) & \{ `auth`: [`AuthInfo`](interfaces/AuthInfo)  }\>

#### Parameters[[whoami.parameters]]

| Name | Type |
| :------ | :------ |
| `params` | \{ `fetch?`: (`input`: URL \| RequestInfo, `init?`: `RequestInit`) => `Promise`\<`Response`\>(`input`: `string` \| `URL` \| `Request`, `init?`: `RequestInit`) => `Promise`\<`Response`\> ; `hubUrl?`: `string`  } & `CredentialsParams` |

#### Returns[[whoami.returns]]

`Promise`\<[`WhoAmI`](modules#whoami) & \{ `auth`: [`AuthInfo`](interfaces/AuthInfo)  }\>

#### Defined in[[whoami.defined-in]]

[packages/hub/src/lib/who-am-i.ts:62](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/who-am-i.ts#L62)
