# Repository Cards

The huggingface_hub library provides a Python interface to create, share, and update Model/Dataset Cards.
Visit the [dedicated documentation page](https://huggingface.co/docs/hub/models-cards) for a deeper view of what
Model Cards on the Hub are, and how they work under the hood. You can also check out our [Model Cards guide](../how-to-model-cards) to
get a feel for how you would use these utilities in your own projects.

## Repo Card[[huggingface_hub.RepoCard]]

The `RepoCard` object is the parent class of [ModelCard](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.ModelCard), [DatasetCard](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.DatasetCard) and `SpaceCard`.

#### huggingface_hub.RepoCard[[huggingface_hub.RepoCard]]

```python
huggingface_hub.RepoCard(content: str, ignore_metadata_errors: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L37)

#### __init__[[huggingface_hub.RepoCard.__init__]]

```python
__init__(content: str, ignore_metadata_errors: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L42)

**Parameters:**

content (`str`) : The content of the Markdown file.

Initialize a RepoCard from string content. The content should be a
Markdown file with a YAML block at the beginning and a Markdown body.

Example:
```python
>>> from huggingface_hub.repocard import RepoCard
>>> text = '''
... ---
... language: en
... license: mit
... ---
...
... # My repo
... '''
>>> card = RepoCard(text)
>>> card.data.to_dict()
{'language': 'en', 'license': 'mit'}
>>> card.text
'\n# My repo\n'

```

> [!TIP]
> Raises the following error:
>
>     - [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError)
>       when the content of the repo card metadata is not a dictionary.

#### from_template[[huggingface_hub.RepoCard.from_template]]

```python
from_template(card_data: CardData, template_path: str | None = None, template_str: str | None = None, **template_kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L289)

**Parameters:**

card_data (`huggingface_hub.CardData`) : A huggingface_hub.CardData instance containing the metadata you want to include in the YAML header of the repo card on the Hugging Face Hub.

template_path (`str`, *optional*) : A path to a markdown file with optional Jinja template variables that can be filled in with `template_kwargs`. Defaults to the default template.

template_str (`str`, *optional*) : A raw Jinja template string with optional variables. Used when neither `template_path` nor the default template is appropriate. Ignored if `template_path` is also provided.

**Returns:** [huggingface_hub.repocard.RepoCard](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.RepoCard)

A RepoCard instance with the specified card data and content from the
template.

Initialize a RepoCard from a template. By default, it uses the default template.

Templates are Jinja2 templates that can be customized by passing keyword arguments.

#### load[[huggingface_hub.RepoCard.load]]

```python
load(repo_id_or_path: str | pathlib.Path, repo_type: str | None = None, token: str | None = None, ignore_metadata_errors: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L135)

**Parameters:**

repo_id_or_path (`Union[str, Path]`) : The repo ID associated with a Hugging Face Hub repo or a local filepath.

repo_type (`str`, *optional*) : The type of Hugging Face repo to push to. Defaults to None, which will use "model". Other options are "dataset" and "space". Not used when loading from a local filepath. If this is called from a child class, the default value will be the child class's `repo_type`.

token (`str`, *optional*) : Authentication token, obtained with `huggingface_hub.HfApi.login` method. Will default to the stored token.

ignore_metadata_errors (`str`) : If True, errors while parsing the metadata section will be ignored. Some information might be lost during the process. Use it at your own risk.

**Returns:** [huggingface_hub.repocard.RepoCard](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.RepoCard)

The RepoCard (or subclass) initialized from the repo's
README.md file or filepath.

Initialize a RepoCard from a Hugging Face Hub repo's README.md or a local filepath.

Example:
```python
>>> from huggingface_hub.repocard import RepoCard
>>> card = RepoCard.load("nateraw/food")
>>> assert card.data.tags == ["generated_from_trainer", "image-classification", "pytorch"]

```

#### push_to_hub[[huggingface_hub.RepoCard.push_to_hub]]

```python
push_to_hub(repo_id: str, token: str | None = None, repo_type: str | None = None, commit_message: str | None = None, commit_description: str | None = None, revision: str | None = None, create_pr: bool | None = None, parent_commit: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L226)

**Parameters:**

repo_id (`str`) : The repo ID of the Hugging Face Hub repo to push to. Example: "nateraw/food".

token (`str`, *optional*) : Authentication token, obtained with `huggingface_hub.HfApi.login` method. Will default to the stored token.

repo_type (`str`, *optional*, defaults to "model") : The type of Hugging Face repo to push to. Options are "model", "dataset", and "space". If this function is called by a child class, it will default to the child class's `repo_type`.

commit_message (`str`, *optional*) : The summary / title / first line of the generated commit.

commit_description (`str`, *optional*) : The description of the generated commit.

revision (`str`, *optional*) : The git revision to commit from. Defaults to the head of the `"main"` branch.

create_pr (`bool`, *optional*) : Whether or not to create a Pull Request with this commit. Defaults to `False`.

parent_commit (`str`, *optional*) : The OID / SHA of the parent commit, as a hexadecimal string. Shorthands (7 first characters) are also supported. If specified and `create_pr` is `False`, the commit will fail if `revision` does not point to `parent_commit`. If specified and `create_pr` is `True`, the pull request will be created from `parent_commit`. Specifying `parent_commit` ensures the repo has not changed before committing the changes, and can be especially useful if the repo is updated / committed too concurrently.

**Returns:** `str`

URL of the commit which updated the card metadata.

Push a RepoCard to a Hugging Face Hub repo.

#### save[[huggingface_hub.RepoCard.save]]

```python
save(filepath: pathlib.Path | str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L115)

**Parameters:**

filepath (`Union[Path, str]`) : Filepath to the markdown file to save.

Save a RepoCard to a file.

Example:
```python
>>> from huggingface_hub.repocard import RepoCard
>>> card = RepoCard("---\nlanguage: en\n---\n# This is a test repo card")
>>> card.save("/tmp/test.md")

```

#### validate[[huggingface_hub.RepoCard.validate]]

```python
validate(repo_type: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L189)

**Parameters:**

repo_type (`str`, *optional*, defaults to "model") : The type of Hugging Face repo to push to. Options are "model", "dataset", and "space". If this function is called from a child class, the default will be the child class's `repo_type`.

Validates card against Hugging Face Hub's card validation logic.
Using this function requires access to the internet, so it is only called
internally by [huggingface_hub.repocard.RepoCard.push_to_hub()](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.RepoCard.push_to_hub).

> [!TIP]
> Raises the following errors:
>
>     - [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError)
>       if the card fails validation checks.
>     - [`HTTPError`](https://requests.readthedocs.io/en/latest/api/#requests.HTTPError)
>       if the request to the Hub API fails for any other reason.

## Card Data[[huggingface_hub.CardData]]

The [CardData](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.CardData) object is the parent class of [ModelCardData](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.ModelCardData) and [DatasetCardData](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.DatasetCardData).

#### huggingface_hub.CardData[[huggingface_hub.CardData]]

```python
huggingface_hub.CardData(ignore_metadata_errors: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L165)

Structure containing metadata from a RepoCard.

[CardData](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.CardData) is the parent class of [ModelCardData](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.ModelCardData) and [DatasetCardData](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.DatasetCardData).

Metadata can be exported as a dictionary or YAML. Export can be customized to alter the representation of the data
(example: flatten evaluation results). `CardData` behaves as a dictionary (can get, pop, set values) but do not
inherit from `dict` to allow this export step.

#### get[[huggingface_hub.CardData.get]]

```python
get(key: str, default: typing.Any = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L228)

Get value for a given metadata key.

#### pop[[huggingface_hub.CardData.pop]]

```python
pop(key: str, default: typing.Any = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L233)

Pop value for a given metadata key.

#### to_dict[[huggingface_hub.CardData.to_dict]]

```python
to_dict()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L178)

**Returns:** `dict`

CardData represented as a dictionary ready to be dumped to a YAML
block for inclusion in a README.md file.

Converts CardData to a dict.

#### to_yaml[[huggingface_hub.CardData.to_yaml]]

```python
to_yaml(line_break = None, original_order: list[str] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L198)

**Parameters:**

line_break (str, *optional*) : The line break to use when dumping to yaml.

original_order (`list[str]`, *optional*) : If provided, reorder the metadata fields to match this list before dumping. Any keys not in `original_order` are appended after the listed keys, preserving their existing relative order. Useful for round-tripping a YAML block without shuffling its keys.

**Returns:** `str`

CardData represented as a YAML block.

Dumps CardData to a YAML block for inclusion in a README.md file.

## Model Cards

### ModelCard[[huggingface_hub.ModelCard]]

#### huggingface_hub.ModelCard[[huggingface_hub.ModelCard]]

```python
huggingface_hub.ModelCard(content: str, ignore_metadata_errors: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L336)

#### from_template[[huggingface_hub.ModelCard.from_template]]

```python
from_template(card_data: ModelCardData, template_path: str | None = None, template_str: str | None = None, **template_kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L341)

**Parameters:**

card_data (`huggingface_hub.ModelCardData`) : A huggingface_hub.ModelCardData instance containing the metadata you want to include in the YAML header of the model card on the Hugging Face Hub.

template_path (`str`, *optional*) : A path to a markdown file with optional Jinja template variables that can be filled in with `template_kwargs`. Defaults to the default template.

template_str (`str`, *optional*) : A raw Jinja template string with optional variables. Used when neither `template_path` nor the default template is appropriate. Ignored if `template_path` is also provided.

**Returns:** [huggingface_hub.ModelCard](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.ModelCard)

A ModelCard instance with the specified card data and content from the
template.

Initialize a ModelCard from a template. By default, it uses the default template, which can be found here:
https://github.com/huggingface/huggingface_hub/blob/main/src/huggingface_hub/templates/modelcard_template.md

Templates are Jinja2 templates that can be customized by passing keyword arguments.

Example:
```python
>>> from huggingface_hub import ModelCard, ModelCardData, EvalResult

>>> # Using the Default Template
>>> card_data = ModelCardData(
...     language='en',
...     license='mit',
...     library_name='timm',
...     tags=['image-classification', 'resnet'],
...     datasets=['beans'],
...     metrics=['accuracy'],
... )
>>> card = ModelCard.from_template(
...     card_data,
...     model_description='This model does x + y...'
... )

>>> # Including Evaluation Results
>>> card_data = ModelCardData(
...     language='en',
...     tags=['image-classification', 'resnet'],
...     eval_results=[
...         EvalResult(
...             task_type='image-classification',
...             dataset_type='beans',
...             dataset_name='Beans',
...             metric_type='accuracy',
...             metric_value=0.9,
...         ),
...     ],
...     model_name='my-cool-model',
... )
>>> card = ModelCard.from_template(card_data)

>>> # Using a Custom Template
>>> card_data = ModelCardData(
...     language='en',
...     tags=['image-classification', 'resnet']
... )
>>> card = ModelCard.from_template(
...     card_data=card_data,
...     template_path='./src/huggingface_hub/templates/modelcard_template.md',
...     custom_template_var='custom value',  # will be replaced in template if it exists
... )

```

### ModelCardData[[huggingface_hub.ModelCardData]]

#### huggingface_hub.ModelCardData[[huggingface_hub.ModelCardData]]

```python
huggingface_hub.ModelCardData(base_model: str | list[str] | None = None, datasets: str | list[str] | None = None, eval_results: list[huggingface_hub.repocard_data.EvalResult] | None = None, language: str | list[str] | None = None, library_name: str | None = None, license: str | None = None, license_name: str | None = None, license_link: str | None = None, metrics: list[str] | None = None, model_name: str | None = None, pipeline_tag: str | None = None, tags: list[str] | None = None, ignore_metadata_errors: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L271)

**Parameters:**

base_model (`str` or `list[str]`, *optional*) : The identifier of the base model from which the model derives. This is applicable for example if your model is a fine-tune or adapter of an existing model. The value must be the ID of a model on the Hub (or a list of IDs if your model derives from multiple models). Defaults to None.

datasets (`Union[str, list[str]]`, *optional*) : Dataset or list of datasets that were used to train this model. Should be a dataset ID found on https://hf.co/datasets. Defaults to None.

eval_results (`Union[list[EvalResult], EvalResult]`, *optional*) : List of `huggingface_hub.EvalResult` that define evaluation results of the model. If provided, `model_name` is used to as a name on PapersWithCode's leaderboards. Defaults to `None`.

language (`Union[str, list[str]]`, *optional*) : Language of model's training data or metadata. It must be an ISO 639-1, 639-2 or 639-3 code (two/three letters), or a special value like "code", "multilingual". Defaults to `None`.

library_name (`str`, *optional*) : Name of library used by this model. Example: keras or any library from https://github.com/huggingface/huggingface.js/blob/main/packages/tasks/src/model-libraries.ts. Defaults to None.

license (`str`, *optional*) : License of this model. Example: apache-2.0 or any license from https://huggingface.co/docs/hub/repositories-licenses. Defaults to None.

license_name (`str`, *optional*) : Name of the license of this model. Defaults to None. To be used in conjunction with `license_link`. Common licenses (Apache-2.0, MIT, CC-BY-SA-4.0) do not need a name. In that case, use `license` instead.

license_link (`str`, *optional*) : Link to the license of this model. Defaults to None. To be used in conjunction with `license_name`. Common licenses (Apache-2.0, MIT, CC-BY-SA-4.0) do not need a link. In that case, use `license` instead.

metrics (`list[str]`, *optional*) : List of metrics used to evaluate this model. Should be a metric name that can be found at https://hf.co/metrics. Example: 'accuracy'. Defaults to None.

model_name (`str`, *optional*) : A name for this model. It is used along with `eval_results` to construct the `model-index` within the card's metadata. The name you supply here is what will be used on PapersWithCode's leaderboards. If None is provided then the repo name is used as a default. Defaults to None.

pipeline_tag (`str`, *optional*) : The pipeline tag associated with the model. Example: "text-classification".

tags (`list[str]`, *optional*) : List of tags to add to your model that can be used when filtering on the Hugging Face Hub. Defaults to None.

ignore_metadata_errors (`str`) : If True, errors while parsing the metadata section will be ignored. Some information might be lost during the process. Use it at your own risk.

kwargs (`dict`, *optional*) : Additional metadata that will be added to the model card. Defaults to None.

Model Card Metadata that is used by Hugging Face Hub when included at the top of your README.md

Example:
```python
>>> from huggingface_hub import ModelCardData
>>> card_data = ModelCardData(
...     language="en",
...     license="mit",
...     library_name="timm",
...     tags=['image-classification', 'resnet'],
... )
>>> card_data.to_dict()
{'language': 'en', 'license': 'mit', 'library_name': 'timm', 'tags': ['image-classification', 'resnet']}

```

## Dataset Cards

Dataset cards are also known as Data Cards in the ML Community.

### DatasetCard[[huggingface_hub.DatasetCard]]

#### huggingface_hub.DatasetCard[[huggingface_hub.DatasetCard]]

```python
huggingface_hub.DatasetCard(content: str, ignore_metadata_errors: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L420)

#### from_template[[huggingface_hub.DatasetCard.from_template]]

```python
from_template(card_data: DatasetCardData, template_path: str | None = None, template_str: str | None = None, **template_kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L425)

**Parameters:**

card_data (`huggingface_hub.DatasetCardData`) : A huggingface_hub.DatasetCardData instance containing the metadata you want to include in the YAML header of the dataset card on the Hugging Face Hub.

template_path (`str`, *optional*) : A path to a markdown file with optional Jinja template variables that can be filled in with `template_kwargs`. Defaults to the default template.

template_str (`str`, *optional*) : A raw Jinja template string with optional variables. Used when neither `template_path` nor the default template is appropriate. Ignored if `template_path` is also provided.

**Returns:** [huggingface_hub.DatasetCard](/docs/huggingface_hub/v1.27.0/en/package_reference/cards#huggingface_hub.DatasetCard)

A DatasetCard instance with the specified card data and content from the
template.

Initialize a DatasetCard from a template. By default, it uses the default template, which can be found here:
https://github.com/huggingface/huggingface_hub/blob/main/src/huggingface_hub/templates/datasetcard_template.md

Templates are Jinja2 templates that can be customized by passing keyword arguments.

Example:
```python
>>> from huggingface_hub import DatasetCard, DatasetCardData

>>> # Using the Default Template
>>> card_data = DatasetCardData(
...     language='en',
...     license='mit',
...     annotations_creators='crowdsourced',
...     task_categories=['text-classification'],
...     task_ids=['sentiment-classification', 'text-scoring'],
...     multilinguality='monolingual',
...     pretty_name='My Text Classification Dataset',
... )
>>> card = DatasetCard.from_template(
...     card_data,
...     pretty_name=card_data.pretty_name,
... )

>>> # Using a Custom Template
>>> card_data = DatasetCardData(
...     language='en',
...     license='mit',
... )
>>> card = DatasetCard.from_template(
...     card_data=card_data,
...     template_path='./src/huggingface_hub/templates/datasetcard_template.md',
...     custom_template_var='custom value',  # will be replaced in template if it exists
... )

```

### DatasetCardData[[huggingface_hub.DatasetCardData]]

#### huggingface_hub.DatasetCardData[[huggingface_hub.DatasetCardData]]

```python
huggingface_hub.DatasetCardData(language: str | list[str] | None = None, license: str | list[str] | None = None, annotations_creators: str | list[str] | None = None, language_creators: str | list[str] | None = None, multilinguality: str | list[str] | None = None, size_categories: str | list[str] | None = None, source_datasets: list[str] | None = None, task_categories: str | list[str] | None = None, task_ids: str | list[str] | None = None, paperswithcode_id: str | None = None, pretty_name: str | None = None, train_eval_index: dict | None = None, config_names: str | list[str] | None = None, ignore_metadata_errors: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L400)

**Parameters:**

language (`list[str]`, *optional*) : Language of dataset's data or metadata. It must be an ISO 639-1, 639-2 or 639-3 code (two/three letters), or a special value like "code", "multilingual".

license (`Union[str, list[str]]`, *optional*) : License(s) of this dataset. Example: apache-2.0 or any license from https://huggingface.co/docs/hub/repositories-licenses.

annotations_creators (`Union[str, list[str]]`, *optional*) : How the annotations for the dataset were created. Options are: 'found', 'crowdsourced', 'expert-generated', 'machine-generated', 'no-annotation', 'other'.

language_creators (`Union[str, list[str]]`, *optional*) : How the text-based data in the dataset was created. Options are: 'found', 'crowdsourced', 'expert-generated', 'machine-generated', 'other'

multilinguality (`Union[str, list[str]]`, *optional*) : Whether the dataset is multilingual. Options are: 'monolingual', 'multilingual', 'translation', 'other'.

size_categories (`Union[str, list[str]]`, *optional*) : The number of examples in the dataset. Options are: 'n<1K', '1K1T', and 'other'.

source_datasets (`list[str]]`, *optional*) : Indicates whether the dataset is an original dataset or extended from another existing dataset. Options are: 'original' and 'extended'.

task_categories (`Union[str, list[str]]`, *optional*) : What categories of task does the dataset support?

task_ids (`Union[str, list[str]]`, *optional*) : What specific tasks does the dataset support?

paperswithcode_id (`str`, *optional*) : ID of the dataset on PapersWithCode.

pretty_name (`str`, *optional*) : A more human-readable name for the dataset. (ex. "Cats vs. Dogs")

train_eval_index (`dict`, *optional*) : A dictionary that describes the necessary spec for doing evaluation on the Hub. If not provided, it will be gathered from the 'train-eval-index' key of the kwargs.

config_names (`Union[str, list[str]]`, *optional*) : A list of the available dataset configs for the dataset.

Dataset Card Metadata that is used by Hugging Face Hub when included at the top of your README.md

## Space Cards

### SpaceCard[[huggingface_hub.SpaceCard]]

#### huggingface_hub.SpaceCard[[huggingface_hub.SpaceCard]]

```python
huggingface_hub.SpaceCard(content: str, ignore_metadata_errors: bool = False)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L488)

### SpaceCardData[[huggingface_hub.SpaceCardData]]

#### huggingface_hub.SpaceCardData[[huggingface_hub.SpaceCardData]]

```python
huggingface_hub.SpaceCardData(title: str | None = None, sdk: str | None = None, sdk_version: str | None = None, python_version: str | None = None, app_file: str | None = None, app_port: int | None = None, license: str | None = None, duplicated_from: str | None = None, models: list[str] | None = None, datasets: list[str] | None = None, tags: list[str] | None = None, ignore_metadata_errors: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L480)

**Parameters:**

title (`str`, *optional*) : Title of the Space.

sdk (`str`, *optional*) : SDK of the Space (one of `gradio`, `streamlit`, `docker`, or `static`).

sdk_version (`str`, *optional*) : Version of the used SDK (if Gradio/Streamlit sdk).

python_version (`str`, *optional*) : Python version used in the Space (if Gradio/Streamlit sdk).

app_file (`str`, *optional*) : Path to your main application file (which contains either gradio or streamlit Python code, or static html code). Path is relative to the root of the repository.

app_port (`str`, *optional*) : Port on which your application is running. Used only if sdk is `docker`.

license (`str`, *optional*) : License of this model. Example: apache-2.0 or any license from https://huggingface.co/docs/hub/repositories-licenses.

duplicated_from (`str`, *optional*) : ID of the original Space if this is a duplicated Space.

models (list`str`, *optional*) : List of models related to this Space. Should be a dataset ID found on https://hf.co/models.

datasets (`list[str]`, *optional*) : List of datasets related to this Space. Should be a dataset ID found on https://hf.co/datasets.

tags (`list[str]`, *optional*) : List of tags to add to your Space that can be used when filtering on the Hub.

ignore_metadata_errors (`str`) : If True, errors while parsing the metadata section will be ignored. Some information might be lost during the process. Use it at your own risk.

kwargs (`dict`, *optional*) : Additional metadata that will be added to the space card.

Space Card Metadata that is used by Hugging Face Hub when included at the top of your README.md

To get an exhaustive reference of Spaces configuration, please visit https://huggingface.co/docs/hub/spaces-config-reference#spaces-configuration-reference.

Example:
```python
>>> from huggingface_hub import SpaceCardData
>>> card_data = SpaceCardData(
...     title="Dreambooth Training",
...     license="mit",
...     sdk="gradio",
...     duplicated_from="multimodalart/dreambooth-training"
... )
>>> card_data.to_dict()
{'title': 'Dreambooth Training', 'sdk': 'gradio', 'license': 'mit', 'duplicated_from': 'multimodalart/dreambooth-training'}
```

## Utilities

### EvalResult[[huggingface_hub.EvalResult]]

#### huggingface_hub.EvalResult[[huggingface_hub.EvalResult]]

```python
huggingface_hub.EvalResult(task_type: str, dataset_type: str, dataset_name: str, metric_type: str, metric_value: typing.Any, task_name: str | None = None, dataset_config: str | None = None, dataset_split: str | None = None, dataset_revision: str | None = None, dataset_args: dict[str, typing.Any] | None = None, metric_name: str | None = None, metric_config: str | None = None, metric_args: dict[str, typing.Any] | None = None, verified: bool | None = None, verify_token: str | None = None, source_name: str | None = None, source_url: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L13)

**Parameters:**

task_type (`str`) : The task identifier. Example: "image-classification".

dataset_type (`str`) : The dataset identifier. Example: "common_voice". Use dataset id from https://hf.co/datasets.

dataset_name (`str`) : A pretty name for the dataset. Example: "Common Voice (French)".

metric_type (`str`) : The metric identifier. Example: "wer". Use metric id from https://hf.co/metrics.

metric_value (`Any`) : The metric value. Example: 0.9 or "20.0 ± 1.2".

task_name (`str`, *optional*) : A pretty name for the task. Example: "Speech Recognition".

dataset_config (`str`, *optional*) : The name of the dataset configuration used in `load_dataset()`. Example: fr in `load_dataset("common_voice", "fr")`. See the `datasets` docs for more info: https://hf.co/docs/datasets/package_reference/loading_methods#datasets.load_dataset.name

dataset_split (`str`, *optional*) : The split used in `load_dataset()`. Example: "test".

dataset_revision (`str`, *optional*) : The revision (AKA Git Sha) of the dataset used in `load_dataset()`. Example: 5503434ddd753f426f4b38109466949a1217c2bb

dataset_args (`dict[str, Any]`, *optional*) : The arguments passed during `Metric.compute()`. Example for `bleu`: `{"max_order": 4}`

metric_name (`str`, *optional*) : A pretty name for the metric. Example: "Test WER".

metric_config (`str`, *optional*) : The name of the metric configuration used in `load_metric()`. Example: bleurt-large-512 in `load_metric("bleurt", "bleurt-large-512")`. See the `datasets` docs for more info: https://huggingface.co/docs/datasets/v2.1.0/en/loading#load-configurations

metric_args (`dict[str, Any]`, *optional*) : The arguments passed during `Metric.compute()`. Example for `bleu`: max_order: 4

verified (`bool`, *optional*) : Indicates whether the metrics originate from Hugging Face's [evaluation service](https://huggingface.co/spaces/autoevaluate/model-evaluator) or not. Automatically computed by Hugging Face, do not set.

verify_token (`str`, *optional*) : A JSON Web Token that is used to verify whether the metrics originate from Hugging Face's [evaluation service](https://huggingface.co/spaces/autoevaluate/model-evaluator) or not.

source_name (`str`, *optional*) : The name of the source of the evaluation result. Example: "Open LLM Leaderboard".

source_url (`str`, *optional*) : The URL of the source of the evaluation result. Example: "https://huggingface.co/spaces/open-llm-leaderboard/open_llm_leaderboard".

Flattened representation of individual evaluation results found in model-index of Model Cards.

For more information on the model-index spec, see https://github.com/huggingface/hub-docs/blob/main/modelcard.md?plain=1.

#### is_equal_except_value[[huggingface_hub.EvalResult.is_equal_except_value]]

```python
is_equal_except_value(other: EvalResult)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L145)

