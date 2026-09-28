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

## Classes

### PretrainedConfig

Base class for all configuration classes. For more information, see the corresponding
[Python documentation](https://huggingface.co/docs/transformers/main/en/main_classes/configuration#transformers.PretrainedConfig).

#### `PretrainedConfig.model_type` : `string` | `null`

**Default:** `null`

#### `PretrainedConfig.is_encoder_decoder` : `boolean`

**Default:** `false`

#### `PretrainedConfig.max_position_embeddings` : `number`

#### `PretrainedConfig.'transformers.js_config'` : [`TransformersJSConfig`](./configs#module_configs.TransformersJSConfig)

#### `PretrainedConfig.constructor(configJSON)`

Create a new `PretrainedConfig` from a parsed `config.json` object.

**Parameters**

- `configJSON` (`Object`) — The JSON of the config.

#### `PretrainedConfig.from_pretrained(pretrained_model_name_or_path, options)`

Loads a pretrained config from the given `pretrained_model_name_or_path`.

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The path to the pretrained config.
- `options` ([`PretrainedOptions`](./utils/hub#module_utils/hub.PretrainedOptions)) — Additional options for loading the config.

**Returns:** `Promise`<[`PretrainedConfig`](./configs#module_configs.PretrainedConfig)> — A new instance of the `PretrainedConfig` class.

**Throws**

- `Error` — Throws an error if the config.json is not found in the `pretrained_model_name_or_path`.

### AutoConfig

Loads a model config from a pretrained id. Thin alias for
`PretrainedConfig.from_pretrained`.

```javascript
import { AutoConfig } from '@huggingface/transformers';

const config = await AutoConfig.from_pretrained('Xenova/bert-base-uncased');
```

#### `AutoConfig.from_pretrained(pretrained_model_name_or_path, options)`

Loads a pretrained config from the given `pretrained_model_name_or_path`.

**Parameters**

- `pretrained_model_name_or_path` (`string`) — The path to the pretrained config.
- `options` ([`PretrainedOptions`](./utils/hub#module_utils/hub.PretrainedOptions)) — Additional options for loading the config.

**Returns:** `Promise`<[`PretrainedConfig`](./configs#module_configs.PretrainedConfig)> — A new instance of the `PretrainedConfig` class.

**Throws**

- `Error` — Throws an error if the config.json is not found in the `pretrained_model_name_or_path`.

## Type Definitions

### TransformersJSConfig

Transformers.js-specific configuration, possibly present in config.json under the key `transformers.js_config`.

**Properties**

- `device_config` (`Record`<[`DeviceType`](./transformers#module_transformers.DeviceType), [`DeviceConfig`](./configs#module_configs.DeviceConfig)>) _optional_ — Device-specific configurations.
- `free_dimension_overrides` (`Record`<`string`, `number`>) _optional_ — Override the free dimensions of the model.
  See https://onnxruntime.ai/docs/tutorials/web/env-flags-and-session-options.html#freedimensionoverrides
  for more information.
- `device` ([`DeviceType`](./transformers#module_transformers.DeviceType)) _optional_ — The default device to use for the model.
- `dtype` ([`DataType`](./transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](./transformers#module_transformers.DataType)>) _optional_ — The default data type to use for the model.
- `use_external_data_format` ([`ExternalData`](./utils/hub#module_utils/hub.ExternalData) | `Record`<`string`, [`ExternalData`](./utils/hub#module_utils/hub.ExternalData)>) _optional_ — defaults to `null` — Whether to load the model using the external data format (used for models >= 2GB in size).

### DeviceConfig

Device-specific configuration options.

_Type:_ `Omit<TransformersJSConfig, 'device' | 'device_config'>`
