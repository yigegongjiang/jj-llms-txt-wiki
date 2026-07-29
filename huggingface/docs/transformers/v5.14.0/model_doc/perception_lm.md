# PerceptionLM

## Overview

The [PerceptionLM](https://huggingface.co/papers/2504.13180) model was proposed in [PerceptionLM: Open-Access Data and Models for Detailed Visual Understanding](https://ai.meta.com/research/publications/perceptionlm-open-access-data-and-models-for-detailed-visual-understanding/) by Jang Hyun Cho et al. It's a fully open, reproducible model for transparent research in image and video understanding. PLM consists of
a vision encoder with a small scale (<8B parameters) LLM decoder.

The abstract from the paper is the following:

*Vision-language models are integral to computer vision research, yet many high-performing models
remain closed-source, obscuring their data, design and training recipe. The research community
has responded by using distillation from black-box models to label training data, achieving strong
benchmark results, at the cost of measurable scientific progress. However, without knowing the details
of the teacher model and its data sources, scientific progress remains difficult to measure. In this
paper, we study building a Perception Language Model (PLM) in a fully open and reproducible
framework for transparent research in image and video understanding. We analyze standard training
pipelines without distillation from proprietary models and explore large-scale synthetic data to identify
critical data gaps, particularly in detailed video understanding. To bridge these gaps, we release 2.8M
human-labeled instances of fine-grained video question-answer pairs and spatio-temporally grounded
video captions. Additionally, we introduce PLM–VideoBench, a suite for evaluating challenging video
understanding tasks focusing on the ability to reason about “what”, “where”, “when”, and “how” of a
video. We make our work fully reproducible by providing data, training recipes, code & models.*

This model was contributed by [shumingh](https://huggingface.co/shumingh).
The original code can be found [here](https://github.com/facebookresearch/perception_models).

## PerceptionLMConfig[[transformers.PerceptionLMConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_use_cls_token** (`bool`, *optional*, defaults to `True`) --
  Whether CLS token is used in the vision backbone. If used, we remove CLS token embedding from vision output.
- **projector_pooling_ratio** (`int`, *optional*, defaults to 1) --
  The pooling ratio used in the multimodal projector.
- **image_token_id** (`int`, *optional*, defaults to `128002`) --
  The image token index used as a placeholder for input images.
- **video_token_id** (`int`, *optional*, defaults to `128003`) --
  The video token index used as a placeholder for input videos.
- **tie_word_embeddings** (`bool`, *optional*) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a PerceptionLMModel. It is used to instantiate a Perception Lm
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/Perception-LM-1B](https://huggingface.co/facebook/Perception-LM-1B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## PerceptionLMProcessor[[transformers.PerceptionLMProcessor]]

- **video_processor** (`PerceptionLMVideoProcessor`) --
  The video processor is a required input.
- **image_processor** (`PerceptionLMImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **patch_size** (`int`, *optional*) --
  Patch size from the vision tower.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **pooling_ratio** (`int`, *optional*, defaults to 2) --
  Pooling ratio for vision tokens. If not 1, 2D adaptive pooling is applied over projected vision tokens.
Constructs a PerceptionLMProcessor which wraps a video processor, a image processor, and a tokenizer into a single processor.

[PerceptionLMProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMProcessor) offers all the functionalities of [PerceptionLMVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMVideoProcessor), [PerceptionLMImageProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMImageProcessor), and `tokenizer_class`. See the
[~PerceptionLMVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMVideoProcessor), [~PerceptionLMImageProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMImageProcessor), and `~tokenizer_class` for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **videos** (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) --
  Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If
  passing in videos with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:

- **input_ids** -- List of token ids to be fed to a model. Returned when `text` is provided.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is provided).
- **pixel_values** -- Pixel values to be fed to a model. Returned when `images` is provided.
- **pixel_values_videos** -- Video pixel values to be fed to a model. Returned when `videos` is provided.

## PerceptionLMImageProcessor[[transformers.PerceptionLMImageProcessor]]

- **vision_input_type** (`str`, *kwargs*, *optional*, defaults to `"thumb+tile"`) --
  Vision processing strategy. `"thumb+tile"` uses both thumbnails and multiple tiles for
  multi-scale processing, otherwise uses single tile for lower memory usage.
- **tile_size** (`int`, *kwargs*, *optional*, defaults to `448`) --
  Height and width dimension (in pixels) of each tile used for image processing.
- **max_num_tiles** (`int`, *kwargs*, *optional*, defaults to `36`) --
  Maximum number of tiles an image can be split into based on its aspect ratio.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PerceptionLMImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **vision_input_type** (`str`, *kwargs*, *optional*, defaults to `"thumb+tile"`) --
  Vision processing strategy. `"thumb+tile"` uses both thumbnails and multiple tiles for
  multi-scale processing, otherwise uses single tile for lower memory usage.
- **tile_size** (`int`, *kwargs*, *optional*, defaults to `448`) --
  Height and width dimension (in pixels) of each tile used for image processing.
- **max_num_tiles** (`int`, *kwargs*, *optional*, defaults to `36`) --
  Maximum number of tiles an image can be split into based on its aspect ratio.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## PerceptionLMVideoProcessor[[transformers.PerceptionLMVideoProcessor]]

## PerceptionLMModel[[transformers.PerceptionLMModel]]

- **config** ([PerceptionLMConfig](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Perception Lm Model outputting raw hidden-states without any specific head on top.

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
  [PerceptionLMImageProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMImageProcessor). See `PerceptionLMImageProcessor.__call__()` for details ([PerceptionLMProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMProcessor) uses
  [PerceptionLMImageProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMImageProcessor) for processing images).
- **pixel_values_videos** (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [PerceptionLMVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMVideoProcessor). See `PerceptionLMVideoProcessor.__call__()` for details ([PerceptionLMProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMProcessor) uses
  [PerceptionLMVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMVideoProcessor) for processing videos).
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
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`PerceptionLMModelOutputWithPast` or `tuple(torch.FloatTensor)`A `PerceptionLMModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PerceptionLMConfig](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMConfig)) and inputs.
The [PerceptionLMModel](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMModel) forward method, overrides the `__call__` special method.

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
  Image hidden_states of the model produced by the vision encoder and after projecting the last hidden state.
- **video_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_videos, sequence_length, hidden_size)`.
  Video hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PerceptionLMImageProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMImageProcessor). See `PerceptionLMImageProcessor.__call__()` for details ([PerceptionLMProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMProcessor) uses
  [PerceptionLMImageProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMImageProcessor) for processing images).[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PerceptionLMConfig](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMConfig)) and inputs.
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

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

## PerceptionLMForConditionalGeneration[[transformers.PerceptionLMForConditionalGeneration]]

- **config** ([PerceptionLMConfig](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Perception Lm Model for token generation conditioned on other modalities (e.g. image-text-to-text generation).

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
  [PerceptionLMImageProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMImageProcessor). See `PerceptionLMImageProcessor.__call__()` for details ([PerceptionLMProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMProcessor) uses
  [PerceptionLMImageProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMImageProcessor) for processing images).
- **pixel_values_videos** (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [PerceptionLMVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMVideoProcessor). See `PerceptionLMVideoProcessor.__call__()` for details ([PerceptionLMProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMProcessor) uses
  [PerceptionLMVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMVideoProcessor) for processing videos).
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).`PerceptionLMCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `PerceptionLMCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PerceptionLMConfig](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMConfig)) and inputs.
The [PerceptionLMForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/perception_lm#transformers.PerceptionLMForConditionalGeneration) forward method, overrides the `__call__` special method.

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
  Image hidden_states of the model produced by the vision encoder and after projecting the last hidden state.
- **video_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_videos, sequence_length, hidden_size)`.
  Video hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
from transformers import AutoProcessor, AutoModelForImageTextToText
from huggingface_hub import hf_hub_download

MODEL_PATH = "facebook/Perception-LM-1B"
processor = AutoProcessor.from_pretrained(MODEL_PATH, use_fast=True)
model = AutoModelForImageTextToText.from_pretrained(MODEL_PATH).to("cuda")
test_image_file = hf_hub_download(
            repo_id="shumingh/perception_lm_test_images",
            filename="14496_0.PNG",
            repo_type="dataset",
)
conversation = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "url": test_image_file,
            },
            {"type": "text", "text": "Describe the bar plot in the image."},
        ],
    }
]

inputs = processor.apply_chat_template(
    [conversation],
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
)
inputs = inputs.to(model.device)
generate_ids = model.generate(**inputs, max_new_tokens=256)
input_length = inputs["input_ids"].shape[1]
generate_ids_without_inputs = generate_ids[:, input_length:]

for output in processor.batch_decode(generate_ids_without_inputs, skip_special_tokens=True):
    print(output)
```
