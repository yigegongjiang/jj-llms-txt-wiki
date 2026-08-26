# FLAVA

## Overview

The FLAVA model was proposed in [FLAVA: A Foundational Language And Vision Alignment Model](https://huggingface.co/papers/2112.04482) by Amanpreet Singh, Ronghang Hu, Vedanuj Goswami, Guillaume Couairon, Wojciech Galuba, Marcus Rohrbach, and Douwe Kiela and is accepted at CVPR 2022.

The paper aims at creating a single unified foundation model which can work across vision, language
as well as vision-and-language multimodal tasks.

The abstract from the paper is the following:

*State-of-the-art vision and vision-and-language models rely on large-scale visio-linguistic pretraining for obtaining good performance on a variety
of downstream tasks. Generally, such models are often either cross-modal (contrastive) or multi-modal
(with earlier fusion) but not both; and they often only target specific modalities or tasks. A promising
direction would be to use a single holistic universal model, as a "foundation", that targets all modalities
at once -- a true vision and language foundation model should be good at vision tasks, language tasks, and
cross- and multi-modal vision and language tasks. We introduce FLAVA as such a model and demonstrate
impressive performance on a wide range of 35 tasks spanning these target modalities.*

This model was contributed by [aps](https://huggingface.co/aps). The original code can be found [here](https://github.com/facebookresearch/multimodal/tree/main/examples/flava).

## FlavaConfig[[transformers.FlavaConfig]]

#### transformers.FlavaConfig[[transformers.FlavaConfig]]

```python
transformers.FlavaConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, image_config: dict[str, typing.Any] | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict[str, typing.Any] | transformers.configuration_utils.PreTrainedConfig | None = None, multimodal_config: dict[str, typing.Any] | transformers.configuration_utils.PreTrainedConfig | None = None, image_codebook_config: dict[str, typing.Any] | transformers.configuration_utils.PreTrainedConfig | None = None, hidden_size: int = 768, layer_norm_eps: float = 1e-12, projection_dim: int = 768, init_codebook: bool = True, logit_scale_init_value: float = 2.6592, initializer_range: float = 0.02, ce_ignore_index: int = -100, mim_weight: float = 1.0, mlm_weight: float = 1.0, global_contrastive_weight: float = 1.0, itm_weight: float = 1.0, mmm_image_weight: float = 1.0, mmm_text_weight: float = 1.0, global_backprop_contrastive: bool = True, skip_unmasked_multimodal_encoder: bool = True, return_loss: bool = True, tie_word_embeddings: bool = True, initializer_factor: float = 1.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/configuration_flava.py#L183)

**Parameters:**

image_config (`dict`, *optional*) : Dictionary of configuration options used to initialize [FlavaImageConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageConfig).

text_config (`Union[dict[str, Any], ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

multimodal_config (`dict`, *optional*) : Dictionary of configuration options used to initialize [FlavaMultimodalConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaMultimodalConfig).

image_codebook_config (`dict`, *optional*) : Dictionary of configuration options used to initialize `FlavaCodebookConfig`.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

layer_norm_eps (`float`, *optional*, defaults to `1e-12`) : The epsilon used by the layer normalization layers.

projection_dim (`int`, *optional*, defaults to `768`) : Dimensionality of text and vision projection layers.

init_codebook (`bool`, *optional*, defaults to `True`) : Whether to initialize the codebook

logit_scale_init_value (`float`, *optional*, defaults to 2.6592) : The initial value of the *logit_scale* parameter. Default is used as per the original FLAVA/CLIP implementation.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

ce_ignore_index (`int`, *optional*, defaults to -100) : Cross entropy index to ignore.

mim_weight (`float`, *optional*, defaults to 1.0) : Weight to be assigned to MIM (Masked Image Modeling) unimodal loss

mlm_weight (`float`, *optional*, defaults to 1.0) : Weight to be assigned to MLM (Masked Language Modeling) unimodal loss

global_contrastive_weight (`float`, *optional*, defaults to 1.0) : Weight to be assigned to global contrastive cross-alignment loss.

itm_weight (`float`, *optional*, defaults to 1.0) : Weight to be assigned to image-text matching multimodal loss.

mmm_image_weight (`float`, *optional*, defaults to 1.0) : Weight to be assigned to MMM loss's image part.

mmm_text_weight (`float`, *optional*, defaults to 1.0) : Weight to be assigned to MMM loss's text part.

global_backprop_contrastive (`bool`, *optional*, defaults to `True`) : Whether to use global backpropgation through all workers in contrastive loss.

skip_unmasked_multimodal_encoder (`bool`, *optional*, defaults to `True`) : Whether to skip running unmasked multimodal encoder whose outputs are not used by FLAVA losses.

return_loss (`bool`, *optional*, defaults to `True`) : Whether to return loss or not

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

initializer_factor (`float`, *optional*, defaults to `1.0`) : A factor for initializing all weight matrices (should be kept to 1, used internally for initialization testing).

This is the configuration class to store the configuration of a FlavaModel. It is used to instantiate a Flava
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/flava-full](https://huggingface.co/facebook/flava-full)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import FlavaConfig, FlavaModel, FlavaForPreTraining

>>> # Initializing a FlavaConfig with style configuration
>>> configuration = FlavaConfig()

>>> # Initializing a FlavaModel and FlavaForPreTraining model (with random weights) from the style configuration
>>> model = FlavaModel(configuration)
>>> model_pre = FlavaForPreTraining(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
>>> configuration_pre = model_pre.config
```

## FlavaTextConfig[[transformers.FlavaTextConfig]]

#### transformers.FlavaTextConfig[[transformers.FlavaTextConfig]]

```python
transformers.FlavaTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 30522, type_vocab_size: int = 2, max_position_embeddings: int = 512, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, intermediate_size: int = 3072, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.0, attention_probs_dropout_prob: float | int = 0.0, initializer_range: float = 0.02, layer_norm_eps: float = 1e-12, pad_token_id: int | None = 0, qkv_bias: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/configuration_flava.py#L71)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `30522`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

type_vocab_size (`int`, *optional*, defaults to `2`) : The vocabulary size of the `token_type_ids`.

max_position_embeddings (`int`, *optional*, defaults to `512`) : The maximum sequence length that this model might ever be used with.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-12`) : The epsilon used by the layer normalization layers.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

This is the configuration class to store the configuration of a FlavaModel. It is used to instantiate a Flava
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/flava-full](https://huggingface.co/facebook/flava-full)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import FlavaTextConfig, FlavaTextModel

>>> # Initializing a FlavaTextModel with  style configuration
>>> configuration = FlavaTextConfig()

>>> # Initializing a FlavaTextModel model (with random weights) from the style configuration
>>> model = FlavaTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## FlavaImageConfig[[transformers.FlavaImageConfig]]

#### transformers.FlavaImageConfig[[transformers.FlavaImageConfig]]

```python
transformers.FlavaImageConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, intermediate_size: int = 3072, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.0, attention_probs_dropout_prob: float | int = 0.0, initializer_range: float = 0.02, layer_norm_eps: float = 1e-12, image_size: int | list[int] | tuple[int, int] = 224, patch_size: int | list[int] | tuple[int, int] = 16, num_channels: int = 3, qkv_bias: bool = True, mask_token: bool = True, vocab_size: int = 8192)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/configuration_flava.py#L29)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-12`) : The epsilon used by the layer normalization layers.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

mask_token (`bool`, *optional*, defaults to `True`) : Whether to use a mask token or not. Used in MIM (Masked Image Modeling) loss for FLAVA.

vocab_size (`int`, *optional*, defaults to `8192`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

This is the configuration class to store the configuration of a FlavaModel. It is used to instantiate a Flava
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/flava-full](https://huggingface.co/facebook/flava-full)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import FlavaImageConfig, FlavaImageModel

>>> # Initializing a FlavaImageModel with  style configuration
>>> configuration = FlavaImageConfig()

>>> # Initializing a FlavaImageModel model (with random weights) from the style configuration
>>> model = FlavaImageModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## FlavaMultimodalConfig[[transformers.FlavaMultimodalConfig]]

#### transformers.FlavaMultimodalConfig[[transformers.FlavaMultimodalConfig]]

```python
transformers.FlavaMultimodalConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, num_hidden_layers: int = 6, num_attention_heads: int = 12, intermediate_size: int = 3072, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.0, attention_probs_dropout_prob: float | int = 0.0, initializer_range: float = 0.02, layer_norm_eps: float = 1e-12, qkv_bias: bool = True, use_cls_token: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/configuration_flava.py#L109)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-12`) : The epsilon used by the layer normalization layers.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

use_cls_token (`bool`, *optional*, defaults to `True`) : Whether to use an extra CLS token for multimodal settings. Usually needed by the FLAVA model.

This is the configuration class to store the configuration of a FlavaModel. It is used to instantiate a Flava
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/flava-full](https://huggingface.co/facebook/flava-full)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import FlavaMultimodalConfig, FlavaMultimodalModel

>>> # Initializing a FlavaMultimodalModel with  style configuration
>>> configuration = FlavaMultimodalConfig()

>>> # Initializing a FlavaMultimodalModel model (with random weights) from the style configuration
>>> model = FlavaMultimodalModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## FlavaImageCodebookConfig[[transformers.FlavaImageCodebookConfig]]

#### transformers.FlavaImageCodebookConfig[[transformers.FlavaImageCodebookConfig]]

```python
transformers.FlavaImageCodebookConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_groups: int = 4, input_channels: int = 3, num_blocks_per_group: int = 2, hidden_size: int = 256, vocab_size: int = 8192, freeze: bool = True, initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/configuration_flava.py#L147)

**Parameters:**

num_groups (`int`, *optional*, defaults to 4) : Number of groups to be created. This parameter as of now doesn't affect the model and is used for some internal calculation and estimations.

input_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

num_blocks_per_group (`int`, *optional*, defaults to 2) : Number of conv-based blocks per group.

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

vocab_size (`int`, *optional*, defaults to `8192`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

freeze (`bool`, defaults to `True`) : Whether to freeze the weights of the model.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a FlavaModel. It is used to instantiate a Flava
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/flava-full](https://huggingface.co/facebook/flava-full)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import FlavaImageCodebookConfig, FlavaImageCodebook

>>> # Initializing a FlavaImageCodebook with style configuration
>>> configuration = FlavaImageCodebookConfig()

>>> # Initializing a FlavaImageCodebook model (with random weights) from the style configuration
>>> model = FlavaImageCodebook(configuration)
>>> # Accessing the model configuration
>>> configuration = model.config
```

## FlavaProcessor[[transformers.FlavaProcessor]]

#### transformers.FlavaProcessor[[transformers.FlavaProcessor]]

```python
transformers.FlavaProcessor(image_processor = None, tokenizer = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/processing_flava.py#L23)

**Parameters:**

image_processor (`FlavaImageProcessor`) : The image processor is a required input.

tokenizer (`BertTokenizer`) : The tokenizer is a required input.

Constructs a FlavaProcessor which wraps a image processor and a tokenizer into a single processor.

[FlavaProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaProcessor) offers all the functionalities of [FlavaImageProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageProcessor) and [BertTokenizer](/docs/transformers/v5.15.1/en/model_doc/layoutlm#transformers.BertTokenizer). See the
[~FlavaImageProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageProcessor) and [~BertTokenizer](/docs/transformers/v5.15.1/en/model_doc/layoutlm#transformers.BertTokenizer) for more information.

#### __call__[[transformers.FlavaProcessor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: str | list[str] | list[list[str]] | None = None, videos: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]], NoneType] = None, audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L651)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

videos (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) : Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If passing in videos with pixel values between 0 and 1, set `do_rescale=False`.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

## FlavaImageProcessor[[transformers.FlavaImageProcessor]]

#### transformers.FlavaImageProcessor[[transformers.FlavaImageProcessor]]

```python
transformers.FlavaImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/image_processing_flava.py#L213)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 224, 'width': 224}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 224, 'width': 224}`): Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.48145466, 0.4578275, 0.40821073]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.26862954, 0.26130258, 0.27577711]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

return_image_mask (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to return the image mask. Can be overridden by the `return_image_mask` parameter in `preprocess`.

input_size_patches (`int`, *kwargs*, *optional*, defaults to `14`) : Number of patches in the image in height and width direction. 14x14 = 196 total patches. Can be overridden by the `input_size_patches` parameter in `preprocess`.

total_mask_patches (`int`, *kwargs*, *optional*, defaults to `75`) : Total number of patches that should be masked. Can be overridden by the `total_mask_patches` parameter in `preprocess`.

mask_group_min_patches (`int`, *kwargs*, *optional*, defaults to `16`) : Minimum number of patches that should be masked. Can be overridden by the `mask_group_min_patches` parameter in `preprocess`.

mask_group_max_patches (`int`, *kwargs*, *optional*) : Maximum number of patches that should be masked. Can be overridden by the `mask_group_max_patches` parameter in `preprocess`.

mask_group_min_aspect_ratio (`float`, *kwargs*, *optional*, defaults to `0.3`) : Minimum aspect ratio of the mask window. Can be overridden by the `mask_group_min_aspect_ratio` parameter in `preprocess`.

mask_group_max_aspect_ratio (`float`, *kwargs*, *optional*) : Maximum aspect ratio of the mask window. Can be overridden by the `mask_group_max_aspect_ratio` parameter in `preprocess`.

return_codebook_pixels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to return the codebook pixel values.

codebook_do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the input for codebook to a certain. Can be overridden by the `codebook_do_resize` parameter in `preprocess`. `codebook_size`.

codebook_size (`dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" : 224, "width": 224}`): Resize the input for codebook to the given size. Can be overridden by the `codebook_size` parameter in `preprocess`.

codebook_resample (`PILImageResampling`, *kwargs*, *optional*, defaults to `PILImageResampling.LANCZOS`) : Resampling filter to use if resizing the codebook image. With torchvision < 0.27, LANCZOS is not supported for torch Tensors and BICUBIC is used as the closest alternative. Can be overridden by the `codebook_resample` parameter in `preprocess`.

codebook_do_center_crop (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to crop the input for codebook at the center. If the input size is smaller than `codebook_crop_size` along any edge, the image is padded with 0's and then center cropped. Can be overridden by the `codebook_do_center_crop` parameter in `preprocess`.

codebook_crop_size (`dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" : 224, "width": 224}`): Desired output size for codebook input when applying center-cropping. Can be overridden by the `codebook_crop_size` parameter in `preprocess`.

codebook_do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the input for codebook by the specified scale `codebook_rescale_factor`. Can be overridden by the `codebook_do_rescale` parameter in `preprocess`.

codebook_rescale_factor (`int`, *kwargs* or `float`, *optional*, defaults to `1/255`) : Defines the scale factor to use if rescaling the codebook image. Can be overridden by the `codebook_rescale_factor` parameter in `preprocess`.

codebook_do_map_pixels (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to map the pixel values of the codebook input to (1 - 2e)x + e. Can be overridden by the `codebook_do_map_pixels` parameter in `preprocess`.

codebook_do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether or not to normalize the input for codebook with `codebook_image_mean` and `codebook_image_std`. Can be overridden by the `codebook_do_normalize` parameter in `preprocess`.

codebook_image_mean (`Optional[Union[float, *kwargs*, Iterable[float]]]`, *optional*, defaults to `[0, 0, 0]`) : The sequence of means for each channel, to be used when normalizing images for codebook. Can be overridden by the `codebook_image_mean` parameter in `preprocess`.

codebook_image_std (`Optional[Union[float, *kwargs*, Iterable[float]]]`, *optional*, defaults to `[0.5, 0.5, 0.5]`) : The sequence of standard deviations for each channel, to be used when normalizing images for codebook. Can be overridden by the `codebook_image_std` parameter in `preprocess`.

Constructs a FlavaImageProcessor image processor.

#### preprocess[[transformers.FlavaImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/image_processing_flava.py#L250)

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

return_image_mask (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to return the image mask. Can be overridden by the `return_image_mask` parameter in `preprocess`.

input_size_patches (`int`, *kwargs*, *optional*, defaults to `14`) : Number of patches in the image in height and width direction. 14x14 = 196 total patches. Can be overridden by the `input_size_patches` parameter in `preprocess`.

total_mask_patches (`int`, *kwargs*, *optional*, defaults to `75`) : Total number of patches that should be masked. Can be overridden by the `total_mask_patches` parameter in `preprocess`.

mask_group_min_patches (`int`, *kwargs*, *optional*, defaults to `16`) : Minimum number of patches that should be masked. Can be overridden by the `mask_group_min_patches` parameter in `preprocess`.

mask_group_max_patches (`int`, *kwargs*, *optional*) : Maximum number of patches that should be masked. Can be overridden by the `mask_group_max_patches` parameter in `preprocess`.

mask_group_min_aspect_ratio (`float`, *kwargs*, *optional*, defaults to `0.3`) : Minimum aspect ratio of the mask window. Can be overridden by the `mask_group_min_aspect_ratio` parameter in `preprocess`.

mask_group_max_aspect_ratio (`float`, *kwargs*, *optional*) : Maximum aspect ratio of the mask window. Can be overridden by the `mask_group_max_aspect_ratio` parameter in `preprocess`.

return_codebook_pixels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to return the codebook pixel values.

codebook_do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the input for codebook to a certain. Can be overridden by the `codebook_do_resize` parameter in `preprocess`. `codebook_size`.

codebook_size (`dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" : 224, "width": 224}`): Resize the input for codebook to the given size. Can be overridden by the `codebook_size` parameter in `preprocess`.

codebook_resample (`PILImageResampling`, *kwargs*, *optional*, defaults to `PILImageResampling.LANCZOS`) : Resampling filter to use if resizing the codebook image. With torchvision < 0.27, LANCZOS is not supported for torch Tensors and BICUBIC is used as the closest alternative. Can be overridden by the `codebook_resample` parameter in `preprocess`.

codebook_do_center_crop (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to crop the input for codebook at the center. If the input size is smaller than `codebook_crop_size` along any edge, the image is padded with 0's and then center cropped. Can be overridden by the `codebook_do_center_crop` parameter in `preprocess`.

codebook_crop_size (`dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" : 224, "width": 224}`): Desired output size for codebook input when applying center-cropping. Can be overridden by the `codebook_crop_size` parameter in `preprocess`.

codebook_do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the input for codebook by the specified scale `codebook_rescale_factor`. Can be overridden by the `codebook_do_rescale` parameter in `preprocess`.

codebook_rescale_factor (`int`, *kwargs* or `float`, *optional*, defaults to `1/255`) : Defines the scale factor to use if rescaling the codebook image. Can be overridden by the `codebook_rescale_factor` parameter in `preprocess`.

codebook_do_map_pixels (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to map the pixel values of the codebook input to (1 - 2e)x + e. Can be overridden by the `codebook_do_map_pixels` parameter in `preprocess`.

codebook_do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether or not to normalize the input for codebook with `codebook_image_mean` and `codebook_image_std`. Can be overridden by the `codebook_do_normalize` parameter in `preprocess`.

codebook_image_mean (`Optional[Union[float, *kwargs*, Iterable[float]]]`, *optional*, defaults to `[0, 0, 0]`) : The sequence of means for each channel, to be used when normalizing images for codebook. Can be overridden by the `codebook_image_mean` parameter in `preprocess`.

codebook_image_std (`Optional[Union[float, *kwargs*, Iterable[float]]]`, *optional*, defaults to `[0.5, 0.5, 0.5]`) : The sequence of standard deviations for each channel, to be used when normalizing images for codebook. Can be overridden by the `codebook_image_std` parameter in `preprocess`.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## FlavaImageProcessorPil[[transformers.FlavaImageProcessorPil]]

#### transformers.FlavaImageProcessorPil[[transformers.FlavaImageProcessorPil]]

```python
transformers.FlavaImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/image_processing_pil_flava.py#L227)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 224, 'width': 224}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 224, 'width': 224}`): Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.48145466, 0.4578275, 0.40821073]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.26862954, 0.26130258, 0.27577711]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

return_image_mask (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to return the image mask. Can be overridden by the `return_image_mask` parameter in `preprocess`.

input_size_patches (`int`, *kwargs*, *optional*, defaults to `14`) : Number of patches in the image in height and width direction. 14x14 = 196 total patches. Can be overridden by the `input_size_patches` parameter in `preprocess`.

total_mask_patches (`int`, *kwargs*, *optional*, defaults to `75`) : Total number of patches that should be masked. Can be overridden by the `total_mask_patches` parameter in `preprocess`.

mask_group_min_patches (`int`, *kwargs*, *optional*, defaults to `16`) : Minimum number of patches that should be masked. Can be overridden by the `mask_group_min_patches` parameter in `preprocess`.

mask_group_max_patches (`int`, *kwargs*, *optional*) : Maximum number of patches that should be masked. Can be overridden by the `mask_group_max_patches` parameter in `preprocess`.

mask_group_min_aspect_ratio (`float`, *kwargs*, *optional*, defaults to `0.3`) : Minimum aspect ratio of the mask window. Can be overridden by the `mask_group_min_aspect_ratio` parameter in `preprocess`.

mask_group_max_aspect_ratio (`float`, *kwargs*, *optional*) : Maximum aspect ratio of the mask window. Can be overridden by the `mask_group_max_aspect_ratio` parameter in `preprocess`.

return_codebook_pixels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to return the codebook pixel values.

codebook_do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the input for codebook to a certain. Can be overridden by the `codebook_do_resize` parameter in `preprocess`. `codebook_size`.

codebook_size (`dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" : 224, "width": 224}`): Resize the input for codebook to the given size. Can be overridden by the `codebook_size` parameter in `preprocess`.

codebook_resample (`PILImageResampling`, *kwargs*, *optional*, defaults to `PILImageResampling.LANCZOS`) : Resampling filter to use if resizing the codebook image. With torchvision < 0.27, LANCZOS is not supported for torch Tensors and BICUBIC is used as the closest alternative. Can be overridden by the `codebook_resample` parameter in `preprocess`.

codebook_do_center_crop (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to crop the input for codebook at the center. If the input size is smaller than `codebook_crop_size` along any edge, the image is padded with 0's and then center cropped. Can be overridden by the `codebook_do_center_crop` parameter in `preprocess`.

codebook_crop_size (`dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" : 224, "width": 224}`): Desired output size for codebook input when applying center-cropping. Can be overridden by the `codebook_crop_size` parameter in `preprocess`.

codebook_do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the input for codebook by the specified scale `codebook_rescale_factor`. Can be overridden by the `codebook_do_rescale` parameter in `preprocess`.

codebook_rescale_factor (`int`, *kwargs* or `float`, *optional*, defaults to `1/255`) : Defines the scale factor to use if rescaling the codebook image. Can be overridden by the `codebook_rescale_factor` parameter in `preprocess`.

codebook_do_map_pixels (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to map the pixel values of the codebook input to (1 - 2e)x + e. Can be overridden by the `codebook_do_map_pixels` parameter in `preprocess`.

codebook_do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether or not to normalize the input for codebook with `codebook_image_mean` and `codebook_image_std`. Can be overridden by the `codebook_do_normalize` parameter in `preprocess`.

codebook_image_mean (`Optional[Union[float, *kwargs*, Iterable[float]]]`, *optional*, defaults to `[0, 0, 0]`) : The sequence of means for each channel, to be used when normalizing images for codebook. Can be overridden by the `codebook_image_mean` parameter in `preprocess`.

codebook_image_std (`Optional[Union[float, *kwargs*, Iterable[float]]]`, *optional*, defaults to `[0.5, 0.5, 0.5]`) : The sequence of standard deviations for each channel, to be used when normalizing images for codebook. Can be overridden by the `codebook_image_std` parameter in `preprocess`.

Constructs a FlavaImageProcessor image processor.

#### preprocess[[transformers.FlavaImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/image_processing_pil_flava.py#L264)

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

return_image_mask (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to return the image mask. Can be overridden by the `return_image_mask` parameter in `preprocess`.

input_size_patches (`int`, *kwargs*, *optional*, defaults to `14`) : Number of patches in the image in height and width direction. 14x14 = 196 total patches. Can be overridden by the `input_size_patches` parameter in `preprocess`.

total_mask_patches (`int`, *kwargs*, *optional*, defaults to `75`) : Total number of patches that should be masked. Can be overridden by the `total_mask_patches` parameter in `preprocess`.

mask_group_min_patches (`int`, *kwargs*, *optional*, defaults to `16`) : Minimum number of patches that should be masked. Can be overridden by the `mask_group_min_patches` parameter in `preprocess`.

mask_group_max_patches (`int`, *kwargs*, *optional*) : Maximum number of patches that should be masked. Can be overridden by the `mask_group_max_patches` parameter in `preprocess`.

mask_group_min_aspect_ratio (`float`, *kwargs*, *optional*, defaults to `0.3`) : Minimum aspect ratio of the mask window. Can be overridden by the `mask_group_min_aspect_ratio` parameter in `preprocess`.

mask_group_max_aspect_ratio (`float`, *kwargs*, *optional*) : Maximum aspect ratio of the mask window. Can be overridden by the `mask_group_max_aspect_ratio` parameter in `preprocess`.

return_codebook_pixels (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to return the codebook pixel values.

codebook_do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the input for codebook to a certain. Can be overridden by the `codebook_do_resize` parameter in `preprocess`. `codebook_size`.

codebook_size (`dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" : 224, "width": 224}`): Resize the input for codebook to the given size. Can be overridden by the `codebook_size` parameter in `preprocess`.

codebook_resample (`PILImageResampling`, *kwargs*, *optional*, defaults to `PILImageResampling.LANCZOS`) : Resampling filter to use if resizing the codebook image. With torchvision < 0.27, LANCZOS is not supported for torch Tensors and BICUBIC is used as the closest alternative. Can be overridden by the `codebook_resample` parameter in `preprocess`.

codebook_do_center_crop (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to crop the input for codebook at the center. If the input size is smaller than `codebook_crop_size` along any edge, the image is padded with 0's and then center cropped. Can be overridden by the `codebook_do_center_crop` parameter in `preprocess`.

codebook_crop_size (`dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" : 224, "width": 224}`): Desired output size for codebook input when applying center-cropping. Can be overridden by the `codebook_crop_size` parameter in `preprocess`.

codebook_do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the input for codebook by the specified scale `codebook_rescale_factor`. Can be overridden by the `codebook_do_rescale` parameter in `preprocess`.

codebook_rescale_factor (`int`, *kwargs* or `float`, *optional*, defaults to `1/255`) : Defines the scale factor to use if rescaling the codebook image. Can be overridden by the `codebook_rescale_factor` parameter in `preprocess`.

codebook_do_map_pixels (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to map the pixel values of the codebook input to (1 - 2e)x + e. Can be overridden by the `codebook_do_map_pixels` parameter in `preprocess`.

codebook_do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether or not to normalize the input for codebook with `codebook_image_mean` and `codebook_image_std`. Can be overridden by the `codebook_do_normalize` parameter in `preprocess`.

codebook_image_mean (`Optional[Union[float, *kwargs*, Iterable[float]]]`, *optional*, defaults to `[0, 0, 0]`) : The sequence of means for each channel, to be used when normalizing images for codebook. Can be overridden by the `codebook_image_mean` parameter in `preprocess`.

codebook_image_std (`Optional[Union[float, *kwargs*, Iterable[float]]]`, *optional*, defaults to `[0.5, 0.5, 0.5]`) : The sequence of standard deviations for each channel, to be used when normalizing images for codebook. Can be overridden by the `codebook_image_std` parameter in `preprocess`.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## FlavaForPreTraining[[transformers.FlavaForPreTraining]]

#### transformers.FlavaForPreTraining[[transformers.FlavaForPreTraining]]

```python
transformers.FlavaForPreTraining(config: FlavaConfig, image_codebook: typing.Optional[torch.nn.Module] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L1502)

**Parameters:**

config ([FlavaConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

image_codebook (`torch.nn.Module`, *optional*) : If passed, the image codebook will be set to this. Otherwise, it will be initialized using the image_codebook_config defined in the config first as the first parameter.

The FLAVA model for pretraining which outputs losses, embeddings, logits and transformer outputs.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.FlavaForPreTraining.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_ids_masked: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, codebook_pixel_values: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, bool_masked_pos: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, image_attention_mask: typing.Optional[torch.Tensor] = None, skip_unmasked_multimodal_encoder: bool | None = None, mlm_labels: typing.Optional[torch.Tensor] = None, mim_labels: typing.Optional[torch.Tensor] = None, itm_labels: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool = True, return_dict: bool | None = None, return_loss: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L1551)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, text_seq_len)`) : Indices of input sequence tokens in the vocabulary. Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details. [What are input IDs?](../glossary#input-ids)

input_ids_masked (`torch.LongTensor` of shape `(batch_size, text_seq_len)`) : Indices of input sequence tokens in the vocabulary. These ones are the masked version of the original task to be used with MLM. Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer) along with `DataCollatorForMaskedLanguageModeling`. See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details. [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [FlavaImageProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageProcessor). See `FlavaImageProcessor.__call__()` for details ([FlavaProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaProcessor) uses [FlavaImageProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageProcessor) for processing images).

codebook_pixel_values (`torch.FloatTensor` of shape `(batch_size, num_image_patches, patch_size, patch_size, 3)`, *optional*) : Pixel values for image patches that are used to compute the image codebook labels for masked image modeling.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, text_seq_len)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`: - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token. [What are token type IDs?](../glossary#token-type-ids)

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, image_num_patches)`) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

image_attention_mask (`torch.FloatTensor` of shape `(batch_size, image_num_patches)`, *optional*) : Mask to avoid performing attention on padding token indices specifically for images. Mask values selected in `[0, 1]`: - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**. [What are attention masks?](../glossary#attention-mask)

skip_unmasked_multimodal_encoder (`*bool*`, *optional*) : Skip any calculations for multimodal encoder for unmasked inputs. FLAVA pretraining doesn't need unmasked multimodal embeddings or outputs as of now.

mlm_labels (`torch.LongTensor` of shape `(batch_size, text_seq_len)`, *optional*) : Labels for computing the left-to-right language and multimodal masked modeling loss (next word prediction). Indices should be in `[-100, 0, ..., text_config.vocab_size - 1]` (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., text_config.vocab_size - 1]`.

mim_labels (`torch.LongTensor` of shape `(batch_size, image_num_patches)`, *optional*) : Labels for computing the image and multimodal masked modeling loss. Indices should be in `[-100, 0, ..., image_config.vocab_size - 1]`. Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., image_config.vocab_size - 1]`. If not passed, they are generated automatically using the image codebook assigned to the model. By default, it uses [FlavaImageCodebook](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageCodebook). See [FlavaImageCodebook](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageCodebook) to understand how to generate mim_labels.

itm_labels (`torch.LongTensor` of shape `(batch_size, 1)`, *optional*) : Labels for computing the image-text matching loss. 0 means the pairs don't match and 1 means they match. The pairs with 0 will be skipped for calculation of MMM and global contrastive losses as well.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*, defaults to `True`) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

return_loss (`bool`, *optional*, default to None) : Whether to return calculated loss or not.

**Returns:** `FlavaForPreTrainingOutput` or `tuple(torch.FloatTensor)`

A `FlavaForPreTrainingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FlavaConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaConfig)) and inputs.

The [FlavaForPreTraining](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaForPreTraining) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor`, *optional*, returned when `return_loss` is True) -- Total loss calculated for this model.
- **loss_info** (`~models.flava.modeling_flava.FlavaLosses`, *optional*) -- Detailed info for FLAVA Pretraining losses. Check `FlavaLosses` class description for the information on
  the keys.
- **image_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when `pixel_values` are present) -- The image embeddings which are basically the pooled output of [FlavaImageModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageModel).
- **image_output** (`BaseModelOutputWithPooling`, *optional*, returned when `pixel_values` are present) -- The output of the [FlavaImageModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageModel).
- **text_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when `input_ids` are present) -- The text embeddings which are basically the pooled output of [FlavaTextModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextModel).
- **text_output** (`BaseModelOutputWithPooling`, *optional*, returned when `input_ids` are present) -- The output of the [FlavaTextModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextModel).
- **multimodal_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when `input_ids` and `pixel_values` are present and `skip_unmasked_multimodal_encoder` is `None` or `False`) -- The multimodal embeddings which are basically the pooled output of [FlavaTextModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextModel).
- **multimodal_output** (`BaseModelOutputWithPooling`, returned when `input_ids` and `pixel_values` are present and `skip_unmasked_multimodal_encoder` is `None` or `False`) -- The output of the [FlavaMultimodalModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaMultimodalModel).
- **image_masked_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when `pixel_values` are present) -- The image embeddings which are basically the pooled output of [FlavaImageModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageModel). Uses `bool_masked_pos`
  to create masked images.
- **image_masked_output** (`BaseModelOutputWithPooling`, *optional*, returned when `pixel_values` are present) -- The output of the [FlavaImageModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageModel). Uses `bool_masked_pos` to create masked images.
- **text_masked_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when `input_ids_masked` are present) -- The text embeddings which are basically the pooled output of [FlavaTextModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextModel).
- **text_masked_output** (`BaseModelOutputWithPooling`, *optional*, returned when `input_ids_masked` are present) -- The output of the [FlavaTextModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextModel).
- **multimodal_masked_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when `input_ids` and `pixel_values` are present) -- The multimodal embeddings which are basically the pooled output of [FlavaTextModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextModel).
- **multimodal_masked_output** (`BaseModelOutputWithPooling`, *optional*, returned when `input_ids_masked` and `pixel_values` are present) -- The output of the [FlavaMultimodalModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaMultimodalModel).
- **mim_logits** (`torch.FloatTensor` of shape `(batch_size, num_image_patches, image_vocab_size)` or of shape `(total_masked_patches, image_vocab_size)` , *optional*, returned when `pixel_values` are present and `input_ids_masked` are not) -- The logits for MIM unimodal loss. Uses `bool_masked_pos` to get masked patches. The flattened output is
  returned when `bool_masked_pos` has some of the patches masked.
- **mlm_logits** (`torch.FloatTensor` of shape `(batch_size, text_seq_length, text_vocab_size)` or of shape `(total_masked_seq_length, text_vocab_size)`, *optional*, returned when `input_ids_masked` are present and `pixel_values` are not) -- The logits for MLM unimodal loss. The flattened output is returned when `input_ids_masked` has some of
  the tokens masked.
- **itm_logits** (`torch.FloatTensor` of shape `(batch_size, 2)`, *optional*, returned when `input_ids_masked` and `pixel_values` are present) -- The logits for ITM loss. Note that ITM loss is calculated on masked pairs in FLAVA.
- **contrastive_logits_per_image** (`torch.FloatTensor` of shape `(image_batch_size, text_batch_size)`) -- The scaled dot product scores between `image_embeddings` and `text_embeddings` but passed through FLAVA's
  `image_projection` and `text_projection` layers respectively. This represents the image-text similarity
  scores. This is calculated on unmasked images and texts.
- **contrastive_logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, image_batch_size)`) -- The scaled dot product scores between `text_embeddings` and `image_embeddings` but passed through FLAVA's
  `text_projection` and `image_projection` layers respectively. This is calculated on unmasked images and
  texts.
- **mmm_image_logits** (`torch.FloatTensor` of shape `(batch_size, num_image_patches, image_vocab_size)` or of shape`(total_masked_patches, image_vocab_size)`, *optional*, returned when `pixel_values` and `input_ids_masked` are present) -- The logits for MMM image multimodal loss. Uses `bool_masked_pos` to get masked patches. The flattened
  output is returned when `bool_masked_pos` has some of the patches masked.
- **mmm_text_logits** (`torch.FloatTensor` of shape `(batch_size, text_seq_length, text_vocab_size)` or of shape `(total_masked_seq_length, text_vocab_size)`, *optional*, returned when `pixel_values` and `input_ids_masked` are present) -- The logits for MMM text multimodal loss. The flattened output is returned when `input_ids_masked` has
  some of the tokens masked.

Examples:
```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import FlavaForPreTraining, AutoProcessor

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> model = FlavaForPreTraining.from_pretrained("facebook/flava-full")
>>> processor = AutoProcessor.from_pretrained("facebook/flava-full")

>>> text = ["a photo of a cat"]

>>> inputs = processor(
...     images=[image],
...     text=text,
...     return_masks=True,
...     return_codebook_pixels=True,
...     padding=True,
...     max_length=77,
...     return_tensors="pt",
... )

>>> output = model(**inputs)
```

## FlavaModel[[transformers.FlavaModel]]

#### transformers.FlavaModel[[transformers.FlavaModel]]

```python
transformers.FlavaModel(config: FlavaConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L935)

**Parameters:**

config ([FlavaConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Flava Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.FlavaModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, bool_masked_pos: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, image_attention_mask: typing.Optional[torch.Tensor] = None, skip_multimodal_encoder: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool = True, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L1078)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, image_num_patches + text_seq_len)`) : Indices of input sequence tokens in the vocabulary. Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details. [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [FlavaImageProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageProcessor). See `FlavaImageProcessor.__call__()` for details ([FlavaProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaProcessor) uses [FlavaImageProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageProcessor) for processing images).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, image_num_patches + text_seq_len)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`: - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token. [What are token type IDs?](../glossary#token-type-ids)

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, image_num_patches)`) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

image_attention_mask (`torch.Tensor` of shape `(batch_size, image_num_patches)`, *optional*) : Mask to avoid performing attention on padding pixel values for image inputs. Mask values selected in `[0, 1]`: - 1 for pixel values that are real (i.e., **not masked**), - 0 for pixel values that are padding (i.e., **masked**).

skip_multimodal_encoder (`*bool*`, *optional*) : Skip any calculations for multimodal encoder. Useful if multimodal encoding is not going to be used.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*, defaults to `True`) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `FlavaModelOutput` or `tuple(torch.FloatTensor)`

A `FlavaModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FlavaConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaConfig)) and inputs.

The [FlavaModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **image_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when `pixel_values` are present) -- The image embeddings which are basically the pooled output of [FlavaImageModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageModel).
- **image_output** (`BaseModelOutputWithPooling`, *optional*, returned when `pixel_values` are present) -- The output of the [FlavaImageModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageModel).
- **text_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when `input_ids` are present) -- The text embeddings which are basically the pooled output of [FlavaTextModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextModel).
- **text_output** (`BaseModelOutputWithPooling`, *optional*, returned when `input_ids` are present) -- The output of the [FlavaTextModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextModel).
- **multimodal_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when `input_ids` and `pixel_values` are present and `skip_multimodal_encoder` is `None` or `False`) -- The multimodal embeddings which are basically the pooled output of [FlavaTextModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextModel).
- **multimodal_output** (`BaseModelOutputWithPooling`, returned when `input_ids` and `pixel_values` are present and `skip_multimodal_encoder` is `None` or `False`) -- The output of the [FlavaMultimodalModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaMultimodalModel).

