# Tasks

## LightevalTask
### LightevalTaskConfig[[lighteval.tasks.lighteval_task.LightevalTaskConfig]]
#### lighteval.tasks.lighteval_task.LightevalTaskConfig[[lighteval.tasks.lighteval_task.LightevalTaskConfig]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/lighteval_task.py#L49)

Configuration dataclass for a LightevalTask.

This class stores all the configuration parameters needed to define and run
an evaluation task, including dataset information, prompt formatting,
evaluation metrics, and generation parameters.

Dataset Configuration:
hf_revision (str | None, optional): Specific dataset revision to use.
Defaults to None (latest).
hf_filter (Callable[[dict], bool] | None, optional): Filter function to
apply to dataset items. Defaults to None.
hf_avail_splits (ListLike[str], optional): Available dataset splits.
Defaults to ["train", "validation", "test"].

Evaluation Splits:
evaluation_splits (ListLike[str], optional): Dataset splits to use for
evaluation. Defaults to ["validation"].
few_shots_split (str | None, optional): Split to sample few-shot examples
from. Defaults to None.
few_shots_select (str | None, optional): Method for selecting few-shot
examples. Defaults to None.

Generation Parameters:
generation_size (int | None, optional): Maximum token length for generated
text. Defaults to None.
generation_grammar (TextGenerationInputGrammarType | None, optional): Grammar
for structured text generation. Only available for TGI and Inference
Endpoint models. Defaults to None.
stop_sequence (ListLike[str] | None, optional): Sequences that stop text
generation. Defaults to None.
num_samples (list[int] | None, optional): Number of samples to generate
per input. Defaults to None.

Task Configuration:
version (int, optional): Task version number. Increment when dataset or
prompt changes. Defaults to 0.
num_fewshots (int, optional): Number of few-shot examples to include.
Defaults to 0.
truncate_fewshots (bool, optional): Whether to truncate few-shot examples.
Defaults to False.
must_remove_duplicate_docs (bool, optional): Whether to remove duplicate
documents. Defaults to False.

Document Tracking:
original_num_docs (int, optional): Total number of documents in the task.
Defaults to -1.
effective_num_docs (int, optional): Number of documents actually used
in evaluation. Defaults to -1.

**Parameters:**

name (str) : Short name of the evaluation task.

prompt_function (Callable[[dict, str], Doc]) : Function that converts dataset row to Doc objects for evaluation. Takes a dataset row dict and task name as input.

hf_repo (str) : HuggingFace Hub repository path containing the evaluation dataset.

hf_subset (str) : Dataset subset/configuration name to use for this task.

metrics (ListLike[Metric | Metrics]) : List of metrics or metric enums to compute for this task.

### LightevalTask[[lighteval.tasks.lighteval_task.LightevalTask]]
#### lighteval.tasks.lighteval_task.LightevalTask[[lighteval.tasks.lighteval_task.LightevalTask]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/lighteval_task.py#L202)

aggregationlighteval.tasks.lighteval_task.LightevalTask.aggregationhttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/lighteval_task.py#L405[]
Return a dict with metric name and its aggregation function for all
metrics
#### download_dataset_worker[[lighteval.tasks.lighteval_task.LightevalTask.download_dataset_worker]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/lighteval_task.py#L437)

Worker function to download a dataset from the HuggingFace Hub.

Downloads the dataset specified in the task configuration, optionally
applies a filter if configured, and returns the dataset dictionary.
This method is designed to be used for parallel dataset loading.

**Parameters:**

task (LightevalTask) : The task object containing dataset configuration.

**Returns:**

`DatasetDict`

The loaded dataset dictionary containing all splits.
#### eval_docs[[lighteval.tasks.lighteval_task.LightevalTask.eval_docs]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/lighteval_task.py#L348)

Returns the evaluation documents.

**Returns:**

`list[Doc]`

Evaluation documents.
#### fewshot_docs[[lighteval.tasks.lighteval_task.LightevalTask.fewshot_docs]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/lighteval_task.py#L329)

Returns the few shot documents. If the few shot documents are not
available, it gets them from the few shot split or the evaluation split.

**Returns:**

`list[Doc]`

Documents that will be used for few shot examples. One
document = one few shot example.
#### get_docs[[lighteval.tasks.lighteval_task.LightevalTask.get_docs]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/lighteval_task.py#L360)

Get evaluation documents with few-shot examples and generation parameters configured.

Retrieves evaluation documents, optionally limits the number of samples,
shuffles them for reproducibility, and configures each document with
few-shot examples and generation parameters for evaluation.

**Parameters:**

max_samples (int | None, optional) : Maximum number of documents to return. If None, returns all available documents. Defaults to None.

**Returns:**

`list[Doc]`

List of documents ready for evaluation with few-shot examples
and generation parameters configured.
#### get_first_possible_fewshot_splits[[lighteval.tasks.lighteval_task.LightevalTask.get_first_possible_fewshot_splits]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/lighteval_task.py#L255)

Parses the possible fewshot split keys in order: train, then validation
keys and matches them with the available keys.  Returns the first
available.

