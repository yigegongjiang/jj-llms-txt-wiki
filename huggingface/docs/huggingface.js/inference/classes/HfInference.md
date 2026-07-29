# Class: HfInference

For backward compatibility only, will remove soon.

**`Deprecated`**

replace with InferenceClient

## Hierarchy

- [`InferenceClient`](InferenceClient)

  ↳ **`HfInference`**

## Constructors

### constructor

• **new HfInference**(`accessToken?`, `defaultOptions?`): [`HfInference`](HfInference)

#### Parameters[[constructor.parameters]]

| Name | Type | Default value |
| :------ | :------ | :------ |
| `accessToken` | `string` | `""` |
| `defaultOptions` | [`Options`](../interfaces/Options) & \{ `endpointUrl?`: `string`  } | `{}` |

#### Returns[[constructor.returns]]

[`HfInference`](HfInference)

#### Inherited from[[constructor.inherited-from]]

[InferenceClient](InferenceClient).[constructor](InferenceClient#constructor)

#### Defined in[[constructor.defined-in]]

[inference/src/InferenceClient.ts:15](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/InferenceClient.ts#L15)

## Methods

### audioClassification

▸ **audioClassification**(`args`, `options?`): `Promise`\<`AudioClassificationOutput`\>

This task reads some audio input and outputs the likelihood of classes.
Recommended model:  superb/hubert-large-superb-er

#### Parameters[[audioclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`AudioClassificationArgs`](../modules#audioclassificationargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[audioclassification.returns]]

`Promise`\<`AudioClassificationOutput`\>

#### Inherited from[[audioclassification.inherited-from]]

[InferenceClient](InferenceClient).[audioClassification](InferenceClient#audioclassification)

#### Defined in[[audioclassification.defined-in]]

[inference/src/tasks/audio/audioClassification.ts:15](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/audioClassification.ts#L15)

___

### audioToAudio

▸ **audioToAudio**(`args`, `options?`): `Promise`\<[`AudioToAudioOutput`](../interfaces/AudioToAudioOutput)[]\>

This task reads some audio input and outputs one or multiple audio files.
Example model: speechbrain/sepformer-wham does audio source separation.

#### Parameters[[audiotoaudio.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`AudioToAudioArgs`](../modules#audiotoaudioargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[audiotoaudio.returns]]

`Promise`\<[`AudioToAudioOutput`](../interfaces/AudioToAudioOutput)[]\>

#### Inherited from[[audiotoaudio.inherited-from]]

[InferenceClient](InferenceClient).[audioToAudio](InferenceClient#audiotoaudio)

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
| `args` | [`AutomaticSpeechRecognitionArgs`](../modules#automaticspeechrecognitionargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[automaticspeechrecognition.returns]]

`Promise`\<`AutomaticSpeechRecognitionOutput`\>

#### Inherited from[[automaticspeechrecognition.inherited-from]]

[InferenceClient](InferenceClient).[automaticSpeechRecognition](InferenceClient#automaticspeechrecognition)

#### Defined in[[automaticspeechrecognition.defined-in]]

[inference/src/tasks/audio/automaticSpeechRecognition.ts:13](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/automaticSpeechRecognition.ts#L13)

___

### chatCompletion

▸ **chatCompletion**(`args`, `options?`): `Promise`\<`ChatCompletionOutput`\>

Use the chat completion endpoint to generate a response to a prompt, using OpenAI message completion API no stream

#### Parameters[[chatcompletion.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`BaseArgs`](../interfaces/BaseArgs) & `ChatCompletionInput` |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[chatcompletion.returns]]

`Promise`\<`ChatCompletionOutput`\>

#### Inherited from[[chatcompletion.inherited-from]]

[InferenceClient](InferenceClient).[chatCompletion](InferenceClient#chatcompletion)

#### Defined in[[chatcompletion.defined-in]]

[inference/src/tasks/nlp/chatCompletion.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/chatCompletion.ts#L12)

___

### chatCompletionStream

▸ **chatCompletionStream**(`args`, `options?`): `AsyncGenerator`\<`ChatCompletionStreamOutput`\>

Use to continue text from a prompt. Same as `textGeneration` but returns generator that can be read one token at a time

#### Parameters[[chatcompletionstream.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`BaseArgs`](../interfaces/BaseArgs) & `ChatCompletionInput` |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[chatcompletionstream.returns]]

`AsyncGenerator`\<`ChatCompletionStreamOutput`\>

#### Inherited from[[chatcompletionstream.inherited-from]]

[InferenceClient](InferenceClient).[chatCompletionStream](InferenceClient#chatcompletionstream)

#### Defined in[[chatcompletionstream.defined-in]]

[inference/src/tasks/nlp/chatCompletionStream.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/chatCompletionStream.ts#L12)

___

### documentQuestionAnswering

▸ **documentQuestionAnswering**(`args`, `options?`): `Promise`\<`DocumentQuestionAnsweringOutput`[`number`]\>

Answers a question on a document image. Recommended model: impira/layoutlm-document-qa.

#### Parameters[[documentquestionanswering.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`DocumentQuestionAnsweringArgs`](../modules#documentquestionansweringargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[documentquestionanswering.returns]]

`Promise`\<`DocumentQuestionAnsweringOutput`[`number`]\>

#### Inherited from[[documentquestionanswering.inherited-from]]

[InferenceClient](InferenceClient).[documentQuestionAnswering](InferenceClient#documentquestionanswering)

#### Defined in[[documentquestionanswering.defined-in]]

[inference/src/tasks/multimodal/documentQuestionAnswering.ts:19](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/multimodal/documentQuestionAnswering.ts#L19)

___

### endpoint

▸ **endpoint**(`endpointUrl`): [`InferenceClient`](InferenceClient)

Returns a new instance of InferenceClient tied to a specified endpoint.

For backward compatibility mostly.

#### Parameters[[endpoint.parameters]]

| Name | Type |
| :------ | :------ |
| `endpointUrl` | `string` |

#### Returns[[endpoint.returns]]

[`InferenceClient`](InferenceClient)

#### Inherited from[[endpoint.inherited-from]]

[InferenceClient](InferenceClient).[endpoint](InferenceClient#endpoint)

#### Defined in[[endpoint.defined-in]]

[inference/src/InferenceClient.ts:46](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/InferenceClient.ts#L46)

___

### featureExtraction

▸ **featureExtraction**(`args`, `options?`): `Promise`\<[`FeatureExtractionOutput`](../modules#featureextractionoutput)\>

This task reads some text and outputs raw float values, that are usually consumed as part of a semantic database/semantic search.

#### Parameters[[featureextraction.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`FeatureExtractionArgs`](../modules#featureextractionargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[featureextraction.returns]]

`Promise`\<[`FeatureExtractionOutput`](../modules#featureextractionoutput)\>

#### Inherited from[[featureextraction.inherited-from]]

[InferenceClient](InferenceClient).[featureExtraction](InferenceClient#featureextraction)

#### Defined in[[featureextraction.defined-in]]

[inference/src/tasks/nlp/featureExtraction.ts:22](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/featureExtraction.ts#L22)

___

### fillMask

▸ **fillMask**(`args`, `options?`): `Promise`\<`FillMaskOutput`\>

Tries to fill in a hole with a missing word (token to be precise). That’s the base task for BERT models.

#### Parameters[[fillmask.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`FillMaskArgs`](../modules#fillmaskargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[fillmask.returns]]

`Promise`\<`FillMaskOutput`\>

#### Inherited from[[fillmask.inherited-from]]

[InferenceClient](InferenceClient).[fillMask](InferenceClient#fillmask)

#### Defined in[[fillmask.defined-in]]

[inference/src/tasks/nlp/fillMask.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/fillMask.ts#L12)

___

### imageClassification

▸ **imageClassification**(`args`, `options?`): `Promise`\<`ImageClassificationOutput`\>

This task reads some image input and outputs the likelihood of classes.
Recommended model: google/vit-base-patch16-224

#### Parameters[[imageclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ImageClassificationArgs`](../modules#imageclassificationargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[imageclassification.returns]]

`Promise`\<`ImageClassificationOutput`\>

#### Inherited from[[imageclassification.inherited-from]]

[InferenceClient](InferenceClient).[imageClassification](InferenceClient#imageclassification)

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
| `args` | [`ImageSegmentationArgs`](../modules#imagesegmentationargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[imagesegmentation.returns]]

`Promise`\<`ImageSegmentationOutput`\>

#### Inherited from[[imagesegmentation.inherited-from]]

[InferenceClient](InferenceClient).[imageSegmentation](InferenceClient#imagesegmentation)

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
| `args` | [`ImageTextToImageArgs`](../modules#imagetexttoimageargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[imagetexttoimage.returns]]

`Promise`\<`Blob`\>

#### Inherited from[[imagetexttoimage.inherited-from]]

[InferenceClient](InferenceClient).[imageTextToImage](InferenceClient#imagetexttoimage)

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
| `args` | [`ImageTextToVideoArgs`](../modules#imagetexttovideoargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[imagetexttovideo.returns]]

`Promise`\<`Blob`\>

#### Inherited from[[imagetexttovideo.inherited-from]]

[InferenceClient](InferenceClient).[imageTextToVideo](InferenceClient#imagetexttovideo)

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
| `args` | [`ImageToImageArgs`](../modules#imagetoimageargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[imagetoimage.returns]]

`Promise`\<`Blob`\>

#### Inherited from[[imagetoimage.inherited-from]]

[InferenceClient](InferenceClient).[imageToImage](InferenceClient#imagetoimage)

#### Defined in[[imagetoimage.defined-in]]

[inference/src/tasks/cv/imageToImage.ts:14](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageToImage.ts#L14)

___

### imageToText

▸ **imageToText**(`args`, `options?`): `Promise`\<`ImageToTextOutput`\>

This task reads some image input and outputs the text caption.

#### Parameters[[imagetotext.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ImageToTextArgs`](../modules#imagetotextargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[imagetotext.returns]]

`Promise`\<`ImageToTextOutput`\>

#### Inherited from[[imagetotext.inherited-from]]

[InferenceClient](InferenceClient).[imageToText](InferenceClient#imagetotext)

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
| `args` | [`ImageToVideoArgs`](../modules#imagetovideoargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[imagetovideo.returns]]

`Promise`\<`Blob`\>

#### Inherited from[[imagetovideo.inherited-from]]

[InferenceClient](InferenceClient).[imageToVideo](InferenceClient#imagetovideo)

#### Defined in[[imagetovideo.defined-in]]

[inference/src/tasks/cv/imageToVideo.ts:14](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/imageToVideo.ts#L14)

___

### objectDetection

▸ **objectDetection**(`args`, `options?`): `Promise`\<`ObjectDetectionOutput`\>

This task reads some image input and outputs the likelihood of classes & bounding boxes of detected objects.
Recommended model: facebook/detr-resnet-50

#### Parameters[[objectdetection.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ObjectDetectionArgs`](../modules#objectdetectionargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[objectdetection.returns]]

`Promise`\<`ObjectDetectionOutput`\>

#### Inherited from[[objectdetection.inherited-from]]

[InferenceClient](InferenceClient).[objectDetection](InferenceClient#objectdetection)

#### Defined in[[objectdetection.defined-in]]

[inference/src/tasks/cv/objectDetection.ts:14](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/objectDetection.ts#L14)

___

### questionAnswering

▸ **questionAnswering**(`args`, `options?`): `Promise`\<`QuestionAnsweringOutput`[`number`]\>

Want to have a nice know-it-all bot that can answer any question?. Recommended model: deepset/roberta-base-squad2

#### Parameters[[questionanswering.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`QuestionAnsweringArgs`](../modules#questionansweringargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[questionanswering.returns]]

`Promise`\<`QuestionAnsweringOutput`[`number`]\>

#### Inherited from[[questionanswering.inherited-from]]

[InferenceClient](InferenceClient).[questionAnswering](InferenceClient#questionanswering)

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
| `args` | [`RequestArgs`](../modules#requestargs) |
| `options?` | [`Options`](../interfaces/Options) & \{ `task?`: [`InferenceTask`](../modules#inferencetask)  } |

#### Returns[[request.returns]]

`Promise`\<`T`\>

**`Deprecated`**

Use specific task functions instead. This function will be removed in a future version.

#### Inherited from[[request.inherited-from]]

[InferenceClient](InferenceClient).[request](InferenceClient#request)

#### Defined in[[request.defined-in]]

[inference/src/tasks/custom/request.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/custom/request.ts#L11)

___

### sentenceSimilarity

▸ **sentenceSimilarity**(`args`, `options?`): `Promise`\<`SentenceSimilarityOutput`\>

Calculate the semantic similarity between one text and a list of other sentences by comparing their embeddings.

#### Parameters[[sentencesimilarity.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`SentenceSimilarityArgs`](../modules#sentencesimilarityargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[sentencesimilarity.returns]]

`Promise`\<`SentenceSimilarityOutput`\>

#### Inherited from[[sentencesimilarity.inherited-from]]

[InferenceClient](InferenceClient).[sentenceSimilarity](InferenceClient#sentencesimilarity)

#### Defined in[[sentencesimilarity.defined-in]]

[inference/src/tasks/nlp/sentenceSimilarity.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/sentenceSimilarity.ts#L12)

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
| `args` | [`RequestArgs`](../modules#requestargs) |
| `options?` | [`Options`](../interfaces/Options) & \{ `task?`: [`InferenceTask`](../modules#inferencetask)  } |

#### Returns[[streamingrequest.returns]]

`AsyncGenerator`\<`T`\>

**`Deprecated`**

Use specific task functions instead. This function will be removed in a future version.

#### Inherited from[[streamingrequest.inherited-from]]

[InferenceClient](InferenceClient).[streamingRequest](InferenceClient#streamingrequest)

#### Defined in[[streamingrequest.defined-in]]

[inference/src/tasks/custom/streamingRequest.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/custom/streamingRequest.ts#L11)

___

### summarization

▸ **summarization**(`args`, `options?`): `Promise`\<`SummarizationOutput`\>

This task is well known to summarize longer text into shorter text. Be careful, some models have a maximum length of input. That means that the summary cannot handle full books for instance. Be careful when choosing your model.

#### Parameters[[summarization.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`SummarizationArgs`](../modules#summarizationargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[summarization.returns]]

`Promise`\<`SummarizationOutput`\>

#### Inherited from[[summarization.inherited-from]]

[InferenceClient](InferenceClient).[summarization](InferenceClient#summarization)

#### Defined in[[summarization.defined-in]]

[inference/src/tasks/nlp/summarization.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/summarization.ts#L12)

___

### tableQuestionAnswering

▸ **tableQuestionAnswering**(`args`, `options?`): `Promise`\<`TableQuestionAnsweringOutput`[`number`]\>

Don’t know SQL? Don’t want to dive into a large spreadsheet? Ask questions in plain english! Recommended model: google/tapas-base-finetuned-wtq.

#### Parameters[[tablequestionanswering.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TableQuestionAnsweringArgs`](../modules#tablequestionansweringargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[tablequestionanswering.returns]]

`Promise`\<`TableQuestionAnsweringOutput`[`number`]\>

#### Inherited from[[tablequestionanswering.inherited-from]]

[InferenceClient](InferenceClient).[tableQuestionAnswering](InferenceClient#tablequestionanswering)

#### Defined in[[tablequestionanswering.defined-in]]

[inference/src/tasks/nlp/tableQuestionAnswering.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/tableQuestionAnswering.ts#L12)

___

### tabularClassification

▸ **tabularClassification**(`args`, `options?`): `Promise`\<[`TabularClassificationOutput`](../modules#tabularclassificationoutput)\>

Predicts target label for a given set of features in tabular form.
Typically, you will want to train a classification model on your training data and use it with your new data of the same format.
Example model: vvmnnnkv/wine-quality

#### Parameters[[tabularclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TabularClassificationArgs`](../modules#tabularclassificationargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[tabularclassification.returns]]

`Promise`\<[`TabularClassificationOutput`](../modules#tabularclassificationoutput)\>

#### Inherited from[[tabularclassification.inherited-from]]

[InferenceClient](InferenceClient).[tabularClassification](InferenceClient#tabularclassification)

#### Defined in[[tabularclassification.defined-in]]

[inference/src/tasks/tabular/tabularClassification.ts:25](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/tabular/tabularClassification.ts#L25)

___

### tabularRegression

▸ **tabularRegression**(`args`, `options?`): `Promise`\<[`TabularRegressionOutput`](../modules#tabularregressionoutput)\>

Predicts target value for a given set of features in tabular form.
Typically, you will want to train a regression model on your training data and use it with your new data of the same format.
Example model: scikit-learn/Fish-Weight

#### Parameters[[tabularregression.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TabularRegressionArgs`](../modules#tabularregressionargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[tabularregression.returns]]

`Promise`\<[`TabularRegressionOutput`](../modules#tabularregressionoutput)\>

#### Inherited from[[tabularregression.inherited-from]]

[InferenceClient](InferenceClient).[tabularRegression](InferenceClient#tabularregression)

#### Defined in[[tabularregression.defined-in]]

[inference/src/tasks/tabular/tabularRegression.ts:25](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/tabular/tabularRegression.ts#L25)

___

### textClassification

▸ **textClassification**(`args`, `options?`): `Promise`\<`TextClassificationOutput`\>

Usually used for sentiment-analysis this will output the likelihood of classes of an input. Recommended model: distilbert-base-uncased-finetuned-sst-2-english

#### Parameters[[textclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextClassificationArgs`](../modules#textclassificationargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[textclassification.returns]]

`Promise`\<`TextClassificationOutput`\>

#### Inherited from[[textclassification.inherited-from]]

[InferenceClient](InferenceClient).[textClassification](InferenceClient#textclassification)

#### Defined in[[textclassification.defined-in]]

[inference/src/tasks/nlp/textClassification.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textClassification.ts#L12)

___

### textGeneration

▸ **textGeneration**(`args`, `options?`): `Promise`\<[`TextGenerationOutput`](../interfaces/TextGenerationOutput)\>

Use to continue text from a prompt. This is a very generic task. Recommended model: gpt2 (it’s a simple model, but fun to play with).

#### Parameters[[textgeneration.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`BaseArgs`](../interfaces/BaseArgs) & [`TextGenerationInput`](../interfaces/TextGenerationInput) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[textgeneration.returns]]

`Promise`\<[`TextGenerationOutput`](../interfaces/TextGenerationOutput)\>

#### Inherited from[[textgeneration.inherited-from]]

[InferenceClient](InferenceClient).[textGeneration](InferenceClient#textgeneration)

#### Defined in[[textgeneration.defined-in]]

[inference/src/tasks/nlp/textGeneration.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/textGeneration.ts#L12)

___

### textGenerationStream

▸ **textGenerationStream**(`args`, `options?`): `AsyncGenerator`\<[`TextGenerationStreamOutput`](../interfaces/TextGenerationStreamOutput)\>

Use to continue text from a prompt. Same as `textGeneration` but returns generator that can be read one token at a time

#### Parameters[[textgenerationstream.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`BaseArgs`](../interfaces/BaseArgs) & [`TextGenerationInput`](../interfaces/TextGenerationInput) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[textgenerationstream.returns]]

`AsyncGenerator`\<[`TextGenerationStreamOutput`](../interfaces/TextGenerationStreamOutput)\>

#### Inherited from[[textgenerationstream.inherited-from]]

[InferenceClient](InferenceClient).[textGenerationStream](InferenceClient#textgenerationstream)

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
| `args` | [`TextToAudioArgs`](../modules#texttoaudioargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[texttoaudio.returns]]

`Promise`\<`Blob`\>

#### Inherited from[[texttoaudio.inherited-from]]

[InferenceClient](InferenceClient).[textToAudio](InferenceClient#texttoaudio)

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
| `args` | [`TextToImageArgs`](../modules#texttoimageargs) |
| `options?` | `TextToImageOptions` & \{ `outputType`: ``"url"``  } |

#### Returns[[texttoimage.returns]]

`Promise`\<`string`\>

#### Inherited from[[texttoimage.inherited-from]]

[InferenceClient](InferenceClient).[textToImage](InferenceClient#texttoimage)

#### Defined in[[texttoimage.defined-in]]

[inference/src/tasks/cv/textToImage.ts:18](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToImage.ts#L18)

▸ **textToImage**(`args`, `options?`): `Promise`\<`string`\>

#### Parameters[[texttoimage.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextToImageArgs`](../modules#texttoimageargs) |
| `options?` | `TextToImageOptions` & \{ `outputType`: ``"dataUrl"``  } |

#### Returns[[texttoimage.returns]]

`Promise`\<`string`\>

#### Inherited from[[texttoimage.inherited-from]]

[InferenceClient](InferenceClient).[textToImage](InferenceClient#texttoimage)

#### Defined in[[texttoimage.defined-in]]

[inference/src/tasks/cv/textToImage.ts:22](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToImage.ts#L22)

▸ **textToImage**(`args`, `options?`): `Promise`\<`Blob`\>

#### Parameters[[texttoimage.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextToImageArgs`](../modules#texttoimageargs) |
| `options?` | `TextToImageOptions` & \{ `outputType?`: ``"blob"``  } |

#### Returns[[texttoimage.returns]]

`Promise`\<`Blob`\>

#### Inherited from[[texttoimage.inherited-from]]

[InferenceClient](InferenceClient).[textToImage](InferenceClient#texttoimage)

#### Defined in[[texttoimage.defined-in]]

[inference/src/tasks/cv/textToImage.ts:26](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToImage.ts#L26)

▸ **textToImage**(`args`, `options?`): `Promise`\<`Record`\<`string`, `unknown`\>\>

#### Parameters[[texttoimage.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextToImageArgs`](../modules#texttoimageargs) |
| `options?` | `TextToImageOptions` & \{ `outputType?`: ``"json"``  } |

#### Returns[[texttoimage.returns]]

`Promise`\<`Record`\<`string`, `unknown`\>\>

#### Inherited from[[texttoimage.inherited-from]]

[InferenceClient](InferenceClient).[textToImage](InferenceClient#texttoimage)

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
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[texttospeech.returns]]

`Promise`\<`Blob`\>

#### Inherited from[[texttospeech.inherited-from]]

[InferenceClient](InferenceClient).[textToSpeech](InferenceClient#texttospeech)

#### Defined in[[texttospeech.defined-in]]

[inference/src/tasks/audio/textToSpeech.ts:15](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/audio/textToSpeech.ts#L15)

___

### textToVideo

▸ **textToVideo**(`args`, `options?`): `Promise`\<[`TextToVideoOutput`](../modules#texttovideooutput)\>

#### Parameters[[texttovideo.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TextToVideoArgs`](../modules#texttovideoargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[texttovideo.returns]]

`Promise`\<[`TextToVideoOutput`](../modules#texttovideooutput)\>

#### Inherited from[[texttovideo.inherited-from]]

[InferenceClient](InferenceClient).[textToVideo](InferenceClient#texttovideo)

#### Defined in[[texttovideo.defined-in]]

[inference/src/tasks/cv/textToVideo.ts:15](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/textToVideo.ts#L15)

___

### tokenClassification

▸ **tokenClassification**(`args`, `options?`): `Promise`\<`TokenClassificationOutput`\>

Usually used for sentence parsing, either grammatical, or Named Entity Recognition (NER) to understand keywords contained within text. Recommended model: dbmdz/bert-large-cased-finetuned-conll03-english

#### Parameters[[tokenclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TokenClassificationArgs`](../modules#tokenclassificationargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[tokenclassification.returns]]

`Promise`\<`TokenClassificationOutput`\>

#### Inherited from[[tokenclassification.inherited-from]]

[InferenceClient](InferenceClient).[tokenClassification](InferenceClient#tokenclassification)

#### Defined in[[tokenclassification.defined-in]]

[inference/src/tasks/nlp/tokenClassification.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/tokenClassification.ts#L12)

___

### translation

▸ **translation**(`args`, `options?`): `Promise`\<`TranslationOutput`\>

This task is well known to translate text from one language to another. Recommended model: Helsinki-NLP/opus-mt-ru-en.

#### Parameters[[translation.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`TranslationArgs`](../modules#translationargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[translation.returns]]

`Promise`\<`TranslationOutput`\>

#### Inherited from[[translation.inherited-from]]

[InferenceClient](InferenceClient).[translation](InferenceClient#translation)

#### Defined in[[translation.defined-in]]

[inference/src/tasks/nlp/translation.ts:11](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/nlp/translation.ts#L11)

___

### visualQuestionAnswering

▸ **visualQuestionAnswering**(`args`, `options?`): `Promise`\<`VisualQuestionAnsweringOutput`[`number`]\>

Answers a question on an image. Recommended model: dandelin/vilt-b32-finetuned-vqa.

#### Parameters[[visualquestionanswering.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`VisualQuestionAnsweringArgs`](../modules#visualquestionansweringargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[visualquestionanswering.returns]]

`Promise`\<`VisualQuestionAnsweringOutput`[`number`]\>

#### Inherited from[[visualquestionanswering.inherited-from]]

[InferenceClient](InferenceClient).[visualQuestionAnswering](InferenceClient#visualquestionanswering)

#### Defined in[[visualquestionanswering.defined-in]]

[inference/src/tasks/multimodal/visualQuestionAnswering.ts:19](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/multimodal/visualQuestionAnswering.ts#L19)

___

### zeroShotClassification

▸ **zeroShotClassification**(`args`, `options?`): `Promise`\<`ZeroShotClassificationOutput`\>

This task is super useful to try out classification with zero code, you simply pass a sentence/paragraph and the possible labels for that sentence, and you get a result. Recommended model: facebook/bart-large-mnli.

#### Parameters[[zeroshotclassification.parameters]]

| Name | Type |
| :------ | :------ |
| `args` | [`ZeroShotClassificationArgs`](../modules#zeroshotclassificationargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[zeroshotclassification.returns]]

`Promise`\<`ZeroShotClassificationOutput`\>

#### Inherited from[[zeroshotclassification.inherited-from]]

[InferenceClient](InferenceClient).[zeroShotClassification](InferenceClient#zeroshotclassification)

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
| `args` | [`ZeroShotImageClassificationArgs`](../modules#zeroshotimageclassificationargs) |
| `options?` | [`Options`](../interfaces/Options) |

#### Returns[[zeroshotimageclassification.returns]]

`Promise`\<`ZeroShotImageClassificationOutput`\>

#### Inherited from[[zeroshotimageclassification.inherited-from]]

[InferenceClient](InferenceClient).[zeroShotImageClassification](InferenceClient#zeroshotimageclassification)

#### Defined in[[zeroshotimageclassification.defined-in]]

[inference/src/tasks/cv/zeroShotImageClassification.ts:44](https://github.com/huggingface/huggingface.js/blob/main/packages/inference/src/tasks/cv/zeroShotImageClassification.ts#L44)
