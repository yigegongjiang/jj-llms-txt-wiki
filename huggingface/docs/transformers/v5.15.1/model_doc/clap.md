# CLAP

[CLAP (Contrastive Language-Audio Pretraining)](https://huggingface.co/papers/2211.06687) is a multimodal model that combines audio data with natural language descriptions through contrastive learning.

It incorporates feature fusion and keyword-to-caption augmentation to process variable-length audio inputs and to improve performance. CLAP doesn't require task-specific training data and can learn meaningful audio representations through natural language.

You can find all the original CLAP checkpoints under the [CLAP](https://huggingface.co/collections/laion/clap-contrastive-language-audio-pretraining-65415c0b18373b607262a490) collection.

> [!TIP]
> This model was contributed by [ybelkada](https://huggingface.co/ybelkada) and [ArthurZ](https://huggingface.co/ArthurZ).
>
> Click on the CLAP models in the right sidebar for more examples of how to apply CLAP to different audio retrieval and classification tasks.

The example below demonstrates how to extract text embeddings with the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel) class.

```python
import torch

from transformers import AutoModel, AutoTokenizer

model = AutoModel.from_pretrained("laion/clap-htsat-unfused", device_map="auto")
tokenizer = AutoTokenizer.from_pretrained("laion/clap-htsat-unfused")

texts = ["the sound of a cat", "the sound of a dog", "music playing"]

inputs = tokenizer(texts, padding=True, return_tensors="pt").to(model.device)

with torch.no_grad():
    text_features = model.get_text_features(**inputs)

print(f"Text embeddings shape: {text_features.shape}")
print(f"Text embeddings: {text_features}")
```

## ClapConfig[[transformers.ClapConfig]]

#### transformers.ClapConfig[[transformers.ClapConfig]]

```python
transformers.ClapConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, audio_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, logit_scale_init_value: float = 14.285714285714285, projection_dim: int = 512, projection_hidden_act: str = 'relu', initializer_factor: float = 1.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/configuration_clap.py#L145)

**Parameters:**

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

audio_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the audio backbone.

logit_scale_init_value (`float`, *optional*, defaults to `14.285714285714285`) : The initial value of the *logit_scale* parameter.

projection_dim (`int`, *optional*, defaults to `512`) : Dimensionality of text and vision projection layers.

projection_hidden_act (`str`, *optional*, defaults to `relu`) : The activation function used by the multimodal projector.

initializer_factor (`float`, *optional*, defaults to `1.0`) : A factor for initializing all weight matrices (should be kept to 1, used internally for initialization testing).

This is the configuration class to store the configuration of a ClapModel. It is used to instantiate a Clap
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [laion/clap-htsat-fused](https://huggingface.co/laion/clap-htsat-fused)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ClapConfig, ClapModel

>>> # Initializing a ClapConfig with laion-ai/base style configuration
>>> configuration = ClapConfig()

>>> # Initializing a ClapModel (with random weights) from the laion-ai/base style configuration
>>> model = ClapModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a ClapConfig from a ClapTextConfig and a ClapAudioConfig
>>> from transformers import ClapTextConfig, ClapAudioConfig

>>> # Initializing a ClapText and ClapAudioConfig configuration
>>> config_text = ClapTextConfig()
>>> config_audio = ClapAudioConfig()

>>> config = ClapConfig(text_config=config_text, audio_config=config_audio)
```

## ClapTextConfig[[transformers.ClapTextConfig]]

#### transformers.ClapTextConfig[[transformers.ClapTextConfig]]

```python
transformers.ClapTextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 50265, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, intermediate_size: int = 3072, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.1, attention_probs_dropout_prob: float | int = 0.1, max_position_embeddings: int = 514, type_vocab_size: int = 1, initializer_factor: float = 1.0, layer_norm_eps: float = 1e-12, projection_dim: int = 512, pad_token_id: int | None = 1, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 2, projection_hidden_act: str = 'relu')
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/configuration_clap.py#L27)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `50265`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

max_position_embeddings (`int`, *optional*, defaults to `514`) : The maximum sequence length that this model might ever be used with.

type_vocab_size (`int`, *optional*, defaults to `1`) : The vocabulary size of the `token_type_ids`.

initializer_factor (`float`, *optional*, defaults to `1.0`) : A factor for initializing all weight matrices (should be kept to 1, used internally for initialization testing).

layer_norm_eps (`float`, *optional*, defaults to `1e-12`) : The epsilon used by the layer normalization layers.

projection_dim (`int`, *optional*, defaults to `512`) : Dimensionality of text and vision projection layers.

pad_token_id (`int`, *optional*, defaults to `1`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

projection_hidden_act (`str`, *optional*, defaults to `relu`) : The activation function used by the multimodal projector.

This is the configuration class to store the configuration of a ClapModel. It is used to instantiate a Clap
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [laion/clap-htsat-fused](https://huggingface.co/laion/clap-htsat-fused)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import ClapTextConfig, ClapTextModel

>>> # Initializing a CLAP text configuration
>>> configuration = ClapTextConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = ClapTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ClapAudioConfig[[transformers.ClapAudioConfig]]

#### transformers.ClapAudioConfig[[transformers.ClapAudioConfig]]

```python
transformers.ClapAudioConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, window_size: int = 8, num_mel_bins: int = 64, spec_size: int = 256, hidden_act: str = 'gelu', patch_size: int | list[int] | tuple[int, int] = 4, patch_stride: int | list[int] | tuple[int, ...] = (4, 4), num_classes: int = 527, hidden_size: int = 768, projection_dim: int = 512, depths: list[int] | tuple[int, ...] = (2, 2, 6, 2), num_attention_heads: list[int] | tuple[int, ...] = (4, 8, 16, 32), enable_fusion: bool = False, hidden_dropout_prob: float | int = 0.1, fusion_type: str | None = None, patch_embed_input_channels: int = 1, flatten_patch_embeds: bool = True, patch_embeds_hidden_size: int = 96, enable_patch_layer_norm: bool = True, drop_path_rate: float | int = 0.0, attention_probs_dropout_prob: float | int = 0.0, qkv_bias: bool = True, mlp_ratio: float = 4.0, aff_block_r: int = 4, num_hidden_layers: int = 4, projection_hidden_act: str = 'relu', layer_norm_eps: float = 1e-05, initializer_factor: float = 1.0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/configuration_clap.py#L68)

**Parameters:**

window_size (`int`, *optional*, defaults to 8) : Image size of the spectrogram

num_mel_bins (`int`, *optional*, defaults to `64`) : Number of mel features used per input frame. Should correspond to the value used in the `AutoFeatureExtractor` class.

spec_size (`int`, *optional*, defaults to 256) : Desired input size of the spectrogram that the model supports. It can be different from the output of the `ClapFeatureExtractor`, in which case the input features will be resized. Corresponds to the `image_size` of the audio models.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `4`) : The size (resolution) of each patch.

patch_stride (`list`, *optional*, defaults to `[4, 4]`) : Patch stride for the audio spectrogram

num_classes (`int`, *optional*, defaults to 527) : Number of classes used for the head training

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

projection_dim (`int`, *optional*, defaults to `512`) : Dimensionality of text and vision projection layers.

depths (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(2, 2, 6, 2)`) : Depth of each layer in the Transformer.

num_attention_heads (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(4, 8, 16, 32)`) : Number of attention heads for each attention layer in the Transformer decoder.

enable_fusion (`bool`, *optional*, defaults to `False`) : Whether or not to enable patch fusion. This is the main contribution of the authors, and should give the best results.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

fusion_type (`[type]`, *optional*) : Fusion type used for the patch fusion.

patch_embed_input_channels (`int`, *optional*, defaults to 1) : Number of channels used for the input spectrogram

flatten_patch_embeds (`bool`, *optional*, defaults to `True`) : Whether or not to flatten the patch embeddings

patch_embeds_hidden_size (`int`, *optional*, defaults to 96) : Hidden size of the patch embeddings. It is used as the number of output channels.

enable_patch_layer_norm (`bool`, *optional*, defaults to `True`) : Whether or not to enable layer normalization for the patch embeddings

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.0`) : Drop path rate for the patch fusion.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

mlp_ratio (`float`, *optional*, defaults to `4.0`) : Ratio of the MLP hidden dim to the embedding dim.

aff_block_r (`int`, *optional*, defaults to 4) : downsize_ratio used in the AudioFF block

num_hidden_layers (`int`, *optional*, defaults to `4`) : Number of hidden layers in the Transformer decoder.

projection_hidden_act (`str`, *optional*, defaults to `relu`) : The activation function used by the multimodal projector.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

initializer_factor (`float`, *optional*, defaults to `1.0`) : A factor for initializing all weight matrices (should be kept to 1, used internally for initialization testing).

This is the configuration class to store the configuration of a ClapModel. It is used to instantiate a Clap
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [laion/clap-htsat-fused](https://huggingface.co/laion/clap-htsat-fused)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ClapAudioConfig, ClapAudioModel

>>> # Initializing a ClapAudioConfig with laion/clap-htsat-fused style configuration
>>> configuration = ClapAudioConfig()

>>> # Initializing a ClapAudioModel (with random weights) from the laion/clap-htsat-fused style configuration
>>> model = ClapAudioModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ClapFeatureExtractor[[transformers.ClapFeatureExtractor]]

#### transformers.ClapFeatureExtractor[[transformers.ClapFeatureExtractor]]

```python
transformers.ClapFeatureExtractor(feature_size = 64, sampling_rate = 48000, hop_length = 480, max_length_s = 10, fft_window_size = 1024, padding_value = 0.0, return_attention_mask = False, frequency_min: float = 0, frequency_max: float = 14000, top_db: int | None = None, truncation: str = 'fusion', padding: str = 'repeatpad', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/feature_extraction_clap.py#L36)

**Parameters:**

feature_size (`int`, *optional*, defaults to 64) : The feature dimension of the extracted Mel spectrograms. This corresponds to the number of mel filters (`n_mels`).

sampling_rate (`int`, *optional*, defaults to 48000) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz). This only serves to warn users if the audio fed to the feature extractor does not have the same sampling rate.

hop_length (`int`,*optional*, defaults to 480) : Length of the overlapping windows for the STFT used to obtain the Mel Spectrogram. The audio will be split in smaller `frames` with a step of `hop_length` between each frame.

max_length_s (`int`, *optional*, defaults to 10) : The maximum input length of the model in seconds. This is used to pad the audio.

fft_window_size (`int`, *optional*, defaults to 1024) : Size of the window (in samples) on which the Fourier transform is applied. This controls the frequency resolution of the spectrogram. 400 means that the fourier transform is computed on windows of 400 samples.

padding_value (`float`, *optional*, defaults to 0.0) : Padding value used to pad the audio. Should correspond to silences.

return_attention_mask (`bool`, *optional*, defaults to `False`) : Whether or not the model should return the attention masks corresponding to the input.

frequency_min (`float`, *optional*, defaults to 0) : The lowest frequency of interest. The STFT will not be computed for values below this.

frequency_max (`float`, *optional*, defaults to 14000) : The highest frequency of interest. The STFT will not be computed for values above this.

top_db (`float`, *optional*) : The highest decibel value used to convert the mel spectrogram to the log scale. For more details see the `audio_utils.power_to_db` function

truncation (`str`, *optional*, defaults to `"fusion"`) : Truncation pattern for long audio inputs. Two patterns are available: - `fusion` will use `_random_mel_fusion`, which stacks 3 random crops from the mel spectrogram and a downsampled version of the entire mel spectrogram. If `config.fusion` is set to True, shorter audios also need to return 4 mels, which will just be a copy of the original mel obtained from the padded audio. - `rand_trunc` will select a random crop of the mel spectrogram.

padding (`str`, *optional*, defaults to `"repeatpad"`) : Padding pattern for shorter audio inputs. Three patterns were originally implemented: - `repeatpad`: the audio is repeated, and then padded to fit the `max_length`. - `repeat`: the audio is repeated and then cut to fit the `max_length` - `pad`: the audio is padded.

Constructs a CLAP feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

This class extracts mel-filter bank features from raw speech using a custom numpy implementation of the *Short Time
Fourier Transform* (STFT) which should match pytorch's `torch.stft` equivalent.

#### to_dict[[transformers.ClapFeatureExtractor.to_dict]]

```python
to_dict()
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/feature_extraction_clap.py#L141)

**Returns:** `dict[str, Any]`

Dictionary of all the attributes that make up this configuration instance, except for the
mel filter banks, which do not need to be saved or printed as they are too long.

Serializes this instance to a Python dictionary.

## ClapProcessor[[transformers.ClapProcessor]]

#### transformers.ClapProcessor[[transformers.ClapProcessor]]

```python
transformers.ClapProcessor(feature_extractor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/processing_clap.py#L26)

**Parameters:**

feature_extractor (`ClapFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`RobertaTokenizer`) : The tokenizer is a required input.

Constructs a ClapProcessor which wraps a feature extractor and a tokenizer into a single processor.

[ClapProcessor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapProcessor) offers all the functionalities of [ClapFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapFeatureExtractor) and [RobertaTokenizer](/docs/transformers/v5.15.1/en/model_doc/roberta#transformers.RobertaTokenizer). See the
[~ClapFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapFeatureExtractor) and [~RobertaTokenizer](/docs/transformers/v5.15.1/en/model_doc/roberta#transformers.RobertaTokenizer) for more information.

#### __call__[[transformers.ClapProcessor.__call__]]

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

## ClapModel[[transformers.ClapModel]]

#### transformers.ClapModel[[transformers.ClapModel]]

```python
transformers.ClapModel(config: ClapConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1432)

**Parameters:**

config ([ClapConfig](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Clap Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ClapModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, input_features: typing.Optional[torch.FloatTensor] = None, is_longer: typing.Optional[torch.BoolTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, return_loss: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1537)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [ClapFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapFeatureExtractor). See `ClapFeatureExtractor.__call__()` for details ([ClapProcessor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapProcessor) uses [ClapFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapFeatureExtractor) for processing audios).

is_longer (`torch.FloatTensor`, of shape `(batch_size, 1)`, *optional*) : Whether the audio clip is longer than `max_length`. If `True`, a feature fusion will be enabled to enhance the features.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

return_loss (`bool`, *optional*) : Whether or not to return the contrastive loss.

**Returns:** `ClapOutput` or `tuple(torch.FloatTensor)`

A `ClapOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClapConfig](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapConfig)) and inputs.

The [ClapModel](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Contrastive loss for audio-text similarity.
- **logits_per_audio** (`torch.FloatTensor` of shape `(audio_batch_size, text_batch_size)`) -- The scaled dot product scores between `audio_embeds` and `text_embeds`. This represents the audio-text
  similarity scores.
- **logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, audio_batch_size)`) -- The scaled dot product scores between `text_embeds` and `audio_embeds`. This represents the text-audio
  similarity scores.
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The text embeddings obtained by applying the projection layer to the pooled output of [ClapTextModel](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapTextModel).
- **audio_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The audio embeddings obtained by applying the projection layer to the pooled output of [ClapAudioModel](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapAudioModel).
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [ClapTextModel](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapTextModel).
- **audio_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [ClapAudioModel](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapAudioModel).

Examples:

```python
>>> from datasets import load_dataset
>>> from transformers import AutoProcessor, ClapModel

>>> dataset = load_dataset("hf-internal-testing/ashraq-esc50-1-dog-example")
>>> audio_sample = dataset["train"]["audio"][0]["array"]

>>> model = ClapModel.from_pretrained("laion/clap-htsat-unfused")
>>> processor = AutoProcessor.from_pretrained("laion/clap-htsat-unfused")

>>> input_text = ["Sound of a dog", "Sound of vacuum cleaner"]

>>> inputs = processor(text=input_text, audio=audio_sample, return_tensors="pt", padding=True)

>>> outputs = model(**inputs)
>>> logits_per_audio = outputs.logits_per_audio  # this is the audio-text similarity score
>>> probs = logits_per_audio.softmax(dim=-1)  # we can take the softmax to get the label probabilities
```

#### get_text_features[[transformers.ClapModel.get_text_features]]

```python
get_text_features(input_ids: Tensor, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1467)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClapConfig](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapConfig)) and inputs.

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
>>> from transformers import AutoTokenizer, ClapModel

>>> model = ClapModel.from_pretrained("laion/clap-htsat-unfused")
>>> tokenizer = AutoTokenizer.from_pretrained("laion/clap-htsat-unfused")

>>> inputs = tokenizer(["the sound of a cat", "the sound of a dog"], padding=True, return_tensors="pt")
>>> with torch.inference_mode():
...     text_features = model.get_text_features(**inputs)
```

#### get_audio_features[[transformers.ClapModel.get_audio_features]]

```python
get_audio_features(input_features: Tensor, is_longer: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1501)

**Parameters:**

input_features (`torch.Tensor` of shape `(batch_size, sequence_length, feature_dim)`) : The tensors corresponding to the input audio features. Audio features can be obtained using [ClapFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapFeatureExtractor). See `ClapFeatureExtractor.__call__()` for details ([ClapProcessor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapProcessor) uses [ClapFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapFeatureExtractor) for processing audios).

is_longer (`torch.FloatTensor`, of shape `(batch_size, 1)`, *optional*) : Whether the audio clip is longer than `max_length`. If `True`, a feature fusion will be enabled to enhance the features.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClapConfig](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapConfig)) and inputs.

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
>>> from transformers import AutoFeatureExtractor, ClapModel

>>> model = ClapModel.from_pretrained("laion/clap-htsat-unfused")
>>> feature_extractor = AutoFeatureExtractor.from_pretrained("laion/clap-htsat-unfused")
>>> random_audio = torch.rand((16_000))

>>> inputs = feature_extractor(random_audio, return_tensors="pt")
>>> with torch.inference_mode():
...     audio_features = model.get_audio_features(**inputs)
```

## ClapTextModel[[transformers.ClapTextModel]]

#### transformers.ClapTextModel[[transformers.ClapTextModel]]

```python
transformers.ClapTextModel(config, add_pooling_layer = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1346)

**Parameters:**

config ([ClapTextModel](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapTextModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

add_pooling_layer (`bool`, *optional*, defaults to `True`) : Whether to add a pooling layer

The model can behave as an encoder (with only self-attention) as well as a decoder, in which case a layer of
cross-attention is added between the self-attention layers, following the architecture described in *Attention is
all you need*_ by Ashish Vaswani, Noam Shazeer, Niki Parmar, Jakob Uszkoreit, Llion Jones, Aidan N. Gomez, Lukasz
Kaiser and Illia Polosukhin.

To behave as an decoder the model needs to be initialized with the `is_decoder` argument of the configuration set
to `True`. To be used in a Seq2Seq model, the model needs to initialized with both `is_decoder` argument and
`add_cross_attention` set to `True`; an `encoder_hidden_states` is then expected as an input to the forward pass.

.. _*Attention is all you need*: https://huggingface.co/papers/1706.03762

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ClapTextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, token_type_ids: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, inputs_embeds: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1376)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

token_type_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:  - 0 corresponds to a *sentence A* token, - 1 corresponds to a *sentence B* token.  [What are token type IDs?](../glossary#token-type-ids)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClapConfig](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapConfig)) and inputs.

The [ClapTextModel](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapTextModel) forward method, overrides the `__call__` special method.

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

## ClapTextModelWithProjection[[transformers.ClapTextModelWithProjection]]

#### transformers.ClapTextModelWithProjection[[transformers.ClapTextModelWithProjection]]

```python
transformers.ClapTextModelWithProjection(config: ClapTextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1623)

**Parameters:**

config ([ClapTextConfig](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapTextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Clap Model with a projection layer on top (a linear layer on top of the pooled output).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ClapTextModelWithProjection.forward]]

```python
forward(input_ids: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1644)

**Parameters:**

input_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

**Returns:** `ClapTextModelOutput` or `tuple(torch.FloatTensor)`

A `ClapTextModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClapConfig](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapConfig)) and inputs.

The [ClapTextModelWithProjection](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapTextModelWithProjection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim)` *optional* returned when model is initialized with `with_projection=True`) -- The text embeddings obtained by applying the projection layer to the pooler_output.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoTokenizer, ClapTextModelWithProjection

>>> model = ClapTextModelWithProjection.from_pretrained("laion/clap-htsat-unfused")
>>> tokenizer = AutoTokenizer.from_pretrained("laion/clap-htsat-unfused")

>>> inputs = tokenizer(["a sound of a cat", "a sound of a dog"], padding=True, return_tensors="pt")

>>> outputs = model(**inputs)
>>> text_embeds = outputs.text_embeds
```

## ClapAudioModel[[transformers.ClapAudioModel]]

#### transformers.ClapAudioModel[[transformers.ClapAudioModel]]

```python
transformers.ClapAudioModel(config: ClapAudioConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1282)

#### forward[[transformers.ClapAudioModel.forward]]

```python
forward(input_features: typing.Optional[torch.FloatTensor] = None, is_longer: typing.Optional[torch.BoolTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1296)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [ClapFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapFeatureExtractor). See `ClapFeatureExtractor.__call__()` for details ([ClapProcessor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapProcessor) uses [ClapFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapFeatureExtractor) for processing audios).

is_longer (`torch.FloatTensor`, of shape `(batch_size, 1)`, *optional*) : Whether the audio clip is longer than `max_length`. If `True`, a feature fusion will be enabled to enhance the features.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClapConfig](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapConfig)) and inputs.

The [ClapAudioModel](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapAudioModel) forward method, overrides the `__call__` special method.

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

Examples:

```python
>>> from datasets import load_dataset
>>> from transformers import AutoProcessor, ClapAudioModel

>>> dataset = load_dataset("hf-internal-testing/ashraq-esc50-1-dog-example")
>>> audio_sample = dataset["train"]["audio"][0]["array"]

>>> model = ClapAudioModel.from_pretrained("laion/clap-htsat-fused")
>>> processor = AutoProcessor.from_pretrained("laion/clap-htsat-fused")

>>> inputs = processor(audio=audio_sample, return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
```

## ClapAudioModelWithProjection[[transformers.ClapAudioModelWithProjection]]

#### transformers.ClapAudioModelWithProjection[[transformers.ClapAudioModelWithProjection]]

```python
transformers.ClapAudioModelWithProjection(config: ClapAudioConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1685)

**Parameters:**

config ([ClapAudioConfig](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapAudioConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Clap Model with a projection layer on top (a linear layer on top of the pooled output).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.ClapAudioModelWithProjection.forward]]

```python
forward(input_features: typing.Optional[torch.FloatTensor] = None, is_longer: typing.Optional[torch.BoolTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/clap/modeling_clap.py#L1700)

**Parameters:**

input_features (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [ClapFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapFeatureExtractor). See `ClapFeatureExtractor.__call__()` for details ([ClapProcessor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapProcessor) uses [ClapFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapFeatureExtractor) for processing audios).

is_longer (`torch.FloatTensor`, of shape `(batch_size, 1)`, *optional*) : Whether the audio clip is longer than `max_length`. If `True`, a feature fusion will be enabled to enhance the features.

**Returns:** `ClapAudioModelOutput` or `tuple(torch.FloatTensor)`

A `ClapAudioModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClapConfig](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapConfig)) and inputs.

The [ClapAudioModelWithProjection](/docs/transformers/v5.15.1/en/model_doc/clap#transformers.ClapAudioModelWithProjection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **audio_embeds** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- The Audio embeddings obtained by applying the projection layer to the pooler_output.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from datasets import load_dataset
>>> from transformers import ClapAudioModelWithProjection, ClapProcessor

>>> model = ClapAudioModelWithProjection.from_pretrained("laion/clap-htsat-fused")
>>> processor = ClapProcessor.from_pretrained("laion/clap-htsat-fused")

>>> dataset = load_dataset("hf-internal-testing/ashraq-esc50-1-dog-example")
>>> audio_sample = dataset["train"]["audio"][0]["array"]

>>> inputs = processor(audio=audio_sample, return_tensors="pt")
>>> outputs = model(**inputs)
>>> audio_embeds = outputs.audio_embeds
```