**Returns:**

`str`

the first available fewshot splits or None if nothing is available
#### load_datasets[[lighteval.tasks.lighteval_task.LightevalTask.load_datasets]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/lighteval_task.py#L414)

Load datasets from the HuggingFace Hub for the given tasks.

**Parameters:**

tasks (dict[str, LightevalTask]) : Dictionary mapping task names to task objects.

dataset_loading_processes (int, optional) : Number of processes to use for parallel dataset loading. Defaults to 1 (sequential loading).

## PromptManager[[lighteval.tasks.prompt_manager.PromptManager]]
#### lighteval.tasks.prompt_manager.PromptManager[[lighteval.tasks.prompt_manager.PromptManager]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/prompt_manager.py#L42)

prepare_promptlighteval.tasks.prompt_manager.PromptManager.prepare_prompthttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/prompt_manager.py#L48[{"name": "doc", "val": ": Doc"}]strThe formatted prompt string
Prepare a prompt from a document, either using chat template or plain text format.

**Returns:**

`str`

The formatted prompt string
#### prepare_prompt_api[[lighteval.tasks.prompt_manager.PromptManager.prepare_prompt_api]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/prompt_manager.py#L88)

Prepare a prompt for API calls, using a chat-like format.
Will not tokenize the message because APIs will usually handle this.

**Returns:**

`list[dict[str, str]]`

List of message dictionaries for API calls

## Registry[[lighteval.tasks.registry.Registry]]
#### lighteval.tasks.registry.Registry[[lighteval.tasks.registry.Registry]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/registry.py#L112)

The Registry class is used to manage the task registry and get task classes.

create_custom_tasks_modulelighteval.tasks.registry.Registry.create_custom_tasks_modulehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/registry.py#L293[{"name": "custom_tasks", "val": ": str | pathlib.Path | module"}]- **custom_tasks** (Optional[Union[str, ModuleType]]) -- Path to the custom tasks file or name of a module to import containing custom tasks or the module itself0ModuleTypeThe newly imported/created custom tasks modules
Creates a custom task module to load tasks defined by the user in their own file.

**Parameters:**

custom_tasks (Optional[Union[str, ModuleType]]) : Path to the custom tasks file or name of a module to import containing custom tasks or the module itself

**Returns:**

`ModuleType`

The newly imported/created custom tasks modules
#### get_tasks_dump[[lighteval.tasks.registry.Registry.get_tasks_dump]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/registry.py#L442)

Get all task names, metadata, and docstrings as a Python object.

**Returns:**

`list[dict]`

List of dictionaries, each containing:
- module: Module name
- docstring: Parsed docstring as dict
- tasks: List of task configs for this module
#### load_all_task_configs[[lighteval.tasks.registry.Registry.load_all_task_configs]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/registry.py#L344)

Load all LightevalTaskConfig objects from all Python files in the tasks/ directory.
#### print_all_tasks[[lighteval.tasks.registry.Registry.print_all_tasks]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/registry.py#L382)

Print all the tasks in the task registry.

**Parameters:**

suites : Comma-separated list of suites to display. If None, shows core suites only. Use 'all' to show all available suites (core + optional). Special handling for 'multilingual' suite with dependency checking.

## Doc[[lighteval.tasks.requests.Doc]]
#### lighteval.tasks.requests.Doc[[lighteval.tasks.requests.Doc]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/requests.py#L44)

Dataclass representing a single evaluation sample for a benchmark.

This class encapsulates all the information needed to evaluate a model on a single
task instance. It contains the input query, expected outputs, metadata, and
configuration parameters for different types of evaluation tasks.

**Required Fields:**
- `query`: The input prompt or question
- `choices`: Available answer choices (for multiple choice tasks)
- `gold_index`: Index(es) of the correct answer(s)

**Optional Fields:**
- `instruction`: System prompt, task specific. Will be appended to model specific system prompt.
- `images`: Visual inputs for multimodal tasks.

Methods:
get_golds():
Returns the correct answer(s) as strings based on gold_index.
Handles both single and multiple correct answers.

Usage Examples:

**Multiple Choice Question:**

```python
doc = Doc(
    query="What is the capital of France?",
    choices=["London", "Paris", "Berlin", "Madrid"],
    gold_index=1,  # Paris is the correct answer
    instruction="Answer the following geography question:",
)
```

**Generative Task:**

```python
doc = Doc(
    query="Write a short story about a robot.",
    choices=[],  # No predefined choices for generative tasks
    gold_index=0,  # Not used for generative tasks
    generation_size=100,
    stop_sequences=["

End"],
)
```

**Few-shot Learning:**

```python
doc = Doc(
    query="Translate 'Hello world' to Spanish.",
    choices=["Hola mundo", "Bonjour monde", "Ciao mondo"],
    gold_index=0,
    fewshot_samples=[
        Doc(query="Translate 'Good morning' to Spanish.",
            choices=["Buenos días", "Bonjour", "Buongiorno"],
            gold_index=0),
        Doc(query="Translate 'Thank you' to Spanish.",
            choices=["Gracias", "Merci", "Grazie"],
            gold_index=0)
    ],
)
```

