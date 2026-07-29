# @huggingface/inference

## Namespaces

- [snippets](modules/snippets)

## Classes

- [HfInference](classes/HfInference)
- [InferenceClient](classes/InferenceClient)
- [InferenceClientEndpoint](classes/InferenceClientEndpoint)
- [InferenceClientError](classes/InferenceClientError)
- [InferenceClientHubApiError](classes/InferenceClientHubApiError)
- [InferenceClientInputError](classes/InferenceClientInputError)
- [InferenceClientProviderApiError](classes/InferenceClientProviderApiError)
- [InferenceClientProviderOutputError](classes/InferenceClientProviderOutputError)
- [InferenceClientRoutingError](classes/InferenceClientRoutingError)

## Interfaces

- [AudioToAudioOutput](interfaces/AudioToAudioOutput)
- [AudioToAudioOutputElem](interfaces/AudioToAudioOutputElem)
- [BaseArgs](interfaces/BaseArgs)
- [BodyParams](interfaces/BodyParams)
- [HeaderParams](interfaces/HeaderParams)
- [InferenceProviderMappingEntry](interfaces/InferenceProviderMappingEntry)
- [Logger](interfaces/Logger)
- [Options](interfaces/Options)
- [TextGenerationInput](interfaces/TextGenerationInput)
- [TextGenerationOutput](interfaces/TextGenerationOutput)
- [TextGenerationStreamBestOfSequence](interfaces/TextGenerationStreamBestOfSequence)
- [TextGenerationStreamDetails](interfaces/TextGenerationStreamDetails)
- [TextGenerationStreamOutput](interfaces/TextGenerationStreamOutput)
- [TextGenerationStreamPrefillToken](interfaces/TextGenerationStreamPrefillToken)
- [TextGenerationStreamToken](interfaces/TextGenerationStreamToken)
- [UrlParams](interfaces/UrlParams)

## Type Aliases

### AudioClassificationArgs

Ƭ **AudioClassificationArgs**: [`BaseArgs`](interfaces/BaseArgs) & `AudioClassificationInput` \| `LegacyAudioInput`

#### Defined in[[audioclassificationargs.defined-in]]

