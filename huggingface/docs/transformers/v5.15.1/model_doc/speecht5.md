# SpeechT5

## Overview

The SpeechT5 model was proposed in [SpeechT5: Unified-Modal Encoder-Decoder Pre-Training for Spoken Language Processing](https://huggingface.co/papers/2110.07205) by Junyi Ao, Rui Wang, Long Zhou, Chengyi Wang, Shuo Ren, Yu Wu, Shujie Liu, Tom Ko, Qing Li, Yu Zhang, Zhihua Wei, Yao Qian, Jinyu Li, Furu Wei.

The abstract from the paper is the following:

*Motivated by the success of T5 (Text-To-Text Transfer Transformer) in pre-trained natural language processing models, we propose a unified-modal SpeechT5 framework that explores the encoder-decoder pre-training for self-supervised speech/text representation learning. The SpeechT5 framework consists of a shared encoder-decoder network and six modal-specific (speech/text) pre/post-nets. After preprocessing the input speech/text through the pre-nets, the shared encoder-decoder network models the sequence-to-sequence transformation, and then the post-nets generate the output in the speech/text modality based on the output of the decoder. Leveraging large-scale unlabeled speech and text data, we pre-train SpeechT5 to learn a unified-modal representation, hoping to improve the modeling capability for both speech and text. To align the textual and speech information into this unified semantic space, we propose a cross-modal vector quantization approach that randomly mixes up speech/text states with latent units as the interface between encoder and decoder. Extensive evaluations show the superiority of the proposed SpeechT5 framework on a wide variety of spoken language processing tasks, including automatic speech recognition, speech synthesis, speech translation, voice conversion, speech enhancement, and speaker identification.*

This model was contributed by [Matthijs](https://huggingface.co/Matthijs). The original code can be found [here](https://github.com/microsoft/SpeechT5).

## SpeechT5Config[[transformers.SpeechT5Config]]

#### transformers.SpeechT5Config[[transformers.SpeechT5Config]]

```python
transformers.SpeechT5Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, vocab_size: int = 81, hidden_size: int = 768, encoder_layers: int = 12, encoder_attention_heads: int = 12, encoder_ffn_dim: int = 3072, encoder_layerdrop: float | int = 0.1, decoder_layers: int = 6, decoder_ffn_dim: int = 3072, decoder_attention_heads: int = 12, decoder_layerdrop: float | int = 0.1, hidden_act: str = 'gelu', positional_dropout: float | int = 0.1, hidden_dropout: float | int = 0.1, attention_dropout: float | int = 0.1, activation_dropout: float | int = 0.1, initializer_range: float = 0.02, layer_norm_eps: float = 1e-05, scale_embedding: bool = False, feat_extract_norm: str = 'group', feat_proj_dropout: float | int = 0.0, feat_extract_activation: str = 'gelu', conv_dim: list[int] | tuple[int, ...] = (512, 512, 512, 512, 512, 512, 512), conv_stride: list[int] | tuple[int, ...] = (5, 2, 2, 2, 2, 2, 2), conv_kernel: list[int] | tuple[int, ...] = (10, 3, 3, 3, 3, 2, 2), conv_bias: bool = False, num_conv_pos_embeddings: int = 128, num_conv_pos_embedding_groups: int = 16, apply_spec_augment: bool = True, mask_time_prob: float | int = 0.05, mask_time_length: int = 10, mask_time_min_masks: int = 2, mask_feature_prob: float | int = 0.0, mask_feature_length: int = 10, mask_feature_min_masks: int = 0, pad_token_id: int | None = 1, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 2, decoder_start_token_id: int | None = 2, num_mel_bins: int = 80, speech_decoder_prenet_layers: int = 2, speech_decoder_prenet_units: int = 256, speech_decoder_prenet_dropout: float | int = 0.5, speaker_embedding_dim: int = 512, speech_decoder_postnet_layers: int = 5, speech_decoder_postnet_units: int = 256, speech_decoder_postnet_kernel: int = 5, speech_decoder_postnet_dropout: float | int = 0.5, reduction_factor: int = 2, max_speech_positions: int = 4000, max_text_positions: int = 450, encoder_max_relative_position: int = 160, use_guided_attention_loss: bool = True, guided_attention_loss_num_heads: int = 2, guided_attention_loss_sigma: float = 0.4, guided_attention_loss_scale: float = 10.0, use_cache: bool = True, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/configuration_speecht5.py#L27)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

vocab_size (`int`, *optional*, defaults to `81`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

encoder_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

encoder_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer encoder.

encoder_ffn_dim (`int`, *optional*, defaults to `3072`) : Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.

encoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.1`) : The LayerDrop probability for the encoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

decoder_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

decoder_ffn_dim (`int`, *optional*, defaults to `3072`) : Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.

decoder_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

decoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.1`) : The LayerDrop probability for the decoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

positional_dropout (`float`, *optional*, defaults to 0.1) : The dropout probability for the text position encoding layers.

hidden_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for activations inside the fully connected layer.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

scale_embedding (`bool`, *optional*, defaults to `False`) : Whether to scale embeddings by dividing by sqrt(d_model).

feat_extract_norm (`str`, *optional*, defaults to `"group"`) : The norm to be applied to 1D convolutional layers in the speech encoder pre-net. One of `"group"` for group normalization of only the first 1D convolutional layer or `"layer"` for layer normalization of all 1D convolutional layers.

feat_proj_dropout (`float`, *optional*, defaults to 0.0) : The dropout probability for output of the speech encoder pre-net.

feat_extract_activation (`str, `optional`, defaults to `"gelu"`) : The non-linear activation function (function or string) in the 1D convolutional layers of the feature extractor. If string, `"gelu"`, `"relu"`, `"selu"` and `"gelu_new"` are supported.

conv_dim (`tuple[int]` or `list[int]`, *optional*, defaults to `(512, 512, 512, 512, 512, 512, 512)`) : A tuple of integers defining the number of input and output channels of each 1D convolutional layer in the speech encoder pre-net. The length of *conv_dim* defines the number of 1D convolutional layers.

conv_stride (`tuple[int]` or `list[int]`, *optional*, defaults to `(5, 2, 2, 2, 2, 2, 2)`) : A tuple of integers defining the stride of each 1D convolutional layer in the speech encoder pre-net. The length of *conv_stride* defines the number of convolutional layers and has to match the length of *conv_dim*.

conv_kernel (`tuple[int]` or `list[int]`, *optional*, defaults to `(10, 3, 3, 3, 3, 3, 3)`) : A tuple of integers defining the kernel size of each 1D convolutional layer in the speech encoder pre-net. The length of *conv_kernel* defines the number of convolutional layers and has to match the length of *conv_dim*.

