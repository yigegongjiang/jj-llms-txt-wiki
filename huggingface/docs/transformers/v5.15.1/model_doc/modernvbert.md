# ModernVBert

## Overview

ModernVBert is a Vision-Language encoder that combines [ModernBert](modernbert) with a [SigLIP](siglip) vision encoder. It is optimized for visual document understanding and retrieval tasks.

The model was introduced in [ModernVBERT: Towards Smaller Visual Document Retrievers](https://huggingface.co/papers/2510.01149).

```python
import torch
from huggingface_hub import hf_hub_download
from PIL import Image

from transformers import AutoModelForMaskedLM, AutoProcessor

processor = AutoProcessor.from_pretrained("./mvb")
model = AutoModelForMaskedLM.from_pretrained("./mvb", device_map="auto")

image = Image.open(hf_hub_download("HuggingFaceTB/SmolVLM", "example_images/rococo.jpg", repo_type="space"))
text = "This [MASK] is on the wall."

# Create input messages
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image"},
            {"type": "text", "text": text}
        ]
    },
]

# Prepare inputs
prompt = processor.apply_chat_template(messages)
inputs = processor(text=prompt, images=[image], return_tensors="pt").to(model.device)

# Inference
with torch.no_grad():
  outputs = model(**inputs)

# To get predictions for the mask:
masked_index = inputs["input_ids"][0].tolist().index(processor.tokenizer.mask_token_id)
predicted_token_id = outputs.logits[0, masked_index].argmax(axis=-1)
predicted_token = processor.tokenizer.decode(predicted_token_id)
print("Predicted token:", predicted_token)  # Predicted token: painting
```

## ModernVBertConfig[[transformers.ModernVBertConfig]]

#### transformers.ModernVBertConfig[[transformers.ModernVBertConfig]]

```python
transformers.ModernVBertConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: transformers.configuration_utils.PreTrainedConfig | dict | None = None, vision_config: transformers.configuration_utils.PreTrainedConfig | dict | None = None, image_token_id: int = 50407, pixel_shuffle_factor: int = 4, initializer_range: float = 0.02, initializer_cutoff_factor: float = 2.0, classifier_pooling: typing.Literal['cls', 'mean'] = 'cls', classifier_dropout: float | int = 0.0, classifier_bias: bool = False, tie_word_embeddings: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/modernvbert/configuration_modernvbert.py#L32)

**Parameters:**

text_config (`Union[~configuration_utils.PreTrainedConfig, dict]`, *optional*) : The config object or dictionary of the text backbone.

vision_config (`Union[~configuration_utils.PreTrainedConfig, dict]`, *optional*) : The config object or dictionary of the vision backbone.

image_token_id (`int`, *optional*, defaults to `50407`) : The image token index used as a placeholder for input images.

pixel_shuffle_factor (`int | None`, *optional*, defaults to 4) : Scale factor used by any pixel-shuffle / upsampling operations in the vision head.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

initializer_cutoff_factor (`float | None`, *optional*, defaults to 2.0) : The cutoff factor for the truncated_normal_initializer for initializing all weight matrices.

classifier_pooling (`Literal["cls", "mean"]`, *optional*, defaults to `"cls"`) : The pooling strategy to use for classification tasks.

classifier_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for classifier.

classifier_bias (`bool | None`, *optional*, defaults to `False`) : Whether to add a bias term to the classification head

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a ModernVBertModel. It is used to instantiate a Modernvbert
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ModernVBERT/modernvbert](https://huggingface.co/ModernVBERT/modernvbert)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import ModernVBertConfig

>>> # Initializing configuration
>>> configuration = ModernVBertConfig()

>>> # Initializing a model from the configuration (model class is implemented in
>>> # `modernvbert.modeling_modernvbert`)

>>> from transformers import ModernVBertModel
>>> model = ModernVBertModel(configuration)

>>> # Accessing the model configuration
>>> cfg = model.config
```

## ModernVBertModel[[transformers.ModernVBertModel]]

#### transformers.ModernVBertModel[[transformers.ModernVBertModel]]

```python
transformers.ModernVBertModel(config: ModernVBertConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/modernvbert/modeling_modernvbert.py#L202)

**Parameters:**

config ([ModernVBertConfig](/docs/transformers/v5.15.1/en/model_doc/modernvbert#transformers.ModernVBertConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ModernVBertModel is a model that combines a vision encoder (SigLIP) and a text encoder (ModernBert).

ModernVBert is the base model of the visual retriever ColModernVBert, and was introduced in the following paper:
[*ModernVBERT: Towards Smaller Visual Document Retrievers*](https://arxiv.org/abs/2510.01149).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ModernVBertModel.forward]]

```python
forward(input_ids: LongTensor = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_attention_mask: typing.Optional[torch.BoolTensor] = None, image_hidden_states: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/modernvbert/modeling_modernvbert.py#L330)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details ([Idefics3Processor](/docs/transformers/v5.15.1/en/model_doc/idefics3#transformers.Idefics3Processor) uses `image_processor_class` for processing images).

pixel_attention_mask (`torch.Tensor` of shape `(batch_size, image_size, image_size)`, *optional*) : Mask to avoid performing attention on padding pixel indices.

image_hidden_states (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The hidden states of the image encoder after modality projection.

**Returns:** `ModernVBertBaseModelOutput` or `tuple(torch.FloatTensor)`

A `ModernVBertBaseModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ModernVBertConfig](/docs/transformers/v5.15.1/en/model_doc/modernvbert#transformers.ModernVBertConfig)) and inputs.

Inputs fed to the model can have an arbitrary number of images. To account for this, pixel_values fed to
the model have image padding -> (batch_size, max_num_images, 3, max_heights, max_widths) where
max_num_images is the maximum number of images among the batch_size samples in the batch.
Padding images are not needed beyond padding the pixel_values at the entrance of the model.
For efficiency, we only pass through the vision_model's forward the real images by
discarding the padding images i.e. pixel_values of size (image_batch_size, 3, height, width) where
image_batch_size would be 7 when num_images_per_sample=[1, 3, 1, 2] and max_num_images would be 3.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.
  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.
  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the image embeddings, `(batch_size, num_images,
  sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder

## ModernVBertForMaskedLM[[transformers.ModernVBertForMaskedLM]]

#### transformers.ModernVBertForMaskedLM[[transformers.ModernVBertForMaskedLM]]

```python
transformers.ModernVBertForMaskedLM(config: ModernVBertConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/modernvbert/modeling_modernvbert.py#L406)

**Parameters:**

config ([ModernVBertConfig](/docs/transformers/v5.15.1/en/model_doc/modernvbert#transformers.ModernVBertConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Modernvbert Model with a `language modeling` head on top."

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ModernVBertForMaskedLM.forward]]

```python
forward(input_ids: LongTensor = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_attention_mask: typing.Optional[torch.BoolTensor] = None, image_hidden_states: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/modernvbert/modeling_modernvbert.py#L427)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details ([Idefics3Processor](/docs/transformers/v5.15.1/en/model_doc/idefics3#transformers.Idefics3Processor) uses `image_processor_class` for processing images).

pixel_attention_mask (`torch.Tensor` of shape `(batch_size, image_size, image_size)`, *optional*) : Mask to avoid performing attention on padding pixel indices.

image_hidden_states (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The hidden states of the image encoder after modality projection.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., text_config.]` or `model.image_token_id`. Tokens with indices set to `model.image_token_id` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., text_config.]`.

**Returns:** `ModernVBertMaskedLMOutput` or `tuple(torch.FloatTensor)`

A `ModernVBertMaskedLMOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ModernVBertConfig](/docs/transformers/v5.15.1/en/model_doc/modernvbert#transformers.ModernVBertConfig)) and inputs.

Inputs fed to the model can have an arbitrary number of images. To account for this, pixel_values fed to
the model have image padding -> (batch_size, max_num_images, 3, max_heights, max_widths) where
max_num_images is the maximum number of images among the batch_size samples in the batch.
Padding images are not needed beyond padding the pixel_values at the entrance of the model.
For efficiency, we only pass through the vision_model's forward the real images by
discarding the padding images i.e. pixel_values of size (image_batch_size, 3, height, width) where
image_batch_size would be 7 when num_images_per_sample=[1, 3, 1, 2] and max_num_images would be 3.

- **loss** (`torch.FloatTensor`, *optional*, returned when `labels` is provided) -- Masked language modeling (MLM) loss.
- **logits** (`torch.FloatTensor`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.
  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.
  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the image embeddings, `(batch_size, num_images,
  sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder

Example:

```python
>>> from transformers import AutoTokenizer, ModernVBertForMaskedLM
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("ModernVBERT/modernvbert")
>>> model = ModernVBertForMaskedLM.from_pretrained("ModernVBERT/modernvbert")

>>> inputs = tokenizer("The capital of France is <mask>.", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # retrieve index of <mask>
>>> mask_token_index = (inputs.input_ids == tokenizer.mask_token_id)[0].nonzero(as_tuple=True)[0]

>>> predicted_token_id = logits[0, mask_token_index].argmax(axis=-1)
>>> tokenizer.decode(predicted_token_id)
...

>>> labels = tokenizer("The capital of France is Paris.", return_tensors="pt")["input_ids"]
>>> # mask labels of non-<mask> tokens
>>> labels = torch.where(inputs.input_ids == tokenizer.mask_token_id, labels, -100)

>>> outputs = model(**inputs, labels=labels)
>>> round(outputs.loss.item(), 2)
...
```

## ModernVBertForSequenceClassification[[transformers.ModernVBertForSequenceClassification]]

#### transformers.ModernVBertForSequenceClassification[[transformers.ModernVBertForSequenceClassification]]

```python
transformers.ModernVBertForSequenceClassification(config: ModernVBertConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/modernvbert/modeling_modernvbert.py#L496)

**Parameters:**

config ([ModernVBertConfig](/docs/transformers/v5.15.1/en/model_doc/modernvbert#transformers.ModernVBertConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The ModernVBert Model with a sequence classification head on top that performs pooling.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ModernVBertForSequenceClassification.forward]]

```python
forward(input_ids: LongTensor = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_attention_mask: typing.Optional[torch.BoolTensor] = None, image_hidden_states: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/modernvbert/modeling_modernvbert.py#L510)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details ([Idefics3Processor](/docs/transformers/v5.15.1/en/model_doc/idefics3#transformers.Idefics3Processor) uses `image_processor_class` for processing images).

pixel_attention_mask (`torch.Tensor` of shape `(batch_size, image_size, image_size)`, *optional*) : Mask to avoid performing attention on padding pixel indices.

image_hidden_states (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The hidden states of the image encoder after modality projection.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., text_config.]` or `model.image_token_id`. Tokens with indices set to `model.image_token_id` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., text_config.]`.

**Returns:** [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`

A [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ModernVBertConfig](/docs/transformers/v5.15.1/en/model_doc/modernvbert#transformers.ModernVBertConfig)) and inputs.

Inputs fed to the model can have an arbitrary number of images. To account for this, pixel_values fed to
the model have image padding -> (batch_size, max_num_images, 3, max_heights, max_widths) where
max_num_images is the maximum number of images among the batch_size samples in the batch.
Padding images are not needed beyond padding the pixel_values at the entrance of the model.
For efficiency, we only pass through the vision_model's forward the real images by
discarding the padding images i.e. pixel_values of size (image_batch_size, 3, height, width) where
image_batch_size would be 7 when num_images_per_sample=[1, 3, 1, 2] and max_num_images would be 3.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example of single-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, ModernVBertForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("ModernVBERT/modernvbert")
>>> model = ModernVBertForSequenceClassification.from_pretrained("ModernVBERT/modernvbert")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> model.config.id2label[predicted_class_id]
...

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = ModernVBertForSequenceClassification.from_pretrained("ModernVBERT/modernvbert", num_labels=num_labels)

>>> labels = torch.tensor([1])
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

Example of multi-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, ModernVBertForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("ModernVBERT/modernvbert")
>>> model = ModernVBertForSequenceClassification.from_pretrained("ModernVBERT/modernvbert", problem_type="multi_label_classification")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_ids = torch.arange(0, logits.shape[-1])[torch.sigmoid(logits).squeeze(dim=0) > 0.5]

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = ModernVBertForSequenceClassification.from_pretrained(
...     "ModernVBERT/modernvbert", num_labels=num_labels, problem_type="multi_label_classification"
... )

>>> labels = torch.sum(
...     torch.nn.functional.one_hot(predicted_class_ids[None, :].clone(), num_classes=num_labels), dim=1
... ).to(torch.float)
>>> loss = model(**inputs, labels=labels).loss
```

## ModernVBertForTokenClassification[[transformers.ModernVBertForTokenClassification]]

#### transformers.ModernVBertForTokenClassification[[transformers.ModernVBertForTokenClassification]]

```python
transformers.ModernVBertForTokenClassification(config: ModernVBertConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/modernvbert/modeling_modernvbert.py#L612)

**Parameters:**

config ([ModernVBertConfig](/docs/transformers/v5.15.1/en/model_doc/modernvbert#transformers.ModernVBertConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The ModernVBert Model with a token classification head on top, e.g. for Named Entity Recognition (NER) tasks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ModernVBertForTokenClassification.forward]]

```python
forward(input_ids: LongTensor = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_attention_mask: typing.Optional[torch.BoolTensor] = None, image_hidden_states: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/modernvbert/modeling_modernvbert.py#L625)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details ([Idefics3Processor](/docs/transformers/v5.15.1/en/model_doc/idefics3#transformers.Idefics3Processor) uses `image_processor_class` for processing images).

pixel_attention_mask (`torch.Tensor` of shape `(batch_size, image_size, image_size)`, *optional*) : Mask to avoid performing attention on padding pixel indices.

image_hidden_states (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The hidden states of the image encoder after modality projection.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., text_config.]` or `model.image_token_id`. Tokens with indices set to `model.image_token_id` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., text_config.]`.

**Returns:** [TokenClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or `tuple(torch.FloatTensor)`

A [TokenClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ModernVBertConfig](/docs/transformers/v5.15.1/en/model_doc/modernvbert#transformers.ModernVBertConfig)) and inputs.

Inputs fed to the model can have an arbitrary number of images. To account for this, pixel_values fed to
the model have image padding -> (batch_size, max_num_images, 3, max_heights, max_widths) where
max_num_images is the maximum number of images among the batch_size samples in the batch.
Padding images are not needed beyond padding the pixel_values at the entrance of the model.
For efficiency, we only pass through the vision_model's forward the real images by
discarding the padding images i.e. pixel_values of size (image_batch_size, 3, height, width) where
image_batch_size would be 7 when num_images_per_sample=[1, 3, 1, 2] and max_num_images would be 3.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`) -- Classification scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, ModernVBertForTokenClassification
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("ModernVBERT/modernvbert")
>>> model = ModernVBertForTokenClassification.from_pretrained("ModernVBERT/modernvbert")

>>> inputs = tokenizer(
...     "HuggingFace is a company based in Paris and New York", add_special_tokens=False, return_tensors="pt"
... )

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_token_class_ids = logits.argmax(-1)

>>> # Note that tokens are classified rather then input words which means that
>>> # there might be more predicted token classes than words.
>>> # Multiple token classes might account for the same word
>>> predicted_tokens_classes = [model.config.id2label[t.item()] for t in predicted_token_class_ids[0]]
>>> predicted_tokens_classes
...

>>> labels = predicted_token_class_ids
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```
