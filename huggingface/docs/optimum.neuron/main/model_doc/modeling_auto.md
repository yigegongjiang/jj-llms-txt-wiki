# Models

## Generic model classes

### NeuronTracedModel[[optimum.neuron.NeuronTracedModel]]

The `NeuronTracedModel` class is available for instantiating a base Neuron model without a specific head.
It is used as the base class for all tasks but text generation.

#### optimum.neuron.NeuronTracedModel[[optimum.neuron.NeuronTracedModel]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_traced.py#L68)

Base class running compiled and optimized models on Neuron devices.

It implements generic methods for interacting with the Hugging Face Hub as well as compiling vanilla
transformers models to neuron-optimized TorchScript module and export it using `optimum.exporters.neuron` toolchain.

Class attributes:
- model_type (`str`, *optional*, defaults to `"neuron_model"`) -- The name of the model type to use when
registering the NeuronTracedModel classes.
- auto_model_class (`Type`, *optional*, defaults to `AutoModel`) -- The `AutoModel` class to be represented by the
current NeuronTracedModel class.

Common attributes:
- model (`torch.jit._script.ScriptModule`) -- The loaded `ScriptModule` compiled for neuron devices.
- config (`PretrainedConfig`) -- The configuration of the model.
- model_save_dir (`Path`) -- The directory where a neuron compiled model is saved.
By default, if the loaded model is local, the directory where the original model will be used. Otherwise, the
cache directory will be used.

can_generateoptimum.neuron.NeuronTracedModel.can_generatehttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_traced.py#L662[]

Returns whether this model can generate sequences with `.generate()`.
#### get_input_static_shapes[[optimum.neuron.NeuronTracedModel.get_input_static_shapes]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_traced.py#L515)

Gets a dictionary of inputs with their valid static shapes.
#### load_model[[optimum.neuron.NeuronTracedModel.load_model]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_traced.py#L116)

