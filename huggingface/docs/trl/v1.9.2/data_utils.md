# Data Utilities

## is_conversational[[trl.is_conversational]]

- **example** (`dict[str, Any]`) --
  A single data entry of a dataset. The example can have different keys depending on the dataset type.`bool``True` if the data is in a conversational format, `False` otherwise.

Check if the example is in a conversational format.

Examples:

```python
>>> example = {"prompt": [{"role": "user", "content": "What color is the sky?"}]}
>>> is_conversational(example)
True

>>> example = {"prompt": "The sky is"}
>>> is_conversational(example)
False
```

## maybe_convert_to_chatml[[trl.maybe_convert_to_chatml]]

- **example** (`dict[str, list]`) --
  A single data entry containing a list of messages.`dict[str, list]`Example reformatted to ChatML style.

Convert a conversational dataset with fields `from` and `value` to ChatML format.

This function modifies conversational data to align with OpenAI's ChatML format:
- Replaces the key `"from"` with `"role"` in message dictionaries.
- Replaces the key `"value"` with `"content"` in message dictionaries.
- Renames `"conversations"` to `"messages"` for consistency with ChatML.

Example:
```python
>>> from trl import maybe_convert_to_chatml

>>> example = {
...     "conversations": [
...         {"from": "user", "value": "What color is the sky?"},
...         {"from": "assistant", "value": "It is blue."},
...     ]
... }
>>> maybe_convert_to_chatml(example)
{'messages': [{'role': 'user', 'content': 'What color is the sky?'},
              {'role': 'assistant', 'content': 'It is blue.'}]}
```

## extract_prompt[[trl.extract_prompt]]

- **example** (`dict[str, list]`) --
  A dictionary representing a single data entry in the preference dataset. It must contain the keys
  `"chosen"` and `"rejected"`, where each value is either conversational or standard (`str`).`dict[str, list]`A dictionary containing:
- `"prompt"`: The longest common prefix between the "chosen" and "rejected" completions.
- `"chosen"`: The remainder of the "chosen" completion, with the prompt removed.
- `"rejected"`: The remainder of the "rejected" completion, with the prompt removed.

Extracts the shared prompt from a preference data example, where the prompt is implicit within both the chosen and
rejected completions.

The function identifies the longest common sequence (prefix) of conversation turns between the "chosen" and
"rejected" completions and extracts this as the prompt. It then removes this prompt from the respective "chosen"
and "rejected" completions.

Examples:

```python
>>> example = {
...     "chosen": [
...         {"role": "user", "content": "What color is the sky?"},
...         {"role": "assistant", "content": "It is blue."},
...     ],
...     "rejected": [
...         {"role": "user", "content": "What color is the sky?"},
...         {"role": "assistant", "content": "It is green."},
...     ],
... }
>>> extract_prompt(example)
{'prompt': [{'role': 'user', 'content': 'What color is the sky?'}],
 'chosen': [{'role': 'assistant', 'content': 'It is blue.'}],
 'rejected': [{'role': 'assistant', 'content': 'It is green.'}]}
```

Or, with the `map` method of `Dataset`:

```python
>>> from trl import extract_prompt
>>> from datasets import Dataset

>>> dataset_dict = {
...     "chosen": [
...         [
...             {"role": "user", "content": "What color is the sky?"},
...             {"role": "assistant", "content": "It is blue."},
...         ],
...         [
...             {"role": "user", "content": "Where is the sun?"},
...             {"role": "assistant", "content": "In the sky."},
...         ],
...     ],
...     "rejected": [
...         [
...             {"role": "user", "content": "What color is the sky?"},
...             {"role": "assistant", "content": "It is green."},
...         ],
...         [
...             {"role": "user", "content": "Where is the sun?"},
...             {"role": "assistant", "content": "In the sea."},
...         ],
...     ],
... }
>>> dataset = Dataset.from_dict(dataset_dict)
>>> dataset = dataset.map(extract_prompt)
>>> dataset[0]
{'prompt': [{'role': 'user', 'content': 'What color is the sky?'}],
 'chosen': [{'role': 'assistant', 'content': 'It is blue.'}],
 'rejected': [{'role': 'assistant', 'content': 'It is green.'}]}
```

## unpair_preference_dataset[[trl.unpair_preference_dataset]]

- **dataset** (`Dataset` or `DatasetDict` or `IterableDataset` or `IterableDatasetDict`) --
  Preference dataset to unpair. The dataset must have columns `"chosen"`, `"rejected"` and optionally
  `"prompt"`.
- ****map_kwargs** (`dict`, *optional*) --
  Additional keyword arguments to pass to the dataset's map method when unpairing preferences.`Dataset` or `DatasetDict` or `IterableDataset` or `IterableDatasetDict`The unpaired preference dataset.

Unpair a preference dataset.

The output contains `"prompt"`, `"completion"`, and `"label"` plus any extra columns, which are duplicated for
each chosen and rejected row.

Example:

```python
>>> from datasets import Dataset

>>> dataset_dict = {
...     "prompt": ["The sky is", "The sun is"],
...     "chosen": [" blue.", "in the sky."],
...     "rejected": [" green.", " in the sea."],
... }
>>> dataset = Dataset.from_dict(dataset_dict)
>>> dataset = unpair_preference_dataset(dataset)
>>> dataset
Dataset({
    features: ['prompt', 'completion', 'label'],
    num_rows: 4
})

>>> dataset[0]
{'prompt': 'The sky is', 'completion': ' blue.', 'label': True}
```
