# Configuration

Schedulers from [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin) and models from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin) inherit from [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin) which stores all the parameters that are passed to their respective `__init__` methods in a JSON-configuration file.

> [!TIP]
> To use private or [gated](https://huggingface.co/docs/hub/models-gated#gated-models) models, log-in with `hf auth login`.

## ConfigMixin[[diffusers.ConfigMixin]]

#### diffusers.ConfigMixin[[diffusers.ConfigMixin]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/configuration_utils.py#L88)

Base class for all configuration classes. All configuration parameters are stored under `self.config`. Also
provides the [from_config()](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin.from_config) and [save_config()](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin.save_config) methods for loading, downloading, and
saving classes that inherit from [ConfigMixin](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin).

Class attributes:
- **config_name** (`str`) -- A filename under which the config should stored when calling
  [save_config()](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin.save_config) (should be overridden by parent class).
- **ignore_for_config** (`list[str]`) -- A list of attributes that should not be saved in the config (should be
  overridden by subclass).
- **has_compatibles** (`bool`) -- Whether the class has compatible classes (should be overridden by subclass).
- **_deprecated_kwargs** (`list[str]`) -- Keyword arguments that are deprecated. Note that the `init` function
  should only have a `kwargs` argument if at least one argument is deprecated (should be overridden by
  subclass).

load_configdiffusers.ConfigMixin.load_confighttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/configuration_utils.py#L323[{"name": "pretrained_model_name_or_path", "val": ": str | os.PathLike"}, {"name": "return_unused_kwargs", "val": " = False"}, {"name": "return_commit_hash", "val": " = False"}, {"name": "**kwargs", "val": ""}]- **pretrained_model_name_or_path** (`str` or `os.PathLike`, *optional*) --
  Can be either:

  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on
    the Hub.
  - A path to a *directory* (for example `./my_model_directory`) containing model weights saved with
    [save_config()](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin.save_config).

- **cache_dir** (`str | os.PathLike`, *optional*) --
  Path to a directory where a downloaded pretrained model configuration is cached if the standard cache
  is not used.
- **force_download** (`bool`, *optional*, defaults to `False`) --
  Whether or not to force the (re-)download of the model weights and configuration files, overriding the
  cached versions if they exist.
- **proxies** (`dict[str, str]`, *optional*) --
  A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128',
  'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.
- **output_loading_info(`bool`,** *optional*, defaults to `False`) --
  Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.
- **local_files_only** (`bool`, *optional*, defaults to `False`) --
  Whether to only load local model weights and configuration files or not. If set to `True`, the model
  won't be downloaded from the Hub.
- **token** (`str` or *bool*, *optional*) --
  The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from
  `diffusers-cli login` (stored in `~/.huggingface`) is used.
- **revision** (`str`, *optional*, defaults to `"main"`) --
  The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier
  allowed by Git.
- **subfolder** (`str`, *optional*, defaults to `""`) --
  The subfolder location of a model file within a larger model repository on the Hub or locally.
- **return_unused_kwargs** (`bool`, *optional*, defaults to `False) --
  Whether unused keyword arguments of the config are returned.
- **return_commit_hash** (`bool`, *optional*, defaults to `False) --
  Whether the `commit_hash` of the loaded configuration are returned.0`dict`A dictionary of all the parameters stored in a JSON configuration file.

Load a model or scheduler configuration.

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`, *optional*) : Can be either:  - A string, the *model id* (for example `google/ddpm-celebahq-256`) of a pretrained model hosted on the Hub. - A path to a *directory* (for example `./my_model_directory`) containing model weights saved with [save_config()](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin.save_config). 

cache_dir (`str | os.PathLike`, *optional*) : Path to a directory where a downloaded pretrained model configuration is cached if the standard cache is not used.

force_download (`bool`, *optional*, defaults to `False`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

proxies (`dict[str, str]`, *optional*) : A dictionary of proxy servers to use by protocol or endpoint, for example, `{'http': 'foo.bar:3128', 'http://hostname': 'foo.bar:4012'}`. The proxies are used on each request.

output_loading_info(`bool`, *optional*, defaults to `False`) : Whether or not to also return a dictionary containing missing keys, unexpected keys and error messages.

local_files_only (`bool`, *optional*, defaults to `False`) : Whether to only load local model weights and configuration files or not. If set to `True`, the model won't be downloaded from the Hub.

token (`str` or *bool*, *optional*) : The token to use as HTTP bearer authorization for remote files. If `True`, the token generated from `diffusers-cli login` (stored in `~/.huggingface`) is used.

revision (`str`, *optional*, defaults to `"main"`) : The specific model version to use. It can be a branch name, a tag name, a commit id, or any identifier allowed by Git.

subfolder (`str`, *optional*, defaults to `""`) : The subfolder location of a model file within a larger model repository on the Hub or locally.

return_unused_kwargs (`bool`, *optional*, defaults to `False) : Whether unused keyword arguments of the config are returned.

