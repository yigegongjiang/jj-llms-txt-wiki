# Speech2Text

## Overview

The Speech2Text model was proposed in [fairseq S2T: Fast Speech-to-Text Modeling with fairseq](https://huggingface.co/papers/2010.05171) by Changhan Wang, Yun Tang, Xutai Ma, Anne Wu, Dmytro Okhonko, Juan Pino. It's a
transformer-based seq2seq (encoder-decoder) model designed for end-to-end Automatic Speech Recognition (ASR) and Speech
Translation (ST). It uses a convolutional downsampler to reduce the length of speech inputs by 3/4th before they are
fed into the encoder. The model is trained with standard autoregressive cross-entropy loss and generates the
transcripts/translations autoregressively. Speech2Text has been fine-tuned on several datasets for ASR and ST:
[LibriSpeech](http://www.openslr.org/12), [CoVoST 2](https://github.com/facebookresearch/covost), [MuST-C](https://ict.fbk.eu/must-c/).

This model was contributed by [valhalla](https://huggingface.co/valhalla). The original code can be found [here](https://github.com/pytorch/fairseq/tree/master/examples/speech_to_text).

## Inference

Speech2Text is a speech model that accepts a float tensor of log-mel filter-bank features extracted from the speech
signal. It's a transformer-based seq2seq model, so the transcripts/translations are generated autoregressively. The
`generate()` method can be used for inference.

The [Speech2TextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor) class is responsible for extracting the log-mel filter-bank
features. The [Speech2TextProcessor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextProcessor) wraps [Speech2TextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor) and
[Speech2TextTokenizer](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextTokenizer) into a single instance to both extract the input features and decode the
predicted token ids.

The feature extractor depends on `torchaudio` and the tokenizer depends on `sentencepiece` so be sure to
install those packages before running the examples. You could either install those as extra speech dependencies with
`pip install transformers"[speech, sentencepiece]"` or install the packages separately with `pip install torchaudio sentencepiece`. Also `torchaudio` requires the development version of the [libsndfile](http://www.mega-nerd.com/libsndfile/) package which can be installed via a system package manager. On Ubuntu it can
be installed as follows: `apt install libsndfile1-dev`

- ASR and Speech Translation

```python
from datasets import load_dataset

from transformers import Speech2TextForConditionalGeneration, Speech2TextProcessor

model = Speech2TextForConditionalGeneration.from_pretrained("facebook/s2t-small-librispeech-asr", device_map="auto")
processor = Speech2TextProcessor.from_pretrained("facebook/s2t-small-librispeech-asr")

ds = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")

inputs = processor(ds[0]["audio"]["array"], sampling_rate=ds[0]["audio"]["sampling_rate"], return_tensors="pt").to(model.device)
generated_ids = model.generate(inputs["input_features"], attention_mask=inputs["attention_mask"])

transcription = processor.batch_decode(generated_ids, skip_special_tokens=True)
transcription
['mister quilter is the apostle of the middle classes and we are glad to welcome his gospel']
```

- Multilingual speech translation

  For multilingual speech translation models, `eos_token_id` is used as the `decoder_start_token_id` and
  the target language id is forced as the first generated token. To force the target language id as the first
  generated token, pass the `forced_bos_token_id` parameter to the `generate()` method. The following
  example shows how to translate English speech to French text using the *facebook/s2t-medium-mustc-multilingual-st*
  checkpoint.

```python
from datasets import load_dataset

from transformers import Speech2TextForConditionalGeneration, Speech2TextProcessor

model = Speech2TextForConditionalGeneration.from_pretrained("facebook/s2t-medium-mustc-multilingual-st", device_map="auto")
processor = Speech2TextProcessor.from_pretrained("facebook/s2t-medium-mustc-multilingual-st")

ds = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")

inputs = processor(ds[0]["audio"]["array"], sampling_rate=ds[0]["audio"]["sampling_rate"], return_tensors="pt").to(model.device)
generated_ids = model.generate(
    inputs["input_features"],
    attention_mask=inputs["attention_mask"],
    forced_bos_token_id=processor.tokenizer.lang_code_to_id["fr"],
)

translation = processor.batch_decode(generated_ids, skip_special_tokens=True)
translation
["(Vidéo) Si M. Kilder est l'apossible des classes moyennes, et nous sommes heureux d'être accueillis dans son évangile."]
```

See the [model hub](https://huggingface.co/models?filter=speech_to_text) to look for Speech2Text checkpoints.

## Speech2TextConfig[[transformers.Speech2TextConfig]]

#### transformers.Speech2TextConfig[[transformers.Speech2TextConfig]]

```python
transformers.Speech2TextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, vocab_size: int = 10000, encoder_layers: int = 12, encoder_ffn_dim: int = 2048, encoder_attention_heads: int = 4, decoder_layers: int = 6, decoder_ffn_dim: int = 2048, decoder_attention_heads: int = 4, encoder_layerdrop: float | int = 0.0, decoder_layerdrop: float | int = 0.0, use_cache: bool = True, activation_function: str = 'relu', d_model: int = 256, dropout: float | int = 0.1, attention_dropout: float | int = 0.0, activation_dropout: float | int = 0.0, init_std: float = 0.02, decoder_start_token_id: int = 2, scale_embedding: bool = True, pad_token_id: int | None = 1, bos_token_id: int | None = 0, eos_token_id: int | list[int] | None = 2, max_source_positions: int = 6000, max_target_positions: int = 1024, num_conv_layers: int = 2, conv_kernel_sizes: list[int] | tuple[int, ...] = (5, 5), conv_channels: int = 1024, input_feat_per_channel: int = 80, input_channels: int = 1, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/configuration_speech_to_text.py#L24)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

vocab_size (`int`, *optional*, defaults to `10000`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

encoder_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

encoder_ffn_dim (`int`, *optional*, defaults to `2048`) : Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.

encoder_attention_heads (`int`, *optional*, defaults to `4`) : Number of attention heads for each attention layer in the Transformer encoder.

decoder_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

decoder_ffn_dim (`int`, *optional*, defaults to `2048`) : Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.

decoder_attention_heads (`int`, *optional*, defaults to `4`) : Number of attention heads for each attention layer in the Transformer decoder.

encoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.0`) : The LayerDrop probability for the encoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

decoder_layerdrop (`Union[float, int]`, *optional*, defaults to `0.0`) : The LayerDrop probability for the decoder. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

activation_function (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

d_model (`int`, *optional*, defaults to `256`) : Size of the encoder layers and the pooler layer.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for activations inside the fully connected layer.

init_std (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

decoder_start_token_id (`int`, *optional*, defaults to `2`) : If an encoder-decoder model starts decoding with a different token than `bos`, the id of that token.

scale_embedding (`bool`, *optional*, defaults to `True`) : Whether to scale embeddings by dividing by sqrt(d_model).

pad_token_id (`int`, *optional*, defaults to `1`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `0`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

max_source_positions (`int`, *optional*, defaults to 6000) : The maximum sequence length of log-mel filter-bank features that this model might ever be used with.

max_target_positions (`int`, *optional*, defaults to 1024) : The maximum sequence length that this model might ever be used with. Typically, set this to something large just in case (e.g., 512 or 1024 or 2048).

num_conv_layers (`int`, *optional*, defaults to 2) : Number of 1D convolutional layers in the conv module.

conv_kernel_sizes (`tuple[int]`, *optional*, defaults to `(5, 5)`) : A tuple of integers defining the kernel size of each 1D convolutional layer in the conv module. The length of `conv_kernel_sizes` has to match `num_conv_layers`.

conv_channels (`int`, *optional*, defaults to 1024) : An integer defining the number of output channels of each convolution layers except the final one in the conv module.

input_feat_per_channel (`int`, *optional*, defaults to 80) : An integer specifying the size of feature vector. This is also the dimensions of log-mel filter-bank features.

input_channels (`int`, *optional*, defaults to `1`) : The number of input channels.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Speech2TextModel. It is used to instantiate a Speech To Text
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/s2t-small-librispeech-asr](https://huggingface.co/facebook/s2t-small-librispeech-asr)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Speech2TextConfig, Speech2TextModel

>>> # Initializing a Speech2Text s2t_transformer_s style configuration
>>> configuration = Speech2TextConfig()

>>> # Initializing a model (with random weights) from the s2t_transformer_s style configuration
>>> model = Speech2TextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Speech2TextTokenizer[[transformers.Speech2TextTokenizer]]

#### transformers.Speech2TextTokenizer[[transformers.Speech2TextTokenizer]]

```python
transformers.Speech2TextTokenizer(vocab_file, spm_file, bos_token = '<s>', eos_token = '</s>', pad_token = '<pad>', unk_token = '<unk>', do_upper_case = False, do_lower_case = False, tgt_lang = None, lang_codes = None, additional_special_tokens = None, sp_model_kwargs: dict[str, typing.Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/tokenization_speech_to_text.py#L49)

**Parameters:**

vocab_file (`str`) : File containing the vocabulary.

spm_file (`str`) : Path to the [SentencePiece](https://github.com/google/sentencepiece) model file

bos_token (`str`, *optional*, defaults to `"<s>"`) : The beginning of sentence token.

eos_token (`str`, *optional*, defaults to `"</s>"`) : The end of sentence token.

unk_token (`str`, *optional*, defaults to `"<unk>"`) : The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this token instead.

pad_token (`str`, *optional*, defaults to `"<pad>"`) : The token used for padding, for example when batching sequences of different lengths.

do_upper_case (`bool`, *optional*, defaults to `False`) : Whether or not to uppercase the output when decoding.

do_lower_case (`bool`, *optional*, defaults to `False`) : Whether or not to lowercase the input when tokenizing.

tgt_lang (`str`, *optional*) : A string representing the target language.

sp_model_kwargs (`dict`, *optional*) : Will be passed to the `SentencePieceProcessor.__init__()` method. The [Python wrapper for SentencePiece](https://github.com/google/sentencepiece/tree/master/python) can be used, among other things, to set:  - `enable_sampling`: Enable subword regularization. - `nbest_size`: Sampling parameters for unigram. Invalid for BPE-Dropout.  - `nbest_size = {0,1}`: No sampling is performed. - `nbest_size > 1`: samples from the nbest_size results. - `nbest_size < 0`: assuming that nbest_size is infinite and samples from the all hypothesis (lattice) using forward-filtering-and-backward-sampling algorithm.  - `alpha`: Smoothing parameter for unigram sampling, and dropout probability of merge operations for BPE-dropout. 

- ****kwargs** : Additional keyword arguments passed along to [PreTrainedTokenizer](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.PythonBackend)

Construct an Speech2Text tokenizer.

This tokenizer inherits from [PreTrainedTokenizer](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.PythonBackend) which contains some of the main methods. Users should refer to
the superclass for more information regarding such methods.

#### build_inputs_with_special_tokens[[transformers.Speech2TextTokenizer.build_inputs_with_special_tokens]]

```python
build_inputs_with_special_tokens(token_ids_0, token_ids_1 = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/tokenization_speech_to_text.py#L206)

Build model inputs from a sequence by appending eos_token_id.

#### get_special_tokens_mask[[transformers.Speech2TextTokenizer.get_special_tokens_mask]]

```python
get_special_tokens_mask(token_ids_0: list, token_ids_1: list[int] | None = None, already_has_special_tokens: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/tokenization_speech_to_text.py#L213)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

already_has_special_tokens (`bool`, *optional*, defaults to `False`) : Whether or not the token list is already formatted with special tokens for the model.

**Returns:** `list[int]`

A list of integers in the range [0, 1]: 1 for a special token, 0 for a sequence token.

Retrieve sequence ids from a token list that has no special tokens added. This method is called when adding
special tokens using the tokenizer `prepare_for_model` method.

#### create_token_type_ids_from_sequences[[transformers.Speech2TextTokenizer.create_token_type_ids_from_sequences]]

```python
create_token_type_ids_from_sequences(token_ids_0: list, token_ids_1: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_python.py#L1298)

**Parameters:**

token_ids_0 (`list[int]`) : List of IDs.

token_ids_1 (`list[int]`, *optional*) : Optional second list of IDs for sequence pairs.

**Returns:** `list[int]`

Token type IDs according to the configured pattern.

Create a mask from the two sequences passed to be used in a sequence-pair classification task.

This method dynamically builds the token type IDs based on the tokenizer's configuration attributes:
- `token_type_ids_pattern`: Pattern to use ("all_zeros" or "bert_style")
- `token_type_ids_include_special_tokens`: Whether to account for special tokens in length calculation

Examples:
```python
# All zeros pattern (default, used by RoBERTa, BART, etc.)
tokenizer.token_type_ids_pattern = "all_zeros"
# Returns: [0, 0, 0, ...] for both sequences

# BERT-style pattern (first sequence gets 0s, second gets 1s)
tokenizer.token_type_ids_pattern = "bert_style"
# Returns: [0, 0, 0, ..., 1, 1, 1, ...] for sequence pairs
```

#### save_vocabulary[[transformers.Speech2TextTokenizer.save_vocabulary]]

```python
save_vocabulary(save_directory: str, filename_prefix: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/tokenization_speech_to_text.py#L257)

## Speech2TextFeatureExtractor[[transformers.Speech2TextFeatureExtractor]]

#### transformers.Speech2TextFeatureExtractor[[transformers.Speech2TextFeatureExtractor]]

```python
transformers.Speech2TextFeatureExtractor(feature_size = 80, sampling_rate = 16000, num_mel_bins = 80, padding_value = 0.0, dither = 0.0, do_ceptral_normalize = True, normalize_means = True, normalize_vars = True, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/feature_extraction_speech_to_text.py#L33)

**Parameters:**

feature_size (`int`, *optional*, defaults to 80) : The feature dimension of the extracted features.

sampling_rate (`int`, *optional*, defaults to 16000) : The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).

num_mel_bins (`int`, *optional*, defaults to 80) : Number of Mel-frequency bins.

padding_value (`float`, *optional*, defaults to 0.0) : The value that is used to fill the padding vectors.

dither (`float`, *optional*, defaults to 0.0) : Adds dithering. In other words, adds a small Gaussian noise to each frame. E.g. use 4.0 to add dithering with a normal distribution centered around 0.0 with standard deviation 4.0 (assuming [-32k,+32k] range of kaldi waveform). The value 0.0 means no dithering. Dithering has similar effect as `mel_floor`. It reduces the high log_mel_fbank values for signals with hard-zero sections, when VAD cutoff is present in the signal.

do_ceptral_normalize (`bool`, *optional*, defaults to `True`) : Whether or not to apply utterance-level cepstral mean and variance normalization to extracted features.

normalize_means (`bool`, *optional*, defaults to `True`) : Whether or not to zero-mean normalize the extracted features.

normalize_vars (`bool`, *optional*, defaults to `True`) : Whether or not to unit-variance normalize the extracted features.

Constructs a Speech2Text feature extractor.

This feature extractor inherits from [Speech2TextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor) which contains most of the main methods. Users
should refer to this superclass for more information regarding those methods.

This class extracts mel-filter bank features from raw speech using TorchAudio if installed or using numpy
otherwise, and applies utterance-level cepstral mean and variance normalization to the extracted features.

#### __call__[[transformers.Speech2TextFeatureExtractor.__call__]]

```python
__call__(raw_speech: numpy.ndarray | list[float] | list[numpy.ndarray] | list[list[float]], padding: bool | str | transformers.utils.generic.PaddingStrategy = False, max_length: int | None = None, truncation: bool = False, pad_to_multiple_of: int | None = None, return_tensors: str | transformers.utils.generic.TensorType | None = None, sampling_rate: int | None = None, return_attention_mask: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/feature_extraction_speech_to_text.py#L174)

**Parameters:**

raw_speech (`np.ndarray`, `list[float]`, `list[np.ndarray]`, `list[list[float]]`) : The sequence or batch of sequences to be padded. Each sequence can be a numpy array, a list of float values, a list of numpy arrays or a list of list of float values. Must be mono channel audio, not stereo, i.e. single float per timestep.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `True`) : Select a strategy to pad the returned sequences (according to the model's padding side and padding index) among:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence if provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

max_length (`int`, *optional*) : Maximum length of the returned list and optionally padding length (see above).

truncation (`bool`) : Activates truncation to cut input sequences longer than *max_length* to *max_length*.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value.  This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta), or on TPUs which benefit from having sequence lengths be a multiple of 128.

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific feature_extractor's default.  [What are attention masks?](../glossary#attention-mask)    For Speech2TextTransformer models, `attention_mask` should always be passed for batched inference, to avoid subtle bugs.   

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects.

sampling_rate (`int`, *optional*) : The sampling rate at which the `raw_speech` input was sampled. It is strongly recommended to pass `sampling_rate` at the forward call to prevent silent errors.

padding_value (`float`, *optional*, defaults to 0.0) : The value that is used to fill the padding values / vectors.

Main method to featurize and prepare for the model one or several sequence(s).

## Speech2TextProcessor[[transformers.Speech2TextProcessor]]

#### transformers.Speech2TextProcessor[[transformers.Speech2TextProcessor]]

```python
transformers.Speech2TextProcessor(feature_extractor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/processing_speech_to_text.py#L23)

**Parameters:**

feature_extractor (`Speech2TextFeatureExtractor`) : The feature extractor is a required input.

tokenizer (`Speech2TextTokenizer`) : The tokenizer is a required input.

Constructs a Speech2TextProcessor which wraps a feature extractor and a tokenizer into a single processor.

[Speech2TextProcessor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextProcessor) offers all the functionalities of [Speech2TextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor) and [Speech2TextTokenizer](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextTokenizer). See the
[~Speech2TextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor) and [~Speech2TextTokenizer](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextTokenizer) for more information.

#### __call__[[transformers.Speech2TextProcessor.__call__]]

```python
__call__(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/processing_speech_to_text.py#L27)

**Parameters:**

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

#### from_pretrained[[transformers.Speech2TextProcessor.from_pretrained]]

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

#### save_pretrained[[transformers.Speech2TextProcessor.save_pretrained]]

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

#### batch_decode[[transformers.Speech2TextProcessor.batch_decode]]

```python
batch_decode(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1930)

This method forwards all its arguments to PreTrainedTokenizer's [batch_decode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.batch_decode). Please
refer to the docstring of this method for more information.

#### decode[[transformers.Speech2TextProcessor.decode]]

```python
decode(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L1939)

This method forwards all its arguments to PreTrainedTokenizer's [decode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.decode). Please refer to
the docstring of this method for more information.

## Speech2TextModel[[transformers.Speech2TextModel]]

#### transformers.Speech2TextModel[[transformers.Speech2TextModel]]

```python
transformers.Speech2TextModel(config: Speech2TextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/modeling_speech_to_text.py#L720)

**Parameters:**

config ([Speech2TextConfig](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Speech To Text Text Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Speech2TextModel.forward]]

```python
forward(input_features: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/modeling_speech_to_text.py#L736)

**Parameters:**

input_features (`torch.LongTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [Speech2TextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor). See [Speech2TextFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor.__call__) for details ([Speech2TextProcessor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextProcessor) uses [Speech2TextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor) for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using `SpeechToTextTokenizer`. See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)  SpeechToText uses the `eos_token_id` as the starting token for `decoder_input_ids` generation. If `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see `past_key_values`).

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also be used by default.  If you want to change padding behavior, you should read `modeling_speech_to_text._prepare_decoder_attention_mask` and modify to your needs. See diagram 1 in [the paper](https://huggingface.co/papers/1910.13461) for more information on the default strategy.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Speech2TextConfig](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextConfig)) and inputs.

The [Speech2TextModel](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextModel) forward method, overrides the `__call__` special method.

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
>>> import torch
>>> from transformers import Speech2TextModel, AutoFeatureExtractor
>>> from datasets import load_dataset

>>> model = Speech2TextModel.from_pretrained("facebook/s2t-small-librispeech-asr")
>>> feature_extractor = AutoFeatureExtractor.from_pretrained("facebook/s2t-small-librispeech-asr")
>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> inputs = feature_extractor(
...     ds[0]["audio"]["array"], sampling_rate=ds[0]["audio"]["sampling_rate"], return_tensors="pt"
... )
>>> input_features = inputs.input_features
>>> decoder_input_ids = torch.tensor([[1, 1]]) * model.config.decoder_start_token_id
>>> last_hidden_state = model(input_features, decoder_input_ids=decoder_input_ids).last_hidden_state
>>> list(last_hidden_state.shape)
[1, 2, 256]
```

## Speech2TextForConditionalGeneration[[transformers.Speech2TextForConditionalGeneration]]

#### transformers.Speech2TextForConditionalGeneration[[transformers.Speech2TextForConditionalGeneration]]

```python
transformers.Speech2TextForConditionalGeneration(config: Speech2TextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/modeling_speech_to_text.py#L842)

**Parameters:**

config ([Speech2TextConfig](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Speech2Text Model with a language modeling head. Can be used for summarization.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Speech2TextForConditionalGeneration.forward]]

```python
forward(input_features: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, decoder_input_ids: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: tuple[tuple[torch.FloatTensor]] | None = None, past_key_values: transformers.cache_utils.Cache | None = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/speech_to_text/modeling_speech_to_text.py#L855)

**Parameters:**

input_features (`torch.LongTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) : The tensors corresponding to the input audio features. Audio features can be obtained using [Speech2TextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor). See [Speech2TextFeatureExtractor.__call__()](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor.__call__) for details ([Speech2TextProcessor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextProcessor) uses [Speech2TextFeatureExtractor](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextFeatureExtractor) for processing audios).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

decoder_input_ids (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Indices of decoder input sequence tokens in the vocabulary.  Indices can be obtained using `SpeechToTextTokenizer`. See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are decoder input IDs?](../glossary#decoder-input-ids)  SpeechToText uses the `eos_token_id` as the starting token for `decoder_input_ids` generation. If `past_key_values` is used, optionally only the last `decoder_input_ids` have to be input (see `past_key_values`).

decoder_attention_mask (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) : Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also be used by default.  If you want to change padding behavior, you should read `modeling_speech_to_text._prepare_decoder_attention_mask` and modify to your needs. See diagram 1 in [the paper](https://huggingface.co/papers/1910.13461) for more information on the default strategy.

encoder_outputs (`tuple[tuple[torch.FloatTensor]]`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be input (see `past_key_values`). This is useful if you want more control over how to convert `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value of `inputs_embeds`.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or `tuple(torch.FloatTensor)`

A [Seq2SeqLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Seq2SeqLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Speech2TextConfig](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextConfig)) and inputs.

The [Speech2TextForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/speech_to_text#transformers.Speech2TextForConditionalGeneration) forward method, overrides the `__call__` special method.

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
>>> import torch
>>> from transformers import Speech2TextProcessor, Speech2TextForConditionalGeneration
>>> from datasets import load_dataset

>>> model = Speech2TextForConditionalGeneration.from_pretrained("facebook/s2t-small-librispeech-asr")
>>> processor = Speech2TextProcessor.from_pretrained("facebook/s2t-small-librispeech-asr")

>>> ds = load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")

>>> inputs = processor(
...     ds[0]["audio"]["array"], sampling_rate=ds[0]["audio"]["sampling_rate"], return_tensors="pt"
... )
>>> input_features = inputs.input_features

>>> generated_ids = model.generate(inputs=input_features)

>>> transcription = processor.batch_decode(generated_ids, skip_special_tokens=True)[0]
>>> transcription
'mister quilter is the apostle of the middle classes and we are glad to welcome his gospel'
```
