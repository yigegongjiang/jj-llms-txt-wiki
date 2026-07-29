# Ovis2

## Overview

The [Ovis2](https://github.com/AIDC-AI/Ovis) is an updated version of the [Ovis](https://huggingface.co/papers/2405.20797) model developed by the AIDC-AI team at Alibaba International Digital Commerce Group.

Ovis2 is the latest advancement in multi-modal large language models (MLLMs), succeeding Ovis1.6. It retains the architectural design of the Ovis series, which focuses on aligning visual and textual embeddings, and introduces major improvements in data curation and training methods.

 Ovis2 architecture.

This model was contributed by [thisisiron](https://huggingface.co/thisisiron).

## Usage example

```python

import requests
import torch
from PIL import Image

from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained(
    "thisisiron/Ovis2-2B-hf",
).eval().to(model.device, device_map="auto")
processor = AutoProcessor.from_pretrained("thisisiron/Ovis2-2B-hf")

messages = [
    {
        "role": "user",
        "content": [
            {"type": "image"},
            {"type": "text", "text": "Describe the image."},
        ],
    },
]
url = "http://images.cocodataset.org/val2014/COCO_val2014_000000537955.jpg"
image = Image.open(requests.get(url, stream=True).raw)
messages = processor.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
print(messages)

inputs = processor(
    images=[image],
    text=messages,
    return_tensors="pt",
)
inputs = inputs.to(model.device)
inputs['pixel_values'] = inputs['pixel_values'].to(torch.bfloat16)

with torch.inference_mode():
    output_ids = model.generate(**inputs, max_new_tokens=128, do_sample=False)
    generated_ids = [output_ids[len(input_ids):] for input_ids, output_ids in zip(inputs.input_ids, output_ids)]
    output_text = processor.batch_decode(generated_ids, skip_special_tokens=True)
    print(output_text)
```

## Ovis2Config[[transformers.Ovis2Config]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **image_token_id** (`int`, *optional*, defaults to `151665`) --
  The image token index used as a placeholder for input images.
- **visual_indicator_token_ids** (`List[int]`, *optional*, defaults to `[151666, 151667, 151668, 151669, 151670]`) --
  The visual indicator token ids to encode the image prompt.
- **vocab_size** (`int`, *optional*, defaults to `151643`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `1536`) --
  Dimension of the hidden representations.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Ovis2Model. It is used to instantiate a Ovis2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [thisisiron/Ovis2-1B-hf](https://huggingface.co/thisisiron/Ovis2-1B-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import Ovis2ForConditionalGeneration, Ovis2Config

>>> # Initializing a Ovis2 style configuration
>>> configuration = Ovis2Config()

>>> # Initializing a model from the Ovis2-2B style configuration
>>> model = Ovis2ForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Ovis2VisionConfig[[transformers.Ovis2VisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `2816`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) --
  The size (resolution) of each patch.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **qkv_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to add a bias to the queries, keys and values.
- **mlp_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **vocab_size** (`int`, *optional*, defaults to `16384`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_stride** (`int`, *optional*, defaults to 1) --
  The stride of the hidden layer in the Vision Transformer.
- **num_visual_indicator_tokens** (`int`, *optional*, defaults to 5) --
  Number of visual indicator tokens.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **tokenize_function** (`str`, *optional*, defaults to `"softmax"`) --
  The function used to tokenize the visual indicator tokens.

This is the configuration class to store the configuration of a Ovis2Model. It is used to instantiate a Ovis2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [thisisiron/Ovis2-1B-hf](https://huggingface.co/thisisiron/Ovis2-1B-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Ovis2Model[[transformers.Ovis2Model]]

- **config** ([Ovis2Config](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Ovis2 model which consists of a vision backbone and a language model, without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Ovis2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ImageProcessor). See `Ovis2ImageProcessor.__call__()` for details ([Ovis2Processor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Processor) uses
  [Ovis2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).`Ovis2ModelOutputWithPast` or `tuple(torch.FloatTensor)`A `Ovis2ModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Ovis2Config](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Config)) and inputs.
The [Ovis2Model](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Ovis2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ImageProcessor). See `Ovis2ImageProcessor.__call__()` for details ([Ovis2Processor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Processor) uses
  [Ovis2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ImageProcessor) for processing images).`BaseModelOutputWithVisualIndicatorFeatures` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithVisualIndicatorFeatures` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Ovis2Config](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Config)) and inputs.
Obtains image last hidden states from the vision tower and apply multimodal projection.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **visual_indicator_features** (`torch.FloatTensor` of shape `(batch_size, visual_indicator_size)`) -- Visual indicator features extracted from the model, which can be used for auxiliary tasks or further processing.

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

## Ovis2ForConditionalGeneration[[transformers.Ovis2ForConditionalGeneration]]

- **config** ([Ovis2Config](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Ovis2 Model for token generation conditioned on other modalities (e.g. image-text-to-text generation).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Ovis2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ImageProcessor). See `Ovis2ImageProcessor.__call__()` for details ([Ovis2Processor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Processor) uses
  [Ovis2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).`Ovis2CausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `Ovis2CausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Ovis2Config](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Config)) and inputs.
The [Ovis2ForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size (batch_size * num_patches, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, Ovis2ForConditionalGeneration

>>> model = Ovis2ForConditionalGeneration.from_pretrained("thisisiron/Ovis2-2B-hf")
>>> processor = AutoProcessor.from_pretrained("thisisiron/Ovis2-2B-hf")

>>> prompt = "<|im_start|>user\n<image>\nDescribe the image.<|im_end|>\n<|im_start|>assistant\n"
>>> url = "http://images.cocodataset.org/val2014/COCO_val2014_000000537955.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, text=prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(**inputs, max_new_tokens=15)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
"user\n\nDescribe the image.\nassistant\nThe image features a brown dog standing on a wooden floor, looking up with"
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Ovis2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ImageProcessor). See `Ovis2ImageProcessor.__call__()` for details ([Ovis2Processor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Processor) uses
  [Ovis2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ImageProcessor) for processing images).`BaseModelOutputWithVisualIndicatorFeatures` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithVisualIndicatorFeatures` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Ovis2Config](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Config)) and inputs.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **visual_indicator_features** (`torch.FloatTensor` of shape `(batch_size, visual_indicator_size)`) -- Visual indicator features extracted from the model, which can be used for auxiliary tasks or further processing.

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Ovis2ForConditionalGeneration

>>> model = Ovis2ForConditionalGeneration.from_pretrained("thisisiron/Ovis2-1B-hf")
>>> processor = AutoProcessor.from_pretrained("thisisiron/Ovis2-1B-hf")

>>> messages = [
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
```

## Ovis2ImageProcessor[[transformers.Ovis2ImageProcessor]]

- **crop_to_patches** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to crop the image to patches. Can be overridden by the `crop_to_patches` parameter in the
  `preprocess` method.
- **min_patches** (`int`, *kwargs*, *optional*, defaults to 1) --
  The minimum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `min_patches` parameter in the `preprocess` method.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to 12) --
  The maximum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `max_patches` parameter in the `preprocess` method.
- **use_covering_area_grid** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to use the covering area grid to determine the number of patches. Only has an effect if
  `crop_to_patches` is set to `True`. Can be overridden by the `use_covering_area_grid` parameter in the
  `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Ovis2ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **crop_to_patches** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to crop the image to patches. Can be overridden by the `crop_to_patches` parameter in the
  `preprocess` method.
- **min_patches** (`int`, *kwargs*, *optional*, defaults to 1) --
  The minimum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `min_patches` parameter in the `preprocess` method.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to 12) --
  The maximum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `max_patches` parameter in the `preprocess` method.
- **use_covering_area_grid** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to use the covering area grid to determine the number of patches. Only has an effect if
  `crop_to_patches` is set to `True`. Can be overridden by the `use_covering_area_grid` parameter in the
  `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Ovis2ImageProcessorPil[[transformers.Ovis2ImageProcessorPil]]

- **crop_to_patches** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to crop the image to patches. Can be overridden by the `crop_to_patches` parameter in the
  `preprocess` method.
- **min_patches** (`int`, *kwargs*, *optional*, defaults to 1) --
  The minimum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `min_patches` parameter in the `preprocess` method.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to 12) --
  The maximum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `max_patches` parameter in the `preprocess` method.
- **use_covering_area_grid** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to use the covering area grid to determine the number of patches. Only has an effect if
  `crop_to_patches` is set to `True`. Can be overridden by the `use_covering_area_grid` parameter in the
  `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Ovis2ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **crop_to_patches** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to crop the image to patches. Can be overridden by the `crop_to_patches` parameter in the
  `preprocess` method.
- **min_patches** (`int`, *kwargs*, *optional*, defaults to 1) --
  The minimum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `min_patches` parameter in the `preprocess` method.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to 12) --
  The maximum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `max_patches` parameter in the `preprocess` method.
- **use_covering_area_grid** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to use the covering area grid to determine the number of patches. Only has an effect if
  `crop_to_patches` is set to `True`. Can be overridden by the `use_covering_area_grid` parameter in the
  `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Ovis2Processor[[transformers.Ovis2Processor]]

'"}, {"name": "image_seq_length", "val": " = 256"}, {"name": "**kwargs", "val": ""}]}>
- **image_processor** (`Ovis2ImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`Qwen2Tokenizer`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **image_token** (`str`, *optional*, defaults to `"<image>"`) --
  Special token used to denote image location.
- **image_seq_length** (`int`, *optional*, defaults to 256) --
  The number of image tokens to be used for each image in the input.
Constructs a Ovis2Processor which wraps a image processor and a tokenizer into a single processor.

[Ovis2Processor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2Processor) offers all the functionalities of [Ovis2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ImageProcessor) and [Qwen2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer). See the
[~Ovis2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/ovis2#transformers.Ovis2ImageProcessor) and [~Qwen2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:

- **input_ids** -- List of token ids to be fed to a model. Returned when `text` is not `None`.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).
- **pixel_values** -- Pixel values to be fed to a model. Returned when `images` is not `None`.
- **image_sizes** -- Size of each image that will be used to unpad an image. Returned when `images` is not `None`.
