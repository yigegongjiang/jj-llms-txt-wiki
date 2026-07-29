# Models

## Generic model classes[[optimum.intel.openvino.modeling_base.OVBaseModel]]

#### optimum.intel.openvino.modeling_base.OVBaseModel[[optimum.intel.openvino.modeling_base.OVBaseModel]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L185)

Base OVModel class.

from_pretrainedoptimum.intel.openvino.modeling_base.OVBaseModel.from_pretrainedhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L556[{"name": "model_id", "val": ": typing.Union[str, pathlib.Path]"}, {"name": "export", "val": ": bool = False"}, {"name": "force_download", "val": ": bool = False"}, {"name": "token", "val": ": typing.Union[bool, str, NoneType] = None"}, {"name": "cache_dir", "val": ": str = '/home/runner/.cache/huggingface/hub'"}, {"name": "subfolder", "val": ": str = ''"}, {"name": "config", "val": ": typing.Optional[transformers.configuration_utils.PretrainedConfig] = None"}, {"name": "local_files_only", "val": ": bool = False"}, {"name": "trust_remote_code", "val": ": bool = False"}, {"name": "revision", "val": ": typing.Optional[str] = None"}, {"name": "**kwargs", "val": ""}]- **model_id** (`Union[str, Path]`) --
  Can be either:
  - A string, the *model id* of a pretrained model hosted inside a model repo on huggingface.co.
    Valid model ids can be located at the root-level, like `bert-base-uncased`, or namespaced under a
    user or organization name, like `dbmdz/bert-base-german-cased`.
  - A path to a *directory* containing a model saved using `~OptimizedModel.save_pretrained`,
    e.g., `./my_model_directory/`.
- **export** (`bool`, defaults to `False`) --
  Defines whether the provided `model_id` needs to be exported to the targeted format.
- **force_download** (`bool`, defaults to `True`) --
  Whether or not to force the (re-)download of the model weights and configuration files, overriding the
  cached versions if they exist.
- **token** (`Optional[Union[bool,str]]`, defaults to `None`) --
  The token to use as HTTP bearer authorization for remote files. If `True`, will use the token generated
  when running `huggingface-cli login` (stored in `huggingface_hub.constants.HF_TOKEN_PATH`).
- **cache_dir** (`Optional[str]`, defaults to `None`) --
  Path to a directory in which a downloaded pretrained model configuration should be cached if the
  standard cache should not be used.
- **subfolder** (`str`, defaults to `""`) --
  In case the relevant files are located inside a subfolder of the model repo either locally or on huggingface.co, you can
  specify the folder name here.
- **config** (`Optional[transformers.PretrainedConfig]`, defaults to `None`) --
  The model configuration.
- **local_files_only** (`Optional[bool]`, defaults to `False`) --
  Whether or not to only look at local files (i.e., do not try to download the model).
- **trust_remote_code** (`bool`, defaults to `False`) --
  Whether or not to allow for custom code defined on the Hub in their own modeling. This option should only be set
  to `True` for repositories you trust and in which you have read the code, as it will execute code present on
  the Hub on your local machine.
- **revision** (`Optional[str]`, defaults to `None`) --
  The specific model version to use. It can be a branch name, a tag name, or a commit id, since we use a
  git-based system for storing models and other artifacts on huggingface.co, so `revision` can be any
  identifier allowed by git.0

Instantiate a pretrained model from a pre-trained model configuration.

**Parameters:**

model_id (`Union[str, Path]`) : Can be either: - A string, the *model id* of a pretrained model hosted inside a model repo on huggingface.co. Valid model ids can be located at the root-level, like `bert-base-uncased`, or namespaced under a user or organization name, like `dbmdz/bert-base-german-cased`. - A path to a *directory* containing a model saved using `~OptimizedModel.save_pretrained`, e.g., `./my_model_directory/`.

export (`bool`, defaults to `False`) : Defines whether the provided `model_id` needs to be exported to the targeted format.

force_download (`bool`, defaults to `True`) : Whether or not to force the (re-)download of the model weights and configuration files, overriding the cached versions if they exist.

token (`Optional[Union[bool,str]]`, defaults to `None`) : The token to use as HTTP bearer authorization for remote files. If `True`, will use the token generated when running `huggingface-cli login` (stored in `huggingface_hub.constants.HF_TOKEN_PATH`).

cache_dir (`Optional[str]`, defaults to `None`) : Path to a directory in which a downloaded pretrained model configuration should be cached if the standard cache should not be used.

subfolder (`str`, defaults to `""`) : In case the relevant files are located inside a subfolder of the model repo either locally or on huggingface.co, you can specify the folder name here.

config (`Optional[transformers.PretrainedConfig]`, defaults to `None`) : The model configuration.

local_files_only (`Optional[bool]`, defaults to `False`) : Whether or not to only look at local files (i.e., do not try to download the model).