Examples:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, FlavaModel

>>> model = FlavaModel.from_pretrained("facebook/flava-full")
>>> processor = AutoProcessor.from_pretrained("facebook/flava-full")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(text=["a photo of a cat"], images=image, return_tensors="pt", padding=True)

>>> outputs = model(**inputs)

>>> image_embeddings = outputs.image_embeddings
>>> text_embeddings = outputs.text_embeddings
>>> multimodal_embeddings = outputs.multimodal_embeddings

>>> outputs.image_embeddings.shape
torch.Size([1, 197, 768])

>>> text_embeddings.shape
torch.Size([1, 7, 768])

>>> multimodal_embeddings.shape
torch.Size([1, 205, 768])
```

#### get_text_features[[transformers.FlavaModel.get_text_features]]

```python
get_text_features(input_ids: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L981)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, text_seq_length)`) : Indices of input sequence tokens in the vocabulary. Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details. [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, text_seq_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`: - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token. [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FlavaConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaConfig)) and inputs.

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

Examples:

```python
>>> import torch
>>> from transformers import AutoProcessor, FlavaModel

>>> model = FlavaModel.from_pretrained("{0}")
>>> processor = AutoProcessor.from_pretrained("{0}")

>>> inputs = processor(
...     text=["a photo of a cat", "a photo of a dog"], max_length=77, padding="max_length", return_tensors="pt"
... )
>>> with torch.inference_mode():
...     text_features = model.get_text_features(**inputs)
```