return_commit_hash (`bool`, *optional*, defaults to `False) : Whether the `commit_hash` of the loaded configuration are returned.

**Returns:**

``dict``

A dictionary of all the parameters stored in a JSON configuration file.
#### from_config[[diffusers.ConfigMixin.from_config]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/configuration_utils.py#L222)

Instantiate a Python class from a config dictionary.

Examples:

```python
>>> from diffusers import DDPMScheduler, DDIMScheduler, PNDMScheduler

>>> # Download scheduler from huggingface.co and cache.
>>> scheduler = DDPMScheduler.from_pretrained("google/ddpm-cifar10-32")

>>> # Instantiate DDIM scheduler class with same config as DDPM
>>> scheduler = DDIMScheduler.from_config(scheduler.config)

>>> # Instantiate PNDM scheduler class with same config as DDPM
>>> scheduler = PNDMScheduler.from_config(scheduler.config)
```

**Parameters:**

config (`dict[str, Any]`) : A config dictionary from which the Python class is instantiated. Make sure to only load configuration files of compatible classes.

return_unused_kwargs (`bool`, *optional*, defaults to `False`) : Whether kwargs that are not consumed by the Python class should be returned or not.

kwargs (remaining dictionary of keyword arguments, *optional*) : Can be used to update the configuration object (after it is loaded) and initiate the Python class. `**kwargs` are passed directly to the underlying scheduler/model's `__init__` method and eventually overwrite the same named arguments in `config`.

**Returns:**

`[ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin) or [SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)`

A model or scheduler object instantiated from a config dictionary.
#### save_config[[diffusers.ConfigMixin.save_config]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/configuration_utils.py#L178)

Save a configuration object to the directory specified in `save_directory` so that it can be reloaded using the
[from_config()](/docs/diffusers/v0.39.0/en/api/configuration#diffusers.ConfigMixin.from_config) class method.

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory where the configuration JSON file is saved (will be created if it does not exist).

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether or not to push your model to the Hugging Face Hub after saving it. You can specify the repository you want to push to with `repo_id` (will default to the name of `save_directory` in your namespace).

kwargs (`dict[str, Any]`, *optional*) : Additional keyword arguments passed along to the [push_to_hub()](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.utils.PushToHubMixin.push_to_hub) method.
#### to_json_file[[diffusers.ConfigMixin.to_json_file]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/configuration_utils.py#L664)

Save the configuration instance's parameters to a JSON file.

**Parameters:**

json_file_path (`str` or `os.PathLike`) : Path to the JSON file to save a configuration instance's parameters.
#### to_json_string[[diffusers.ConfigMixin.to_json_string]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/configuration_utils.py#L619)

Serializes the configuration instance to a JSON string.

**Returns:**

``str``

String containing all the attributes that make up the configuration instance in JSON format.
