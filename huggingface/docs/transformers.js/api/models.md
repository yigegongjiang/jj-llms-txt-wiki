# models

Definitions of all models available in Transformers.js.

We also provide other `AutoModel`s (listed below), which you can use in the same way as the Python library. For example:

**Example:** Load and run an `AutoModel`.
```javascript
import { AutoModel, AutoTokenizer } from '@huggingface/transformers';

const tokenizer = await AutoTokenizer.from_pretrained('Xenova/bert-base-uncased');
const model = await AutoModel.from_pretrained('Xenova/bert-base-uncased');

const inputs = await tokenizer('I love transformers!');
const { logits } = await model(inputs);
// Tensor {
//     data: Float32Array(183132) [-7.117443084716797, -7.107812881469727, -7.092104911804199, ...]
//     dims: (3) [1, 6, 30522],
//     type: "float32",
//     size: 183132,
// }
```

**Example:** Load and run an `AutoModelForSeq2SeqLM`.
```javascript
import { AutoModelForSeq2SeqLM, AutoTokenizer } from '@huggingface/transformers';

const tokenizer = await AutoTokenizer.from_pretrained('Xenova/t5-small');
const model = await AutoModelForSeq2SeqLM.from_pretrained('Xenova/t5-small');

const { input_ids } = await tokenizer('translate English to German: I love transformers!');
const outputs = await model.generate(input_ids);
const decoded = tokenizer.decode(outputs[0], { skip_special_tokens: true });
// 'Ich liebe Transformatoren!'
```

## On this page

