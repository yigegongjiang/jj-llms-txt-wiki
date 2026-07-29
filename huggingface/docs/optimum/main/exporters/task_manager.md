# The Tasks Manager

Exporting a model from one framework to some format (also called backend here) involves specifying inputs and outputs information that the export function needs. The way `optimum.exporters` is structured for each backend is as follows:
- Configuration classes containing the information for each model to perform the export.
- Exporting functions using the proper configuration for the model to export.

The role of the [TasksManager](/docs/optimum/main/en/exporters/task_manager#optimum.exporters.tasks.TasksManager) is to be the main entry-point to load a model given a name and a task, and to get the proper configuration for a given (architecture, backend) couple. 
That way, there is a centralized place to register the `task -> model class` and `(architecture, backend) -> configuration` mappings. This allows the export functions to use this, and to rely on the various checks it provides.

## Task names

The tasks supported might depend on the backend, but here are the mappings between a task name and the auto class for PyTorch.

It is possible to know which tasks are supported for a model for a given backend, by doing:

```python
>>> from optimum.exporters.tasks import TasksManager

>>> model_type = "distilbert"
>>> # For instance, for the ONNX export.
>>> backend = "onnx"
>>> distilbert_tasks = list(TasksManager.get_supported_tasks_for_model_type(model_type, backend).keys())

>>> print(distilbert_tasks)
['default', 'fill-mask', 'text-classification', 'multiple-choice', 'token-classification', 'question-answering']
```

### PyTorch

#### Transformers

| Task                             | Auto Class                                                    |
|----------------------------------|---------------------------------------------------------------|
| `audio-classification`           | `AutoModelForAudioClassification`                             |
| `audio-frame-classification`     | `AutoModelForAudioFrameClassification`                        |
| `audio-xvector`                  | `AutoModelForAudioXVector`                                    |
| `automatic-speech-recognition`   | `AutoModelForSpeechSeq2Seq`, `AutoModelForCTC`                |
| `depth-estimation`               | `AutoModelForDepthEstimation`                                 |
| `feature-extraction`             | `AutoModel`                                                   |
| `fill-mask`                      | `AutoModelForMaskedLM`                                        |
| `image-classification`           | `AutoModelForImageClassification`                             |
| `image-to-image`                 | `AutoModelForImageToImage`                                    |
| `image-to-text`                  | `AutoModelForVision2Seq`, `AutoModel`                         |
| `image-text-to-text`             | `AutoModelForImageTextToText`                                 |
| `mask-generation`                | `AutoModel`                                                   |
| `masked-im`                      | `AutoModelForMaskedImageModeling`                             |
| `multiple-choice`                | `AutoModelForMultipleChoice`                                  |
| `object-detection`               | `AutoModelForObjectDetection`                                 |
| `question-answering`             | `AutoModelForQuestionAnswering`                               |
| `reinforcement-learning`         | `AutoModel`                                                   |
| `semantic-segmentation`          | `AutoModelForSemanticSegmentation`                            |
| `text-to-audio`                  | `AutoModelForTextToSpectrogram`, `AutoModelForTextToWaveform` |
| `text-generation`                | `AutoModelForCausalLM`                                        |
| `text2text-generation`           | `AutoModelForSeq2SeqLM`                                       |
| `text-classification`            | `AutoModelForSequenceClassification`                          |
| `token-classification`           | `AutoModelForTokenClassification`                             |
| `visual-question-answering`      | `AutoModelForVisualQuestionAnswering`                         |
| `zero-shot-image-classification` | `AutoModelForZeroShotImageClassification`                     |
| `zero-shot-object-detection`     | `AutoModelForZeroShotObjectDetection`                         |

#### Diffusers

| Task             | Auto Class                   |
|------------------|------------------------------|
| `text-to-image`  | `AutoPipelineForText2Image`  |
| `image-to-image` | `AutoPipelineForImage2Image` |
| `inpainting`     | `AutoPipelineForInpainting`  |

#### Sentence Transformers

| Task                  | Auto Class            |
|-----------------------|-----------------------|
| `feature-extraction`  | `SentenceTransformer` |
| `sentence-similarity` | `SentenceTransformer` |

## Reference[[optimum.exporters.tasks.TasksManager]]

#### optimum.exporters.tasks.TasksManager[[optimum.exporters.tasks.TasksManager]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L118)

Handles the `task name -> model class` and `architecture -> configuration` mappings.

create_registeroptimum.exporters.tasks.TasksManager.create_registerhttps://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L289[{"name": "backend", "val": ": str"}, {"name": "overwrite_existing", "val": ": bool = False"}]- **backend** (`str`) --
  The name of the backend that the register function will handle.
- **overwrite_existing** (`bool`, defaults to `False`) --
  Whether or not the register function is allowed to overwrite an already existing config.0`Callable[[str, Tuple[str, ...]], Callable[[Type], Type]]`A decorator taking the model type and a the
supported tasks.

Creates a register function for the specified backend.

Example:
```python
>>> register_for_new_backend = create_register("new-backend")

>>> @register_for_new_backend("bert", "text-classification", "token-classification")
>>> class BertNewBackendConfig(NewBackendConfig):
>>>     pass
```

**Parameters:**

backend (`str`) : The name of the backend that the register function will handle.

overwrite_existing (`bool`, defaults to `False`) : Whether or not the register function is allowed to overwrite an already existing config.

**Returns:**

``Callable[[str, Tuple[str, ...]], Callable[[Type], Type]]``

A decorator taking the model type and a the
supported tasks.
#### determine_framework[[optimum.exporters.tasks.TasksManager.determine_framework]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L581)

Determines the framework to use for the export.

The priority is in the following order:
1. User input via `framework`.
2. If local checkpoint is provided, use the same framework as the checkpoint.
3. If model repo, try to infer the framework from the cache if available, else from the Hub.
4. If could not infer, use available framework in environment, with priority given to PyTorch.

**Parameters:**

model_name_or_path (`Union[str, Path]`) : Can be either the model id of a model repo on the Hugging Face Hub, or a path to a local directory containing a model.

subfolder (`str`, *optional*, defaults to `""`) : In case the model files are located inside a subfolder of the model directory / repo on the Hugging Face Hub, you can specify the subfolder name here.

revision (`Optional[str]`,  defaults to `None`) : Revision is the specific model version to use. It can be a branch name, a tag name, or a commit id.

cache_dir (`Optional[str]`, *optional*) : Path to a directory in which a downloaded pretrained model weights have been cached if the standard cache should not be used.

token (`Optional[Union[bool,str]]`, defaults to `None`) : The token to use as HTTP bearer authorization for remote files. If `True`, will use the token generated when running `huggingface-cli login` (stored in `huggingface_hub.constants.HF_TOKEN_PATH`).

**Returns:**

``str``

The framework to use for the export.
#### get_all_tasks[[optimum.exporters.tasks.TasksManager.get_all_tasks]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L1055)

Retrieves all the possible tasks.

**Returns:**

``List``

all the possible tasks.
#### get_exporter_config_constructor[[optimum.exporters.tasks.TasksManager.get_exporter_config_constructor]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L1228)

Gets the `ExportConfigConstructor` for a model (or alternatively for a model type) and task combination.

**Parameters:**

exporter (`str`) : The exporter to use.

model (`Optional[PreTrainedModel]`, defaults to `None`) : The instance of the model.

task (`str`, defaults to `"feature-extraction"`) : The task to retrieve the config for.

model_type (`Optional[str]`, defaults to `None`) : The model type to retrieve the config for.

model_name (`Optional[str]`, defaults to `None`) : The name attribute of the model object, only used for the exception message.

exporter_config_kwargs (`Optional[Dict[str, Any]]`, defaults to `None`) : Arguments that will be passed to the exporter config class when building the config constructor.

library_name (`Optional[str]`, defaults to `None`) : The library name of the model. Can be any of "transformers", "timm", "diffusers", "sentence_transformers".

**Returns:**

``ExportConfigConstructor``

The `ExporterConfig` constructor for the requested backend.
#### get_model_class_for_task[[optimum.exporters.tasks.TasksManager.get_model_class_for_task]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L447)

Attempts to retrieve an AutoModel class from a task name.

**Parameters:**

task (`str`) : The task required.

framework (`str`, defaults to `"pt"`) : The framework to use for the export.

model_type (`Optional[str]`, defaults to `None`) : The model type to retrieve the model class for. Some architectures need a custom class to be loaded, and can not be loaded from auto class.

model_class_name (`Optional[str]`, defaults to `None`) : A model class name, allowing to override the default class that would be detected for the task. This parameter is useful for example for "automatic-speech-recognition", that may map to AutoModelForSpeechSeq2Seq or to AutoModelForCTC.

library (`str`, defaults to `transformers`) : The library name of the model. Can be any of "transformers", "timm", "diffusers", "sentence_transformers".

**Returns:**

The AutoModel class corresponding to the task.
#### get_model_from_task[[optimum.exporters.tasks.TasksManager.get_model_from_task]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L1075)

Retrieves a model from its name and the task to be enabled.

**Parameters:**

task (`str`) : The task required.

model_name_or_path (`Union[str, Path]`) : Can be either the model id of a model repo on the Hugging Face Hub, or a path to a local directory containing a model.

subfolder (`str`, defaults to `""`) : In case the model files are located inside a subfolder of the model directory / repo on the Hugging Face Hub, you can specify the subfolder name here.

revision (`Optional[str]`, *optional*) : Revision is the specific model version to use. It can be a branch name, a tag name, or a commit id.

cache_dir (`Optional[str]`, *optional*) : Path to a directory in which a downloaded pretrained model weights have been cached if the standard cache should not be used.

token (`Optional[Union[bool,str]]`, defaults to `None`) : The token to use as HTTP bearer authorization for remote files. If `True`, will use the token generated when running `huggingface-cli login` (stored in `huggingface_hub.constants.HF_TOKEN_PATH`).

framework (`Optional[str]`, *optional*) : The framework to use for the export. See `TasksManager.determine_framework` for the priority should none be provided.

torch_dtype (`Optional[torch.dtype]`, defaults to `None`) : Data type to load the model on. PyTorch-only argument.

device (`Optional[torch.device]`, defaults to `None`) : Device to initialize the model on. PyTorch-only argument. For PyTorch, defaults to "cpu".

library_name (`Optional[str]`, defaults to `None`) : The library name of the model. Can be any of "transformers", "timm", "diffusers", "sentence_transformers". See `TasksManager.infer_library_from_model` for the priority should none be provided.

model_kwargs (`Dict[str, Any]`, *optional*) : Keyword arguments to pass to the model `.from_pretrained()` method.

library_name (`Optional[str]`, defaults to `None`) : The library name of the model. Can be any of "transformers", "timm", "diffusers", "sentence_transformers". See `TasksManager.infer_library_from_model` for the priority should none be provided.

**Returns:**

The instance of the model.
#### get_supported_model_type_for_task[[optimum.exporters.tasks.TasksManager.get_supported_model_type_for_task]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L405)

Returns the list of supported architectures by the exporter for a given task. Transformers-specific.
#### get_supported_tasks_for_model_type[[optimum.exporters.tasks.TasksManager.get_supported_tasks_for_model_type]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L344)

Retrieves the `task -> exporter backend config constructors` map from the model type.

**Parameters:**

model_type (`str`) : The model type to retrieve the supported tasks for.

exporter (`str`) : The name of the exporter.

model_name (`Optional[str]`, defaults to `None`) : The name attribute of the model object, only used for the exception message.

library_name (`Optional[str]`, defaults to `None`) : The library name of the model. Can be any of "transformers", "timm", "diffusers", "sentence_transformers".

**Returns:**

``Dict[str, ExportConfigConstructor]``

The mapping between the supported tasks and the backend config
constructors for the specified model type.
#### infer_library_from_model[[optimum.exporters.tasks.TasksManager.infer_library_from_model]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L958)

Infers the library from the model repo, model instance, or model class.

**Parameters:**

model (`Union[str, PreTrainedModel, DiffusionPipeline, Type]`) : The model to infer the task from. This can either be the name of a repo on the HuggingFace Hub, an instance of a model, or a model class.

subfolder (`str`, defaults to `""`) : In case the model files are located inside a subfolder of the model directory / repo on the Hugging Face Hub, you can specify the subfolder name here.

revision (`Optional[str]`, *optional*, defaults to `None`) : Revision is the specific model version to use. It can be a branch name, a tag name, or a commit id.

cache_dir (`Optional[str]`, *optional*) : Path to a directory in which a downloaded pretrained model weights have been cached if the standard cache should not be used.

token (`Optional[Union[bool,str]]`, defaults to `None`) : The token to use as HTTP bearer authorization for remote files. If `True`, will use the token generated when running `huggingface-cli login` (stored in `huggingface_hub.constants.HF_TOKEN_PATH`).

**Returns:**

``str``

The library name automatically detected from the model repo, model instance, or model class.
#### infer_task_from_model[[optimum.exporters.tasks.TasksManager.infer_task_from_model]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L799)

Infers the task from the model repo, model instance, or model class.

**Parameters:**

model (`Union[str, PreTrainedModel, DiffusionPipeline, Type]`) : The model to infer the task from. This can either be the name of a repo on the HuggingFace Hub, an instance of a model, or a model class.

subfolder (`str`, *optional*, defaults to `""`) : In case the model files are located inside a subfolder of the model directory / repo on the Hugging Face Hub, you can specify the subfolder name here.

revision (`Optional[str]`,  defaults to `None`) : Revision is the specific model version to use. It can be a branch name, a tag name, or a commit id.

cache_dir (`Optional[str]`, *optional*) : Path to a directory in which a downloaded pretrained model weights have been cached if the standard cache should not be used.

token (`Optional[Union[bool,str]]`, defaults to `None`) : The token to use as HTTP bearer authorization for remote files. If `True`, will use the token generated when running `huggingface-cli login` (stored in `huggingface_hub.constants.HF_TOKEN_PATH`).

library_name (`Optional[str]`, defaults to `None`) : The library name of the model. Can be any of "transformers", "timm", "diffusers", "sentence_transformers". See `TasksManager.infer_library_from_model` for the priority should none be provided.

**Returns:**

``str``

The task name automatically detected from the HF hub repo, model instance, or model class.
#### standardize_model_attributes[[optimum.exporters.tasks.TasksManager.standardize_model_attributes]]

[Source](https://github.com/huggingface/optimum/blob/main/optimum/exporters/tasks.py#L1004)

Updates the model for export. This function is suitable to make required changes to the models from different
libraries to follow transformers style.

**Parameters:**

model (`Union[PreTrainedModel, DiffusionPipeline]`) : The instance of the model.
