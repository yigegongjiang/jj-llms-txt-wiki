# models

Definitions of all models available in Transformers.js.

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

We also provide other `AutoModel`s (listed below), which you can use in the same way as the Python library. For example:

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

* [models](#module_models)
    * _static_
        * [.AutoModel](#module_models.AutoModel)
            * [`new AutoModel()`](#new_module_models.AutoModel_new)
            * [`.MODEL_CLASS_MAPPINGS`](#module_models.AutoModel+MODEL_CLASS_MAPPINGS) : Array.&lt;Map&gt;
        * [.AutoModelForSequenceClassification](#module_models.AutoModelForSequenceClassification)
            * [`new AutoModelForSequenceClassification()`](#new_module_models.AutoModelForSequenceClassification_new)
        * [.AutoModelForTokenClassification](#module_models.AutoModelForTokenClassification)
            * [`new AutoModelForTokenClassification()`](#new_module_models.AutoModelForTokenClassification_new)
        * [.AutoModelForSeq2SeqLM](#module_models.AutoModelForSeq2SeqLM)
            * [`new AutoModelForSeq2SeqLM()`](#new_module_models.AutoModelForSeq2SeqLM_new)
        * [.AutoModelForSpeechSeq2Seq](#module_models.AutoModelForSpeechSeq2Seq)
            * [`new AutoModelForSpeechSeq2Seq()`](#new_module_models.AutoModelForSpeechSeq2Seq_new)
        * [.AutoModelForTextToSpectrogram](#module_models.AutoModelForTextToSpectrogram)
            * [`new AutoModelForTextToSpectrogram()`](#new_module_models.AutoModelForTextToSpectrogram_new)
        * [.AutoModelForTextToWaveform](#module_models.AutoModelForTextToWaveform)
            * [`new AutoModelForTextToWaveform()`](#new_module_models.AutoModelForTextToWaveform_new)
        * [.AutoModelForCausalLM](#module_models.AutoModelForCausalLM)
            * [`new AutoModelForCausalLM()`](#new_module_models.AutoModelForCausalLM_new)
        * [.AutoModelForMaskedLM](#module_models.AutoModelForMaskedLM)
            * [`new AutoModelForMaskedLM()`](#new_module_models.AutoModelForMaskedLM_new)
        * [.AutoModelForQuestionAnswering](#module_models.AutoModelForQuestionAnswering)
            * [`new AutoModelForQuestionAnswering()`](#new_module_models.AutoModelForQuestionAnswering_new)
        * [.AutoModelForVision2Seq](#module_models.AutoModelForVision2Seq)
            * [`new AutoModelForVision2Seq()`](#new_module_models.AutoModelForVision2Seq_new)
        * [.AutoModelForImageClassification](#module_models.AutoModelForImageClassification)
            * [`new AutoModelForImageClassification()`](#new_module_models.AutoModelForImageClassification_new)
        * [.AutoModelForImageSegmentation](#module_models.AutoModelForImageSegmentation)
            * [`new AutoModelForImageSegmentation()`](#new_module_models.AutoModelForImageSegmentation_new)
        * [.AutoModelForSemanticSegmentation](#module_models.AutoModelForSemanticSegmentation)
            * [`new AutoModelForSemanticSegmentation()`](#new_module_models.AutoModelForSemanticSegmentation_new)
        * [.AutoModelForUniversalSegmentation](#module_models.AutoModelForUniversalSegmentation)
            * [`new AutoModelForUniversalSegmentation()`](#new_module_models.AutoModelForUniversalSegmentation_new)
        * [.AutoModelForObjectDetection](#module_models.AutoModelForObjectDetection)
            * [`new AutoModelForObjectDetection()`](#new_module_models.AutoModelForObjectDetection_new)
        * [.AutoModelForMaskGeneration](#module_models.AutoModelForMaskGeneration)
            * [`new AutoModelForMaskGeneration()`](#new_module_models.AutoModelForMaskGeneration_new)
    * _inner_
        * [~PretrainedMixin](#module_models..PretrainedMixin)
            * _instance_
                * [`.MODEL_CLASS_MAPPINGS`](#module_models..PretrainedMixin+MODEL_CLASS_MAPPINGS) : Array.&lt;Map&gt;
                * [`.BASE_IF_FAIL`](#module_models..PretrainedMixin+BASE_IF_FAIL)
            * _static_
                * [`.supports(model_type)`](#module_models..PretrainedMixin.supports) ⇒ boolean
                * [`.from_pretrained()`](#module_models..PretrainedMixin.from_pretrained) : Object.from_pretrained

* * *

## models.AutoModel

Helper class which is used to instantiate pretrained models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* [.AutoModel](#module_models.AutoModel)
    * [`new AutoModel()`](#new_module_models.AutoModel_new)
    * [`.MODEL_CLASS_MAPPINGS`](#module_models.AutoModel+MODEL_CLASS_MAPPINGS) : Array.&lt;Map&gt;

* * *

### `new AutoModel()`

**Example**  
```js
const model = await AutoModel.from_pretrained('Xenova/bert-base-uncased');
```

* * *

### `autoModel.MODEL_CLASS_MAPPINGS` : Array.&lt;Map&gt;

**Kind**: instance property of [AutoModel](#module_models.AutoModel)  

* * *

## models.AutoModelForSequenceClassification

Helper class which is used to instantiate pretrained sequence classification models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForSequenceClassification()`

**Example**  
```js
const model = await AutoModelForSequenceClassification.from_pretrained('Xenova/distilbert-base-uncased-finetuned-sst-2-english');
```

* * *

## models.AutoModelForTokenClassification

Helper class which is used to instantiate pretrained token classification models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForTokenClassification()`

**Example**  
```js
const model = await AutoModelForTokenClassification.from_pretrained('Xenova/distilbert-base-multilingual-cased-ner-hrl');
```

* * *

## models.AutoModelForSeq2SeqLM

Helper class which is used to instantiate pretrained sequence-to-sequence models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForSeq2SeqLM()`

**Example**  
```js
const model = await AutoModelForSeq2SeqLM.from_pretrained('Xenova/t5-small');
```

* * *

## models.AutoModelForSpeechSeq2Seq

Helper class which is used to instantiate pretrained sequence-to-sequence speech-to-text models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForSpeechSeq2Seq()`

**Example**  
```js
const model = await AutoModelForSpeechSeq2Seq.from_pretrained('openai/whisper-tiny.en');
```

* * *

## models.AutoModelForTextToSpectrogram

Helper class which is used to instantiate pretrained sequence-to-sequence text-to-spectrogram models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForTextToSpectrogram()`

**Example**  
```js
const model = await AutoModelForTextToSpectrogram.from_pretrained('microsoft/speecht5_tts');
```

* * *

## models.AutoModelForTextToWaveform

Helper class which is used to instantiate pretrained text-to-waveform models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForTextToWaveform()`

**Example**  
```js
const model = await AutoModelForTextToSpectrogram.from_pretrained('facebook/mms-tts-eng');
```

* * *

## models.AutoModelForCausalLM

Helper class which is used to instantiate pretrained causal language models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForCausalLM()`

**Example**  
```js
const model = await AutoModelForCausalLM.from_pretrained('Xenova/gpt2');
```

* * *

## models.AutoModelForMaskedLM

Helper class which is used to instantiate pretrained masked language models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForMaskedLM()`

**Example**  
```js
const model = await AutoModelForMaskedLM.from_pretrained('Xenova/bert-base-uncased');
```

* * *

## models.AutoModelForQuestionAnswering

Helper class which is used to instantiate pretrained question answering models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForQuestionAnswering()`

**Example**  
```js
const model = await AutoModelForQuestionAnswering.from_pretrained('Xenova/distilbert-base-cased-distilled-squad');
```

* * *

## models.AutoModelForVision2Seq

Helper class which is used to instantiate pretrained vision-to-sequence models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForVision2Seq()`

**Example**  
```js
const model = await AutoModelForVision2Seq.from_pretrained('Xenova/vit-gpt2-image-captioning');
```

* * *

## models.AutoModelForImageClassification

Helper class which is used to instantiate pretrained image classification models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForImageClassification()`

**Example**  
```js
const model = await AutoModelForImageClassification.from_pretrained('Xenova/vit-base-patch16-224');
```

* * *

## models.AutoModelForImageSegmentation

Helper class which is used to instantiate pretrained image segmentation models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForImageSegmentation()`

**Example**  
```js
const model = await AutoModelForImageSegmentation.from_pretrained('Xenova/detr-resnet-50-panoptic');
```

* * *

## models.AutoModelForSemanticSegmentation

Helper class which is used to instantiate pretrained image segmentation models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForSemanticSegmentation()`

**Example**  
```js
const model = await AutoModelForSemanticSegmentation.from_pretrained('nvidia/segformer-b3-finetuned-cityscapes-1024-1024');
```

* * *

## models.AutoModelForUniversalSegmentation

Helper class which is used to instantiate pretrained universal image segmentation models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForUniversalSegmentation()`

**Example**  
```js
const model = await AutoModelForUniversalSegmentation.from_pretrained('hf-internal-testing/tiny-random-MaskFormerForInstanceSegmentation');
```

* * *

## models.AutoModelForObjectDetection

Helper class which is used to instantiate pretrained object detection models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForObjectDetection()`

**Example**  
```js
const model = await AutoModelForObjectDetection.from_pretrained('Xenova/detr-resnet-50');
```

* * *

## models.AutoModelForMaskGeneration

Helper class which is used to instantiate pretrained mask generation models with the `from_pretrained` function.
The chosen model class is determined by the type specified in the model config.

**Kind**: static class of [models](#module_models)  

* * *

### `new AutoModelForMaskGeneration()`

**Example**  
```js
const model = await AutoModelForMaskGeneration.from_pretrained('Xenova/sam-vit-base');
```

* * *

## models~PretrainedMixin

Base class of all AutoModels. Contains the `from_pretrained` function
which is used to instantiate pretrained models.

**Kind**: inner class of [models](#module_models)  

* [~PretrainedMixin](#module_models..PretrainedMixin)
    * _instance_
        * [`.MODEL_CLASS_MAPPINGS`](#module_models..PretrainedMixin+MODEL_CLASS_MAPPINGS) : Array.&lt;Map&gt;
        * [`.BASE_IF_FAIL`](#module_models..PretrainedMixin+BASE_IF_FAIL)
    * _static_
        * [`.supports(model_type)`](#module_models..PretrainedMixin.supports) ⇒ boolean
        * [`.from_pretrained()`](#module_models..PretrainedMixin.from_pretrained) : Object.from_pretrained

* * *

### `pretrainedMixin.MODEL_CLASS_MAPPINGS` : Array.&lt;Map&gt;

Mapping from model type to model class.

**Kind**: instance property of [PretrainedMixin](#module_models..PretrainedMixin)  

* * *

### `pretrainedMixin.BASE_IF_FAIL`

Whether to attempt to instantiate the base class (`PretrainedModel`) if
the model type is not found in the mapping.

**Kind**: instance property of [PretrainedMixin](#module_models..PretrainedMixin)  

* * *

### `PretrainedMixin.supports(model_type)` ⇒ boolean

Check whether this AutoModel class supports a given model type.

**Kind**: static method of [PretrainedMixin](#module_models..PretrainedMixin)  
**Returns**: boolean - Whether this class can handle the given model type.  

  
    
      ParamTypeDescription
    
  
  

    model_typestringThe model type from config (e.g., &#39;bert&#39;, &#39;whisper&#39;).

      

* * *

### `PretrainedMixin.from_pretrained()` : Object.from_pretrained

**Kind**: static method of [PretrainedMixin](#module_models..PretrainedMixin)  

* * *
