# Configuration

`PeftConfigMixin` is the base configuration class for storing the adapter configuration of a [PeftModel](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel), and [PromptLearningConfig](/docs/peft/v0.20.0/en/package_reference/config#peft.PromptLearningConfig) is the base configuration class for soft prompt methods (p-tuning, prefix tuning, and prompt tuning). These base classes contain methods for saving and loading model configurations from the Hub, specifying the PEFT method to use, type of task to perform, and model configurations like number of layers and number of attention heads.

## PeftConfigMixin[[peft.config.PeftConfigMixin]]

#### peft.config.PeftConfigMixin[[peft.config.PeftConfigMixin]]

```python
peft.config.PeftConfigMixin(task_type: Optional[TaskType] = None, peft_type: Optional[PeftType] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/config.py#L77)

**Parameters:**

peft_type (Union[`~peft.utils.config.PeftType`, `str`]) : The type of Peft method to use.

This is the base configuration class for PEFT adapter models. It contains all the methods that are common to all
PEFT adapter models. This class inherits from [PushToHubMixin](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.utils.PushToHubMixin) which contains the methods to
push your model to the Hub. The method `save_pretrained` will save the configuration of your adapter model in a
directory. The method `from_pretrained` will load the configuration of your adapter model from a directory.

#### check_kwargs[[peft.config.PeftConfigMixin.check_kwargs]]

```python
check_kwargs(**kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/config.py#L328)

Check kwargs before initializing the config instance.

Subclasses can override this method to add specific checks.

#### from_json_file[[peft.config.PeftConfigMixin.from_json_file]]

```python
from_json_file(path_json_file: str, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/config.py#L266)

**Parameters:**

path_json_file (`str`) : The path to the json file.

Loads a configuration file from a json file.

#### from_peft_type[[peft.config.PeftConfigMixin.from_peft_type]]

```python
from_peft_type(**kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/config.py#L165)

**Parameters:**

kwargs (configuration keyword arguments) : Keyword arguments passed along to the configuration initialization.

This method loads the configuration of your adapter model from a set of kwargs.

The appropriate configuration type is determined by the `peft_type` argument. If `peft_type` is not provided,
the calling class type is instantiated.

#### from_pretrained[[peft.config.PeftConfigMixin.from_pretrained]]

```python
from_pretrained(pretrained_model_name_or_path: str, subfolder: Optional[str] = None, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/config.py#L230)

**Parameters:**

pretrained_model_name_or_path (`str`) : The directory or the Hub repository id where the configuration is saved.

kwargs (additional keyword arguments, *optional*) : Additional keyword arguments passed along to the child class initialization.

This method loads the configuration of your adapter model from a directory.

#### save_pretrained[[peft.config.PeftConfigMixin.save_pretrained]]

```python
save_pretrained(save_directory: str, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/config.py#L132)

**Parameters:**

save_directory (`str`) : The directory where the configuration will be saved.

kwargs (additional keyword arguments, *optional*) : Additional keyword arguments passed along to the [push_to_hub](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.utils.PushToHubMixin.push_to_hub) method.

This method saves the configuration of your adapter model in a directory.

#### to_dict[[peft.config.PeftConfigMixin.to_dict]]

```python
to_dict()
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/config.py#L126)

Returns the configuration for your adapter model as a dictionary.

## PeftConfig[[peft.PeftConfig]]

#### peft.PeftConfig[[peft.PeftConfig]]

```python
peft.PeftConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/config.py#L351)

**Parameters:**

peft_type (Union[`~peft.utils.config.PeftType`, `str`]) : The type of Peft method to use.

task_type (Union[`~peft.utils.config.TaskType`, `str`]) : The type of task to perform.

inference_mode (`bool`, defaults to `False`) : Whether to use the Peft model in inference mode.

This is the base configuration class to store the configuration of a [PeftModel](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel).

## PromptLearningConfig[[peft.PromptLearningConfig]]

#### peft.PromptLearningConfig[[peft.PromptLearningConfig]]

```python
peft.PromptLearningConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, num_virtual_tokens: int = None, token_dim: int = None, num_transformer_submodules: Optional[int] = None, num_attention_heads: Optional[int] = None, num_layers: Optional[int] = None, modules_to_save: Optional[list[str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/config.py#L371)

**Parameters:**

num_virtual_tokens (`int`) : The number of virtual tokens to use.

token_dim (`int`) : The hidden embedding dimension of the base transformer model.

num_transformer_submodules (`int`) : The number of transformer submodules in the base transformer model.

num_attention_heads (`int`) : The number of attention heads in the base transformer model.

num_layers (`int`) : The number of layers in the base transformer model.

This is the base configuration class to store the configuration of `PrefixTuning`, [PromptEncoder](/docs/peft/v0.20.0/en/package_reference/p_tuning#peft.PromptEncoder), or
`PromptTuning`.
