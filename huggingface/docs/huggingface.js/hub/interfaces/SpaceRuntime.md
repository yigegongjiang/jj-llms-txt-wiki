# Interface: SpaceRuntime

## Properties

### errorMessage

• `Optional` **errorMessage**: `string`

#### Defined in[[errormessage.defined-in]]

[packages/hub/src/types/public.ts:100](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L100)

___

### gcTimeout

• `Optional` **gcTimeout**: ``null`` \| `number`

in seconds

#### Defined in[[gctimeout.defined-in]]

[packages/hub/src/types/public.ts:110](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L110)

___

### hardware

• `Optional` **hardware**: `Object`

#### Type declaration[[hardware.type-declaration]]

| Name | Type |
| :------ | :------ |
| `current` | ``null`` \| [`SpaceHardwareFlavor`](../modules#spacehardwareflavor) |
| `currentPrettyName?` | `string` |
| `requested` | ``null`` \| [`SpaceHardwareFlavor`](../modules#spacehardwareflavor) |
| `requestedPrettyName?` | `string` |

#### Defined in[[hardware.defined-in]]

[packages/hub/src/types/public.ts:101](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L101)

___

### resources

• `Optional` **resources**: [`SpaceResourceConfig`](SpaceResourceConfig)

when calling /spaces, those props are only fetched if ?full=true

#### Defined in[[resources.defined-in]]

[packages/hub/src/types/public.ts:108](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L108)

___

### sdk

• `Optional` **sdk**: [`SpaceSdk`](../modules#spacesdk)

#### Defined in[[sdk.defined-in]]

[packages/hub/src/types/public.ts:98](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L98)

___

### sdkVersion

• `Optional` **sdkVersion**: `string`

#### Defined in[[sdkversion.defined-in]]

[packages/hub/src/types/public.ts:99](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L99)

___

### stage

• **stage**: [`SpaceStage`](../modules#spacestage)

#### Defined in[[stage.defined-in]]

[packages/hub/src/types/public.ts:97](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/types/public.ts#L97)
