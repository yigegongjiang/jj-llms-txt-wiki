# configs

Helper module for using model configs. For more information, see the corresponding
[Python documentation](https://huggingface.co/docs/transformers/main/en/model_doc/auto#transformers.AutoConfig).

**Example:** Load an `AutoConfig`.

```javascript
import { AutoConfig } from '@huggingface/transformers';
const config = await AutoConfig.from_pretrained('bert-base-uncased');
console.log(config);
// PretrainedConfig {
//   "model_type": "bert",
//   "is_encoder_decoder": false,
//   "architectures": [
//       "BertForMaskedLM"
//   ],
//   "vocab_size": 30522
//   "num_attention_heads": 12,
//   "num_hidden_layers": 12,
//   "hidden_size": 768,
//   "max_position_embeddings": 512,
//   ...
// }
```

* [configs](#module_configs)
    * _static_
        * [.PretrainedConfig](#module_configs.PretrainedConfig)
            * [`new PretrainedConfig(configJSON)`](#new_module_configs.PretrainedConfig_new)
            * _instance_
                * [`.model_type`](#module_configs.PretrainedConfig+model_type) : string | null
                * [`.is_encoder_decoder`](#module_configs.PretrainedConfig+is_encoder_decoder) : boolean
                * [`.max_position_embeddings`](#module_configs.PretrainedConfig+max_position_embeddings) : number
            * _static_
                * [`.from_pretrained(pretrained_model_name_or_path, options)`](#module_configs.PretrainedConfig.from_pretrained) ⇒ [Promise.&lt;PretrainedConfig&gt;](#PretrainedConfig)
        * [.AutoConfig](#module_configs.AutoConfig)
            * [`new AutoConfig()`](#new_module_configs.AutoConfig_new)
            * [`.from_pretrained()`](#module_configs.AutoConfig.from_pretrained) : Object.from_pretrained
        * [`.getCacheShapes(config)`](#module_configs.getCacheShapes) ⇒ Record.&lt;string, Array&gt;
            * [`~cache_values`](#module_configs.getCacheShapes..cache_values) : Record.&lt;string, Array&gt;
            * [`~cache_values`](#module_configs.getCacheShapes..cache_values) : Record.&lt;string, Array&gt;
            * [`~cache_values`](#module_configs.getCacheShapes..cache_values) : Record.&lt;string, Array&gt;
            * [`~cache_values`](#module_configs.getCacheShapes..cache_values) : Record.&lt;string, Array&gt;
    * _inner_
        * [`~loadConfig(pretrained_model_name_or_path, options)`](#module_configs..loadConfig) ⇒ Promise.&lt;Object&gt;
        * [`~getNormalizedConfig(config)`](#module_configs..getNormalizedConfig) ⇒ Object
        * [`~getKeyValueShapes()`](#module_configs..getKeyValueShapes) : Object
            * [`~decoderFeeds`](#module_configs..getKeyValueShapes..decoderFeeds) : Record.&lt;string, Array&gt;
        * [`~PretrainedOptions`](#module_configs..PretrainedOptions) : [PretrainedOptions](#PretrainedOptions)
        * [`~ProgressCallback`](#module_configs..ProgressCallback) : ProgressCallback
        * [`~ProgressInfo`](#module_configs..ProgressInfo) : ProgressInfo

* * *

## configs.PretrainedConfig

Base class for all configuration classes. For more information, see the corresponding
[Python documentation](https://huggingface.co/docs/transformers/main/en/main_classes/configuration#transformers.PretrainedConfig).

**Kind**: static class of [configs](#module_configs)  

* [.PretrainedConfig](#module_configs.PretrainedConfig)
    * [`new PretrainedConfig(configJSON)`](#new_module_configs.PretrainedConfig_new)
    * _instance_
        * [`.model_type`](#module_configs.PretrainedConfig+model_type) : string | null
        * [`.is_encoder_decoder`](#module_configs.PretrainedConfig+is_encoder_decoder) : boolean
        * [`.max_position_embeddings`](#module_configs.PretrainedConfig+max_position_embeddings) : number
    * _static_
        * [`.from_pretrained(pretrained_model_name_or_path, options)`](#module_configs.PretrainedConfig.from_pretrained) ⇒ [Promise.&lt;PretrainedConfig&gt;](#PretrainedConfig)

* * *

### `new PretrainedConfig(configJSON)`

Create a new PreTrainedTokenizer instance.

  
    
      ParamTypeDescription
    
  
  

    configJSONObjectThe JSON of the config.

      

* * *

### `pretrainedConfig.model_type` : string | null

**Kind**: instance property of [PretrainedConfig](#module_configs.PretrainedConfig)  

* * *

### `pretrainedConfig.is_encoder_decoder` : boolean

**Kind**: instance property of [PretrainedConfig](#module_configs.PretrainedConfig)  

* * *

### `pretrainedConfig.max_position_embeddings` : number

**Kind**: instance property of [PretrainedConfig](#module_configs.PretrainedConfig)  

* * *

### `PretrainedConfig.from_pretrained(pretrained_model_name_or_path, options)` ⇒ [Promise.&lt;PretrainedConfig&gt;](#PretrainedConfig)

Loads a pre-trained config from the given `pretrained_model_name_or_path`.

**Kind**: static method of [PretrainedConfig](#module_configs.PretrainedConfig)  
**Returns**: [Promise.&lt;PretrainedConfig&gt;](#PretrainedConfig) - A new instance of the `PretrainedConfig` class.  
**Throws**:

- Error Throws an error if the config.json is not found in the `pretrained_model_name_or_path`.

  
    
      ParamTypeDescription
    
  
  

    pretrained_model_name_or_pathstringThe path to the pre-trained config.

    
    optionsPretrainedOptionsAdditional options for loading the config.

      

* * *

## configs.AutoConfig

Helper class which is used to instantiate pretrained configs with the `from_pretrained` function.

**Kind**: static class of [configs](#module_configs)  

* [.AutoConfig](#module_configs.AutoConfig)
    * [`new AutoConfig()`](#new_module_configs.AutoConfig_new)
    * [`.from_pretrained()`](#module_configs.AutoConfig.from_pretrained) : Object.from_pretrained

* * *

### `new AutoConfig()`

**Example**  
```js
const config = await AutoConfig.from_pretrained('Xenova/bert-base-uncased');
```

* * *

### `AutoConfig.from_pretrained()` : Object.from_pretrained

**Kind**: static method of [AutoConfig](#module_configs.AutoConfig)  

* * *

## `configs.getCacheShapes(config)` ⇒ Record.&lt;string, Array&gt;

**Kind**: static method of [configs](#module_configs)  

  
    
      ParamType
    
  
  

    configPretrainedConfig
      

* [`.getCacheShapes(config)`](#module_configs.getCacheShapes) ⇒ Record.&lt;string, Array&gt;
    * [`~cache_values`](#module_configs.getCacheShapes..cache_values) : Record.&lt;string, Array&gt;
    * [`~cache_values`](#module_configs.getCacheShapes..cache_values) : Record.&lt;string, Array&gt;
    * [`~cache_values`](#module_configs.getCacheShapes..cache_values) : Record.&lt;string, Array&gt;
    * [`~cache_values`](#module_configs.getCacheShapes..cache_values) : Record.&lt;string, Array&gt;

* * *

### `getCacheShapes~cache_values` : Record.&lt;string, Array&gt;

**Kind**: inner constant of [getCacheShapes](#module_configs.getCacheShapes)  

* * *

### `getCacheShapes~cache_values` : Record.&lt;string, Array&gt;

**Kind**: inner constant of [getCacheShapes](#module_configs.getCacheShapes)  

* * *

### `getCacheShapes~cache_values` : Record.&lt;string, Array&gt;

**Kind**: inner constant of [getCacheShapes](#module_configs.getCacheShapes)  

* * *

### `getCacheShapes~cache_values` : Record.&lt;string, Array&gt;

**Kind**: inner constant of [getCacheShapes](#module_configs.getCacheShapes)  

* * *

## `configs~loadConfig(pretrained_model_name_or_path, options)` ⇒ Promise.&lt;Object&gt;

Loads a config from the specified path.

**Kind**: inner method of [configs](#module_configs)  
**Returns**: Promise.&lt;Object&gt; - A promise that resolves with information about the loaded config.  

  
    
      ParamTypeDescription
    
  
  

    pretrained_model_name_or_pathstringThe path to the config directory.

    
    optionsPretrainedOptionsAdditional options for loading the config.

      

* * *

## `configs~getNormalizedConfig(config)` ⇒ Object

**Kind**: inner method of [configs](#module_configs)  
**Returns**: Object - The normalized configuration.  

  
    
      ParamType
    
  
  

    configPretrainedConfig
      

* * *

## `configs~getKeyValueShapes()` : Object

**Kind**: inner method of [configs](#module_configs)  

* * *

### `getKeyValueShapes~decoderFeeds` : Record.&lt;string, Array&gt;

**Kind**: inner constant of [getKeyValueShapes](#module_configs..getKeyValueShapes)  

* * *

## `configs~PretrainedOptions` : [PretrainedOptions](#PretrainedOptions)

**Kind**: inner typedef of [configs](#module_configs)  

* * *

## `configs~ProgressCallback` : ProgressCallback

**Kind**: inner typedef of [configs](#module_configs)  

* * *

## `configs~ProgressInfo` : ProgressInfo

**Kind**: inner typedef of [configs](#module_configs)  

* * *
