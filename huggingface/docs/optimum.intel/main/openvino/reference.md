# Models

## Generic model classes[[optimum.intel.openvino.modeling_base.OVBaseModel]]

#### optimum.intel.openvino.modeling_base.OVBaseModel[[optimum.intel.openvino.modeling_base.OVBaseModel]]

```python
optimum.intel.openvino.modeling_base.OVBaseModel(model: Model, config: PretrainedConfig = None, device: str = 'CPU', dynamic_shapes: bool = True, ov_config: typing.Optional[typing.Dict[str, str]] = None, model_save_dir: typing.Union[str, pathlib.Path, optimum.intel.openvino.utils.TemporaryDirectory, NoneType] = None, quantization_config: typing.Union[optimum.intel.openvino.configuration.OVWeightQuantizationConfig, typing.Dict, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L222)

Base OVModel class.

#### from_pretrained[[optimum.intel.openvino.modeling_base.OVBaseModel.from_pretrained]]

```python
from_pretrained(model_id: typing.Union[str, pathlib.Path], export: bool = False, force_download: bool = False, token: typing.Union[bool, str, NoneType] = None, cache_dir: str = '/home/runner/.cache/huggingface/hub', subfolder: str = '', config: typing.Optional[transformers.configuration_utils.PretrainedConfig] = None, local_files_only: bool = False, trust_remote_code: bool = False, revision: typing.Optional[str] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L593)

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

Instantiate a pretrained model from a pre-trained model configuration.

#### reshape[[optimum.intel.openvino.modeling_base.OVBaseModel.reshape]]