trust_remote_code (`bool`, defaults to `False`) : Whether or not to allow for custom code defined on the Hub in their own modeling. This option should only be set to `True` for repositories you trust and in which you have read the code, as it will execute code present on the Hub on your local machine.

revision (`Optional[str]`, defaults to `None`) : The specific model version to use. It can be a branch name, a tag name, or a commit id, since we use a git-based system for storing models and other artifacts on huggingface.co, so `revision` can be any identifier allowed by git.
#### reshape[[optimum.intel.openvino.modeling_base.OVBaseModel.reshape]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L934)

Propagates the given input shapes on the model's layers, fixing the inputs shapes of the model.

**Parameters:**

batch_size (`int`) : The batch size.

sequence_length (`int`) : The sequence length or number of channels.

height (`int`, *optional*) : The image height.

width (`int`, *optional*) : The image width.

## Natural Language Processing

The following classes are available for the following natural language processing tasks.

### OVModelForCausalLM[[optimum.intel.OVModelForCausalLM]]

#### optimum.intel.OVModelForCausalLM[[optimum.intel.OVModelForCausalLM]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_decoder.py#L453)

OpenVINO Model with a causal language modeling head on top (linear layer with weights tied to the input
embeddings).

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.intel.OVModelForCausalLM.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_decoder.py#L566[{"name": "input_ids", "val": ": LongTensor"}, {"name": "attention_mask", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "past_key_values", "val": ": typing.Optional[typing.Tuple[typing.Tuple[torch.FloatTensor]]] = None"}, {"name": "position_ids", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "token_type_ids", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "**kwargs", "val": ""}]

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.
#### generate[[optimum.intel.OVModelForCausalLM.generate]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_decoder.py#L746)

### OVModelForMaskedLM[[optimum.intel.OVModelForMaskedLM]]

#### optimum.intel.OVModelForMaskedLM[[optimum.intel.OVModelForMaskedLM]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L448)

