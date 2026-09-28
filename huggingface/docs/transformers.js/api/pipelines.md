# pipelines

Pipelines provide a high-level, easy-to-use API for running machine learning models.

**Example:** Instantiate pipeline using the `pipeline` function.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('sentiment-analysis');
const output = await classifier('I love transformers!');
// [{'label': 'POSITIVE', 'score': 0.999817686}]
```

## On this page

**Classes** — [`AudioClassificationPipeline`](#module_pipelines.AudioClassificationPipeline) · [`AutomaticSpeechRecognitionPipeline`](#module_pipelines.AutomaticSpeechRecognitionPipeline) · [`BackgroundRemovalPipeline`](#module_pipelines.BackgroundRemovalPipeline) · [`DepthEstimationPipeline`](#module_pipelines.DepthEstimationPipeline) · [`DocumentQuestionAnsweringPipeline`](#module_pipelines.DocumentQuestionAnsweringPipeline) · [`FeatureExtractionPipeline`](#module_pipelines.FeatureExtractionPipeline) · [`FillMaskPipeline`](#module_pipelines.FillMaskPipeline) · [`ImageClassificationPipeline`](#module_pipelines.ImageClassificationPipeline) · [`ImageFeatureExtractionPipeline`](#module_pipelines.ImageFeatureExtractionPipeline) · [`ImageSegmentationPipeline`](#module_pipelines.ImageSegmentationPipeline) · [`ImageToImagePipeline`](#module_pipelines.ImageToImagePipeline) · [`ImageToTextPipeline`](#module_pipelines.ImageToTextPipeline) · [`ObjectDetectionPipeline`](#module_pipelines.ObjectDetectionPipeline) · [`QuestionAnsweringPipeline`](#module_pipelines.QuestionAnsweringPipeline) · [`SummarizationPipeline`](#module_pipelines.SummarizationPipeline) · [`TextClassificationPipeline`](#module_pipelines.TextClassificationPipeline) · [`TextGenerationPipeline`](#module_pipelines.TextGenerationPipeline) · [`TextToAudioPipeline`](#module_pipelines.TextToAudioPipeline) · [`Text2TextGenerationPipeline`](#module_pipelines.Text2TextGenerationPipeline) · [`TokenClassificationPipeline`](#module_pipelines.TokenClassificationPipeline) · [`TranslationPipeline`](#module_pipelines.TranslationPipeline) · [`ZeroShotAudioClassificationPipeline`](#module_pipelines.ZeroShotAudioClassificationPipeline) · [`ZeroShotClassificationPipeline`](#module_pipelines.ZeroShotClassificationPipeline) · [`ZeroShotImageClassificationPipeline`](#module_pipelines.ZeroShotImageClassificationPipeline) · [`ZeroShotObjectDetectionPipeline`](#module_pipelines.ZeroShotObjectDetectionPipeline)

**Functions** — [`pipeline`](#module_pipelines.pipeline)

## Classes

### AudioClassificationPipeline

Audio classification pipeline using any `AutoModelForAudioClassification`.
This pipeline predicts the class of a raw waveform or an audio file.

**Example:** Perform audio classification with `Xenova/wav2vec2-large-xlsr-53-gender-recognition-librispeech`.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('audio-classification', 'Xenova/wav2vec2-large-xlsr-53-gender-recognition-librispeech');
const audio = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/jfk.wav';
const output = await classifier(audio);
// [
//   { label: 'male', score: 0.9981542229652405 },
//   { label: 'female', score: 0.001845747814513743 }
// ]
```

