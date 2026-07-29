# Interface: TextGenerationStreamBestOfSequence

## Properties

### finish\_reason

• **finish\_reason**: [`TextGenerationStreamFinishReason`](../modules#textgenerationstreamfinishreason)

Generation finish reason

#### Defined in[[finishreason.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:37](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L37)

___

### generated\_text

• **generated\_text**: `string`

Generated text

#### Defined in[[generatedtext.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:35](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L35)

___

### generated\_tokens

• **generated\_tokens**: `number`

Number of generated tokens

#### Defined in[[generatedtokens.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:39](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L39)

___

### prefill

• **prefill**: [`TextGenerationStreamPrefillToken`](TextGenerationStreamPrefillToken)[]

Prompt tokens

#### Defined in[[prefill.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:43](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L43)

___

### seed

• `Optional` **seed**: `number`

Sampling seed if sampling was activated

#### Defined in[[seed.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:41](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L41)

___

### tokens

• **tokens**: [`TextGenerationStreamToken`](TextGenerationStreamToken)[]

Generated tokens

#### Defined in[[tokens.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:45](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L45)
