# BERT

## Overview

[BERT](https://huggingface.co/papers/1810.04805) is a bidirectional transformer pretrained on unlabeled text to predict masked tokens in a sentence and to predict whether one sentence follows another. The main idea is that by randomly masking some tokens, the model can train on text to the left and right, giving it a more thorough understanding. BERT is also very versatile because its learned language representations can be adapted for other NLP tasks by fine-tuning an additional layer or head.

You can find all the original BERT checkpoints under the [BERT](https://huggingface.co/collections/google/bert-release-64ff5e7a4be99045d1896dbc) collection.

## Export to Neuron

To deploy 🤗 [Transformers](https://huggingface.co/docs/transformers/index) models on Neuron devices, you first need to compile the models and export them to a serialized format for inference. Below are two approaches to compile the model, you can choose the one that best suits your needs. Here we take the `feature-extraction` as an example:

### Option 1: CLI

You can export the model using the Optimum command-line interface as follows:

```bash
optimum-cli export neuron --model google-bert/bert-base-uncased --task feature-extraction --batch_size 1 --sequence_length 128 bert_feature_extraction_neuronx/
```

> [!TIP]
> Execute `optimum-cli export neuron --help` to display all command line options and their description.

### Option 2: Python API

```python
from optimum.neuron import NeuronModelForFeatureExtraction

input_shapes = {"batch_size": 1, "sequence_length": 128}
compiler_args = {"auto_cast": "matmul", "auto_cast_type": "bf16"}
neuron_model = NeuronModelForFeatureExtraction.from_pretrained(
    "google-bert/bert-base-uncased",
    export=True,
    **input_shapes,
    **compiler_args,
)
# Save locally
neuron_model.save_pretrained("bert_feature_extraction_neuronx")

# Upload to the HuggingFace Hub
neuron_model.push_to_hub(
    "bert_feature_extraction_neuronx", repository_id="my-neuron-repo"  # Replace with your HF Hub repo id
)
```

## NeuronBertModel[[optimum.neuron.NeuronBertModel]]

#### optimum.neuron.NeuronBertModel[[optimum.neuron.NeuronBertModel]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L62)

Bare Bert Model transformer outputting raw hidden-states without any specific head on top, used for the task "feature-extraction".

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronBertModel.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L65[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  See [`PreTrainedTokenizer.encode`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.encode) and
  [`PreTrainedTokenizer.__call__`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.__call__) for details.
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The `NeuronBertModel` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronBertModel

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/bert-base-uncased-neuronx-bs1-sq128")
>>> model = NeuronBertModel.from_pretrained("optimum/bert-base-uncased-neuronx-bs1-sq128")

>>> inputs = tokenizer("Dear Evan Hansen is the winner of six Tony Awards.", return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> list(last_hidden_state.shape)
[1, 13, 384]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

## NeuronBertForMaskedLM[[optimum.neuron.NeuronBertForMaskedLM]]

#### optimum.neuron.NeuronBertForMaskedLM[[optimum.neuron.NeuronBertForMaskedLM]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L110)

Masked language Bert Model with a `language modeling` head on top, for masked language modeling tasks on Neuron devices.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronBertForMaskedLM.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L113[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  See [`PreTrainedTokenizer.encode`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.encode) and
  [`PreTrainedTokenizer.__call__`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.__call__) for details.
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The `NeuronBertForMaskedLM` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronBertForMaskedLM

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/legal-bert-base-uncased-neuronx")
>>> model = NeuronBertForMaskedLM.from_pretrained("optimum/legal-bert-base-uncased-neuronx")

>>> inputs = tokenizer("This [MASK] Agreement is between General Motors and John Murray.", return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> list(logits.shape)
[1, 13, 30522]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

## NeuronBertForSequenceClassification[[optimum.neuron.NeuronBertForSequenceClassification]]

#### optimum.neuron.NeuronBertForSequenceClassification[[optimum.neuron.NeuronBertForSequenceClassification]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L197)

Neuron Model with a sequence classification/regression head on top (a linear layer on top of the
pooled output) e.g. for GLUE tasks.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronBertForSequenceClassification.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L200[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  See [`PreTrainedTokenizer.encode`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.encode) and
  [`PreTrainedTokenizer.__call__`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.__call__) for details.
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The `NeuronBertForSequenceClassification` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronBertForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/bert-base-multilingual-uncased-sentiment-neuronx")
>>> model = NeuronBertForSequenceClassification.from_pretrained("optimum/bert-base-multilingual-uncased-sentiment-neuronx")

>>> inputs = tokenizer("Hamilton is considered to be the best musical of human history.", return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> list(logits.shape)
[1, 2]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

## NeuronBertForTokenClassification[[optimum.neuron.NeuronBertForTokenClassification]]

#### optimum.neuron.NeuronBertForTokenClassification[[optimum.neuron.NeuronBertForTokenClassification]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L240)

Neuron Model with a token classification head on top (a linear layer on top of the hidden-states output) e.g.
for Named-Entity-Recognition (NER) tasks.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronBertForTokenClassification.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L243[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  See [`PreTrainedTokenizer.encode`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.encode) and
  [`PreTrainedTokenizer.__call__`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.__call__) for details.
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The `NeuronBertForTokenClassification` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronBertForTokenClassification

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/bert-base-NER-neuronx")
>>> model = NeuronBertForTokenClassification.from_pretrained("optimum/bert-base-NER-neuronx")

>>> inputs = tokenizer("Lin-Manuel Miranda is an American songwriter, actor, singer, filmmaker, and playwright.", return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> list(logits.shape)
[1, 20, 9]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

## NeuronBertForQuestionAnswering[[optimum.neuron.NeuronBertForQuestionAnswering]]

#### optimum.neuron.NeuronBertForQuestionAnswering[[optimum.neuron.NeuronBertForQuestionAnswering]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L153)

Bert with a span classification head on top for extractive question-answering tasks like SQuAD (a linear
layers on top of the hidden-states output to compute `span start logits` and `span end logits`).

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronBertForQuestionAnswering.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L156[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  See [`PreTrainedTokenizer.encode`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.encode) and
  [`PreTrainedTokenizer.__call__`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.__call__) for details.
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The `NeuronBertForQuestionAnswering` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> import torch
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronBertForQuestionAnswering

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/bert-base-cased-squad2-neuronx")
>>> model = NeuronBertForQuestionAnswering.from_pretrained("optimum/bert-base-cased-squad2-neuronx")

>>> question, text = "Are there wheelchair spaces in the theatres?", "Yes, we have reserved wheelchair spaces with a good view."
>>> inputs = tokenizer(question, text, return_tensors="pt")
>>> start_positions = torch.tensor([1])
>>> end_positions = torch.tensor([12])

>>> outputs = model(**inputs, start_positions=start_positions, end_positions=end_positions)
>>> start_scores = outputs.start_logits
>>> end_scores = outputs.end_logits
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

## NeuronBertForMultipleChoice[[optimum.neuron.NeuronBertForMultipleChoice]]

#### optimum.neuron.NeuronBertForMultipleChoice[[optimum.neuron.NeuronBertForMultipleChoice]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L284)

Neuron Model with a multiple choice classification head on top (a linear layer on top of the pooled output and a
softmax) e.g. for RocStories/SWAG tasks.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronBertForMultipleChoice.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/bert/modeling_bert.py#L287[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, num_choices, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  See [`PreTrainedTokenizer.encode`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.encode) and
  [`PreTrainedTokenizer.__call__`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.__call__) for details.
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor | None` of shape `(batch_size, num_choices, sequence_length)`, defaults to `None`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor | None` of shape `(batch_size, num_choices, sequence_length)`, defaults to `None`) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The `NeuronBertForMultipleChoice` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronBertForMultipleChoice

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/bert-base-cased-swag-neuronx")
>>> model = NeuronBertForMultipleChoice.from_pretrained("optimum/bert-base-cased-swag-neuronx")

>>> num_choices = 4
>>> first_sentence = ["Members of the procession walk down the street holding small horn brass instruments."] * num_choices
>>> second_sentence = [
...     "A drum line passes by walking down the street playing their instruments.",
...     "A drum line has heard approaching them.",
...     "A drum line arrives and they're outside dancing and asleep.",
...     "A drum line turns the lead singer watches the performance."
... ]
>>> inputs = tokenizer(first_sentence, second_sentence, truncation=True, padding=True)

# Unflatten the inputs values expanding it to the shape [batch_size, num_choices, seq_length]
>>> for k, v in inputs.items():
...     inputs[k] = [v[i: i + num_choices] for i in range(0, len(v), num_choices)]
>>> inputs = dict(inputs.convert_to_tensors(tensor_type="pt"))
>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> logits.shape
[1, 4]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.
