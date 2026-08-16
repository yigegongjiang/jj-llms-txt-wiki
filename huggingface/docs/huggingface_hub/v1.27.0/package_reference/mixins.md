# Mixins & serialization methods

## Mixins

The `huggingface_hub` library offers a range of mixins that can be used as a parent class for your objects, in order to
provide simple uploading and downloading functions. Check out our [integration guide](../guides/integrations) to learn
how to integrate any ML framework with the Hub.

### Generic[[huggingface_hub.ModelHubMixin]]

#### huggingface_hub.ModelHubMixin[[huggingface_hub.ModelHubMixin]]

```python
huggingface_hub.ModelHubMixin(*args, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/hub_mixin.py#L77)

**Parameters:**

repo_url (`str`, *optional*) : URL of the library repository. Used to generate model card.

paper_url (`str`, *optional*) : URL of the library paper. Used to generate model card.

docs_url (`str`, *optional*) : URL of the library documentation. Used to generate model card.

model_card_template (`str`, *optional*) : Template of the model card. Used to generate model card. Defaults to a generic template.

language (`str` or `list[str]`, *optional*) : Language supported by the library. Used to generate model card.

library_name (`str`, *optional*) : Name of the library integrating ModelHubMixin. Used to generate model card.

license (`str`, *optional*) : License of the library integrating ModelHubMixin. Used to generate model card. E.g: "apache-2.0"

license_name (`str`, *optional*) : Name of the library integrating ModelHubMixin. Used to generate model card. Only used if `license` is set to `other`. E.g: "coqui-public-model-license".

license_link (`str`, *optional*) : URL to the license of the library integrating ModelHubMixin. Used to generate model card. Only used if `license` is set to `other` and `license_name` is set. E.g: "https://coqui.ai/cpml".

pipeline_tag (`str`, *optional*) : Tag of the pipeline. Used to generate model card. E.g. "text-classification".

tags (`list[str]`, *optional*) : Tags to be added to the model card. Used to generate model card. E.g. ["computer-vision"]

coders (`dict[Type, tuple[Callable, Callable]]`, *optional*) : Dictionary of custom types and their encoders/decoders. Used to encode/decode arguments that are not jsonable by default. E.g. dataclasses, argparse.Namespace, OmegaConf, etc.

A generic mixin to integrate ANY machine learning framework with the Hub.

To integrate your framework, your model class must inherit from this class. Custom logic for saving/loading models
have to be overwritten in  `_from_pretrained` and `_save_pretrained`. [PyTorchModelHubMixin](/docs/huggingface_hub/v1.27.0/en/package_reference/mixins#huggingface_hub.PyTorchModelHubMixin) is a good example
of mixin integration with the Hub. Check out our [integration guide](../guides/integrations) for more instructions.

When inheriting from [ModelHubMixin](/docs/huggingface_hub/v1.27.0/en/package_reference/mixins#huggingface_hub.ModelHubMixin), you can define class-level attributes. These attributes are not passed to
`__init__` but to the class definition itself. This is useful to define metadata about the library integrating
[ModelHubMixin](/docs/huggingface_hub/v1.27.0/en/package_reference/mixins#huggingface_hub.ModelHubMixin).

For more details on how to integrate the mixin with your library, checkout the [integration guide](../guides/integrations).

Example:

```python
>>> from huggingface_hub import ModelHubMixin

# Inherit from ModelHubMixin
>>> class MyCustomModel(
...         ModelHubMixin,
...         library_name="my-library",
...         tags=["computer-vision"],
...         repo_url="https://github.com/huggingface/my-cool-library",
...         paper_url="https://arxiv.org/abs/2304.12244",
...         docs_url="https://huggingface.co/docs/my-cool-library",
...         # ^ optional metadata to generate model card
...     ):
...     def __init__(self, size: int = 512, device: str = "cpu"):
...         # define how to initialize your model
...         super().__init__()
...         ...
...
...     def _save_pretrained(self, save_directory: Path) -> None:
...         # define how to serialize your model
...         ...
...
...     @classmethod
...     def from_pretrained(
...         cls: type[T],
...         pretrained_model_name_or_path: Union[str, Path],
...         *,
...         force_download: bool = False,
...         token: Optional[Union[str, bool]] = None,
...         cache_dir: Optional[Union[str, Path]] = None,
...         local_files_only: bool = False,
...         revision: Optional[str] = None,
...         **model_kwargs,
...     ) -> T:
...         # define how to deserialize your model
...         ...

>>> model = MyCustomModel(size=256, device="gpu")

# Save model weights to local directory
>>> model.save_pretrained("my-awesome-model")

