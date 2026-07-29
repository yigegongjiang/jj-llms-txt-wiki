# Command A Vision

## Overview

Command A Vision ([blog post](https://cohere.com/blog/command-a-vision)) is a state-of-the-art multimodal model designed to seamlessly integrate visual and textual information for a wide range of applications. By combining advanced computer vision techniques with natural language processing capabilities, Command A Vision enables users to analyze, understand, and generate insights from both visual and textual data.

The model excels at tasks including image captioning, visual question answering, document understanding, and chart understanding. This makes it a versatile tool for AI practitioners. Its ability to process complex visual and textual inputs makes it useful in settings where text-only representations are imprecise or unavailable, like real-world image understanding and graphics-heavy document processing.

Command A Vision is built upon a robust architecture that leverages the latest advancements in VLMs. It's highly performant and efficient, even when dealing with large-scale datasets. The model's flexibility makes it suitable for a wide range of use cases, from content moderation and image search to medical imaging analysis and robotics.

## Usage tips

The model and image processor can be loaded as follows:

```python

from transformers import AutoModelForImageTextToText, AutoProcessor

model_id = "CohereLabs/command-a-vision-07-2025"

processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForImageTextToText.from_pretrained(
    model_id, device_map="auto"
)

# Format message with the Command-A-Vision chat template
messages = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "url": "https://images.pexels.com/photos/1108099/pexels-photo-1108099.jpeg",
            },
            {"type": "text", "text": "what is in this image?"},
        ],
    },
]

inputs = processor.apply_chat_template(
    messages,
    padding=True,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
).to(model.device)

gen_tokens = model.generate(
    **inputs,
    max_new_tokens=300,
    do_sample=True,
    temperature=0.3,
)

print(
    processor.tokenizer.decode(
        gen_tokens[0][inputs.input_ids.shape[1] :], skip_special_tokens=True
    )
)
```

```python
from transformers import pipeline

pipe = pipeline(model="CohereLabs/command-a-vision-07-2025", task="image-text-to-text", device_map="auto")

messages = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "url": "https://media.istockphoto.com/id/458012057/photo/istanbul-turkey.jpg?s=612x612&w=0&k=20&c=qogAOVvkpfUyqLUMr_XJQyq-HkACXyYUSZbKhBlPrxo=",
            },
            {"type": "text", "text": "Where was this taken ?"},
        ],
    },
]

outputs = pipe(text=messages, max_new_tokens=300, return_full_text=False)

print(outputs)
```

## Cohere2VisionConfig[[transformers.Cohere2VisionConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **downsample_factor** (`int`, *optional*, defaults to 2) --
  The factor by which to downsample the input image.
- **image_token_id** (`int`, *optional*, defaults to `255036`) --
  The image token index used as a placeholder for input images.
- **alignment_intermediate_size** (`int`, *optional*, defaults to 36864) --
  The size of the intermediate layer for alignment.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Cohere2VisionModel. It is used to instantiate a Cohere2 Vision
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [CohereLabs/command-a-vision-07-2025](https://huggingface.co/CohereLabs/command-a-vision-07-2025)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Cohere2VisionForConditionalGeneration[[transformers.Cohere2VisionForConditionalGeneration]]

- **config** ([Cohere2VisionConfig](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The COHERE2_VISION model which consists of a vision backbone and a language model.

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
  [Cohere2VisionImageProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionImageProcessor). See `Cohere2VisionImageProcessor.__call__()` for details ([Cohere2VisionProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionProcessor) uses
  [Cohere2VisionImageProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionImageProcessor) for processing images).
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).
- **image_sizes** (`torch.Tensor` of shape `(batch_size, 2)`, *optional*) --
  The sizes of the images in the batch, being (height, width) for each image.`Cohere2VisionCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `Cohere2VisionCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Cohere2VisionConfig](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionConfig)) and inputs.
The [Cohere2VisionForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionForConditionalGeneration) forward method, overrides the `__call__` special method.

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
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from transformers import AutoProcessor, Cohere2VisionForConditionalGeneration
>>> import torch

>>> processor = AutoProcessor.from_pretrained("CohereLabs/command-a-vision-07-2025", use_fast=True)
>>> model = Cohere2VisionForConditionalGeneration.from_pretrained("CohereLabs/command-a-vision-07-2025", device_map="auto")

>>> messages = [
...     {
...         "role": "user",
...         "content": [
...             {
...                 "type": "image",
...                 "url": "https://images.pexels.com/photos/1108099/pexels-photo-1108099.jpeg",
...             },
...             {"type": "text", "text": "what is in this image?"},
...         ],
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages, padding=True, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt",
... ).to(model.device)

>>> gen_tokens = model.generate(**inputs, max_new_tokens=300, do_sample=True, temperature=0.3)
>>> processor.tokenizer.decode(gen_tokens[0][inputs.input_ids.shape[1]:], skip_special_tokens=True)
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Cohere2VisionImageProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionImageProcessor). See `Cohere2VisionImageProcessor.__call__()` for details ([Cohere2VisionProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionProcessor) uses
  [Cohere2VisionImageProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionImageProcessor) for processing images).[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Cohere2VisionConfig](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionConfig)) and inputs.

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

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Cohere2VisionForConditionalGeneration

>>> model = Cohere2VisionForConditionalGeneration.from_pretrained("CohereLabs/command-a-vision-07-2025")
>>> processor = AutoProcessor.from_pretrained("CohereLabs/command-a-vision-07-2025")

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

## Cohere2VisionModel[[transformers.Cohere2VisionModel]]

- **config** ([Cohere2VisionConfig](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Cohere2Vision model which consists of a vision backbone and a language model, without a language modeling head.

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
  [Cohere2VisionImageProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionImageProcessor). See `Cohere2VisionImageProcessor.__call__()` for details ([Cohere2VisionProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionProcessor) uses
  [Cohere2VisionImageProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionImageProcessor) for processing images).
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
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`Cohere2VisionModelOutputWithPast` or `tuple(torch.FloatTensor)`A `Cohere2VisionModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Cohere2VisionConfig](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionConfig)) and inputs.
The [Cohere2VisionModel](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionModel) forward method, overrides the `__call__` special method.

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

Example:

```python
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Cohere2VisionImageProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionImageProcessor). See `Cohere2VisionImageProcessor.__call__()` for details ([Cohere2VisionProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionProcessor) uses
  [Cohere2VisionImageProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionImageProcessor) for processing images).[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Cohere2VisionConfig](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionConfig)) and inputs.
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

Example:

```python
```

## Cohere2VisionImageProcessor[[transformers.Cohere2VisionImageProcessor]]

- **crop_to_patches** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to crop the image to patches. Can be overridden by the `crop_to_patches` parameter in the
  `preprocess` method.
- **min_patches** (`int`, *kwargs*, *optional*, defaults to 1) --
  The minimum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `min_patches` parameter in the `preprocess` method.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to 12) --
  The maximum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `max_patches` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Cohere2VisionImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Cohere2VisionProcessor[[transformers.Cohere2VisionProcessor]]

- **image_processor** (`Cohere2VisionImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
Constructs a Cohere2VisionProcessor which wraps a image processor and a tokenizer into a single processor.

[Cohere2VisionProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionProcessor) offers all the functionalities of [Cohere2VisionImageProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionImageProcessor) and `tokenizer_class`. See the
[~Cohere2VisionImageProcessor](/docs/transformers/v5.14.0/en/model_doc/cohere2_vision#transformers.Cohere2VisionImageProcessor) and `~tokenizer_class` for more information.

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