[inference/src/tasks/audio/audioClassification.ts:9](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/audioClassification.ts#L9)

___

### AudioToAudioArgs

Ƭ **AudioToAudioArgs**: [`BaseArgs`](interfaces/BaseArgs) & \{ `inputs`: `Blob`  } \| `LegacyAudioInput`

#### Defined in[[audiotoaudioargs.defined-in]]

[inference/src/tasks/audio/audioToAudio.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/audioToAudio.ts#L8)

___

### AuthMethod

Ƭ **AuthMethod**: ``"none"`` \| ``"hf-token"`` \| ``"credentials-include"`` \| ``"provider-key"``

#### Defined in[[authmethod.defined-in]]

[inference/src/types.ts:161](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L161)

___

### AutomaticSpeechRecognitionArgs

Ƭ **AutomaticSpeechRecognitionArgs**: [`BaseArgs`](interfaces/BaseArgs) & `AutomaticSpeechRecognitionInput` \| `LegacyAudioInput`

#### Defined in[[automaticspeechrecognitionargs.defined-in]]

[inference/src/tasks/audio/automaticSpeechRecognition.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/automaticSpeechRecognition.ts#L8)

___

### DocumentQuestionAnsweringArgs

Ƭ **DocumentQuestionAnsweringArgs**: [`BaseArgs`](interfaces/BaseArgs) & `DocumentQuestionAnsweringInput` & \{ `inputs`: `DocumentQuestionAnsweringInputData` & \{ `image`: `Blob`  }  }

#### Defined in[[documentquestionansweringargs.defined-in]]

[inference/src/tasks/multimodal/documentQuestionAnswering.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/multimodal/documentQuestionAnswering.ts#L13)

___

### FeatureExtractionArgs

Ƭ **FeatureExtractionArgs**: [`BaseArgs`](interfaces/BaseArgs) & `FeatureExtractionInput` & `FeatureExtractionOAICompatInput`

#### Defined in[[featureextractionargs.defined-in]]

[inference/src/tasks/nlp/featureExtraction.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/featureExtraction.ts#L12)

___

### FeatureExtractionOutput

Ƭ **FeatureExtractionOutput**: (`number` \| `number`[] \| `number`[][])[]

Returned values are a multidimensional array of floats (dimension depending on if you sent a string or a list of string, and if the automatic reduction, usually mean_pooling for instance was applied for you or not. This should be explained on the model's README).

#### Defined in[[featureextractionoutput.defined-in]]

[inference/src/tasks/nlp/featureExtraction.ts:17](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/featureExtraction.ts#L17)

___

### FillMaskArgs

Ƭ **FillMaskArgs**: [`BaseArgs`](interfaces/BaseArgs) & `FillMaskInput`

#### Defined in[[fillmaskargs.defined-in]]

[inference/src/tasks/nlp/fillMask.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/fillMask.ts#L7)

___

### ImageClassificationArgs

Ƭ **ImageClassificationArgs**: [`BaseArgs`](interfaces/BaseArgs) & `ImageClassificationInput` \| `LegacyImageInput`

#### Defined in[[imageclassificationargs.defined-in]]

[inference/src/tasks/cv/imageClassification.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageClassification.ts#L8)

___

### ImageSegmentationArgs

Ƭ **ImageSegmentationArgs**: [`BaseArgs`](interfaces/BaseArgs) & `ImageSegmentationInput`

#### Defined in[[imagesegmentationargs.defined-in]]

[inference/src/tasks/cv/imageSegmentation.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageSegmentation.ts#L8)

___

### ImageTextToImageArgs

Ƭ **ImageTextToImageArgs**: [`BaseArgs`](interfaces/BaseArgs) & `ImageTextToImageInput`

#### Defined in[[imagetexttoimageargs.defined-in]]

[inference/src/tasks/cv/imageTextToImage.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageTextToImage.ts#L7)

___

### ImageTextToVideoArgs

Ƭ **ImageTextToVideoArgs**: [`BaseArgs`](interfaces/BaseArgs) & `ImageTextToVideoInput`

#### Defined in[[imagetexttovideoargs.defined-in]]

[inference/src/tasks/cv/imageTextToVideo.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageTextToVideo.ts#L7)

___

### ImageToImageArgs

Ƭ **ImageToImageArgs**: [`BaseArgs`](interfaces/BaseArgs) & `ImageToImageInput`

#### Defined in[[imagetoimageargs.defined-in]]

[inference/src/tasks/cv/imageToImage.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageToImage.ts#L8)

___

### ImageToTextArgs

Ƭ **ImageToTextArgs**: [`BaseArgs`](interfaces/BaseArgs) & `ImageToTextInput` \| `LegacyImageInput`

#### Defined in[[imagetotextargs.defined-in]]

[inference/src/tasks/cv/imageToText.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageToText.ts#L8)

___

### ImageToVideoArgs

Ƭ **ImageToVideoArgs**: [`BaseArgs`](interfaces/BaseArgs) & `ImageToVideoInput`

#### Defined in[[imagetovideoargs.defined-in]]

[inference/src/tasks/cv/imageToVideo.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageToVideo.ts#L8)

___

### InferenceProvider

Ƭ **InferenceProvider**: typeof [`INFERENCE_PROVIDERS`](modules#inference_providers)[`number`]

#### Defined in[[inferenceprovider.defined-in]]

[inference/src/types.ts:71](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L71)

___

### InferenceProviderOrPolicy

Ƭ **InferenceProviderOrPolicy**: typeof [`PROVIDERS_OR_POLICIES`](modules#providers_or_policies)[`number`]

#### Defined in[[inferenceproviderorpolicy.defined-in]]

[inference/src/types.ts:73](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L73)

___

### InferenceTask

Ƭ **InferenceTask**: `Exclude`\<`PipelineType`, ``"other"``\> \| ``"conversational"``

#### Defined in[[inferencetask.defined-in]]

[inference/src/types.ts:45](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L45)

___

### ModelId

Ƭ **ModelId**: `string`

HF model id, like "meta-llama/Llama-3.3-70B-Instruct"

#### Defined in[[modelid.defined-in]]

[inference/src/types.ts:6](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L6)

___

### ObjectDetectionArgs

Ƭ **ObjectDetectionArgs**: [`BaseArgs`](interfaces/BaseArgs) & `ObjectDetectionInput` \| `LegacyImageInput`

#### Defined in[[objectdetectionargs.defined-in]]

[inference/src/tasks/cv/objectDetection.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/objectDetection.ts#L8)

___

### OutputType

Ƭ **OutputType**: ``"url"`` \| ``"dataUrl"`` \| ``"blob"`` \| ``"json"``

#### Defined in[[outputtype.defined-in]]

[inference/src/types.ts:175](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L175)

___

### QuestionAnsweringArgs

Ƭ **QuestionAnsweringArgs**: [`BaseArgs`](interfaces/BaseArgs) & `QuestionAnsweringInput`

#### Defined in[[questionansweringargs.defined-in]]

[inference/src/tasks/nlp/questionAnswering.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/questionAnswering.ts#L8)

___

### RequestArgs

Ƭ **RequestArgs**: [`BaseArgs`](interfaces/BaseArgs) & \{ `data`: `Blob` \| `ArrayBuffer`  } \| \{ `inputs`: `unknown`  } \| \{ `prompt`: `string`  } \| \{ `text`: `string`  } \| \{ `audio_url`: `string`  } \| `ChatCompletionInput` & \{ `parameters?`: `Record`\<`string`, `unknown`\> ; `urlTransform?`: (`url`: `string`) => `string`  }

#### Defined in[[requestargs.defined-in]]

[inference/src/types.ts:148](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L148)

___

### SentenceSimilarityArgs

Ƭ **SentenceSimilarityArgs**: [`BaseArgs`](interfaces/BaseArgs) & `SentenceSimilarityInput`

#### Defined in[[sentencesimilarityargs.defined-in]]

[inference/src/tasks/nlp/sentenceSimilarity.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/sentenceSimilarity.ts#L7)

___

### SummarizationArgs

Ƭ **SummarizationArgs**: [`BaseArgs`](interfaces/BaseArgs) & `SummarizationInput`

#### Defined in[[summarizationargs.defined-in]]

[inference/src/tasks/nlp/summarization.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/summarization.ts#L7)

___

### TableQuestionAnsweringArgs

Ƭ **TableQuestionAnsweringArgs**: [`BaseArgs`](interfaces/BaseArgs) & `TableQuestionAnsweringInput`

#### Defined in[[tablequestionansweringargs.defined-in]]

[inference/src/tasks/nlp/tableQuestionAnswering.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/tableQuestionAnswering.ts#L7)

___

### TabularClassificationArgs

Ƭ **TabularClassificationArgs**: [`BaseArgs`](interfaces/BaseArgs) & \{ `inputs`: \{ `data`: `Record`\<`string`, `string`[]\>  }  }

#### Defined in[[tabularclassificationargs.defined-in]]

[inference/src/tasks/tabular/tabularClassification.ts:6](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/tabular/tabularClassification.ts#L6)

___

### TabularClassificationOutput

Ƭ **TabularClassificationOutput**: `number`[]

A list of predicted labels for each row

#### Defined in[[tabularclassificationoutput.defined-in]]

[inference/src/tasks/tabular/tabularClassification.ts:18](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/tabular/tabularClassification.ts#L18)

___

### TabularRegressionArgs

Ƭ **TabularRegressionArgs**: [`BaseArgs`](interfaces/BaseArgs) & \{ `inputs`: \{ `data`: `Record`\<`string`, `string`[]\>  }  }

#### Defined in[[tabularregressionargs.defined-in]]

[inference/src/tasks/tabular/tabularRegression.ts:6](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/tabular/tabularRegression.ts#L6)

___

### TabularRegressionOutput

Ƭ **TabularRegressionOutput**: `number`[]

a list of predicted values for each row

#### Defined in[[tabularregressionoutput.defined-in]]

[inference/src/tasks/tabular/tabularRegression.ts:18](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/tabular/tabularRegression.ts#L18)

___

### TextClassificationArgs

Ƭ **TextClassificationArgs**: [`BaseArgs`](interfaces/BaseArgs) & `TextClassificationInput`

#### Defined in[[textclassificationargs.defined-in]]

[inference/src/tasks/nlp/textClassification.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textClassification.ts#L7)

___

### TextGenerationStreamFinishReason

Ƭ **TextGenerationStreamFinishReason**: ``"length"`` \| ``"eos_token"`` \| ``"stop_sequence"``

#### Defined in[[textgenerationstreamfinishreason.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:48](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L48)

___

### TextToAudioArgs

Ƭ **TextToAudioArgs**: [`BaseArgs`](interfaces/BaseArgs) & `TextToAudioInput`

#### Defined in[[texttoaudioargs.defined-in]]

[inference/src/tasks/audio/textToAudio.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/textToAudio.ts#L8)

___

### TextToImageArgs

Ƭ **TextToImageArgs**: [`BaseArgs`](interfaces/BaseArgs) & `TextToImageInput`

#### Defined in[[texttoimageargs.defined-in]]

[inference/src/tasks/cv/textToImage.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToImage.ts#L8)

___

### TextToVideoArgs

Ƭ **TextToVideoArgs**: [`BaseArgs`](interfaces/BaseArgs) & `TextToVideoInput`

#### Defined in[[texttovideoargs.defined-in]]

[inference/src/tasks/cv/textToVideo.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToVideo.ts#L11)

___

### TextToVideoOutput

Ƭ **TextToVideoOutput**: `Blob`

#### Defined in[[texttovideooutput.defined-in]]

[inference/src/tasks/cv/textToVideo.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToVideo.ts#L13)

___

### TokenClassificationArgs

Ƭ **TokenClassificationArgs**: [`BaseArgs`](interfaces/BaseArgs) & `TokenClassificationInput`

#### Defined in[[tokenclassificationargs.defined-in]]

[inference/src/tasks/nlp/tokenClassification.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/tokenClassification.ts#L7)

___

### TranslationArgs

Ƭ **TranslationArgs**: [`BaseArgs`](interfaces/BaseArgs) & `TranslationInput`

#### Defined in[[translationargs.defined-in]]

[inference/src/tasks/nlp/translation.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/translation.ts#L7)

___

### VisualQuestionAnsweringArgs

Ƭ **VisualQuestionAnsweringArgs**: [`BaseArgs`](interfaces/BaseArgs) & `VisualQuestionAnsweringInput` & \{ `inputs`: `VisualQuestionAnsweringInputData` & \{ `image`: `Blob`  }  }

#### Defined in[[visualquestionansweringargs.defined-in]]

[inference/src/tasks/multimodal/visualQuestionAnswering.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/multimodal/visualQuestionAnswering.ts#L13)

___

### ZeroShotClassificationArgs

Ƭ **ZeroShotClassificationArgs**: [`BaseArgs`](interfaces/BaseArgs) & `ZeroShotClassificationInput`

#### Defined in[[zeroshotclassificationargs.defined-in]]

[inference/src/tasks/nlp/zeroShotClassification.ts:7](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/zeroShotClassification.ts#L7)

___

### ZeroShotImageClassificationArgs

Ƭ **ZeroShotImageClassificationArgs**: [`BaseArgs`](interfaces/BaseArgs) & `ZeroShotImageClassificationInput` \| `LegacyZeroShotImageClassificationInput`

#### Defined in[[zeroshotimageclassificationargs.defined-in]]

[inference/src/tasks/cv/zeroShotImageClassification.ts:15](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/zeroShotImageClassification.ts#L15)

## Variables

### INFERENCE\_PROVIDERS

• `Const` **INFERENCE\_PROVIDERS**: readonly [``"baseten"``, ``"cerebras"``, ``"cohere"``, ``"deepinfra"``, ``"fal-ai"``, ``"featherless-ai"``, ``"fireworks-ai"``, ``"groq"``, ``"hf-inference"``, ``"novita"``, ``"nscale"``, ``"openai"``, ``"ovhcloud"``, ``"publicai"``, ``"replicate"``, ``"scaleway"``, ``"together"``, ``"wavespeed"``, ``"zai-org"``]

#### Defined in[[inferenceproviders.defined-in]]

[inference/src/types.ts:47](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L47)

___

### PROVIDERS

• `Const` **PROVIDERS**: `Record`\<[`InferenceProvider`](modules#inferenceprovider), `Partial`\<`Record`\<[`InferenceTask`](modules#inferencetask), `TaskProviderHelper`\>\>\>

#### Defined in[[providers.defined-in]]

[inference/src/lib/getProviderHelper.ts:58](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L58)

___

### PROVIDERS\_HUB\_ORGS

• `Const` **PROVIDERS\_HUB\_ORGS**: `Record`\<[`InferenceProvider`](modules#inferenceprovider), `string`\>

The org namespace on the HF Hub i.e. hf.co/…

Whenever possible, InferenceProvider should == org namespace

#### Defined in[[providershuborgs.defined-in]]

[inference/src/types.ts:80](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L80)

___

### PROVIDERS\_OR\_POLICIES

• `Const` **PROVIDERS\_OR\_POLICIES**: readonly [``"baseten"``, ``"cerebras"``, ``"cohere"``, ``"deepinfra"``, ``"fal-ai"``, ``"featherless-ai"``, ``"fireworks-ai"``, ``"groq"``, ``"hf-inference"``, ``"novita"``, ``"nscale"``, ``"openai"``, ``"ovhcloud"``, ``"publicai"``, ``"replicate"``, ``"scaleway"``, ``"together"``, ``"wavespeed"``, ``"zai-org"``, ``"auto"``]

#### Defined in[[providersorpolicies.defined-in]]

[inference/src/types.ts:69](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/types.ts#L69)

## Functions

### audioClassification

▸ **audioClassification**(`args`, `options?`): `Promise`\<`AudioClassificationOutput`\>

This task reads some audio input and outputs the likelihood of classes.
Recommended model:  superb/hubert-large-superb-er

#### Parameters[[audioclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`AudioClassificationArgs`](modules#audioclassificationargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[audioclassification.returns]]

`Promise`\<`AudioClassificationOutput`\>

#### Defined in[[audioclassification.defined-in]]

[inference/src/tasks/audio/audioClassification.ts:15](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/audioClassification.ts#L15)

___

### audioToAudio

▸ **audioToAudio**(`args`, `options?`): `Promise`\<[`AudioToAudioOutput`](interfaces/AudioToAudioOutput)[]\>

This task reads some audio input and outputs one or multiple audio files.
Example model: speechbrain/sepformer-wham does audio source separation.

#### Parameters[[audiotoaudio.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`AudioToAudioArgs`](modules#audiotoaudioargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[audiotoaudio.returns]]

`Promise`\<[`AudioToAudioOutput`](interfaces/AudioToAudioOutput)[]\>

#### Defined in[[audiotoaudio.defined-in]]

[inference/src/tasks/audio/audioToAudio.ts:41](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/audioToAudio.ts#L41)

___

### automaticSpeechRecognition

▸ **automaticSpeechRecognition**(`args`, `options?`): `Promise`\<`AutomaticSpeechRecognitionOutput`\>

This task reads some audio input and outputs the said words within the audio files.
Recommended model (english language): facebook/wav2vec2-large-960h-lv60-self

#### Parameters[[automaticspeechrecognition.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`AutomaticSpeechRecognitionArgs`](modules#automaticspeechrecognitionargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[automaticspeechrecognition.returns]]

`Promise`\<`AutomaticSpeechRecognitionOutput`\>

#### Defined in[[automaticspeechrecognition.defined-in]]

[inference/src/tasks/audio/automaticSpeechRecognition.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/automaticSpeechRecognition.ts#L13)

___

### chatCompletion

▸ **chatCompletion**(`args`, `options?`): `Promise`\<`ChatCompletionOutput`\>

Use the chat completion endpoint to generate a response to a prompt, using OpenAI message completion API no stream

#### Parameters[[chatcompletion.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`BaseArgs`](interfaces/BaseArgs) & `ChatCompletionInput` |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[chatcompletion.returns]]

`Promise`\<`ChatCompletionOutput`\>

#### Defined in[[chatcompletion.defined-in]]

[inference/src/tasks/nlp/chatCompletion.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/chatCompletion.ts#L12)

___

### chatCompletionStream

▸ **chatCompletionStream**(`args`, `options?`): `AsyncGenerator`\<`ChatCompletionStreamOutput`\>

Use to continue text from a prompt. Same as `textGeneration` but returns generator that can be read one token at a time

#### Parameters[[chatcompletionstream.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`BaseArgs`](interfaces/BaseArgs) & `ChatCompletionInput` |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[chatcompletionstream.returns]]

`AsyncGenerator`\<`ChatCompletionStreamOutput`\>

#### Defined in[[chatcompletionstream.defined-in]]

[inference/src/tasks/nlp/chatCompletionStream.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/chatCompletionStream.ts#L12)

___

### documentQuestionAnswering

▸ **documentQuestionAnswering**(`args`, `options?`): `Promise`\<`DocumentQuestionAnsweringOutput`[`number`]\>

Answers a question on a document image. Recommended model: impira/layoutlm-document-qa.

#### Parameters[[documentquestionanswering.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`DocumentQuestionAnsweringArgs`](modules#documentquestionansweringargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[documentquestionanswering.returns]]

`Promise`\<`DocumentQuestionAnsweringOutput`[`number`]\>

#### Defined in[[documentquestionanswering.defined-in]]

[inference/src/tasks/multimodal/documentQuestionAnswering.ts:19](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/multimodal/documentQuestionAnswering.ts#L19)

___

### featureExtraction

▸ **featureExtraction**(`args`, `options?`): `Promise`\<[`FeatureExtractionOutput`](modules#featureextractionoutput)\>

This task reads some text and outputs raw float values, that are usually consumed as part of a semantic database/semantic search.

#### Parameters[[featureextraction.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`FeatureExtractionArgs`](modules#featureextractionargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[featureextraction.returns]]

`Promise`\<[`FeatureExtractionOutput`](modules#featureextractionoutput)\>

#### Defined in[[featureextraction.defined-in]]

[inference/src/tasks/nlp/featureExtraction.ts:22](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/featureExtraction.ts#L22)

___

### fillMask

▸ **fillMask**(`args`, `options?`): `Promise`\<`FillMaskOutput`\>

Tries to fill in a hole with a missing word (token to be precise). That’s the base task for BERT models.

#### Parameters[[fillmask.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`FillMaskArgs`](modules#fillmaskargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[fillmask.returns]]

`Promise`\<`FillMaskOutput`\>

#### Defined in[[fillmask.defined-in]]

[inference/src/tasks/nlp/fillMask.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/fillMask.ts#L12)

___

### getProviderHelper

▸ **getProviderHelper**(`provider`, `task`): `TextToImageTaskHelper` & `TaskProviderHelper`

Get provider helper instance by name and task

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"text-to-image"`` |

#### Returns[[getproviderhelper.returns]]

`TextToImageTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:187](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L187)

▸ **getProviderHelper**(`provider`, `task`): `ConversationalTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"conversational"`` |

#### Returns[[getproviderhelper.returns]]

`ConversationalTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:191](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L191)

▸ **getProviderHelper**(`provider`, `task`): `TextGenerationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"text-generation"`` |

#### Returns[[getproviderhelper.returns]]

`TextGenerationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:195](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L195)

▸ **getProviderHelper**(`provider`, `task`): `TextToSpeechTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"text-to-speech"`` |

#### Returns[[getproviderhelper.returns]]

`TextToSpeechTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:199](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L199)

▸ **getProviderHelper**(`provider`, `task`): `TextToAudioTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"text-to-audio"`` |

#### Returns[[getproviderhelper.returns]]

`TextToAudioTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:203](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L203)

▸ **getProviderHelper**(`provider`, `task`): `AutomaticSpeechRecognitionTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"automatic-speech-recognition"`` |

#### Returns[[getproviderhelper.returns]]

`AutomaticSpeechRecognitionTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:207](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L207)

▸ **getProviderHelper**(`provider`, `task`): `TextToVideoTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"text-to-video"`` |

#### Returns[[getproviderhelper.returns]]

`TextToVideoTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:211](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L211)

▸ **getProviderHelper**(`provider`, `task`): `TextClassificationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"text-classification"`` |

#### Returns[[getproviderhelper.returns]]

`TextClassificationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:215](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L215)

▸ **getProviderHelper**(`provider`, `task`): `QuestionAnsweringTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"question-answering"`` |

#### Returns[[getproviderhelper.returns]]

`QuestionAnsweringTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:219](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L219)

▸ **getProviderHelper**(`provider`, `task`): `AudioClassificationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"audio-classification"`` |

#### Returns[[getproviderhelper.returns]]

`AudioClassificationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:223](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L223)

▸ **getProviderHelper**(`provider`, `task`): `AudioToAudioTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"audio-to-audio"`` |

#### Returns[[getproviderhelper.returns]]

`AudioToAudioTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:227](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L227)

▸ **getProviderHelper**(`provider`, `task`): `FillMaskTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"fill-mask"`` |

#### Returns[[getproviderhelper.returns]]

`FillMaskTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:231](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L231)

▸ **getProviderHelper**(`provider`, `task`): `FeatureExtractionTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"feature-extraction"`` |

#### Returns[[getproviderhelper.returns]]

`FeatureExtractionTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:235](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L235)

▸ **getProviderHelper**(`provider`, `task`): `ImageClassificationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"image-classification"`` |

#### Returns[[getproviderhelper.returns]]

`ImageClassificationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:239](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L239)

▸ **getProviderHelper**(`provider`, `task`): `ImageSegmentationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"image-segmentation"`` |

#### Returns[[getproviderhelper.returns]]

`ImageSegmentationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:243](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L243)

▸ **getProviderHelper**(`provider`, `task`): `DocumentQuestionAnsweringTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"document-question-answering"`` |

#### Returns[[getproviderhelper.returns]]

`DocumentQuestionAnsweringTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:247](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L247)

▸ **getProviderHelper**(`provider`, `task`): `ImageToTextTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"image-to-text"`` |

#### Returns[[getproviderhelper.returns]]

`ImageToTextTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:251](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L251)

▸ **getProviderHelper**(`provider`, `task`): `ObjectDetectionTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"object-detection"`` |

#### Returns[[getproviderhelper.returns]]

`ObjectDetectionTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:255](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L255)

▸ **getProviderHelper**(`provider`, `task`): `ZeroShotImageClassificationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"zero-shot-image-classification"`` |

#### Returns[[getproviderhelper.returns]]

`ZeroShotImageClassificationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:259](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L259)

▸ **getProviderHelper**(`provider`, `task`): `ZeroShotClassificationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"zero-shot-classification"`` |

#### Returns[[getproviderhelper.returns]]

`ZeroShotClassificationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:263](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L263)

▸ **getProviderHelper**(`provider`, `task`): `ImageToImageTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"image-to-image"`` |

#### Returns[[getproviderhelper.returns]]

`ImageToImageTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:267](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L267)

▸ **getProviderHelper**(`provider`, `task`): `ImageToVideoTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"image-to-video"`` |

#### Returns[[getproviderhelper.returns]]

`ImageToVideoTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:271](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L271)

▸ **getProviderHelper**(`provider`, `task`): `ImageTextToImageTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"image-text-to-image"`` |

#### Returns[[getproviderhelper.returns]]

`ImageTextToImageTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:275](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L275)

▸ **getProviderHelper**(`provider`, `task`): `ImageTextToVideoTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"image-text-to-video"`` |

#### Returns[[getproviderhelper.returns]]

`ImageTextToVideoTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:279](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L279)

▸ **getProviderHelper**(`provider`, `task`): `SentenceSimilarityTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"sentence-similarity"`` |

#### Returns[[getproviderhelper.returns]]

`SentenceSimilarityTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:283](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L283)

▸ **getProviderHelper**(`provider`, `task`): `TableQuestionAnsweringTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"table-question-answering"`` |

#### Returns[[getproviderhelper.returns]]

`TableQuestionAnsweringTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:287](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L287)

▸ **getProviderHelper**(`provider`, `task`): `TabularClassificationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"tabular-classification"`` |

#### Returns[[getproviderhelper.returns]]

`TabularClassificationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:291](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L291)

▸ **getProviderHelper**(`provider`, `task`): `TabularRegressionTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"tabular-regression"`` |

#### Returns[[getproviderhelper.returns]]

`TabularRegressionTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:295](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L295)

▸ **getProviderHelper**(`provider`, `task`): `TokenClassificationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"token-classification"`` |

#### Returns[[getproviderhelper.returns]]

`TokenClassificationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:299](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L299)

▸ **getProviderHelper**(`provider`, `task`): `TranslationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"translation"`` |

#### Returns[[getproviderhelper.returns]]

`TranslationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:303](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L303)

▸ **getProviderHelper**(`provider`, `task`): `SummarizationTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"summarization"`` |

#### Returns[[getproviderhelper.returns]]

`SummarizationTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:307](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L307)

▸ **getProviderHelper**(`provider`, `task`): `VisualQuestionAnsweringTaskHelper` & `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | ``"visual-question-answering"`` |

#### Returns[[getproviderhelper.returns]]

`VisualQuestionAnsweringTaskHelper` & `TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:311](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L311)

▸ **getProviderHelper**(`provider`, `task`): `TaskProviderHelper`

#### Parameters[[getproviderhelper.parameters]]

| Name | Type |
| :------ | :------ |
| `provider` | ``"baseten"`` \| ``"cerebras"`` \| ``"cohere"`` \| ``"deepinfra"`` \| ``"fal-ai"`` \| ``"featherless-ai"`` \| ``"fireworks-ai"`` \| ``"groq"`` \| ``"hf-inference"`` \| ``"novita"`` \| ``"nscale"`` \| ``"openai"`` \| ``"ovhcloud"`` \| ``"publicai"`` \| ``"replicate"`` \| ``"scaleway"`` \| ``"together"`` \| ``"wavespeed"`` \| ``"zai-org"`` \| ``"auto"`` |
| `task` | `undefined` \| [`InferenceTask`](modules#inferencetask) |

#### Returns[[getproviderhelper.returns]]

`TaskProviderHelper`

#### Defined in[[getproviderhelper.defined-in]]

[inference/src/lib/getProviderHelper.ts:315](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/getProviderHelper.ts#L315)

___

### imageClassification

▸ **imageClassification**(`args`, `options?`): `Promise`\<`ImageClassificationOutput`\>

This task reads some image input and outputs the likelihood of classes.
Recommended model: google/vit-base-patch16-224

#### Parameters[[imageclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ImageClassificationArgs`](modules#imageclassificationargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[imageclassification.returns]]

`Promise`\<`ImageClassificationOutput`\>

#### Defined in[[imageclassification.defined-in]]

[inference/src/tasks/cv/imageClassification.ts:14](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageClassification.ts#L14)

___

### imageSegmentation

▸ **imageSegmentation**(`args`, `options?`): `Promise`\<`ImageSegmentationOutput`\>

This task reads some image input and outputs the likelihood of classes & bounding boxes of detected objects.
Recommended model: facebook/detr-resnet-50-panoptic

#### Parameters[[imagesegmentation.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ImageSegmentationArgs`](modules#imagesegmentationargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[imagesegmentation.returns]]

`Promise`\<`ImageSegmentationOutput`\>

#### Defined in[[imagesegmentation.defined-in]]

[inference/src/tasks/cv/imageSegmentation.ts:14](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageSegmentation.ts#L14)

___

### imageTextToImage

▸ **imageTextToImage**(`args`, `options?`): `Promise`\<`Blob`\>

This task takes an image and text input and outputs a new generated image.
Recommended model: black-forest-labs/FLUX.2-dev

#### Parameters[[imagetexttoimage.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ImageTextToImageArgs`](modules#imagetexttoimageargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[imagetexttoimage.returns]]

`Promise`\<`Blob`\>

#### Defined in[[imagetexttoimage.defined-in]]

[inference/src/tasks/cv/imageTextToImage.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageTextToImage.ts#L13)

___

### imageTextToVideo

▸ **imageTextToVideo**(`args`, `options?`): `Promise`\<`Blob`\>

This task takes an image and text input and outputs a generated video.
Recommended model: Lightricks/LTX-Video

#### Parameters[[imagetexttovideo.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ImageTextToVideoArgs`](modules#imagetexttovideoargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[imagetexttovideo.returns]]

`Promise`\<`Blob`\>

#### Defined in[[imagetexttovideo.defined-in]]

[inference/src/tasks/cv/imageTextToVideo.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageTextToVideo.ts#L13)

___

### imageToImage

▸ **imageToImage**(`args`, `options?`): `Promise`\<`Blob`\>

This task reads some text input and outputs an image.
Recommended model: lllyasviel/sd-controlnet-depth

#### Parameters[[imagetoimage.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ImageToImageArgs`](modules#imagetoimageargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[imagetoimage.returns]]

`Promise`\<`Blob`\>

#### Defined in[[imagetoimage.defined-in]]

[inference/src/tasks/cv/imageToImage.ts:14](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageToImage.ts#L14)

___

### imageToText

▸ **imageToText**(`args`, `options?`): `Promise`\<`ImageToTextOutput`\>

This task reads some image input and outputs the text caption.

#### Parameters[[imagetotext.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ImageToTextArgs`](modules#imagetotextargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[imagetotext.returns]]

`Promise`\<`ImageToTextOutput`\>

#### Defined in[[imagetotext.defined-in]]

[inference/src/tasks/cv/imageToText.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageToText.ts#L12)

___

### imageToVideo

▸ **imageToVideo**(`args`, `options?`): `Promise`\<`Blob`\>

This task reads some text input and outputs an image.
Recommended model: Wan-AI/Wan2.1-I2V-14B-720P

#### Parameters[[imagetovideo.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ImageToVideoArgs`](modules#imagetovideoargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[imagetovideo.returns]]

`Promise`\<`Blob`\>

#### Defined in[[imagetovideo.defined-in]]

[inference/src/tasks/cv/imageToVideo.ts:14](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageToVideo.ts#L14)

___

### makeRequestOptions

▸ **makeRequestOptions**(`args`, `providerHelper`, `options?`): `Promise`\<\{ `info`: `RequestInit` ; `url`: `string`  }\>

Helper that prepares request arguments.
This async version handle the model ID resolution step.

#### Parameters[[makerequestoptions.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`RequestArgs`](modules#requestargs) & \{ `data?`: `Blob` \| `ArrayBuffer` ; `stream?`: `boolean`  } |
| `providerHelper` | `TaskProviderHelper` |
| `options?` | [`Options`](interfaces/Options) & \{ `task?`: [`InferenceTask`](modules#inferencetask)  } |

#### Returns[[makerequestoptions.returns]]

`Promise`\<\{ `info`: `RequestInit` ; `url`: `string`  }\>

#### Defined in[[makerequestoptions.defined-in]]

[inference/src/lib/makeRequestOptions.ts:19](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/makeRequestOptions.ts#L19)

___

### makeRequestOptionsFromResolvedModel

▸ **makeRequestOptionsFromResolvedModel**(`resolvedModel`, `providerHelper`, `args`, `mapping`, `options?`): `Object`

Helper that prepares request arguments. - for internal use only
This sync version skips the model ID resolution step

#### Parameters[[makerequestoptionsfromresolvedmodel.parameters]]

| Name | Type |
| :------ | :------ |
| `resolvedModel` | `string` |
| `providerHelper` | `TaskProviderHelper` |
| `args` | [`RequestArgs`](modules#requestargs) & \{ `data?`: `Blob` \| `ArrayBuffer` ; `stream?`: `boolean`  } |
| `mapping` | `undefined` \| [`InferenceProviderMappingEntry`](interfaces/InferenceProviderMappingEntry) |
| `options?` | [`Options`](interfaces/Options) & \{ `outputType?`: [`OutputType`](modules#outputtype) ; `task?`: [`InferenceTask`](modules#inferencetask)  } |

#### Returns[[makerequestoptionsfromresolvedmodel.returns]]

`Object`

| Name | Type |
| :------ | :------ |
| `info` | `RequestInit` |
| `url` | `string` |

#### Defined in[[makerequestoptionsfromresolvedmodel.defined-in]]

[inference/src/lib/makeRequestOptions.ts:105](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/makeRequestOptions.ts#L105)

___

### objectDetection

▸ **objectDetection**(`args`, `options?`): `Promise`\<`ObjectDetectionOutput`\>

This task reads some image input and outputs the likelihood of classes & bounding boxes of detected objects.
Recommended model: facebook/detr-resnet-50

#### Parameters[[objectdetection.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ObjectDetectionArgs`](modules#objectdetectionargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[objectdetection.returns]]

`Promise`\<`ObjectDetectionOutput`\>

#### Defined in[[objectdetection.defined-in]]

[inference/src/tasks/cv/objectDetection.ts:14](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/objectDetection.ts#L14)

___

### questionAnswering

▸ **questionAnswering**(`args`, `options?`): `Promise`\<`QuestionAnsweringOutput`[`number`]\>

Want to have a nice know-it-all bot that can answer any question?. Recommended model: deepset/roberta-base-squad2

#### Parameters[[questionanswering.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`QuestionAnsweringArgs`](modules#questionansweringargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[questionanswering.returns]]

`Promise`\<`QuestionAnsweringOutput`[`number`]\>

#### Defined in[[questionanswering.defined-in]]

[inference/src/tasks/nlp/questionAnswering.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/questionAnswering.ts#L13)

___

### request

▸ **request**\<`T`\>(`args`, `options?`): `Promise`\<`T`\>

Primitive to make custom calls to the inference provider

#### Type parameters[[request.type-parameters]]

| Name |
| :------ |
| `T` |

#### Parameters[[request.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`RequestArgs`](modules#requestargs) |
| `options?` | [`Options`](interfaces/Options) & \{ `task?`: [`InferenceTask`](modules#inferencetask)  } |

#### Returns[[request.returns]]

`Promise`\<`T`\>

**`Deprecated`**

Use specific task functions instead. This function will be removed in a future version.

#### Defined in[[request.defined-in]]

[inference/src/tasks/custom/request.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/custom/request.ts#L11)

___

### sentenceSimilarity

▸ **sentenceSimilarity**(`args`, `options?`): `Promise`\<`SentenceSimilarityOutput`\>

Calculate the semantic similarity between one text and a list of other sentences by comparing their embeddings.

#### Parameters[[sentencesimilarity.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`SentenceSimilarityArgs`](modules#sentencesimilarityargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[sentencesimilarity.returns]]

`Promise`\<`SentenceSimilarityOutput`\>

#### Defined in[[sentencesimilarity.defined-in]]

[inference/src/tasks/nlp/sentenceSimilarity.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/sentenceSimilarity.ts#L12)

___

### setLogger

▸ **setLogger**(`logger`): `void`

#### Parameters[[setlogger.parameters]]

| Name | Type |
| :------ | :------ |
| `logger` | [`Logger`](interfaces/Logger) |

#### Returns[[setlogger.returns]]

`void`

#### Defined in[[setlogger.defined-in]]

[inference/src/lib/logger.ts:5](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/lib/logger.ts#L5)

___

### streamingRequest

▸ **streamingRequest**\<`T`\>(`args`, `options?`): `AsyncGenerator`\<`T`\>

Primitive to make custom inference calls that expect server-sent events, and returns the response through a generator

#### Type parameters[[streamingrequest.type-parameters]]

| Name |
| :------ |
| `T` |

#### Parameters[[streamingrequest.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`RequestArgs`](modules#requestargs) |
| `options?` | [`Options`](interfaces/Options) & \{ `task?`: [`InferenceTask`](modules#inferencetask)  } |

#### Returns[[streamingrequest.returns]]

`AsyncGenerator`\<`T`\>

**`Deprecated`**

Use specific task functions instead. This function will be removed in a future version.

#### Defined in[[streamingrequest.defined-in]]

[inference/src/tasks/custom/streamingRequest.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/custom/streamingRequest.ts#L11)

___

### summarization

▸ **summarization**(`args`, `options?`): `Promise`\<`SummarizationOutput`\>

This task is well known to summarize longer text into shorter text. Be careful, some models have a maximum length of input. That means that the summary cannot handle full books for instance. Be careful when choosing your model.

#### Parameters[[summarization.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`SummarizationArgs`](modules#summarizationargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[summarization.returns]]

`Promise`\<`SummarizationOutput`\>

#### Defined in[[summarization.defined-in]]

[inference/src/tasks/nlp/summarization.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/summarization.ts#L12)

___

### tableQuestionAnswering

▸ **tableQuestionAnswering**(`args`, `options?`): `Promise`\<`TableQuestionAnsweringOutput`[`number`]\>

Don’t know SQL? Don’t want to dive into a large spreadsheet? Ask questions in plain english! Recommended model: google/tapas-base-finetuned-wtq.

#### Parameters[[tablequestionanswering.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TableQuestionAnsweringArgs`](modules#tablequestionansweringargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[tablequestionanswering.returns]]

`Promise`\<`TableQuestionAnsweringOutput`[`number`]\>

#### Defined in[[tablequestionanswering.defined-in]]

[inference/src/tasks/nlp/tableQuestionAnswering.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/tableQuestionAnswering.ts#L12)

___

### tabularClassification

▸ **tabularClassification**(`args`, `options?`): `Promise`\<[`TabularClassificationOutput`](modules#tabularclassificationoutput)\>

Predicts target label for a given set of features in tabular form.
Typically, you will want to train a classification model on your training data and use it with your new data of the same format.
Example model: vvmnnnkv/wine-quality

#### Parameters[[tabularclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TabularClassificationArgs`](modules#tabularclassificationargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[tabularclassification.returns]]

`Promise`\<[`TabularClassificationOutput`](modules#tabularclassificationoutput)\>

#### Defined in[[tabularclassification.defined-in]]

[inference/src/tasks/tabular/tabularClassification.ts:25](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/tabular/tabularClassification.ts#L25)

___

### tabularRegression

▸ **tabularRegression**(`args`, `options?`): `Promise`\<[`TabularRegressionOutput`](modules#tabularregressionoutput)\>

Predicts target value for a given set of features in tabular form.
Typically, you will want to train a regression model on your training data and use it with your new data of the same format.
Example model: scikit-learn/Fish-Weight

#### Parameters[[tabularregression.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TabularRegressionArgs`](modules#tabularregressionargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[tabularregression.returns]]

`Promise`\<[`TabularRegressionOutput`](modules#tabularregressionoutput)\>

#### Defined in[[tabularregression.defined-in]]

[inference/src/tasks/tabular/tabularRegression.ts:25](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/tabular/tabularRegression.ts#L25)

___

### textClassification

▸ **textClassification**(`args`, `options?`): `Promise`\<`TextClassificationOutput`\>

Usually used for sentiment-analysis this will output the likelihood of classes of an input. Recommended model: distilbert-base-uncased-finetuned-sst-2-english

#### Parameters[[textclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextClassificationArgs`](modules#textclassificationargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[textclassification.returns]]

`Promise`\<`TextClassificationOutput`\>

#### Defined in[[textclassification.defined-in]]

[inference/src/tasks/nlp/textClassification.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textClassification.ts#L12)

___

### textGeneration

▸ **textGeneration**(`args`, `options?`): `Promise`\<[`TextGenerationOutput`](interfaces/TextGenerationOutput)\>

Use to continue text from a prompt. This is a very generic task. Recommended model: gpt2 (it’s a simple model, but fun to play with).

#### Parameters[[textgeneration.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`BaseArgs`](interfaces/BaseArgs) & [`TextGenerationInput`](interfaces/TextGenerationInput) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[textgeneration.returns]]

`Promise`\<[`TextGenerationOutput`](interfaces/TextGenerationOutput)\>

#### Defined in[[textgeneration.defined-in]]

[inference/src/tasks/nlp/textGeneration.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGeneration.ts#L12)

___

### textGenerationStream

▸ **textGenerationStream**(`args`, `options?`): `AsyncGenerator`\<[`TextGenerationStreamOutput`](interfaces/TextGenerationStreamOutput)\>

Use to continue text from a prompt. Same as `textGeneration` but returns generator that can be read one token at a time

#### Parameters[[textgenerationstream.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`BaseArgs`](interfaces/BaseArgs) & [`TextGenerationInput`](interfaces/TextGenerationInput) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[textgenerationstream.returns]]

`AsyncGenerator`\<[`TextGenerationStreamOutput`](interfaces/TextGenerationStreamOutput)\>

#### Defined in[[textgenerationstream.defined-in]]

[inference/src/tasks/nlp/textGenerationStream.ts:90](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGenerationStream.ts#L90)

___

### textToAudio

▸ **textToAudio**(`args`, `options?`): `Promise`\<`Blob`\>

This task generates audio (e.g. music or sound effects) from an input text prompt.
Example model: stabilityai/stable-audio-open-1.0

#### Parameters[[texttoaudio.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextToAudioArgs`](modules#texttoaudioargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[texttoaudio.returns]]

`Promise`\<`Blob`\>

#### Defined in[[texttoaudio.defined-in]]

[inference/src/tasks/audio/textToAudio.ts:18](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/textToAudio.ts#L18)

___

### textToImage

▸ **textToImage**(`args`, `options?`): `Promise`\<`string`\>

This task reads some text input and outputs an image.
Recommended model: stabilityai/stable-diffusion-2

#### Parameters[[texttoimage.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextToImageArgs`](modules#texttoimageargs) |
| `options?` | `TextToImageOptions` & \{ `outputType`: ``"url"``  } |

#### Returns[[texttoimage.returns]]

`Promise`\<`string`\>

#### Defined in[[texttoimage.defined-in]]

[inference/src/tasks/cv/textToImage.ts:18](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToImage.ts#L18)

▸ **textToImage**(`args`, `options?`): `Promise`\<`string`\>

#### Parameters[[texttoimage.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextToImageArgs`](modules#texttoimageargs) |
| `options?` | `TextToImageOptions` & \{ `outputType`: ``"dataUrl"``  } |

#### Returns[[texttoimage.returns]]

`Promise`\<`string`\>

#### Defined in[[texttoimage.defined-in]]

[inference/src/tasks/cv/textToImage.ts:22](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToImage.ts#L22)

▸ **textToImage**(`args`, `options?`): `Promise`\<`Blob`\>

#### Parameters[[texttoimage.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextToImageArgs`](modules#texttoimageargs) |
| `options?` | `TextToImageOptions` & \{ `outputType?`: ``"blob"``  } |

#### Returns[[texttoimage.returns]]

`Promise`\<`Blob`\>

#### Defined in[[texttoimage.defined-in]]

[inference/src/tasks/cv/textToImage.ts:26](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToImage.ts#L26)

▸ **textToImage**(`args`, `options?`): `Promise`\<`Record`\<`string`, `unknown`\>\>

#### Parameters[[texttoimage.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextToImageArgs`](modules#texttoimageargs) |
| `options?` | `TextToImageOptions` & \{ `outputType?`: ``"json"``  } |

#### Returns[[texttoimage.returns]]

`Promise`\<`Record`\<`string`, `unknown`\>\>

#### Defined in[[texttoimage.defined-in]]

[inference/src/tasks/cv/textToImage.ts:30](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToImage.ts#L30)

___

### textToSpeech

▸ **textToSpeech**(`args`, `options?`): `Promise`\<`Blob`\>

This task synthesize an audio of a voice pronouncing a given text.
Recommended model: espnet/kan-bayashi_ljspeech_vits

#### Parameters[[texttospeech.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | `TextToSpeechArgs` |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[texttospeech.returns]]

`Promise`\<`Blob`\>

#### Defined in[[texttospeech.defined-in]]

[inference/src/tasks/audio/textToSpeech.ts:15](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/textToSpeech.ts#L15)

___

### textToVideo

▸ **textToVideo**(`args`, `options?`): `Promise`\<[`TextToVideoOutput`](modules#texttovideooutput)\>

#### Parameters[[texttovideo.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextToVideoArgs`](modules#texttovideoargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[texttovideo.returns]]

`Promise`\<[`TextToVideoOutput`](modules#texttovideooutput)\>

#### Defined in[[texttovideo.defined-in]]

[inference/src/tasks/cv/textToVideo.ts:15](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToVideo.ts#L15)

___

### tokenClassification

▸ **tokenClassification**(`args`, `options?`): `Promise`\<`TokenClassificationOutput`\>

Usually used for sentence parsing, either grammatical, or Named Entity Recognition (NER) to understand keywords contained within text. Recommended model: dbmdz/bert-large-cased-finetuned-conll03-english

#### Parameters[[tokenclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TokenClassificationArgs`](modules#tokenclassificationargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[tokenclassification.returns]]

`Promise`\<`TokenClassificationOutput`\>

#### Defined in[[tokenclassification.defined-in]]

[inference/src/tasks/nlp/tokenClassification.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/tokenClassification.ts#L12)

___

### translation

▸ **translation**(`args`, `options?`): `Promise`\<`TranslationOutput`\>

This task is well known to translate text from one language to another. Recommended model: Helsinki-NLP/opus-mt-ru-en.

#### Parameters[[translation.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TranslationArgs`](modules#translationargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[translation.returns]]

`Promise`\<`TranslationOutput`\>

#### Defined in[[translation.defined-in]]

[inference/src/tasks/nlp/translation.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/translation.ts#L11)

___

### visualQuestionAnswering

▸ **visualQuestionAnswering**(`args`, `options?`): `Promise`\<`VisualQuestionAnsweringOutput`[`number`]\>

Answers a question on an image. Recommended model: dandelin/vilt-b32-finetuned-vqa.

#### Parameters[[visualquestionanswering.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`VisualQuestionAnsweringArgs`](modules#visualquestionansweringargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[visualquestionanswering.returns]]

`Promise`\<`VisualQuestionAnsweringOutput`[`number`]\>

#### Defined in[[visualquestionanswering.defined-in]]

[inference/src/tasks/multimodal/visualQuestionAnswering.ts:19](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/multimodal/visualQuestionAnswering.ts#L19)

___

### zeroShotClassification

▸ **zeroShotClassification**(`args`, `options?`): `Promise`\<`ZeroShotClassificationOutput`\>

This task is super useful to try out classification with zero code, you simply pass a sentence/paragraph and the possible labels for that sentence, and you get a result. Recommended model: facebook/bart-large-mnli.

#### Parameters[[zeroshotclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ZeroShotClassificationArgs`](modules#zeroshotclassificationargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[zeroshotclassification.returns]]

`Promise`\<`ZeroShotClassificationOutput`\>

#### Defined in[[zeroshotclassification.defined-in]]

[inference/src/tasks/nlp/zeroShotClassification.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/zeroShotClassification.ts#L12)

___

### zeroShotImageClassification

▸ **zeroShotImageClassification**(`args`, `options?`): `Promise`\<`ZeroShotImageClassificationOutput`\>

Classify an image to specified classes.
Recommended model: openai/clip-vit-large-patch14-336

#### Parameters[[zeroshotimageclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ZeroShotImageClassificationArgs`](modules#zeroshotimageclassificationargs) |
| `options?` | [`Options`](interfaces/Options) |

#### Returns[[zeroshotimageclassification.returns]]

`Promise`\<`ZeroShotImageClassificationOutput`\>

#### Defined in[[zeroshotimageclassification.defined-in]]

[inference/src/tasks/cv/zeroShotImageClassification.ts:44](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/zeroShotImageClassification.ts#L44)