#### get_image_features[[transformers.FlavaModel.get_image_features]]

```python
get_image_features(pixel_values: Tensor, bool_masked_pos: typing.Optional[torch.BoolTensor] = None, interpolate_pos_encoding: bool | None = None, attention_mask: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L1032)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [FlavaImageProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageProcessor). See `FlavaImageProcessor.__call__()` for details ([FlavaProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaProcessor) uses [FlavaImageProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageProcessor) for processing images).

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, image_num_patches)`) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).

interpolate_pos_encoding (`bool`, *optional*) : Whether to interpolate the pre-trained position encodings.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FlavaConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaConfig)) and inputs.

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

Examples:

```python
>>> import torch
>>> from transformers import AutoProcessor, FlavaModel
>>> from transformers.image_utils import load_image

>>> model = FlavaModel.from_pretrained("{0}")
>>> processor = AutoProcessor.from_pretrained("{0}")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = load_image(url)

>>> inputs = processor(images=image, return_tensors="pt")

>>> with torch.inference_mode():
...     image_features = model.get_image_features(**inputs)
```

## FlavaImageCodebook[[transformers.FlavaImageCodebook]]

#### transformers.FlavaImageCodebook[[transformers.FlavaImageCodebook]]

```python
transformers.FlavaImageCodebook(config: FlavaImageCodebookConfig, **kwargs: typing.Any)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L1289)

**Parameters:**

config ([FlavaImageCodebookConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageCodebookConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The FLAVA's image codebook model inspired from DALL-E's original encoder. Outputs raw hidden states and can be used
to generate image tokens for an image based on DALL-E's vocab. Used to generate labels for MIM. Use
`get_codebook_indices` to get image tokens for an image.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.FlavaImageCodebook.forward]]

```python
forward(pixel_values: FloatTensor, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L1374)

