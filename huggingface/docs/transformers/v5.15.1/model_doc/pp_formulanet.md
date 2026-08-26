# PP-FormulaNet

## Overview

**PP-FormulaNet-L** and **PP-FormulaNet_plus-L** are part of a series of dedicated lightweight models for table structure recognition, focusing on accurately recognizing table structures in documents and natural scenes. For more details about the SLANet series model, please refer to the [official documentation](https://www.paddleocr.ai/latest/en/version3.x/module_usage/table_structure_recognition.html).

## Usage

### Single input inference

The example below demonstrates how to detect text with PP-FormulaNet_plus-L using the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel).

```py
from io import BytesIO

import httpx
from PIL import Image
from transformers import AutoProcessor, AutoModelForImageTextToText

model_path = "PaddlePaddle/PP-FormulaNet_plus-L_safetensors" # or "PaddlePaddle/PP-FormulaNet-L_safetensors"
model = AutoModelForImageTextToText.from_pretrained(model_path, device_map="auto")
processor = AutoProcessor.from_pretrained(model_path)

image_url = "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_formula_rec_001.png"
image = Image.open(BytesIO(httpx.get(image_url).content)).convert("RGB")
inputs = processor(images=image, return_tensors="pt").to(model.device)
outputs = model(**inputs)
result = processor.post_process(outputs)
print(result)
```

## PPFormulaNetConfig[[transformers.PPFormulaNetConfig]]

#### transformers.PPFormulaNetConfig[[transformers.PPFormulaNetConfig]]

