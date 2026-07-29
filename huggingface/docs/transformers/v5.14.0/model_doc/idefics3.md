# Idefics3

## Overview

The Idefics3 model was proposed in [Building and better understanding vision-language models: insights and future directions](https://huggingface.co/papers/2408.12637) by Hugo Laurençon, Andrés Marafioti, Victor Sanh, and Léo Tronchon.

Idefics3 is an adaptation of the Idefics2 model with three main differences:

- It uses Llama3 for the text model.
- It uses an updated processing logic for the images.
- It removes the perceiver.

The abstract from the paper is the following:

*The field of vision-language models (VLMs), which take images and texts as inputs and output texts, is rapidly evolving and has yet to reach consensus on several key aspects of the development pipeline, including data, architecture, and training methods. This paper can be seen as a tutorial for building a VLM. We begin by providing a comprehensive overview of the current state-of-the-art approaches, highlighting the strengths and weaknesses of each, addressing the major challenges in the field, and suggesting promising research directions for underexplored areas. We then walk through the practical steps to build Idefics3-8B, a powerful VLM that significantly outperforms its predecessor Idefics2-8B, while being trained efficiently, exclusively on open datasets, and using a straightforward pipeline. These steps include the creation of Docmatix, a dataset for improving document understanding capabilities, which is 240 times larger than previously available datasets. We release the model along with the datasets created for its training.*

## Usage tips

Input images are processed either by upsampling (if resizing is enabled) or at their original resolution. The resizing behavior depends on two parameters: do_resize and size.

If `do_resize` is set to `True`, the model resizes images so that the longest edge is 4*364 pixels by default.
The default resizing behavior can be customized by passing a dictionary to the `size` parameter. For example, `{"longest_edge": 4 * 364}` is the default, but you can change it to a different value if needed.

Here’s how to control resizing and set a custom size:

```python
image_processor = Idefics3ImageProcessor(do_resize=True, size={"longest_edge": 2 * 364}, max_image_size=364)
```

Additionally, the `max_image_size` parameter, which controls the size of each square patch the image is decomposed into, is set to 364 by default but can be adjusted as needed. After resizing (if applicable), the image processor decomposes the images into square patches based on the `max_image_size` parameter.

This model was contributed by [amyeroberts](https://huggingface.co/amyeroberts) and [andimarafioti](https://huggingface.co/andito).

## Idefics3Config[[transformers.Idefics3Config]]

- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **image_token_id** (`int`, *optional*, defaults to `128257`) --
  The image token index used as a placeholder for input images.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **scale_factor** (`int`, *optional*, defaults to 2) --
  The scale factor for the image encoder.
- **pad_token_id** (`int`, *optional*, defaults to `128002`) --
  Token id used for padding in the vocabulary.

This is the configuration class to store the configuration of a Idefics3Model. It is used to instantiate a Idefics3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [HuggingFaceM4/Idefics3-8B-Llama3](https://huggingface.co/HuggingFaceM4/Idefics3-8B-Llama3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import Idefics3Model, Idefics3Config
>>> # Initializing configuration
>>> configuration = Idefics3Config()
>>> # Initializing a model from the configuration
>>> model = Idefics3Model(configuration)
>>> # Accessing the model configuration
>>> configuration = model.config
```

## Idefics3VisionConfig[[transformers.Idefics3VisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1152`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `32`) --
  The size (resolution) of each patch.
- **hidden_act** (`str`, *optional*, defaults to `gelu_pytorch_tanh`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Idefics3Model. It is used to instantiate a Idefics3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [HuggingFaceM4/Idefics3-8B-Llama3](https://huggingface.co/HuggingFaceM4/Idefics3-8B-Llama3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers.models.idefics3.modeling_idefics3 import Idefics3VisionTransformer
>>> from transformers.models.idefics3.configuration_idefics3 import Idefics3VisionConfig

>>> # Initializing a Idefics3VisionConfig with google/siglip-base-patch16-224 style configuration
>>> configuration = Idefics3VisionConfig()

>>> # Initializing a Idefics3VisionTransformer (with random weights) from the google/siglip-base-patch16-224 style configuration
>>> model = Idefics3VisionTransformer(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Idefics3VisionTransformer[[transformers.Idefics3VisionTransformer]]

- **config** ([Idefics3VisionConfig](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3VisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Idefics3 Vision Transformer Model outputting raw image embedding.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

## Idefics3Model[[transformers.Idefics3Model]]

- **config** ([Idefics3Config](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Idefics3 model consisting of a SIGLIP vision encoder and Llama3 language decoder

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
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Idefics3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3ImageProcessor). See `Idefics3ImageProcessor.__call__()` for details ([Idefics3Processor](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3Processor) uses
  [Idefics3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3ImageProcessor) for processing images).
- **pixel_attention_mask** (`torch.Tensor` of shape `(batch_size, image_size, image_size)`, *optional*) --
  Mask to avoid performing attention on padding pixel indices.
- **image_hidden_states** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The hidden states of the image encoder after modality projection.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`Idefics3BaseModelOutputWithPast` or `tuple(torch.FloatTensor)`A `Idefics3BaseModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Idefics3Config](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3Config)) and inputs.

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
- **past_key_values** (`~cache_utils.Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the image embeddings, `(batch_size, num_images,
  sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images.
- **pixel_attention_mask** (`torch.LongTensor`, *optional*) --
  The attention mask indicating padded regions in the image.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Idefics3Config](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3Config)) and inputs.

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

## Idefics3ForConditionalGeneration[[transformers.Idefics3ForConditionalGeneration]]

- **config** ([Idefics3ForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3ForConditionalGeneration)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Idefics3 Model with a language modeling head. It is made up a SigLIP vision encoder, with a language modeling head on top.

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
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Idefics3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3ImageProcessor). See `Idefics3ImageProcessor.__call__()` for details ([Idefics3Processor](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3Processor) uses
  [Idefics3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3ImageProcessor) for processing images).
- **pixel_attention_mask** (`torch.Tensor` of shape `(batch_size, image_size, image_size)`, *optional*) --
  Mask to avoid performing attention on padding pixel indices.
- **image_hidden_states** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The hidden states of the image encoder after modality projection.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or `model.image_token_id` (where `model` is your instance of `Idefics3ForConditionalGeneration`).
  Tokens with indices set to `model.image_token_id` are ignored (masked), the loss is only
  computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`Idefics3CausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `Idefics3CausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Idefics3Config](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3Config)) and inputs.
The [Idefics3ForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3ForConditionalGeneration) forward method, overrides the `__call__` special method.

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
- **image_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the image embeddings, `(batch_size, num_images,
  sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder

Example:

```python
>>> import torch
>>> from PIL import Image
>>> from io import BytesIO

>>> from transformers import AutoProcessor, AutoModelForImageTextToText
>>> from transformers.image_utils import load_image

>>> # Note that passing the image urls (instead of the actual pil images) to the processor is also possible
>>> image1 = load_image("https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg")
>>> image2 = load_image("https://cdn.britannica.com/59/94459-050-DBA42467/Skyline-Chicago.jpg")
>>> image3 = load_image("https://cdn.britannica.com/68/170868-050-8DDE8263/Golden-Gate-Bridge-San-Francisco.jpg")

>>> processor = AutoProcessor.from_pretrained("HuggingFaceM4/Idefics3-8B-Llama3")
>>> model = AutoModelForImageTextToText.from_pretrained("HuggingFaceM4/Idefics3-8B-Llama3", dtype=torch.bfloat16, device_map="auto")

>>> # Create inputs
>>> messages = [
...     {
...         "role": "user",
...         "content": [
...             {"type": "image"},
...             {"type": "text", "text": "In this image, we can see the city of New York, and more specifically the Statue of Liberty."},
...             {"type": "image"},
...             {"type": "text", "text": "What can we see in this image?"},
...         ]
...     },
...     {
...         "role": "user",
...         "content": [
...             {"type": "image"},
...             {"type": "text", "text": "In which city is that bridge located?"},
...         ]
...     }
... ]

>>> prompts = [processor.apply_chat_template([message], add_generation_prompt=True) for message in messages]
>>> images = [[image1, image2], [image3]]
>>> inputs = processor(text=prompts, images=images, padding=True, return_tensors="pt").to(model.device)

>>> # Generate
>>> generated_ids = model.generate(**inputs, max_new_tokens=256)
>>> generated_texts = processor.batch_decode(generated_ids, skip_special_tokens=True)

>>> print(generated_texts[0])
Assistant: There are buildings, trees, lights, and water visible in this image.

>>> print(generated_texts[1])
Assistant: The bridge is in San Francisco.
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images.
- **pixel_attention_mask** (`torch.LongTensor`, *optional*) --
  The attention mask indicating padded regions in the image.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Idefics3Config](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3Config)) and inputs.

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
>>> from transformers import AutoProcessor, Idefics3ForConditionalGeneration

>>> model = Idefics3ForConditionalGeneration.from_pretrained("HuggingFaceM4/Idefics3-8B-Llama3")
>>> processor = AutoProcessor.from_pretrained("HuggingFaceM4/Idefics3-8B-Llama3")

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

## Idefics3ImageProcessor[[transformers.Idefics3ImageProcessor]]

- **do_image_splitting** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to split the image into sub-images concatenated with the original image. They are split into patches
  such that each patch has a size of `max_image_size["height"]` x `max_image_size["width"]`.
- **max_image_size** (`Dict`, *kwargs*, *optional*, defaults to `{"longest_edge" -- 364}`):
  Maximum resolution of the patches of images accepted by the model. This is a dictionary containing the key "longest_edge".
- **return_row_col_info** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to return the row and column information of the images.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Idefics3ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **do_image_splitting** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to split the image into sub-images concatenated with the original image. They are split into patches
  such that each patch has a size of `max_image_size["height"]` x `max_image_size["width"]`.
- **max_image_size** (`Dict`, *kwargs*, *optional*, defaults to `{"longest_edge" -- 364}`):
  Maximum resolution of the patches of images accepted by the model. This is a dictionary containing the key "longest_edge".
- **return_row_col_info** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to return the row and column information of the images.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Idefics3ImageProcessorPil[[transformers.Idefics3ImageProcessorPil]]

- **do_image_splitting** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to split the image into sub-images concatenated with the original image. They are split into patches
  such that each patch has a size of `max_image_size["height"]` x `max_image_size["width"]`.
- **max_image_size** (`Dict`, *kwargs*, *optional*, defaults to `{"longest_edge" -- 364}`):
  Maximum resolution of the patches of images accepted by the model. This is a dictionary containing the key "longest_edge".
- **return_row_col_info** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to return the row and column information of the images.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Idefics3ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **do_image_splitting** (`bool`, *kwargs*, *optional*, defaults to `True`) --
  Whether to split the image into sub-images concatenated with the original image. They are split into patches
  such that each patch has a size of `max_image_size["height"]` x `max_image_size["width"]`.
- **max_image_size** (`Dict`, *kwargs*, *optional*, defaults to `{"longest_edge" -- 364}`):
  Maximum resolution of the patches of images accepted by the model. This is a dictionary containing the key "longest_edge".
- **return_row_col_info** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to return the row and column information of the images.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Idefics3Processor[[transformers.Idefics3Processor]]

- **image_processor** (`Idefics3ImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **image_seq_len** (`int`, *optional*, defaults to 169) --
  The length of the image sequence i.e. the number of  tokens per image in the input.
  This parameter is used to build the string from the input prompt and image tokens and should match the
  value the model used. It is computed as: image_seq_len = int(((image_size // patch_size) ** 2) / (scale_factor**2))
- **chat_template** (`str`, *optional*) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
Constructs a Idefics3Processor which wraps a image processor and a tokenizer into a single processor.

[Idefics3Processor](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3Processor) offers all the functionalities of [Idefics3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3ImageProcessor) and `tokenizer_class`. See the
[~Idefics3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/idefics3#transformers.Idefics3ImageProcessor) and `~tokenizer_class` for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor], list[Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]], list[list[Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]]]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, PreTokenizedInput, list[str], list[PreTokenizedInput]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **image_seq_len** (`int`, *optional*) --
  The length of the image sequence. If not provided, the default value of self.image_seq_len is used.
  image_seq_len should be equal to int(((image_size // patch_size) ** 2) / (scale_factor**2))
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.`~tokenization_utils_base.BatchEncoding`- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the `__call__`/`encode_plus`/`batch_encode_plus` methods
  ('input_ids', 'attention_mask', etc.).
- **encoding** (`tokenizers.Encoding` or `Sequence[tokenizers.Encoding]`, *optional*) -- If the tokenizer is a fast tokenizer which outputs additional information like mapping from word/character
  space to token space the `tokenizers.Encoding` instance or list of instance (for batches) hold this
  information.
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **prepend_batch_axis** (`bool`, *optional*, defaults to `False`) -- Whether or not to add a batch axis when converting to tensors (see `tensor_type` above). Note that this
  parameter has an effect if the parameter `tensor_type` is set, *otherwise has no effect*.
- **n_sequences** (`int`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