```python
reshape(batch_size: int, sequence_length: int, height: int = None, width: int = None)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L974)

**Parameters:**

batch_size (`int`) : The batch size.

sequence_length (`int`) : The sequence length or number of channels.

height (`int`, *optional*) : The image height.

width (`int`, *optional*) : The image width.

Propagates the given input shapes on the model's layers, fixing the inputs shapes of the model.

## Natural Language Processing

The following classes are available for the following natural language processing tasks.

### OVModelForCausalLM[[optimum.intel.OVModelForCausalLM]]

#### optimum.intel.OVModelForCausalLM[[optimum.intel.OVModelForCausalLM]]

```python
optimum.intel.OVModelForCausalLM(model: Model, config: PretrainedConfig = None, device: str = 'CPU', dynamic_shapes: bool = None, ov_config: typing.Optional[typing.Dict[str, str]] = None, model_save_dir: typing.Union[str, pathlib.Path, optimum.intel.openvino.utils.TemporaryDirectory, NoneType] = None, quantization_config: typing.Union[optimum.intel.openvino.configuration.OVWeightQuantizationConfig, typing.Dict, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_decoder.py#L464)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model with a causal language modeling head on top (linear layer with weights tied to the input
embeddings).

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

#### forward[[optimum.intel.OVModelForCausalLM.forward]]

```python
forward(input_ids: LongTensor, attention_mask: typing.Optional[torch.LongTensor] = None, past_key_values: typing.Optional[typing.Tuple[typing.Tuple[torch.FloatTensor]]] = None, position_ids: typing.Optional[torch.LongTensor] = None, token_type_ids: typing.Optional[torch.LongTensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_decoder.py#L577)

#### generate[[optimum.intel.OVModelForCausalLM.generate]]

```python
generate(inputs: typing.Optional[torch.Tensor] = None, generation_config: typing.Optional[transformers.generation.configuration_utils.GenerationConfig] = None, logits_processor: typing.Optional[transformers.generation.logits_process.LogitsProcessorList] = None, stopping_criteria: typing.Optional[transformers.generation.stopping_criteria.StoppingCriteriaList] = None, prefix_allowed_tokens_fn: typing.Optional[typing.Callable[[int, torch.Tensor], typing.List[int]]] = None, synced_gpus: typing.Optional[bool] = None, assistant_model: typing.Optional[ForwardRef('PreTrainedModel')] = None, streamer: typing.Optional[ForwardRef('BaseStreamer')] = None, negative_prompt_ids: typing.Optional[torch.Tensor] = None, negative_prompt_attention_mask: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_decoder.py#L758)

### OVModelForMaskedLM[[optimum.intel.OVModelForMaskedLM]]

#### optimum.intel.OVModelForMaskedLM[[optimum.intel.OVModelForMaskedLM]]

```python
optimum.intel.OVModelForMaskedLM(model = None, config = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L459)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model with a MaskedLMOutput for masked language modeling tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

#### forward[[optimum.intel.OVModelForMaskedLM.forward]]

```python
forward(input_ids: typing.Union[torch.Tensor, numpy.ndarray], attention_mask: typing.Union[torch.Tensor, numpy.ndarray], token_type_ids: typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L466)

**Parameters:**

input_ids (`torch.Tensor`) : Indices of input sequence tokens in the vocabulary. Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer). [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)

attention_mask (`torch.Tensor`), *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`: - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**. [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)

token_type_ids (`torch.Tensor`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`: - 1 for tokens that are **sentence A**, - 0 for tokens that are **sentence B**. [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)

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

### OVModelForSeq2SeqLM[[optimum.intel.OVModelForSeq2SeqLM]]

#### optimum.intel.OVModelForSeq2SeqLM[[optimum.intel.OVModelForSeq2SeqLM]]

```python
optimum.intel.OVModelForSeq2SeqLM(encoder: Model, decoder: Model, decoder_with_past: Model = None, config: PretrainedConfig = None, device: str = 'CPU', dynamic_shapes: bool = True, ov_config: typing.Optional[typing.Dict[str, str]] = None, model_save_dir: typing.Union[str, pathlib.Path, optimum.intel.openvino.utils.TemporaryDirectory, NoneType] = None, quantization_config: typing.Union[optimum.intel.openvino.configuration.OVWeightQuantizationConfig, typing.Dict] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L325)

**Parameters:**

encoder (`openvino.Model`) : The OpenVINO Runtime model associated to the encoder.

decoder (`openvino.Model`) : The OpenVINO Runtime model associated to the decoder.

decoder_with_past (`openvino.Model`) : The OpenVINO Runtime model associated  to the decoder with past key values.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is an instance of the configuration associated to the model. Initializing with a config file does not load the weights associated with the model, only the configuration.

Sequence-to-sequence model with a language modeling head for OpenVINO inference.

#### forward[[optimum.intel.OVModelForSeq2SeqLM.forward]]

```python
forward(input_ids: LongTensor = None, attention_mask: typing.Optional[torch.FloatTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None, past_key_values: typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None, cache_position: typing.Optional[torch.LongTensor] = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L667)

**Parameters:**

input_ids (`torch.LongTensor`) : Indices of input sequence tokens in the vocabulary of shape `(batch_size, encoder_sequence_length)`.

attention_mask (`torch.LongTensor`) : Mask to avoid performing attention on padding token indices, of shape `(batch_size, encoder_sequence_length)`. Mask values selected in `[0, 1]`.

decoder_input_ids (`torch.LongTensor`) : Indices of decoder input sequence tokens in the vocabulary of shape `(batch_size, decoder_sequence_length)`.

encoder_outputs (`torch.FloatTensor`) : The encoder `last_hidden_state` of shape `(batch_size, encoder_sequence_length, hidden_size)`.

past_key_values (`tuple(tuple(torch.FloatTensor), *optional*)` : Contains the precomputed key and value hidden states of the attention blocks used to speed up decoding. The tuple is of length `config.n_layers` with each tuple having 2 tensors of shape `(batch_size, num_heads, decoder_sequence_length, embed_size_per_head)` and 2 additional tensors of shape `(batch_size, num_heads, encoder_sequence_length, embed_size_per_head)`.

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

### OVModelForQuestionAnswering[[optimum.intel.OVModelForQuestionAnswering]]

#### optimum.intel.OVModelForQuestionAnswering[[optimum.intel.OVModelForQuestionAnswering]]

```python
optimum.intel.OVModelForQuestionAnswering(model = None, config = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L222)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model with a QuestionAnsweringModelOutput for extractive question-answering tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

#### forward[[optimum.intel.OVModelForQuestionAnswering.forward]]

```python
forward(input_ids: typing.Union[torch.Tensor, numpy.ndarray], attention_mask: typing.Union[torch.Tensor, numpy.ndarray], token_type_ids: typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L229)

**Parameters:**

input_ids (`torch.Tensor`) : Indices of input sequence tokens in the vocabulary. Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer). [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)

attention_mask (`torch.Tensor`), *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`: - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**. [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)

token_type_ids (`torch.Tensor`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`: - 1 for tokens that are **sentence A**, - 0 for tokens that are **sentence B**. [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)

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

### OVModelForSequenceClassification[[optimum.intel.OVModelForSequenceClassification]]

#### optimum.intel.OVModelForSequenceClassification[[optimum.intel.OVModelForSequenceClassification]]

```python
optimum.intel.OVModelForSequenceClassification(model = None, config = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L157)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model with a SequenceClassifierOutput for sequence classification tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

#### forward[[optimum.intel.OVModelForSequenceClassification.forward]]

```python
forward(input_ids: typing.Union[torch.Tensor, numpy.ndarray], attention_mask: typing.Union[torch.Tensor, numpy.ndarray], token_type_ids: typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L164)

**Parameters:**

input_ids (`torch.Tensor`) : Indices of input sequence tokens in the vocabulary. Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer). [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)

attention_mask (`torch.Tensor`), *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`: - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**. [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)

token_type_ids (`torch.Tensor`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`: - 1 for tokens that are **sentence A**, - 0 for tokens that are **sentence B**. [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)

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

### OVModelForTokenClassification[[optimum.intel.OVModelForTokenClassification]]

#### optimum.intel.OVModelForTokenClassification[[optimum.intel.OVModelForTokenClassification]]

```python
optimum.intel.OVModelForTokenClassification(model = None, config = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L291)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model with a TokenClassifierOutput for token classification tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

#### forward[[optimum.intel.OVModelForTokenClassification.forward]]

```python
forward(input_ids: typing.Union[torch.Tensor, numpy.ndarray], attention_mask: typing.Union[torch.Tensor, numpy.ndarray], token_type_ids: typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L298)

**Parameters:**

input_ids (`torch.Tensor`) : Indices of input sequence tokens in the vocabulary. Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer). [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)

