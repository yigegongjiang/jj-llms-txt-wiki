# MusicGen Melody

## Overview

The MusicGen Melody model was proposed in [Simple and Controllable Music Generation](https://huggingface.co/papers/2306.05284) by Jade Copet, Felix Kreuk, Itai Gat, Tal Remez, David Kant, Gabriel Synnaeve, Yossi Adi and Alexandre Défossez.

MusicGen Melody is a single stage auto-regressive Transformer model capable of generating high-quality music samples conditioned on text descriptions or audio prompts. The text descriptions are passed through a frozen text encoder model to obtain a sequence of hidden-state representations. MusicGen is then trained to predict discrete audio tokens, or *audio codes*, conditioned on these hidden-states. These audio tokens are then decoded using an audio compression model, such as EnCodec, to recover the audio waveform.

Through an efficient token interleaving pattern, MusicGen does not require a self-supervised semantic representation of the text/audio prompts, thus eliminating the need to cascade multiple models to predict a set of codebooks (e.g. hierarchically or upsampling). Instead, it is able to generate all the codebooks in a single forward pass.

The abstract from the paper is the following:

*We tackle the task of conditional music generation. We introduce MusicGen, a single Language Model (LM) that operates over several streams of compressed discrete music representation, i.e., tokens. Unlike prior work, MusicGen is comprised of a single-stage transformer LM together with efficient token interleaving patterns, which eliminates the need for cascading several models, e.g., hierarchically or upsampling. Following this approach, we demonstrate how MusicGen can generate high-quality samples, while being conditioned on textual description or melodic features, allowing better controls over the generated output. We conduct extensive empirical evaluation, considering both automatic and human studies, showing the proposed approach is superior to the evaluated baselines on a standard text-to-music benchmark. Through ablation studies, we shed light over the importance of each of the components comprising MusicGen.*

This model was contributed by [ylacombe](https://huggingface.co/ylacombe). The original code can be found [here](https://github.com/facebookresearch/audiocraft). The pre-trained checkpoints can be found on the [Hugging Face Hub](https://huggingface.co/models?sort=downloads&search=facebook%2Fmusicgen).

## Difference with [MusicGen](https://huggingface.co/docs/transformers/main/en/model_doc/musicgen)

There are two key differences with MusicGen:

1. The audio prompt is used here as a conditional signal for the generated audio sample, whereas it's used for audio continuation in [MusicGen](https://huggingface.co/docs/transformers/main/en/model_doc/musicgen).
2. Conditional text and audio signals are concatenated to the decoder's hidden states instead of being used as a cross-attention signal, as in MusicGen.

## Generation

MusicGen Melody is compatible with two generation modes: greedy and sampling. In practice, sampling leads to significantly better results than greedy, thus we encourage sampling mode to be used where possible. Sampling is enabled by default, and can be explicitly specified by setting `do_sample=True` in the call to `MusicgenMelodyForConditionalGeneration.generate()`, or by overriding the model's generation config (see below).

Transformers supports both mono (1-channel) and stereo (2-channel) variants of MusicGen Melody. The mono channel versions generate a single set of codebooks. The stereo versions generate 2 sets of codebooks, 1 for each channel (left/right), and each set of codebooks is decoded independently through the audio compression model. The audio streams for each channel are combined to give the final stereo output.

### Audio Conditional Generation

The model can generate an audio sample conditioned on a text and an audio prompt through use of the [MusicgenMelodyProcessor](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyProcessor) to pre-process the inputs.

In the following examples, we load an audio file using the 🤗 Datasets library, which can be pip installed through the command below:

```bash
pip install --upgrade pip
pip install datasets[audio]
```

The audio file we are about to use is loaded as follows:

```python
from datasets import load_dataset

dataset = load_dataset("sanchit-gandhi/gtzan", split="train", streaming=True)
sample = next(iter(dataset))["audio"]
```

The audio prompt should ideally be free of the low-frequency signals usually produced by instruments such as drums and bass. The [Demucs](https://github.com/adefossez/demucs/tree/main) model can be used to separate vocals and other signals from the drums and bass components.

If you wish to use Demucs, you first need to follow the installation steps [here](https://github.com/adefossez/demucs/tree/main?tab=readme-ov-file#for-musicians) before using the following snippet:

```python
import torch
from demucs import pretrained
from demucs.apply import apply_model
from demucs.audio import convert_audio

wav = torch.tensor(sample["array"]).to(torch.float32)

demucs = pretrained.get_model('htdemucs')

wav = convert_audio(wav[None], sample["sampling_rate"], demucs.samplerate, demucs.audio_channels)
wav = apply_model(demucs, wav[None])
```

You can then use the following snippet to generate music:

```python
from transformers import AutoProcessor, MusicgenMelodyForConditionalGeneration

processor = AutoProcessor.from_pretrained("facebook/musicgen-melody")
model = MusicgenMelodyForConditionalGeneration.from_pretrained("facebook/musicgen-melody", device_map="auto")

inputs = processor(
    audio=wav,
    sampling_rate=demucs.samplerate,
    text=["80s blues track with groovy saxophone"],
    padding=True,
    return_tensors="pt",
)
audio_values = model.generate(**inputs, do_sample=True, guidance_scale=3, max_new_tokens=256)
```

You can also pass the audio signal directly without using Demucs, although the quality of the generation will probably be degraded:

```python
from transformers import AutoProcessor, MusicgenMelodyForConditionalGeneration

processor = AutoProcessor.from_pretrained("facebook/musicgen-melody")
model = MusicgenMelodyForConditionalGeneration.from_pretrained("facebook/musicgen-melody", device_map="auto")

inputs = processor(
    audio=sample["array"],
    sampling_rate=sample["sampling_rate"],
    text=["80s blues track with groovy saxophone"],
    padding=True,
    return_tensors="pt",
)
audio_values = model.generate(**inputs, do_sample=True, guidance_scale=3, max_new_tokens=256)
```

The audio outputs are a three-dimensional Torch tensor of shape `(batch_size, num_channels, sequence_length)`. To listen to the generated audio samples, you can either play them in an ipynb notebook:

```python
from IPython.display import Audio

sampling_rate = model.config.audio_encoder.sampling_rate
Audio(audio_values[0].numpy(), rate=sampling_rate)
```

Or save them as a `.wav` file using a third-party library, e.g. `soundfile`:

```python
import soundfile as sf

sampling_rate = model.config.audio_encoder.sampling_rate
sf.write("musicgen_out.wav", audio_values[0].T.numpy(), sampling_rate)
```

### Text-only Conditional Generation

The same [MusicgenMelodyProcessor](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyProcessor) can be used to pre-process a text-only prompt.

```python
from transformers import AutoProcessor, MusicgenMelodyForConditionalGeneration

processor = AutoProcessor.from_pretrained("facebook/musicgen-melody")
model = MusicgenMelodyForConditionalGeneration.from_pretrained("facebook/musicgen-melody", device_map="auto")

inputs = processor(
    text=["80s pop track with bassy drums and synth", "90s rock song with loud guitars and heavy drums"],
    padding=True,
    return_tensors="pt",
)
audio_values = model.generate(**inputs, do_sample=True, guidance_scale=3, max_new_tokens=256)
```

The `guidance_scale` is used in classifier free guidance (CFG), setting the weighting between the conditional logits (which are predicted from the text prompts) and the unconditional logits (which are predicted from an unconditional or 'null' prompt). Higher guidance scale encourages the model to generate samples that are more closely linked to the input prompt, usually at the expense of poorer audio quality. CFG is enabled by setting `guidance_scale > 1`. For best results, use `guidance_scale=3` (default).

You can also generate in batch:

```python

from transformers import AutoProcessor, MusicgenMelodyForConditionalGeneration

processor = AutoProcessor.from_pretrained("facebook/musicgen-melody")
model = MusicgenMelodyForConditionalGeneration.from_pretrained("facebook/musicgen-melody", device_map="auto")

# take the first quarter of the audio sample
sample_1 = sample["array"][: len(sample["array"]) // 4]

# take the first half of the audio sample
sample_2 = sample["array"][: len(sample["array"]) // 2]

inputs = processor(
    audio=[sample_1, sample_2],
    sampling_rate=sample["sampling_rate"],
    text=["80s blues track with groovy saxophone", "90s rock song with loud guitars and heavy drums"],
    padding=True,
    return_tensors="pt",
)
audio_values = model.generate(**inputs, do_sample=True, guidance_scale=3, max_new_tokens=256)
```

### Unconditional Generation

The inputs for unconditional (or 'null') generation can be obtained through the method [MusicgenMelodyProcessor.get_unconditional_inputs()](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyProcessor.get_unconditional_inputs):

```python
from transformers import MusicgenMelodyForConditionalGeneration, MusicgenMelodyProcessor

model = MusicgenMelodyForConditionalGeneration.from_pretrained("facebook/musicgen-melody", device_map="auto")
unconditional_inputs = MusicgenMelodyProcessor.from_pretrained("facebook/musicgen-melody").get_unconditional_inputs(num_samples=1)

audio_values = model.generate(**unconditional_inputs, do_sample=True, max_new_tokens=256)
```

### Generation Configuration

The default parameters that control the generation process, such as sampling, guidance scale and number of generated tokens, can be found in the model's generation config, and updated as desired:

```python
from transformers import MusicgenMelodyForConditionalGeneration

model = MusicgenMelodyForConditionalGeneration.from_pretrained("facebook/musicgen-melody", device_map="auto")

# inspect the default generation config
model.generation_config

# increase the guidance scale to 4.0
model.generation_config.guidance_scale = 4.0

# decrease the max length to 256 tokens
model.generation_config.max_length = 256
```

Note that any arguments passed to the generate method will **supersede** those in the generation config, so setting `do_sample=False` in the call to generate will supersede the setting of `model.generation_config.do_sample` in the generation config.

## Model Structure

The MusicGen model can be de-composed into three distinct stages:

1. Text encoder: maps the text inputs to a sequence of hidden-state representations. The pre-trained MusicGen models use a frozen text encoder from either T5 or Flan-T5.
2. MusicGen Melody decoder: a language model (LM) that auto-regressively generates audio tokens (or codes) conditional on the encoder hidden-state representations
3. Audio decoder: used to recover the audio waveform from the audio tokens predicted by the decoder.

Thus, the MusicGen model can either be used as a standalone decoder model, corresponding to the class [MusicgenMelodyForCausalLM](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyForCausalLM), or as a composite model that includes the text encoder and audio encoder, corresponding to the class [MusicgenMelodyForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyForConditionalGeneration). If only the decoder needs to be loaded from the pre-trained checkpoint, it can be loaded by first specifying the correct config, or be accessed through the `.decoder` attribute of the composite model:

```python
from transformers import AutoConfig, MusicgenMelodyForCausalLM, MusicgenMelodyForConditionalGeneration

# Option 1: get decoder config and pass to `.from_pretrained`
decoder_config = AutoConfig.from_pretrained("facebook/musicgen-melody").decoder
decoder = MusicgenMelodyForCausalLM.from_pretrained("facebook/musicgen-melody", **decoder_config.to_dict(), device_map="auto")

# Option 2: load the entire composite model, but only return the decoder
decoder = MusicgenMelodyForConditionalGeneration.from_pretrained("facebook/musicgen-melody", device_map="auto").decoder
```

Since the text encoder and audio encoder models are frozen during training, the MusicGen decoder [MusicgenMelodyForCausalLM](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyForCausalLM) can be trained standalone on a dataset of encoder hidden-states and audio codes. For inference, the trained decoder can be combined with the frozen text encoder and audio encoder to recover the composite [MusicgenMelodyForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyForConditionalGeneration) model.

## Checkpoint Conversion

- After downloading the original checkpoints from [here](https://github.com/facebookresearch/audiocraft/blob/main/docs/MUSICGEN.md#importing--exporting-models), you can convert them using the **conversion script** available at `src/transformers/models/musicgen_melody/convert_musicgen_melody_transformers.py` with the following command:

```bash
python src/transformers/models/musicgen_melody/convert_musicgen_melody_transformers.py \
    --checkpoint="facebook/musicgen-melody" --pytorch_dump_folder /output/path 
```

Tips:

* MusicGen is trained on the 32kHz checkpoint of Encodec. You should ensure you use a compatible version of the Encodec model.
* Sampling mode tends to deliver better results than greedy - you can toggle sampling with the variable `do_sample` in the call to `MusicgenMelodyForConditionalGeneration.generate()`

## MusicgenMelodyDecoderConfig[[transformers.MusicgenMelodyDecoderConfig]]

- **vocab_size** (`int`, *optional*, defaults to `2048`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **max_position_embeddings** (`int`, *optional*, defaults to `2048`) --
  The maximum sequence length that this model might ever be used with.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **ffn_dim** (`int`, *optional*, defaults to `4096`) --
  Dimension of the MLP representations.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **layerdrop** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The LayerDrop probability. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for
  more details.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **activation_function** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for activations inside the fully connected layer.
- **initializer_factor** (`float`, *optional*, defaults to `0.02`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).
- **scale_embedding** (`bool`, *optional*, defaults to `False`) --
  Whether to scale embeddings by dividing by sqrt(d_model).
- **num_codebooks** (`int`, *optional*, defaults to `4`) --
  The number of parallel codebooks used by the model.
- **audio_channels** (`int`, *optional*, defaults to 1) --
  Number of audio channels used by the model (either mono or stereo). Stereo models generate a separate
  audio stream for the left/right output channels. Mono models generate a single audio stream output.
- **pad_token_id** (`int`, *optional*, defaults to `2048`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `2048`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*) --
  Token id used for end-of-stream in the vocabulary.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **is_decoder** (`bool`, *optional*, defaults to `False`) --
  Whether the model is used as a decoder or not. If `False`, the model is used as an encoder.
- **add_cross_attention** (`bool`, *optional*, defaults to `False`) --
  Whether cross-attention layers should be added to the model.

This is the configuration class to store the configuration of a MusicgenMelodyModel. It is used to instantiate a Musicgen Melody
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/musicgen-melody](https://huggingface.co/facebook/musicgen-melody)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## MusicgenMelodyProcessor[[transformers.MusicgenMelodyProcessor]]

- **feature_extractor** (`MusicgenMelodyFeatureExtractor`) --
  The feature extractor is a required input.
- **tokenizer** (`T5Tokenizer`) --
  The tokenizer is a required input.
Constructs a MusicgenMelodyProcessor which wraps a feature extractor and a tokenizer into a single processor.

[MusicgenMelodyProcessor](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyProcessor) offers all the functionalities of [MusicgenMelodyFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyFeatureExtractor) and [T5Tokenizer](/docs/transformers/v5.14.0/en/model_doc/t5#transformers.T5Tokenizer). See the
[~MusicgenMelodyFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyFeatureExtractor) and [~T5Tokenizer](/docs/transformers/v5.14.0/en/model_doc/t5#transformers.T5Tokenizer) for more information.

- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.

- **num_samples** (int, *optional*) --
  Number of audio samples to unconditionally generate.

Helper function to get null inputs for unconditional generation, enabling the model to be used without the
feature extractor or tokenizer.

Example:
```python
>>> from transformers import MusicgenMelodyForConditionalGeneration, MusicgenMelodyProcessor

>>> model = MusicgenMelodyForConditionalGeneration.from_pretrained("facebook/musicgen-melody")

>>> # get the unconditional (or 'null') inputs for the model
>>> processor = MusicgenMelodyProcessor.from_pretrained("facebook/musicgen-melody")
>>> unconditional_inputs = processor.get_unconditional_inputs(num_samples=1)

>>> audio_samples = model.generate(**unconditional_inputs, max_new_tokens=256)
```

## MusicgenMelodyFeatureExtractor[[transformers.MusicgenMelodyFeatureExtractor]]

- **feature_size** (`int`, *optional*, defaults to 12) --
  The feature dimension of the extracted features.
- **sampling_rate** (`int`, *optional*, defaults to 32000) --
  The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).
- **hop_length** (`int`, *optional*, defaults to 4096) --
  Length of the overlapping windows for the STFT used to obtain the Mel Frequency coefficients.
- **chunk_length** (`int`, *optional*, defaults to 30) --
  The maximum number of chunks of `sampling_rate` samples used to trim and pad longer or shorter audio
  sequences.
- **n_fft** (`int`, *optional*, defaults to 16384) --
  Size of the Fourier transform.
- **num_chroma** (`int`, *optional*, defaults to 12) --
  Number of chroma bins to use.
- **padding_value** (`float`, *optional*, defaults to 0.0) --
  Padding value used to pad the audio.
- **return_attention_mask** (`bool`, *optional*, defaults to `False`) --
  Whether to return the attention mask. Can be overwritten when calling the feature extractor.

  [What are attention masks?](../glossary#attention-mask)

  

  For Whisper models, `attention_mask` should always be passed for batched inference, to avoid subtle
  bugs.

  
- **stem_indices** (`list[int]`, *optional*, defaults to `[3, 2]`) --
  Stem channels to extract if demucs outputs are passed.

Constructs a MusicgenMelody feature extractor.

This feature extractor inherits from [SequenceFeatureExtractor](/docs/transformers/v5.14.0/en/main_classes/feature_extractor#transformers.SequenceFeatureExtractor) which contains
most of the main methods. Users should refer to this superclass for more information regarding those methods.

This class extracts chroma features from audio processed by [Demucs](https://github.com/adefossez/demucs/tree/main) or
directly from raw audio waveform.

## MusicgenMelodyConfig[[transformers.MusicgenMelodyConfig]]

- **text_encoder** (`Union[dict, `PretrainedConfig`]`) --
  An instance of a configuration object that defines the text encoder config.
- **audio_encoder** (`Union[dict, `PretrainedConfig`]`) --
  An instance of a configuration object that defines the audio encoder config.
- **decoder** (`Union[dict, `PretrainedConfig`]`) --
  An instance of a configuration object that defines the decoder config.
- **num_chroma** (`int`, *optional*, defaults to 12) --
  Number of chroma bins to use.
- **chroma_length** (`int`, *optional*, defaults to 235) --
  Maximum chroma duration if audio is used to condition the model. Corresponds to the maximum duration used during training.
- **initializer_factor** (`float`, *optional*, defaults to `0.02`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).

This is the configuration class to store the configuration of a MusicgenMelodyModel. It is used to instantiate a Musicgen Melody
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/musicgen-melody](https://huggingface.co/facebook/musicgen-melody)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     MusicgenMelodyConfig,
...     MusicgenMelodyDecoderConfig,
...     T5Config,
...     EncodecConfig,
...     MusicgenMelodyForConditionalGeneration,
... )

>>> # Initializing text encoder, audio encoder, and decoder model configurations
>>> text_encoder_config = T5Config()
>>> audio_encoder_config = EncodecConfig()
>>> decoder_config = MusicgenMelodyDecoderConfig()

>>> configuration = MusicgenMelodyConfig(
...     text_encoder=text_encoder_config, audio_encoder=audio_encoder_config, decoder=decoder_config
... )

>>> # Initializing a MusicgenMelodyForConditionalGeneration (with random weights) from the facebook/musicgen-melody style configuration
>>> model = MusicgenMelodyForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
>>> config_text_encoder = model.config.text_encoder
>>> config_audio_encoder = model.config.audio_encoder
>>> config_decoder = model.config.decoder

>>> # Saving the model, including its configuration
>>> model.save_pretrained("musicgen_melody-model")

>>> # loading model and config from pretrained folder
>>> musicgen_melody_config = MusicgenMelodyConfig.from_pretrained("musicgen_melody-model")
>>> model = MusicgenMelodyForConditionalGeneration.from_pretrained("musicgen_melody-model", config=musicgen_melody_config)
```

## MusicgenMelodyModel[[transformers.MusicgenMelodyModel]]

- **config** ([MusicgenMelodyDecoderConfig](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyDecoderConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Musicgen Melody Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size * num_codebooks, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary, corresponding to the sequence of audio codes.

  Indices can be obtained by encoding an audio prompt with an audio encoder model to predict audio codes,
  such as with the [EncodecModel](/docs/transformers/v5.14.0/en/model_doc/encodec#transformers.EncodecModel). See [EncodecModel.encode()](/docs/transformers/v5.14.0/en/model_doc/encodec#transformers.EncodecModel.encode) for details.

  [What are input IDs?](../glossary#input-ids)

  

  The `input_ids` will automatically be converted from shape `(batch_size * num_codebooks,
  target_sequence_length)` to `(batch_size, num_codebooks, target_sequence_length)` in the forward pass. If
  you obtain audio codes from an audio encoding model, such as [EncodecModel](/docs/transformers/v5.14.0/en/model_doc/encodec#transformers.EncodecModel), ensure that the number of
  frames is equal to 1, and that you reshape the audio codes from `(frames, batch_size, num_codebooks,
  target_sequence_length)` to `(batch_size * num_codebooks, target_sequence_length)` prior to passing them as
  `input_ids`.

  
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch_size, encoder_sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states representing the concatenation of the text encoder output and the processed audio encoder output.
  Used as a conditional signal and will thus be concatenated to the projected `decoder_input_ids`.
- **encoder_attention_mask** (`torch.LongTensor` of shape `(batch_size, encoder_sequence_length)`, *optional*) --
  Mask to avoid performing attention on conditional hidden states. Mask values
  selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
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
  `past_key_values`).[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [MusicgenMelodyModel](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyModel) forward method, overrides the `__call__` special method.

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

## MusicgenMelodyForCausalLM[[transformers.MusicgenMelodyForCausalLM]]

- **config** ([MusicgenMelodyDecoderConfig](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyDecoderConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Musicgen Melody decoder model with a language modelling head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size * num_codebooks, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary, corresponding to the sequence of audio codes.

  Indices can be obtained by encoding an audio prompt with an audio encoder model to predict audio codes,
  such as with the [EncodecModel](/docs/transformers/v5.14.0/en/model_doc/encodec#transformers.EncodecModel). See [EncodecModel.encode()](/docs/transformers/v5.14.0/en/model_doc/encodec#transformers.EncodecModel.encode) for details.

  [What are input IDs?](../glossary#input-ids)

  

  The `input_ids` will automatically be converted from shape `(batch_size * num_codebooks,
  target_sequence_length)` to `(batch_size, num_codebooks, target_sequence_length)` in the forward pass. If
  you obtain audio codes from an audio encoding model, such as [EncodecModel](/docs/transformers/v5.14.0/en/model_doc/encodec#transformers.EncodecModel), ensure that the number of
  frames is equal to 1, and that you reshape the audio codes from `(frames, batch_size, num_codebooks,
  target_sequence_length)` to `(batch_size * num_codebooks, target_sequence_length)` prior to passing them as
  `input_ids`.

  
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch_size, encoder_sequence_length, hidden_size)`, *optional*) --
  Sequence of hidden-states representing the concatenation of the text encoder output and the processed audio encoder output.
  Used as a conditional signal and will thus be concatenated to the projected `decoder_input_ids`.
- **encoder_attention_mask** (`torch.LongTensor` of shape `(batch_size, encoder_sequence_length)`, *optional*) --
  Mask to avoid performing attention on conditional hidden states. Mask values
  selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
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
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length, num_codebooks)`, *optional*) --
  Labels for language modeling. Note that the labels **are shifted** inside the model, i.e. you can set
  `labels = input_ids` Indices are selected in `[-100, 0, ..., config.vocab_size]` All labels set to `-100`
  are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size]``MusicgenMelodyOutputWithPast` or `tuple(torch.FloatTensor)`A `MusicgenMelodyOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MusicgenMelodyConfig](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyConfig)) and inputs.
The [MusicgenMelodyForCausalLM](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyForCausalLM) forward method, overrides the `__call__` special method.

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
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch_size, encoder_sequence_length, hidden_size)`, *optional*) -- Sequence of conditional hidden-states representing the concatenation of the projected text encoder output and the projected audio encoder output.
  Used as a conditional signal.

Example:

```python
```

## MusicgenMelodyForConditionalGeneration[[transformers.MusicgenMelodyForConditionalGeneration]]

- **config** ([MusicgenMelodyConfig](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyConfig), *optional*) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
- **text_encoder** (`PreTrainedModel`, *optional*) --
  The text encoder model that encodes text into hidden states for conditioning.
- **audio_encoder** (`PreTrainedModel`, *optional*) --
  The audio encoder model that encodes audio into hidden states for conditioning.
- **decoder** (`MusicgenMelodyForCausalLM`, *optional*) --
  The decoder model that generates audio tokens based on conditioning signals.

The Musicgen Melody Model for token generation conditioned on other modalities (e.g. image-text-to-text generation).

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
- **attention_mask** (`torch.BoolTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **input_features** (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  [MusicgenMelodyFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyFeatureExtractor). See `MusicgenMelodyFeatureExtractor.__call__()` for details ([MusicgenMelodyProcessor](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyProcessor) uses
  [MusicgenMelodyFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyFeatureExtractor) for processing audios).
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size * num_codebooks, target_sequence_length)`, *optional*) --
  Indices of decoder input sequence tokens in the vocabulary, corresponding to the sequence of audio codes.

  Indices can be obtained by encoding an audio prompt with an audio encoder model to predict audio codes,
  such as with the [EncodecModel](/docs/transformers/v5.14.0/en/model_doc/encodec#transformers.EncodecModel). See [EncodecModel.encode()](/docs/transformers/v5.14.0/en/model_doc/encodec#transformers.EncodecModel.encode) for details.

  [What are decoder input IDs?](../glossary#decoder-input-ids)

  

  The `decoder_input_ids` will automatically be converted from shape `(batch_size * num_codebooks,
  target_sequence_length)` to `(batch_size, num_codebooks, target_sequence_length)` in the forward pass. If
  you obtain audio codes from an audio encoding model, such as [EncodecModel](/docs/transformers/v5.14.0/en/model_doc/encodec#transformers.EncodecModel), ensure that the number of
  frames is equal to 1, and that you reshape the audio codes from `(frames, batch_size, num_codebooks,
  target_sequence_length)` to `(batch_size * num_codebooks, target_sequence_length)` prior to passing them as
  `decoder_input_ids`.

  
- **decoder_attention_mask** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Default behavior: generate a tensor that ignores pad tokens in `decoder_input_ids`. Causal mask will also
  be used by default.
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
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch_size, encoder_sequence_length, hidden_size)`, *optional*) --
  Sequence of conditional hidden-states representing the concatenation of the projected text encoder output and the projected audio encoder output.
  Used as a conditional signal and will thus be concatenated to the projected `decoder_input_ids`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded
  representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be
  input (see `past_key_values`). This is useful if you want more control over how to convert
  `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value
  of `inputs_embeds`.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length, num_codebooks)`, *optional*) --
  Labels for language modeling. Note that the labels **are shifted** inside the model, i.e. you can set
  `labels = input_ids` Indices are selected in `[-100, 0, ..., config.vocab_size]` All labels set to `-100`
  are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size]`
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`MusicgenMelodyOutputWithPast` or `tuple(torch.FloatTensor)`A `MusicgenMelodyOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MusicgenMelodyConfig](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyConfig)) and inputs.
The [MusicgenMelodyForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/musicgen_melody#transformers.MusicgenMelodyForConditionalGeneration) forward method, overrides the `__call__` special method.

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
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch_size, encoder_sequence_length, hidden_size)`, *optional*) -- Sequence of conditional hidden-states representing the concatenation of the projected text encoder output and the projected audio encoder output.
  Used as a conditional signal.

Examples:
```python
>>> from transformers import AutoProcessor, MusicgenMelodyForConditionalGeneration
>>> import torch

>>> processor = AutoProcessor.from_pretrained("facebook/musicgen-melody")
>>> model = MusicgenMelodyForConditionalGeneration.from_pretrained("facebook/musicgen-melody")

>>> inputs = processor(
...     text=["80s pop track with bassy drums and synth", "90s rock song with loud guitars and heavy drums"],
...     padding=True,
...     return_tensors="pt",
... )

>>> pad_token_id = model.generation_config.pad_token_id
>>> decoder_input_ids = (
...     torch.ones((inputs.input_ids.shape[0] * model.decoder.num_codebooks, 1), dtype=torch.long)
...     * pad_token_id
... )

>>> logits = model(**inputs, decoder_input_ids=decoder_input_ids).logits
>>> logits.shape  # (bsz * num_codebooks, encoder_len + tgt_len, vocab_size)
torch.Size([8, 249, 2048])
```