**Example:** Perform audio classification with `Xenova/ast-finetuned-audioset-10-10-0.4593` and return the top 4 results.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('audio-classification', 'Xenova/ast-finetuned-audioset-10-10-0.4593');
const audio = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/cat_meow.wav';
const output = await classifier(audio, { top_k: 4 });
// [
//   { label: 'Meow', score: 0.5617874264717102 },
//   { label: 'Cat', score: 0.22365376353263855 },
//   { label: 'Domestic animals, pets', score: 0.1141069084405899 },
//   { label: 'Animal', score: 0.08985692262649536 },
// ]
```

#### `AudioClassificationPipeline(audio, [options])`

**Parameters**

- `audio` ([`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[])
- `options` ([`AudioClassificationPipelineOptions`](./pipelines#module_pipelines.AudioClassificationPipelineOptions)) _optional_ — Parameters specific to audio classification pipelines.

**Returns:** `Promise`<[`AudioClassificationPipelineResult`](./pipelines#module_pipelines.AudioClassificationPipelineResult)<[`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[]>>

### AutomaticSpeechRecognitionPipeline

Automatic speech recognition pipeline for transcribing spoken text from audio.

**Example:** Transcribe English.
```javascript
import { pipeline } from '@huggingface/transformers';

const transcriber = await pipeline('automatic-speech-recognition', 'Xenova/whisper-tiny.en');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/jfk.wav';
const output = await transcriber(url);
// { text: " And so my fellow Americans ask not what your country can do for you, ask what you can do for your country." }
```

**Example:** Transcribe English with timestamps.
```javascript
import { pipeline } from '@huggingface/transformers';

const transcriber = await pipeline('automatic-speech-recognition', 'Xenova/whisper-tiny.en');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/jfk.wav';
const output = await transcriber(url, { return_timestamps: true });
// {
//   text: " And so my fellow Americans ask not what your country can do for you, ask what you can do for your country.",
//   chunks: [
//     { timestamp: [0, 8],  text: " And so my fellow Americans ask not what your country can do for you" },
//     { timestamp: [8, 11], text: " ask what you can do for your country." }
//   ]
// }
```

**Example:** Transcribe English with word-level timestamps.
```javascript
import { pipeline } from '@huggingface/transformers';

const transcriber = await pipeline('automatic-speech-recognition', 'Xenova/whisper-tiny.en');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/jfk.wav';
const output = await transcriber(url, { return_timestamps: 'word' });
// {
//   "text": " And so my fellow Americans ask not what your country can do for you ask what you can do for your country.",
//   "chunks": [
//     { "text": " And", "timestamp": [0, 0.78] },
//     { "text": " so", "timestamp": [0.78, 1.06] },
//     { "text": " my", "timestamp": [1.06, 1.46] },
//     ...
//     { "text": " for", "timestamp": [9.72, 9.92] },
//     { "text": " your", "timestamp": [9.92, 10.22] },
//     { "text": " country.", "timestamp": [10.22, 13.5] }
//   ]
// }
```

**Example:** Transcribe French.
```javascript
import { pipeline } from '@huggingface/transformers';

const transcriber = await pipeline('automatic-speech-recognition', 'Xenova/whisper-small');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/french-audio.mp3';
const output = await transcriber(url, { language: 'french', task: 'transcribe' });
// { text: " J'adore, j'aime, je n'aime pas, je déteste." }
```

**Example:** Translate French to English.
```javascript
import { pipeline } from '@huggingface/transformers';

const transcriber = await pipeline('automatic-speech-recognition', 'Xenova/whisper-small');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/french-audio.mp3';
const output = await transcriber(url, { language: 'french', task: 'translate' });
// { text: " I love, I like, I don't like, I hate." }
```

**Example:** Transcribe/translate audio longer than 30 seconds.
```javascript
import { pipeline } from '@huggingface/transformers';

const transcriber = await pipeline('automatic-speech-recognition', 'Xenova/whisper-tiny.en');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/ted_60.wav';
const output = await transcriber(url, { chunk_length_s: 30, stride_length_s: 5 });
// { text: " So in college, I was a government major, which means [...] So I'd start off light and I'd bump it up" }
```

#### `AutomaticSpeechRecognitionPipeline(audio, [options])`

**Parameters**

- `audio` ([`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[])
- `options` (`Partial`<[`AutomaticSpeechRecognitionConfig`](./pipelines#module_pipelines.AutomaticSpeechRecognitionConfig)>) _optional_

**Returns:** `Promise`<[`AutomaticSpeechRecognitionPipelineResult`](./pipelines#module_pipelines.AutomaticSpeechRecognitionPipelineResult)<[`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[]>>

### BackgroundRemovalPipeline

Background removal pipeline using compatible image segmentation models.
This pipeline removes the backgrounds of images.

**Example:** Perform background removal with `Xenova/modnet`.
```javascript
import { pipeline } from '@huggingface/transformers';

const segmenter = await pipeline('background-removal', 'Xenova/modnet');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/portrait-of-woman_small.jpg';
const output = await segmenter(url);
// RawImage { data: Uint8ClampedArray(648000) [ ... ], width: 360, height: 450, channels: 4 }
```

#### `BackgroundRemovalPipeline(images, [options])`

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `options` ([`BackgroundRemovalPipelineOptions`](./pipelines#module_pipelines.BackgroundRemovalPipelineOptions)) _optional_ — Parameters specific to background removal pipelines.

**Returns:** `Promise`<[`BackgroundRemovalPipelineResult`](./pipelines#module_pipelines.BackgroundRemovalPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### DepthEstimationPipeline

Depth estimation pipeline using any `AutoModelForDepthEstimation`. This pipeline predicts the depth of an image.

**Example:** Depth estimation with `onnx-community/depth-anything-v2-small`
```javascript
import { pipeline } from '@huggingface/transformers';

const depth_estimator = await pipeline('depth-estimation', 'onnx-community/depth-anything-v2-small');
const image = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/cats.jpg';
const output = await depth_estimator(image);
// {
//   predicted_depth: Tensor {
//     dims: [ 480, 640 ],
//     type: 'float32',
//     data: Float32Array(307200) [ 2.6300313472747803, 2.5856235027313232, 2.620532751083374, ... ],
//     size: 307200
//   },
//   depth: RawImage {
//     data: Uint8Array(307200) [ 106, 104, 106, ... ],
//     width: 640,
//     height: 480,
//     channels: 1
//   }
// }
```

#### `DepthEstimationPipeline(images)`

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])

**Returns:** `Promise`<[`DepthEstimationPipelineResult`](./pipelines#module_pipelines.DepthEstimationPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### DocumentQuestionAnsweringPipeline

Document Question Answering pipeline using any `AutoModelForDocumentQuestionAnswering`.
The inputs/outputs are similar to the (extractive) question answering pipeline; however,
the pipeline takes an image (and optional OCR'd words/boxes) as input instead of text context.

**Example:** Answer questions about a document with `Xenova/donut-base-finetuned-docvqa`.
```javascript
import { pipeline } from '@huggingface/transformers';

const qa_pipeline = await pipeline('document-question-answering', 'Xenova/donut-base-finetuned-docvqa');
const image = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/invoice.png';
const question = 'What is the invoice number?';
const output = await qa_pipeline(image, question);
// [{ answer: 'us-001' }]
```

#### `DocumentQuestionAnsweringPipeline(image, question, [options])`

**Parameters**

- `image` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]) — The image of the document to use.
- `question` (`string`) — A question to ask of the document.
- `options` (`Partial`<[`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters)>) _optional_ — Additional keyword arguments to pass along to the generate method of the model.

**Returns:** `Promise`<[`DocumentQuestionAnsweringOutput`](./pipelines#module_pipelines.DocumentQuestionAnsweringOutput)> — An object (or array of objects) containing the answer(s).

### FeatureExtractionPipeline

Feature extraction pipeline using no model head. This pipeline extracts the hidden
states from the base transformer for use as features in downstream tasks.

**Example:** Run feature extraction using `onnx-community/all-MiniLM-L6-v2-ONNX` (without pooling or normalization).
```javascript
import { pipeline } from '@huggingface/transformers';

const extractor = await pipeline('feature-extraction', 'onnx-community/all-MiniLM-L6-v2-ONNX');
const output = await extractor('This is a simple test.');
// Tensor {
//   type: 'float32',
//   data: Float32Array [0.2157987803220749, -0.09140099585056305, ...],
//   dims: [1, 8, 384]
// }

// You can convert this Tensor to a nested JavaScript array using `.tolist()`:
console.log(output.tolist());
```

**Example:** Run feature extraction using `onnx-community/all-MiniLM-L6-v2-ONNX` (with pooling and normalization).
```javascript
import { pipeline } from '@huggingface/transformers';

const extractor = await pipeline('feature-extraction', 'onnx-community/all-MiniLM-L6-v2-ONNX');
const output = await extractor('This is a simple test.', { pooling: 'mean', normalize: true });
// Tensor {
//   type: 'float32',
//   data: Float32Array [0.09528215229511261, -0.024730168282985687, ...],
//   dims: [1, 384]
// }

// You can convert this Tensor to a nested JavaScript array using `.tolist()`:
console.log(output.tolist());
```

**Example:** Run feature extraction using `onnx-community/all-MiniLM-L6-v2-ONNX` with pooling and binary quantization.
```javascript
import { pipeline } from '@huggingface/transformers';

const extractor = await pipeline('feature-extraction', 'onnx-community/all-MiniLM-L6-v2-ONNX');
const output = await extractor('This is a simple test.', { pooling: 'mean', quantize: true, precision: 'binary' });
// Tensor {
//   type: 'int8',
//   data: Int8Array [49, 108, 25, ...],
//   dims: [1, 48]
// }

// You can convert this Tensor to a nested JavaScript array using `.tolist()`:
console.log(output.tolist());
```

#### `FeatureExtractionPipeline(texts, [options])`

**Parameters**

- `texts` (`string` | `string[]`) — One or several texts (or one list of texts) to get the features of.
- `options` ([`FeatureExtractionPipelineOptions`](./pipelines#module_pipelines.FeatureExtractionPipelineOptions)) _optional_ — The options to use for feature extraction.

**Returns:** `Promise`<[`Tensor`](./utils/tensor#module_utils/tensor.Tensor)> — The features computed by the model.

### FillMaskPipeline

Masked language modeling prediction pipeline using compatible masked language models.

**Example:** Perform masked language modeling (a.k.a. "fill-mask") with `onnx-community/ettin-encoder-32m-ONNX`.
```javascript
import { pipeline } from '@huggingface/transformers';

const unmasker = await pipeline('fill-mask', 'onnx-community/ettin-encoder-32m-ONNX');
const output = await unmasker('The capital of France is [MASK].');
// [
//   { score: 0.5151872038841248, token: 7785, token_str: ' Paris', sequence: 'The capital of France is Paris.' },
//   { score: 0.033725105226039886, token: 42268, token_str: ' Lyon', sequence: 'The capital of France is Lyon.' },
//   { score: 0.031234024092555046, token: 23397, token_str: ' Nancy', sequence: 'The capital of France is Nancy.' },
//   { score: 0.02075139433145523, token: 30167, token_str: ' Brussels', sequence: 'The capital of France is Brussels.' },
//   { score: 0.018962178379297256, token: 31955, token_str: ' Geneva', sequence: 'The capital of France is Geneva.' }
// ]
```

**Example:** Perform masked language modeling (a.k.a. "fill-mask") with `Xenova/bert-base-cased`.
```javascript
import { pipeline } from '@huggingface/transformers';

const unmasker = await pipeline('fill-mask', 'Xenova/bert-base-cased');
const output = await unmasker('The goal of life is [MASK].');
// [
//   { score: 0.11368396878242493, sequence: "The goal of life is survival.", token: 8115, token_str: "survival" },
//   { score: 0.053510840982198715, sequence: "The goal of life is love.", token: 1567, token_str: "love" },
//   { score: 0.05041185021400452, sequence: "The goal of life is happiness.", token: 9266, token_str: "happiness" },
//   { score: 0.033218126744031906, sequence: "The goal of life is freedom.", token: 4438, token_str: "freedom" },
//   { score: 0.03301157429814339, sequence: "The goal of life is success.", token: 2244, token_str: "success" },
// ]
```

**Example:** Perform masked language modeling (a.k.a. "fill-mask") with `Xenova/bert-base-cased` (and return the top result).
```javascript
import { pipeline } from '@huggingface/transformers';

const unmasker = await pipeline('fill-mask', 'Xenova/bert-base-cased');
const output = await unmasker('The Milky Way is a [MASK] galaxy.', { top_k: 1 });
// [{ score: 0.5982972383499146, sequence: "The Milky Way is a spiral galaxy.", token: 14061, token_str: "spiral" }]
```

#### `FillMaskPipeline(texts, [options])`

**Parameters**

- `texts` (`string` | `string[]`)
- `options` ([`FillMaskPipelineOptions`](./pipelines#module_pipelines.FillMaskPipelineOptions)) _optional_ — Parameters specific to fill mask pipelines.

**Returns:** `Promise`<[`FillMaskPipelineResult`](./pipelines#module_pipelines.FillMaskPipelineResult)<`string` | `string[]`>>

### ImageClassificationPipeline

Image classification pipeline using any `AutoModelForImageClassification`.
This pipeline predicts the class of an image.

**Example:** Classify an image.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('image-classification', 'Xenova/vit-base-patch16-224');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/tiger.jpg';
const output = await classifier(url);
// [
//   { label: 'tiger, Panthera tigris', score: 0.632695734500885 },
//   ... (top 5 labels by default)
// ]
```

**Example:** Classify an image and return the top `n` classes.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('image-classification', 'Xenova/vit-base-patch16-224');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/tiger.jpg';
const output = await classifier(url, { top_k: 3 });
// [
//   { label: 'tiger, Panthera tigris', score: 0.632695734500885 },
//   { label: 'tiger cat', score: 0.3634825646877289 },
//   { label: 'lion, king of beasts, Panthera leo', score: 0.00045060308184474707 },
// ]
```

**Example:** Classify an image and return all classes.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('image-classification', 'Xenova/vit-base-patch16-224');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/tiger.jpg';
const output = await classifier(url, { top_k: null });
// [
//   { label: 'tiger, Panthera tigris', score: 0.632695734500885 },
//   { label: 'tiger cat', score: 0.3634825646877289 },
//   { label: 'lion, king of beasts, Panthera leo', score: 0.00045060308184474707 },
//   { label: 'jaguar, panther, Panthera onca, Felis onca', score: 0.00035465499968267977 },
//   ...
// ]
```

#### `ImageClassificationPipeline(images, [options])`

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `options` ([`ImageClassificationPipelineOptions`](./pipelines#module_pipelines.ImageClassificationPipelineOptions)) _optional_ — Parameters specific to image classification pipelines.

**Returns:** `Promise`<[`ImageClassificationPipelineResult`](./pipelines#module_pipelines.ImageClassificationPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### ImageFeatureExtractionPipeline

Image feature extraction pipeline using no model head. This pipeline extracts the hidden
states from the base transformer for use as features in downstream tasks.

**Example:** Perform image feature extraction with `onnx-community/dinov3-vits16-pretrain-lvd1689m-ONNX`.
```javascript
import { pipeline } from '@huggingface/transformers';

const image_feature_extractor = await pipeline('image-feature-extraction', 'onnx-community/dinov3-vits16-pretrain-lvd1689m-ONNX');
const image = 'https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/cats.png';
const features = await image_feature_extractor(image);
// Tensor {
//   dims: [ 1, 201, 384 ],
//   type: 'float32',
//   data: Float32Array(77184) [ ... ],
//   size: 77184
// }
```

**Example:** Compute image embeddings with `Xenova/clip-vit-base-patch32`.
```javascript
import { pipeline } from '@huggingface/transformers';

const image_feature_extractor = await pipeline('image-feature-extraction', 'Xenova/clip-vit-base-patch32');
const image = 'https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/cats.png';
const features = await image_feature_extractor(image);
// Tensor {
//   dims: [ 1, 512 ],
//   type: 'float32',
//   data: Float32Array(512) [ ... ],
//   size: 512
// }
```

#### `ImageFeatureExtractionPipeline(images, [options])`

**Parameters**

- `images` ([`ImagePipelineInputs`](./pipelines#module_pipelines.ImagePipelineInputs)) — One or several images (or one list of images) to get the features of.
- `options` ([`ImageFeatureExtractionPipelineOptions`](./pipelines#module_pipelines.ImageFeatureExtractionPipelineOptions)) _optional_ — The options to use for image feature extraction.

**Returns:** `Promise`<[`Tensor`](./utils/tensor#module_utils/tensor.Tensor)> — The image features computed by the model.

### ImageSegmentationPipeline

Image segmentation pipeline using compatible segmentation models.
This pipeline predicts masks of objects and their classes.

**Example:** Perform image segmentation with `Xenova/detr-resnet-50-panoptic`.
```javascript
import { pipeline } from '@huggingface/transformers';

const segmenter = await pipeline('image-segmentation', 'Xenova/detr-resnet-50-panoptic');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/cats.jpg';
const output = await segmenter(url);
// [
//   { label: 'remote', score: 0.9984649419784546, mask: RawImage { ... } },
//   { label: 'cat', score: 0.9994316101074219, mask: RawImage { ... } }
// ]
```

#### `ImageSegmentationPipeline(images, [options])`

**Parameters**

- `images` ([`ImagePipelineInputs`](./pipelines#module_pipelines.ImagePipelineInputs)) — The input images.
- `options` ([`ImageSegmentationPipelineOptions`](./pipelines#module_pipelines.ImageSegmentationPipelineOptions)) _optional_ — The options to use for image segmentation.

**Returns:** `Promise`<[`ImageSegmentationOutput`](./pipelines#module_pipelines.ImageSegmentationOutput)> — The annotated segments.

### ImageToImagePipeline

Image-to-image pipeline using any `AutoModelForImageToImage`. This pipeline generates an image based on a previous image input.

**Example:** Super-resolution with `Xenova/swin2SR-classical-sr-x2-64`
```javascript
import { pipeline } from '@huggingface/transformers';

const upscaler = await pipeline('image-to-image', 'Xenova/swin2SR-classical-sr-x2-64');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/butterfly.jpg';
const output = await upscaler(url);
// RawImage {
//   data: Uint8Array(786432) [ 41, 31, 24,  43, ... ],
//   width: 512,
//   height: 512,
//   channels: 3
// }
```

#### `ImageToImagePipeline(images)`

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])

**Returns:** `Promise`<[`ImageToImagePipelineResult`](./pipelines#module_pipelines.ImageToImagePipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### ImageToTextPipeline

Image-to-text pipeline using an `AutoModelForVision2Seq`. This pipeline predicts a caption for a given image.

**Example:** Generate a caption for an image with `Xenova/vit-gpt2-image-captioning`.
```javascript
import { pipeline } from '@huggingface/transformers';

const captioner = await pipeline('image-to-text', 'Xenova/vit-gpt2-image-captioning');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/cats.jpg';
const output = await captioner(url);
// [{ generated_text: 'a cat laying on a couch with another cat' }]
```

**Example:** Optical Character Recognition (OCR) with `Xenova/trocr-small-handwritten`.
```javascript
import { pipeline } from '@huggingface/transformers';

const captioner = await pipeline('image-to-text', 'Xenova/trocr-small-handwritten');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/handwriting.jpg';
const output = await captioner(url);
// [{ generated_text: 'Mr. Brown commented icily.' }]
```

#### `ImageToTextPipeline(texts, [options])`

**Parameters**

- `texts` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `options` (`Partial`<[`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters)>) _optional_

**Returns:** `Promise`<[`ImageToTextPipelineResult`](./pipelines#module_pipelines.ImageToTextPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### ObjectDetectionPipeline

Object detection pipeline using any `AutoModelForObjectDetection`.
This pipeline predicts bounding boxes of objects and their classes.

**Example:** Run object-detection with `Xenova/detr-resnet-50`.
```javascript
import { pipeline } from '@huggingface/transformers';

const detector = await pipeline('object-detection', 'Xenova/detr-resnet-50');
const img = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/cats.jpg';
const output = await detector(img, { threshold: 0.9 });
// [{
//   score: 0.9976370930671692,
//   label: "remote",
//   box: { xmin: 31, ymin: 68, xmax: 190, ymax: 118 }
// },
// ...
// {
//   score: 0.9984092116355896,
//   label: "cat",
//   box: { xmin: 331, ymin: 19, xmax: 649, ymax: 371 }
// }]
```

#### `ObjectDetectionPipeline(images, [options])`

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `options` ([`ObjectDetectionPipelineOptions`](./pipelines#module_pipelines.ObjectDetectionPipelineOptions)) _optional_ — Parameters specific to object detection pipelines.

**Returns:** `Promise`<[`ObjectDetectionPipelineResult`](./pipelines#module_pipelines.ObjectDetectionPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### QuestionAnsweringPipeline

Question answering pipeline using `AutoModelForQuestionAnswering`.

**Example:** Run question answering with `Xenova/distilbert-base-uncased-distilled-squad`.
```javascript
import { pipeline } from '@huggingface/transformers';

const answerer = await pipeline('question-answering', 'Xenova/distilbert-base-uncased-distilled-squad');
const question = 'Who was Jim Henson?';
const context = 'Jim Henson was a nice puppet.';
const output = await answerer(question, context);
// {
//   answer: "a nice puppet",
//   score: 0.5768911502526741
// }
```

#### `QuestionAnsweringPipeline(question, context, [options])`

**Parameters**

- `question` (`string` | `string[]`)
- `context` (`string` | `string[]`)
- `options` (`{ top_k?: number }`) _optional_

**Returns:** `Promise`<[`QuestionAnsweringPipelineResult`](./pipelines#module_pipelines.QuestionAnsweringPipelineResult)<`string` | `string[]`, `{ top_k?: number }`>>

### SummarizationPipeline

Summarization pipeline using sequence-to-sequence language models.

**Example:** Summarization with `Xenova/distilbart-cnn-6-6`.
```javascript
import { pipeline } from '@huggingface/transformers';

const summarizer = await pipeline('summarization', 'Xenova/distilbart-cnn-6-6');
const text = 'The tower is 324 metres (1,063 ft) tall, about the same height as an 81-storey building, ' +
  'and the tallest structure in Paris. Its base is square, measuring 125 metres (410 ft) on each side. ' +
  'During its construction, the Eiffel Tower surpassed the Washington Monument to become the tallest ' +
  'man-made structure in the world, a title it held for 41 years until the Chrysler Building in New ' +
  'York City was finished in 1930. It was the first structure to reach a height of 300 metres. Due to ' +
  'the addition of a broadcasting aerial at the top of the tower in 1957, it is now taller than the ' +
  'Chrysler Building by 5.2 metres (17 ft). Excluding transmitters, the Eiffel Tower is the second ' +
  'tallest free-standing structure in France after the Millau Viaduct.';
const output = await summarizer(text, {
  max_new_tokens: 100,
});
// [{ summary_text: ' The Eiffel Tower is about the same height as an 81-storey building and the tallest structure in Paris. It is the second tallest free-standing structure in France after the Millau Viaduct.' }]
```

#### `SummarizationPipeline(texts, [options])`

**Parameters**

- `texts` (`string` | `string[]`) — One or several articles (or one list of articles) to summarize.
- `options` ([`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters)) _optional_ — Additional keyword arguments to pass along to the generate method of the model.

**Returns:** `Promise`<[`SummarizationOutput`](./pipelines#module_pipelines.SummarizationOutput)>

### TextClassificationPipeline

Text classification pipeline using `AutoModelForSequenceClassification`.

**Example:** Sentiment analysis with `Xenova/distilbert-base-uncased-finetuned-sst-2-english`.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('sentiment-analysis', 'Xenova/distilbert-base-uncased-finetuned-sst-2-english');
const output = await classifier('I love transformers!');
// [{ label: 'POSITIVE', score: 0.999788761138916 }]
```

**Example:** Multilingual sentiment analysis with `Xenova/bert-base-multilingual-uncased-sentiment` (and return the top 5 classes).
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('sentiment-analysis', 'Xenova/bert-base-multilingual-uncased-sentiment');
const output = await classifier('Le meilleur film de tous les temps.', { top_k: 5 });
// [
//   { label: '5 stars', score: 0.9610759615898132 },
//   { label: '4 stars', score: 0.03323351591825485 },
//   { label: '3 stars', score: 0.0036155181005597115 },
//   { label: '1 star', score: 0.0011325967498123646 },
//   { label: '2 stars', score: 0.0009423971059732139 }
// ]
```

**Example:** Toxic comment classification with `Xenova/toxic-bert` (and return all classes).
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('text-classification', 'Xenova/toxic-bert');
const output = await classifier('I hate you!', { top_k: null });
// [
//   { label: 'toxic', score: 0.9593140482902527 },
//   { label: 'insult', score: 0.16187334060668945 },
//   { label: 'obscene', score: 0.03452680632472038 },
//   { label: 'identity_hate', score: 0.0223250575363636 },
//   { label: 'threat', score: 0.019197041168808937 },
//   { label: 'severe_toxic', score: 0.005651099607348442 }
// ]
```

#### `TextClassificationPipeline(texts, [options])`

**Parameters**

- `texts` (`string` | `string[]`)
- `options` ([`TextClassificationPipelineOptions`](./pipelines#module_pipelines.TextClassificationPipelineOptions)) _optional_

**Returns:** `Promise`<[`TextClassificationPipelineResult`](./pipelines#module_pipelines.TextClassificationPipelineResult)<`string` | `string[]`, [`TextClassificationPipelineOptions`](./pipelines#module_pipelines.TextClassificationPipelineOptions)>>

### TextGenerationPipeline

Language generation pipeline using compatible causal language models.
This pipeline predicts the words that will follow a specified text prompt.
For all generation parameters, see `GenerationConfig`.

**Example:** Text generation with `onnx-community/SmolLM2-135M-ONNX` (default settings).
```javascript
import { pipeline } from '@huggingface/transformers';

const generator = await pipeline('text-generation', 'onnx-community/SmolLM2-135M-ONNX');
const text = 'Once upon a time,';
const output = await generator(text, { max_new_tokens: 8 });
// [{ generated_text: 'Once upon a time, there was a little girl named Lily.' }]
```

**Example:** Chat completion with `onnx-community/Qwen3-0.6B-ONNX`.
```javascript
import { pipeline, TextStreamer } from '@huggingface/transformers';

// Create a text generation pipeline
const generator = await pipeline(
  'text-generation',
  'onnx-community/Qwen3-0.6B-ONNX',
  { dtype: 'q4f16' },
);

// Define the list of messages
const messages = [
  { role: 'system', content: 'You are a helpful assistant.' },
  { role: 'user', content: 'Write me a poem about Machine Learning.' },
];

// Generate a response
const output = await generator(messages, {
  max_new_tokens: 512,
  do_sample: false,
  streamer: new TextStreamer(generator.tokenizer, { skip_prompt: true, skip_special_tokens: true }),
});
console.log(output[0].generated_text.at(-1)?.content);
```

#### `TextGenerationPipeline(texts, [options])`

**Parameters**

- `texts` (`string` | [`Chat`](./pipelines#module_pipelines.Chat) | `string[]` | [`Chat`](./pipelines#module_pipelines.Chat)[])
- `options` (`Partial`<[`TextGenerationConfig`](./pipelines#module_pipelines.TextGenerationConfig)>) _optional_

**Returns:** `Promise`<[`TextGenerationResult`](./pipelines#module_pipelines.TextGenerationResult)<`string` | [`Chat`](./pipelines#module_pipelines.Chat) | `string[]` | [`Chat`](./pipelines#module_pipelines.Chat)[]>>

### TextToAudioPipeline

Text-to-audio generation pipeline using any `AutoModelForTextToWaveform` or `AutoModelForTextToSpectrogram`.
This pipeline generates an audio file from an input text and optional other conditional inputs.

**Example:** Generate audio from text with `onnx-community/Supertonic-TTS-ONNX`.
```javascript
import { pipeline } from '@huggingface/transformers';

const synthesizer = await pipeline('text-to-speech', 'onnx-community/Supertonic-TTS-ONNX');
const speaker_embeddings = 'https://huggingface.co/onnx-community/Supertonic-TTS-ONNX/resolve/main/voices/F1.bin';
const output = await synthesizer('Hello there, how are you doing?', { speaker_embeddings });
// RawAudio {
//   audio: Float32Array(95232) [-0.000482565927086398, -0.0004853440332226455, ...],
//   sampling_rate: 44100
// }

// Optional: Save the audio to a .wav file or Blob
await output.save('output.wav'); // You can also use `output.toBlob()` to access the audio as a Blob
```

**Example:** Multilingual speech generation with `Xenova/mms-tts-fra`. See the [MMS-TTS model list](https://huggingface.co/models?pipeline_tag=text-to-speech&other=vits&sort=trending) for available languages.
```javascript
import { pipeline } from '@huggingface/transformers';

const synthesizer = await pipeline('text-to-speech', 'Xenova/mms-tts-fra');
const output = await synthesizer('Bonjour');
// RawAudio {
//   audio: Float32Array(23808) [-0.00037693005288019776, 0.0003325853613205254, ...],
//   sampling_rate: 16000
// }
```

#### `TextToAudioPipeline(text, [options])`

**Parameters**

- `text` (`string` | `string[]`)
- `options` ([`TextToAudioPipelineOptions`](./pipelines#module_pipelines.TextToAudioPipelineOptions)) _optional_ — Parameters specific to text-to-audio pipelines.

**Returns:** `Promise`<[`TextToAudioPipelineResult`](./pipelines#module_pipelines.TextToAudioPipelineResult)<`string` | `string[]`>>

#### `TextToAudioPipeline.constructor(options)`

Create a new TextToAudioPipeline.

**Parameters**

- `options` ([`TextToAudioPipelineConstructorArgs`](./pipelines#module_pipelines.TextToAudioPipelineConstructorArgs)) — An object used to instantiate the pipeline.

### Text2TextGenerationPipeline

Text-to-text generation pipeline using a model that performs text-to-text generation tasks.

**Example:** Text-to-text generation with `Xenova/LaMini-Flan-T5-783M`.
```javascript
import { pipeline } from '@huggingface/transformers';

const generator = await pipeline('text2text-generation', 'Xenova/LaMini-Flan-T5-783M');
const output = await generator('how can I become more healthy?', {
  max_new_tokens: 100,
});
// [{ generated_text: "To become more healthy, you can: 1. Eat a balanced diet with plenty of fruits, vegetables, whole grains, lean proteins, and healthy fats. 2. Stay hydrated by drinking plenty of water. 3. Get enough sleep and manage stress levels. 4. Avoid smoking and excessive alcohol consumption. 5. Regularly exercise and maintain a healthy weight. 6. Practice good hygiene and sanitation. 7. Seek medical attention if you experience any health issues." }]
```

#### `Text2TextGenerationPipeline(texts, [options])`

**Parameters**

- `texts` (`string` | `string[]`) — Input text for the encoder.
- `options` (`Partial`<[`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters)>) _optional_ — Additional keyword arguments to pass along to the generate method of the model.

**Returns:** `Promise`<[`Text2TextGenerationOutput`](./pipelines#module_pipelines.Text2TextGenerationOutput)>

### TokenClassificationPipeline

Named entity recognition pipeline using `AutoModelForTokenClassification`.

**Example:** Perform named entity recognition with `Xenova/bert-base-NER`.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('token-classification', 'Xenova/bert-base-NER');
const output = await classifier('My name is Sarah and I live in London');
// [
//   { entity: 'B-PER', score: 0.9980202913284302, index: 4, word: 'Sarah' },
//   { entity: 'B-LOC', score: 0.9994474053382874, index: 9, word: 'London' }
// ]
```

**Example:** Perform named entity recognition with `Xenova/bert-base-NER` (and return all entity labels).
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('token-classification', 'Xenova/bert-base-NER');
const output = await classifier('Sarah lives in the United States of America', { ignore_labels: [] });
// [
//   { entity: 'B-PER', score: 0.9966587424278259, index: 1, word: 'Sarah' },
//   { entity: 'O', score: 0.9987385869026184, index: 2, word: 'lives' },
//   { entity: 'O', score: 0.9990072846412659, index: 3, word: 'in' },
//   { entity: 'O', score: 0.9988298416137695, index: 4, word: 'the' },
//   { entity: 'B-LOC', score: 0.9995510578155518, index: 5, word: 'United' },
//   { entity: 'I-LOC', score: 0.9990395307540894, index: 6, word: 'States' },
//   { entity: 'I-LOC', score: 0.9986724853515625, index: 7, word: 'of' },
//   { entity: 'I-LOC', score: 0.9975294470787048, index: 8, word: 'America' }
// ]
```

**Example:** Group adjacent BIO/BIOES tokens into entity spans using `aggregation_strategy: "simple"`.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('token-classification', 'Xenova/bert-base-NER');
const output = await classifier('My name is Sarah and I live in London', { aggregation_strategy: 'simple' });
// [
//   { entity_group: 'PER', score: 0.9985477924346924, word: 'Sarah' },
//   { entity_group: 'LOC', score: 0.999621570110321, word: 'London' }
// ]
```

#### `TokenClassificationPipeline(texts, [options])`

**Parameters**

- `texts` (`string` | `string[]`)
- `options` ([`TokenClassificationPipelineOptions`](./pipelines#module_pipelines.TokenClassificationPipelineOptions)) _optional_

**Returns:** `Promise`<[`TokenClassificationOutput`](./pipelines#module_pipelines.TokenClassificationOutput)<[`TokenClassificationPipelineOptions`](./pipelines#module_pipelines.TokenClassificationPipelineOptions)>[] | [`TokenClassificationOutput`](./pipelines#module_pipelines.TokenClassificationOutput)<[`TokenClassificationPipelineOptions`](./pipelines#module_pipelines.TokenClassificationPipelineOptions)>>

### TranslationPipeline

Translates text from one language to another.

**Example:** Multilingual translation with `Xenova/nllb-200-distilled-600M`.

See the [FLORES-200 language list](https://github.com/facebookresearch/flores/blob/main/flores200/README.md#languages-in-flores-200)
for the available languages and their corresponding codes.

```javascript
import { pipeline } from '@huggingface/transformers';

const translator = await pipeline('translation', 'Xenova/nllb-200-distilled-600M');
const output = await translator('जीवन एक चॉकलेट बॉक्स की तरह है।', {
  src_lang: 'hin_Deva', // Hindi
  tgt_lang: 'fra_Latn', // French
});
// [{ translation_text: 'La vie est comme une boîte à chocolat.' }]
```

**Example:** Multilingual translation with `Xenova/m2m100_418M`.

See the [M2M100 language list](https://huggingface.co/facebook/m2m100_418M#languages-covered)
for the available languages and their corresponding codes.

```javascript
import { pipeline } from '@huggingface/transformers';

const translator = await pipeline('translation', 'Xenova/m2m100_418M');
const output = await translator('生活就像一盒巧克力。', {
  src_lang: 'zh', // Chinese
  tgt_lang: 'en', // English
});
// [{ translation_text: 'Life is like a box of chocolate.' }]
```

**Example:** Multilingual translation with `Xenova/mbart-large-50-many-to-many-mmt`.

See the [mBART-50 language list](https://huggingface.co/facebook/mbart-large-50-many-to-many-mmt#languages-covered)
for the available languages and their corresponding codes.

```javascript
import { pipeline } from '@huggingface/transformers';

const translator = await pipeline('translation', 'Xenova/mbart-large-50-many-to-many-mmt');
const output = await translator('संयुक्त राष्ट्र के प्रमुख का कहना है कि सीरिया में कोई सैन्य समाधान नहीं है', {
  src_lang: 'hi_IN', // Hindi
  tgt_lang: 'fr_XX', // French
});
// [{ translation_text: "Le chef de la mission de l 'ONU a déclaré qu 'il n' y a pas de solution militaire en Syria." }]
```

#### `TranslationPipeline(texts, [options])`

**Parameters**

- `texts` (`string` | `string[]`) — Texts to be translated.
- `options` ([`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters)) _optional_ — Additional keyword arguments to pass along to the generate method of the model.

**Returns:** `Promise`<[`TranslationOutput`](./pipelines#module_pipelines.TranslationOutput)>

### ZeroShotAudioClassificationPipeline

Zero-shot audio classification pipeline using `ClapModel`. This pipeline predicts the class of an audio clip from
audio and a set of `candidate_labels`.

**Example:** Perform zero-shot audio classification with `Xenova/clap-htsat-unfused`.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('zero-shot-audio-classification', 'Xenova/clap-htsat-unfused');
const audio = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/dog_barking.wav';
const candidate_labels = ['dog', 'vacuum cleaner'];
const scores = await classifier(audio, candidate_labels);
// [
//   { score: 0.9993992447853088, label: 'dog' },
//   { score: 0.0006007603369653225, label: 'vacuum cleaner' }
// ]
```

#### `ZeroShotAudioClassificationPipeline(audio, candidate_labels, [options])`

**Parameters**

- `audio` ([`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[])
- `candidate_labels` (`string[]`)
- `options` ([`ZeroShotAudioClassificationPipelineOptions`](./pipelines#module_pipelines.ZeroShotAudioClassificationPipelineOptions)) _optional_ — Parameters specific to zero-shot audio classification pipelines.

**Returns:** `Promise`<[`ZeroShotAudioClassificationPipelineResult`](./pipelines#module_pipelines.ZeroShotAudioClassificationPipelineResult)<[`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[]>>

### ZeroShotClassificationPipeline

NLI-based zero-shot classification pipeline using `AutoModelForSequenceClassification`
trained on NLI (natural language inference) tasks. This is similar to `text-classification`
pipelines, but these models do not require a hard-coded set of classes. Candidate classes can
be provided at runtime, making zero-shot classification slower but much more flexible.

**Example:** Zero-shot classification with `Xenova/mobilebert-uncased-mnli`.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('zero-shot-classification', 'Xenova/mobilebert-uncased-mnli');
const text = 'Last week I upgraded my iOS version and ever since then my phone has been overheating whenever I use your app.';
const labels = [ 'mobile', 'billing', 'website', 'account access' ];
const output = await classifier(text, labels);
// {
//   sequence: 'Last week I upgraded my iOS version and ever since then my phone has been overheating whenever I use your app.',
//   labels: [ 'mobile', 'website', 'billing', 'account access' ],
//   scores: [ 0.5562091040482018, 0.1843621307860853, 0.13942646639336376, 0.12000229877234923 ]
// }
```

**Example:** Zero-shot classification with `Xenova/nli-deberta-v3-xsmall` (multi-label).
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('zero-shot-classification', 'Xenova/nli-deberta-v3-xsmall');
const text = 'I have a problem with my iphone that needs to be resolved asap!';
const labels = [ 'urgent', 'not urgent', 'phone', 'tablet', 'computer' ];
const output = await classifier(text, labels, { multi_label: true });
// {
//   sequence: 'I have a problem with my iphone that needs to be resolved asap!',
//   labels: [ 'urgent', 'phone', 'computer', 'tablet', 'not urgent' ],
//   scores: [ 0.9958870956360275, 0.9923963400697035, 0.002333537946160235, 0.0015134138567598765, 0.0010699384208377163 ]
// }
```

#### `ZeroShotClassificationPipeline(texts, candidate_labels, [options])`

**Parameters**

- `texts` (`string` | `string[]`)
- `candidate_labels` (`string` | `string[]`)
- `options` ([`ZeroShotClassificationPipelineOptions`](./pipelines#module_pipelines.ZeroShotClassificationPipelineOptions)) _optional_ — Parameters specific to zero-shot classification pipelines.

**Returns:** `Promise`<[`ZeroShotClassificationPipelineResult`](./pipelines#module_pipelines.ZeroShotClassificationPipelineResult)<`string` | `string[]`>>

#### `ZeroShotClassificationPipeline.constructor(options)`

Create a new ZeroShotClassificationPipeline.

**Parameters**

- `options` ([`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs)) — An object used to instantiate the pipeline.

### ZeroShotImageClassificationPipeline

Zero-shot image classification pipeline. This pipeline predicts the class of
an image when you provide an image and a set of `candidate_labels`.

**Example:** Zero-shot image classification with `Xenova/clip-vit-base-patch32`.
```javascript
import { pipeline } from '@huggingface/transformers';

const classifier = await pipeline('zero-shot-image-classification', 'Xenova/clip-vit-base-patch32');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/tiger.jpg';
const output = await classifier(url, ['tiger', 'horse', 'dog']);
// [
//   { score: 0.9993917942047119, label: 'tiger' },
//   { score: 0.0003519294841680676, label: 'horse' },
//   { score: 0.0002562698791734874, label: 'dog' }
// ]
```

#### `ZeroShotImageClassificationPipeline(images, candidate_labels, [options])`

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `candidate_labels` (`string[]`)
- `options` ([`ZeroShotImageClassificationPipelineOptions`](./pipelines#module_pipelines.ZeroShotImageClassificationPipelineOptions)) _optional_ — Parameters specific to zero-shot image classification pipelines.

**Returns:** `Promise`<[`ZeroShotImageClassificationPipelineResult`](./pipelines#module_pipelines.ZeroShotImageClassificationPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### ZeroShotObjectDetectionPipeline

Zero-shot object detection pipeline. This pipeline predicts bounding boxes of
objects when you provide an image and a set of `candidate_labels`.

**Example:** Zero-shot object detection with `Xenova/owlvit-base-patch32`.
```javascript
import { pipeline } from '@huggingface/transformers';

const detector = await pipeline('zero-shot-object-detection', 'Xenova/owlvit-base-patch32');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/astronaut.png';
const candidate_labels = ['human face', 'rocket', 'helmet', 'american flag'];
const output = await detector(url, candidate_labels);
// [
//   {
//     score: 0.24392342567443848,
//     label: 'human face',
//     box: { xmin: 180, ymin: 67, xmax: 274, ymax: 175 }
//   },
//   {
//     score: 0.15129457414150238,
//     label: 'american flag',
//     box: { xmin: 0, ymin: 4, xmax: 106, ymax: 513 }
//   },
//   {
//     score: 0.13649864494800568,
//     label: 'helmet',
//     box: { xmin: 277, ymin: 337, xmax: 511, ymax: 511 }
//   },
//   {
//     score: 0.10262022167444229,
//     label: 'rocket',
//     box: { xmin: 352, ymin: -1, xmax: 463, ymax: 287 }
//   }
// ]
```

**Example:** Zero-shot object detection with `Xenova/owlvit-base-patch32` (returning the top 4 matches and setting a threshold).
```javascript
import { pipeline } from '@huggingface/transformers';

const detector = await pipeline('zero-shot-object-detection', 'Xenova/owlvit-base-patch32');
const url = 'https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/beach.png';
const candidate_labels = ['hat', 'book', 'sunglasses', 'camera'];
const output = await detector(url, candidate_labels, { top_k: 4, threshold: 0.05 });
// [
//   {
//     score: 0.1606510728597641,
//     label: 'sunglasses',
//     box: { xmin: 347, ymin: 229, xmax: 429, ymax: 264 }
//   },
//   {
//     score: 0.08935828506946564,
//     label: 'hat',
//     box: { xmin: 38, ymin: 174, xmax: 258, ymax: 364 }
//   },
//   {
//     score: 0.08530698716640472,
//     label: 'camera',
//     box: { xmin: 187, ymin: 350, xmax: 260, ymax: 411 }
//   },
//   {
//     score: 0.08349756896495819,
//     label: 'book',
//     box: { xmin: 261, ymin: 280, xmax: 494, ymax: 425 }
//   }
// ]
```

#### `ZeroShotObjectDetectionPipeline(images, candidate_labels, [options])`

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `candidate_labels` (`string[]`)
- `options` ([`ZeroShotObjectDetectionPipelineOptions`](./pipelines#module_pipelines.ZeroShotObjectDetectionPipelineOptions)) _optional_ — Parameters specific to zero-shot object detection pipelines.

**Returns:** `Promise`<[`ZeroShotObjectDetectionPipelineResult`](./pipelines#module_pipelines.ZeroShotObjectDetectionPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

## Functions

### `pipeline(task, [model], [options])`

Utility factory method to build a `Pipeline` object.

**Parameters**

- `task` ([`PipelineType`](./pipelines#module_pipelines.PipelineType)) — The task defining which pipeline will be returned. Currently accepted tasks are:
  - `"audio-classification"`: will return a `AudioClassificationPipeline`.
  - `"automatic-speech-recognition"` (alias "asr" available): will return a `AutomaticSpeechRecognitionPipeline`.
  - `"background-removal"`: will return a `BackgroundRemovalPipeline`.
  - `"depth-estimation"`: will return a `DepthEstimationPipeline`.
  - `"document-question-answering"`: will return a `DocumentQuestionAnsweringPipeline`.
  - `"feature-extraction"` (alias "embeddings" available): will return a `FeatureExtractionPipeline`.
  - `"fill-mask"`: will return a `FillMaskPipeline`.
  - `"image-classification"`: will return a `ImageClassificationPipeline`.
  - `"image-feature-extraction"`: will return a `ImageFeatureExtractionPipeline`.
  - `"image-segmentation"`: will return a `ImageSegmentationPipeline`.
  - `"image-to-image"`: will return a `ImageToImagePipeline`.
  - `"image-to-text"`: will return a `ImageToTextPipeline`.
  - `"object-detection"`: will return a `ObjectDetectionPipeline`.
  - `"question-answering"`: will return a `QuestionAnsweringPipeline`.
  - `"summarization"`: will return a `SummarizationPipeline`.
  - `"text2text-generation"`: will return a `Text2TextGenerationPipeline`.
  - `"text-classification"` (alias "sentiment-analysis" available): will return a `TextClassificationPipeline`.
  - `"text-generation"`: will return a `TextGenerationPipeline`.
  - `"text-to-audio"` (alias "text-to-speech" available): will return a `TextToAudioPipeline`.
  - `"token-classification"` (alias "ner" available): will return a `TokenClassificationPipeline`.
  - `"translation"`: will return a `TranslationPipeline`.
  - `"translation_xx_to_yy"`: will return a `TranslationPipeline`.
  - `"zero-shot-classification"`: will return a `ZeroShotClassificationPipeline`.
  - `"zero-shot-audio-classification"`: will return a `ZeroShotAudioClassificationPipeline`.
  - `"zero-shot-image-classification"`: will return a `ZeroShotImageClassificationPipeline`.
  - `"zero-shot-object-detection"`: will return a `ZeroShotObjectDetectionPipeline`.
- `model` (`string`) _optional_ — defaults to `null` — The name of the pretrained model to use. If not specified, the default model for the task will be used.
- `options` ([`PretrainedModelOptions`](./utils/hub#module_utils/hub.PretrainedModelOptions)) _optional_ — Optional parameters for the pipeline.

**Returns:** `Promise`<[`AllTasks`](./pipelines#module_pipelines.AllTasks)> — A Pipeline object for the specified task.

**Throws**

- `Error` — If an unsupported pipeline is requested.

## Type Definitions

### ImageInput

_Type:_ `string` | [`RawImage`](./utils/image#module_utils/image.RawImage) | `URL` | `Blob` | `HTMLCanvasElement` | `OffscreenCanvas`

### ImagePipelineInputs

_Type:_ [`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]

### AudioInput

_Type:_ `string` | `URL` | `Float32Array` | `Float64Array`

### AudioPipelineInputs

_Type:_ [`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[]

### BoundingBox

**Properties**

- `xmin` (`number`) — The minimum x coordinate of the bounding box.
- `ymin` (`number`) — The minimum y coordinate of the bounding box.
- `xmax` (`number`) — The maximum x coordinate of the bounding box.
- `ymax` (`number`) — The maximum y coordinate of the bounding box.

### Disposable

**Properties**

- `dispose` ([`DisposeType`](./pipelines#module_pipelines.DisposeType)) — A promise that resolves when the pipeline has been disposed.

### ModelTokenizerConstructorArgs

**Properties**

- `task` (`string`) — The task of the pipeline. Useful for specifying subtasks.
- `model` ([`PreTrainedModel`](./models#module_models.PreTrainedModel)) — The model used by the pipeline.
- `tokenizer` ([`PreTrainedTokenizer`](./tokenizers#module_tokenizers.PreTrainedTokenizer)) — The tokenizer used by the pipeline.

### TextPipelineConstructorArgs

An object used to instantiate a text-based pipeline.

_Type:_ [`ModelTokenizerConstructorArgs`](./pipelines#module_pipelines.ModelTokenizerConstructorArgs)

### ModelProcessorConstructorArgs

**Properties**

- `task` (`string`) — The task of the pipeline. Useful for specifying subtasks.
- `model` ([`PreTrainedModel`](./models#module_models.PreTrainedModel)) — The model used by the pipeline.
- `processor` ([`Processor`](./processors#module_processors.Processor)) — The processor used by the pipeline.

### AudioPipelineConstructorArgs

An object used to instantiate an audio-based pipeline.

_Type:_ [`ModelProcessorConstructorArgs`](./pipelines#module_pipelines.ModelProcessorConstructorArgs)

### ImagePipelineConstructorArgs

An object used to instantiate an image-based pipeline.

_Type:_ [`ModelProcessorConstructorArgs`](./pipelines#module_pipelines.ModelProcessorConstructorArgs)

### ModelTokenizerProcessorConstructorArgs

**Properties**

- `task` (`string`) — The task of the pipeline. Useful for specifying subtasks.
- `model` ([`PreTrainedModel`](./models#module_models.PreTrainedModel)) — The model used by the pipeline.
- `tokenizer` ([`PreTrainedTokenizer`](./tokenizers#module_tokenizers.PreTrainedTokenizer)) — The tokenizer used by the pipeline.
- `processor` ([`Processor`](./processors#module_processors.Processor)) — The processor used by the pipeline.

### TextAudioPipelineConstructorArgs

An object used to instantiate a text- and audio-based pipeline.

_Type:_ [`ModelTokenizerProcessorConstructorArgs`](./pipelines#module_pipelines.ModelTokenizerProcessorConstructorArgs)

### TextImagePipelineConstructorArgs

An object used to instantiate a text- and image-based pipeline.

_Type:_ [`ModelTokenizerProcessorConstructorArgs`](./pipelines#module_pipelines.ModelTokenizerProcessorConstructorArgs)

### TaskType

_Type:_ `keyof typeof SUPPORTED_TASKS`

### AliasType

_Type:_ `keyof typeof TASK_ALIASES`

### PipelineType

All possible pipeline types.

_Type:_ [`TaskType`](./pipelines#module_pipelines.TaskType) | [`AliasType`](./pipelines#module_pipelines.AliasType)

### SupportedTasks

A mapping of pipeline names to their corresponding pipeline classes.

### AliasTasks

A mapping from pipeline aliases to their corresponding pipeline classes.

### AllTasks

A mapping from all pipeline names and aliases to their corresponding pipeline classes.

_Type:_ [`SupportedTasks`](./pipelines#module_pipelines.SupportedTasks) & [`AliasTasks`](./pipelines#module_pipelines.AliasTasks)

### FillMaskOutput

_Type:_ [`FillMaskSingle`](./pipelines#module_pipelines.FillMaskSingle)[]

### TextClassificationOutput

_Type:_ [`TextClassificationSingle`](./pipelines#module_pipelines.TextClassificationSingle)[]

### TokenClassificationOutput

Single element of a token-classification result, parameterised by the options type `O` so that
`entity` vs. `entity_group` is known statically based on `aggregation_strategy`.

- Grouped (present when `O["aggregation_strategy"]` is `"simple"`):
    `{ word, score, entity_group }`
- Raw (the default — when `aggregation_strategy` is missing, `"none"`, or `undefined`):
    `{ word, score, entity, index }`
- Both variants also carry optional `start` / `end` character offsets.

When `O` is the untyped `TokenClassificationPipelineOptions`, the element is the union of both shapes,
narrowable via `if ("entity_group" in item)` / `if (item.entity !== undefined)`.

### QuestionAnsweringOutput

**Properties**

- `score` (`number`) — The probability associated with the answer.
- `start` (`number`) _optional_ — The character start index of the answer (in the tokenized version of the input).
- `end` (`number`) _optional_ — The character end index of the answer (in the tokenized version of the input).
- `answer` (`string`) — The answer to the question.

### SummarizationOutput

_Type:_ [`SummarizationSingle`](./pipelines#module_pipelines.SummarizationSingle)[]

### TranslationOutput

_Type:_ [`TranslationSingle`](./pipelines#module_pipelines.TranslationSingle)[]

### Text2TextGenerationOutput

_Type:_ [`Text2TextGenerationSingle`](./pipelines#module_pipelines.Text2TextGenerationSingle)[]

### TextGenerationOutput

_Type:_ [`TextGenerationSingle`](./pipelines#module_pipelines.TextGenerationSingle)[]

### TextGenerationStringOutput

_Type:_ [`TextGenerationSingleString`](./pipelines#module_pipelines.TextGenerationSingleString)[]

### TextGenerationChatOutput

_Type:_ [`TextGenerationSingleChat`](./pipelines#module_pipelines.TextGenerationSingleChat)[]

### ZeroShotClassificationOutput

**Properties**

- `sequence` (`string`) — The sequence for which this is the output.
- `labels` (`string[]`) — The labels sorted by order of likelihood.
- `scores` (`number[]`) — The probabilities for each of the labels.

### AudioClassificationOutput

_Type:_ [`AudioClassificationSingle`](./pipelines#module_pipelines.AudioClassificationSingle)[]

### ZeroShotAudioClassificationOutput

_Type:_ [`ZeroShotAudioClassificationOutputSingle`](./pipelines#module_pipelines.ZeroShotAudioClassificationOutputSingle)[]

### AutomaticSpeechRecognitionOutput

**Properties**

- `text` (`string`) — The recognized text.
- `chunks` ([`Chunk`](./pipelines#module_pipelines.Chunk)[]) _optional_ — When using `return_timestamps`, the `chunks` will become a list
  containing all the various text chunks identified by the model.

### TextToAudioOutput

_Type:_ [`RawAudio`](./utils/audio#module_utils/audio.RawAudio)[]

### ImageClassificationOutput

_Type:_ [`ImageClassificationSingle`](./pipelines#module_pipelines.ImageClassificationSingle)[]

### ImageSegmentationOutput

_Type:_ [`ImageSegmentationOutputSingle`](./pipelines#module_pipelines.ImageSegmentationOutputSingle)[]

### ImageToTextOutput

_Type:_ [`ImageToTextSingle`](./pipelines#module_pipelines.ImageToTextSingle)[]

### ObjectDetectionOutput

_Type:_ [`ObjectDetectionPipelineSingle`](./pipelines#module_pipelines.ObjectDetectionPipelineSingle)[]

### ZeroShotObjectDetectionOutput

_Type:_ [`ZeroShotObjectDetectionOutputSingle`](./pipelines#module_pipelines.ZeroShotObjectDetectionOutputSingle)[]

### ZeroShotImageClassificationOutput

_Type:_ [`ZeroShotImageClassificationOutputSingle`](./pipelines#module_pipelines.ZeroShotImageClassificationOutputSingle)[]

### DocumentQuestionAnsweringOutput

_Type:_ [`DocumentQuestionAnsweringSingle`](./pipelines#module_pipelines.DocumentQuestionAnsweringSingle)[]

### DepthEstimationOutput

**Properties**

- `predicted_depth` ([`Tensor`](./utils/tensor#module_utils/tensor.Tensor)) — The raw depth map predicted by the model.
- `depth` ([`RawImage`](./utils/image#module_utils/image.RawImage)) — The processed depth map as an image (with the same size as the input image).

### AudioClassificationSingle

**Properties**

- `label` (`string`) — The label predicted.
- `score` (`number`) — The corresponding probability.

### AudioClassificationPipelineOptions

Parameters specific to audio classification pipelines.

**Properties**

- `top_k` (`number`) _optional_ — defaults to `5` — The number of top labels to return.
  If the provided number is `null` or higher than the number of labels available in the model configuration,
  it will default to the number of labels.

### AudioClassificationPipelineType

_Type:_ [`AudioPipelineConstructorArgs`](./pipelines#module_pipelines.AudioPipelineConstructorArgs) & [`AudioClassificationPipelineCallback`](./pipelines#module_pipelines.AudioClassificationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### AudioClassificationPipelineResult

_Type:_ [`AudioClassificationOutput`](./pipelines#module_pipelines.AudioClassificationOutput)[] | [`AudioClassificationOutput`](./pipelines#module_pipelines.AudioClassificationOutput)

### Chunk

**Properties**

- `timestamp` (`[number, number]`) — The start and end timestamp of the chunk in seconds.
- `text` (`string`) — The recognized text.

### AutomaticSpeechRecognitionSpecificParams

Parameters specific to automatic-speech-recognition pipelines.

**Properties**

- `return_timestamps` (`boolean` | `'word'`) _optional_ — Whether to return timestamps or not. Default is `false`.
- `chunk_length_s` (`number`) _optional_ — The length of audio chunks to process in seconds. Default is 0 (no chunking).
- `stride_length_s` (`number`) _optional_ — The length of overlap between consecutive audio chunks in seconds. If not provided, defaults to `chunk_length_s / 6`.
- `force_full_sequences` (`boolean`) _optional_ — Whether to force outputting full sequences or not. Default is `false`.
- `language` (`string`) _optional_ — The source language. Default is `null`, meaning it should be auto-detected. Use this to potentially improve performance if the source language is known.
- `task` (`string`) _optional_ — The task to perform. Default is `null`, meaning it should be auto-detected.
- `num_frames` (`number`) _optional_ — The number of frames in the input audio.

### AutomaticSpeechRecognitionConfig

_Type:_ [`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters) & [`AutomaticSpeechRecognitionSpecificParams`](./pipelines#module_pipelines.AutomaticSpeechRecognitionSpecificParams)

### AutomaticSpeechRecognitionPipelineType

_Type:_ [`TextAudioPipelineConstructorArgs`](./pipelines#module_pipelines.TextAudioPipelineConstructorArgs) & [`AutomaticSpeechRecognitionPipelineCallback`](./pipelines#module_pipelines.AutomaticSpeechRecognitionPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### AutomaticSpeechRecognitionPipelineResult

_Type:_ [`AutomaticSpeechRecognitionOutput`](./pipelines#module_pipelines.AutomaticSpeechRecognitionOutput)[] | [`AutomaticSpeechRecognitionOutput`](./pipelines#module_pipelines.AutomaticSpeechRecognitionOutput)

### BackgroundRemovalPipelineOptions

Parameters specific to background removal pipelines.

### BackgroundRemovalPipelineType

_Type:_ [`ImagePipelineConstructorArgs`](./pipelines#module_pipelines.ImagePipelineConstructorArgs) & [`BackgroundRemovalPipelineCallback`](./pipelines#module_pipelines.BackgroundRemovalPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### BackgroundRemovalPipelineResult

_Type:_ [`RawImage`](./utils/image#module_utils/image.RawImage)[] | [`RawImage`](./utils/image#module_utils/image.RawImage)

### DepthEstimationPipelineType

_Type:_ [`ImagePipelineConstructorArgs`](./pipelines#module_pipelines.ImagePipelineConstructorArgs) & [`DepthEstimationPipelineCallback`](./pipelines#module_pipelines.DepthEstimationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### DepthEstimationPipelineResult

_Type:_ [`DepthEstimationOutput`](./pipelines#module_pipelines.DepthEstimationOutput)[] | [`DepthEstimationOutput`](./pipelines#module_pipelines.DepthEstimationOutput)

### DocumentQuestionAnsweringSingle

**Properties**

- `answer` (`string`) — The generated text.

### DocumentQuestionAnsweringPipelineType

_Type:_ [`TextImagePipelineConstructorArgs`](./pipelines#module_pipelines.TextImagePipelineConstructorArgs) & [`DocumentQuestionAnsweringPipelineCallback`](./pipelines#module_pipelines.DocumentQuestionAnsweringPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### FeatureExtractionPipelineOptions

Parameters specific to feature extraction pipelines.

**Properties**

- `pooling` (`'none'` | `'mean'` | `'cls'` | `'first_token'` | `'eos'` | `'last_token'`) _optional_ — defaults to `"none"` — The pooling method to use.
- `normalize` (`boolean`) _optional_ — defaults to `false` — Whether to normalize the embeddings in the last dimension.
- `quantize` (`boolean`) _optional_ — defaults to `false` — Whether to quantize the embeddings.
- `precision` (`'binary'` | `'ubinary'`) _optional_ — defaults to `'binary'` — The precision to use for quantization.

### FeatureExtractionPipelineType

_Type:_ [`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs) & [`FeatureExtractionPipelineCallback`](./pipelines#module_pipelines.FeatureExtractionPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### FillMaskSingle

**Properties**

- `sequence` (`string`) — The corresponding input with the mask token prediction.
- `score` (`number`) — The corresponding probability.
- `token` (`number`) — The predicted token id (to replace the masked one).
- `token_str` (`string`) — The predicted token (to replace the masked one).

### FillMaskPipelineOptions

Parameters specific to fill mask pipelines.

**Properties**

- `top_k` (`number`) _optional_ — defaults to `5` — When passed, overrides the number of predictions to return.

### FillMaskPipelineType

_Type:_ [`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs) & [`FillMaskPipelineCallback`](./pipelines#module_pipelines.FillMaskPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### FillMaskPipelineResult

_Type:_ [`FillMaskOutput`](./pipelines#module_pipelines.FillMaskOutput)[] | [`FillMaskOutput`](./pipelines#module_pipelines.FillMaskOutput)

### ImageClassificationSingle

**Properties**

- `label` (`string`) — The label identified by the model.
- `score` (`number`) — The score attributed by the model for that label.

### ImageClassificationPipelineOptions

Parameters specific to image classification pipelines.

**Properties**

- `top_k` (`number` | `null`) _optional_ — defaults to `5` — The number of top labels to return. Set to `null` to return all labels.

### ImageClassificationPipelineType

_Type:_ [`ImagePipelineConstructorArgs`](./pipelines#module_pipelines.ImagePipelineConstructorArgs) & [`ImageClassificationPipelineCallback`](./pipelines#module_pipelines.ImageClassificationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ImageClassificationPipelineResult

_Type:_ [`ImageClassificationOutput`](./pipelines#module_pipelines.ImageClassificationOutput)[] | [`ImageClassificationOutput`](./pipelines#module_pipelines.ImageClassificationOutput)

### ImageFeatureExtractionPipelineOptions

Parameters specific to image feature extraction pipelines.

**Properties**

- `pool` (`boolean`) _optional_ — defaults to `null` — Whether to return the pooled output. If set to `false`, the model will return the raw hidden states.

### ImageFeatureExtractionPipelineType

_Type:_ [`ImagePipelineConstructorArgs`](./pipelines#module_pipelines.ImagePipelineConstructorArgs) & [`ImageFeatureExtractionPipelineCallback`](./pipelines#module_pipelines.ImageFeatureExtractionPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ImageSegmentationOutputSingle

**Properties**

- `label` (`string` | `null`) — The label of the segment.
- `score` (`number` | `null`) — The score of the segment.
- `mask` ([`RawImage`](./utils/image#module_utils/image.RawImage)) — The mask of the segment.

### ImageSegmentationPipelineOptions

Parameters specific to image segmentation pipelines.

**Properties**

- `threshold` (`number`) _optional_ — defaults to `0.5` — Probability threshold to filter out predicted masks.
- `mask_threshold` (`number`) _optional_ — defaults to `0.5` — Threshold to use when turning the predicted masks into binary values.
- `overlap_mask_area_threshold` (`number`) _optional_ — defaults to `0.8` — Mask overlap threshold to eliminate small, disconnected segments.
- `subtask` (`null` | `string`) _optional_ — defaults to `null` — Segmentation task to be performed. One of [`panoptic`, `instance`, and `semantic`],
  depending on model capabilities. If not set, the pipeline will attempt to resolve (in that order).
- `label_ids_to_fuse` (`number[]`) _optional_ — defaults to `null` — List of label IDs to fuse. If not set, do not fuse any labels.
- `target_sizes` (`number[][]`) _optional_ — defaults to `null` — List of target sizes for the input images. If not set, use the original image sizes.

### ImageSegmentationPipelineType

_Type:_ [`ImagePipelineConstructorArgs`](./pipelines#module_pipelines.ImagePipelineConstructorArgs) & [`ImageSegmentationPipelineCallback`](./pipelines#module_pipelines.ImageSegmentationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ImageToImagePipelineType

_Type:_ [`ImagePipelineConstructorArgs`](./pipelines#module_pipelines.ImagePipelineConstructorArgs) & [`ImageToImagePipelineCallback`](./pipelines#module_pipelines.ImageToImagePipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ImageToImagePipelineResult

_Type:_ [`RawImage`](./utils/image#module_utils/image.RawImage)[] | [`RawImage`](./utils/image#module_utils/image.RawImage)

### ImageToTextSingle

**Properties**

- `generated_text` (`string`) — The generated text.

### ImageToTextPipelineType

_Type:_ [`TextImagePipelineConstructorArgs`](./pipelines#module_pipelines.TextImagePipelineConstructorArgs) & [`ImageToTextPipelineCallback`](./pipelines#module_pipelines.ImageToTextPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ImageToTextPipelineResult

_Type:_ [`ImageToTextOutput`](./pipelines#module_pipelines.ImageToTextOutput)[] | [`ImageToTextOutput`](./pipelines#module_pipelines.ImageToTextOutput)

### ObjectDetectionPipelineSingle

**Properties**

- `label` (`string`) — The class label identified by the model.
- `score` (`number`) — The score attributed by the model for that label.
- `box` ([`BoundingBox`](./pipelines#module_pipelines.BoundingBox)) — The bounding box of detected object in image's original size, or as a percentage if `percentage` is set to true.

### ObjectDetectionPipelineOptions

Parameters specific to object detection pipelines.

**Properties**

- `threshold` (`number`) _optional_ — defaults to `0.9` — The threshold used to filter boxes by score.
- `percentage` (`boolean`) _optional_ — defaults to `false` — Whether to return box coordinates as percentages (true) or pixels (false).

### ObjectDetectionPipelineType

_Type:_ [`ImagePipelineConstructorArgs`](./pipelines#module_pipelines.ImagePipelineConstructorArgs) & [`ObjectDetectionPipelineCallback`](./pipelines#module_pipelines.ObjectDetectionPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ObjectDetectionPipelineResult

_Type:_ [`ObjectDetectionOutput`](./pipelines#module_pipelines.ObjectDetectionOutput)[] | [`ObjectDetectionOutput`](./pipelines#module_pipelines.ObjectDetectionOutput)

### QuestionAnsweringPipelineOptions

Parameters specific to question answering pipelines.

**Properties**

- `top_k` (`number`) _optional_ — defaults to `1` — The number of top answer predictions to return.

### QuestionAnsweringPipelineType

_Type:_ [`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs) & [`QuestionAnsweringPipelineCallback`](./pipelines#module_pipelines.QuestionAnsweringPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### QuestionAnsweringPipelineResult

_Type:_ [`QuestionAnsweringOutput`](./pipelines#module_pipelines.QuestionAnsweringOutput)[][] | [`QuestionAnsweringOutput`](./pipelines#module_pipelines.QuestionAnsweringOutput)[] | [`QuestionAnsweringOutput`](./pipelines#module_pipelines.QuestionAnsweringOutput)

### SummarizationSingle

**Properties**

- `summary_text` (`string`) — The summary text.

### SummarizationPipelineType

_Type:_ [`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs) & [`SummarizationPipelineCallback`](./pipelines#module_pipelines.SummarizationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### TextClassificationSingle

**Properties**

- `label` (`string`) — The label predicted.
- `score` (`number`) — The corresponding probability.

### TextClassificationPipelineOptions

Parameters specific to text classification pipelines.

**Properties**

- `top_k` (`number` | `null`) _optional_ — defaults to `1` — The number of top predictions to return. If set to `null`, all predictions are returned.

### TextClassificationPipelineResult

_Type:_ [`TextClassificationOutput`](./pipelines#module_pipelines.TextClassificationOutput)[] | [`TextClassificationOutput`](./pipelines#module_pipelines.TextClassificationOutput)

### TextClassificationPipelineType

_Type:_ [`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs) & [`TextClassificationPipelineCallback`](./pipelines#module_pipelines.TextClassificationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### Chat

_Type:_ [`Message`](./tokenizers#module_tokenizers.Message)[]

### TextGenerationSingleString

**Properties**

- `generated_text` (`string`) — The generated text.

### TextGenerationSingleChat

**Properties**

- `generated_text` ([`Chat`](./pipelines#module_pipelines.Chat)) — The generated chat.

### TextGenerationSingle

_Type:_ [`TextGenerationSingleString`](./pipelines#module_pipelines.TextGenerationSingleString) | [`TextGenerationSingleChat`](./pipelines#module_pipelines.TextGenerationSingleChat)

### TextGenerationSpecificParams

Parameters specific to text-generation pipelines.

**Properties**

- `add_special_tokens` (`boolean`) _optional_ — Whether to add special tokens when tokenizing the sequences.
- `return_full_text` (`boolean`) _optional_ — defaults to `true` — If set to `false` only added text is returned, otherwise the full text is returned.
- `tools` (`Object[]` | `null`) _optional_ — defaults to `null` — A list of tools to expose to chat templates that support tool use.
- `documents` (`Record`<`string`, `string`>[] | `null`) _optional_ — defaults to `null` — A list of documents to expose to chat templates that support RAG.
- `chat_template` (`string` | `null`) _optional_ — defaults to `null` — A specific chat template (or template name) to apply.
- `tokenizer_encode_kwargs` (`Object`) _optional_ — Additional keyword arguments to pass along to the encoding step of the tokenizer.
  If the text input is a chat, it is passed to `apply_chat_template`. Otherwise, it is passed to the tokenizer's call function.

### TextGenerationConfig

_Type:_ [`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters) & [`TextGenerationSpecificParams`](./pipelines#module_pipelines.TextGenerationSpecificParams)

### TextGenerationPipelineType

_Type:_ [`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs) & [`TextGenerationPipelineCallback`](./pipelines#module_pipelines.TextGenerationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### TextGenerationResult

_Type:_ [`TextGenerationStringOutput`](./pipelines#module_pipelines.TextGenerationStringOutput) | [`TextGenerationChatOutput`](./pipelines#module_pipelines.TextGenerationChatOutput) | [`TextGenerationStringOutput`](./pipelines#module_pipelines.TextGenerationStringOutput)[] | [`TextGenerationChatOutput`](./pipelines#module_pipelines.TextGenerationChatOutput)[]

### VocoderOptions

**Properties**

- `vocoder` ([`PreTrainedModel`](./models#module_models.PreTrainedModel)) _optional_ — The vocoder used by the pipeline (if the model uses one). If not provided, use the default HifiGan vocoder.

### TextToAudioPipelineConstructorArgs

_Type:_ [`TextAudioPipelineConstructorArgs`](./pipelines#module_pipelines.TextAudioPipelineConstructorArgs) & [`VocoderOptions`](./pipelines#module_pipelines.VocoderOptions)

### TextToAudioPipelineOptions

Parameters specific to text-to-audio pipelines.

**Properties**

- `speaker_embeddings` ([`Tensor`](./utils/tensor#module_utils/tensor.Tensor) | `Float32Array` | `string` | `URL`) _optional_ — defaults to `null` — The speaker embeddings (if the model requires it).
- `num_inference_steps` (`number`) _optional_ — The number of denoising steps (if the model supports it).
  More denoising steps usually lead to higher quality audio but slower inference.
- `speed` (`number`) _optional_ — The speed of the generated audio (if the model supports it).

### TextToAudioPipelineType

_Type:_ [`TextToAudioPipelineConstructorArgs`](./pipelines#module_pipelines.TextToAudioPipelineConstructorArgs) & [`TextToAudioPipelineCallback`](./pipelines#module_pipelines.TextToAudioPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### TextToAudioPipelineResult

_Type:_ [`TextToAudioOutput`](./pipelines#module_pipelines.TextToAudioOutput) | [`RawAudio`](./utils/audio#module_utils/audio.RawAudio)

### Text2TextGenerationSingle

**Properties**

- `generated_text` (`string`) — The generated text.

### Text2TextGenerationPipelineType

_Type:_ [`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs) & [`Text2TextGenerationPipelineCallback`](./pipelines#module_pipelines.Text2TextGenerationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### AggregationStrategy

Strategy for fusing tokens based on the model prediction.
- `"none"`: Return raw per-token predictions.
- `"simple"`: Group entities using BIO / BIOES tags (see pipeline docs for details).

_Type:_ `'none'` | `'simple'`

### TokenClassificationPipelineOptions

**Properties**

- `ignore_labels` (`string[]`) _optional_ — A list of labels to ignore.
- `aggregation_strategy` ([`AggregationStrategy`](./pipelines#module_pipelines.AggregationStrategy)) _optional_ — defaults to `"none"` — Token-fusion strategy.
  When set to anything other than `"none"`, results use `entity_group` instead of `entity`.

### TokenClassificationPipelineType

_Type:_ [`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs) & [`TokenClassificationPipelineCallback`](./pipelines#module_pipelines.TokenClassificationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### TranslationSingle

**Properties**

- `translation_text` (`string`) — The translated text.

### TranslationPipelineType

_Type:_ [`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs) & [`TranslationPipelineCallback`](./pipelines#module_pipelines.TranslationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ZeroShotAudioClassificationOutputSingle

**Properties**

- `label` (`string`) — The label identified by the model. It is one of the suggested `candidate_label`.
- `score` (`number`) — The score attributed by the model for that label (between 0 and 1).

### ZeroShotAudioClassificationPipelineOptions

Parameters specific to zero-shot audio classification pipelines.

**Properties**

- `hypothesis_template` (`string`) _optional_ — defaults to `"This is a sound of {}."` — The sentence used in conjunction with `candidate_labels`
  to attempt the audio classification by replacing the placeholder with the candidate_labels.
  Then likelihood is estimated by using `logits_per_audio`.

### ZeroShotAudioClassificationPipelineType

_Type:_ [`TextAudioPipelineConstructorArgs`](./pipelines#module_pipelines.TextAudioPipelineConstructorArgs) & [`ZeroShotAudioClassificationPipelineCallback`](./pipelines#module_pipelines.ZeroShotAudioClassificationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ZeroShotAudioClassificationPipelineResult

_Type:_ [`ZeroShotAudioClassificationOutput`](./pipelines#module_pipelines.ZeroShotAudioClassificationOutput)[] | [`ZeroShotAudioClassificationOutput`](./pipelines#module_pipelines.ZeroShotAudioClassificationOutput)

### ZeroShotClassificationPipelineOptions

Parameters specific to zero-shot classification pipelines.

**Properties**

- `hypothesis_template` (`string`) _optional_ — defaults to `"This example is {}."` — The template used to turn each
  candidate label into an NLI-style hypothesis. The candidate label will replace the {} placeholder.
- `multi_label` (`boolean`) _optional_ — defaults to `false` — Whether multiple candidate labels can be true.
  If `false`, the scores are normalized such that the sum of the label likelihoods for each sequence
  is 1. If `true`, the labels are considered independent and probabilities are normalized for each
  candidate by doing a softmax of the entailment score vs. the contradiction score.

### ZeroShotClassificationPipelineType

_Type:_ [`TextPipelineConstructorArgs`](./pipelines#module_pipelines.TextPipelineConstructorArgs) & [`ZeroShotClassificationPipelineCallback`](./pipelines#module_pipelines.ZeroShotClassificationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ZeroShotClassificationPipelineResult

_Type:_ [`ZeroShotClassificationOutput`](./pipelines#module_pipelines.ZeroShotClassificationOutput)[] | [`ZeroShotClassificationOutput`](./pipelines#module_pipelines.ZeroShotClassificationOutput)

### ZeroShotImageClassificationOutputSingle

**Properties**

- `label` (`string`) — The label identified by the model. It is one of the suggested `candidate_label`.
- `score` (`number`) — The score attributed by the model for that label (between 0 and 1).

### ZeroShotImageClassificationPipelineOptions

Parameters specific to zero-shot image classification pipelines.

**Properties**

- `hypothesis_template` (`string`) _optional_ — defaults to `"This is a photo of {}"` — The sentence used in conjunction with `candidate_labels`
  to attempt the image classification by replacing the placeholder with the candidate_labels.
  Then likelihood is estimated by using `logits_per_image`.

### ZeroShotImageClassificationPipelineType

_Type:_ [`TextImagePipelineConstructorArgs`](./pipelines#module_pipelines.TextImagePipelineConstructorArgs) & [`ZeroShotImageClassificationPipelineCallback`](./pipelines#module_pipelines.ZeroShotImageClassificationPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ZeroShotImageClassificationPipelineResult

_Type:_ [`ZeroShotImageClassificationOutput`](./pipelines#module_pipelines.ZeroShotImageClassificationOutput)[] | [`ZeroShotImageClassificationOutput`](./pipelines#module_pipelines.ZeroShotImageClassificationOutput)

### ZeroShotObjectDetectionOutputSingle

**Properties**

- `label` (`string`) — Text query corresponding to the found object.
- `score` (`number`) — Score corresponding to the object (between 0 and 1).
- `box` ([`BoundingBox`](./pipelines#module_pipelines.BoundingBox)) — Bounding box of the detected object in image's original size, or as a percentage if `percentage` is set to true.

### ZeroShotObjectDetectionPipelineOptions

Parameters specific to zero-shot object detection pipelines.

**Properties**

- `threshold` (`number`) _optional_ — defaults to `0.1` — The probability necessary to make a prediction.
- `top_k` (`number`) _optional_ — defaults to `null` — The number of top predictions to return.
  If the provided number is `null` or higher than the number of predictions available, it will default
  to the number of predictions.
- `percentage` (`boolean`) _optional_ — defaults to `false` — Whether to return box coordinates as percentages (true) or pixels (false).

### ZeroShotObjectDetectionPipelineType

_Type:_ [`TextImagePipelineConstructorArgs`](./pipelines#module_pipelines.TextImagePipelineConstructorArgs) & [`ZeroShotObjectDetectionPipelineCallback`](./pipelines#module_pipelines.ZeroShotObjectDetectionPipelineCallback) & [`Disposable`](./pipelines#module_pipelines.Disposable)

### ZeroShotObjectDetectionPipelineResult

_Type:_ [`ZeroShotObjectDetectionOutput`](./pipelines#module_pipelines.ZeroShotObjectDetectionOutput)[] | [`ZeroShotObjectDetectionOutput`](./pipelines#module_pipelines.ZeroShotObjectDetectionOutput)

## Callbacks

### DisposeType

**Returns:** `Promise`<`void`> — A promise that resolves when the item has been disposed.

### AudioClassificationPipelineCallback

**Parameters**

- `audio` ([`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[])
- `options` ([`AudioClassificationPipelineOptions`](./pipelines#module_pipelines.AudioClassificationPipelineOptions)) _optional_ — Parameters specific to audio classification pipelines.

**Returns:** `Promise`<[`AudioClassificationPipelineResult`](./pipelines#module_pipelines.AudioClassificationPipelineResult)<[`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[]>>

### AutomaticSpeechRecognitionPipelineCallback

**Parameters**

- `audio` ([`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[])
- `options` (`Partial`<[`AutomaticSpeechRecognitionConfig`](./pipelines#module_pipelines.AutomaticSpeechRecognitionConfig)>) _optional_

**Returns:** `Promise`<[`AutomaticSpeechRecognitionPipelineResult`](./pipelines#module_pipelines.AutomaticSpeechRecognitionPipelineResult)<[`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[]>>

### BackgroundRemovalPipelineCallback

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `options` ([`BackgroundRemovalPipelineOptions`](./pipelines#module_pipelines.BackgroundRemovalPipelineOptions)) _optional_ — Parameters specific to background removal pipelines.

**Returns:** `Promise`<[`BackgroundRemovalPipelineResult`](./pipelines#module_pipelines.BackgroundRemovalPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### DepthEstimationPipelineCallback

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])

**Returns:** `Promise`<[`DepthEstimationPipelineResult`](./pipelines#module_pipelines.DepthEstimationPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### DocumentQuestionAnsweringPipelineCallback

**Parameters**

- `image` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]) — The image of the document to use.
- `question` (`string`) — A question to ask of the document.
- `options` (`Partial`<[`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters)>) _optional_ — Additional keyword arguments to pass along to the generate method of the model.

**Returns:** `Promise`<[`DocumentQuestionAnsweringOutput`](./pipelines#module_pipelines.DocumentQuestionAnsweringOutput)> — An object (or array of objects) containing the answer(s).

### FeatureExtractionPipelineCallback

**Parameters**

- `texts` (`string` | `string[]`) — One or several texts (or one list of texts) to get the features of.
- `options` ([`FeatureExtractionPipelineOptions`](./pipelines#module_pipelines.FeatureExtractionPipelineOptions)) _optional_ — The options to use for feature extraction.

**Returns:** `Promise`<[`Tensor`](./utils/tensor#module_utils/tensor.Tensor)> — The features computed by the model.

### FillMaskPipelineCallback

**Parameters**

- `texts` (`string` | `string[]`)
- `options` ([`FillMaskPipelineOptions`](./pipelines#module_pipelines.FillMaskPipelineOptions)) _optional_ — Parameters specific to fill mask pipelines.

**Returns:** `Promise`<[`FillMaskPipelineResult`](./pipelines#module_pipelines.FillMaskPipelineResult)<`string` | `string[]`>>

### ImageClassificationPipelineCallback

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `options` ([`ImageClassificationPipelineOptions`](./pipelines#module_pipelines.ImageClassificationPipelineOptions)) _optional_ — Parameters specific to image classification pipelines.

**Returns:** `Promise`<[`ImageClassificationPipelineResult`](./pipelines#module_pipelines.ImageClassificationPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### ImageFeatureExtractionPipelineCallback

**Parameters**

- `images` ([`ImagePipelineInputs`](./pipelines#module_pipelines.ImagePipelineInputs)) — One or several images (or one list of images) to get the features of.
- `options` ([`ImageFeatureExtractionPipelineOptions`](./pipelines#module_pipelines.ImageFeatureExtractionPipelineOptions)) _optional_ — The options to use for image feature extraction.

**Returns:** `Promise`<[`Tensor`](./utils/tensor#module_utils/tensor.Tensor)> — The image features computed by the model.

### ImageSegmentationPipelineCallback

**Parameters**

- `images` ([`ImagePipelineInputs`](./pipelines#module_pipelines.ImagePipelineInputs)) — The input images.
- `options` ([`ImageSegmentationPipelineOptions`](./pipelines#module_pipelines.ImageSegmentationPipelineOptions)) _optional_ — The options to use for image segmentation.

**Returns:** `Promise`<[`ImageSegmentationOutput`](./pipelines#module_pipelines.ImageSegmentationOutput)> — The annotated segments.

### ImageToImagePipelineCallback

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])

**Returns:** `Promise`<[`ImageToImagePipelineResult`](./pipelines#module_pipelines.ImageToImagePipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### ImageToTextPipelineCallback

**Parameters**

- `texts` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `options` (`Partial`<[`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters)>) _optional_

**Returns:** `Promise`<[`ImageToTextPipelineResult`](./pipelines#module_pipelines.ImageToTextPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### ObjectDetectionPipelineCallback

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `options` ([`ObjectDetectionPipelineOptions`](./pipelines#module_pipelines.ObjectDetectionPipelineOptions)) _optional_ — Parameters specific to object detection pipelines.

**Returns:** `Promise`<[`ObjectDetectionPipelineResult`](./pipelines#module_pipelines.ObjectDetectionPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### QuestionAnsweringPipelineCallback

**Parameters**

- `question` (`string` | `string[]`)
- `context` (`string` | `string[]`)
- `options` (`{ top_k?: number }`) _optional_

**Returns:** `Promise`<[`QuestionAnsweringPipelineResult`](./pipelines#module_pipelines.QuestionAnsweringPipelineResult)<`string` | `string[]`, `{ top_k?: number }`>>

### SummarizationPipelineCallback

**Parameters**

- `texts` (`string` | `string[]`) — One or several articles (or one list of articles) to summarize.
- `options` ([`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters)) _optional_ — Additional keyword arguments to pass along to the generate method of the model.

**Returns:** `Promise`<[`SummarizationOutput`](./pipelines#module_pipelines.SummarizationOutput)>

### TextClassificationPipelineCallback

**Parameters**

- `texts` (`string` | `string[]`)
- `options` ([`TextClassificationPipelineOptions`](./pipelines#module_pipelines.TextClassificationPipelineOptions)) _optional_

**Returns:** `Promise`<[`TextClassificationPipelineResult`](./pipelines#module_pipelines.TextClassificationPipelineResult)<`string` | `string[]`, [`TextClassificationPipelineOptions`](./pipelines#module_pipelines.TextClassificationPipelineOptions)>>

### TextGenerationPipelineCallback

**Parameters**

- `texts` (`string` | [`Chat`](./pipelines#module_pipelines.Chat) | `string[]` | [`Chat`](./pipelines#module_pipelines.Chat)[])
- `options` (`Partial`<[`TextGenerationConfig`](./pipelines#module_pipelines.TextGenerationConfig)>) _optional_

**Returns:** `Promise`<[`TextGenerationResult`](./pipelines#module_pipelines.TextGenerationResult)<`string` | [`Chat`](./pipelines#module_pipelines.Chat) | `string[]` | [`Chat`](./pipelines#module_pipelines.Chat)[]>>

### TextToAudioPipelineCallback

**Parameters**

- `text` (`string` | `string[]`)
- `options` ([`TextToAudioPipelineOptions`](./pipelines#module_pipelines.TextToAudioPipelineOptions)) _optional_ — Parameters specific to text-to-audio pipelines.

**Returns:** `Promise`<[`TextToAudioPipelineResult`](./pipelines#module_pipelines.TextToAudioPipelineResult)<`string` | `string[]`>>

### Text2TextGenerationPipelineCallback

**Parameters**

- `texts` (`string` | `string[]`) — Input text for the encoder.
- `options` (`Partial`<[`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters)>) _optional_ — Additional keyword arguments to pass along to the generate method of the model.

**Returns:** `Promise`<[`Text2TextGenerationOutput`](./pipelines#module_pipelines.Text2TextGenerationOutput)>

### TokenClassificationPipelineCallback

**Parameters**

- `texts` (`string` | `string[]`)
- `options` ([`TokenClassificationPipelineOptions`](./pipelines#module_pipelines.TokenClassificationPipelineOptions)) _optional_

**Returns:** `Promise`<[`TokenClassificationOutput`](./pipelines#module_pipelines.TokenClassificationOutput)<[`TokenClassificationPipelineOptions`](./pipelines#module_pipelines.TokenClassificationPipelineOptions)>[] | [`TokenClassificationOutput`](./pipelines#module_pipelines.TokenClassificationOutput)<[`TokenClassificationPipelineOptions`](./pipelines#module_pipelines.TokenClassificationPipelineOptions)>>

### TranslationPipelineCallback

**Parameters**

- `texts` (`string` | `string[]`) — Texts to be translated.
- `options` ([`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters)) _optional_ — Additional keyword arguments to pass along to the generate method of the model.

**Returns:** `Promise`<[`TranslationOutput`](./pipelines#module_pipelines.TranslationOutput)>

### ZeroShotAudioClassificationPipelineCallback

**Parameters**

- `audio` ([`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[])
- `candidate_labels` (`string[]`)
- `options` ([`ZeroShotAudioClassificationPipelineOptions`](./pipelines#module_pipelines.ZeroShotAudioClassificationPipelineOptions)) _optional_ — Parameters specific to zero-shot audio classification pipelines.

**Returns:** `Promise`<[`ZeroShotAudioClassificationPipelineResult`](./pipelines#module_pipelines.ZeroShotAudioClassificationPipelineResult)<[`AudioInput`](./pipelines#module_pipelines.AudioInput) | [`AudioInput`](./pipelines#module_pipelines.AudioInput)[]>>

### ZeroShotClassificationPipelineCallback

**Parameters**

- `texts` (`string` | `string[]`)
- `candidate_labels` (`string` | `string[]`)
- `options` ([`ZeroShotClassificationPipelineOptions`](./pipelines#module_pipelines.ZeroShotClassificationPipelineOptions)) _optional_ — Parameters specific to zero-shot classification pipelines.

**Returns:** `Promise`<[`ZeroShotClassificationPipelineResult`](./pipelines#module_pipelines.ZeroShotClassificationPipelineResult)<`string` | `string[]`>>

### ZeroShotImageClassificationPipelineCallback

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `candidate_labels` (`string[]`)
- `options` ([`ZeroShotImageClassificationPipelineOptions`](./pipelines#module_pipelines.ZeroShotImageClassificationPipelineOptions)) _optional_ — Parameters specific to zero-shot image classification pipelines.

**Returns:** `Promise`<[`ZeroShotImageClassificationPipelineResult`](./pipelines#module_pipelines.ZeroShotImageClassificationPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>

### ZeroShotObjectDetectionPipelineCallback

**Parameters**

- `images` ([`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[])
- `candidate_labels` (`string[]`)
- `options` ([`ZeroShotObjectDetectionPipelineOptions`](./pipelines#module_pipelines.ZeroShotObjectDetectionPipelineOptions)) _optional_ — Parameters specific to zero-shot object detection pipelines.

**Returns:** `Promise`<[`ZeroShotObjectDetectionPipelineResult`](./pipelines#module_pipelines.ZeroShotObjectDetectionPipelineResult)<[`ImageInput`](./pipelines#module_pipelines.ImageInput) | [`ImageInput`](./pipelines#module_pipelines.ImageInput)[]>>
