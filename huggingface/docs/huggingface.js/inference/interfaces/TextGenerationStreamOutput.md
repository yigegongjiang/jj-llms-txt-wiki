# Interface: TextGenerationStreamOutput

## Properties

### details

• **details**: ``null`` \| [`TextGenerationStreamDetails`](TextGenerationStreamDetails)

Generation details
Only available when the generation is finished

#### Defined in[[details.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:84](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L84)

___

### generated\_text

• **generated\_text**: ``null`` \| `string`

Complete generated text
Only available when the generation is finished

#### Defined in[[generatedtext.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:79](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L79)

___

### index

• `Optional` **index**: `number`

#### Defined in[[index.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:72](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L72)

___

### token

• **token**: [`TextGenerationStreamToken`](TextGenerationStreamToken)

Generated token, one at a time

#### Defined in[[token.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:74](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L74)
