# InstructBlipVideo

## Overview

The InstructBLIPVideo is an extension of the models proposed in [InstructBLIP: Towards General-purpose Vision-Language Models with Instruction Tuning](https://huggingface.co/papers/2305.06500) by Wenliang Dai, Junnan Li, Dongxu Li, Anthony Meng Huat Tiong, Junqi Zhao, Weisheng Wang, Boyang Li, Pascale Fung, Steven Hoi.
InstructBLIPVideo uses the same architecture as [InstructBLIP](instructblip) and works with the same checkpoints as [InstructBLIP](instructblip). The only difference is the ability to process videos.

The abstract from the paper is the following:

*General-purpose language models that can solve various language-domain tasks have emerged driven by the pre-training and instruction-tuning pipeline. However, building general-purpose vision-language models is challenging due to the increased task discrepancy introduced by the additional visual input. Although vision-language pre-training has been widely studied, vision-language instruction tuning remains relatively less explored. In this paper, we conduct a systematic and comprehensive study on vision-language instruction tuning based on the pre-trained BLIP-2 models. We gather a wide variety of 26 publicly available datasets, transform them into instruction tuning format and categorize them into two clusters for held-in instruction tuning and held-out zero-shot evaluation. Additionally, we introduce instruction-aware visual feature extraction, a crucial method that enables the model to extract informative features tailored to the given instruction. The resulting InstructBLIP models achieve state-of-the-art zero-shot performance across all 13 held-out datasets, substantially outperforming BLIP-2 and the larger Flamingo. Our models also lead to state-of-the-art performance when finetuned on individual downstream tasks (e.g., 90.7% accuracy on ScienceQA IMG). Furthermore, we qualitatively demonstrate the advantages of InstructBLIP over concurrent multimodal models.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/instructblip_architecture.jpg"
alt="drawing" width="600"/>

 InstructBLIPVideo architecture. Taken from the original paper. 

This model was contributed by [RaushanTurganbay](https://huggingface.co/RaushanTurganbay).
The original code can be found [here](https://github.com/salesforce/LAVIS/tree/main/projects/instructblip).

## Usage tips

- The model was trained by sampling 4 frames per video, so it's recommended to sample 4 frames

> [!NOTE]
> BLIP models after release v4.46 will raise warnings about adding `processor.num_query_tokens = {{num_query_tokens}}` and expand model embeddings layer to add special `<image>` token. It is strongly recommended to add the attributes to the processor if you own the model checkpoint, or open a PR if it is not owned by you. Adding these attributes means that BLIP will add the number of query tokens required per image and expand the text with as many `<image>` placeholders as there will be query tokens. Usually it is around 500 tokens per image, so make sure that the text is not truncated as otherwise there will be failure when merging the embeddings.
The attributes can be obtained from model config, as `model.config.num_query_tokens` and model embeddings expansion can be done by following [this link](https://gist.github.com/zucchini-nlp/e9f20b054fa322f84ac9311d9ab67042).

## InstructBlipVideoConfig[[transformers.InstructBlipVideoConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **qformer_config** (`dict`, *optional*) --
  Dictionary of configuration options used to initialize [InstructBlipVideoQFormerConfig](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoQFormerConfig).
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **num_query_tokens** (`int`, *optional*, defaults to 32) --
  The number of query tokens passed through the Transformer.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **video_token_index** (`int`, *optional*) --
  The video token index used as a placeholder for input videos.

This is the configuration class to store the configuration of a InstructBlipVideoModel. It is used to instantiate a Instructblipvideo
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Salesforce/instructblip-flan-t5-xl](https://huggingface.co/Salesforce/instructblip-flan-t5-xl)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     InstructBlipVideoVisionConfig,
...     InstructBlipVideoQFormerConfig,
...     OPTConfig,
...     InstructBlipVideoConfig,
...     InstructBlipVideoForConditionalGeneration,
... )

>>> # Initializing a InstructBlipVideoConfig with Salesforce/instructblip-flan-t5-xl style configuration
>>> configuration = InstructBlipVideoConfig()

>>> # Initializing a InstructBlipVideoForConditionalGeneration (with random weights) from the Salesforce/instructblip-flan-t5-xl style configuration
>>> model = InstructBlipVideoForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a InstructBlipVideoConfig from a InstructBlipVideoVisionConfig, InstructBlipVideoQFormerConfig and any PreTrainedConfig

>>> # Initializing Instructblipvideo vision, Instructblipvideo Q-Former and language model configurations
>>> vision_config = InstructBlipVideoVisionConfig()
>>> qformer_config = InstructBlipVideoQFormerConfig()
>>> text_config = OPTConfig()

>>> config = InstructBlipVideoConfig(vision_config=vision_config, qformer_config=qformer_config, text_config=text_config)
```

## InstructBlipVideoVisionConfig[[transformers.InstructBlipVideoVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1408`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `6144`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `39`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) --
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

This is the configuration class to store the configuration of a InstructBlipVideoModel. It is used to instantiate a Instructblipvideo
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Salesforce/instructblip-flan-t5-xl](https://huggingface.co/Salesforce/instructblip-flan-t5-xl)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import InstructBlipVideoVisionConfig, InstructBlipVideoVisionModel

>>> # Initializing a InstructBlipVideoVisionConfig with Salesforce/instructblip-flan-t5-xl style configuration
>>> configuration = InstructBlipVideoVisionConfig()

>>> # Initializing a InstructBlipVideoVisionModel (with random weights) from the Salesforce/instructblip-flan-t5-xl style configuration
>>> model = InstructBlipVideoVisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## InstructBlipVideoQFormerConfig[[transformers.InstructBlipVideoQFormerConfig]]

- **vocab_size** (`int`, *optional*, defaults to `30522`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **max_position_embeddings** (`int`, *optional*, defaults to `512`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-12`) --
  The epsilon used by the layer normalization layers.
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **cross_attention_frequency** (`int`, *optional*, defaults to 2) --
  The frequency of adding cross-attention to the Transformer layers.
- **encoder_hidden_size** (`int`, *optional*, defaults to 1408) --
  The hidden size of the hidden states for cross-attention.

This is the configuration class to store the configuration of a InstructBlipVideoModel. It is used to instantiate a Instructblipvideo
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Salesforce/instructblip-flan-t5-xl](https://huggingface.co/Salesforce/instructblip-flan-t5-xl)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import InstructBlipVideoQFormerConfig, InstructBlipVideoQFormerModel

>>> # Initializing a InstructBlipVideo Salesforce/instructblip-flan-t5-xl style configuration
>>> configuration = InstructBlipVideoQFormerConfig()

>>> # Initializing a model (with random weights) from the Salesforce/instructblip-flan-t5-xl style configuration
>>> model = InstructBlipVideoQFormerModel(configuration)
>>> # Accessing the model configuration
>>> configuration = model.config
```

## InstructBlipVideoProcessor[[transformers.InstructBlipVideoProcessor]]

- **video_processor** (`InstructBlipVideoVideoProcessor`) --
  The video processor is a required input.
- **tokenizer** (`GPT2Tokenizer`) --
  The tokenizer is a required input.
- **qformer_tokenizer** (`AutoTokenizer`) --
  An instance of ['PreTrainedTokenizer`]. The Q-Former tokenizer is a required input.
- **num_query_tokens** (`int`, *optional*) --
  Number of tokens used by the Qformer as queries, should be same as in model's config.
Constructs a InstructBlipVideoProcessor which wraps a video processor, a tokenizer, and a qformer tokenizer into a single processor.

[InstructBlipVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoProcessor) offers all the functionalities of [InstructBlipVideoVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoVideoProcessor), [GPT2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Tokenizer), and `{qformer_tokenizer_class}`. See the
[~InstructBlipVideoVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoVideoProcessor), [~GPT2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Tokenizer), and `~{qformer_tokenizer_class}` for more information.

## InstructBlipVideoVideoProcessor[[transformers.InstructBlipVideoVideoProcessor]]

- **do_resize** (`bool`, *optional*, defaults to `self.do_resize`) --
  Whether to resize the video's (height, width) dimensions to the specified `size`. Can be overridden by the
  `do_resize` parameter in the `preprocess` method.
- **size** (`dict`, *optional*, defaults to `self.size`) --
  Size of the output video after resizing. Can be overridden by the `size` parameter in the `preprocess`
  method.
- **size_divisor** (`int`, *optional*, defaults to `self.size_divisor`) --
  The size by which to make sure both the height and width can be divided.
- **default_to_square** (`bool`, *optional*, defaults to `self.default_to_square`) --
  Whether to default to a square video when resizing, if size is an int.
- **resample** (`PILImageResampling`, *optional*, defaults to `self.resample`) --
  Resampling filter to use if resizing the video. Only has an effect if `do_resize` is set to `True`. Can be
  overridden by the `resample` parameter in the `preprocess` method.
- **do_center_crop** (`bool`, *optional*, defaults to `self.do_center_crop`) --
  Whether to center crop the video to the specified `crop_size`. Can be overridden by `do_center_crop` in the
  `preprocess` method.
- **crop_size** (`dict[str, int]` *optional*, defaults to `self.crop_size`) --
  Size of the output video after applying `center_crop`. Can be overridden by `crop_size` in the `preprocess`
  method.
- **do_rescale** (`bool`, *optional*, defaults to `self.do_rescale`) --
  Whether to rescale the video by the specified scale `rescale_factor`. Can be overridden by the
  `do_rescale` parameter in the `preprocess` method.
- **rescale_factor** (`int` or `float`, *optional*, defaults to `self.rescale_factor`) --
  Scale factor to use if rescaling the video. Only has an effect if `do_rescale` is set to `True`. Can be
  overridden by the `rescale_factor` parameter in the `preprocess` method.
- **do_normalize** (`bool`, *optional*, defaults to `self.do_normalize`) --
  Whether to normalize the video. Can be overridden by the `do_normalize` parameter in the `preprocess`
  method. Can be overridden by the `do_normalize` parameter in the `preprocess` method.
- **image_mean** (`float` or `list[float]`, *optional*, defaults to `self.image_mean`) --
  Mean to use if normalizing the video. This is a float or list of floats the length of the number of
  channels in the video. Can be overridden by the `image_mean` parameter in the `preprocess` method. Can be
  overridden by the `image_mean` parameter in the `preprocess` method.
- **image_std** (`float` or `list[float]`, *optional*, defaults to `self.image_std`) --
  Standard deviation to use if normalizing the video. This is a float or list of floats the length of the
  number of channels in the video. Can be overridden by the `image_std` parameter in the `preprocess` method.
  Can be overridden by the `image_std` parameter in the `preprocess` method.
- **do_convert_rgb** (`bool`, *optional*, defaults to `self.image_std`) --
  Whether to convert the video to RGB.
- **video_metadata** (`VideoMetadata`, *optional*) --
  Metadata of the video containing information about total duration, fps and total number of frames.
- **do_sample_frames** (`int`, *optional*, defaults to `self.do_sample_frames`) --
  Whether to sample frames from the video before processing or to process the whole video.
- **num_frames** (`int`, *optional*, defaults to `self.num_frames`) --
  Maximum number of frames to sample when `do_sample_frames=True`.
- **fps** (`int` or `float`, *optional*, defaults to `self.fps`) --
  Target frames to sample per second when `do_sample_frames=True`.
- **return_tensors** (`str` or `TensorType`, *optional*) --
  Returns stacked tensors if set to `pt, otherwise returns a list of tensors.
- **data_format** (`ChannelDimension` or `str`, *optional*, defaults to `ChannelDimension.FIRST`) --
  The channel dimension format for the output video. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: video in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: video in (height, width, num_channels) format.
  - Unset: Use the channel dimension format of the input video.
- **input_data_format** (`ChannelDimension` or `str`, *optional*) --
  The channel dimension format for the input video. If unset, the channel dimension format is inferred
  from the input video. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: video in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: video in (height, width, num_channels) format.
  - `"none"` or `ChannelDimension.NONE`: video in (height, width) format.
- **device** (`torch.device`, *optional*) --
  The device to process the videos on. If unset, the device is inferred from the input videos.
- **return_metadata** (`bool`, *optional*) --
  Whether to return video metadata or not.

## InstructBlipVideoVisionModel[[transformers.InstructBlipVideoVisionModel]]

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details ([InstructBlipVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoProcessor) uses
  `image_processor_class` for processing images).
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([InstructBlipVideoConfig](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoConfig)) and inputs.
The [InstructBlipVideoVisionModel](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

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

## InstructBlipVideoQFormerModel[[transformers.InstructBlipVideoQFormerModel]]

Querying Transformer (Q-Former), used in InstructBlipVideo. Slightly modified from BLIP-2 as it also takes the
instruction as input.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **query_embeds** (`torch.FloatTensor`  of shape `(batch_size, sequence_length, hidden_size)`) --
  Hidden states to be used in the attention computation. If cross-attention,
  will be used for the query (i.e., key and value will use the encoder_hidden_states).
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention
  if the model is configured as a decoder.
- **encoder_attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on the padding token indices of the encoder input. This mask is used in
  the cross-attention if the model is configured as a decoder. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.[BaseModelOutputWithPoolingAndCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPoolingAndCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([InstructBlipVideoConfig](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoConfig)) and inputs.
The [InstructBlipVideoQFormerModel](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoQFormerModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

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
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` and `config.add_cross_attention=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.

## InstructBlipVideoModel[[transformers.InstructBlipVideoModel]]

- **config** ([InstructBlipVideoConfig](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

InstructBlipVideo base Model consisting of language model, qformer and vision encoder.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details ([InstructBlipVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoProcessor) uses
  `image_processor_class` for processing images).
- **qformer_input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary of the Q-Former. Input tokens can optionally be provided
  to serve as text prompt, which the Q-Former model will encode.

  Indices can be obtained using [InstructBlipVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoProcessor). See `InstructBlipVideoProcessor.__call__()` for
  details.

  [What are input IDs?](../glossary#input-ids)
- **qformer_attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **input_ids** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Indices of decoder input sequence tokens in the vocabulary.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are decoder input IDs?](../glossary#decoder-input-ids)
- **decoder_attention_mask** (`torch.BoolTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also
  be used by default.

  Only relevant in case an encoder-decoder language model (like T5) is used.
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`InstructBlipVideoForConditionalGenerationModelOutput` or `tuple(torch.FloatTensor)`A `InstructBlipVideoForConditionalGenerationModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([InstructBlipVideoConfig](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoConfig)) and inputs.
The [InstructBlipVideoModel](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor`, *optional*, returned when `labels` is provided, `torch.FloatTensor` of shape `(1,)`) -- Language modeling loss from the language model.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head of the language model.
- **vision_outputs** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- Outputs of the vision encoder.
- **qformer_outputs** (`~modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions`, *optional*) -- Outputs of the Q-Former (Querying Transformer).
- **language_model_outputs** (`CausalLMOutputWithPast` or `Seq2SeqLMOutput`) -- Outputs of the language model.

## InstructBlipVideoForConditionalGeneration[[transformers.InstructBlipVideoForConditionalGeneration]]

- **config** ([InstructBlipVideoConfig](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

InstructBlipVideo Model for generating text given an image and an optional text prompt. The model consists of a vision
encoder, Querying Transformer (Q-Former) and a language model.

One can optionally pass `input_ids` to the model, which serve as a text prompt, to make the language model continue
the prompt. Otherwise, the language model starts generating text from the [BOS] (beginning-of-sequence) token.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details ([InstructBlipVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoProcessor) uses
  `image_processor_class` for processing images).
- **qformer_input_ids** (`torch.LongTensor` of shape (batch_size, sequence_length)) --
  The sequence used as a prompt to be fed to the Q-Former module.
- **qformer_attention_mask** (`torch.LongTensor` of shape (batch_size, sequence_length), *optional*) --
  Mask to avoid performing attention on padding token indices.
- **input_ids** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
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
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`InstructBlipVideoForConditionalGenerationModelOutput` or `tuple(torch.FloatTensor)`A `InstructBlipVideoForConditionalGenerationModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([InstructBlipVideoConfig](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoConfig)) and inputs.
The [InstructBlipVideoForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor`, *optional*, returned when `labels` is provided, `torch.FloatTensor` of shape `(1,)`) -- Language modeling loss from the language model.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head of the language model.
- **vision_outputs** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- Outputs of the vision encoder.
- **qformer_outputs** (`~modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions`, *optional*) -- Outputs of the Q-Former (Querying Transformer).
- **language_model_outputs** (`CausalLMOutputWithPast` or `Seq2SeqLMOutput`) -- Outputs of the language model.

Examples:

```python
>>> from transformers import InstructBlipVideoProcessor, InstructBlipVideoForConditionalGeneration
>>> import torch
>>> from huggingface_hub import hf_hub_download
>>> import av
>>> import numpy as np

>>> def read_video_pyav(container, indices):
...     '''
...     Decode the video with PyAV decoder.
...     Args:
...         container (`av.container.input.InputContainer`): PyAV container.
...         indices (`list[int]`): List of frame indices to decode.
...     Returns:
...         result (np.ndarray): np array of decoded frames of shape (num_frames, height, width, 3).
...     '''
...     frames = []
...     container.seek(0)
...     start_index = indices[0]
...     end_index = indices[-1]
...     for i, frame in enumerate(container.decode(video=0)):
...         if i > end_index:
...             break
...         if i >= start_index and i in indices:
...             frames.append(frame)
...     return np.stack([x.to_ndarray(format="rgb24") for x in frames])

>>> model = InstructBlipVideoForConditionalGeneration.from_pretrained("Salesforce/instructblip-vicuna-7b", device_map="auto")
>>> processor = InstructBlipVideoProcessor.from_pretrained("Salesforce/instructblip-vicuna-7b")

>>> file_path = hf_hub_download(
...       repo_id="nielsr/video-demo", filename="eating_spaghetti.mp4", repo_type="dataset"
... )
>>> container = av.open(file_path)

>>> # sample uniformly 4 frames from the videWhy is this video funny?o
>>> total_frames = container.streams.video[0].frames
>>> indices = np.arange(0, total_frames, total_frames / 4).astype(int)
>>> clip = read_video_pyav(container, indices)

>>> prompt = "What is happening in the video?"
>>> inputs = processor(text=prompt, images=clip, return_tensors="pt").to(model.device)

>>> outputs = model.generate(
...     **inputs,
...     do_sample=False,
...     num_beams=5,
...     max_length=256,
...     repetition_penalty=1.5,
...     length_penalty=1.0,
... )
>>> generated_text = processor.batch_decode(outputs, skip_special_tokens=True)[0].strip()
>>> print(generated_text)
"A person is eating a bowl of pasta, and they are using a fork to eat it. The person is sitting at a table, and the plate of pasta is on the table in front"
```

- **pixel_values** (`torch.FloatTensor` of shape (batch_size, num_channels, height, width) or --
  (batch_size, num_frames, num_channels, height, width)): Input images or videos to be processed.
- **qformer_input_ids** (`torch.LongTensor` of shape (batch_size, sequence_length), *optional*) --
  The sequence used as a prompt to be fed to the Q-Former module.
- **qformer_attention_mask** (`torch.LongTensor` of shape (batch_size, sequence_length), *optional*) --
  Mask to avoid performing attention on padding token indices.
- **input_ids** (`torch.LongTensor` of shape (batch_size, sequence_length), *optional*) --
  The sequence used as a prompt for the generation.
- **attention_mask** (`torch.LongTensor` of shape (batch_size, sequence_length), *optional*) --
  Mask to avoid performing attention on padding token indices.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) --
  Embedded representation of the inputs. Should be float, not int tokens.
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the positional encoding of the image embeddings.captions (list)A list of strings of length batch_size * num_captions.

Overrides `generate` function to be able to use the model as a conditional generator.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images.
- **qformer_input_ids** (`torch.LongTensor` of shape (batch_size, sequence_length)) --
  The sequence used as a prompt to be fed to the Q-Former module.
- **qformer_attention_mask** (`torch.LongTensor` of shape (batch_size, sequence_length), *optional*) --
  Mask to avoid performing attention on padding token indices.
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.`BaseModelOutputWithVisionQformerOutputs` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithVisionQformerOutputs` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([InstructBlipVideoConfig](/docs/transformers/v5.14.0/en/model_doc/instructblipvideo#transformers.InstructBlipVideoConfig)) and inputs.

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
- **vision_outputs** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- Outputs of the vision encoder.
- **qformer_outputs** (`~modeling_outputs.BaseModelOutputWithPoolingAndCrossAttentions`, *optional*) -- Outputs of the Q-Former (Querying Transformer).

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, InstructBlipVideoForConditionalGeneration

>>> model = InstructBlipVideoForConditionalGeneration.from_pretrained("Salesforce/instructblip-flan-t5-xl")
>>> processor = AutoProcessor.from_pretrained("Salesforce/instructblip-flan-t5-xl")

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