Return True if `self` and `other` describe exactly the same metric but with a
different value.

### model_index_to_eval_results[[huggingface_hub.repocard_data.model_index_to_eval_results]]

#### huggingface_hub.repocard_data.model_index_to_eval_results[[huggingface_hub.repocard_data.model_index_to_eval_results]]

```python
huggingface_hub.repocard_data.model_index_to_eval_results(model_index: list)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L561)

**Parameters:**

model_index (`list[dict[str, Any]]`) : A model index data structure, likely coming from a README.md file on the Hugging Face Hub.

**Returns:** model_name (`str`)

The name of the model as found in the model index. This is used as the
identifier for the model on leaderboards like PapersWithCode.
eval_results (`list[EvalResult]`):
A list of `huggingface_hub.EvalResult` objects containing the metrics
reported in the provided model_index.

Takes in a model index and returns the model name and a list of `huggingface_hub.EvalResult` objects.

A detailed spec of the model index can be found here:
https://github.com/huggingface/hub-docs/blob/main/modelcard.md?plain=1

Example:
```python
>>> from huggingface_hub.repocard_data import model_index_to_eval_results
>>> # Define a minimal model index
>>> model_index = [
...     {
...         "name": "my-cool-model",
...         "results": [
...             {
...                 "task": {
...                     "type": "image-classification"
...                 },
...                 "dataset": {
...                     "type": "beans",
...                     "name": "Beans"
...                 },
...                 "metrics": [
...                     {
...                         "type": "accuracy",
...                         "value": 0.9
...                     }
...                 ]
...             }
...         ]
...     }
... ]
>>> model_name, eval_results = model_index_to_eval_results(model_index)
>>> model_name
'my-cool-model'
>>> eval_results[0].task_type
'image-classification'
>>> eval_results[0].metric_type
'accuracy'