attention_mask (`torch.Tensor`), *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`: - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**. [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)

token_type_ids (`torch.Tensor`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`: - 1 for tokens that are **sentence A**, - 0 for tokens that are **sentence B**. [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)

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

## Audio

The following classes are available for the following audio tasks.

### OVModelForAudioClassification[[optimum.intel.OVModelForAudioClassification]]

#### optimum.intel.OVModelForAudioClassification[[optimum.intel.OVModelForAudioClassification]]

```python
optimum.intel.OVModelForAudioClassification(model = None, config = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L662)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model with a SequenceClassifierOutput for audio classification tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

#### forward[[optimum.intel.OVModelForAudioClassification.forward]]

```python
forward(input_values: typing.Union[torch.Tensor, numpy.ndarray], attention_mask: typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L669)

**Parameters:**

input_ids (`torch.Tensor`) : Indices of input sequence tokens in the vocabulary. Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer). [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)

attention_mask (`torch.Tensor`), *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`: - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**. [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)

token_type_ids (`torch.Tensor`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`: - 1 for tokens that are **sentence A**, - 0 for tokens that are **sentence B**. [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)

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

### OVModelForAudioFrameClassification[[optimum.intel.OVModelForAudioFrameClassification]]

#### optimum.intel.OVModelForAudioFrameClassification[[optimum.intel.OVModelForAudioFrameClassification]]

```python
optimum.intel.OVModelForAudioFrameClassification(model: Model, config: PretrainedConfig = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L890)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model for with a frame classification head on top for tasks like Speaker Diarization.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Audio Frame Classification model for OpenVINO.

#### forward[[optimum.intel.OVModelForAudioFrameClassification.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L898)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform.. Input values can be obtained from audio file loaded into an array using [`AutoFeatureExtractor`](https://huggingface.co/docs/transformers/autoclass_tutorial#autofeatureextractor).

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

### OVModelForCTC[[optimum.intel.OVModelForCTC]]

#### optimum.intel.OVModelForCTC[[optimum.intel.OVModelForCTC]]

```python
optimum.intel.OVModelForCTC(model: Model, config: PretrainedConfig = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L734)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model with a language modeling head on top for Connectionist Temporal Classification (CTC).

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

CTC model for OpenVINO.

#### forward[[optimum.intel.OVModelForCTC.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor] = None, attention_mask: typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L742)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform.. Input values can be obtained from audio file loaded into an array using [`AutoFeatureExtractor`](https://huggingface.co/docs/transformers/autoclass_tutorial#autofeatureextractor).

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

### OVModelForAudioXVector[[optimum.intel.OVModelForAudioXVector]]

#### optimum.intel.OVModelForAudioXVector[[optimum.intel.OVModelForAudioXVector]]

```python
optimum.intel.OVModelForAudioXVector(model: Model, config: PretrainedConfig = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L814)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model with an XVector feature extraction head on top for tasks like Speaker Verification.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

Audio XVector model for OpenVINO.

#### forward[[optimum.intel.OVModelForAudioXVector.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L822)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform.. Input values can be obtained from audio file loaded into an array using [`AutoFeatureExtractor`](https://huggingface.co/docs/transformers/autoclass_tutorial#autofeatureextractor).

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
>>> if similarity < threshold:
...     print("Speakers are not the same!")
>>> round(similarity.item(), 2)
```

### OVModelForSpeechSeq2Seq[[optimum.intel.OVModelForSpeechSeq2Seq]]

#### optimum.intel.OVModelForSpeechSeq2Seq[[optimum.intel.OVModelForSpeechSeq2Seq]]