**Multimodal Task:**

```python
doc = Doc(
    query="What is shown in this image?",
    choices=["A cat"],
    gold_index=0,
    images=[pil_image],  # PIL Image object
)
```

get_goldslighteval.tasks.requests.Doc.get_goldshttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/tasks/requests.py#L217[]
Return gold targets extracted from the target dict

**Parameters:**

query (str) : The main query, prompt, or question to be sent to the model. 

choices (list[str]) : List of possible answer choices for the query. For multiple choice tasks, this contains all options (A, B, C, D, etc.). For generative tasks, this may be empty or contain reference answers. 

gold_index (Union[int, list[int]]) : Index or indices of the correct answer(s) in the choices list. For single correct answers,(e.g., 0 for first choice). For multiple correct answers, use a list (e.g., [0, 2] for first and third). 

instruction (str | None) : System prompt or task-specific instructions to guide the model. This is typically prepended to the query to set context or behavior. 

images (list["Image"] | None) : List of PIL Image objects for multimodal tasks. 

specific (dict | None) : Task-specific information or metadata. Can contain any additional data needed for evaluation. 

unconditioned_query (Optional[str]) : Query without task-specific context for PMI normalization. Used to calculate: log P(choice | Query) - log P(choice | Unconditioned Query). 

original_query (str | None) : The query before any preprocessing or modification. 

# Set by task parameters --

id (str) : Unique identifier for this evaluation instance. Set by the task and not the user. 

task_name (str) : Name of the task or benchmark this Doc belongs to. 

## Few-shot Learning Parameters --

fewshot_samples (list) : List of Doc objects representing few-shot examples. These examples are prepended to the main query to provide context. 

sampling_methods (list[SamplingMethod]) : List of sampling methods to use for this instance. Options: GENERATIVE, LOGPROBS, PERPLEXITY. 

fewshot_sorting_class (Optional[str]) : Class label for balanced few-shot example selection. Used to ensure diverse representation in few-shot examples. 

## Generation Control Parameters --

generation_size (int | None) : Maximum number of tokens to generate for this instance. 

stop_sequences (list[str] | None) : List of strings that should stop generation when encountered. **Used for**: Controlled generation, preventing unwanted continuations. 

use_logits (bool) : Whether to return logits (raw model outputs) in addition to text. **Used for**: Probability analysis, confidence scoring, detailed evaluation. 

num_samples (int) : Number of different samples to generate for this instance. **Used for**: Diversity analysis, uncertainty estimation, ensemble methods. 

generation_grammar (None) : Grammar constraints for generation (currently not implemented). **Reserved for**: Future structured generation features.

## Datasets[[lighteval.data.DynamicBatchDataset]]
#### lighteval.data.DynamicBatchDataset[[lighteval.data.DynamicBatchDataset]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/data.py#L44)

get_original_orderlighteval.data.DynamicBatchDataset.get_original_orderhttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/data.py#L88[{"name": "new_arr", "val": ": list"}]- **new_arr** (list) -- Array containing any kind of data that needs to be
  reset in the original order.0listnew_arr in the original order.
Get the original order of the data.

**Parameters:**

new_arr (list) : Array containing any kind of data that needs to be reset in the original order.

**Returns:**

`list`

new_arr in the original order.
#### splits_iterator[[lighteval.data.DynamicBatchDataset.splits_iterator]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/data.py#L110)

Iterator that yields the dataset splits based on the split limits.

#### lighteval.data.LoglikelihoodDataset[[lighteval.data.LoglikelihoodDataset]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/data.py#L161)

#### lighteval.data.GenerativeTaskDataset[[lighteval.data.GenerativeTaskDataset]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/data.py#L186)

init_split_limitslighteval.data.GenerativeTaskDataset.init_split_limitshttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/data.py#L187[{"name": "num_dataset_splits", "val": ""}]- **num_dataset_splits** (_type_) -- _description_0_type__description_
Initialises the split limits based on generation parameters.
The splits are used to estimate time remaining when evaluating, and in the case of generative evaluations, to group similar samples together.

For generative tasks, self._sorting_criteria outputs:
- a boolean (whether the generation task uses logits)
- a list (the stop sequences)
- the item length (the actual size sorting factor).

In the current function, we create evaluation groups by generation parameters (logits and eos), so that samples with similar properties get batched together afterwards.
The samples will then be further organised by length in each split.

**Parameters:**

num_dataset_splits (_type_) : _description_

**Returns:**

`_type_`

_description_

#### lighteval.data.GenerativeTaskDatasetNanotron[[lighteval.data.GenerativeTaskDatasetNanotron]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/data.py#L254)

#### lighteval.data.GenDistributedSampler[[lighteval.data.GenDistributedSampler]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/data.py#L270)

A distributed sampler that copy the last element only when drop_last is False so we keep a small padding in the batches
as our samples are sorted by length.