#### get_codebook_indices[[transformers.FlavaImageCodebook.get_codebook_indices]]

```python
get_codebook_indices(pixel_values: Tensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L1340)

#### get_codebook_probs[[transformers.FlavaImageCodebook.get_codebook_probs]]

```python
get_codebook_probs(pixel_values: Tensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L1370)

## FlavaTextModel[[transformers.FlavaTextModel]]

#### transformers.FlavaTextModel[[transformers.FlavaTextModel]]

```python
transformers.FlavaTextModel(config: FlavaTextConfig, add_pooling_layer: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L759)

**Parameters:**

config ([FlavaTextConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether to add a pooling layer

The bare Flava Text Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.FlavaTextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L787)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, text_seq_length)`) : Indices of input sequence tokens in the vocabulary. Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details. [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.LongTensor` of shape `(batch_size, text_seq_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`: - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token. [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FlavaConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaConfig)) and inputs.

The [FlavaTextModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaTextModel) forward method, overrides the `__call__` special method.

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

## FlavaImageModel[[transformers.FlavaImageModel]]

#### transformers.FlavaImageModel[[transformers.FlavaImageModel]]

```python
transformers.FlavaImageModel(config: FlavaImageConfig, add_pooling_layer: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L677)

**Parameters:**

config ([FlavaImageConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether to add a pooling layer

The bare Flava Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.FlavaImageModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.Tensor] = None, bool_masked_pos: typing.Optional[torch.BoolTensor] = None, interpolate_pos_encoding: bool | None = None, attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L707)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [FlavaImageProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageProcessor). See `FlavaImageProcessor.__call__()` for details ([FlavaProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaProcessor) uses [FlavaImageProcessor](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageProcessor) for processing images).

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, image_num_patches)`) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).

interpolate_pos_encoding (`bool`, *optional*) : Whether to interpolate the pre-trained position encodings.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FlavaConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaConfig)) and inputs.

The [FlavaImageModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaImageModel) forward method, overrides the `__call__` special method.

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

## FlavaMultimodalModel[[transformers.FlavaMultimodalModel]]

#### transformers.FlavaMultimodalModel[[transformers.FlavaMultimodalModel]]

```python
transformers.FlavaMultimodalModel(config: FlavaMultimodalConfig, add_pooling_layer = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L855)

**Parameters:**

config ([FlavaMultimodalConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaMultimodalConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether to add a pooling layer

The bare Flava Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.FlavaMultimodalModel.forward]]

```python
forward(hidden_states: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/flava/modeling_flava.py#L879)

**Parameters:**

hidden_states (`torch.FloatTensor` of shape `(batch_size, image_num_patches + text_seq_len, hidden_size)`) : The concatenated hidden states of unimodal encoders.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([FlavaConfig](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaConfig)) and inputs.

The [FlavaMultimodalModel](/docs/transformers/v5.15.1/en/model_doc/flava#transformers.FlavaMultimodalModel) forward method, overrides the `__call__` special method.

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