conv_bias (`bool`, *optional*, defaults to `False`) : Whether the 1D convolutional layers have a bias.

num_conv_pos_embeddings (`int`, *optional*, defaults to 128) : Number of convolutional positional embeddings. Defines the kernel size of 1D convolutional positional embeddings layer.

num_conv_pos_embedding_groups (`int`, *optional*, defaults to 16) : Number of groups of 1D convolutional positional embeddings layer.

apply_spec_augment (`bool`, *optional*, defaults to `True`) : Whether to apply *SpecAugment* data augmentation to the outputs of the speech encoder pre-net. For reference see [SpecAugment: A Simple Data Augmentation Method for Automatic Speech Recognition](https://huggingface.co/papers/1904.08779).

mask_time_prob (`float`, *optional*, defaults to 0.05) : Percentage (between 0 and 1) of all feature vectors along the time axis which will be masked. The masking procedure generates ''mask_time_prob*len(time_axis)/mask_time_length'' independent masks over the axis. If reasoning from the probability of each feature vector to be chosen as the start of the vector span to be masked, *mask_time_prob* should be `prob_vector_start*mask_time_length`. Note that overlap may decrease the actual percentage of masked vectors. This is only relevant if `apply_spec_augment is True`.

mask_time_length (`int`, *optional*, defaults to 10) : Length of vector span along the time axis.

mask_time_min_masks (`int`, *optional*, defaults to 2), : The minimum number of masks of length `mask_feature_length` generated along the time axis, each time step, irrespectively of `mask_feature_prob`. Only relevant if ''mask_time_prob*len(time_axis)/mask_time_length < mask_time_min_masks''

mask_feature_prob (`float`, *optional*, defaults to 0.0) : Percentage (between 0 and 1) of all feature vectors along the feature axis which will be masked. The masking procedure generates ''mask_feature_prob*len(feature_axis)/mask_time_length'' independent masks over the axis. If reasoning from the probability of each feature vector to be chosen as the start of the vector span to be masked, *mask_feature_prob* should be `prob_vector_start*mask_feature_length`. Note that overlap may decrease the actual percentage of masked vectors. This is only relevant if `apply_spec_augment is True`.

mask_feature_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : Percentage (between 0 and 1) of all feature vectors along the feature axis which will be masked. The masking procedure generates `mask_feature_prob*len(feature_axis)/mask_time_length` independent masks over the axis. If reasoning from the probability of each feature vector to be chosen as the start of the vector span to be masked, *mask_feature_prob* should be `prob_vector_start*mask_feature_length`. Note that overlap may decrease the actual percentage of masked vectors. This is only relevant if `apply_spec_augment` is `True`.

mask_feature_length (`int`, *optional*, defaults to 10) : Length of vector span along the feature axis.

mask_feature_min_masks (`int`, *optional*, defaults to 0), : The minimum number of masks of length `mask_feature_length` generated along the feature axis, each time step, irrespectively of `mask_feature_prob`. Only relevant if ''mask_feature_prob*len(feature_axis)/mask_feature_length < mask_feature_min_masks''

num_mel_bins (`int`, *optional*, defaults to 80) : Number of mel features used per input features. Used by the speech decoder pre-net. Should correspond to the value used in the [SpeechT5Processor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor) class.

pad_token_id (`int`, *optional*, defaults to `1`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

decoder_start_token_id (`int`, *optional*, defaults to `2`) : If an encoder-decoder model starts decoding with a different token than `bos`, the id of that token.

num_mel_bins (`int`, *optional*, defaults to `80`) : Number of mel features used per input frame. Should correspond to the value used in the `AutoFeatureExtractor` class.

speech_decoder_prenet_layers (`int`, *optional*, defaults to 2) : Number of layers in the speech decoder pre-net.

speech_decoder_prenet_units (`int`, *optional*, defaults to 256) : Dimensionality of the layers in the speech decoder pre-net.

speech_decoder_prenet_dropout (`float`, *optional*, defaults to 0.5) : The dropout probability for the speech decoder pre-net layers.

speaker_embedding_dim (`int`, *optional*, defaults to 512) : Dimensionality of the *XVector* embedding vectors.

speech_decoder_postnet_layers (`int`, *optional*, defaults to 5) : Number of layers in the speech decoder post-net.

speech_decoder_postnet_units (`int`, *optional*, defaults to 256) : Dimensionality of the layers in the speech decoder post-net.

speech_decoder_postnet_kernel (`int`, *optional*, defaults to 5) : Number of convolutional filter channels in the speech decoder post-net.

speech_decoder_postnet_dropout (`float`, *optional*, defaults to 0.5) : The dropout probability for the speech decoder post-net layers.

reduction_factor (`int`, *optional*, defaults to 2) : Spectrogram length reduction factor for the speech decoder inputs.

max_speech_positions (`int`, *optional*, defaults to 4000) : The maximum sequence length of speech features that this model might ever be used with.

max_text_positions (`int`, *optional*, defaults to 450) : The maximum sequence length of text features that this model might ever be used with.

encoder_max_relative_position (`int`, *optional*, defaults to 160) : Maximum distance for relative position embedding in the encoder.

use_guided_attention_loss (`bool`, *optional*, defaults to `True`) : Whether to apply guided attention loss while training the TTS model.

guided_attention_loss_num_heads (`int`, *optional*, defaults to 2) : Number of attention heads the guided attention loss will be applied to. Use -1 to apply this loss to all attention heads.

guided_attention_loss_sigma (`float`, *optional*, defaults to 0.4) : Standard deviation for guided attention loss.

guided_attention_loss_scale (`float`, *optional*, defaults to 10.0) : Scaling coefficient for guided attention loss (also known as lambda).

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a SpeechT5Model. It is used to instantiate a Speecht5
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/speecht5_asr](https://huggingface.co/microsoft/speecht5_asr)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import SpeechT5Model, SpeechT5Config

>>> # Initializing a "microsoft/speecht5_asr" style configuration
>>> configuration = SpeechT5Config()

>>> # Initializing a model (with random weights) from the "microsoft/speecht5_asr" style configuration
>>> model = SpeechT5Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SpeechT5HifiGanConfig[[transformers.SpeechT5HifiGanConfig]]

#### transformers.SpeechT5HifiGanConfig[[transformers.SpeechT5HifiGanConfig]]

```python
transformers.SpeechT5HifiGanConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, model_in_dim: int = 80, sampling_rate: int = 16000, upsample_initial_channel: int = 512, upsample_rates: list[int] | tuple[int, ...] = (4, 4, 4, 4), upsample_kernel_sizes: list[int] | tuple[int, ...] = (8, 8, 8, 8), resblock_kernel_sizes: list[int] | tuple[int, ...] = (3, 7, 11), resblock_dilation_sizes: list | tuple = ((1, 3, 5), (1, 3, 5), (1, 3, 5)), initializer_range: float = 0.01, leaky_relu_slope: float = 0.1, normalize_before: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/configuration_speecht5.py#L225)

**Parameters:**

model_in_dim (`int`, *optional*, defaults to 80) : The number of frequency bins in the input log-mel spectrogram.

sampling_rate (`int`, *optional*, defaults to `16000`) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

upsample_initial_channel (`int`, *optional*, defaults to 512) : The number of input channels into the upsampling network.

upsample_rates (`tuple[int]` or `list[int]`, *optional*, defaults to `[4, 4, 4, 4]`) : A tuple of integers defining the stride of each 1D convolutional layer in the upsampling network. The length of *upsample_rates* defines the number of convolutional layers and has to match the length of *upsample_kernel_sizes*.

upsample_kernel_sizes (`tuple[int]` or `list[int]`, *optional*, defaults to `[8, 8, 8, 8]`) : A tuple of integers defining the kernel size of each 1D convolutional layer in the upsampling network. The length of *upsample_kernel_sizes* defines the number of convolutional layers and has to match the length of *upsample_rates*.

resblock_kernel_sizes (`tuple[int]` or `list[int]`, *optional*, defaults to `[3, 7, 11]`) : A tuple of integers defining the kernel sizes of the 1D convolutional layers in the multi-receptive field fusion (MRF) module.

resblock_dilation_sizes (`tuple[tuple[int]]` or `list[list[int]]`, *optional*, defaults to `[[1, 3, 5], [1, 3, 5], [1, 3, 5]]`) : A nested tuple of integers defining the dilation rates of the dilated 1D convolutional layers in the multi-receptive field fusion (MRF) module.

initializer_range (`float`, *optional*, defaults to `0.01`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

leaky_relu_slope (`float`, *optional*, defaults to 0.1) : The angle of the negative slope used by the leaky ReLU activation.

normalize_before (`bool`, *optional*, defaults to `True`) : Whether or not to normalize the spectrogram before vocoding using the vocoder's learned mean and variance.

This is the configuration class to store the configuration of a SpeechT5Model. It is used to instantiate a Speecht5
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/speecht5_asr](https://huggingface.co/microsoft/speecht5_asr)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import SpeechT5HifiGan, SpeechT5HifiGanConfig

>>> # Initializing a "microsoft/speecht5_hifigan" style configuration
>>> configuration = SpeechT5HifiGanConfig()

>>> # Initializing a model (with random weights) from the "microsoft/speecht5_hifigan" style configuration
>>> model = SpeechT5HifiGan(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SpeechT5Tokenizer[[transformers.SpeechT5Tokenizer]]

#### transformers.SpeechT5Tokenizer[[transformers.SpeechT5Tokenizer]]

```python
transformers.SpeechT5Tokenizer(vocab_file, bos_token = '<s>', eos_token = '</s>', unk_token = '<unk>', pad_token = '<pad>', normalize = False, sp_model_kwargs: dict[str, typing.Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/tokenization_speecht5.py#L30)

**Parameters:**

vocab_file (`str`) : [SentencePiece](https://github.com/google/sentencepiece) file (generally has a *.spm* extension) that contains the vocabulary necessary to instantiate a tokenizer.

bos_token (`str`, *optional*, defaults to `"<s>"`) : The begin of sequence token.

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sequence token.

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

normalize (`bool`, *optional*, defaults to `False`) : Whether to convert numeric quantities in the text to their spelt-out english counterparts.

sp_model_kwargs (`dict`, *optional*) : Will be passed to the `SentencePieceProcessor.__init__()` method. The [Python wrapper for SentencePiece](https://github.com/google/sentencepiece/tree/master/python) can be used, among other things, to set:  - `enable_sampling`: Enable subword regularization. - `nbest_size`: Sampling parameters for unigram. Invalid for BPE-Dropout.  - `nbest_size = {0,1}`: No sampling is performed. - `nbest_size > 1`: samples from the nbest_size results. - `nbest_size < 0`: assuming that nbest_size is infinite and samples from the all hypothesis (lattice) using forward-filtering-and-backward-sampling algorithm.  - `alpha`: Smoothing parameter for unigram sampling, and dropout probability of merge operations for BPE-dropout.

sp_model (`SentencePieceProcessor`) : The *SentencePiece* processor that is used for every conversion (string, tokens and IDs).

Construct a SpeechT5 tokenizer. Based on [SentencePiece](https://github.com/google/sentencepiece).

This tokenizer inherits from [PreTrainedTokenizer](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.PythonBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods.

#### __call__[[transformers.SpeechT5Tokenizer.__call__]]

```python
__call__(text: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, text_pair: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, text_target: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, text_pair_target: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, add_special_tokens: bool = True, padding: bool | str | PaddingStrategy = False, truncation: bool | str | TruncationStrategy | None = None, max_length: int | None = None, stride: int = 0, is_split_into_words: bool = False, pad_to_multiple_of: int | None = None, padding_side: str | None = None, return_tensors: str | TensorType | None = None, return_token_type_ids: bool | None = None, return_attention_mask: bool | None = None, return_overflowing_tokens: bool = False, return_special_tokens_mask: bool = False, return_offsets_mapping: bool = False, return_length: bool = False, verbose: bool = True, tokenizer_kwargs: dict[str, Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_base.py#L2417)

**Parameters:**

text (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_pair (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_target (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_pair_target (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

tokenizer_kwargs (`dict[str, Any]`, *optional*) : Additional kwargs to pass to the tokenizer. These will be merged with the explicit parameters and other kwargs, with explicit parameters taking precedence. 

add_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to add special tokens when encoding the sequences. This will use the underlying `PretrainedTokenizerBase.build_inputs_with_special_tokens` function, which defines which tokens are automatically added to the input ids. This is useful if you want to add `bos` or `eos` tokens automatically.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Activates and controls padding. Accepts the following values:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence is provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

truncation (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) : Activates and controls truncation. Accepts the following values:  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will truncate token by token, removing a token from the longest sequence in the pair if a pair of sequences (or a batch of pairs) is provided. - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths greater than the model maximum admissible input size).

max_length (`int`, *optional*) : Controls the maximum length to use by one of the truncation/padding parameters.  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length is required by one of the truncation/padding parameters. If the model has no specific maximum input length (like XLNet) truncation/padding to a maximum length will be deactivated.

stride (`int`, *optional*, defaults to 0) : If set to a number along with `max_length`, the overflowing tokens returned when `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence returned to provide some overlap between truncated and overflowing sequences. The value of this argument defines the number of overlapping tokens.

is_split_into_words (`bool`, *optional*, defaults to `False`) : Whether or not the input is already pre-tokenized (e.g., split into words). If set to `True`, the tokenizer assumes the input is already split into words (for instance, by splitting it on whitespace) which it will tokenize. This is useful for NER or token classification.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value. Requires `padding` to be activated. This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

padding_side (`str`, *optional*) : The side on which the model should have padding applied. Should be selected between ['right', 'left']. Default value is picked from the class attribute of the same name.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects. 

return_token_type_ids (`bool`, *optional*) : Whether to return token type IDs. If left to the default, will return the token type IDs according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are token type IDs?](../glossary#token-type-ids)

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are attention masks?](../glossary#attention-mask)

return_overflowing_tokens (`bool`, *optional*, defaults to `False`) : Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead of returning overflowing tokens.

return_special_tokens_mask (`bool`, *optional*, defaults to `False`) : Whether or not to return special tokens mask information.

return_offsets_mapping (`bool`, *optional*, defaults to `False`) : Whether or not to return `(char_start, char_end)` for each token.  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend), if using Python's tokenizer, this method will raise `NotImplementedError`.

return_length  (`bool`, *optional*, defaults to `False`) : Whether or not to return the lengths of the encoded inputs.

verbose (`bool`, *optional*, defaults to `True`) : Whether or not to print more information and warnings.

- ****kwargs** : passed to the `self.tokenize()` method

**Returns:** [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding)

A [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields:

- **input_ids** -- List of token ids to be fed to a model.

  [What are input IDs?](../glossary#input-ids)

- **token_type_ids** -- List of token type ids to be fed to a model (when `return_token_type_ids=True` or
  if *"token_type_ids"* is in `self.model_input_names`).

  [What are token type IDs?](../glossary#token-type-ids)

- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names`).

  [What are attention masks?](../glossary#attention-mask)

- **overflowing_tokens** -- List of overflowing tokens sequences (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **num_truncated_tokens** -- Number of tokens truncated (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **special_tokens_mask** -- List of 0s and 1s, with 1 specifying added special tokens and 0 specifying
  regular sequence tokens (when `add_special_tokens=True` and `return_special_tokens_mask=True`).
- **length** -- The length of the inputs (when `return_length=True`)

Main method to tokenize and prepare for the model one or several sequence(s) or one or several pair(s) of
sequences.

#### save_vocabulary[[transformers.SpeechT5Tokenizer.save_vocabulary]]

```python
save_vocabulary(save_directory: str, filename_prefix: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_sentencepiece.py#L237)

**Parameters:**

save_directory (`str`) : The directory in which to save the vocabulary.

filename_prefix (`str`, *optional*) : An optional prefix to add to the named of the saved files.

**Returns:** `tuple(str)`

Paths to the files saved.

Save the sentencepiece vocabulary (copy original file) to a directory.

#### decode[[transformers.SpeechT5Tokenizer.decode]]

```python
decode(token_ids: int | list[int] | list[list[int]] | np.ndarray | torch.Tensor, skip_special_tokens: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_base.py#L2853)

**Parameters:**

token_ids (`Union[int, list[int], list[list[int]], np.ndarray, torch.Tensor]`) : A single sequence or a batch (list of sequences) of tokenized input ids. Can be obtained using the `__call__` method.

skip_special_tokens (`bool`, *optional*, defaults to `False`) : Whether or not to remove special tokens in the decoding.

kwargs (additional keyword arguments, *optional*) : Will be passed to the underlying model specific decode method.

**Returns:** `Union[str, list[str]]`

The decoded string for a single sequence, or a list of decoded strings for a
batch of sequences.

Converts a sequence of ids into a string, or a list of sequences into a list of strings,
using the tokenizer and vocabulary with options to remove special tokens and clean up
tokenization spaces.

Similar to doing `self.convert_tokens_to_string(self.convert_ids_to_tokens(token_ids))`.

#### batch_decode[[transformers.SpeechT5Tokenizer.batch_decode]]

```python
batch_decode(sequences: list[int] | list[list[int]] | np.ndarray | torch.Tensor, skip_special_tokens: bool = False, clean_up_tokenization_spaces: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_base.py#L2901)

**Parameters:**

sequences (`Union[list[int], list[list[int]], np.ndarray, torch.Tensor]`) : List of tokenized input ids. Can be obtained using the `__call__` method.

skip_special_tokens (`bool`, *optional*, defaults to `False`) : Whether or not to remove special tokens in the decoding.

clean_up_tokenization_spaces (`bool`, *optional*) : Whether or not to clean up the tokenization spaces. If `None`, will default to `self.clean_up_tokenization_spaces`.

kwargs (additional keyword arguments, *optional*) : Will be passed to the underlying model specific decode method.

**Returns:** `list[str]`

The list of decoded sentences.

Convert a list of lists of token ids into a list of strings by calling decode.

This method is provided for backwards compatibility. The `decode` method now handles batched input natively,
so you can use `decode` directly instead of `batch_decode`.

## SpeechT5FeatureExtractor[[transformers.SpeechT5FeatureExtractor]]

#### transformers.SpeechT5FeatureExtractor[[transformers.SpeechT5FeatureExtractor]]

```python
transformers.SpeechT5FeatureExtractor(feature_size: int = 1, sampling_rate: int = 16000, padding_value: float = 0.0, do_normalize: bool = False, num_mel_bins: int = 80, hop_length: int = 16, win_length: int = 64, win_function: str = 'hann_window', fmin: float = 80, fmax: float = 7600, mel_floor: float = 1e-10, return_attention_mask: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/feature_extraction_speecht5.py#L29)

**Parameters:**

feature_size (`int`, *optional*, defaults to 1) : The feature dimension of the extracted features.

sampling_rate (`int`, *optional*, defaults to 16000) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

padding_value (`float`, *optional*, defaults to 0.0) : The value that is used to fill the padding values.

do_normalize (`bool`, *optional*, defaults to `False`) : Whether or not to zero-mean unit-variance normalize the input. Normalizing can help to significantly improve the performance for some models.

num_mel_bins (`int`, *optional*, defaults to 80) : The number of mel-frequency bins in the extracted spectrogram features.

hop_length (`int`, *optional*, defaults to 16) : Number of ms between windows. Otherwise referred to as "shift" in many papers.

win_length (`int`, *optional*, defaults to 64) : Number of ms per window.

win_function (`str`, *optional*, defaults to `"hann_window"`) : Name for the window function used for windowing, must be accessible via `torch.{win_function}`

fmin (`float`, *optional*, defaults to 80) : Minimum mel frequency in Hz.

fmax (`float`, *optional*, defaults to 7600) : Maximum mel frequency in Hz.

mel_floor (`float`, *optional*, defaults to 1e-10) : Minimum value of mel frequency banks..

return_attention_mask (`bool`, *optional*, defaults to `True`) : Whether or not [__call__()](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5FeatureExtractor.__call__) should return `attention_mask`.

Constructs a SpeechT5 feature extractor.

This class can pre-process a raw speech signal by (optionally) normalizing to zero-mean unit-variance, for use by
the SpeechT5 speech encoder prenet.

This class can also extract log-mel filter bank features from raw speech, for use by the SpeechT5 speech decoder
prenet.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

#### __call__[[transformers.SpeechT5FeatureExtractor.__call__]]

```python
__call__(audio: numpy.ndarray | list[float] | list[numpy.ndarray] | list[list[float]] | None = None, audio_target: numpy.ndarray | list[float] | list[numpy.ndarray] | list[list[float]] | None = None, padding: bool | str | transformers.utils.generic.PaddingStrategy = False, max_length: int | None = None, truncation: bool = False, pad_to_multiple_of: int | None = None, return_attention_mask: bool | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, sampling_rate: int | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/feature_extraction_speecht5.py#L159)

**Parameters:**

audio (`np.ndarray`, `list[float]`, `list[np.ndarray]`, `list[list[float]]`, *optional*) : The sequence or batch of sequences to be processed. Each sequence can be a numpy array, a list of float values, a list of numpy arrays or a list of list of float values. This outputs waveform features. Must be mono channel audio, not stereo, i.e. single float per timestep.

audio_target (`np.ndarray`, `list[float]`, `list[np.ndarray]`, `list[list[float]]`, *optional*) : The sequence or batch of sequences to be processed as targets. Each sequence can be a numpy array, a list of float values, a list of numpy arrays or a list of list of float values. This outputs log-mel spectrogram features.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Select a strategy to pad the returned sequences (according to the model's padding side and padding index) among:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

max_length (`int`, *optional*) : Maximum length of the returned list and optionally padding length (see above).

truncation (`bool`) : Activates truncation to cut input sequences longer than *max_length* to *max_length*.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value.  This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta), or on TPUs which benefit from having sequence lengths be a multiple of 128.

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific feature_extractor's default.  [What are attention masks?](../glossary#attention-mask) 

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects.

sampling_rate (`int`, *optional*) : The sampling rate at which the `audio` or `audio_target` input was sampled. It is strongly recommended to pass `sampling_rate` at the forward call to prevent silent errors.

Main method to featurize and prepare for the model one or several sequence(s).

Pass in a value for `audio` to extract waveform features. Pass in a value for `audio_target` to extract log-mel
spectrogram features.

## SpeechT5Processor[[transformers.SpeechT5Processor]]

#### transformers.SpeechT5Processor[[transformers.SpeechT5Processor]]

```python
transformers.SpeechT5Processor(feature_extractor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/processing_speecht5.py#L21)

**Parameters:**

feature_extractor (`SpeechT5FeatureExtractor`) : The feature extractor is a required input.

tokenizer (`SpeechT5Tokenizer`) : The tokenizer is a required input.

Constructs a SpeechT5Processor which wraps a feature extractor and a tokenizer into a single processor.

[SpeechT5Processor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor) offers all the functionalities of [SpeechT5FeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5FeatureExtractor) and [SpeechT5Tokenizer](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Tokenizer). See the
[~SpeechT5FeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5FeatureExtractor) and [~SpeechT5Tokenizer](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Tokenizer) for more information.

#### __call__[[transformers.SpeechT5Processor.__call__]]

```python
__call__(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/processing_speecht5.py#L25)

**Parameters:**

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

#### pad[[transformers.SpeechT5Processor.pad]]

```python
pad(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/processing_speecht5.py#L74)

Collates the audio and text inputs, as well as their targets, into a padded batch.

Audio inputs are padded by SpeechT5FeatureExtractor's [pad()](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor.pad). Text inputs are padded
by SpeechT5Tokenizer's [pad()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.pad).

Valid input combinations are:

- `input_ids` only
- `input_values` only
- `labels` only, either log-mel spectrograms or text tokens
- `input_ids` and log-mel spectrogram `labels`
- `input_values` and text `labels`

Please refer to the docstring of the above two methods for more information.

#### from_pretrained[[transformers.SpeechT5Processor.from_pretrained]]

```python
from_pretrained(pretrained_model_name_or_path: str | os.PathLike, cache_dir: str | os.PathLike | None = None, force_download: bool = False, local_files_only: bool = False, token: str | bool | None = None, revision: str = 'main', **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1682)

**Parameters:**

pretrained_model_name_or_path (`str` or `os.PathLike`) : This can be either:  - a string, the *model id* of a pretrained feature_extractor hosted inside a model repo on huggingface.co. - a path to a *directory* containing a feature extractor file saved using the [save_pretrained()](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.save_pretrained) method, e.g., `./my_model_directory/`. - a path to a saved feature extractor JSON *file*, e.g., `./my_model_directory/preprocessor_config.json`.

- ****kwargs** : Additional keyword arguments passed along to both [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.from_pretrained) and `~tokenization_utils_base.PreTrainedTokenizer.from_pretrained`.

Instantiate a processor associated with a pretrained model.

This class method is simply calling the feature extractor
[from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.from_pretrained), image processor
[ImageProcessingMixin](/docs/transformers/v5.15.1/en/main_classes/image_processor#transformers.ImageProcessingMixin) and the tokenizer
`~tokenization_utils_base.PreTrainedTokenizer.from_pretrained` methods. Please refer to the docstrings of the
methods above for more information.

#### save_pretrained[[transformers.SpeechT5Processor.save_pretrained]]

```python
save_pretrained(save_directory, push_to_hub: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1107)

**Parameters:**

save_directory (`str` or `os.PathLike`) : Directory where the feature extractor JSON file and the tokenizer files will be saved (directory will be created if it does not exist).

push_to_hub (`bool`, *optional*, defaults to `False`) : Whether or not to push your model to the Hugging Face model hub after saving it. You can specify the repository you want to push to with `repo_id` (will default to the name of `save_directory` in your namespace).

kwargs (`dict[str, Any]`, *optional*) : Additional key word arguments passed along to the [push_to_hub()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.utils.PushToHubMixin.push_to_hub) method.

Saves the attributes of this processor (feature extractor, tokenizer...) in the specified directory so that it
can be reloaded using the [from_pretrained()](/docs/transformers/v5.15.1/en/model_doc/donut#transformers.DonutProcessor.from_pretrained) method.

This class method is simply calling [save_pretrained()](/docs/transformers/v5.15.1/en/main_classes/feature_extractor#transformers.FeatureExtractionMixin.save_pretrained) and
[save_pretrained()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.save_pretrained). Please refer to the docstrings of the
methods above for more information.

#### batch_decode[[transformers.SpeechT5Processor.batch_decode]]

```python
batch_decode(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1930)

This method forwards all its arguments to PreTrainedTokenizer's [batch_decode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.batch_decode). Please
refer to the docstring of this method for more information.

#### decode[[transformers.SpeechT5Processor.decode]]

```python
decode(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1939)

This method forwards all its arguments to PreTrainedTokenizer's [decode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.decode). Please refer to
the docstring of this method for more information.

## SpeechT5Model[[transformers.SpeechT5Model]]

#### transformers.SpeechT5Model[[transformers.SpeechT5Model]]

```python
transformers.SpeechT5Model(config: SpeechT5Config, encoder: typing.Optional[torch.nn.Module] = None, decoder: typing.Optional[torch.nn.Module] = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L1852)

**Parameters:**

config ([SpeechT5Config](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

encoder (`PreTrainedModel`, *optional*) : The encoder model to use.

decoder (`PreTrainedModel`, *optional*) : The decoder model to use.

The bare SpeechT5 Encoder-Decoder Model outputting raw hidden-states without any specific pre- or post-nets.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SpeechT5Model.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_values: typing.Optional[torch.Tensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, use_cache: bool | None = None, speaker_embeddings: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L1894)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`) : Depending on which encoder is being used, the `input_values` are either: float values of the input raw speech waveform, or indices of input sequence tokens in the vocabulary, or hidden states.

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_values (`torch.Tensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Depending on which decoder is being used, the `decoder_input_values` are either: float values of log-mel filterbank features extracted from the raw speech waveform, or indices of decoder input sequence tokens in the vocabulary, or hidden states.

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_values`. Causal mask will also be used by default.  If you want to change padding behavior, you should read `SpeechT5Decoder._prepare_decoder_attention_mask` and modify to your needs. See diagram 1 in [the paper](https://huggingface.co/papers/1910.13461) for more information on the default strategy.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

speaker_embeddings (`torch.FloatTensor` of shape `(batch_size, config.speaker_embedding_dim)`, *optional*) : Tensor containing the speaker embeddings.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [Seq2SeqModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SpeechT5Config](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Config)) and inputs.

The [SpeechT5Model](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Model) forward method, overrides the `__call__` special method.

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

## SpeechT5ForSpeechToText[[transformers.SpeechT5ForSpeechToText]]

#### transformers.SpeechT5ForSpeechToText[[transformers.SpeechT5ForSpeechToText]]

```python
transformers.SpeechT5ForSpeechToText(config: SpeechT5Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L1998)

**Parameters:**

config ([SpeechT5Config](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SpeechT5 Model with a speech encoder and a text decoder.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SpeechT5ForSpeechToText.forward]]

```python
forward(input_values: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, use_cache: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L2034)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a *.flac* or *.wav* audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [SpeechT5Processor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [SpeechT5Processor.__call__()](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor.__call__) for details.

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using [SpeechT5Tokenizer](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Tokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)  SpeechT5 uses the `eos_token_id` as the starting token for `decoder_input_ids` generation. If `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see `past_key_values`).

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_values`. Causal mask will also be used by default.  If you want to change padding behavior, you should read `SpeechT5Decoder._prepare_decoder_attention_mask` and modify to your needs. See diagram 1 in [the paper](https://huggingface.co/papers/1910.13461) for more information on the default strategy.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.  Label indices can be obtained using [SpeechT5Tokenizer](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Tokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SpeechT5Config](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Config)) and inputs.

The [SpeechT5ForSpeechToText](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5ForSpeechToText) forward method, overrides the `__call__` special method.

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
>>> from transformers import SpeechT5Processor, SpeechT5ForSpeechToText
>>> from datasets import load_dataset

>>> dataset = load_dataset(
...     "hf-internal-testing/librispeech_asr_demo", "clean", split="validation"
... )  # doctest: +IGNORE_RESULT
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> processor = SpeechT5Processor.from_pretrained("microsoft/speecht5_asr")
>>> model = SpeechT5ForSpeechToText.from_pretrained("microsoft/speecht5_asr")

>>> # audio file is decoded on the fly
>>> inputs = processor(audio=dataset[0]["audio"]["array"], sampling_rate=sampling_rate, return_tensors="pt")
>>> predicted_ids = model.generate(**inputs, max_length=100)

>>> # transcribe speech
>>> transcription = processor.batch_decode(predicted_ids, skip_special_tokens=True)
>>> transcription[0]
'mister quilter is the apostle of the middle classes and we are glad to welcome his gospel'
```

```python
>>> inputs["labels"] = processor(text_target=dataset[0]["text"], return_tensors="pt").input_ids

>>> # compute loss
>>> loss = model(**inputs).loss
>>> round(loss.item(), 2)
19.68
```

## SpeechT5ForTextToSpeech[[transformers.SpeechT5ForTextToSpeech]]

#### transformers.SpeechT5ForTextToSpeech[[transformers.SpeechT5ForTextToSpeech]]

```python
transformers.SpeechT5ForTextToSpeech(config: SpeechT5Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L2310)

**Parameters:**

config ([SpeechT5Config](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SpeechT5 Model with a text encoder and a speech decoder.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SpeechT5ForTextToSpeech.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_values: typing.Optional[torch.FloatTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, use_cache: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, speaker_embeddings: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.FloatTensor] = None, stop_labels: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L2341)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary.  Indices can be obtained using [SpeechT5Tokenizer](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Tokenizer). See [encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_mel_bins)`) : Float values of input mel spectrogram.  SpeechT5 uses an all-zero spectrum as the starting token for `decoder_input_values` generation. If `past_key_values` is used, optionally only the last `decoder_input_values` have to be input (see `past_key_values`).

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_values`. Causal mask will also be used by default.  If you want to change padding behavior, you should read `SpeechT5Decoder._prepare_decoder_attention_mask` and modify to your needs. See diagram 1 in [the paper](https://huggingface.co/papers/1910.13461) for more information on the default strategy.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

speaker_embeddings (`torch.FloatTensor` of shape `(batch_size, config.speaker_embedding_dim)`, *optional*) : Tensor containing the speaker embeddings.

labels (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_mel_bins)`, *optional*) : Float values of target mel spectrogram. Timesteps set to `-100.0` are ignored (masked) for the loss computation. Spectrograms can be obtained using [SpeechT5Processor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor). See [SpeechT5Processor.__call__()](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor.__call__) for details.

stop_labels (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Binary tensor indicating the position of the stop token in the sequence.

**Returns:** [Seq2SeqSpectrogramOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqSpectrogramOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqSpectrogramOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqSpectrogramOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SpeechT5Config](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Config)) and inputs.

The [SpeechT5ForTextToSpeech](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5ForTextToSpeech) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Spectrogram generation loss.
- **spectrogram** (`torch.FloatTensor` of shape `(batch_size, sequence_length, num_bins)`) -- The predicted spectrogram.
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
>>> from transformers import SpeechT5Processor, SpeechT5ForTextToSpeech, SpeechT5HifiGan, set_seed
>>> import torch

>>> processor = SpeechT5Processor.from_pretrained("microsoft/speecht5_tts")
>>> model = SpeechT5ForTextToSpeech.from_pretrained("microsoft/speecht5_tts")
>>> vocoder = SpeechT5HifiGan.from_pretrained("microsoft/speecht5_hifigan")

>>> inputs = processor(text="Hello, my dog is cute", return_tensors="pt")
>>> speaker_embeddings = torch.zeros((1, 512))  # or load xvectors from a file

>>> set_seed(555)  # make deterministic

>>> # generate speech
>>> speech = model.generate(inputs["input_ids"], speaker_embeddings=speaker_embeddings, vocoder=vocoder)
>>> speech.shape
torch.Size([15872])
```

#### generate[[transformers.SpeechT5ForTextToSpeech.generate]]

```python
generate(input_ids: LongTensor, attention_mask: typing.Optional[torch.LongTensor] = None, speaker_embeddings: typing.Optional[torch.FloatTensor] = None, threshold: float = 0.5, minlenratio: float = 0.0, maxlenratio: float = 20.0, vocoder: typing.Optional[torch.nn.Module] = None, output_cross_attentions: bool = False, return_output_lengths: bool = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L2464)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary.  Indices can be obtained using [SpeechT5Tokenizer](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Tokenizer). See [encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Attention mask from the tokenizer, required for batched inference to signal to the model where to ignore padded tokens from the input_ids.

speaker_embeddings (`torch.FloatTensor` of shape `(batch_size, config.speaker_embedding_dim)`, *optional*) : Tensor containing the speaker embeddings.

threshold (`float`, *optional*, defaults to 0.5) : The generated sequence ends when the predicted stop token probability exceeds this value.

minlenratio (`float`, *optional*, defaults to 0.0) : Used to calculate the minimum required length for the output sequence.

maxlenratio (`float`, *optional*, defaults to 20.0) : Used to calculate the maximum allowed length for the output sequence.

vocoder (`nn.Module`, *optional*) : The vocoder that converts the mel spectrogram into a speech waveform. If `None`, the output is the mel spectrogram.

output_cross_attentions (`bool`, *optional*, defaults to `False`) : Whether or not to return the attentions tensors of the decoder's cross-attention layers.

return_output_lengths (`bool`, *optional*, defaults to `False`) : Whether or not to return the concrete spectrogram/waveform lengths.

**Returns:** `tuple(torch.FloatTensor)` comprising various elements depending on the inputs

- when `return_output_lengths` is False
  - **spectrogram** (*optional*, returned when no `vocoder` is provided) `torch.FloatTensor` of shape
  `(output_sequence_length, config.num_mel_bins)` -- The predicted log-mel spectrogram.
  - **waveform** (*optional*, returned when a `vocoder` is provided) `torch.FloatTensor` of shape
  `(num_frames,)` -- The predicted speech waveform.
  - **cross_attentions** (*optional*, returned when `output_cross_attentions` is `True`)
  `torch.FloatTensor` of shape `(config.decoder_layers, config.decoder_attention_heads,
  output_sequence_length, input_sequence_length)` -- The outputs of the decoder's cross-attention layers.
- when `return_output_lengths` is True
  - **spectrograms** (*optional*, returned when no `vocoder` is provided) `torch.FloatTensor` of shape
  `(batch_size, output_sequence_length, config.num_mel_bins)` -- The predicted log-mel spectrograms that
  are padded to the maximum length.
  - **spectrogram_lengths** (*optional*, returned when no `vocoder` is provided) `list[Int]` -- A list of
  all the concrete lengths for each spectrogram.
  - **waveforms** (*optional*, returned when a `vocoder` is provided) `torch.FloatTensor` of shape
  `(batch_size, num_frames)` -- The predicted speech waveforms that are padded to the maximum length.
  - **waveform_lengths** (*optional*, returned when a `vocoder` is provided) `list[Int]` -- A list of all
  the concrete lengths for each waveform.
  - **cross_attentions** (*optional*, returned when `output_cross_attentions` is `True`)
  `torch.FloatTensor` of shape `(batch_size, config.decoder_layers, config.decoder_attention_heads,
  output_sequence_length, input_sequence_length)` -- The outputs of the decoder's cross-attention layers.

Converts a sequence of input tokens into a sequence of mel spectrograms, which are subsequently turned into a
speech waveform using a vocoder.

## SpeechT5ForSpeechToSpeech[[transformers.SpeechT5ForSpeechToSpeech]]

#### transformers.SpeechT5ForSpeechToSpeech[[transformers.SpeechT5ForSpeechToSpeech]]

```python
transformers.SpeechT5ForSpeechToSpeech(config: SpeechT5Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L2658)

**Parameters:**

config ([SpeechT5Config](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SpeechT5 Model with a speech encoder and a speech decoder.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SpeechT5ForSpeechToSpeech.forward]]

```python
forward(input_values: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, decoder_input_values: typing.Optional[torch.FloatTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, use_cache: bool | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, speaker_embeddings: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.FloatTensor] = None, stop_labels: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L2678)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a *.flac* or *.wav* audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [SpeechT5Processor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [SpeechT5Processor.__call__()](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor.__call__) for details.

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_mel_bins)`) : Float values of input mel spectrogram.  SpeechT5 uses an all-zero spectrum as the starting token for `decoder_input_values` generation. If `past_key_values` is used, optionally only the last `decoder_input_values` have to be input (see `past_key_values`).

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_values`. Causal mask will also be used by default.  If you want to change padding behavior, you should read `SpeechT5Decoder._prepare_decoder_attention_mask` and modify to your needs. See diagram 1 in [the paper](https://huggingface.co/papers/1910.13461) for more information on the default strategy.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

speaker_embeddings (`torch.FloatTensor` of shape `(batch_size, config.speaker_embedding_dim)`, *optional*) : Tensor containing the speaker embeddings.

labels (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_mel_bins)`, *optional*) : Float values of target mel spectrogram. Spectrograms can be obtained using [SpeechT5Processor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor). See [SpeechT5Processor.__call__()](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor.__call__) for details.

stop_labels (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Binary tensor indicating the position of the stop token in the sequence.

**Returns:** [Seq2SeqSpectrogramOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqSpectrogramOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqSpectrogramOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqSpectrogramOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SpeechT5Config](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Config)) and inputs.

The [SpeechT5ForSpeechToSpeech](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5ForSpeechToSpeech) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Spectrogram generation loss.
- **spectrogram** (`torch.FloatTensor` of shape `(batch_size, sequence_length, num_bins)`) -- The predicted spectrogram.
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
>>> from transformers import SpeechT5Processor, SpeechT5ForSpeechToSpeech, SpeechT5HifiGan, set_seed
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset(
...     "hf-internal-testing/librispeech_asr_demo", "clean", split="validation"
... )  # doctest: +IGNORE_RESULT
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> processor = SpeechT5Processor.from_pretrained("microsoft/speecht5_vc")
>>> model = SpeechT5ForSpeechToSpeech.from_pretrained("microsoft/speecht5_vc")
>>> vocoder = SpeechT5HifiGan.from_pretrained("microsoft/speecht5_hifigan")

>>> # audio file is decoded on the fly
>>> inputs = processor(audio=dataset[0]["audio"]["array"], sampling_rate=sampling_rate, return_tensors="pt")

>>> speaker_embeddings = torch.zeros((1, 512))  # or load xvectors from a file

>>> set_seed(555)  # make deterministic

>>> # generate speech
>>> speech = model.generate_speech(inputs["input_values"], speaker_embeddings, vocoder=vocoder)
>>> speech.shape
torch.Size([77824])
```

#### generate_speech[[transformers.SpeechT5ForSpeechToSpeech.generate_speech]]

```python
generate_speech(input_values: FloatTensor, speaker_embeddings: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, threshold: float = 0.5, minlenratio: float = 0.0, maxlenratio: float = 20.0, vocoder: typing.Optional[torch.nn.Module] = None, output_cross_attentions: bool = False, return_output_lengths: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L2796)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform.  Values can be obtained by loading a *.flac* or *.wav* audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [SpeechT5Processor](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [SpeechT5Processor.__call__()](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5Processor.__call__) for details.

speaker_embeddings (`torch.FloatTensor` of shape `(batch_size, config.speaker_embedding_dim)`, *optional*) : Tensor containing the speaker embeddings.

attention_mask (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing convolution and attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

threshold (`float`, *optional*, defaults to 0.5) : The generated sequence ends when the predicted stop token probability exceeds this value.

minlenratio (`float`, *optional*, defaults to 0.0) : Used to calculate the minimum required length for the output sequence.

maxlenratio (`float`, *optional*, defaults to 20.0) : Used to calculate the maximum allowed length for the output sequence.

vocoder (`nn.Module`, *optional*, defaults to `None`) : The vocoder that converts the mel spectrogram into a speech waveform. If `None`, the output is the mel spectrogram.

output_cross_attentions (`bool`, *optional*, defaults to `False`) : Whether or not to return the attentions tensors of the decoder's cross-attention layers.

return_output_lengths (`bool`, *optional*, defaults to `False`) : Whether or not to return the concrete spectrogram/waveform lengths.

**Returns:** `tuple(torch.FloatTensor)` comprising various elements depending on the inputs

- when `return_output_lengths` is False
  - **spectrogram** (*optional*, returned when no `vocoder` is provided) `torch.FloatTensor` of shape
  `(output_sequence_length, config.num_mel_bins)` -- The predicted log-mel spectrogram.
  - **waveform** (*optional*, returned when a `vocoder` is provided) `torch.FloatTensor` of shape
  `(num_frames,)` -- The predicted speech waveform.
  - **cross_attentions** (*optional*, returned when `output_cross_attentions` is `True`)
  `torch.FloatTensor` of shape `(config.decoder_layers, config.decoder_attention_heads,
  output_sequence_length, input_sequence_length)` -- The outputs of the decoder's cross-attention layers.
- when `return_output_lengths` is True
  - **spectrograms** (*optional*, returned when no `vocoder` is provided) `torch.FloatTensor` of shape
  `(batch_size, output_sequence_length, config.num_mel_bins)` -- The predicted log-mel spectrograms that
  are padded to the maximum length.
  - **spectrogram_lengths** (*optional*, returned when no `vocoder` is provided) `list[Int]` -- A list of
  all the concrete lengths for each spectrogram.
  - **waveforms** (*optional*, returned when a `vocoder` is provided) `torch.FloatTensor` of shape
  `(batch_size, num_frames)` -- The predicted speech waveforms that are padded to the maximum length.
  - **waveform_lengths** (*optional*, returned when a `vocoder` is provided) `list[Int]` -- A list of all
  the concrete lengths for each waveform.
  - **cross_attentions** (*optional*, returned when `output_cross_attentions` is `True`)
  `torch.FloatTensor` of shape `(batch_size, config.decoder_layers, config.decoder_attention_heads,
  output_sequence_length, input_sequence_length)` -- The outputs of the decoder's cross-attention layers.

Converts a raw speech waveform into a sequence of mel spectrograms, which are subsequently turned back into a
speech waveform using a vocoder.

## SpeechT5HifiGan[[transformers.SpeechT5HifiGan]]

#### transformers.SpeechT5HifiGan[[transformers.SpeechT5HifiGan]]

```python
transformers.SpeechT5HifiGan(config: SpeechT5HifiGanConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L2954)

**Parameters:**

config ([SpeechT5HifiGanConfig](/docs/transformers/v5.15.1/en/model_doc/speecht5#transformers.SpeechT5HifiGanConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

HiFi-GAN vocoder.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SpeechT5HifiGan.forward]]

```python
forward(spectrogram: FloatTensor, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speecht5/modeling_speecht5.py#L3022)

**Parameters:**

spectrogram (`torch.FloatTensor`) : Tensor containing the log-mel spectrograms. Can be batched and of shape `(batch_size, sequence_length, config.model_in_dim)`, or un-batched and of shape `(sequence_length, config.model_in_dim)`.

**Returns:** `torch.FloatTensor`

Tensor containing the speech waveform. If the input spectrogram is batched, will be of
shape `(batch_size, num_frames,)`. If un-batched, will be of shape `(num_frames,)`.

Converts a log-mel spectrogram into a speech waveform. Passing a batch of log-mel spectrograms returns a batch
of speech waveforms. Passing a single, un-batched log-mel spectrogram returns a single, un-batched speech
waveform.