# Push model weights to the Hub
>>> model.push_to_hub("my-awesome-model")

# Download and initialize weights from the Hub
>>> reloaded_model = MyCustomModel.from_pretrained("username/my-awesome-model")
>>> reloaded_model.size
256

# Model card has been correctly populated
>>> from huggingface_hub import ModelCard
>>> card = ModelCard.load("username/my-awesome-model")
>>> card.data.tags
["x-custom-tag", "pytorch_model_hub_mixin", "model_hub_mixin"]
>>> card.data.library_name
"my-library"
```

#### _save_pretrained[[huggingface_hub.ModelHubMixin._save_pretrained]]

```python
_save_pretrained(save_directory: Path)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/hub_mixin.py#L451)

**Parameters:**

save_directory (`str` or `Path`) : Path to directory in which the model weights and configuration will be saved.

Overwrite this method in subclass to define how to save your model.
Check out our [integration guide](../guides/integrations) for instructions.

#### _from_pretrained[[huggingface_hub.ModelHubMixin._from_pretrained]]

```python
_from_pretrained(model_id: str, revision: str | None, cache_dir: str | pathlib.Path | None, force_download: bool, local_files_only: bool, token: str | bool | None, **model_kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/hub_mixin.py#L578)

**Parameters:**

model_id (`str`) : ID of the model to load from the Huggingface Hub (e.g. `bigscience/bloom`).

revision (`str`, *optional*) : Revision of the model on the Hub. Can be a branch name, a git tag or any commit id. Defaults to the latest commit on `main` branch.

force_download (`bool`, *optional*, defaults to `False`) : Whether to force (re-)downloading the model weights and configuration files from the Hub, overriding the existing cache.

token (`str` or `bool`, *optional*) : The token to use as HTTP bearer authorization for remote files. By default, it will use the token cached when running `hf auth login`.

cache_dir (`str`, `Path`, *optional*) : Path to the folder where cached files are stored.

local_files_only (`bool`, *optional*, defaults to `False`) : If `True`, avoid downloading the file and return the path to the local cached file if it exists.

model_kwargs : Additional keyword arguments passed along to the [_from_pretrained()](/docs/huggingface_hub/v1.27.0/en/package_reference/mixins#huggingface_hub.ModelHubMixin._from_pretrained) method.

Overwrite this method in subclass to define how to load your model from pretrained.

Use [hf_hub_download()](/docs/huggingface_hub/v1.27.0/en/package_reference/file_download#huggingface_hub.hf_hub_download) or [snapshot_download()](/docs/huggingface_hub/v1.27.0/en/package_reference/file_download#huggingface_hub.snapshot_download) to download files from the Hub before loading them. Most
args taken as input can be directly passed to those 2 methods. If needed, you can add more arguments to this
method using "model_kwargs". For example `PyTorchModelHubMixin._from_pretrained()` takes as input a `map_location`
parameter to set on which device the model should be loaded.

Check out our [integration guide](../guides/integrations) for more instructions.

#### from_pretrained[[huggingface_hub.ModelHubMixin.from_pretrained]]

```python
from_pretrained(pretrained_model_name_or_path: str | pathlib.Path, force_download: bool = False, token: str | bool | None = None, cache_dir: str | pathlib.Path | None = None, local_files_only: bool = False, revision: str | None = None, **model_kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/hub_mixin.py#L462)

**Parameters:**

pretrained_model_name_or_path (`str`, `Path`) : - Either the `model_id` (string) of a model hosted on the Hub, e.g. `bigscience/bloom`. - Or a path to a `directory` containing model weights saved using [save_pretrained](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel.save_pretrained), e.g., `../path/to/my_model_directory/`.

revision (`str`, *optional*) : Revision of the model on the Hub. Can be a branch name, a git tag or any commit id. Defaults to the latest commit on `main` branch.

force_download (`bool`, *optional*, defaults to `False`) : Whether to force (re-)downloading the model weights and configuration files from the Hub, overriding the existing cache.

token (`str` or `bool`, *optional*) : The token to use as HTTP bearer authorization for remote files. By default, it will use the token cached when running `hf auth login`.

cache_dir (`str`, `Path`, *optional*) : Path to the folder where cached files are stored.

local_files_only (`bool`, *optional*, defaults to `False`) : If `True`, avoid downloading the file and return the path to the local cached file if it exists.

model_kwargs (`dict`, *optional*) : Additional kwargs to pass to the model during initialization.

Download a model from the Huggingface Hub and instantiate it.

#### push_to_hub[[huggingface_hub.ModelHubMixin.push_to_hub]]

```python
push_to_hub(repo_id: str, config: dict | huggingface_hub.hub_mixin.DataclassInstance | None = None, commit_message: str = 'Push model using huggingface_hub.', private: bool | None = None, token: str | None = None, branch: str | None = None, create_pr: bool | None = None, allow_patterns: list[str] | str | None = None, ignore_patterns: list[str] | str | None = None, delete_patterns: list[str] | str | None = None, model_card_kwargs: dict[str, typing.Any] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/hub_mixin.py#L620)

**Parameters:**

repo_id (`str`) : ID of the repository to push to (example: `"username/my-model"`).

config (`dict` or `DataclassInstance`, *optional*) : Model configuration specified as a key/value dictionary or a dataclass instance.

commit_message (`str`, *optional*) : Message to commit while pushing.

private (`bool`, *optional*) : Whether the repository created should be private. If `None` (default), the repo will be public unless the organization's default is private.

token (`str`, *optional*) : The token to use as HTTP bearer authorization for remote files. By default, it will use the token cached when running `hf auth login`.

branch (`str`, *optional*) : The git branch on which to push the model. This defaults to `"main"`.

create_pr (`boolean`, *optional*) : Whether or not to create a Pull Request from `branch` with that commit. Defaults to `False`.

allow_patterns (`list[str]` or `str`, *optional*) : If provided, only files matching at least one pattern are pushed.

ignore_patterns (`list[str]` or `str`, *optional*) : If provided, files matching any of the patterns are not pushed.

delete_patterns (`list[str]` or `str`, *optional*) : If provided, remote files matching any of the patterns will be deleted from the repo.

model_card_kwargs (`dict[str, Any]`, *optional*) : Additional arguments passed to the model card template to customize the model card.

**Returns:**

The url of the commit of your model in the given repository.

Upload model checkpoint to the Hub.

Use `allow_patterns` and `ignore_patterns` to precisely filter which files should be pushed to the hub. Use
`delete_patterns` to delete existing remote files in the same commit. See [upload_folder()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.upload_folder) reference for more
details.

#### save_pretrained[[huggingface_hub.ModelHubMixin.save_pretrained]]

```python
save_pretrained(save_directory: str | pathlib.Path, config: dict | huggingface_hub.hub_mixin.DataclassInstance | None = None, repo_id: str | None = None, push_to_hub: bool = False, model_card_kwargs: dict[str, typing.Any] | None = None, **push_to_hub_kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/hub_mixin.py#L383)

**Parameters:**

save_directory (`str` or `Path`) : Path to directory in which the model weights and configuration will be saved.

config (`dict` or `DataclassInstance`, *optional*) : Model configuration specified as a key/value dictionary or a dataclass instance.

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether or not to push your model to the Huggingface Hub after saving it.

repo_id (`str`, *optional*) : ID of your repository on the Hub. Used only if `push_to_hub=True`. Will default to the folder name if not provided.

model_card_kwargs (`dict[str, Any]`, *optional*) : Additional arguments passed to the model card template to customize the model card.

push_to_hub_kwargs : Additional key word arguments passed along to the [push_to_hub()](/docs/huggingface_hub/v1.27.0/en/package_reference/mixins#huggingface_hub.ModelHubMixin.push_to_hub) method.

**Returns:** `str` or `None`

url of the commit on the Hub if `push_to_hub=True`, `None` otherwise.

Save weights in local directory.

### PyTorch[[huggingface_hub.PyTorchModelHubMixin]]

#### huggingface_hub.PyTorchModelHubMixin[[huggingface_hub.PyTorchModelHubMixin]]

```python
huggingface_hub.PyTorchModelHubMixin(*args, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/hub_mixin.py#L703)

Implementation of [ModelHubMixin](/docs/huggingface_hub/v1.27.0/en/package_reference/mixins#huggingface_hub.ModelHubMixin) to provide model Hub upload/download capabilities to PyTorch models. The model
is set in evaluation mode by default using `model.eval()` (dropout modules are deactivated). To train the model,
you should first set it back in training mode with `model.train()`.

See [ModelHubMixin](/docs/huggingface_hub/v1.27.0/en/package_reference/mixins#huggingface_hub.ModelHubMixin) for more details on how to use the mixin.

Example:

```python
>>> import torch
>>> import torch.nn as nn
>>> from huggingface_hub import PyTorchModelHubMixin

>>> class MyModel(
...         nn.Module,
...         PyTorchModelHubMixin,
...         library_name="keras-nlp",
...         repo_url="https://github.com/keras-team/keras-nlp",
...         paper_url="https://arxiv.org/abs/2304.12244",
...         docs_url="https://keras.io/keras_nlp/",
...         # ^ optional metadata to generate model card
...     ):
...     def __init__(self, hidden_size: int = 512, vocab_size: int = 30000, output_size: int = 4):
...         super().__init__()
...         self.param = nn.Parameter(torch.rand(hidden_size, vocab_size))
...         self.linear = nn.Linear(output_size, vocab_size)

...     def forward(self, x):
...         return self.linear(x + self.param)
>>> model = MyModel(hidden_size=256)

# Save model weights to local directory
>>> model.save_pretrained("my-awesome-model")

# Push model weights to the Hub
>>> model.push_to_hub("my-awesome-model")

# Download and initialize weights from the Hub
>>> model = MyModel.from_pretrained("username/my-awesome-model")
>>> model.hidden_size
256
```

### Fastai[[huggingface_hub.from_pretrained_fastai]]

#### huggingface_hub.from_pretrained_fastai[[huggingface_hub.from_pretrained_fastai]]

```python
huggingface_hub.from_pretrained_fastai(repo_id: str, revision: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/fastai_utils.py#L289)

**Parameters:**

repo_id (`str`) : The location where the pickled fastai.Learner is. It can be either of the two: - Hosted on the Hugging Face Hub. E.g.: 'espejelomar/fatai-pet-breeds-classification' or 'distilgpt2'. You can add a `revision` by appending `@` at the end of `repo_id`. E.g.: `dbmdz/bert-base-german-cased@main`. Revision is the specific model version to use. Since we use a git-based system for storing models and other artifacts on the Hugging Face Hub, it can be a branch name, a tag name, or a commit id. - Hosted locally. `repo_id` would be a directory containing the pickle and a pyproject.toml indicating the fastai and fastcore versions used to build the `fastai.Learner`. E.g.: `./my_model_directory/`.

revision (`str`, *optional*) : Revision at which the repo's files are downloaded. See documentation of `snapshot_download`.

**Returns:**

The `fastai.Learner` model in the `repo_id` repo.

Load pretrained fastai model from the Hub or from a local directory.

#### huggingface_hub.push_to_hub_fastai[[huggingface_hub.push_to_hub_fastai]]

```python
huggingface_hub.push_to_hub_fastai(learner, repo_id: str, commit_message: str = 'Push FastAI model using huggingface_hub.', private: bool | None = None, token: str | None = None, config: dict | None = None, branch: str | None = None, create_pr: bool | None = None, allow_patterns: list[str] | str | None = None, ignore_patterns: list[str] | str | None = None, delete_patterns: list[str] | str | None = None, api_endpoint: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/fastai_utils.py#L334)

**Parameters:**

learner (*Learner*) : The *fastai.Learner' you'd like to push to the Hub.

repo_id (*str*) : The repository id for your model in Hub in the format of "namespace/repo_name". The namespace can be your individual account or an organization to which you have write access (for example, 'stanfordnlp/stanza-de').

commit_message (*str`, *optional*) : Message to commit while pushing. Will default to `"add model"`.

private (*bool*, *optional*) : Whether or not the repository created should be private. If *None* (default), will default to been public except if the organization's default is private.

token (*str*, *optional*) : The Hugging Face account token to use as HTTP bearer authorization for remote files. If `None`, the token will be asked by a prompt.

config (*dict*, *optional*) : Configuration object to be saved alongside the model weights.

branch (*str*, *optional*) : The git branch on which to push the model. This defaults to the default branch as specified in your repository, which defaults to *"main"*.

create_pr (*boolean*, *optional*) : Whether or not to create a Pull Request from *branch* with that commit. Defaults to *False*.

api_endpoint (*str*, *optional*) : The API endpoint to use when pushing the model to the hub.

allow_patterns (*list[str]* or *str*, *optional*) : If provided, only files matching at least one pattern are pushed.

ignore_patterns (*list[str]* or *str*, *optional*) : If provided, files matching any of the patterns are not pushed.

delete_patterns (*list[str]* or *str*, *optional*) : If provided, remote files matching any of the patterns will be deleted from the repo.

**Returns:**

The url of the commit of your model in the given repository.

Upload learner checkpoint files to the Hub.

Use *allow_patterns* and *ignore_patterns* to precisely filter which files should be pushed to the hub. Use
*delete_patterns* to delete existing remote files in the same commit. See [*upload_folder*] reference for more
details.

> [!TIP]
> Raises the following error:
>
>     - [*ValueError*](https://docs.python.org/3/library/exceptions.html#ValueError)
>       if the user is not log on to the Hugging Face Hub.
