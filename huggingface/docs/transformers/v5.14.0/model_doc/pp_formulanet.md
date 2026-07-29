# PP-FormulaNet

## Overview

**PP-FormulaNet-L** and **PP-FormulaNet_plus-L** are part of a series of dedicated lightweight models for table structure recognition, focusing on accurately recognizing table structures in documents and natural scenes. For more details about the SLANet series model, please refer to the [official documentation](https://www.paddleocr.ai/latest/en/version3.x/module_usage/table_structure_recognition.html).

## Usage

### Single input inference

The example below demonstrates how to detect text with PP-PP-FormulaNet_plus-L using the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

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

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **text_config** (`Union[dict, ~models.pp_formulanet.configuration_pp_formulanet.PPFormulaNetTextConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~models.pp_formulanet.configuration_pp_formulanet.PPFormulaNetVisionConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.

This is the configuration class to store the configuration of a Pp FormulanetModel. It is used to instantiate a Pp Formulanet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PPFormulaNet_plus-L_safetensors](https://huggingface.co/PaddlePaddle/PPFormulaNet_plus-L_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPFormulaNetForConditionalGeneration[[transformers.PPFormulaNetForConditionalGeneration]]

- **config** ([PPFormulaNetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Pp Formulanet Model for token generation conditioned on other modalities (e.g. image-text-to-text generation).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PPFormulaNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor). See `PPFormulaNetImageProcessor.__call__()` for details ([PPFormulaNetProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetProcessor) uses
  [PPFormulaNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor) for processing images).
- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Indices of decoder input sequence tokens in the vocabulary.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are decoder input IDs?](../glossary#decoder-input-ids)
- **decoder_attention_mask** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to
  make sure the model can only look at previous inputs in order to predict the future.
- **encoder_outputs** (`list[torch.FloatTensor]`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded
  representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be
  input (see `past_key_values`). This is useful if you want more control over how to convert
  `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value
  of `inputs_embeds`.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).[Seq2SeqLMOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`A [Seq2SeqLMOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPFormulaNetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetConfig)) and inputs.
The [PPFormulaNetForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

- **config** ([PPFormulaNetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Pp Formulanet Text Model outputting raw hidden-states without any specific head on to.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you
  provide it.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch_size, encoder_sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention
  of the decoder.
- **encoder_attention_mask** (`torch.LongTensor` of shape `(batch_size, encoder_sequence_length)`, *optional*) --
  Mask to avoid performing cross-attention on padding tokens indices of encoder input_ids. Mask values
  selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) --
  It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the
  cross-attention blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.

  If `past_key_values` are used, the user can optionally input only the last `decoder_input_ids` (those
  that don't have their past key value states given to this model) of shape `(batch_size, 1)` instead of
  all `decoder_input_ids` of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation.
  This is useful if you want more control over how to convert `input_ids` indices into associated vectors
  than the model's internal embedding lookup matrix.

## PPFormulaNetVisionModel[[transformers.PPFormulaNetVisionModel]]

## PPFormulaNetModel[[transformers.PPFormulaNetModel]]

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PPFormulaNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor). See `PPFormulaNetImageProcessor.__call__()` for details ([PPFormulaNetProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetProcessor) uses
  [PPFormulaNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor) for processing images).
- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Indices of decoder input sequence tokens in the vocabulary.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are decoder input IDs?](../glossary#decoder-input-ids)
- **decoder_attention_mask** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to
  make sure the model can only look at previous inputs in order to predict the future.
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded
  representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be
  input (see `past_key_values`). This is useful if you want more control over how to convert
  `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value
  of `inputs_embeds`.
- **encoder_outputs** (`list[torch.FloatTensor]`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[Seq2SeqModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or `tuple(torch.FloatTensor)`A [Seq2SeqModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PPFormulaNetConfig](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetConfig)) and inputs.
The [PPFormulaNetModel](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **vocab_size** (`int`, *optional*, defaults to `50000`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **max_position_embeddings** (`int`, *optional*, defaults to `2560`) --
  The maximum sequence length that this model might ever be used with.
- **encoder_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.
- **encoder_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer encoder.
- **decoder_layers** (`int`, *optional*, defaults to `8`) --
  Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.
- **decoder_ffn_dim** (`int`, *optional*, defaults to `2048`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.
- **decoder_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **decoder_layerdrop** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The LayerDrop probability for the decoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556)
  for more details.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **activation_function** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **d_model** (`int`, *optional*, defaults to `512`) --
  Size of the encoder layers and the pooler layer.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for activations inside the fully connected layer.
- **init_std** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **scale_embedding** (`bool`, *optional*, defaults to `True`) --
  Whether to scale embeddings by dividing by sqrt(d_model).
- **pad_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **decoder_start_token_id** (`int`, *optional*, defaults to `2`) --
  If an encoder-decoder model starts decoding with a different token than `bos`, the id of that token.
- **forced_eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  The id of the token to force as the last generated token when `max_length` is reached. Usually set to
  `eos_token_id`.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Pp FormulanetModel. It is used to instantiate a Pp Formulanet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PPFormulaNet_plus-L_safetensors](https://huggingface.co/PaddlePaddle/PPFormulaNet_plus-L_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

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

- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **output_channels** (`int`, *optional*, defaults to 256) --
  Dimensionality of the output channels in the Patch Encoder.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`int`, *optional*, defaults to `512`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `1e-10`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **use_abs_pos** (`bool`, *optional*, defaults to `True`) --
  Whether to use absolute position embeddings.
- **use_rel_pos** (`bool`, *optional*, defaults to `True`) --
  Whether to use relative position bias in the self-attention layers.
- **window_size** (`int`, *optional*, defaults to 14) --
  Window size for relative position.
- **global_attn_indexes** (`list[int]`, *optional*, defaults to `[2, 5, 8, 11]`) --
  The indexes of the global attention layers.
- **mlp_dim** (`int`, *optional*, defaults to 3072) --
  The dimensionality of the MLP layer in the Transformer encoder.
- **post_conv_in_channels** (`int`, *optional*, defaults to 256) --
  Number of input channels for the post-encoder convolution layer.
- **post_conv_out_channels** (`int`, *optional*, defaults to 1024) --
  Number of output channels for the post-encoder convolution layer.
- **post_conv_mid_channels** (`int`, *optional*, defaults to 512) --
  Number of intermediate channels for the post-encoder convolution layer.
- **decoder_hidden_size** (`int`, *optional*, defaults to 512) --
  The hidden size of the decoder that the encoder features are projected to.

This is the configuration class to store the configuration of a Pp FormulanetModel. It is used to instantiate a Pp Formulanet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PPFormulaNet_plus-L_safetensors](https://huggingface.co/PaddlePaddle/PPFormulaNet_plus-L_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PPFormulaNetImageProcessor[[transformers.PPFormulaNetImageProcessor]]

- **do_crop_margin** (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) --
  Whether to crop the image margins.
- **do_thumbnail** (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) --
  Whether to resize the image using thumbnail method.
- **do_align_long_axis** (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) --
  Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PPFormulaNetImageProcessor image processor.

- **image** (`torch.Tensor`) --
  The image to be aligned.
- **size** (`SizeDict`) --
  The size to align the long axis to.`torch.Tensor`The aligned image.

Align the long axis of the image to the longest axis of the specified size.

- **image** (`torch.Tensor`) --
  The image to be cropped.
- **gray_threshold** (`int`, *optional*, defaults to `200`) --
  Value below which pixels are considered to be gray.

Crops the margin of the image. Gray pixels are considered margin (i.e., pixels with a value below the
threshold).

- **image** (`torch.tensor`) --
  The image to be padded.
- **size** (`SizeDict`) --
  The size to pad the image to.

Pads a batch of images to the specified size at the top, bottom, left and right.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **do_crop_margin** (`bool`, *kwargs*, *optional*, defaults to `self.do_crop_margin`) --
  Whether to crop the image margins.
- **do_thumbnail** (`bool`, *kwargs*, *optional*, defaults to `self.do_thumbnail`) --
  Whether to resize the image using thumbnail method.
- **do_align_long_axis** (`bool`, *kwargs*, *optional*, defaults to `self.do_align_long_axis`) --
  Whether to align the long axis of the image with the long axis of `size` by rotating by 90 degrees.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

This is a reimplementation of a BoundingRect function equivalent to cv2.

This is a reimplementation of a findNonZero function equivalent to cv2.

- **image** (`torch.Tensor`) --
  Image to resize.
- **size** (`SizeDict`) --
  Size of the output image.
- **resample** (`PILImageResampling | tvF.InterpolationMode | int`, *optional*) --
  Resampling filter to use when resizing the image.`torch.Tensor`The resized image.

Resize an image to `(size.height, size.width)`.

- **image** (`torch.tensor`) --
  The image to be resized.
- **size** (`SizeDict`) --
  The size to resize the image to.

Resize the image to make a thumbnail. The image is resized so that no dimension is larger than any
corresponding dimension of the specified size.

## PPFormulaNetProcessor[[transformers.PPFormulaNetProcessor]]

- **image_processor** (`PPFormulaNetImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`NougatTokenizer`) --
  The tokenizer is a required input.
Constructs a PPFormulaNetProcessor which wraps a image processor and a tokenizer into a single processor.

[PPFormulaNetProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetProcessor) offers all the functionalities of [PPFormulaNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor) and [NougatTokenizer](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatTokenizer). See the
[~PPFormulaNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pp_formulanet#transformers.PPFormulaNetImageProcessor) and [~NougatTokenizer](/docs/transformers/v5.14.0/en/model_doc/nougat#transformers.NougatTokenizer) for more information.

Normalizes a string by removing unnecessary spaces.

- **generated_outputs** (`torch.Tensor` or `np.ndarray`) --
  The output of the model `generate` function. The output is expected to be a tensor of shape `(batch_size, sequence_length)`
  or `(sequence_length,)`.
- **skip_special_tokens** (`bool`, *optional*, defaults to `True`) --
  Whether or not to remove special tokens in the output. Argument passed to the tokenizer's `batch_decode` method.
- ****kwargs** --
  Additional arguments to be passed to the tokenizer's `batch_decode method`.`list[str]`The decoded text.

Post-process the output of the model to decode the text.

- **text** (str) -- String to post-process.strPost-processed string.
Post-processes a string by fixing text and normalizing it.