Loads a TorchScript module compiled by neuron(x)-cc compiler. It will be first loaded onto CPU and then moved to
one or multiple [NeuronCore](https://awsdocs-neuron.readthedocs-hosted.com/en/latest/general/arch/neuron-hardware/neuroncores-arch.html).

**Parameters:**

path (`str | Path`) : Path of the compiled model.

to_neuron (`bool`, defaults to `False`) : Whether to move manually the traced model to NeuronCore. It's only needed when `inline_weights_to_neff=False`, otherwise it is loaded automatically to a Neuron device.

device_id (`int`, defaults to 0) : Index of NeuronCore to load the traced model to.
#### remove_padding[[optimum.neuron.NeuronTracedModel.remove_padding]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_traced.py#L612)

Removes padding from output tensors.

**Parameters:**

outputs (`list[torch.Tensor]`) : List of torch tensors which are inference output.

dims (`list[int]`) : List of dimensions in which we slice a tensor.

indices (`list[int]`) : List of indices in which we slice a tensor along an axis.

padding_side (`Literal["right", "left"]`, defaults to "right") : The side on which the padding has been applied.

## Natural Language Processing

The following Neuron model classes are available for natural language processing tasks.

### NeuronModelForFeatureExtraction[[optimum.neuron.NeuronModelForFeatureExtraction]]

#### optimum.neuron.NeuronModelForFeatureExtraction[[optimum.neuron.NeuronModelForFeatureExtraction]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L85)

Neuron Model with a BaseModelOutput for feature-extraction tasks.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Feature Extraction model on Neuron devices.

forwardoptimum.neuron.NeuronModelForFeatureExtraction.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L92[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
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
The `NeuronModelForFeatureExtraction` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronModelForFeatureExtraction

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/all-MiniLM-L6-v2-neuronx")
>>> model = NeuronModelForFeatureExtraction.from_pretrained("optimum/all-MiniLM-L6-v2-neuronx")

>>> inputs = tokenizer("Dear Evan Hansen is the winner of six Tony Awards.", return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> list(last_hidden_state.shape)
[1, 13, 384]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

### NeuronSentenceTransformers[[optimum.neuron.NeuronSentenceTransformers]]

#### optimum.neuron.NeuronSentenceTransformers[[optimum.neuron.NeuronSentenceTransformers]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_sentence_transformers.py#L38)

Sentence Transformers model on Neuron devices.

tokenizeoptimum.neuron.NeuronSentenceTransformers.tokenizehttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_sentence_transformers.py#L104[{"name": "texts", "val": ": list[str] | list[dict] | list[tuple[str, str]]"}, {"name": "**kwargs", "val": ""}]- **texts** (list[str] | list[dict] | list[tuple[str, str]]]) -- A list of texts to be tokenized.0dict[str, torch.Tensor]A dictionary of tensors with the tokenized texts. Common keys are "input_ids",
"attention_mask", and "token_type_ids".

Tokenizes the texts.

**Parameters:**

texts (list[str] | list[dict] | list[tuple[str, str]]]) : A list of texts to be tokenized.

**Returns:**

`dict[str, torch.Tensor]`

A dictionary of tensors with the tokenized texts. Common keys are "input_ids",
"attention_mask", and "token_type_ids".

### NeuronModelForMaskedLM[[optimum.neuron.NeuronModelForMaskedLM]]

#### optimum.neuron.NeuronModelForMaskedLM[[optimum.neuron.NeuronModelForMaskedLM]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L137)

Neuron Model with a MaskedLMOutput for masked language modeling tasks.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Masked language model for on Neuron devices.

forwardoptimum.neuron.NeuronModelForMaskedLM.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L144[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
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
The `NeuronModelForMaskedLM` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronModelForMaskedLM

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/legal-bert-base-uncased-neuronx")
>>> model = NeuronModelForMaskedLM.from_pretrained("optimum/legal-bert-base-uncased-neuronx")

>>> inputs = tokenizer("This [MASK] Agreement is between General Motors and John Murray.", return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> list(logits.shape)
[1, 13, 30522]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

### NeuronModelForSequenceClassification[[optimum.neuron.NeuronModelForSequenceClassification]]

#### optimum.neuron.NeuronModelForSequenceClassification[[optimum.neuron.NeuronModelForSequenceClassification]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L231)

Neuron Model with a sequence classification/regression head on top (a linear layer on top of the
pooled output) e.g. for GLUE tasks.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Sequence Classification model on Neuron devices.

forwardoptimum.neuron.NeuronModelForSequenceClassification.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L238[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
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
The `NeuronModelForSequenceClassification` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronModelForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/distilbert-base-uncased-finetuned-sst-2-english-neuronx")
>>> model = NeuronModelForSequenceClassification.from_pretrained("optimum/distilbert-base-uncased-finetuned-sst-2-english-neuronx")

>>> inputs = tokenizer("Hamilton is considered to be the best musical of human history.", return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> list(logits.shape)
[1, 2]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

### NeuronModelForQuestionAnswering[[optimum.neuron.NeuronModelForQuestionAnswering]]

#### optimum.neuron.NeuronModelForQuestionAnswering[[optimum.neuron.NeuronModelForQuestionAnswering]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L183)

Neuron Model with a QuestionAnsweringModelOutput for extractive question-answering tasks like SQuAD.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Question Answering model on Neuron devices.

forwardoptimum.neuron.NeuronModelForQuestionAnswering.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L190[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
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
The `NeuronModelForQuestionAnswering` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> import torch
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronModelForQuestionAnswering

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/roberta-base-squad2-neuronx")
>>> model = NeuronModelForQuestionAnswering.from_pretrained("optimum/roberta-base-squad2-neuronx")

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

### NeuronModelForTokenClassification[[optimum.neuron.NeuronModelForTokenClassification]]

#### optimum.neuron.NeuronModelForTokenClassification[[optimum.neuron.NeuronModelForTokenClassification]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L278)

Neuron Model with a token classification head on top (a linear layer on top of the hidden-states output) e.g.
for Named-Entity-Recognition (NER) tasks.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Token Classification model on Neuron devices.

forwardoptimum.neuron.NeuronModelForTokenClassification.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L285[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
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
The `NeuronModelForTokenClassification` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronModelForTokenClassification

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/bert-base-NER-neuronx")
>>> model = NeuronModelForTokenClassification.from_pretrained("optimum/bert-base-NER-neuronx")

>>> inputs = tokenizer("Lin-Manuel Miranda is an American songwriter, actor, singer, filmmaker, and playwright.", return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> list(logits.shape)
[1, 20, 9]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

### NeuronModelForMultipleChoice[[optimum.neuron.NeuronModelForMultipleChoice]]

#### optimum.neuron.NeuronModelForMultipleChoice[[optimum.neuron.NeuronModelForMultipleChoice]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L326)

Neuron Model with a multiple choice classification head on top (a linear layer on top of the pooled output and a
softmax) e.g. for RocStories/SWAG tasks.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Multiple choice model on Neuron devices.

forwardoptimum.neuron.NeuronModelForMultipleChoice.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L333[{"name": "input_ids", "val": ": Tensor"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "token_type_ids", "val": ": torch.Tensor | None = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor` of shape `(batch_size, num_choices, sequence_length)`) --
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
The `NeuronModelForMultipleChoice` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoTokenizer
>>> from optimum.neuron import NeuronModelForMultipleChoice

>>> tokenizer = AutoTokenizer.from_pretrained("optimum/bert-base-uncased_SWAG-neuronx")
>>> model = NeuronModelForMultipleChoice.from_pretrained("optimum/bert-base-uncased_SWAG-neuronx")

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

### NeuronModelForCausalLM[[optimum.neuron.NeuronModelForCausalLM]]

#### optimum.neuron.NeuronModelForCausalLM[[optimum.neuron.NeuronModelForCausalLM]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/models/inference/modeling_utils.py#L398)

Neuron model with a causal language modeling head for inference on Neuron devices.

This model inherits from `~neuron.NeuronModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronModelForCausalLM.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_base.py#L42[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

### NeuronModelForSeq2SeqLM[[optimum.neuron.NeuronModelForSeq2SeqLM]]

#### optimum.neuron.NeuronModelForSeq2SeqLM[[optimum.neuron.NeuronModelForSeq2SeqLM]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_seq2seq.py#L443)

Neuron Sequence-to-sequence model with a language modeling head for text2text-generation tasks.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.neuron.NeuronModelForSeq2SeqLM.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_seq2seq.py#L447[{"name": "attention_mask", "val": ": torch.FloatTensor | None = None"}, {"name": "decoder_input_ids", "val": ": torch.LongTensor | None = None"}, {"name": "decoder_attention_mask", "val": ": torch.BoolTensor | None = None"}, {"name": "encoder_outputs", "val": ": tuple[tuple[torch.Tensor]] | None = None"}, {"name": "beam_scores", "val": ": torch.FloatTensor | None = None"}, {"name": "return_dict", "val": ": bool = False"}, {"name": "output_attentions", "val": ": bool = False"}, {"name": "output_hidden_states", "val": ": bool = False"}]- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  See [`PreTrainedTokenizer.encode`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.encode) and
  [`PreTrainedTokenizer.__call__`](https://huggingface.co/docs/transformers/main_classes/tokenizer#transformers.PreTrainedTokenizerBase.__call__) for details.
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor | None` of shape `(batch_size, sequence_length)`, defaults to `None`) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)0
The [NeuronModelForSeq2SeqLM](/docs/optimum.neuron/main/en/model_doc/modeling_auto#optimum.neuron.NeuronModelForSeq2SeqLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

*(Following models are compiled with neuronx compiler and can only be run on INF2.)*

Example of text-to-text generation:

```python
from transformers import AutoTokenizer
from optimum.neuron import NeuronModelForSeq2SeqLM
# export
neuron_model = NeuronModelForSeq2SeqLM.from_pretrained(google-t5/t5-small, export=True, dynamic_batch_size=False, batch_size=1, sequence_length=64, num_beams=4)
neuron_model.save_pretrained("t5_small_neuronx")
del neuron_model

# inference
neuron_model = NeuronModelForSeq2SeqLM.from_pretrained("t5_small_neuronx")
tokenizer = AutoTokenizer.from_pretrained("t5_small_neuronx")
inputs = tokenizer("translate English to German: Lets eat good food.", return_tensors="pt")

output = neuron_model.generate(
    **inputs,
    num_return_sequences=1,
)
results = [tokenizer.decode(t, skip_special_tokens=True) for t in output]
```

*(For large models, in order to fit into Neuron cores, we need to apply tensor parallelism. Here below is an example ran on `inf2.24xlarge`.)*

Example of text-to-text generation with tensor parallelism:
```python
from transformers import AutoTokenizer
from optimum.neuron import NeuronModelForSeq2SeqLM
# export
if __name__ == "__main__":  # compulsory for parallel tracing since the API will spawn multiple processes.
    neuron_model = NeuronModelForSeq2SeqLM.from_pretrained(
        google/flan-t5-xl, export=True, tensor_parallel_size=8, dynamic_batch_size=False, batch_size=1, sequence_length=128, num_beams=4,
    )
    neuron_model.save_pretrained("flan_t5_xl_neuronx_tp8")
    del neuron_model
# inference
neuron_model = NeuronModelForSeq2SeqLM.from_pretrained("flan_t5_xl_neuronx_tp8")
tokenizer = AutoTokenizer.from_pretrained("flan_t5_xl_neuronx_tp8")
inputs = tokenizer("translate English to German: Lets eat good food.", return_tensors="pt")

output = neuron_model.generate(
    **inputs,
    num_return_sequences=1,
)
results = [tokenizer.decode(t, skip_special_tokens=True) for t in output]
```

**Parameters:**

encoder (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module of the encoder with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

decoder (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module of the decoder with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

## Computer Vision

The following Neuron model classes are available for computer vision tasks.

### NeuronModelForImageClassification[[optimum.neuron.NeuronModelForImageClassification]]

#### optimum.neuron.NeuronModelForImageClassification[[optimum.neuron.NeuronModelForImageClassification]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L373)

Neuron Model with an image classification head on top (a linear layer on top of the final hidden state of the [CLS] token) e.g. for ImageNet.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Neuron Model for image-classification tasks. This class officially supports beit, convnext, convnextv2, deit, levit, mobilenet_v2, mobilevit, vit, etc.

forwardoptimum.neuron.NeuronModelForImageClassification.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L387[{"name": "pixel_values", "val": ": Tensor"}, {"name": "**kwargs", "val": ""}]- **pixel_values** (`torch.Tensor | None` of shape `(batch_size, num_channels, height, width)`, defaults to `None`) --
  Pixel values corresponding to the images in the current batch.
  Pixel values can be obtained from encoded images using [`AutoImageProcessor`](https://huggingface.co/docs/transformers/en/model_doc/auto#transformers.AutoImageProcessor).0
The `NeuronModelForImageClassification` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> import requests
>>> from PIL import Image
>>> from optimum.neuron import NeuronModelForImageClassification
>>> from transformers import AutoImageProcessor

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)

>>> preprocessor = AutoImageProcessor.from_pretrained("optimum/vit-base-patch16-224-neuronx")
>>> model = NeuronModelForImageClassification.from_pretrained("optimum/vit-base-patch16-224-neuronx")

>>> inputs = preprocessor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> predicted_label = logits.argmax(-1).item()
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

### NeuronModelForSemanticSegmentation[[optimum.neuron.NeuronModelForSemanticSegmentation]]
#### optimum.neuron.NeuronModelForSemanticSegmentation[[optimum.neuron.NeuronModelForSemanticSegmentation]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L420)

Neuron Model with a semantic segmentation head on top, e.g. for Pascal VOC.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Neuron Model for semantic-segmentation, with an all-MLP decode head on top e.g. for ADE20k, CityScapes. This class officially supports mobilevit, mobilenet-v2, etc.

forwardoptimum.neuron.NeuronModelForSemanticSegmentation.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L434[{"name": "pixel_values", "val": ": Tensor"}, {"name": "**kwargs", "val": ""}]- **pixel_values** (`torch.Tensor | None` of shape `(batch_size, num_channels, height, width)`, defaults to `None`) --
  Pixel values corresponding to the images in the current batch.
  Pixel values can be obtained from encoded images using [`AutoImageProcessor`](https://huggingface.co/docs/transformers/en/model_doc/auto#transformers.AutoImageProcessor).0
The `NeuronModelForSemanticSegmentation` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> import requests
>>> from PIL import Image
>>> from optimum.neuron import NeuronModelForSemanticSegmentation
>>> from transformers import AutoImageProcessor

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)

>>> preprocessor = AutoImageProcessor.from_pretrained("optimum/deeplabv3-mobilevit-small-neuronx")
>>> model = NeuronModelForSemanticSegmentation.from_pretrained("optimum/deeplabv3-mobilevit-small-neuronx")

>>> inputs = preprocessor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> logits = outputs.logits
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

### NeuronModelForObjectDetection[[optimum.neuron.NeuronModelForObjectDetection]]
#### optimum.neuron.NeuronModelForObjectDetection[[optimum.neuron.NeuronModelForObjectDetection]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L467)

Neuron Model with object detection heads on top, for tasks such as COCO detection.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Neuron Model for object-detection, with object detection heads on top, for tasks such as COCO detection.

forwardoptimum.neuron.NeuronModelForObjectDetection.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L481[{"name": "pixel_values", "val": ": Tensor"}, {"name": "**kwargs", "val": ""}]- **pixel_values** (`torch.Tensor | None` of shape `(batch_size, num_channels, height, width)`, defaults to `None`) --
  Pixel values corresponding to the images in the current batch.
  Pixel values can be obtained from encoded images using [`AutoImageProcessor`](https://huggingface.co/docs/transformers/en/model_doc/auto#transformers.AutoImageProcessor).0
The `NeuronModelForObjectDetection` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> import requests
>>> from PIL import Image
>>> from optimum.neuron import NeuronModelForObjectDetection
>>> from transformers import AutoImageProcessor

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)

>>> preprocessor = AutoImageProcessor.from_pretrained("hustvl/yolos-tiny")
>>> model = NeuronModelForObjectDetection.from_pretrained("hustvl/yolos-tiny")

>>> inputs = preprocessor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> target_sizes = torch.tensor([image.size[::-1]])
>>> results = image_processor.post_process_object_detection(outputs, threshold=0.9, target_sizes=target_sizes)[0]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

## Audio

The following auto classes are available for the following audio tasks.

### NeuronModelForAudioClassification[[optimum.neuron.NeuronModelForAudioClassification]]
#### optimum.neuron.NeuronModelForAudioClassification[[optimum.neuron.NeuronModelForAudioClassification]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L516)

Neuron Model with an audio classification head.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Neuron Model for audio-classification, with a sequence classification head on top (a linear layer over the pooled output) for tasks like
SUPERB Keyword Spotting.

forwardoptimum.neuron.NeuronModelForAudioClassification.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L524[{"name": "input_values", "val": ": Tensor"}, {"name": "**kwargs", "val": ""}]- **input_values** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Float values of input raw speech waveform..
  Input values can be obtained from audio file loaded into an array using [`AutoProcessor`](https://huggingface.co/docs/transformers/en/model_doc/auto#transformers.AutoProcessor).0
The `NeuronModelForAudioClassification` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoProcessor
>>> from optimum.neuron import NeuronModelForAudioClassification
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> feature_extractor = AutoProcessor.from_pretrained("Jingya/wav2vec2-large-960h-lv60-self-neuronx-audio-classification")
>>> model = NeuronModelForAudioClassification.from_pretrained("Jingya/wav2vec2-large-960h-lv60-self-neuronx-audio-classification")

>>> # audio file is decoded on the fly
>>> inputs = feature_extractor(dataset[0]["audio"]["array"], sampling_rate=sampling_rate, return_tensors="pt")

>>> logits = model(**inputs).logits
>>> predicted_class_ids = torch.argmax(logits, dim=-1).item()
>>> predicted_label = model.config.id2label[predicted_class_ids]
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

### NeuronModelForAudioFrameClassification[[optimum.neuron.NeuronModelForAudioFrameClassification]]
#### optimum.neuron.NeuronModelForAudioFrameClassification[[optimum.neuron.NeuronModelForAudioFrameClassification]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L557)

Neuron Model with an audio frame classification head.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Neuron Model with a frame classification head on top for tasks like Speaker Diarization.

forwardoptimum.neuron.NeuronModelForAudioFrameClassification.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L564[{"name": "input_values", "val": ": Tensor"}, {"name": "**kwargs", "val": ""}]- **input_values** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Float values of input raw speech waveform..
  Input values can be obtained from audio file loaded into an array using [`AutoProcessor`](https://huggingface.co/docs/transformers/en/model_doc/auto#transformers.AutoProcessor).0
The `NeuronModelForAudioFrameClassification` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoProcessor
>>> from optimum.neuron import NeuronModelForAudioFrameClassification
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> feature_extractor = AutoProcessor.from_pretrained("Jingya/wav2vec2-base-superb-sd-neuronx")
>>> model =  NeuronModelForAudioFrameClassification.from_pretrained("Jingya/wav2vec2-base-superb-sd-neuronx")

>>> inputs = feature_extractor(dataset[0]["audio"]["array"], return_tensors="pt", sampling_rate=sampling_rate)
>>> logits = model(**inputs).logits

>>> probabilities = torch.sigmoid(logits[0])
>>> labels = (probabilities > 0.5).long()
>>> labels[0].tolist()
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

### NeuronModelForCTC[[optimum.neuron.NeuronModelForCTC]]
#### optimum.neuron.NeuronModelForCTC[[optimum.neuron.NeuronModelForCTC]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L597)

Neuron Model with a connectionist temporal classification head.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Neuron Model with a language modeling head on top for Connectionist Temporal Classification (CTC).

forwardoptimum.neuron.NeuronModelForCTC.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L605[{"name": "input_values", "val": ": Tensor"}, {"name": "**kwargs", "val": ""}]- **input_values** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Float values of input raw speech waveform..
  Input values can be obtained from audio file loaded into an array using [`AutoProcessor`](https://huggingface.co/docs/transformers/en/model_doc/auto#transformers.AutoProcessor).0
The `NeuronModelForCTC` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoProcessor
>>> from optimum.neuron import NeuronModelForCTC
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> processor = AutoProcessor.from_pretrained("Jingya/wav2vec2-large-960h-lv60-self-neuronx-ctc")
>>> model = NeuronModelForCTC.from_pretrained("Jingya/wav2vec2-large-960h-lv60-self-neuronx-ctc")

>>> # audio file is decoded on the fly
>>> inputs = processor(dataset[0]["audio"]["array"], sampling_rate=sampling_rate, return_tensors="pt")
>>> logits = model(**inputs).logits
>>> predicted_ids = torch.argmax(logits, dim=-1)

>>> transcription = processor.batch_decode(predicted_ids)
```

Example using `optimum.neuron.pipeline`:

```python
>>> from transformers import AutoProcessor
>>> from optimum.neuron import NeuronModelForCTC, pipeline

>>> processor = AutoProcessor.from_pretrained("Jingya/wav2vec2-large-960h-lv60-self-neuronx-ctc")
>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")

>>> model = NeuronModelForCTC.from_pretrained("Jingya/wav2vec2-large-960h-lv60-self-neuronx-ctc")
>>> asr = pipeline("automatic-speech-recognition", model=model, feature_extractor=processor.feature_extractor, tokenizer=processor.tokenizer)
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

### NeuronModelForXVector[[optimum.neuron.NeuronModelForXVector]]
#### optimum.neuron.NeuronModelForXVector[[optimum.neuron.NeuronModelForXVector]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L638)

Neuron Model with an XVector feature extraction head on top for tasks like Speaker Verification.

This model inherits from `~neuron.modeling.NeuronTracedModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Neuron Model with an XVector feature extraction head on top for tasks like Speaker Verification.

forwardoptimum.neuron.NeuronModelForXVector.forwardhttps://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling.py#L645[{"name": "input_values", "val": ": Tensor"}, {"name": "**kwargs", "val": ""}]- **input_values** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Float values of input raw speech waveform..
  Input values can be obtained from audio file loaded into an array using [`AutoProcessor`](https://huggingface.co/docs/transformers/en/model_doc/auto#transformers.AutoProcessor).0
The `NeuronModelForXVector` forward method, overrides the `__call__` special method. Accepts only the inputs traced during the compilation step. Any additional inputs provided during inference will be ignored. To include extra inputs, recompile the model with those inputs specified.

Example:

```python
>>> from transformers import AutoProcessor
>>> from optimum.neuron import NeuronModelForXVector
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> feature_extractor = AutoProcessor.from_pretrained("Jingya/wav2vec2-base-superb-sv-neuronx")
>>> model = NeuronModelForXVector.from_pretrained("Jingya/wav2vec2-base-superb-sv-neuronx")

>>> inputs = feature_extractor(
...     [d["array"] for d in dataset[:2]["audio"]], sampling_rate=sampling_rate, return_tensors="pt", padding=True
... )
>>> embeddings = model(**inputs).embeddings

>>> embeddings = torch.nn.functional.normalize(embeddings, dim=-1)

>>> cosine_sim = torch.nn.CosineSimilarity(dim=-1)
>>> similarity = cosine_sim(embeddings[0], embeddings[1])
>>> threshold = 0.7
>>> if similarity >> round(similarity.item(), 2)
```

**Parameters:**

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `optimum.neuron.modeling.NeuronTracedModel.from_pretrained` method to load the model weights.

model (`torch.jit._script.ScriptModule`) : [torch.jit._script.ScriptModule](https://pytorch.org/docs/stable/generated/torch.jit.ScriptModule.html) is the TorchScript module with embedded NEFF(Neuron Executable File Format) compiled by neuron(x) compiler.

## Stable Diffusion

The following Neuron model classes are available for stable diffusion tasks.

### NeuronStableDiffusionPipeline[[optimum.neuron.NeuronStableDiffusionPipeline]]

#### optimum.neuron.NeuronStableDiffusionPipeline[[optimum.neuron.NeuronStableDiffusionPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1525)

__call__optimum.neuron.NeuronStableDiffusionPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1094[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

### NeuronStableDiffusionImg2ImgPipeline[[optimum.neuron.NeuronStableDiffusionImg2ImgPipeline]]

#### optimum.neuron.NeuronStableDiffusionImg2ImgPipeline[[optimum.neuron.NeuronStableDiffusionImg2ImgPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1538)

__call__optimum.neuron.NeuronStableDiffusionImg2ImgPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1094[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

### NeuronStableDiffusionInpaintPipeline[[optimum.neuron.NeuronStableDiffusionInpaintPipeline]]

#### optimum.neuron.NeuronStableDiffusionInpaintPipeline[[optimum.neuron.NeuronStableDiffusionInpaintPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1543)

__call__optimum.neuron.NeuronStableDiffusionInpaintPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1094[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

### NeuronLatentConsistencyModelPipeline[[optimum.neuron.NeuronLatentConsistencyModelPipeline]]

#### optimum.neuron.NeuronLatentConsistencyModelPipeline[[optimum.neuron.NeuronLatentConsistencyModelPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1556)

__call__optimum.neuron.NeuronLatentConsistencyModelPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1094[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

### NeuronStableDiffusionControlNetPipeline[[optimum.neuron.NeuronStableDiffusionControlNetPipeline]]

#### optimum.neuron.NeuronStableDiffusionControlNetPipeline[[optimum.neuron.NeuronStableDiffusionControlNetPipeline]]

[Source](https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/modeling_diffusion.py#L1561)

__call__optimum.neuron.NeuronStableDiffusionControlNetPipeline.__call__https://github.com/huggingface/optimum-neuron/blob/main/optimum/neuron/pipelines/diffusers/pipeline_controlnet.py#L33[{"name": "prompt", "val": ": str | list[str] | None = None"}, {"name": "image", "val": ": typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, typing.List[PIL.Image.Image], typing.List[numpy.ndarray], typing.List[torch.Tensor]] = None"}, {"name": "num_inference_steps", "val": ": int = 50"}, {"name": "timesteps", "val": ": list[int] | None = None"}, {"name": "sigmas", "val": ": list[float] | None = None"}, {"name": "guidance_scale", "val": ": float = 7.5"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_images_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "ip_adapter_image", "val": ": typing.Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, typing.List[PIL.Image.Image], typing.List[numpy.ndarray], typing.List[torch.Tensor], NoneType] = None"}, {"name": "ip_adapter_image_embeds", "val": ": list[torch.Tensor] | None = None"}, {"name": "output_type", "val": ": str = 'pil'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_conditioning_scale", "val": ": float | list[float] = 1.0"}, {"name": "guess_mode", "val": ": bool = False"}, {"name": "control_guidance_start", "val": ": float | list[float] = 0.0"}, {"name": "control_guidance_end", "val": ": float | list[float] = 1.0"}, {"name": "clip_skip", "val": ": int | None = None"}, {"name": "callback_on_step_end", "val": ": typing.Union[typing.Callable[[int, int, dict], NoneType], diffusers.callbacks.PipelineCallback, diffusers.callbacks.MultiPipelineCallbacks, NoneType] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": list[str] = ['latents']"}, {"name": "**kwargs", "val": ""}]- **prompt** (`str | list[str] | None`, defaults to `None`) --
  The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.
- **image** (`"PipelineImageInput" | None`, defaults to `None`) --
  The ControlNet input condition to provide guidance to the `unet` for generation. If the type is
  specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted
  as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or
  width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`,
  images must be passed as a list such that each element of the list can be correctly batched for input
  to a single ControlNet. When `prompt` is a list, and if a list of images is passed for a single
  ControlNet, each will be paired with each prompt in the `prompt` list. This also applies to multiple
  ControlNets, where a list of image lists can be passed to batch for each prompt and each ControlNet.
- **num_inference_steps** (`int`, defaults to 50) --
  The number of denoising steps. More denoising steps usually lead to a higher quality image at the
  expense of slower inference.
- **timesteps** (`list[int] | None`, defaults to `None`) --
  Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument
  in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is
  passed will be used. Must be in descending order.
- **sigmas** (`list[int] | None`, defaults to `None`) --
  Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in
  their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed
  will be used.
- **guidance_scale** (`float`, defaults to 7.5) --
  A higher guidance scale value encourages the model to generate images closely linked to the text
  `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str | list[str] | None`, defaults to `None`) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0`diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` or `tuple`If `return_dict` is `True`, `diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` is returned,
otherwise a `tuple` is returned where the first element is a list with the generated images and the
second element is a list of `bool`s indicating whether the corresponding generated image contains
"not-safe-for-work" (nsfw) content.

The call function to the pipeline for generation.

**Parameters:**

prompt (`str | list[str] | None`, defaults to `None`) : The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.

image (`"PipelineImageInput" | None`, defaults to `None`) : The ControlNet input condition to provide guidance to the `unet` for generation. If the type is specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted as an image. The dimensions of the output image defaults to `image`'s dimensions. If height and/or width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`, images must be passed as a list such that each element of the list can be correctly batched for input to a single ControlNet. When `prompt` is a list, and if a list of images is passed for a single ControlNet, each will be paired with each prompt in the `prompt` list. This also applies to multiple ControlNets, where a list of image lists can be passed to batch for each prompt and each ControlNet.

num_inference_steps (`int`, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int] | None`, defaults to `None`) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

sigmas (`list[int] | None`, defaults to `None`) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

guidance_scale (`float`, defaults to 7.5) : A higher guidance scale value encourages the model to generate images closely linked to the text `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.

negative_prompt (`str | list[str] | None`, defaults to `None`) : The prompt or prompts to guide what to not include in image generation. If not defined, you need to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale  1`.
- **negative_prompt** (`str | list[str] | None`, defaults to `None`) --
  The prompt or prompts to guide what to not include in image generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale 0`diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` or `tuple`If `return_dict` is `True`, `diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` is returned,
otherwise a `tuple` is returned containing the output images.

The call function to the pipeline for generation.

Examples:

**Parameters:**

prompt (`str | list[str]`, defaults to `None`) : The prompt or prompts to guide image generation. If not defined, you need to pass `prompt_embeds`.

prompt_2 (`str | list[str]`, defaults to `None`) : The prompt or prompts to be sent to `tokenizer_2` and `text_encoder_2`. If not defined, `prompt` is used in both text-encoders.

image (`PipelineImageInput | None`, defaults to `None`) : The ControlNet input condition to provide guidance to the `unet` for generation. If the type is specified as `torch.Tensor`, it is passed to ControlNet as is. `PIL.Image.Image` can also be accepted as an image. The dimensions of the output image default to `image`'s dimensions. If height and/or width are passed, `image` is resized accordingly. If multiple ControlNets are specified in `init`, images must be passed as a list such that each element of the list can be correctly batched for input to a single ControlNet.

num_inference_steps (`int`, defaults to 50) : The number of denoising steps. More denoising steps usually lead to a higher quality image at the expense of slower inference.

timesteps (`list[int] | None`, defaults to `None`) : Custom timesteps to use for the denoising process with schedulers which support a `timesteps` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used. Must be in descending order.

sigmas (`list[float] | None`, defaults to `None`) : Custom sigmas to use for the denoising process with schedulers which support a `sigmas` argument in their `set_timesteps` method. If not defined, the default behavior when `num_inference_steps` is passed will be used.

denoising_end (`float | None`, defaults to `None`) : When specified, determines the fraction (between 0.0 and 1.0) of the total denoising process to be completed before it is intentionally prematurely terminated. As a result, the returned sample will still retain a substantial amount of noise as determined by the discrete timesteps selected by the scheduler. The denoising_end parameter should ideally be utilized when this pipeline forms a part of a "Mixture of Denoisers" multi-pipeline setup, as elaborated in [**Refining the Image Output**](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_xl#refining-the-image-output)

guidance_scale (`float`, defaults to 5.0) : A higher guidance scale value encourages the model to generate images closely linked to the text `prompt` at the expense of lower image quality. Guidance scale is enabled when `guidance_scale > 1`.

negative_prompt (`str | list[str] | None`, defaults to `None`) : The prompt or prompts to guide what to not include in image generation. If not defined, you need to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

negative_prompt_2 (`str | list[str] | None`, defaults to `None`) : The prompt or prompts to guide what to not include in image generation. This is sent to `tokenizer_2` and `text_encoder_2`. If not defined, `negative_prompt` is used in both text-encoders.

num_images_per_prompt (`int`, defaults to 1) : The number of images to generate per prompt.

eta (`float`, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://arxiv.org/abs/2010.02502) paper. Only applies to the `diffusers.schedulers.DDIMScheduler`, and is ignored in other schedulers.

generator (`torch.Generator | list[torch.Generator] | None`, defaults to `None`) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor | None`, defaults to `None`) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for image generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

prompt_embeds (`torch.Tensor | None`, defaults to `None`) : Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, text embeddings are generated from the `prompt` input argument.

negative_prompt_embeds (`torch.Tensor | None`, defaults to `None`) : Pre-generated negative text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, `negative_prompt_embeds` are generated from the `negative_prompt` input argument.

pooled_prompt_embeds (`torch.Tensor | None`, defaults to `None`) : Pre-generated pooled text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, pooled text embeddings are generated from `prompt` input argument.

negative_pooled_prompt_embeds (`torch.Tensor | None`, defaults to `None`) : Pre-generated negative pooled text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not provided, pooled `negative_prompt_embeds` are generated from `negative_prompt` input argument.

ip_adapter_image (`PipelineImageInput | None`, defaults to `None`) : Optional image input to work with IP Adapters.

ip_adapter_image_embeds (`list[torch.Tensor] | None`, defaults to `None`) : Pre-generated image embeddings for IP-Adapter. It should be a list of length the same as the number of IP-adapters. Each element should be a tensor of shape `(batch_size, num_images, emb_dim)`. It should contain the negative image embedding if `do_classifier_free_guidance` is set to `True`. If not provided, embeddings are computed from the `ip_adapter_image` input argument.

output_type (`str | None`, defaults to `"pil"`) : The output format of the generated image. Choose between `PIL.Image` or `np.array`.

return_dict (`bool`, defaults to `True`) : Whether or not to return a `~pipelines.stable_diffusion.StableDiffusionPipelineOutput` instead of a plain tuple.

cross_attention_kwargs (`dict[str, Any] | None`, defaults to `None`) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined in [`self.processor`](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

controlnet_conditioning_scale (`float | list[float]`, defaults to 1.0) : The outputs of the ControlNet are multiplied by `controlnet_conditioning_scale` before they are added to the residual in the original `unet`. If multiple ControlNets are specified in `init`, you can set the corresponding scale as a list.

guess_mode (`bool`, defaults to `False`) : The ControlNet encoder tries to recognize the content of the input image even if you remove all prompts. A `guidance_scale` value between 3.0 and 5.0 is recommended.

control_guidance_start (`float | list[float]`, defaults to 0.0) : The percentage of total steps at which the ControlNet starts applying.

control_guidance_end (`float | list[float]`, defaults to 1.0) : The percentage of total steps at which the ControlNet stops applying.

original_size (`tuple[int, int] | None`, defaults to (1024, 1024)) : If `original_size` is not the same as `target_size`, the image will appear to be down- or upsampled. `original_size` defaults to `(height, width)` if not specified. Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).

crops_coords_top_left (`tuple[int, int]`, defaults to (0, 0)) : `crops_coords_top_left` can be used to generate an image that appears to be "cropped" from the position `crops_coords_top_left` downwards. Favorable, well-centered images are usually achieved by setting `crops_coords_top_left` to (0, 0). Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).

target_size (`tuple[int, int] | None`, defaults to `None`) : For most cases, `target_size` should be set to the desired height and width of the generated image. If not specified, it will default to `(height, width)`. Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).

negative_original_size (`tuple[int, int] | None`, defaults to `None`) : To negatively condition the generation process based on a specific image resolution. Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.

negative_crops_coords_top_left (`tuple[int, int]`, defaults to (0, 0)) : To negatively condition the generation process based on a specific crop coordinates. Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.

negative_target_size (`tuple[int, int] | None`, defaults to `None`) : To negatively condition the generation process based on a target image resolution. It should be the same as the `target_size` for most cases. Part of SDXL's micro-conditioning as explained in section 2.2 of [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952). For more information, refer to this issue thread: https://github.com/huggingface/diffusers/issues/4208.

clip_skip (`int | None`, defaults to `None`) : Number of layers to be skipped from CLIP while computing the prompt embeddings. A value of 1 means that the output of the pre-final layer will be used for computing the prompt embeddings.

callback_on_step_end (`Callable[[int, int, dict], None] | PipelineCallback | MultiPipelineCallbacks | None`, defaults to `None`) : A function or a subclass of `PipelineCallback` or `MultiPipelineCallbacks` that is called at the end of each denoising step during the inference with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int, callback_kwargs: dict)`. `callback_kwargs` will include a list of all tensors as specified by `callback_on_step_end_tensor_inputs`.

callback_on_step_end_tensor_inputs (`list[str]`, defaults to `["latents"]`) : The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the `._callback_tensor_inputs` attribute of your pipeline class.

**Returns:**

``diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` or `tuple``

If `return_dict` is `True`, `diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput` is returned,
otherwise a `tuple` is returned containing the output images.