```python
optimum.intel.OVModelForSpeechSeq2Seq(encoder: Model, decoder: Model, decoder_with_past: Model = None, config: PretrainedConfig = None, device: str = 'CPU', dynamic_shapes: bool = True, ov_config: typing.Optional[typing.Dict[str, str]] = None, model_save_dir: typing.Union[str, pathlib.Path, optimum.intel.openvino.utils.TemporaryDirectory, NoneType] = None, quantization_config: typing.Union[optimum.intel.openvino.configuration.OVWeightQuantizationConfig, typing.Dict] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1326)

**Parameters:**

encoder (`openvino.Model`) : The OpenVINO Runtime model associated to the encoder.

decoder (`openvino.Model`) : The OpenVINO Runtime model associated to the decoder.

decoder_with_past (`openvino.Model`) : The OpenVINO Runtime model associated  to the decoder with past key values.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is an instance of the configuration associated to the model. Initializing with a config file does not load the weights associated with the model, only the configuration.

Speech Sequence-to-sequence model with a language modeling head for OpenVINO inference. This class officially supports whisper, speech_to_text.

#### forward[[optimum.intel.OVModelForSpeechSeq2Seq.forward]]

```python
forward(input_features: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.BoolTensor] = None, encoder_outputs: typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None, past_key_values: typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None, cache_position: typing.Optional[torch.LongTensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1400)

**Parameters:**

input_features (`torch.FloatTensor`) : Mel features extracted from the raw speech waveform. `(batch_size, feature_size, encoder_sequence_length)`.

decoder_input_ids (`torch.LongTensor`) : Indices of decoder input sequence tokens in the vocabulary of shape `(batch_size, decoder_sequence_length)`.

encoder_outputs (`torch.FloatTensor`) : The encoder `last_hidden_state` of shape `(batch_size, encoder_sequence_length, hidden_size)`.