```

### eval_results_to_model_index[[huggingface_hub.repocard_data.eval_results_to_model_index]]

#### huggingface_hub.repocard_data.eval_results_to_model_index[[huggingface_hub.repocard_data.eval_results_to_model_index]]

```python
huggingface_hub.repocard_data.eval_results_to_model_index(model_name: str, eval_results: list)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard_data.py#L677)

**Parameters:**

model_name (`str`) : Name of the model (ex. "my-cool-model"). This is used as the identifier for the model on leaderboards like PapersWithCode.

eval_results (`list[EvalResult]`) : List of `huggingface_hub.EvalResult` objects containing the metrics to be reported in the model-index.

**Returns:** model_index (`list[dict[str, Any]]`)

The eval_results converted to a model-index.

Takes in given model name and list of `huggingface_hub.EvalResult` and returns a
valid model-index that will be compatible with the format expected by the
Hugging Face Hub.

Example:
```python
>>> from huggingface_hub.repocard_data import eval_results_to_model_index, EvalResult
>>> # Define minimal eval_results
>>> eval_results = [
...     EvalResult(
...         task_type="image-classification",  # Required
...         dataset_type="beans",  # Required
...         dataset_name="Beans",  # Required
...         metric_type="accuracy",  # Required
...         metric_value=0.9,  # Required
...     )
... ]
>>> eval_results_to_model_index("my-cool-model", eval_results)
[{'name': 'my-cool-model', 'results': [{'task': {'type': 'image-classification'}, 'dataset': {'name': 'Beans', 'type': 'beans'}, 'metrics': [{'type': 'accuracy', 'value': 0.9}]}]}]

```

