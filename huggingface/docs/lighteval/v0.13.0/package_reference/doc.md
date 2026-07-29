# Doc[[lighteval.tasks.requests.Doc]]

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
