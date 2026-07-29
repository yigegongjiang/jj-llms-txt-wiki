# processors

Processors are used to prepare inputs (e.g., text, image or audio) for a model.

**Example:** Using a `WhisperProcessor` to prepare an audio input for a model.
```javascript
import { AutoProcessor, read_audio } from '@huggingface/transformers';

const processor = await AutoProcessor.from_pretrained('openai/whisper-tiny.en');
const audio = await read_audio('https://huggingface.co/datasets/Narsil/asr_dummy/resolve/main/mlk.flac', 16000);
const { input_features } = await processor(audio);
// Tensor {
//   data: Float32Array(240000) [0.4752984642982483, 0.5597258806228638, 0.56434166431427, ...],
//   dims: [1, 80, 3000],
//   type: 'float32',
//   size: 240000,
// }
```

* [processors](#module_processors)
    * _static_
        * [.Processor](#module_processors.Processor)
            * [`new Processor(config, components, chat_template)`](#new_module_processors.Processor_new)
            * _instance_
                * [`.image_processor`](#module_processors.Processor+image_processor) ⇒ ImageProcessor | undefined
                * [`.tokenizer`](#module_processors.Processor+tokenizer) ⇒ PreTrainedTokenizer | undefined
                * [`.feature_extractor`](#module_processors.Processor+feature_extractor) ⇒ [FeatureExtractor](#FeatureExtractor) | undefined
                * [`.apply_chat_template(messages, options)`](#module_processors.Processor+apply_chat_template) ⇒ ReturnType.&lt;PreTrainedTokenizer&gt;
                * [`.batch_decode(...args)`](#module_processors.Processor+batch_decode) ⇒ ReturnType.&lt;PreTrainedTokenizer&gt;
                * [`.decode(...args)`](#module_processors.Processor+decode) ⇒ ReturnType.&lt;PreTrainedTokenizer&gt;
                * [`._call(input, ...args)`](#module_processors.Processor+_call) ⇒ Promise.&lt;any&gt;
            * _static_
                * [`.from_pretrained(pretrained_model_name_or_path, options)`](#module_processors.Processor.from_pretrained) ⇒ Promise.&lt;Processor&gt;
    * _inner_
        * [`~PreTrainedTokenizer`](#module_processors..PreTrainedTokenizer) : Object

* * *

## processors.Processor

Represents a Processor that extracts features from an input.

**Kind**: static class of [processors](#module_processors)  

* [.Processor](#module_processors.Processor)
    * [`new Processor(config, components, chat_template)`](#new_module_processors.Processor_new)
    * _instance_
        * [`.image_processor`](#module_processors.Processor+image_processor) ⇒ ImageProcessor | undefined
        * [`.tokenizer`](#module_processors.Processor+tokenizer) ⇒ PreTrainedTokenizer | undefined
        * [`.feature_extractor`](#module_processors.Processor+feature_extractor) ⇒ [FeatureExtractor](#FeatureExtractor) | undefined
        * [`.apply_chat_template(messages, options)`](#module_processors.Processor+apply_chat_template) ⇒ ReturnType.&lt;PreTrainedTokenizer&gt;
        * [`.batch_decode(...args)`](#module_processors.Processor+batch_decode) ⇒ ReturnType.&lt;PreTrainedTokenizer&gt;
        * [`.decode(...args)`](#module_processors.Processor+decode) ⇒ ReturnType.&lt;PreTrainedTokenizer&gt;
        * [`._call(input, ...args)`](#module_processors.Processor+_call) ⇒ Promise.&lt;any&gt;
    * _static_
        * [`.from_pretrained(pretrained_model_name_or_path, options)`](#module_processors.Processor.from_pretrained) ⇒ Promise.&lt;Processor&gt;

* * *

### `new Processor(config, components, chat_template)`

Creates a new Processor with the given components

  
    
      ParamType
    
  
  

    configObject
    
    componentsRecord.&lt;string, Object&gt;
    
    chat_templatestring
      

* * *

### `processor.image_processor` ⇒ ImageProcessor | undefined

**Kind**: instance property of [Processor](#module_processors.Processor)  
**Returns**: ImageProcessor | undefined - The image processor of the processor, if it exists.  

* * *

### `processor.tokenizer` ⇒ PreTrainedTokenizer | undefined

**Kind**: instance property of [Processor](#module_processors.Processor)  
**Returns**: PreTrainedTokenizer | undefined - The tokenizer of the processor, if it exists.  

* * *

### `processor.feature_extractor` ⇒ [FeatureExtractor](#FeatureExtractor) | undefined

**Kind**: instance property of [Processor](#module_processors.Processor)  
**Returns**: [FeatureExtractor](#FeatureExtractor) | undefined - The feature extractor of the processor, if it exists.  

* * *

### `processor.apply_chat_template(messages, options)` ⇒ ReturnType.&lt;PreTrainedTokenizer&gt;

**Kind**: instance method of [Processor](#module_processors.Processor)  

  
    
      ParamType
    
  
  

    messagesParameters
    
    optionsParameters
      

* * *

### `processor.batch_decode(...args)` ⇒ ReturnType.&lt;PreTrainedTokenizer&gt;

**Kind**: instance method of [Processor](#module_processors.Processor)  

  
    
      ParamType
    
  
  

    ...argsParameters.&lt;PreTrainedTokenizer&gt;
      

* * *

### `processor.decode(...args)` ⇒ ReturnType.&lt;PreTrainedTokenizer&gt;

**Kind**: instance method of [Processor](#module_processors.Processor)  

  
    
      ParamType
    
  
  

    ...argsParameters.&lt;PreTrainedTokenizer&gt;
      

* * *

### `processor._call(input, ...args)` ⇒ Promise.&lt;any&gt;

Calls the feature_extractor function with the given input.

**Kind**: instance method of [Processor](#module_processors.Processor)  
**Returns**: Promise.&lt;any&gt; - A Promise that resolves with the extracted features.  

  
    
      ParamTypeDescription
    
  
  

    inputanyThe input to extract features from.

    
    ...argsanyAdditional arguments.

      

* * *

### `Processor.from_pretrained(pretrained_model_name_or_path, options)` ⇒ Promise.&lt;Processor&gt;

Instantiate one of the processor classes of the library from a pretrained model.

The processor class to instantiate is selected based on the `image_processor_type` (or `feature_extractor_type`; legacy)
property of the config object (either passed as an argument or loaded from `pretrained_model_name_or_path` if possible)

**Kind**: static method of [Processor](#module_processors.Processor)  
**Returns**: Promise.&lt;Processor&gt; - A new instance of the Processor class.  

  
    
      ParamTypeDescription
    
  
  

    pretrained_model_name_or_pathstringThe name or path of the pretrained model. Can be either:

A string, the model id of a pretrained processor hosted inside a model repo on huggingface.co.
Valid model ids can be located at the root-level, like bert-base-uncased, or namespaced under a
user or organization name, like dbmdz/bert-base-german-cased.
A path to a directory containing processor files, e.g., ./my_model_directory/.

    
    optionsPretrainedProcessorOptionsAdditional options for loading the processor.

      

* * *

## `processors~PreTrainedTokenizer` : Object

Additional processor-specific properties.

**Kind**: inner typedef of [processors](#module_processors)  

* * *