past_key_values (`tuple(tuple(torch.FloatTensor), *optional*, defaults to `None`)` : Contains the precomputed key and value hidden states of the attention blocks used to speed up decoding. The tuple is of length `config.n_layers` with each tuple having 2 tensors of shape `(batch_size, num_heads, decoder_sequence_length, embed_size_per_head)` and 2 additional tensors of shape `(batch_size, num_heads, encoder_sequence_length, embed_size_per_head)`.

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

## Computer Vision

The following classes are available for the following computer vision tasks.

### OVModelForImageClassification[[optimum.intel.OVModelForImageClassification]]

#### optimum.intel.OVModelForImageClassification[[optimum.intel.OVModelForImageClassification]]

```python
optimum.intel.OVModelForImageClassification(model = None, config = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L549)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model with a ImageClassifierOutput for image classification tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

#### forward[[optimum.intel.OVModelForImageClassification.forward]]

```python
forward(pixel_values: typing.Union[torch.Tensor, numpy.ndarray], **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L611)

**Parameters:**

pixel_values (`torch.Tensor`) : Pixel values corresponding to the images in the current batch. Pixel values can be obtained from encoded images using [`AutoFeatureExtractor`](https://huggingface.co/docs/transformers/autoclass_tutorial#autofeatureextractor).

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

## Multimodal

The following classes are available for the following multimodal tasks.

### OVModelForImageTextToText[[optimum.intel.OVModelForImageTextToText]]

#### optimum.intel.OVModelForImageTextToText[[optimum.intel.OVModelForImageTextToText]]

```python
optimum.intel.OVModelForImageTextToText(encoder: Model, decoder: Model, decoder_with_past: Model = None, config: PretrainedConfig = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1119)

**Parameters:**

encoder (`openvino.Model`) : The OpenVINO Runtime model associated to the encoder.

decoder (`openvino.Model`) : The OpenVINO Runtime model associated to the decoder.

decoder_with_past (`openvino.Model`) : The OpenVINO Runtime model associated  to the decoder with past key values.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is an instance of the configuration associated to the model. Initializing with a config file does not load the weights associated with the model, only the configuration.

VisionEncoderDecoder Sequence-to-sequence model with a language modeling head for OpenVINO inference.

#### forward[[optimum.intel.OVModelForImageTextToText.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.BoolTensor] = None, encoder_outputs: typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None, past_key_values: typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1173)

**Parameters:**

pixel_values (`torch.FloatTensor`) : Features extracted from an Image. This tensor should be of shape `(batch_size, num_channels, height, width)`.

decoder_input_ids (`torch.LongTensor`) : Indices of decoder input sequence tokens in the vocabulary of shape `(batch_size, decoder_sequence_length)`.

encoder_outputs (`torch.FloatTensor`) : The encoder `last_hidden_state` of shape `(batch_size, encoder_sequence_length, hidden_size)`.

past_key_values (`tuple(tuple(torch.FloatTensor), *optional*, defaults to `None`)` : Contains the precomputed key and value hidden states of the attention blocks used to speed up decoding. The tuple is of length `config.n_layers` with each tuple having 2 tensors of shape `(batch_size, num_heads, decoder_sequence_length, embed_size_per_head)` and 2 additional tensors of shape `(batch_size, num_heads, encoder_sequence_length, embed_size_per_head)`.

The [OVModelForImageTextToText](/docs/optimum.intel/main/en/openvino/reference#optimum.intel.OVModelForImageTextToText) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example of text generation:

```python
>>> from transformers import AutoProcessor, AutoTokenizer
>>> from optimum.intel import OVModelForImageTextToText
>>> from PIL import Image
>>> import requests

>>> processor = AutoProcessor.from_pretrained("microsoft/trocr-small-handwritten")
>>> tokenizer = AutoTokenizer.from_pretrained("microsoft/trocr-small-handwritten")
>>> model = OVModelForImageTextToText.from_pretrained("microsoft/trocr-small-handwritten", export=True)

>>> url = "https://fki.tic.heia-fr.ch/static/img/a01-122-02-00.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)
>>> inputs = processor(image, return_tensors="pt")

>>> gen_tokens = model.generate(**inputs)
>>> outputs = tokenizer.batch_decode(gen_tokens, skip_special_tokens=True)

```

Example using `transformers.pipeline`:

```python
>>> from transformers import AutoProcessor, AutoTokenizer, pipeline
>>> from optimum.intel import OVModelForImageTextToText
>>> from PIL import Image
>>> import requests

>>> processor = AutoProcessor.from_pretrained("microsoft/trocr-small-handwritten")
>>> tokenizer = AutoTokenizer.from_pretrained("microsoft/trocr-small-handwritten")
>>> model = OVModelForImageTextToText.from_pretrained("microsoft/trocr-small-handwritten", export=True)

>>> url = "https://fki.tic.heia-fr.ch/static/img/a01-122-02-00.jpg"
>>> image = Image.open(requests.get(url, stream=True).raw)

>>> image_to_text = pipeline("image-to-text", model=model, tokenizer=tokenizer, feature_extractor=processor, image_processor=processor)
>>> pred = image_to_text(image)
```

### OVModelForPix2Struct[[optimum.intel.OVModelForPix2Struct]]

#### optimum.intel.OVModelForPix2Struct[[optimum.intel.OVModelForPix2Struct]]

```python
optimum.intel.OVModelForPix2Struct(encoder: Model, decoder: Model, decoder_with_past: Model = None, config: PretrainedConfig = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1224)

**Parameters:**

encoder (`openvino.Model`) : The OpenVINO Runtime model associated to the encoder.

decoder (`openvino.Model`) : The OpenVINO Runtime model associated to the decoder.

decoder_with_past (`openvino.Model`) : The OpenVINO Runtime model associated  to the decoder with past key values.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is an instance of the configuration associated to the model. Initializing with a config file does not load the weights associated with the model, only the configuration.

Pix2Struct model with a language modeling head for OpenVINO inference.

#### forward[[optimum.intel.OVModelForPix2Struct.forward]]

```python
forward(flattened_patches: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.BoolTensor] = None, encoder_outputs: typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None, past_key_values: typing.Optional[typing.Tuple[typing.Tuple[torch.Tensor]]] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_seq2seq.py#L1264)

**Parameters:**

flattened_patches (`torch.FloatTensor` of shape `(batch_size, seq_length, hidden_size)`) : Flattened pixel patches. the `hidden_size` is obtained by the following formula: `hidden_size` = `num_channels` * `patch_size` * `patch_size` The process of flattening the pixel patches is done by `Pix2StructProcessor`.

attention_mask (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices.

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary. Pix2StructText uses the `pad_token_id` as the starting token for `decoder_input_ids` generation. If `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see `past_key_values`).

decoder_attention_mask (`torch.BoolTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also be used by default.

encoder_outputs (`tuple(tuple(torch.FloatTensor)`, *optional*) : Tuple consists of (`last_hidden_state`, `optional`: *hidden_states*, `optional`: *attentions*) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)` is a sequence of hidden states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`tuple(tuple(torch.FloatTensor), *optional*, defaults to `None`)` : Contains the precomputed key and value hidden states of the attention blocks used to speed up decoding. The tuple is of length `config.n_layers` with each tuple having 2 tensors of shape `(batch_size, num_heads, decoder_sequence_length, embed_size_per_head)` and 2 additional tensors of shape `(batch_size, num_heads, encoder_sequence_length, embed_size_per_head)`.

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

##  Custom Tasks

### OVModelForCustomTasks[[optimum.intel.OVModelForCustomTasks]]

#### optimum.intel.OVModelForCustomTasks[[optimum.intel.OVModelForCustomTasks]]

```python
optimum.intel.OVModelForCustomTasks(model: Model, config: PretrainedConfig = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L956)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model for custom tasks. It can be used to leverage the inference acceleration for any single-file OpenVINO model, that may use custom inputs and outputs.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

#### forward[[optimum.intel.OVModelForCustomTasks.forward]]

```python
forward(**kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L957)

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

### OVModelForFeatureExtraction[[optimum.intel.OVModelForFeatureExtraction]]

#### optimum.intel.OVModelForFeatureExtraction[[optimum.intel.OVModelForFeatureExtraction]]

```python
optimum.intel.OVModelForFeatureExtraction(model = None, config = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L355)

**Parameters:**

model (`openvino.Model`) : is the main class used to run OpenVINO Runtime inference.

config (`transformers.PretrainedConfig`) : [PretrainedConfig](https://huggingface.co/docs/transformers/main_classes/configuration#transformers.PretrainedConfig) is the Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the `~intel.openvino.modeling.OVBaseModel.from_pretrained` method to load the model weights.

device (`str`, defaults to `"CPU"`) : The device type for which the model will be optimized for. The resulting compiled model will contains nodes specific to this device.

dynamic_shapes (`bool`, defaults to `True`) : All the model's dimension will be set to dynamic when set to `True`. Should be set to `False` for the model to not be dynamically reshaped by default.

ov_config (`Optional[Dict]`, defaults to `None`) : The dictionary containing the information related to the model compilation.

compile (`bool`, defaults to `True`) : Disable the model compilation during the loading step when set to `False`. Can be useful to avoid unnecessary compilation, in the case where the model needs to be statically reshaped, the device modified or if FP16 conversion is enabled.

OpenVINO Model with a BaseModelOutput for feature extraction tasks.

This model inherits from `optimum.intel.openvino.modeling.OVBaseModel`. Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving)

#### forward[[optimum.intel.OVModelForFeatureExtraction.forward]]

```python
forward(input_ids: typing.Union[torch.Tensor, numpy.ndarray], attention_mask: typing.Union[torch.Tensor, numpy.ndarray], token_type_ids: typing.Union[torch.Tensor, numpy.ndarray, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling.py#L369)

**Parameters:**

input_ids (`torch.Tensor`) : Indices of input sequence tokens in the vocabulary. Indices can be obtained using [`AutoTokenizer`](https://huggingface.co/docs/transformers/autoclass_tutorial#autotokenizer). [What are input IDs?](https://huggingface.co/docs/transformers/glossary#input-ids)

attention_mask (`torch.Tensor`), *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`: - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**. [What are attention masks?](https://huggingface.co/docs/transformers/glossary#attention-mask)

token_type_ids (`torch.Tensor`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`: - 1 for tokens that are **sentence A**, - 0 for tokens that are **sentence B**. [What are token type IDs?](https://huggingface.co/docs/transformers/glossary#token-type-ids)

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

## Text-to-image

### OVStableDiffusionPipeline[[optimum.intel.OVStableDiffusionPipeline]]

#### optimum.intel.OVStableDiffusionPipeline[[optimum.intel.OVStableDiffusionPipeline]]

```python
optimum.intel.OVStableDiffusionPipeline(scheduler: SchedulerMixin, unet: typing.Optional[openvino.Model] = None, vae_decoder: typing.Optional[openvino.Model] = None, vae_encoder: typing.Optional[openvino.Model] = None, text_encoder: typing.Optional[openvino.Model] = None, text_encoder_2: typing.Optional[openvino.Model] = None, text_encoder_3: typing.Optional[openvino.Model] = None, transformer: typing.Optional[openvino.Model] = None, connectors: typing.Optional[openvino.Model] = None, audio_vae_decoder: typing.Optional[openvino.Model] = None, vocoder: typing.Optional[openvino.Model] = None, tokenizer: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_2: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_3: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, feature_extractor: typing.Optional[transformers.models.clip.image_processing_clip.CLIPImageProcessor] = None, force_zeros_for_empty_prompt: bool = True, requires_aesthetics_score: bool = False, add_watermarker: typing.Optional[bool] = None, device: str = 'CPU', compile: bool = True, compile_only: bool = False, dynamic_shapes: bool = True, ov_config: typing.Optional[typing.Dict[str, str]] = None, model_save_dir: typing.Union[str, pathlib.Path, optimum.intel.openvino.utils.TemporaryDirectory, NoneType] = None, quantization_config: typing.Union[optimum.intel.openvino.configuration.OVWeightQuantizationConfig, typing.Dict, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1802)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.StableDiffusionPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion#diffusers.StableDiffusionPipeline).

#### forward[[optimum.intel.OVStableDiffusionPipeline.forward]]

```python
forward(*args, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L1014)

### OVStableDiffusionXLPipeline[[optimum.intel.OVStableDiffusionXLPipeline]]

#### optimum.intel.OVStableDiffusionXLPipeline[[optimum.intel.OVStableDiffusionXLPipeline]]

```python
optimum.intel.OVStableDiffusionXLPipeline(scheduler: SchedulerMixin, unet: typing.Optional[openvino.Model] = None, vae_decoder: typing.Optional[openvino.Model] = None, vae_encoder: typing.Optional[openvino.Model] = None, text_encoder: typing.Optional[openvino.Model] = None, text_encoder_2: typing.Optional[openvino.Model] = None, text_encoder_3: typing.Optional[openvino.Model] = None, transformer: typing.Optional[openvino.Model] = None, connectors: typing.Optional[openvino.Model] = None, audio_vae_decoder: typing.Optional[openvino.Model] = None, vocoder: typing.Optional[openvino.Model] = None, tokenizer: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_2: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_3: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, feature_extractor: typing.Optional[transformers.models.clip.image_processing_clip.CLIPImageProcessor] = None, force_zeros_for_empty_prompt: bool = True, requires_aesthetics_score: bool = False, add_watermarker: typing.Optional[bool] = None, device: str = 'CPU', compile: bool = True, compile_only: bool = False, dynamic_shapes: bool = True, ov_config: typing.Optional[typing.Dict[str, str]] = None, model_save_dir: typing.Union[str, pathlib.Path, optimum.intel.openvino.utils.TemporaryDirectory, NoneType] = None, quantization_config: typing.Union[optimum.intel.openvino.configuration.OVWeightQuantizationConfig, typing.Dict, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1836)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.StableDiffusionXLPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_xl#diffusers.StableDiffusionXLPipeline).

#### forward[[optimum.intel.OVStableDiffusionXLPipeline.forward]]

```python
forward(*args, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L1014)

### OVLatentConsistencyModelPipeline[[optimum.intel.OVLatentConsistencyModelPipeline]]

#### optimum.intel.OVLatentConsistencyModelPipeline[[optimum.intel.OVLatentConsistencyModelPipeline]]

```python
optimum.intel.OVLatentConsistencyModelPipeline(scheduler: SchedulerMixin, unet: typing.Optional[openvino.Model] = None, vae_decoder: typing.Optional[openvino.Model] = None, vae_encoder: typing.Optional[openvino.Model] = None, text_encoder: typing.Optional[openvino.Model] = None, text_encoder_2: typing.Optional[openvino.Model] = None, text_encoder_3: typing.Optional[openvino.Model] = None, transformer: typing.Optional[openvino.Model] = None, connectors: typing.Optional[openvino.Model] = None, audio_vae_decoder: typing.Optional[openvino.Model] = None, vocoder: typing.Optional[openvino.Model] = None, tokenizer: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_2: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_3: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, feature_extractor: typing.Optional[transformers.models.clip.image_processing_clip.CLIPImageProcessor] = None, force_zeros_for_empty_prompt: bool = True, requires_aesthetics_score: bool = False, add_watermarker: typing.Optional[bool] = None, device: str = 'CPU', compile: bool = True, compile_only: bool = False, dynamic_shapes: bool = True, ov_config: typing.Optional[typing.Dict[str, str]] = None, model_save_dir: typing.Union[str, pathlib.Path, optimum.intel.openvino.utils.TemporaryDirectory, NoneType] = None, quantization_config: typing.Union[optimum.intel.openvino.configuration.OVWeightQuantizationConfig, typing.Dict, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1937)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.LatentConsistencyModelPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/latent_consistency#diffusers.LatentConsistencyModelPipeline).

#### forward[[optimum.intel.OVLatentConsistencyModelPipeline.forward]]

```python
forward(*args, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L1014)

## Image-to-image

### OVStableDiffusionImg2ImgPipeline[[optimum.intel.OVStableDiffusionImg2ImgPipeline]]

#### optimum.intel.OVStableDiffusionImg2ImgPipeline[[optimum.intel.OVStableDiffusionImg2ImgPipeline]]

```python
optimum.intel.OVStableDiffusionImg2ImgPipeline(scheduler: SchedulerMixin, unet: typing.Optional[openvino.Model] = None, vae_decoder: typing.Optional[openvino.Model] = None, vae_encoder: typing.Optional[openvino.Model] = None, text_encoder: typing.Optional[openvino.Model] = None, text_encoder_2: typing.Optional[openvino.Model] = None, text_encoder_3: typing.Optional[openvino.Model] = None, transformer: typing.Optional[openvino.Model] = None, connectors: typing.Optional[openvino.Model] = None, audio_vae_decoder: typing.Optional[openvino.Model] = None, vocoder: typing.Optional[openvino.Model] = None, tokenizer: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_2: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_3: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, feature_extractor: typing.Optional[transformers.models.clip.image_processing_clip.CLIPImageProcessor] = None, force_zeros_for_empty_prompt: bool = True, requires_aesthetics_score: bool = False, add_watermarker: typing.Optional[bool] = None, device: str = 'CPU', compile: bool = True, compile_only: bool = False, dynamic_shapes: bool = True, ov_config: typing.Optional[typing.Dict[str, str]] = None, model_save_dir: typing.Union[str, pathlib.Path, optimum.intel.openvino.utils.TemporaryDirectory, NoneType] = None, quantization_config: typing.Union[optimum.intel.openvino.configuration.OVWeightQuantizationConfig, typing.Dict, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1812)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.StableDiffusionImg2ImgPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_img2img#diffusers.StableDiffusionImg2ImgPipeline).

#### forward[[optimum.intel.OVStableDiffusionImg2ImgPipeline.forward]]

```python
forward(*args, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L1014)

### OVStableDiffusionXLImg2ImgPipeline[[optimum.intel.OVStableDiffusionXLImg2ImgPipeline]]

#### optimum.intel.OVStableDiffusionXLImg2ImgPipeline[[optimum.intel.OVStableDiffusionXLImg2ImgPipeline]]

```python
optimum.intel.OVStableDiffusionXLImg2ImgPipeline(scheduler: SchedulerMixin, unet: typing.Optional[openvino.Model] = None, vae_decoder: typing.Optional[openvino.Model] = None, vae_encoder: typing.Optional[openvino.Model] = None, text_encoder: typing.Optional[openvino.Model] = None, text_encoder_2: typing.Optional[openvino.Model] = None, text_encoder_3: typing.Optional[openvino.Model] = None, transformer: typing.Optional[openvino.Model] = None, connectors: typing.Optional[openvino.Model] = None, audio_vae_decoder: typing.Optional[openvino.Model] = None, vocoder: typing.Optional[openvino.Model] = None, tokenizer: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_2: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_3: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, feature_extractor: typing.Optional[transformers.models.clip.image_processing_clip.CLIPImageProcessor] = None, force_zeros_for_empty_prompt: bool = True, requires_aesthetics_score: bool = False, add_watermarker: typing.Optional[bool] = None, device: str = 'CPU', compile: bool = True, compile_only: bool = False, dynamic_shapes: bool = True, ov_config: typing.Optional[typing.Dict[str, str]] = None, model_save_dir: typing.Union[str, pathlib.Path, optimum.intel.openvino.utils.TemporaryDirectory, NoneType] = None, quantization_config: typing.Union[optimum.intel.openvino.configuration.OVWeightQuantizationConfig, typing.Dict, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1859)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.StableDiffusionXLImg2ImgPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_xl#diffusers.StableDiffusionXLImg2ImgPipeline).

#### forward[[optimum.intel.OVStableDiffusionXLImg2ImgPipeline.forward]]

```python
forward(*args, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L1014)

## Inpainting

### OVStableDiffusionInpaintPipeline[[optimum.intel.OVStableDiffusionInpaintPipeline]]

#### optimum.intel.OVStableDiffusionInpaintPipeline[[optimum.intel.OVStableDiffusionInpaintPipeline]]

```python
optimum.intel.OVStableDiffusionInpaintPipeline(scheduler: SchedulerMixin, unet: typing.Optional[openvino.Model] = None, vae_decoder: typing.Optional[openvino.Model] = None, vae_encoder: typing.Optional[openvino.Model] = None, text_encoder: typing.Optional[openvino.Model] = None, text_encoder_2: typing.Optional[openvino.Model] = None, text_encoder_3: typing.Optional[openvino.Model] = None, transformer: typing.Optional[openvino.Model] = None, connectors: typing.Optional[openvino.Model] = None, audio_vae_decoder: typing.Optional[openvino.Model] = None, vocoder: typing.Optional[openvino.Model] = None, tokenizer: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_2: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, tokenizer_3: typing.Optional[transformers.models.clip.tokenization_clip.CLIPTokenizer] = None, feature_extractor: typing.Optional[transformers.models.clip.image_processing_clip.CLIPImageProcessor] = None, force_zeros_for_empty_prompt: bool = True, requires_aesthetics_score: bool = False, add_watermarker: typing.Optional[bool] = None, device: str = 'CPU', compile: bool = True, compile_only: bool = False, dynamic_shapes: bool = True, ov_config: typing.Optional[typing.Dict[str, str]] = None, model_save_dir: typing.Union[str, pathlib.Path, optimum.intel.openvino.utils.TemporaryDirectory, NoneType] = None, quantization_config: typing.Union[optimum.intel.openvino.configuration.OVWeightQuantizationConfig, typing.Dict, NoneType] = None, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_diffusion.py#L1824)

OpenVINO-powered stable diffusion pipeline corresponding to [diffusers.StableDiffusionInpaintPipeline](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/stable_diffusion_inpaint#diffusers.StableDiffusionInpaintPipeline).

#### forward[[optimum.intel.OVStableDiffusionInpaintPipeline.forward]]

```python
forward(*args, **kwargs)
```

[Source](https://github.com/huggingface/optimum-intel/blob/main/optimum/intel/openvino/modeling_base.py#L1014)
