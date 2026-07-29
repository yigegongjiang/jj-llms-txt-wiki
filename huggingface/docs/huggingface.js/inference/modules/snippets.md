# Namespace: snippets

## Type Aliases

### InferenceSnippetOptions

Ƭ **InferenceSnippetOptions**: \{ `accessToken?`: `string` ; `billTo?`: `string` ; `directRequest?`: `boolean` ; `endpointUrl?`: `string` ; `inputs?`: `Record`\<`string`, `unknown`\> ; `streaming?`: `boolean`  } & `Record`\<`string`, `unknown`\>

#### Defined in[[inferencesnippetoptions.defined-in]]

[inference/src/snippets/getInferenceSnippets.ts:18](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/snippets/getInferenceSnippets.ts#L18)

## Functions

### getInferenceSnippets

▸ **getInferenceSnippets**(`model`, `provider`, `inferenceProviderMapping?`, `opts?`): `InferenceSnippet`[]

#### Parameters[[getinferencesnippets.parameters]]

| Name | Type |
| :------ | :------ |
| `model` | `ModelDataMinimal` |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `inferenceProviderMapping?` | [`InferenceProviderMappingEntry`](../interfaces/InferenceProviderMappingEntry) |
| `opts?` | `Record`\<`string`, `unknown`\> |

#### Returns[[getinferencesnippets.returns]]

`InferenceSnippet`[]

#### Defined in[[getinferencesnippets.defined-in]]

[inference/src/snippets/getInferenceSnippets.ts:421](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/snippets/getInferenceSnippets.ts#L421)
