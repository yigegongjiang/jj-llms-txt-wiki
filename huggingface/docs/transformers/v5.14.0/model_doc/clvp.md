# CLVP

## Overview

The CLVP (Contrastive Language-Voice Pretrained Transformer) model was proposed in [Better speech synthesis through scaling](https://huggingface.co/papers/2305.07243) by James Betker.

The abstract from the paper is the following:

*In recent years, the field of image generation has been revolutionized by the application of autoregressive transformers and DDPMs. These approaches model the process of image generation as a step-wise probabilistic processes and leverage large amounts of compute and data to learn the image distribution. This methodology of improving performance need not be confined to images. This paper describes a way to apply advances in the image generative domain to speech synthesis. The result is TorToise - an expressive, multi-voice text-to-speech system.*

This model was contributed by [Susnato Dhar](https://huggingface.co/susnato).
The original code can be found [here](https://github.com/neonbjb/tortoise-tts).

## Usage tips

1. CLVP is an integral part of the Tortoise TTS model.
2. CLVP can be used to compare different generated speech candidates with the provided text, and the best speech tokens are forwarded to the diffusion model.
3. The use of the `ClvpModelForConditionalGeneration.generate()` method is strongly recommended for tortoise usage.
4. Note that the CLVP model expects the audio to be sampled at 22.05 kHz contrary to other audio models which expects 16 kHz.

## Brief Explanation

- The [ClvpTokenizer](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpTokenizer) tokenizes the text input, and the [ClvpFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpFeatureExtractor) extracts the log mel-spectrogram from the desired audio.
- `ClvpConditioningEncoder` takes those text tokens and audio representations and converts them into embeddings conditioned on the text and audio.
- The [ClvpForCausalLM](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpForCausalLM) uses those embeddings to generate multiple speech candidates.
- Each speech candidate is passed through the speech encoder ([ClvpEncoder](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpEncoder)) which converts them into a vector representation, and the text encoder ([ClvpEncoder](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpEncoder)) converts the text tokens into the same latent space.
- At the end, we compare each speech vector with the text vector to see which speech vector is most similar to the text vector.
- `ClvpModelForConditionalGeneration.generate()` compresses all of the logic described above into a single method.  

Example :

```python
import datasets

from transformers import ClvpModelForConditionalGeneration, ClvpProcessor

# Define the Text and Load the Audio (We are taking an audio example from HuggingFace Hub using `datasets` library).
text = "This is an example text."

ds = datasets.load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
ds = ds.cast_column("audio", datasets.Audio(sampling_rate=22050))
sample = ds[0]["audio"]

# Define processor and model.
processor = ClvpProcessor.from_pretrained("susnato/clvp_dev")
model = ClvpModelForConditionalGeneration.from_pretrained("susnato/clvp_dev", device_map="auto")

# Generate processor output and model output.
processor_output = processor(raw_speech=sample["array"], sampling_rate=sample["sampling_rate"], text=text, return_tensors="pt").to(model.device)
generated_output = model.generate(**processor_output)
```

## ClvpConfig[[transformers.ClvpConfig]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **speech_config** (`dict`, *optional*) --
  Dictionary of configuration options used to initialize CLVP speech encoder.
- **decoder_config** (`dict`, *optional*) --
  Dictionary of configuration options used to initialize [ClvpDecoderConfig](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpDecoderConfig).
- **projection_dim** (`int`, *optional*, defaults to `768`) --
  Dimensionality of text and vision projection layers.
- **logit_scale_init_value** (`float`, *optional*, defaults to `2.6592`) --
  The initial value of the *logit_scale* parameter.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).

This is the configuration class to store the configuration of a ClvpModelForConditionalGeneration. It is used to instantiate a Clvp
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [susnato/clvp_dev](https://huggingface.co/susnato/clvp_dev)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ClvpConfig, ClvpModelForConditionalGeneration

>>> # Initializing a ClvpConfig with susnato/clvp_dev style configuration
>>> configuration = ClvpConfig()

>>> # Initializing a ClvpModelForConditionalGeneration (with random weights) from the susnato/clvp_dev style configuration
>>> model = ClvpModelForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a CLVPConfig from a CLVPTextConfig, CLVPSpeechConfig and a CLVPAutoRegressiveConfig
>>> from transformers import ClvpEncoderConfig, ClvpDecoderConfig

>>> # Initializing a CLVP text, CLVP speech and CLVP decoder configuration
>>> config_text = ClvpEncoderConfig()
>>> config_speech = ClvpEncoderConfig()
>>> decoder_config = ClvpDecoderConfig()

>>> config = ClvpConfig(config_text, config_speech, decoder_config)
```

## ClvpEncoderConfig[[transformers.ClvpEncoderConfig]]

- **vocab_size** (`int`, *optional*, defaults to `256`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `1536`) --
  Dimension of the MLP representations.
- **projection_dim** (`int`, *optional*, defaults to `768`) --
  Dimensionality of text and vision projection layers.
- **num_hidden_layers** (`int`, *optional*, defaults to `20`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **use_rotary_embedding** (`bool`, *optional*, defaults to `True`) --
  Whether to use rotary_embedding or not.
- **use_attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use bias in Query, Key and Value layers during self attention.
- **summary_type** (`str`, *optional*, defaults to `"mean"`) --
  What strategy to use to get pooler_output from the last_hidden_state. `"last"`, `"first"`, `"mean"` and
  `"cls_index"` are supported.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).
- **bos_token_id** (`int`, *optional*, defaults to `255`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `0`) --
  Token id used for end-of-stream in the vocabulary.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.

This is the configuration class to store the configuration of a ClvpModelForConditionalGeneration. It is used to instantiate a Clvp
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [susnato/clvp_dev](https://huggingface.co/susnato/clvp_dev)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ClvpEncoderConfig, ClvpEncoder

>>> # Initializing a ClvpEncoderConfig with susnato/clvp_dev style configuration
>>> encoder_configuration = ClvpEncoderConfig()

>>> # Initializing a ClvpEncoder (with random weights) from the susnato/clvp_dev style configuration
>>> model = ClvpEncoder(encoder_configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ClvpDecoderConfig[[transformers.ClvpDecoderConfig]]

- **vocab_size** (`int`, *optional*, defaults to `8194`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **max_position_embeddings** (`int`, *optional*, defaults to `608`) --
  The maximum sequence length that this model might ever be used with.
- **max_text_tokens** (`int`, *optional*, defaults to 404) --
  The maximum sequence length of text tokens that this model might ever be used with. Similar to
  `n_positions` in `GPT2Config`.
- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `30`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **n_inner** (`int`, *optional*) --
  Dimensionality of the inner feed-forward layers. `None` will set it to 4 times `hidden_size`.
- **num_mel_attn_blocks** (`int`, *optional*, defaults to 6) --
  Denotes the number of self attention layers in `ClvpConditioningEncoder`.
- **activation_function** (`str`, *optional*, defaults to `gelu_new`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **resid_pdrop** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **embd_pdrop** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the embeddings.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **layer_norm_epsilon** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **summary_type** (`string`, *optional*, defaults to `"cls_index"`) --
  Argument used when doing sequence summary.
  Has to be one of the following options:
  - `"last"`: Take the last token hidden state (like XLNet).
  - `"first"`: Take the first token hidden state (like BERT).
  - `"mean"`: Take the mean of all tokens hidden states.
  - `"cls_index"`: Supply a Tensor of classification token position (like GPT/GPT-2).
  - `"attn"`: Not implemented now, use multi-head attention.
- **summary_use_proj** (`bool`, *optional*, defaults to `True`) --
  Whether or not to add a projection after the vector extraction.
- **summary_activation** (`str`, *optional*) --
  Pass `"tanh"` for a tanh activation to the output, any other value will result in no activation.
- **summary_proj_to_labels** (`bool`, *optional*, defaults to `True`) --
  Whether the projection outputs should have `config.num_labels` or `config.hidden_size` classes.
- **summary_first_dropout** (`float`, *optional*, defaults to 0.1) --
  The dropout ratio to be used after the projection and activation.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **bos_token_id** (`int`, *optional*, defaults to `8192`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `8193`) --
  Token id used for end-of-stream in the vocabulary.
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **feature_size** (`int`, *optional*, defaults to 80) --
  The feature dimension of the extracted mel features. This value is used in `ClvpConditioningEncoder`.
- **use_attention_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in Query, Key and Value layers during self attention.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).
- **decoder_fixing_codes** (`list`, *optional*, defaults to `[83, 45, 45, 248]`) --
  These values are used in the method `fix_speech_decoder_output` to fix decoder generated outputs.
- **add_cross_attention** (`bool`, *optional*, defaults to `False`) --
  Whether cross-attention layers should be added to the model.

This is the configuration class to store the configuration of a ClvpModelForConditionalGeneration. It is used to instantiate a Clvp
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [susnato/clvp_dev](https://huggingface.co/susnato/clvp_dev)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import ClvpDecoderConfig, ClvpDecoder

>>> # Initializing a ClvpDecoderConfig with susnato/clvp_dev style configuration
>>> decoder_configuration = ClvpDecoderConfig()

>>> # Initializing a ClvpDecoder (with random weights) from the susnato/clvp_dev style configuration
>>> model = ClvpDecoder(decoder_configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## ClvpTokenizer[[transformers.ClvpTokenizer]]

'"}, {"name": "eos_token", "val": " = '[STOP]'"}, {"name": "pad_token", "val": " = '[STOP]'"}, {"name": "add_prefix_space", "val": " = False"}, {"name": "**kwargs", "val": ""}]}>
- **vocab_file** (`str`) --
  Path to the vocabulary file.
- **merges_file** (`str`) --
  Path to the merges file.
- **errors** (`str`, *optional*, defaults to `"replace"`) --
  Paradigm to follow when decoding bytes to UTF-8. See
  [bytes.decode](https://docs.python.org/3/library/stdtypes.html#bytes.decode) for more information.
- **unk_token** (`str`, *optional*, defaults to `"[UNK]"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **bos_token** (`str`, *optional*, defaults to `"<|endoftext|>"`) --
  The beginning of sequence token.
- **eos_token** (`str`, *optional*, defaults to `"[STOP]"`) --
  The end of sequence token.
- **pad_token** (`str`, *optional*, defaults to `"[STOP]"`) --
  The pad token of the sequence.
- **add_prefix_space** (`bool`, *optional*, defaults to `False`) --
  Whether or not to add an initial space to the input. This allows to treat the leading word just as any
  other word. (CLVP tokenizer detect beginning of words by the preceding space).

Construct a CLVP tokenizer. Based on byte-level Byte-Pair-Encoding.

This tokenizer has been trained to treat spaces like parts of the tokens (a bit like sentencepiece) so a word will

be encoded differently whether it is at the beginning of the sentence (without space) or not:

```python
>>> from transformers import ClvpTokenizer

>>> tokenizer = ClvpTokenizer.from_pretrained("susnato/clvp_dev")
>>> tokenizer("Hello world")["input_ids"]
[62, 84, 28, 2, 179, 79]

>>> tokenizer(" Hello world")["input_ids"]
[2, 62, 84, 28, 2, 179, 79]
```

You can get around that behavior by passing `add_prefix_space=True` when instantiating this tokenizer or when you
call it on some text, but since the model was not pretrained this way, it might yield a decrease in performance.

When used with `is_split_into_words=True`, this tokenizer will add a space before each word (even the first one).

This tokenizer inherits from [PreTrainedTokenizer](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.PythonBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods.

- **save_directory** (`str`) --
  The directory in which to save the vocabulary.
- **filename_prefix** (`str`, *optional*) --
  An optional prefix to add to the named of the saved files.`tuple[str, ...]`Paths to the files saved, or empty tuple if no files saved.

Default implementation for common vocabulary saving patterns.
Saves self.encoder/self.vocab as JSON, optionally with self.bpe_ranks as merges.
Returns empty tuple if no vocabulary exists.

Override this method if your tokenizer needs custom saving logic (e.g., SentencePiece models,
multiple vocabulary files, or special file formats).

## ClvpFeatureExtractor[[transformers.ClvpFeatureExtractor]]

- **feature_size** (`int`, *optional*, defaults to 80) --
  The feature dimension of the extracted features.
- **sampling_rate** (`int`, *optional*, defaults to 22050) --
  The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).
- **default_audio_length** (`int`, *optional*, defaults to 6) --
  The default length of raw audio in seconds. If `max_length` is not set during `__call__` then it will
  automatically be set to default_audio_length * `self.sampling_rate`.
- **hop_length** (`int`, *optional*, defaults to 256) --
  Length of the overlapping windows for the STFT used to obtain the Mel Frequency coefficients.
- **chunk_length** (`int`, *optional*, defaults to 30) --
  The maximum number of chunks of `sampling_rate` samples used to trim and pad longer or shorter audio
  sequences.
- **n_fft** (`int`, *optional*, defaults to 1024) --
  Size of the Fourier transform.
- **padding_value** (`float`, *optional*, defaults to 0.0) --
  Padding value used to pad the audio. Should correspond to silences.
- **mel_norms** (`list` of length `feature_size`, *optional*) --
  If `mel_norms` is provided then it will be used to normalize the log-mel spectrograms along each
  mel-filter.
- **return_attention_mask** (`bool`, *optional*, defaults to `False`) --
  Whether to return the attention mask. If left to the default, it will return the attention mask.

  [What are attention masks?](../glossary#attention-mask)

Constructs a CLVP feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

This class extracts log-mel-spectrogram features from raw speech using a custom numpy implementation of the `Short
Time Fourier Transform` which should match pytorch's `torch.stft` equivalent.

- **raw_speech** (`np.ndarray`, `list[float]`, `list[np.ndarray]`, `list[list[float]]`) --
  The sequence or batch of sequences to be padded. Each sequence can be a numpy array, a list of float
  values, a list of numpy arrays or a list of list of float values. Must be mono channel audio, not
  stereo, i.e. single float per timestep.
- **sampling_rate** (`int`, *optional*) --
  The sampling rate at which the `raw_speech` input was sampled. It is strongly recommended to pass
  `sampling_rate` at the forward call to prevent silent errors and allow automatic speech recognition
  pipeline.
- **truncation** (`bool`, *optional*, default to `True`) --
  Activates truncation to cut input sequences longer than *max_length* to *max_length*.
- **pad_to_multiple_of** (`int`, *optional*) --
  If set will pad the sequence to a multiple of the provided value.

  This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability
  `>= 7.5` (Volta), or on TPUs which benefit from having sequence lengths be a multiple of 128.
- **return_attention_mask** (`bool`, *optional*, defaults to `True`) --
  Whether to return the attention mask. If left to the default, it will return the attention mask.

  [What are attention masks?](../glossary#attention-mask)
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors instead of list of python integers. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return Numpy `np.ndarray` objects.
- **padding_value** (`float`, *optional*, defaults to 0.0) --
  The value that is used to fill the padding values / vectors.
- **max_length** (`int`, *optional*) --
  The maximum input length of the inputs.

`ClvpFeatureExtractor` is used to extract various voice specific properties such as the pitch and tone of the
voice, speaking speed, and even speaking defects like a lisp or stuttering from a sample voice or `raw_speech`.

First the voice is padded or truncated in a way such that it becomes a waveform of `self.default_audio_length`
seconds long and then the log-mel spectrogram is extracted from it.

## ClvpProcessor[[transformers.ClvpProcessor]]

- **feature_extractor** (`ClvpFeatureExtractor`) --
  The feature extractor is a required input.
- **tokenizer** (`ClvpTokenizer`) --
  The tokenizer is a required input.
Constructs a ClvpProcessor which wraps a feature extractor and a tokenizer into a single processor.

[ClvpProcessor](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpProcessor) offers all the functionalities of [ClvpFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpFeatureExtractor) and [ClvpTokenizer](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpTokenizer). See the
[~ClvpFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpFeatureExtractor) and [~ClvpTokenizer](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpTokenizer) for more information.

- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.

This method forwards all its arguments to PreTrainedTokenizer's [decode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.decode). Please refer to
the docstring of this method for more information.

This method forwards all its arguments to PreTrainedTokenizer's [batch_decode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.batch_decode). Please
refer to the docstring of this method for more information.

## ClvpModelForConditionalGeneration[[transformers.ClvpModelForConditionalGeneration]]

- **config** ([ClvpConfig](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The composite CLVP model with a text encoder, speech encoder and speech decoder model.

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
- **input_features** (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  [ClvpFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpFeatureExtractor). See [ClvpFeatureExtractor.__call__()](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpFeatureExtractor.__call__) for details ([ClvpProcessor](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpProcessor) uses
  [ClvpFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpFeatureExtractor) for processing audios).
- **conditioning_encoder_inputs_embeds** (`torch.FloatTensor`, *optional*) --
  inputs_embeds for `ClvpConditioningEncoder`. Can be used in place of `input_ids`.
- **text_encoder_inputs_embeds** (`torch.FloatTensor`, *optional*) --
  inputs_embeds for the text encoder model passed in place of `input_ids`.
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **return_loss** (`bool`, *optional*) --
  Whether or not to return the contrastive loss.`ClvpOutput` or `tuple(torch.FloatTensor)`A `ClvpOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClvpConfig](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpConfig)) and inputs.
The [ClvpModelForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpModelForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Contrastive loss for speech-text similarity.
- **speech_ids** (`torch.LongTensor`, *optional*) -- speech_ids (or speech candidates) generated by the `ClvpForCausalLM` model.
- **logits_per_speech** (`torch.FloatTensor` of shape `(speech_batch_size, text_batch_size)`) -- The scaled dot product scores between `speech_embeds` and `text_embeds`. This represents the speech-text
  similarity scores.
- **logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, speech_batch_size)`) -- The scaled dot product scores between `text_embeds` and `speech_embeds`. This represents the text-speech
  similarity scores.
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The text embeddings obtained by applying the projection layer to the pooled output of the text encoder
  model.
- **speech_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The speech embeddings obtained by applying the projection layer to the pooled output of the speech encoder
  model.
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The pooled output of the `last_hidden_state` of the text encoder Model.
- **speech_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The pooled output of the `last_hidden_state` of the speech encoder Model.
- **decoder_hidden_states** (`torch.FloatTensor`, *optional*) -- The hidden states of the decoder model.
- **text_encoder_hidden_states** (`torch.FloatTensor`, *optional*) -- The hidden states of the text encoder model.
- **speech_encoder_hidden_states** (`torch.FloatTensor`, *optional*) -- The hidden states of the speech encoder model.

Examples:

```python
>>> import datasets
>>> from transformers import ClvpProcessor, ClvpModelForConditionalGeneration

>>> # Define the Text and Load the Audio (We are taking an audio example from HuggingFace Hub using `datasets` library)
>>> text = "This is an example text."

>>> ds = datasets.load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", datasets.Audio(sampling_rate=22050))
>>> audio = ds.sort("id")["audio"][0]
>>> audio_sample, sr = audio["array"], audio["sampling_rate"]

>>> # Define processor and model
>>> processor = ClvpProcessor.from_pretrained("susnato/clvp_dev")
>>> model = ClvpModelForConditionalGeneration.from_pretrained("susnato/clvp_dev")

>>> # processor outputs and model outputs
>>> processor_output = processor(raw_speech=audio_sample, sampling_rate=sr, text=text, return_tensors="pt")
>>> outputs = model(
...     input_ids=processor_output["input_ids"],
...     input_features=processor_output["input_features"],
...     return_dict=True,
... )
```

- **input_ids** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Input text Tokens. Processed from the [ClvpTokenizer](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpTokenizer).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding text token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **generation_config** (`~generation.GenerationConfig`, *optional*) --
  The generation configuration to be used as base parametrization for the generation call. `**kwargs`
  passed to generate matching the attributes of `generation_config` will override them. If
  `generation_config` is not provided, the default will be used, which had the following loading
  priority: 1) from the `generation_config.json` model file, if it exists; 2) from the model
  configuration. Please note that unspecified parameters will inherit [GenerationConfig](/docs/transformers/v5.14.0/en/main_classes/text_generation#transformers.GenerationConfig)'s
  default values, whose documentation should be checked to parameterize generation.
- **pad_to_max_mel_tokens** (`int`, *optional*) --
  Pads generated speech_ids to the specified value. This is to implement the same logic from the official
  repo, link: https://github.com/neonbjb/tortoise-tts/blob/80f89987a5abda5e2b082618cd74f9c7411141dc/tortoise/api.py#L430
  and to make sure the logits are same.
  This does not affect generation quality so please don't consider using it since it is less efficient.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of decoder model, text encoder and speech encoder models.`ClvpOutput` or tupleA `ClvpOutput` (if `return_dict_in_generate=True` or when
`config.return_dict_in_generate=True`) or a tuple.

Generate method for `ClvpModelForConditionalGeneration`, this method calls the `generate` method of
`ClvpForCausalLM` and then uses those generated `speech_ids` to process `text_embeds` and `speech_embeds` using
`ClvpEncoder`.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **text_encoder_inputs_embeds** (`torch.FloatTensor`, *optional*) --
  inputs_embeds for the text encoder model passed in place of `input_ids`.
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)`ClvpEncoderOutput` or `tuple(torch.FloatTensor)`A `ClvpEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClvpConfig](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpConfig)) and inputs.

This method can be used to extract text_embeds from a text. The text embeddings obtained by applying the
projection layer to the pooled output of the CLVP text encoder model.

- **embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when model is initialized with `with_projection=True`) -- The embeddings obtained by applying the projection layer to the pooler_output.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- The hidden state of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Pooled output of the `last_hidden_state`.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import ClvpProcessor, ClvpModelForConditionalGeneration

>>> # Define the Text
>>> text = "This is an example text."

>>> # Define processor and model
>>> processor = ClvpProcessor.from_pretrained("susnato/clvp_dev")
>>> model = ClvpModelForConditionalGeneration.from_pretrained("susnato/clvp_dev")

>>> # Generate processor output and text embeds
>>> processor_output = processor(text=text, return_tensors="pt")
>>> text_embeds = model.get_text_features(input_ids=processor_output["input_ids"])
```

- **speech_ids** (`torch.LongTensor` of shape `(batch_size, num_speech_ids)`, *optional*) --
  Speech Tokens. Padding will be ignored by default should you provide it. If speech_ids are provided
  then input_ids and input_features will be automatically ignored.
- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Input text Tokens. Processed from the [ClvpTokenizer](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpTokenizer). If speech_ids is not provided, then input_ids
  and input_features will be used.
- **conditioning_encoder_inputs_embeds** (`torch.FloatTensor`, *optional*) --
  inputs_embeds for `ClvpConditioningEncoder`. Can be used in place of `input_ids`.
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding speech token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **generation_config** (`GenerationConfig`, *optional*) --
  generation config to control the generation of speech_ids if they are not provided.`torch.FloatTensor` of shape `(batch_size, output_dim)`The speech embeddings obtained by applying the projection layer to the pooled output of the CLVP Speech
Model.

This method can be used to extract speech_embeds. The speech embeddings are obtained by applying the speech
model on speech_ids. If speech_ids is not present but both input_ids and input_features are given then the
decoder model will be used to first generate the speech_ids and then applying the speech model.

Examples:

```python
>>> import datasets
>>> from transformers import ClvpProcessor, ClvpModelForConditionalGeneration

>>> # Define the Text and Load the Audio (We are taking an audio example from HuggingFace Hub using `datasets` library)
>>> text = "This is an example text."
>>> ds = datasets.load_dataset("hf-internal-testing/librispeech_asr_dummy", "clean", split="validation")
>>> ds = ds.cast_column("audio", datasets.Audio(sampling_rate=22050))
>>> audio = ds.sort("id")["audio"][0]
>>> audio_sample, sr = audio["array"], audio["sampling_rate"]

>>> # Define processor and model
>>> processor = ClvpProcessor.from_pretrained("susnato/clvp_dev")
>>> model = ClvpModelForConditionalGeneration.from_pretrained("susnato/clvp_dev")

>>> # Generate processor output and model output
>>> processor_output = processor(raw_speech=audio_sample, sampling_rate=sr, text=text, return_tensors="pt")
>>> speech_embeds = model.get_speech_features(
...     input_ids=processor_output["input_ids"], input_features=processor_output["input_features"]
... )
```

## ClvpForCausalLM[[transformers.ClvpForCausalLM]]

- **config** ([ClvpForCausalLM](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpForCausalLM)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The CLVP decoder model with a language modelling head on top.

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
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for language modeling. Note that the labels **are shifted** inside the model, i.e. you can set
  `labels = input_ids` Indices are selected in `[-100, 0, ..., config.vocab_size]` All labels set to `-100`
  are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size]`
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).[CausalLMOutputWithCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithCrossAttentions) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithCrossAttentions) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClvpConfig](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpConfig)) and inputs.
The [ClvpForCausalLM](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Cross attentions weights after the attention softmax, used to compute the weighted average in the
  cross-attention heads.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.

## ClvpModel[[transformers.ClvpModel]]

- **config** ([ClvpDecoderConfig](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpDecoderConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Clvp Model outputting raw hidden-states without any specific head on top.

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
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
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
  `past_key_values`).[BaseModelOutputWithPastAndCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPastAndCrossAttentions) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPastAndCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPastAndCrossAttentions) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClvpConfig](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpConfig)) and inputs.
The [ClvpModel](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
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

## ClvpEncoder[[transformers.ClvpEncoder]]

- **config** -- ClvpConfig

Transformer encoder consisting of `config.num_hidden_layers` self attention layers. Each layer is a
`ClvpEncoderLayer`.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **inputs_embeds** (`torch.LongTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)`ClvpEncoderOutput` or `tuple(torch.FloatTensor)`A `ClvpEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClvpConfig](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpConfig)) and inputs.
The [ClvpEncoder](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpEncoder) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim)`, *optional*, returned when model is initialized with `with_projection=True`) -- The embeddings obtained by applying the projection layer to the pooler_output.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- The hidden state of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Pooled output of the `last_hidden_state`.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## ClvpDecoder[[transformers.ClvpDecoder]]

Transformer decoder consisting of *config.num_hidden_layers* layers. Each layer is a `ClvpDecoderLayer`

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.FloatTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
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
  `past_key_values`).[BaseModelOutputWithPastAndCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPastAndCrossAttentions) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPastAndCrossAttentions](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPastAndCrossAttentions) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ClvpConfig](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpConfig)) and inputs.
The [ClvpDecoder](/docs/transformers/v5.14.0/en/model_doc/clvp#transformers.ClvpDecoder) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
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