```python
transformers.PPFormulaNetConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, text_config: dict | transformers.models.pp_formulanet.configuration_pp_formulanet.PPFormulaNetTextConfig | None = None, vision_config: dict | transformers.models.pp_formulanet.configuration_pp_formulanet.PPFormulaNetVisionConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/configuration_pp_formulanet.py#L131)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

text_config (`Union[dict, ~models.pp_formulanet.configuration_pp_formulanet.PPFormulaNetTextConfig]`, *optional*) : The config object or dictionary of the text backbone.

vision_config (`Union[dict, ~models.pp_formulanet.configuration_pp_formulanet.PPFormulaNetVisionConfig]`, *optional*) : The config object or dictionary of the vision backbone.

This is the configuration class to store the configuration of a Pp FormulanetModel. It is used to instantiate a Pp Formulanet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PPFormulaNet_plus-L_safetensors](https://huggingface.co/PaddlePaddle/PPFormulaNet_plus-L_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPFormulaNetForConditionalGeneration[[transformers.PPFormulaNetForConditionalGeneration]]

#### transformers.PPFormulaNetForConditionalGeneration[[transformers.PPFormulaNetForConditionalGeneration]]

```python
transformers.PPFormulaNetForConditionalGeneration(config: PPFormulaNetConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/modeling_pp_formulanet.py#L1033)

**Parameters:**

config ([PPFormulaNetConfig](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Pp Formulanet Model for token generation conditioned on other modalities (e.g. image-text-to-text generation).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.PPFormulaNetForConditionalGeneration.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: list[torch.FloatTensor] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/modeling_pp_formulanet.py#L1045)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [PPFormulaNetImageProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor). See `PPFormulaNetImageProcessor.__call__()` for details ([PPFormulaNetProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetProcessor) uses [PPFormulaNetImageProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor) for processing images).

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to make sure the model can only look at previous inputs in order to predict the future.

encoder_outputs (`list[torch.FloatTensor]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPFormulaNetConfig](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetConfig)) and inputs.

The [PPFormulaNetForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

Example:

```python
>>> from io import BytesIO

>>> import httpx
>>> from PIL import Image
>>> from transformers import AutoProcessor, PPFormulaNetForConditionalGeneration

>>> model_path = "PaddlePaddle/PP-FormulaNet_plus-L_safetensors" # or "PaddlePaddle/PP-FormulaNet-L_safetensors"
>>> model = PPFormulaNetForConditionalGeneration.from_pretrained(model_path, device_map="auto")
>>> processor = AutoProcessor.from_pretrained(model_path)

>>> image_url = "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/general_formula_rec_001.png"
>>> image = Image.open(BytesIO(httpx.get(image_url).content)).convert("RGB")
>>> inputs = processor(images=image, return_tensors="pt").to(model.device)
>>> outputs = model(**inputs)
>>> result = processor.post_process(outputs)
>>> print(result)
['\\zeta_{0}(\\nu)=-\\frac{\\nu\\varrho^{-2\\nu}}{\\pi}\\int_{\\mu}^{\\infty}d\\omega\\int_{C_{+}}d z\\frac{2z^{2}}{(z^{2}+\\omega^{2})^{\\nu+1}}\\breve{\\Psi}(\\omega;z)e^{i\\epsilon z}\\quad,']
```

## PPFormulaNetTextModel[[transformers.PPFormulaNetTextModel]]

#### transformers.PPFormulaNetTextModel[[transformers.PPFormulaNetTextModel]]

```python
transformers.PPFormulaNetTextModel(config: PPFormulaNetConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/modeling_pp_formulanet.py#L771)

**Parameters:**

config ([PPFormulaNetConfig](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Pp Formulanet Text Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.PPFormulaNetTextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, encoder_hidden_states: typing.Optional[torch.FloatTensor] = None, encoder_attention_mask: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/modeling_pp_formulanet.py#L814)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you provide it.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

encoder_hidden_states (`torch.FloatTensor` of shape `(batch_size, encoder_sequence_length, hidden_size)`, *optional*) : Sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

encoder_attention_mask (`torch.LongTensor` of shape `(batch_size, encoder_sequence_length)`, *optional*) : Mask to avoid performing cross-attention on padding tokens indices of encoder input_ids. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

past_key_values (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) : It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.  If `past_key_values` are used, the user can optionally input only the last `decoder_input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, 1)` instead of all `decoder_input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

## PPFormulaNetVisionModel[[transformers.PPFormulaNetVisionModel]]

#### transformers.PPFormulaNetVisionModel[[transformers.PPFormulaNetVisionModel]]

```python
transformers.PPFormulaNetVisionModel(config: PPFormulaNetVisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/modeling_pp_formulanet.py#L431)

## PPFormulaNetModel[[transformers.PPFormulaNetModel]]

#### transformers.PPFormulaNetModel[[transformers.PPFormulaNetModel]]

```python
transformers.PPFormulaNetModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/modeling_pp_formulanet.py#L964)

#### forward[[transformers.PPFormulaNetModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, encoder_outputs: list[torch.FloatTensor] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/modeling_pp_formulanet.py#L973)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [PPFormulaNetImageProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor). See `PPFormulaNetImageProcessor.__call__()` for details ([PPFormulaNetProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetProcessor) uses [PPFormulaNetImageProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor) for processing images).

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to make sure the model can only look at previous inputs in order to predict the future.

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

encoder_outputs (`list[torch.FloatTensor]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [Seq2SeqModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPFormulaNetConfig](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetConfig)) and inputs.

The [PPFormulaNetModel](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the optional initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.

## PPFormulaNetTextConfig[[transformers.PPFormulaNetTextConfig]]

#### transformers.PPFormulaNetTextConfig[[transformers.PPFormulaNetTextConfig]]

```python
transformers.PPFormulaNetTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, vocab_size: int = 50000, max_position_embeddings: int = 2560, encoder_layers: int = 12, encoder_attention_heads: int = 16, decoder_layers: int = 8, decoder_ffn_dim: int = 2048, decoder_attention_heads: int = 16, decoder_layerdrop: float | int = 0.0, use_cache: bool = True, activation_function: str = 'gelu', d_model: int = 512, dropout: float | int = 0.1, attention_dropout: float | int = 0.0, activation_dropout: float | int = 0.0, init_std: float = 0.02, scale_embedding: bool = True, pad_token_id: int | None = 1, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 2, decoder_start_token_id: int | None = 2, forced_eos_token_id: int | list[int] | None = 2, tie_word_embeddings: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/configuration_pp_formulanet.py#L79)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

vocab_size (`int`, *optional*, defaults to `50000`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

max_position_embeddings (`int`, *optional*, defaults to `2560`) : The maximum sequence length that this model might ever be used with.

encoder_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

encoder_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer encoder.

decoder_layers (`int`, *optional*, defaults to `8`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

decoder_ffn_dim (`int`, *optional*, defaults to `2048`) : Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.

decoder_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

decoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.0`) : The LayerDrop probability for the decoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

activation_function (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

d_model (`int`, *optional*, defaults to `512`) : Size of the encoder layers and the pooler layer.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for activations inside the fully connected layer.

init_std (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

scale_embedding (`bool`, *optional*, defaults to `True`) : Whether to scale embeddings by dividing by sqrt(d_model).

pad_token_id (`int`, *optional*, defaults to `1`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

decoder_start_token_id (`int`, *optional*, defaults to `2`) : If an encoder-decoder model starts decoding with a different token than `bos`, the id of that token.

forced_eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : The id of the token to force as the last generated token when `max_length` is reached. Usually set to `eos_token_id`.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Pp FormulanetModel. It is used to instantiate a Pp Formulanet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PPFormulaNet_plus-L_safetensors](https://huggingface.co/PaddlePaddle/PPFormulaNet_plus-L_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PPFormulaNetTextConfig, PPFormulaNetTextModel

>>> # Initializing a PP_FORMULANET facebook/pp_formulanet-large-cc25 style configuration
>>> configuration = PPFormulaNetTextConfig()

>>> # Initializing a model (with random weights) from the facebook/pp_formulanet-large-cc25 style configuration
>>> model = PPFormulaNetTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PPFormulaNetVisionConfig[[transformers.PPFormulaNetVisionConfig]]

#### transformers.PPFormulaNetVisionConfig[[transformers.PPFormulaNetVisionConfig]]

```python
transformers.PPFormulaNetVisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, output_channels: int = 256, num_hidden_layers: int = 12, num_attention_heads: int = 12, num_channels: int = 3, image_size: int = 512, patch_size: int | list[int] | tuple[int, int] = 16, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06, attention_dropout: float | int = 0.0, initializer_range: float = 1e-10, qkv_bias: bool = True, use_abs_pos: bool = True, use_rel_pos: bool = True, window_size: int = 14, global_attn_indexes: list[int] | tuple[int, ...] = (2, 5, 8, 11), mlp_dim: int = 3072, post_conv_in_channels: int = 256, post_conv_out_channels: int = 1024, post_conv_mid_channels: int = 512, decoder_hidden_size: int = 512)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/configuration_pp_formulanet.py#L32)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

output_channels (`int`, *optional*, defaults to 256) : Dimensionality of the output channels in the Patch Encoder.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

image_size (`int`, *optional*, defaults to `512`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `1e-10`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

use_abs_pos (`bool`, *optional*, defaults to `True`) : Whether to use absolute position embeddings.

use_rel_pos (`bool`, *optional*, defaults to `True`) : Whether to use relative position bias in the self-attention layers.

window_size (`int`, *optional*, defaults to 14) : Window size for relative position.

global_attn_indexes (`list[int]`, *optional*, defaults to `[2, 5, 8, 11]`) : The indexes of the global attention layers.

mlp_dim (`int`, *optional*, defaults to 3072) : The dimensionality of the MLP layer in the Transformer encoder.

post_conv_in_channels (`int`, *optional*, defaults to 256) : Number of input channels for the post-encoder convolution layer.

post_conv_out_channels (`int`, *optional*, defaults to 1024) : Number of output channels for the post-encoder convolution layer.

post_conv_mid_channels (`int`, *optional*, defaults to 512) : Number of intermediate channels for the post-encoder convolution layer.

decoder_hidden_size (`int`, *optional*, defaults to 512) : The hidden size of the decoder that the encoder features are projected to.

This is the configuration class to store the configuration of a Pp FormulanetModel. It is used to instantiate a Pp Formulanet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PPFormulaNet_plus-L_safetensors](https://huggingface.co/PaddlePaddle/PPFormulaNet_plus-L_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPFormulaNetImageProcessor[[transformers.PPFormulaNetImageProcessor]]

#### transformers.PPFormulaNetImageProcessor[[transformers.PPFormulaNetImageProcessor]]

```python
transformers.PPFormulaNetImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/image_processing_pp_formulanet.py#L51)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 768, 'width': 768}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.7931, 0.7931, 0.7931]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.1738, 0.1738, 0.1738]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

do_crop_margin (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) : Whether to crop the image margins.

do_thumbnail (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) : Whether to resize the image using thumbnail method.

do_align_long_axis (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) : Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.

Constructs a PPFormulaNetImageProcessor image processor.

#### align_long_axis[[transformers.PPFormulaNetImageProcessor.align_long_axis]]

```python
align_long_axis(image: torch.Tensor, size: SizeDict)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/image_processing_pp_formulanet.py#L124)

**Parameters:**

image (`torch.Tensor`) : The image to be aligned.

size (`SizeDict`) : The size to align the long axis to.

**Returns:** `torch.Tensor`

The aligned image.

Align the long axis of the image to the longest axis of the specified size.

#### crop_margin[[transformers.PPFormulaNetImageProcessor.crop_margin]]

```python
crop_margin(image: torch.Tensor, gray_threshold: int = 200)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/image_processing_pp_formulanet.py#L94)

**Parameters:**

image (`torch.Tensor`) : The image to be cropped.

gray_threshold (`int`, *optional*, defaults to `200`) : Value below which pixels are considered to be gray.

Crops the margin of the image. Gray pixels are considered margin (i.e., pixels with a value below the
threshold).

#### pad_images[[transformers.PPFormulaNetImageProcessor.pad_images]]

```python
pad_images(image: torch.Tensor, size: SizeDict)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/image_processing_pp_formulanet.py#L185)

**Parameters:**

image (`torch.tensor`) : The image to be padded.

size (`SizeDict`) : The size to pad the image to.

Pads a batch of images to the specified size at the top, bottom, left and right.

#### preprocess[[transformers.PPFormulaNetImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/image_processing_pp_formulanet.py#L68)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

do_crop_margin (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) : Whether to crop the image margins.

do_thumbnail (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) : Whether to resize the image using thumbnail method.

do_align_long_axis (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) : Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### python_bounding_rect[[transformers.PPFormulaNetImageProcessor.python_bounding_rect]]

```python
python_bounding_rect(coordinates)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/image_processing_pp_formulanet.py#L83)

This is a reimplementation of a BoundingRect function equivalent to cv2.

#### python_find_non_zero[[transformers.PPFormulaNetImageProcessor.python_find_non_zero]]

```python
python_find_non_zero(image: torch.Tensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/image_processing_pp_formulanet.py#L72)

This is a reimplementation of a findNonZero function equivalent to cv2.

#### resize[[transformers.PPFormulaNetImageProcessor.resize]]

```python
resize(image: torch.Tensor, size: SizeDict, resample: PILImageResampling | tvF.InterpolationMode | int | None = None, antialias: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/image_processing_pp_formulanet.py#L214)

**Parameters:**

image (`torch.Tensor`) : Image to resize.

size (`SizeDict`) : Size of the output image.

resample (`PILImageResampling | tvF.InterpolationMode | int`, *optional*) : Resampling filter to use when resizing the image.

**Returns:** `torch.Tensor`

The resized image.

Resize an image to `(size.height, size.width)`.

#### thumbnail[[transformers.PPFormulaNetImageProcessor.thumbnail]]

```python
thumbnail(image: torch.Tensor, size: SizeDict)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/image_processing_pp_formulanet.py#L150)

**Parameters:**

image (`torch.tensor`) : The image to be resized.

size (`SizeDict`) : The size to resize the image to.

Resize the image to make a thumbnail. The image is resized so that no dimension is larger than any
corresponding dimension of the specified size.

## PPFormulaNetProcessor[[transformers.PPFormulaNetProcessor]]

#### transformers.PPFormulaNetProcessor[[transformers.PPFormulaNetProcessor]]

```python
transformers.PPFormulaNetProcessor(image_processor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/processing_pp_formulanet.py#L38)

**Parameters:**

image_processor (`PPFormulaNetImageProcessor`) : The image processor is a required input.

tokenizer (`NougatTokenizer`) : The tokenizer is a required input.

Constructs a PPFormulaNetProcessor which wraps a image processor and a tokenizer into a single processor.

[PPFormulaNetProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetProcessor) offers all the functionalities of [PPFormulaNetImageProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor) and [NougatTokenizer](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatTokenizer). See the
[~PPFormulaNetImageProcessor](/docs/transformers/v5.15.1/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor) and [~NougatTokenizer](/docs/transformers/v5.15.1/en/model_doc/nougat#transformers.NougatTokenizer) for more information.

#### normalize[[transformers.PPFormulaNetProcessor.normalize]]

```python
normalize(text: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/processing_pp_formulanet.py#L110)

Normalizes a string by removing unnecessary spaces.

#### post_process[[transformers.PPFormulaNetProcessor.post_process]]

```python
post_process(generated_outputs, skip_special_tokens = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/processing_pp_formulanet.py#L142)

**Parameters:**

generated_outputs (`torch.Tensor` or `np.ndarray`) : The output of the model `generate` function. The output is expected to be a tensor of shape `(batch_size, sequence_length)` or `(sequence_length,)`.

skip_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to remove special tokens in the output. Argument passed to the tokenizer's `batch_decode` method.

- ****kwargs** : Additional arguments to be passed to the tokenizer's `batch_decode method`.

**Returns:** `list[str]`

The decoded text.

Post-process the output of the model to decode the text.

#### post_process_generation[[transformers.PPFormulaNetProcessor.post_process_generation]]

```python
post_process_generation(text: str)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_formulanet/processing_pp_formulanet.py#L88)

**Parameters:**

text (str) : String to post-process.

**Returns:** `str`

Post-processed string.

Post-processes a string by fixing text and normalizing it.
