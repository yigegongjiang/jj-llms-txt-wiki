# utils/hub

Utility functions to interact with the Hugging Face Hub (https://huggingface.co/models)

## Type Definitions

### ExternalData

Specifies whether to load the model using the external data format.
- `false`: Do not use external data format
- `true`: Use external data format with 1 chunk
- `number`: Use external data format with the specified number of chunks

_Type:_ `boolean` | `number`

### PretrainedOptions

Options for loading a pretrained model.

**Properties**

- `progress_callback` ([`ProgressCallback`](./core#module_utils/core.ProgressCallback)) _optional_ — defaults to `null` — If specified, this function is called during model construction with progress updates.
- `config` ([`PretrainedConfig`](../configs#module_configs.PretrainedConfig)) _optional_ — defaults to `null` — Configuration to use for the model instead of an automatically loaded configuration. Configuration can be automatically loaded when:
  - The model is provided by the library and loaded with the *model ID* string of a pretrained model.
  - The model is loaded by supplying a local directory as `pretrained_model_name_or_path` and a configuration JSON file named *config.json* is found in the directory.
- `cache_dir` (`string`) _optional_ — defaults to `null` — Path to a directory where downloaded model files should be cached if the standard cache should not be used.
- `local_files_only` (`boolean`) _optional_ — defaults to `false` — Whether to only look at local files (e.g., not try downloading the model).
- `revision` (`string`) _optional_ — defaults to `'main'` — The model revision to use. This can be a branch name, tag name, or commit ID.
  Because the Hub uses Git-based storage, `revision` can be any identifier accepted by Git. Ignored for local requests.

### ModelSpecificPretrainedOptions

Options for loading a pretrained model.

**Properties**

- `subfolder` (`string`) _optional_ — defaults to `'onnx'` — In case the relevant files are located inside a subfolder of the model repo on huggingface.co,
  you can specify the folder name here.
- `model_file_name` (`string`) _optional_ — defaults to `null` — Override the base ONNX model file name, excluding dtype and `.onnx` suffixes. This is most useful for single-session models.
- `device` ([`DeviceType`](../transformers#module_transformers.DeviceType) | `Record`<`string`, [`DeviceType`](../transformers#module_transformers.DeviceType)>) _optional_ — defaults to `null` — The device to run the model on. If not specified, the device will be chosen from the environment settings.
- `dtype` ([`DataType`](../transformers#module_transformers.DataType) | `Record`<`string`, [`DataType`](../transformers#module_transformers.DataType)>) _optional_ — defaults to `null` — The data type to use for the model. If not specified, the data type will be chosen from the environment settings.
- `use_external_data_format` ([`ExternalData`](./hub#module_utils/hub.ExternalData) | `Record`<`string`, [`ExternalData`](./hub#module_utils/hub.ExternalData)>) _optional_ — defaults to `null` — Whether to load external data files. `null` uses the model configuration; `false` disables external data; `true` uses one chunk; a number selects the chunk count.
- `session_options` (`InferenceSession.SessionOptions`) _optional_ — defaults to `{}` — User-specified ONNX Runtime session options. Suitable defaults are filled in when omitted.

### PretrainedModelOptions

Options for loading a pretrained model.

_Type:_ [`PretrainedOptions`](./hub#module_utils/hub.PretrainedOptions) & [`ModelSpecificPretrainedOptions`](./hub#module_utils/hub.ModelSpecificPretrainedOptions)