### metadata_eval_result[[huggingface_hub.metadata_eval_result]]

#### huggingface_hub.metadata_eval_result[[huggingface_hub.metadata_eval_result]]

```python
huggingface_hub.metadata_eval_result(model_pretty_name: str, task_pretty_name: str, task_id: str, metrics_pretty_name: str, metrics_id: str, metrics_value: typing.Any, dataset_pretty_name: str, dataset_id: str, metrics_config: str | None = None, metrics_verified: bool = False, dataset_config: str | None = None, dataset_split: str | None = None, dataset_revision: str | None = None, metrics_verification_token: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L560)

**Parameters:**

model_pretty_name (`str`) : The name of the model in natural language.

task_pretty_name (`str`) : The name of a task in natural language.

task_id (`str`) : Example: automatic-speech-recognition. A task id.

metrics_pretty_name (`str`) : A name for the metric in natural language. Example: Test WER.

metrics_id (`str`) : Example: wer. A metric id from https://hf.co/metrics.

metrics_value (`Any`) : The value from the metric. Example: 20.0 or "20.0 ± 1.2".

dataset_pretty_name (`str`) : The name of the dataset in natural language.

dataset_id (`str`) : Example: common_voice. A dataset id from https://hf.co/datasets.

metrics_config (`str`, *optional*) : The name of the metric configuration used in `load_metric()`. Example: bleurt-large-512 in `load_metric("bleurt", "bleurt-large-512")`.

metrics_verified (`bool`, *optional*, defaults to `False`) : Indicates whether the metrics originate from Hugging Face's [evaluation service](https://huggingface.co/spaces/autoevaluate/model-evaluator) or not. Automatically computed by Hugging Face, do not set.

dataset_config (`str`, *optional*) : Example: fr. The name of the dataset configuration used in `load_dataset()`.

dataset_split (`str`, *optional*) : Example: test. The name of the dataset split used in `load_dataset()`.

dataset_revision (`str`, *optional*) : Example: 5503434ddd753f426f4b38109466949a1217c2bb. The name of the dataset dataset revision used in `load_dataset()`.

metrics_verification_token (`bool`, *optional*) : A JSON Web Token that is used to verify whether the metrics originate from Hugging Face's [evaluation service](https://huggingface.co/spaces/autoevaluate/model-evaluator) or not.

**Returns:** `dict`

a metadata dict with the result from a model evaluated on a dataset.

Creates a metadata dict with the result from a model evaluated on a dataset.

Example:
```python
>>> from huggingface_hub import metadata_eval_result
>>> results = metadata_eval_result(
...         model_pretty_name="RoBERTa fine-tuned on ReactionGIF",
...         task_pretty_name="Text Classification",
...         task_id="text-classification",
...         metrics_pretty_name="Accuracy",
...         metrics_id="accuracy",
...         metrics_value=0.2662102282047272,
...         dataset_pretty_name="ReactionJPEG",
...         dataset_id="julien-c/reactionjpeg",
...         dataset_config="default",
...         dataset_split="test",
... )
>>> results == {
...     'model-index': [
...         {
...             'name': 'RoBERTa fine-tuned on ReactionGIF',
...             'results': [
...                 {
...                     'task': {
...                         'type': 'text-classification',
...                         'name': 'Text Classification'
...                     },
...                     'dataset': {
...                         'name': 'ReactionJPEG',
...                         'type': 'julien-c/reactionjpeg',
...                         'config': 'default',
...                         'split': 'test'
...                     },
...                     'metrics': [
...                         {
...                             'type': 'accuracy',
...                             'value': 0.2662102282047272,
...                             'name': 'Accuracy',
...                             'verified': False
...                         }
...                     ]
...                 }
...             ]
...         }
...     ]
... }
True

