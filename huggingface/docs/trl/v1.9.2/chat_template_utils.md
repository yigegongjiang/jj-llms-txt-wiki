# Chat template utilities

For an overview of the chat templates bundled with TRL and the rationale behind the training patches, see [Chat Templates](chat_templates).

## clone_chat_template[[trl.clone_chat_template]]

- **model** ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel)) --
  Model to update.
- **tokenizer** ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase)) --
  Tokenizer to update.
- **source_tokenizer_path** (`str`) --
  Path or identifier of the pretrained tokenizer to clone from.
- **resize_to_multiple_of** (`int` or `None`, *optional*, defaults to `64`) --
  The embedding layer will be resized to the new vocabulary size. If this is not `None`, it will round up the
  new vocabulary size to the nearest multiple of this value.model ([PreTrainedModel](https://huggingface.co/docs/transformers/v5.14.1/en/main_classes/model#transformers.PreTrainedModel))Updated model with resized token embeddings and EOS token configured.
tokenizer ([PreTrainedTokenizerBase](https://huggingface.co/docs/transformers/v5.14.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase)):
Updated tokenizer with the chat template and special tokens applied.
added_tokens (`list[int]`):
List of tokens that were added to the tokenizer from the source tokenizer.

Clones a chat template from a source tokenizer to the target tokenizer and updates the model accordingly.

This function:
- Copies the chat template from a source tokenizer to the target tokenizer.
- Adds any new tokens from the source tokenizer to the target tokenizer.
- Sets and synchronizes the EOS token across the tokenizer and model.
- Resizes the model's token embeddings to match the new vocabulary size, optionally rounding it up to a multiple of
  a specified value. In such cases, dummy tokens are added to the tokenizer to ensure the vocabulary size matches
  the embedding dimensions.

Example:
```python
>>> from transformers import AutoModelForCausalLM, AutoTokenizer
>>> from trl import clone_chat_template

>>> model = AutoModelForCausalLM.from_pretrained("meta-llama/Llama-3.2-1B")
>>> tokenizer = AutoTokenizer.from_pretrained("meta-llama/Llama-3.2-1B")
>>> model, tokenizer, added_tokens = clone_chat_template(model, tokenizer, "Qwen/Qwen3-0.6B")
```

## is_chat_template_prefix_preserving[[trl.chat_template_utils.is_chat_template_prefix_preserving]]

- **processing_class** (`PreTrainedTokenizerBase` or `ProcessorMixin`) --
  Tokenizer or processor instance to check.`bool``True` if the chat template preserves prefixes, `False` otherwise.

Check whether the chat template preserves prefixes when applied.

A prefix-preserving chat template renders earlier messages identically regardless of what messages follow. This
property is required by `_get_tool_suffix_ids`, which extracts tool response formatting tokens by comparing
tokenizations with and without tool messages appended.

## get_training_chat_template[[trl.get_training_chat_template]]

- **processing_class** (`PreTrainedTokenizerBase` or `ProcessorMixin`) --
  Tokenizer or processor instance to check.`str` or `None`Training-compatible chat template, or `None` if no patching is needed.

Get a training-compatible chat template, if needed.

Returns a patched chat template that is prefix-preserving and includes `{%% generation %%}` / `{%% endgeneration
%%}` markers for assistant-only loss masking. Returns `None` if the template already satisfies both requirements.
Currently Cohere, Cohere 2, DeepSeek-V3, Gemma, Gemma 2, Gemma 3, GLM-4-MoE, GPT-OSS, Idefics3, LLaMA 3, Phi-3,
Phi-3.5, Qwen2-VL, Qwen2.5, Qwen2.5-VL, Qwen3 (including the Instruct-2507 variant), Qwen3-VL, Qwen3.5, and Qwen3.6
are supported.

Example:

```python
>>> from trl.chat_template_utils import get_training_chat_template
>>> from transformers import AutoTokenizer

>>> tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen3-0.6B")
>>> messages1 = [
...     {"role": "user", "content": "What is 2 * 3?"},
...     {
...         "role": "assistant",
...         "content": "",
...         "tool_calls": [{"type": "function", "function": {"name": "multiply", "arguments": {"a": 2, "b": 3}}}],
...     },
... ]
>>> messages2 = messages1 + [
...     {"role": "tool", "name": "multiply", "content": "6"},
... ]
>>> tokenizer.apply_chat_template(messages1, tokenize=False)
'<|im_start|>user\nWhat is 2 * 3?<|im_end|>\n<|im_start|>assistant\n<think>\n\n</think>\n\n<tool_call>\n{"name": "multiply", "arguments": {"a": 2, "b": 3}}\n</tool_call><|im_end|>\n'

>>> tokenizer.apply_chat_template(messages2, tokenize=False, add_generation_prompt=True)
'<|im_start|>user\nWhat is 2 * 3?<|im_end|>\n<|im_start|>assistant\n<tool_call>\n{"name": "multiply", "arguments": {"a": 2, "b": 3}}\n</tool_call><|im_end|>\n<|im_start|>user\n<tool_response>\n6\n</tool_response><|im_end|>\n<|im_start|>assistant\n'

>>> #                                                        ^ think tags missing
>>> chat_template = get_training_chat_template(tokenizer)
>>> tokenizer.apply_chat_template(messages1, tokenize=False, chat_template=chat_template)
'<|im_start|>user\nWhat is 2 * 3?<|im_end|>\n<|im_start|>assistant\n<think>\n\n</think>\n\n<tool_call>\n{"name": "multiply", "arguments": {"a": 2, "b": 3}}\n</tool_call><|im_end|>\n'

>>> tokenizer.apply_chat_template(
...     messages2, tokenize=False, add_generation_prompt=True, chat_template=chat_template
... )
'<|im_start|>user\nWhat is 2 * 3?<|im_end|>\n<|im_start|>assistant\n<think>\n\n</think>\n\n<tool_call>\n{"name": "multiply", "arguments": {"a": 2, "b": 3}}\n</tool_call><|im_end|>\n<|im_start|>user\n<tool_response>\n6\n</tool_response><|im_end|>\n<|im_start|>assistant\n'
```
