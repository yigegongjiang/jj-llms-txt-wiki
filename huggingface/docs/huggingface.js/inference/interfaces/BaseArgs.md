# Interface: BaseArgs

## Properties

### accessToken

• `Optional` **accessToken**: `string`

The access token to use. Without it, you'll get rate-limited quickly.

Can be created for free in hf.co/settings/token

You can also pass an external Inference provider's key if you intend to call a compatible provider like Together, Replicate, Cohere...

#### Defined in[[accesstoken.defined-in]]

[inference/src/types.ts:121](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L121)

___

### endpointUrl

• `Optional` **endpointUrl**: `string`

The URL of the endpoint to use.

If not specified, will call the default router.huggingface.co Inference Providers endpoint.

#### Defined in[[endpointurl.defined-in]]

[inference/src/types.ts:138](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L138)

___

### model

• `Optional` **model**: `string`

The HF model to use.

If not specified, will call huggingface.co/api/tasks to get the default model for the task.

/!\ Legacy behavior allows this to be an URL, but this is deprecated and will be removed in the future.
Use the `endpointUrl` parameter instead.

#### Defined in[[model.defined-in]]

[inference/src/types.ts:131](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L131)

___

### provider

• `Optional` **provider**: ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"``

Set an Inference provider to run this model on.

Defaults to "auto" i.e. the first of the providers available for the model, sorted by the user's order in https://hf.co/settings/inference-providers.

#### Defined in[[provider.defined-in]]

[inference/src/types.ts:145](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L145)