```

### metadata_update[[huggingface_hub.metadata_update]]

#### huggingface_hub.metadata_update[[huggingface_hub.metadata_update]]

```python
huggingface_hub.metadata_update(repo_id: str, metadata: dict, repo_type: str | None = None, overwrite: bool = False, token: str | None = None, commit_message: str | None = None, commit_description: str | None = None, revision: str | None = None, create_pr: bool = False, parent_commit: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/repocard.py#L688)

**Parameters:**

repo_id (`str`) : The name of the repository.

metadata (`dict`) : A dictionary containing the metadata to be updated.

repo_type (`str`, *optional*) : Set to `"dataset"` or `"space"` if updating to a dataset or space, `None` or `"model"` if updating to a model. Default is `None`.

overwrite (`bool`, *optional*, defaults to `False`) : If set to `True` an existing field can be overwritten, otherwise attempting to overwrite an existing field will cause an error.

token (`str`, *optional*) : The Hugging Face authentication token.

commit_message (`str`, *optional*) : The summary / title / first line of the generated commit. Defaults to `f"Update metadata with huggingface_hub"`

commit_description (`str` *optional*) : The description of the generated commit

revision (`str`, *optional*) : The git revision to commit from. Defaults to the head of the `"main"` branch.

create_pr (`boolean`, *optional*) : Whether or not to create a Pull Request from `revision` with that commit. Defaults to `False`.

parent_commit (`str`, *optional*) : The OID / SHA of the parent commit, as a hexadecimal string. Shorthands (7 first characters) are also supported. If specified and `create_pr` is `False`, the commit will fail if `revision` does not point to `parent_commit`. If specified and `create_pr` is `True`, the pull request will be created from `parent_commit`. Specifying `parent_commit` ensures the repo has not changed before committing the changes, and can be especially useful if the repo is updated / committed too concurrently.

**Returns:** `str`

URL of the commit which updated the card metadata.

Updates the metadata in the README.md of a repository on the Hugging Face Hub.
If the README.md file doesn't exist yet, a new one is created with metadata and
the default ModelCard or DatasetCard template. For `space` repo, an error is thrown
as a Space cannot exist without a `README.md` file.

Example:
```python
>>> from huggingface_hub import metadata_update
>>> metadata = {'model-index': [{'name': 'RoBERTa fine-tuned on ReactionGIF',
...             'results': [{'dataset': {'name': 'ReactionGIF',
...                                      'type': 'julien-c/reactiongif'},
...                           'metrics': [{'name': 'Recall',
...                                        'type': 'recall',
...                                        'value': 0.7762102282047272}],
...                          'task': {'name': 'Text Classification',
...                                   'type': 'text-classification'}}]}]}
>>> url = metadata_update("hf-internal-testing/reactiongif-roberta-card", metadata)

```