**Classes** — [`AutoModel`](#module_models.AutoModel) · [`AutoModelForSequenceClassification`](#module_models.AutoModelForSequenceClassification) · [`AutoModelForTokenClassification`](#module_models.AutoModelForTokenClassification) · [`AutoModelForSeq2SeqLM`](#module_models.AutoModelForSeq2SeqLM) · [`AutoModelForSpeechSeq2Seq`](#module_models.AutoModelForSpeechSeq2Seq) · [`AutoModelForTextToSpectrogram`](#module_models.AutoModelForTextToSpectrogram) · [`AutoModelForTextToWaveform`](#module_models.AutoModelForTextToWaveform) · [`AutoModelForCausalLM`](#module_models.AutoModelForCausalLM) · [`AutoModelForMaskedLM`](#module_models.AutoModelForMaskedLM) · [`AutoModelForQuestionAnswering`](#module_models.AutoModelForQuestionAnswering) · [`AutoModelForVision2Seq`](#module_models.AutoModelForVision2Seq) · [`AutoModelForImageClassification`](#module_models.AutoModelForImageClassification) · [`AutoModelForImageSegmentation`](#module_models.AutoModelForImageSegmentation) · [`AutoModelForSemanticSegmentation`](#module_models.AutoModelForSemanticSegmentation) · [`AutoModelForUniversalSegmentation`](#module_models.AutoModelForUniversalSegmentation) · [`AutoModelForObjectDetection`](#module_models.AutoModelForObjectDetection) · [`AutoModelForZeroShotObjectDetection`](#module_models.AutoModelForZeroShotObjectDetection) · [`AutoModelForMaskGeneration`](#module_models.AutoModelForMaskGeneration) · [`AutoModelForCTC`](#module_models.AutoModelForCTC) · [`AutoModelForAudioClassification`](#module_models.AutoModelForAudioClassification) · [`AutoModelForXVector`](#module_models.AutoModelForXVector) · [`AutoModelForAudioFrameClassification`](#module_models.AutoModelForAudioFrameClassification) · [`AutoModelForDocumentQuestionAnswering`](#module_models.AutoModelForDocumentQuestionAnswering) · [`AutoModelForImageMatting`](#module_models.AutoModelForImageMatting) · [`AutoModelForImageToImage`](#module_models.AutoModelForImageToImage) · [`AutoModelForDepthEstimation`](#module_models.AutoModelForDepthEstimation) · [`AutoModelForNormalEstimation`](#module_models.AutoModelForNormalEstimation) · [`AutoModelForPoseEstimation`](#module_models.AutoModelForPoseEstimation) · [`AutoModelForImageFeatureExtraction`](#module_models.AutoModelForImageFeatureExtraction) · [`AutoModelForImageTextToText`](#module_models.AutoModelForImageTextToText) · [`AutoModelForAudioTextToText`](#module_models.AutoModelForAudioTextToText) · [`PreTrainedModel`](#module_models.PreTrainedModel)

## Classes

### AutoModel

Helper class which is used to instantiate pretrained models with the `from_pretrained` function.

```javascript
import { AutoModel } from '@huggingface/transformers';

const model = await AutoModel.from_pretrained('Xenova/bert-base-uncased');
```

### AutoModelForSequenceClassification

Helper class which is used to instantiate pretrained sequence classification models with the `from_pretrained` function.

```javascript
import { AutoModelForSequenceClassification } from '@huggingface/transformers';

const model = await AutoModelForSequenceClassification.from_pretrained('Xenova/distilbert-base-uncased-finetuned-sst-2-english');
```

### AutoModelForTokenClassification

Helper class which is used to instantiate pretrained token classification models with the `from_pretrained` function.

```javascript
import { AutoModelForTokenClassification } from '@huggingface/transformers';

const model = await AutoModelForTokenClassification.from_pretrained('Xenova/distilbert-base-multilingual-cased-ner-hrl');
```

### AutoModelForSeq2SeqLM

Helper class which is used to instantiate pretrained sequence-to-sequence models with the `from_pretrained` function.

```javascript
import { AutoModelForSeq2SeqLM } from '@huggingface/transformers';

const model = await AutoModelForSeq2SeqLM.from_pretrained('Xenova/t5-small');
```

### AutoModelForSpeechSeq2Seq

Helper class which is used to instantiate pretrained sequence-to-sequence speech-to-text models with the `from_pretrained` function.

```javascript
import { AutoModelForSpeechSeq2Seq } from '@huggingface/transformers';

const model = await AutoModelForSpeechSeq2Seq.from_pretrained('onnx-community/whisper-tiny.en');
```

### AutoModelForTextToSpectrogram

Helper class which is used to instantiate pretrained sequence-to-sequence text-to-spectrogram models with the `from_pretrained` function.

```javascript
import { AutoModelForTextToSpectrogram } from '@huggingface/transformers';

const model = await AutoModelForTextToSpectrogram.from_pretrained('Xenova/speecht5_tts');
```

### AutoModelForTextToWaveform

Helper class which is used to instantiate pretrained text-to-waveform models with the `from_pretrained` function.

```javascript
import { AutoModelForTextToWaveform } from '@huggingface/transformers';

const model = await AutoModelForTextToWaveform.from_pretrained('Xenova/mms-tts-eng');
```

### AutoModelForCausalLM

Helper class which is used to instantiate pretrained causal language models with the `from_pretrained` function.

```javascript
import { AutoModelForCausalLM } from '@huggingface/transformers';

const model = await AutoModelForCausalLM.from_pretrained('Xenova/gpt2');
```

### AutoModelForMaskedLM

Helper class which is used to instantiate pretrained masked language models with the `from_pretrained` function.

```javascript
import { AutoModelForMaskedLM } from '@huggingface/transformers';

const model = await AutoModelForMaskedLM.from_pretrained('Xenova/bert-base-uncased');
```

### AutoModelForQuestionAnswering

Helper class which is used to instantiate pretrained question answering models with the `from_pretrained` function.

```javascript
import { AutoModelForQuestionAnswering } from '@huggingface/transformers';

const model = await AutoModelForQuestionAnswering.from_pretrained('Xenova/distilbert-base-cased-distilled-squad');
```

### AutoModelForVision2Seq

Helper class which is used to instantiate pretrained vision-to-sequence models with the `from_pretrained` function.

```javascript
import { AutoModelForVision2Seq } from '@huggingface/transformers';

const model = await AutoModelForVision2Seq.from_pretrained('Xenova/vit-gpt2-image-captioning');
```

### AutoModelForImageClassification

Helper class which is used to instantiate pretrained image classification models with the `from_pretrained` function.

```javascript
import { AutoModelForImageClassification } from '@huggingface/transformers';

const model = await AutoModelForImageClassification.from_pretrained('Xenova/vit-base-patch16-224');
```

### AutoModelForImageSegmentation

Helper class which is used to instantiate pretrained image segmentation models with the `from_pretrained` function.

```javascript
import { AutoModelForImageSegmentation } from '@huggingface/transformers';

const model = await AutoModelForImageSegmentation.from_pretrained('Xenova/detr-resnet-50-panoptic');
```

### AutoModelForSemanticSegmentation

Helper class which is used to instantiate pretrained image segmentation models with the `from_pretrained` function.

```javascript
import { AutoModelForSemanticSegmentation } from '@huggingface/transformers';

const model = await AutoModelForSemanticSegmentation.from_pretrained('Xenova/segformer-b0-finetuned-ade-512-512');
```

### AutoModelForUniversalSegmentation

Helper class which is used to instantiate pretrained universal image segmentation models with the `from_pretrained` function.

```javascript
import { AutoModelForUniversalSegmentation } from '@huggingface/transformers';

const model = await AutoModelForUniversalSegmentation.from_pretrained('Xenova/detr-resnet-50-panoptic');
```

### AutoModelForObjectDetection

Helper class which is used to instantiate pretrained object detection models with the `from_pretrained` function.

```javascript
import { AutoModelForObjectDetection } from '@huggingface/transformers';

const model = await AutoModelForObjectDetection.from_pretrained('Xenova/detr-resnet-50');
```

### AutoModelForZeroShotObjectDetection

Helper class which is used to instantiate pretrained zero-shot object detection models with the `from_pretrained` function.

```javascript
import { AutoModelForZeroShotObjectDetection } from '@huggingface/transformers';

const model = await AutoModelForZeroShotObjectDetection.from_pretrained('Xenova/owlvit-base-patch32');
```

### AutoModelForMaskGeneration

Helper class which is used to instantiate pretrained mask generation models with the `from_pretrained` function.

```javascript
import { AutoModelForMaskGeneration } from '@huggingface/transformers';

const model = await AutoModelForMaskGeneration.from_pretrained('Xenova/slimsam-77-uniform');
```

### AutoModelForCTC

Helper class which is used to instantiate pretrained connectionist temporal classification (CTC) models with the `from_pretrained` function.

```javascript
import { AutoModelForCTC } from '@huggingface/transformers';

const model = await AutoModelForCTC.from_pretrained('Xenova/wav2vec2-base-960h');
```

### AutoModelForAudioClassification

Helper class which is used to instantiate pretrained audio classification models with the `from_pretrained` function.

```javascript
import { AutoModelForAudioClassification } from '@huggingface/transformers';

const model = await AutoModelForAudioClassification.from_pretrained('Xenova/wav2vec2-base-superb-ks');
```

### AutoModelForXVector

Helper class which is used to instantiate pretrained speaker embedding models (X-Vector) with the `from_pretrained` function.

```javascript
import { AutoModelForXVector } from '@huggingface/transformers';

const model = await AutoModelForXVector.from_pretrained('Xenova/wavlm-base-plus-sv');
```

### AutoModelForAudioFrameClassification

Helper class which is used to instantiate pretrained audio frame (token) classification models with the `from_pretrained` function.

```javascript
import { AutoModelForAudioFrameClassification } from '@huggingface/transformers';

const model = await AutoModelForAudioFrameClassification.from_pretrained('onnx-community/pyannote-segmentation-3.0');
```

### AutoModelForDocumentQuestionAnswering

Helper class which is used to instantiate pretrained document question answering models with the `from_pretrained` function.

```javascript
import { AutoModelForDocumentQuestionAnswering } from '@huggingface/transformers';

const model = await AutoModelForDocumentQuestionAnswering.from_pretrained('Xenova/donut-base-finetuned-docvqa');
```

### AutoModelForImageMatting

Helper class which is used to instantiate pretrained image matting models with the `from_pretrained` function.

```javascript
import { AutoModelForImageMatting } from '@huggingface/transformers';

const model = await AutoModelForImageMatting.from_pretrained('Xenova/vitmatte-small-composition-1k');
```

### AutoModelForImageToImage

Helper class which is used to instantiate pretrained image-to-image models with the `from_pretrained` function.

```javascript
import { AutoModelForImageToImage } from '@huggingface/transformers';

const model = await AutoModelForImageToImage.from_pretrained('Xenova/swin2SR-classical-sr-x2-64');
```

### AutoModelForDepthEstimation

Helper class which is used to instantiate pretrained depth estimation models with the `from_pretrained` function.

```javascript
import { AutoModelForDepthEstimation } from '@huggingface/transformers';

const model = await AutoModelForDepthEstimation.from_pretrained('onnx-community/depth-anything-v2-small-ONNX');
```

### AutoModelForNormalEstimation

Helper class which is used to instantiate pretrained surface-normal estimation models with the `from_pretrained` function.

```javascript
import { AutoModelForNormalEstimation } from '@huggingface/transformers';

const model = await AutoModelForNormalEstimation.from_pretrained('onnx-community/sapiens-normal-0.3b');
```

### AutoModelForPoseEstimation

Helper class which is used to instantiate pretrained pose estimation models with the `from_pretrained` function.

```javascript
import { AutoModelForPoseEstimation } from '@huggingface/transformers';

const model = await AutoModelForPoseEstimation.from_pretrained('onnx-community/vitpose-base-simple');
```

### AutoModelForImageFeatureExtraction

Helper class which is used to instantiate pretrained image feature extraction models with the `from_pretrained` function.

```javascript
import { AutoModelForImageFeatureExtraction } from '@huggingface/transformers';

const model = await AutoModelForImageFeatureExtraction.from_pretrained('onnx-community/dinov3-vits16-pretrain-lvd1689m-ONNX');
```

### AutoModelForImageTextToText

Helper class which is used to instantiate pretrained vision-language models that map images and text to text
(image+text-to-text) with the `from_pretrained` function.

```javascript
import { AutoModelForImageTextToText } from '@huggingface/transformers';

const model = await AutoModelForImageTextToText.from_pretrained('onnx-community/LFM2.5-VL-450M-ONNX');
```

### AutoModelForAudioTextToText

Helper class which is used to instantiate pretrained audio-language models that map audio and text to text
(audio+text-to-text) with the `from_pretrained` function.

```javascript
import { AutoModelForAudioTextToText } from '@huggingface/transformers';

const model = await AutoModelForAudioTextToText.from_pretrained('onnx-community/Voxtral-Mini-4B-Realtime-2602-ONNX');
```

### PreTrainedModel

A base class for pretrained models that provides the model configuration and inference sessions.

#### `PreTrainedModel(model_inputs)`

Runs the model with the provided inputs.

**Parameters**

- `model_inputs` (`Object`) — Object containing input tensors.

**Returns:** `Promise`<`Object`> — Object containing output tensors.

#### `PreTrainedModel.constructor(config, sessions, configs)`

Create a model from configuration and inference sessions.

**Parameters**

- `config` ([`PretrainedConfig`](./configs#module_configs.PretrainedConfig)) — The model configuration.
- `sessions` (`Record`<`string`, `any`>) — The inference sessions for the model.
- `configs` (`Record`<`string`, `Object`>) — Additional configuration files (e.g., generation_config.json).

#### `PreTrainedModel.dispose()`

Disposes of all the ONNX sessions that were created during inference.

**Returns:** `Promise`<`void[]`> — Resolves after each session has been released.

#### `PreTrainedModel.from_pretrained(pretrained_model_name_or_path, options)`

Instantiate one of the model classes of the library from a pretrained model.

The model class to instantiate is selected based on the `model_type` property of the config object
(either passed as an argument or loaded from `pretrained_model_name_or_path` if possible)

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The name or path of the pretrained model. Can be either:
  - A string, the *model ID* of a pretrained model hosted inside a model repo on huggingface.co.
  Valid model IDs can be located at the root level, like `bert-base-uncased`, or namespaced under a
  user or organization name, like `dbmdz/bert-base-german-cased`.
  - A path to a *directory* containing model weights, e.g., `./my_model_directory/`.
- `options` ([`PretrainedModelOptions`](./utils/hub#module_utils/hub.PretrainedModelOptions)) — Additional options for loading the model.

**Returns:** `Promise`<[`PreTrainedModel`](./models#module_models.PreTrainedModel)> — A model instance with ready inference sessions.

#### `PreTrainedModel.forward(model_inputs)`

Run the model's forward pass.

**Parameters**

- `model_inputs` (`Object`) — The input data to the model in the format specified in the ONNX model.

**Returns:** `Promise`<`Object`> — The output data from the model in the format specified in the ONNX model.

#### `PreTrainedModel.generation_config` : [`GenerationConfig`](./generation/configuration_utils#module_generation/configuration_utils.GenerationConfig) | `null`

Get the model's generation config, if it exists.

#### `PreTrainedModel.generate(options)`

Generate token sequences with a language-modeling head.

**Parameters**

- `options` ([`GenerationFunctionParameters`](./generation/parameters#module_generation/parameters.GenerationFunctionParameters))

**Returns:** `Promise`<`ModelOutput` | [`Tensor`](./utils/tensor#module_utils/tensor.Tensor)> — The output of the model, which can contain the generated token ids, attentions, and scores.