OpenVINO Model with a MaskedLMOutput for masked language modeling tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.intel.OVModelForMaskedLM.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L455[{"name": "input_ids", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "attention_mask", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "token_type_ids", "val": ": typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor`), *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The [OVModelForMaskedLM](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForMaskedLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of masked language modeling using `transformers.pipelines`:
```python
>>> from transformers import AutoTokenizer, pipeline
>>> from optimum.intel import OVModelForMaskedLM

>>> tokenizer = AutoTokenizer.from_pretrained("roberta-base")
>>> model = OVModelForMaskedLM.from_pretrained("roberta-base", export=True)
>>> mask_token = tokenizer.mask_token
>>> pipe = pipeline("fill-mask", model=model, tokenizer=tokenizer)
>>> outputs = pipe("The goal of life is" + mask_token)
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

### OVModelForSeq2SeqLM[[optimum.intel.OVModelForSeq2SeqLM]]

#### optimum.intel.OVModelForSeq2SeqLM[[optimum.intel.OVModelForSeq2SeqLM]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L318)

Sequence-to-sequence model with a language modeling head for OpenVINO inference.

forwardoptimum.intel.OVModelForSeq2SeqLM.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L632[{"name": "input_ids", "val": ": LongTensor = None"}, {"name": "attention_mask", "val": ": typing.Optional[torch.FloatTensor] = None"}, {"name": "decoder_input_ids", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "decoder_attention_mask", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "encoder_outputs", "val": ": typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None"}, {"name": "past_key_values", "val": ": typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None"}, {"name": "cache_position", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "labels", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.LongTensor`) --
  Indices of input sequence tokens in the vocabulary of shape `(batch_size, encoder_sequence_length)`.
- **attention_mask** (`torch.LongTensor`) --
  Mask to avoid performing attention on padding token indices, of shape
  `(batch_size, encoder_sequence_length)`. Mask values selected in `[0, 1]`.
- **decoder_input_ids** (`torch.LongTensor`) --
  Indices of decoder input sequence tokens in the vocabulary of shape `(batch_size, decoder_sequence_length)`.
- **encoder_outputs** (`torch.FloatTensor`) --
  The encoder `last_hidden_state` of shape `(batch_size, encoder_sequence_length, hidden_size)`.
- **past_key_values** (`tuple(tuple(torch.FloatTensor), *optional*)` --
  Contains the precomputed key and value hidden states of the attention blocks used to speed up decoding.
  The tuple is of length `config.n_layers` with each tuple having 2 tensors of shape
  `(batch_size, num_heads, decoder_sequence_length, embed_size_per_head)` and 2 additional tensors of shape
  `(batch_size, num_heads, encoder_sequence_length, embed_size_per_head)`.0
The [OVModelForSeq2SeqLM](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForSeq2SeqLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of text generation:
```python
>>> from transformers import AutoTokenizer
>>> from optimum.intel import OVModelForSeq2SeqLM

>>> tokenizer = AutoTokenizer.from_pretrained("echarlaix/t5-small-openvino")
>>> model = OVModelForSeq2SeqLM.from_pretrained("echarlaix/t5-small-openvino")
>>> text = "He never went out without a book under his arm, and he often came back with two."
>>> inputs = tokenizer(text, return_tensors="pt")
>>> gen_tokens = model.generate(**inputs)
>>> outputs = tokenizer.batch_decode(gen_tokens)
```

Example using `transformers.pipeline`:
```python
>>> from transformers import AutoTokenizer, pipeline
>>> from optimum.intel import OVModelForSeq2SeqLM

>>> tokenizer = AutoTokenizer.from_pretrained("echarlaix/t5-small-openvino")
>>> model = OVModelForSeq2SeqLM.from_pretrained("echarlaix/t5-small-openvino")
>>> pipe = pipeline("translation_en_to_fr", model=model, tokenizer=tokenizer)
>>> text = "He never went out without a book under his arm, and he often came back with two."
>>> outputs = pipe(text)
```

**Parameters:**

encoder (`openvino.Model`) : The OpenVINO Runtime model associated to the encoder.

decoder (`openvino.Model`) : The OpenVINO Runtime model associated to the decoder.

decoder_with_past (`openvino.Model`) : The OpenVINO Runtime model associated  to the decoder with past key values.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is an instance of the configuration associated to the model. Initializing with a config file does not load the weights associated with the model, only the configuration.

### OVModelForQuestionAnswering[[optimum.intel.OVModelForQuestionAnswering]]

#### optimum.intel.OVModelForQuestionAnswering[[optimum.intel.OVModelForQuestionAnswering]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L219)

OpenVINO Model with a QuestionAnsweringModelOutput for extractive question-answering tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.intel.OVModelForQuestionAnswering.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L226[{"name": "input_ids", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "attention_mask", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "token_type_ids", "val": ": typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor`), *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The [OVModelForQuestionAnswering](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForQuestionAnswering) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of question answering using `transformers.pipeline`:
```python
>>> from transformers import AutoTokenizer, pipeline
>>> from optimum.intel import OVModelForQuestionAnswering

>>> tokenizer = AutoTokenizer.from_pretrained("distilbert-base-cased-distilled-squad")
>>> model = OVModelForQuestionAnswering.from_pretrained("distilbert-base-cased-distilled-squad", export=True)
>>> pipe = pipeline("question-answering", model=model, tokenizer=tokenizer)
>>> question, text = "Who was Jim Henson?", "Jim Henson was a nice puppet"
>>> outputs = pipe(question, text)
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

### OVModelForSequenceClassification[[optimum.intel.OVModelForSequenceClassification]]

#### optimum.intel.OVModelForSequenceClassification[[optimum.intel.OVModelForSequenceClassification]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L154)

OpenVINO Model with a SequenceClassifierOutput for sequence classification tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.intel.OVModelForSequenceClassification.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L161[{"name": "input_ids", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "attention_mask", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "token_type_ids", "val": ": typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor`), *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The [OVModelForSequenceClassification](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForSequenceClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of sequence classification using `transformers.pipeline`:
```python
>>> from transformers import AutoTokenizer, pipeline
>>> from optimum.intel import OVModelForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("distilbert-base-uncased-finetuned-sst-2-english")
>>> model = OVModelForSequenceClassification.from_pretrained("distilbert-base-uncased-finetuned-sst-2-english", export=True)
>>> pipe = pipeline("text-classification", model=model, tokenizer=tokenizer)
>>> outputs = pipe("Hello, my dog is cute")
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

### OVModelForTokenClassification[[optimum.intel.OVModelForTokenClassification]]

#### optimum.intel.OVModelForTokenClassification[[optimum.intel.OVModelForTokenClassification]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L288)

OpenVINO Model with a TokenClassifierOutput for token classification tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.intel.OVModelForTokenClassification.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L295[{"name": "input_ids", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "attention_mask", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "token_type_ids", "val": ": typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor`), *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The [OVModelForTokenClassification](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForTokenClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of token classification using `transformers.pipelines`:
```python
>>> from transformers import AutoTokenizer, pipeline
>>> from optimum.intel import OVModelForTokenClassification

>>> tokenizer = AutoTokenizer.from_pretrained("dslim/bert-base-NER")
>>> model = OVModelForTokenClassification.from_pretrained("dslim/bert-base-NER", export=True)
>>> pipe = pipeline("token-classification", model=model, tokenizer=tokenizer)
>>> outputs = pipe("My Name is Peter and I live in New York.")
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

## Audio

The following classes are available for the following audio tasks.

### OVModelForAudioClassification[[optimum.intel.OVModelForAudioClassification]]

#### optimum.intel.OVModelForAudioClassification[[optimum.intel.OVModelForAudioClassification]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L651)

OpenVINO Model with a SequenceClassifierOutput for audio classification tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.intel.OVModelForAudioClassification.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L658[{"name": "input_values", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "attention_mask", "val": ": typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor`), *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The [OVModelForAudioClassification](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForAudioClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of audio classification using `transformers.pipelines`:
```python
>>> from datasets import load_dataset
>>> from transformers import AutoFeatureExtractor, pipeline
>>> from optimum.intel import OVModelForAudioClassification

>>> preprocessor = AutoFeatureExtractor.from_pretrained("superb/hubert-base-superb-er")
>>> model = OVModelForAudioClassification.from_pretrained("superb/hubert-base-superb-er", export=True)
>>> pipe = pipeline("audio-classification", model=model, feature_extractor=preprocessor)
>>> dataset = load_dataset("superb", "ks", split="test")
>>> audio_file = dataset[3]["audio"]["array"]
>>> outputs = pipe(audio_file)
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

### OVModelForAudioFrameClassification[[optimum.intel.OVModelForAudioFrameClassification]]

#### optimum.intel.OVModelForAudioFrameClassification[[optimum.intel.OVModelForAudioFrameClassification]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L879)

OpenVINO Model for with a frame classification head on top for tasks like Speaker Diarization.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Audio Frame Classification model for OpenVINO.

forwardoptimum.intel.OVModelForAudioFrameClassification.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L887[{"name": "input_values", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ""}]- **input_values** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Float values of input raw speech waveform..
  Input values can be obtained from audio file loaded into an array using [`AutoFeatureExtractor`](https://huggingface.co/docs/transformers/autoclass_tutorial#autofeatureextractor).0
The [OVModelForAudioFrameClassification](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForAudioFrameClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of audio frame classification:

```python
>>> from transformers import AutoFeatureExtractor
>>> from optimum.intel import OVModelForAudioFrameClassification
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> feature_extractor = AutoFeatureExtractor.from_pretrained("anton-l/wav2vec2-base-superb-sd")
>>> model =  OVModelForAudioFrameClassification.from_pretrained("anton-l/wav2vec2-base-superb-sd", export=True)

>>> inputs = feature_extractor(dataset[0]["audio"]["array"], return_tensors="pt", sampling_rate=sampling_rate)
>>>    logits = model(**inputs).logits

>>> probabilities = torch.sigmoid(torch.as_tensor(logits)[0])
>>> labels = (probabilities > 0.5).long()
>>> labels[0].tolist()
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

### OVModelForCTC[[optimum.intel.OVModelForCTC]]

#### optimum.intel.OVModelForCTC[[optimum.intel.OVModelForCTC]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L723)

Onnx Model with a language modeling head on top for Connectionist Temporal Classification (CTC).

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

CTC model for OpenVINO.

forwardoptimum.intel.OVModelForCTC.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L731[{"name": "input_values", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "attention_mask", "val": ": typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None"}, {"name": "**kwargs", "val": ""}]- **input_values** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Float values of input raw speech waveform..
  Input values can be obtained from audio file loaded into an array using [`AutoFeatureExtractor`](https://huggingface.co/docs/transformers/autoclass_tutorial#autofeatureextractor).0
The [OVModelForCTC](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForCTC) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of CTC:

```python
>>> from transformers import AutoFeatureExtractor
>>> from optimum.intel import OVModelForCTC
>>> from datasets import load_dataset

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> processor = AutoFeatureExtractor.from_pretrained("facebook/hubert-large-ls960-ft")
>>> model = OVModelForCTC.from_pretrained("facebook/hubert-large-ls960-ft", export=True)

>>> # audio file is decoded on the fly
>>> inputs = processor(dataset[0]["audio"]["array"], sampling_rate=sampling_rate, return_tensors="np")
>>> logits = model(**inputs).logits
>>> predicted_ids = np.argmax(logits, axis=-1)

>>> transcription = processor.batch_decode(predicted_ids)
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

### OVModelForAudioXVector[[optimum.intel.OVModelForAudioXVector]]

#### optimum.intel.OVModelForAudioXVector[[optimum.intel.OVModelForAudioXVector]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L803)

Onnx Model with an XVector feature extraction head on top for tasks like Speaker Verification.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Audio XVector model for OpenVINO.

forwardoptimum.intel.OVModelForAudioXVector.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L811[{"name": "input_values", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ""}]- **input_values** (`torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Float values of input raw speech waveform..
  Input values can be obtained from audio file loaded into an array using [`AutoFeatureExtractor`](https://huggingface.co/docs/transformers/autoclass_tutorial#autofeatureextractor).0
The [OVModelForAudioXVector](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForAudioXVector) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of Audio XVector:

```python
>>> from transformers import AutoFeatureExtractor
>>> from optimum.intel import OVModelForAudioXVector
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> feature_extractor = AutoFeatureExtractor.from_pretrained("anton-l/wav2vec2-base-superb-sv")
>>> model = OVModelForAudioXVector.from_pretrained("anton-l/wav2vec2-base-superb-sv", export=True)

>>> # audio file is decoded on the fly
>>> inputs = feature_extractor(
...     [d["array"] for d in dataset[:2]["audio"]], sampling_rate=sampling_rate, return_tensors="pt", padding=True
... )
>>>     embeddings = model(**inputs).embeddings

>>> embeddings = torch.nn.functional.normalize(embeddings, dim=-1).cpu()

>>> cosine_sim = torch.nn.CosineSimilarity(dim=-1)
>>> similarity = cosine_sim(embeddings[0], embeddings[1])
>>> threshold = 0.7
>>> if similarity >> round(similarity.item(), 2)
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

### OVModelForSpeechSeq2Seq[[optimum.intel.OVModelForSpeechSeq2Seq]]

#### optimum.intel.OVModelForSpeechSeq2Seq[[optimum.intel.OVModelForSpeechSeq2Seq]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1234)

Speech Sequence-to-sequence model with a language modeling head for OpenVINO inference. This class officially supports whisper, speech_to_text.

forwardoptimum.intel.OVModelForSpeechSeq2Seq.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1270[{"name": "input_features", "val": ": typing.Optional[torch.FloatTensor] = None"}, {"name": "attention_mask", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "decoder_input_ids", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "decoder_attention_mask", "val": ": typing.Optional[torch.BoolTensor] = None"}, {"name": "encoder_outputs", "val": ": typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None"}, {"name": "past_key_values", "val": ": typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None"}, {"name": "cache_position", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "**kwargs", "val": ""}]- **input_features** (`torch.FloatTensor`) --
  Mel features extracted from the raw speech waveform.
  `(batch_size, feature_size, encoder_sequence_length)`.
- **decoder_input_ids** (`torch.LongTensor`) --
  Indices of decoder input sequence tokens in the vocabulary of shape `(batch_size, decoder_sequence_length)`.
- **encoder_outputs** (`torch.FloatTensor`) --
  The encoder `last_hidden_state` of shape `(batch_size, encoder_sequence_length, hidden_size)`.
- **past_key_values** (`tuple(tuple(torch.FloatTensor), *optional*, defaults to `None`)` --
  Contains the precomputed key and value hidden states of the attention blocks used to speed up decoding.
  The tuple is of length `config.n_layers` with each tuple having 2 tensors of shape
  `(batch_size, num_heads, decoder_sequence_length, embed_size_per_head)` and 2 additional tensors of shape
  `(batch_size, num_heads, encoder_sequence_length, embed_size_per_head)`.0
The [OVModelForSpeechSeq2Seq](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForSpeechSeq2Seq) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of text generation:

```python
>>> from transformers import AutoProcessor
>>> from optimum.intel import OVModelForSpeechSeq2Seq
>>> from datasets import load_dataset

>>> processor = AutoProcessor.from_pretrained("openai/whisper-tiny")
>>> model = OVModelForSpeechSeq2Seq.from_pretrained("openai/whisper-tiny")

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> inputs = processor.feature_extractor(ds[0]["audio"]["array"], return_tensors="pt")

>>> gen_tokens = model.generate(inputs=inputs.input_features)
>>> outputs = processor.tokenizer.batch_decode(gen_tokens)
```

Example using `transformers.pipeline`:

```python
>>> from transformers import AutoProcessor, pipeline
>>> from optimum.intel import OVModelForSpeechSeq2Seq
>>> from datasets import load_dataset

>>> processor = AutoProcessor.from_pretrained("openai/whisper-tiny")
>>> model = OVModelForSpeechSeq2Seq.from_pretrained("openai/whisper-tiny")
>>> speech_recognition = pipeline("automatic-speech-recognition", model=model, tokenizer=processor.tokenizer, feature_extractor=processor.feature_extractor)

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> pred = speech_recognition(ds[0]["audio"]["array"])
```

**Parameters:**

encoder (`openvino.Model`) : The OpenVINO Runtime model associated to the encoder.

decoder (`openvino.Model`) : The OpenVINO Runtime model associated to the decoder.

decoder_with_past (`openvino.Model`) : The OpenVINO Runtime model associated  to the decoder with past key values.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is an instance of the configuration associated to the model. Initializing with a config file does not load the weights associated with the model, only the configuration.

## Computer Vision

The following classes are available for the following computer vision tasks.

### OVModelForImageClassification[[optimum.intel.OVModelForImageClassification]]

#### optimum.intel.OVModelForImageClassification[[optimum.intel.OVModelForImageClassification]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L538)

OpenVINO Model with a ImageClassifierOutput for image classification tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.intel.OVModelForImageClassification.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L600[{"name": "pixel_values", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "**kwargs", "val": ""}]- **pixel_values** (`torch.Tensor`) --
  Pixel values corresponding to the images in the current batch.
  Pixel values can be obtained from encoded images using [`AutoFeatureExtractor`](https://huggingface.co/docs/transformers/autoclass_tutorial#autofeatureextractor).0
The [OVModelForImageClassification](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of image classification using `transformers.pipelines`:
```python
>>> from transformers import AutoFeatureExtractor, pipeline
>>> from optimum.intel import OVModelForImageClassification

>>> preprocessor = AutoFeatureExtractor.from_pretrained("google/vit-base-patch16-224")
>>> model = OVModelForImageClassification.from_pretrained("google/vit-base-patch16-224", export=True)
>>> model.reshape(batch_size=1, sequence_length=3, height=224, width=224)
>>> pipe = pipeline("image-classification", model=model, feature_extractor=preprocessor)
>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> outputs = pipe(url)
```

This class can also be used with [timm](https://github.com/huggingface/pytorch-image-models)

models hosted on [HuggingFaceHub](https://huggingface.co/timm). Example:
```python
>>> from transformers import pipeline
>>> from optimum.intel.openvino.modeling_timm import TimmImageProcessor
>>> from optimum.intel import OVModelForImageClassification

>>> model_id = "timm/vit_tiny_patch16_224.augreg_in21k"
>>> preprocessor = TimmImageProcessor.from_pretrained(model_id)
>>> model = OVModelForImageClassification.from_pretrained(model_id, export=True)
>>> pipe = pipeline("image-classification", model=model, feature_extractor=preprocessor)
>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> outputs = pipe(url)
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

## Multimodal

The following classes are available for the following multimodal tasks.

### OVModelForVision2Seq[[optimum.intel.OVModelForVision2Seq]]

#### optimum.intel.OVModelForVision2Seq[[optimum.intel.OVModelForVision2Seq]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1039)

VisionEncoderDecoder Sequence-to-sequence model with a language modeling head for OpenVINO inference.

forwardoptimum.intel.OVModelForVision2Seq.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1093[{"name": "pixel_values", "val": ": typing.Optional[torch.FloatTensor] = None"}, {"name": "attention_mask", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "decoder_input_ids", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "decoder_attention_mask", "val": ": typing.Optional[torch.BoolTensor] = None"}, {"name": "encoder_outputs", "val": ": typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None"}, {"name": "past_key_values", "val": ": typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None"}, {"name": "**kwargs", "val": ""}]- **pixel_values** (`torch.FloatTensor`) --
  Features extracted from an Image. This tensor should be of shape
  `(batch_size, num_channels, height, width)`.
- **decoder_input_ids** (`torch.LongTensor`) --
  Indices of decoder input sequence tokens in the vocabulary of shape `(batch_size, decoder_sequence_length)`.
- **encoder_outputs** (`torch.FloatTensor`) --
  The encoder `last_hidden_state` of shape `(batch_size, encoder_sequence_length, hidden_size)`.
- **past_key_values** (`tuple(tuple(torch.FloatTensor), *optional*, defaults to `None`)` --
  Contains the precomputed key and value hidden states of the attention blocks used to speed up decoding.
  The tuple is of length `config.n_layers` with each tuple having 2 tensors of shape
  `(batch_size, num_heads, decoder_sequence_length, embed_size_per_head)` and 2 additional tensors of shape
  `(batch_size, num_heads, encoder_sequence_length, embed_size_per_head)`.0
The [OVModelForVision2Seq](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForVision2Seq) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of text generation:

```python
>>> from transformers import AutoProcessor, AutoTokenizer
>>> from optimum.intel import OVModelForVision2Seq
>>> from PIL import Image
>>> import requests

>>> processor = AutoProcessor.from_pretrained("microsoft/trocr-small-handwritten")
>>> tokenizer = AutoTokenizer.from_pretrained("microsoft/trocr-small-handwritten")
>>> model = OVModelForVision2Seq.from_pretrained("microsoft/trocr-small-handwritten", export=True)

>>> url = "https://fki.tic.heia-fr.ch/static/img/a01-122-02-00.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)
>>> inputs = processor(image, return_tensors="pt")

>>> gen_tokens = model.generate(**inputs)
>>> outputs = tokenizer.batch_decode(gen_tokens, skip_special_tokens=True)

```

Example using `transformers.pipeline`:

```python
>>> from transformers import AutoProcessor, AutoTokenizer, pipeline
>>> from optimum.intel import OVModelForVision2Seq
>>> from PIL import Image
>>> import requests

>>> processor = AutoProcessor.from_pretrained("microsoft/trocr-small-handwritten")
>>> tokenizer = AutoTokenizer.from_pretrained("microsoft/trocr-small-handwritten")
>>> model = OVModelForVision2Seq.from_pretrained("microsoft/trocr-small-handwritten", export=True)

>>> url = "https://fki.tic.heia-fr.ch/static/img/a01-122-02-00.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)

>>> image_to_text = pipeline("image-to-text", model=model, tokenizer=tokenizer, feature_extractor=processor, image_processor=processor)
>>> pred = image_to_text(image)
```

**Parameters:**

encoder (`openvino.Model`) : The OpenVINO Runtime model associated to the encoder.

decoder (`openvino.Model`) : The OpenVINO Runtime model associated to the decoder.

decoder_with_past (`openvino.Model`) : The OpenVINO Runtime model associated  to the decoder with past key values.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is an instance of the configuration associated to the model. Initializing with a config file does not load the weights associated with the model, only the configuration.

### OVModelForPix2Struct[[optimum.intel.OVModelForPix2Struct]]

#### optimum.intel.OVModelForPix2Struct[[optimum.intel.OVModelForPix2Struct]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1144)

Pix2Struct model with a language modeling head for OpenVINO inference.

forwardoptimum.intel.OVModelForPix2Struct.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1184[{"name": "flattened_patches", "val": ": typing.Optional[torch.FloatTensor] = None"}, {"name": "attention_mask", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "decoder_input_ids", "val": ": typing.Optional[torch.LongTensor] = None"}, {"name": "decoder_attention_mask", "val": ": typing.Optional[torch.BoolTensor] = None"}, {"name": "encoder_outputs", "val": ": typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None"}, {"name": "past_key_values", "val": ": typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None"}, {"name": "**kwargs", "val": ""}]- **flattened_patches** (`torch.FloatTensor` of shape `(batch_size, seq_length, hidden_size)`) --
  Flattened pixel patches. the `hidden_size` is obtained by the following formula: `hidden_size` =
  `num_channels` * `patch_size` * `patch_size`
  The process of flattening the pixel patches is done by `Pix2StructProcessor`.
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices.
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Indices of decoder input sequence tokens in the vocabulary.
  Pix2StructText uses the `pad_token_id` as the starting token for `decoder_input_ids` generation. If
  `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see
  `past_key_values`).
- **decoder_attention_mask** (`torch.BoolTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also
  be used by default.
- **encoder_outputs** (`tuple(tuple(torch.FloatTensor)`, *optional*) --
  Tuple consists of (`last_hidden_state`, `optional`: *hidden_states*, `optional`: *attentions*)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)` is a sequence of hidden states at
  the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **past_key_values** (`tuple(tuple(torch.FloatTensor), *optional*, defaults to `None`)` --
  Contains the precomputed key and value hidden states of the attention blocks used to speed up decoding.
  The tuple is of length `config.n_layers` with each tuple having 2 tensors of shape
  `(batch_size, num_heads, decoder_sequence_length, embed_size_per_head)` and 2 additional tensors of shape
  `(batch_size, num_heads, encoder_sequence_length, embed_size_per_head)`.0
The [OVModelForPix2Struct](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForPix2Struct) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of pix2struct:

```python
>>> from transformers import AutoProcessor
>>> from optimum.intel import OVModelForPix2Struct
>>> from PIL import Image
>>> import requests

>>> processor = AutoProcessor.from_pretrained("google/pix2struct-ai2d-base")
>>> model = OVModelForPix2Struct.from_pretrained("google/pix2struct-ai2d-base", export=True)

>>> url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/tasks/ai2d-demo.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)
>>> question = "What does the label 15 represent? (1) lava (2) core (3) tunnel (4) ash cloud"
>>> inputs = processor(images=image, text=question, return_tensors="pt")

>>> gen_tokens = model.generate(**inputs)
>>> outputs = processor.batch_decode(gen_tokens, skip_special_tokens=True)
```

**Parameters:**

encoder (`openvino.Model`) : The OpenVINO Runtime model associated to the encoder.

decoder (`openvino.Model`) : The OpenVINO Runtime model associated to the decoder.

decoder_with_past (`openvino.Model`) : The OpenVINO Runtime model associated  to the decoder with past key values.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is an instance of the configuration associated to the model. Initializing with a config file does not load the weights associated with the model, only the configuration.

##  Custom Tasks

### OVModelForCustomTasks[[optimum.intel.OVModelForCustomTasks]]

#### optimum.intel.OVModelForCustomTasks[[optimum.intel.OVModelForCustomTasks]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L945)

OpenVINO Model for custom tasks. It can be used to leverage the inference acceleration for any single-file OpenVINO model, that may use custom inputs and outputs.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.intel.OVModelForCustomTasks.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L946[{"name": "**kwargs", "val": ""}]
The [OVModelForCustomTasks](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForCustomTasks) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of custom tasks (e.g. a sentence transformers with a pooler head):

```python
>>> from transformers import AutoTokenizer
>>> from optimum.intel import OVModelForCustomTasks

>>> tokenizer = AutoTokenizer.from_pretrained("IlyasMoutawwakil/sbert-all-MiniLM-L6-v2-with-pooler")
>>> model = OVModelForCustomTasks.from_pretrained("IlyasMoutawwakil/sbert-all-MiniLM-L6-v2-with-pooler")

>>> inputs = tokenizer("I love burritos!", return_tensors="np")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooler_output = outputs.pooler_output
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

### OVModelForFeatureExtraction[[optimum.intel.OVModelForFeatureExtraction]]

#### optimum.intel.OVModelForFeatureExtraction[[optimum.intel.OVModelForFeatureExtraction]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L352)

OpenVINO Model with a BaseModelOutput for feature extraction tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

forwardoptimum.intel.OVModelForFeatureExtraction.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L366[{"name": "input_ids", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "attention_mask", "val": ": typing.Union[torch.Tensor, numpy.ndarray]"}, {"name": "token_type_ids", "val": ": typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None"}, {"name": "**kwargs", "val": ""}]- **input_ids** (`torch.Tensor`) --
  Indices of input sequence tokens in the vocabulary.
  Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer).
  [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)
- **attention_mask** (`torch.Tensor`), *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:
  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.
  [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)
- **token_type_ids** (`torch.Tensor`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:
  - 1 for tokens that are **sentence A**,
  - 0 for tokens that are **sentence B**.
  [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)0
The [OVModelForFeatureExtraction](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForFeatureExtraction) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of feature extraction using `transformers.pipelines`:
```python
>>> from transformers import AutoTokenizer, pipeline
>>> from optimum.intel import OVModelForFeatureExtraction

>>> tokenizer = AutoTokenizer.from_pretrained("sentence-transformers/all-MiniLM-L6-v2")
>>> model = OVModelForFeatureExtraction.from_pretrained("sentence-transformers/all-MiniLM-L6-v2", export=True)
>>> pipe = pipeline("feature-extraction", model=model, tokenizer=tokenizer)
>>> outputs = pipe("My Name is Peter and I live in New York.")
```

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

## Text-to-image

### OVStableDiffusionPipeline[[optimum.intel.OVStableDiffusionPipeline]]

#### optimum.intel.OVStableDiffusionPipeline[[optimum.intel.OVStableDiffusionPipeline]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1443)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.StableDiffusionPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion#diffusers.StableDiffusionPipeline).

forwardoptimum.intel.OVStableDiffusionPipeline.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L974[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

### OVStableDiffusionXLPipeline[[optimum.intel.OVStableDiffusionXLPipeline]]

#### optimum.intel.OVStableDiffusionXLPipeline[[optimum.intel.OVStableDiffusionXLPipeline]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1477)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.StableDiffusionXLPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_xl#diffusers.StableDiffusionXLPipeline).

forwardoptimum.intel.OVStableDiffusionXLPipeline.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L974[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

### OVLatentConsistencyModelPipeline[[optimum.intel.OVLatentConsistencyModelPipeline]]

#### optimum.intel.OVLatentConsistencyModelPipeline[[optimum.intel.OVLatentConsistencyModelPipeline]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1578)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.LatentConsistencyModelPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/latent_consistency#diffusers.LatentConsistencyModelPipeline).

forwardoptimum.intel.OVLatentConsistencyModelPipeline.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L974[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

## Image-to-image

### OVStableDiffusionImg2ImgPipeline[[optimum.intel.OVStableDiffusionImg2ImgPipeline]]

#### optimum.intel.OVStableDiffusionImg2ImgPipeline[[optimum.intel.OVStableDiffusionImg2ImgPipeline]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1453)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.StableDiffusionImg2ImgPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_img2img#diffusers.StableDiffusionImg2ImgPipeline).

forwardoptimum.intel.OVStableDiffusionImg2ImgPipeline.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L974[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

### OVStableDiffusionXLImg2ImgPipeline[[optimum.intel.OVStableDiffusionXLImg2ImgPipeline]]

#### optimum.intel.OVStableDiffusionXLImg2ImgPipeline[[optimum.intel.OVStableDiffusionXLImg2ImgPipeline]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1500)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.StableDiffusionXLImg2ImgPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_xl#diffusers.StableDiffusionXLImg2ImgPipeline).

forwardoptimum.intel.OVStableDiffusionXLImg2ImgPipeline.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L974[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]

## Inpainting

### OVStableDiffusionInpaintPipeline[[optimum.intel.OVStableDiffusionInpaintPipeline]]

#### optimum.intel.OVStableDiffusionInpaintPipeline[[optimum.intel.OVStableDiffusionInpaintPipeline]]

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1465)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.StableDiffusionInpaintPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_inpaint#diffusers.StableDiffusionInpaintPipeline).

forwardoptimum.intel.OVStableDiffusionInpaintPipeline.forwardhttps://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L974[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]
