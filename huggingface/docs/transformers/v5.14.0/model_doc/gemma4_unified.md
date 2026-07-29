# Gemma4 Unified

## Overview

Gemma 4 12B Unified is an **encoder-free** multimodal model with pretrained and instruction-tuned variants. Unlike [standard Gemma 4](./gemma4), which uses dedicated encoder towers, Gemma 4 12B Unified projects raw inputs directly into the language model's embedding space through lightweight linear pipelines. This results in a simpler architecture while maintaining strong multimodal performance.

Key differences from standard Gemma 4:
- **No Vision Tower**: Raw pixel patches are projected directly into LM space via a `Dense + LayerNorm` pipeline with factorized 2D positional embeddings, replacing the vision encoder.
- **No Audio Tower**: Raw 16 kHz waveform samples are chunked into fixed-length frames and projected through a simple `RMSNorm → Linear` pipeline, replacing the mel spectrogram + Conformer encoder.
- **Shared Multimodal Pipeline**: Both vision and audio use the same `Gemma4UnifiedMultimodalEmbedder` (RMSNorm → Linear) for the final projection to text hidden space.

You can find the original Gemma 4 12B Unified checkpoints under the [Gemma 4](https://huggingface.co/collections/google/gemma-4) release.

### Encoder-Free Vision Pipeline

The key architectural difference from standard Gemma 4 is the removal of the vision encoder tower. Instead, Gemma 4 12B Unified processes images through a lightweight pipeline:

1. **Patchification**: Images are split into `16×16` pixel patches
2. **Patch Merging**: Adjacent `3×3` patches are merged into `48×48` model patches, each with `48² × 3 = 6,912` raw pixel channels
3. **Projection**: `LayerNorm → Dense → LayerNorm` projects each merged patch into the LM embedding dimension
4. **Positional Embedding**: Factorized 2D positional embeddings are added (separate learned embeddings for x and y axes, summed together)
5. **Final Norm**: A final `LayerNorm` is applied
6. **Multimodal Embedder**: `RMSNorm → Linear` projects to the text hidden size

Like standard Gemma 4, the model processes **images of different sizes** using a **fixed-budget number of tokens**. The same constraints apply:
- The total number of pixels must fit within a patch budget
- Both height and width must be divisible by **48** (= patch size 16 × pooling kernel 3)

> [!IMPORTANT]
> Gemma 4 12B Unified does **not** apply mean/std normalization. The model's own patch embedding layer handles the final scaling internally.

The number of soft tokens per image is configurable. The supported options and default (**280 soft tokens**) are:

| Soft Tokens | Patches (before pooling) | Approx. Image Area |
|:-----------:|:------------------------:|:-------------------:|
| 70          | 630                      | ~161K pixels        |
| 140         | 1,260                    | ~323K pixels        |
| **280**     | **2,520**                | **~645K pixels**    |
| 560         | 5,040                    | ~1.3M pixels        |
| 1,120       | 10,080                   | ~2.6M pixels        |

### Encoder-Free Audio Pipeline

The audio pipeline is similarly simplified. Instead of computing mel spectrograms and processing them through a Conformer encoder, raw 16 kHz waveform samples are:

1. **Chunked** into fixed-length frames of 640 samples each (40ms per frame at 16 kHz)
2. **Projected** directly through `RMSNorm → Linear` via the shared `Gemma4UnifiedMultimodalEmbedder`

Since there is **no downsampling**, the number of output soft tokens equals the number of input frames: `ceil(num_samples / 640)`.

## Usage examples

The example below demonstrates how to generate text based on an image and an audio sample with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipe = pipeline(
    task="any-to-any",
    model="google/gemma-4-12B-it",
)

image_messages = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
            },
            {
                "type": "text",
                "text": "What is shown in this image?"
            }
        ]
    }
]

image_output = pipe(image_messages, return_full_text=False)
print(image_output[0]["generated_text"])

audio_messages = [
    {
        "role": "user",
        "content": [
            {"type": "text", "text": "Please transcribe the following audio:"},
            {
                "type": "audio",
                "url": "https://huggingface.co/datasets/eustlb/audio-samples/resolve/main/bcn_weather.mp3",
            },
        ],
    }
]

audio_output = pipe(audio_messages, return_full_text=False)
print(audio_output[0]["generated_text"])
```

### Image

```python
from transformers import AutoModelForMultimodalLM, AutoProcessor

model = AutoModelForMultimodalLM.from_pretrained(
    "google/gemma-4-12B-it",
    device_map="auto"
)
processor = AutoProcessor.from_pretrained(
    "google/gemma-4-12B-it"
)

messages = [
    {
        "role": "user", "content": [
            {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
            {"type": "text", "text": "What is shown in this image?"},
        ]
    },
]
inputs = processor.apply_chat_template(
    messages,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
    add_generation_prompt=True,
).to(model.device)
input_len = inputs["input_ids"].shape[-1]

output = model.generate(**inputs, max_new_tokens=50, cache_implementation="static")
print(processor.decode(output[0][input_len:], skip_special_tokens=True))
```

### Audio

```python
from transformers import AutoModelForMultimodalLM, AutoProcessor

messages = [
    {
        "role": "user",
        "content": [
            {"type": "text", "text": "Please transcribe the following audio:"},
            {
                "type": "audio",
                "url": "https://huggingface.co/datasets/eustlb/audio-samples/resolve/main/bcn_weather.mp3",
            },
        ],
    }
]

model = AutoModelForMultimodalLM.from_pretrained(
    "google/gemma-4-12B-it",
    device_map="auto"
)
processor = AutoProcessor.from_pretrained(
    "google/gemma-4-12B-it"
)

inputs = processor.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_dict=True,
    return_tensors="pt"
).to(model.device, dtype=model.dtype)

input_len = inputs["input_ids"].shape[-1]

outputs = model.generate(**inputs, max_new_tokens=200)
print(processor.decode(outputs[0][input_len:], skip_special_tokens=False))
```

## Gemma4UnifiedAudioConfig[[transformers.Gemma4UnifiedAudioConfig]]

- **audio_embed_dim** (`int`, defaults to 640) --
  Dimension of audio features input to the multimodal embedder. Each audio soft
  token is a raw waveform frame of `audio_samples_per_token` samples, so
  `audio_embed_dim == audio_samples_per_token`.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Gemma4UnifiedModel. It is used to instantiate a Gemma4 Unified
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/gemma-4-12B-it](https://huggingface.co/google/gemma-4-12B-it)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Gemma4UnifiedConfig[[transformers.Gemma4UnifiedConfig]]

- **text_config** (`Union[~models.gemma4_unified.configuration_gemma4_unified.Gemma4UnifiedTextConfig, dict[str, Any]]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[~models.gemma4_unified.configuration_gemma4_unified.Gemma4UnifiedVisionConfig, dict[str, Any]]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **audio_config** (`Union[~models.gemma4_unified.configuration_gemma4_unified.Gemma4UnifiedAudioConfig, dict[str, Any]]`, *optional*) --
  The config object or dictionary of the audio backbone.
- **boi_token_id** (`int`, *optional*, defaults to 255999) --
  The begin-of-image token index to wrap the image prompt.
- **eoi_token_id** (`int`, *optional*, defaults to 258882) --
  The end-of-image token index to wrap the image prompt.
- **image_token_id** (`int`, *optional*, defaults to `258880`) --
  The image token index used as a placeholder for input images.
- **video_token_id** (`int`, *optional*, defaults to `258884`) --
  The video token index used as a placeholder for input videos.
- **boa_token_id** (`int`, *optional*, defaults to 256000) --
  The begin-of-audio token index to wrap the audio prompt.
- **eoa_token_index** (`int`, *optional*, defaults to 258883) --
  The end-of-audio token index to wrap the audio prompt.
- **audio_token_id** (`int`, *optional*, defaults to `258881`) --
  The audio token index used as a placeholder for input audio.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Gemma4UnifiedModel. It is used to instantiate a Gemma4 Unified
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/gemma-4-12B-it](https://huggingface.co/google/gemma-4-12B-it)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
>>>     Gemma4UnifiedAudioConfig,
>>>     Gemma4UnifiedConfig,
>>>     Gemma4UnifiedForConditionalGeneration,
>>>     Gemma4UnifiedTextConfig,
>>>     Gemma4UnifiedVisionConfig,
>>> )

>>> # Initializing a Gemma 4 Audio config.
>>> audio_config = Gemma4UnifiedAudioConfig()

>>> # Initializing a Gemma 4 Text config.
>>> text_config = Gemma4UnifiedTextConfig()

>>> # Initializing a Gemma 4 vision config.
>>> vision_config = Gemma4UnifiedVisionConfig()

>>> # Initializing a Gemma 4 config similar to google/gemma-4-e2b-it
>>> configuration = Gemma4UnifiedConfig(text_config, vision_config, audio_config)

>>> # Initializing a model from the google/gemma-4-e2b-it configuration
>>> model = Gemma4UnifiedForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Gemma4UnifiedTextConfig[[transformers.Gemma4UnifiedTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `262144`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `2304`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `9216`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `30`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `4`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **head_dim** (`int`, *optional*, defaults to `256`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **hidden_activation** (`str`, *optional*, defaults to `gelu_pytorch_tanh`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `262144`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `1`) --
  Token id used for end-of-stream in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `2`) --
  Token id used for beginning-of-stream in the vocabulary.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **rope_parameters** (`dict`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[int, float]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **sliding_window** (`int`, *optional*, defaults to `1024`) --
  Sliding window attention window size. If `None`, no sliding window is applied.
- **layer_types** (`list[str]`, *optional*) --
  A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically
  generated based on config values.
- **final_logit_softcapping** (`float`, *optional*) --
  Soft-capping value applied to the final logits before computing the probability distribution. Logits are
  scaled by `tanh(logit / cap) * cap`.
- **use_bidirectional_attention** (`str`, *optional*) --
  Controls bidirectional attention behavior. When set to `"vision"`, vision tokens
  attend bidirectionally while text tokens use causal attention. When set to `"all"`,
  all tokens use bidirectional attention.
- **num_global_key_value_heads** (`int`, *optional*) --
  Number of key-value heads for global (full) attention layers. If `None`, defaults
  to `num_key_value_heads`.
- **global_head_dim** (`int`, defaults to 512) --
  Dimension of each attention head in global (full) attention layers.
- **attention_k_eq_v** (`bool`, defaults to `False`) --
  Whether keys and values share the same projection weights. When `True`, the key
  projection output is reused as the value projection.
- **num_kv_shared_layers** (`int`, defaults to 0) --
  Number of consecutive decoder layers that share the same key-value projections.
  A value of 0 means no sharing (each layer has independent KV projections).
- **use_double_wide_mlp** (`bool`, defaults to `False`) --
  Whether to use a double-width MLP with fused gate and up projections.

This is the configuration class to store the configuration of a Gemma4UnifiedModel. It is used to instantiate a Gemma4 Unified
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/gemma-4-12B-it](https://huggingface.co/google/gemma-4-12B-it)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Gemma4UnifiedVisionConfig[[transformers.Gemma4UnifiedVisionConfig]]

- **patch_size** (`int`, defaults to 16) --
  Size of the image patches in pixels. Images are first patchified at this resolution.
- **pooling_kernel_size** (`int`, defaults to 3) --
  Kernel size for merging patches into model patches. A 3×3 merge produces
  model patches of size `patch_size * pooling_kernel_size = 48` pixels.
- **mm_embed_dim** (`int`, defaults to 3840) --
  Hidden dimension for the patch embedding Dense projection (matches the text model `hidden_size`).
- **mm_posemb_size** (`int`, defaults to 1120) --
  Size of the factorized 2D positional embedding table. The table has shape
  `(mm_posemb_size, 2, mm_embed_dim)` and is looked up per-axis.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **output_proj_dims** (`int`, defaults to 3840) --
  Output dimension of the multimodal embedder projection (maps to text hidden size).
  This is set by the composite config's text_config.hidden_size at runtime.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Gemma4UnifiedModel. It is used to instantiate a Gemma4 Unified
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/gemma-4-12B-it](https://huggingface.co/google/gemma-4-12B-it)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Gemma4UnifiedAudioFeatureExtractor[[transformers.Gemma4UnifiedAudioFeatureExtractor]]

- **feature_size** (`int`, *optional*, defaults to 640) --
  The feature dimension of the extracted features (samples per token).
- **sampling_rate** (`int`, *optional*, defaults to 16000) --
  The sampling rate at which the audio files should be digitalized expressed in hertz (Hz).
- **padding_value** (`float`, *optional*, defaults to 0.0) --
  Padding value used to pad the audio.
- **audio_samples_per_token** (`int`, *optional*, defaults to 640) --
  Number of raw audio samples per output token. At 16 kHz, 640 samples = 40ms.
Encoder-free audio feature extractor that chunks raw waveform into frames.

Unlike the standard Gemma4 audio feature extractor which computes mel spectrograms,
this unified version simply chunks raw 16 kHz audio into fixed-length frames
of `audio_samples_per_token` samples each. Each frame becomes a single audio
soft token with the raw waveform samples as its features.

- **raw_speech** --
  The raw audio waveform(s) to process.
- **padding** (`str`, *optional*, defaults to `"longest"`) --
  Padding strategy for batches with different lengths.
- **max_length** (`int`, *optional*) --
  Maximum number of tokens to produce per audio.
- **truncation** (`bool`, *optional*, defaults to `True`) --
  Whether to truncate audio above `max_length` tokens.
- **return_tensors** (`str`, *optional*) --
  The type of tensors to return.
Chunk raw audio waveforms into fixed-length frames for the unified model.

## Gemma4UnifiedImageProcessor[[transformers.Gemma4UnifiedImageProcessor]]

- **patch_size** (`int`, *kwargs*, *optional*) --
  Size of each teacher image patch in pixels (before merging).
- **max_soft_tokens** (`int`, *kwargs*, *optional*) --
  Maximum number of soft (vision) tokens per image after patch merging.
  Must be one of {70, 140, 280, 560, 1120}.
- **pooling_kernel_size** (`int`, *kwargs*, *optional*) --
  Kernel size for merging teacher patches into model patches.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Gemma4 unified image processor.

## Gemma4UnifiedVideoProcessor[[transformers.Gemma4UnifiedVideoProcessor]]

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
Constructs a Gemma4Unified video processor that samples frames from videos for use with the Gemma4Unified model.

## Gemma4UnifiedProcessor[[transformers.Gemma4UnifiedProcessor]]

- **feature_extractor** (*Gemma4UnifiedAudioFeatureExtractor*) --
  The feature extractor is a required input.
- **image_processor** (*Gemma4UnifiedImageProcessor*) --
  The image processor is a required input.
- **tokenizer** (*tokenizer_class*) --
  The tokenizer is a required input.
- **video_processor** (*Gemma4UnifiedVideoProcessor*) --
  The video processor is a required input.
- **chat_template** (*str*) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **image_seq_length** (*int*, *optional*, defaults to 280) --
  The number of soft tokens per image used for placeholder expansion.
- **audio_seq_length** (*int*, *optional*, defaults to 750) --
  The maximum number of audio soft tokens per audio segment. Serves as an
  upper-bound cap when dynamic audio token counts are computed.
- **audio_ms_per_token** (*int*, *optional*, defaults to 40) --
  Milliseconds of audio per output soft token. Used to dynamically compute
  the number of audio placeholder tokens as `ceil(duration_ms / audio_ms_per_token)`.
  The default of 40 comes from the SSCP convolution's 4× time reduction on 10ms frames.
Constructs a Gemma4UnifiedProcessor which wraps a feature extractor, a image processor, a tokenizer, and a video processor into a single processor.

[*Gemma4UnifiedProcessor*] offers all the functionalities of [*Gemma4UnifiedAudioFeatureExtractor*], [*Gemma4UnifiedImageProcessor*], [*tokenizer_class*], and [*Gemma4UnifiedVideoProcessor*]. See the
[*~Gemma4UnifiedAudioFeatureExtractor*], [*~Gemma4UnifiedImageProcessor*], [*~tokenizer_class*], and [*~Gemma4UnifiedVideoProcessor*] for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **videos** (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) --
  Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If
  passing in videos with pixel values between 0 and 1, set `do_rescale=False`.
- **audio** (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) --
  The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor.
  In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels,
  and T is the sample length of the audio.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.

## Gemma4UnifiedPreTrainedModel[[transformers.Gemma4UnifiedPreTrainedModel]]

- **config** ([PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

## Gemma4UnifiedModel[[transformers.Gemma4UnifiedModel]]

- **config** ([Gemma4UnifiedConfig](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The base Gemma 4 model comprising a vision backbone, an audio backbone, and a language model without a
language modeling head.

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
  [Gemma4UnifiedImageProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedImageProcessor). See `Gemma4UnifiedImageProcessor.__call__()` for details ([Gemma4UnifiedProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedProcessor) uses
  [Gemma4UnifiedImageProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedImageProcessor) for processing images).
- **pixel_values_videos** (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [Gemma4UnifiedVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedVideoProcessor). See `Gemma4UnifiedVideoProcessor.__call__()` for details ([Gemma4UnifiedProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedProcessor) uses
  [Gemma4UnifiedVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedVideoProcessor) for processing videos).
- **input_features** (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  [Gemma4UnifiedAudioFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedAudioFeatureExtractor). See [Gemma4UnifiedAudioFeatureExtractor.__call__()](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedAudioFeatureExtractor.__call__) for details ([Gemma4UnifiedProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedProcessor) uses
  [Gemma4UnifiedAudioFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedAudioFeatureExtractor) for processing audios).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **input_features_mask** (`torch.FloatTensor]` of shape `(num_images, seq_length)`) --
  The attention mask for the input audio.
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
- **mm_token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens matching each modality. For example text (0), image (1), video (2).
  Multimodal token type ids can be obtained using [AutoProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoProcessor). See [ProcessorMixin.__call__()](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **image_position_ids** (`torch.LongTensor` of shape `(batch_size, max_patches, 2)`, *optional*) --
  2D patch position coordinates from the image processor, with `(-1, -1)` indicating padding.
  Passed through to the vision encoder for positional embedding computation.
- **video_position_ids** (`torch.LongTensor` of shape `(num_videos, num_frames, max_patches, 2)`, *optional*) --
  2D patch position coordinates from the video processor, with `(-1, -1)` indicating padding.
  Passed through to the vision encoder for positional embedding computation.`Gemma4UnifiedModelOutputWithPast` or `tuple(torch.FloatTensor)`A `Gemma4UnifiedModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Gemma4UnifiedConfig](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedConfig)) and inputs.
The [Gemma4UnifiedModel](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedModel) forward method, overrides the `__call__` special method.

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
- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  audio_hidden_states of the model produced by the audio encoder and after projecting the last hidden state.
- **shared_kv_states** (`dict`, *optional*) -- Dictionary mapping layer type strings to tuples of (key_states, value_states) tensors.
  Used to pass shared KV states between layers during KV sharing.

## Gemma4UnifiedTextModel[[transformers.Gemma4UnifiedTextModel]]

- **config** ([Gemma4UnifiedTextConfig](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
The base Gemma 4 unified language model without a language modeling head.

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
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`Gemma4UnifiedTextModelOutputWithPast` or `tuple(torch.FloatTensor)`A `Gemma4UnifiedTextModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Gemma4UnifiedConfig](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedConfig)) and inputs.
The [Gemma4UnifiedTextModel](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **shared_kv_states** (`dict`, *optional*) -- Dictionary mapping layer type strings to tuples of (key_states, value_states) tensors.
  Used to pass shared KV states between layers during KV sharing.

## Gemma4UnifiedForCausalLM[[transformers.Gemma4UnifiedForCausalLM]]

- **config** ([Gemma4UnifiedTextConfig](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
The base Gemma 4 language model with a language modeling head.

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
  This is useful when using packed tensor format (single dimension for batch and sequence length).`Gemma4UnifiedCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `Gemma4UnifiedCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Gemma4UnifiedConfig](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedConfig)) and inputs.
The [Gemma4UnifiedForCausalLM](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.text_config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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
  image_hidden_states of the model produced by the vision encoder after projecting last hidden state.
- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  audio_hidden_states of the model produced by the audio encoder and after projecting the last hidden state.
- **shared_kv_states** (`dict`, *optional*) -- Dictionary mapping layer type strings to tuples of (key_states, value_states) tensors.
  Used to pass shared KV states between layers during KV sharing.

Example:

```python
>>> from transformers import AutoTokenizer, Gemma4UnifiedForCausalLM

>>> model = Gemma4UnifiedForCausalLM.from_pretrained("google/gemma-4-12B-it")
>>> tokenizer = AutoTokenizer.from_pretrained("google/gemma-4-12B-it")

>>> prompt = "What is your favorite condiment?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"What is your favorite condiment?"
```

## Gemma4UnifiedForConditionalGeneration[[transformers.Gemma4UnifiedForConditionalGeneration]]

- **config** ([Gemma4UnifiedConfig](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The base Gemma 4 model comprising a vision backbone, an audio backbone, a language model, and a language modeling
head.

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
  [Gemma4UnifiedImageProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedImageProcessor). See `Gemma4UnifiedImageProcessor.__call__()` for details ([Gemma4UnifiedProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedProcessor) uses
  [Gemma4UnifiedImageProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedImageProcessor) for processing images).
- **pixel_values_videos** (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [Gemma4UnifiedVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedVideoProcessor). See `Gemma4UnifiedVideoProcessor.__call__()` for details ([Gemma4UnifiedProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedProcessor) uses
  [Gemma4UnifiedVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedVideoProcessor) for processing videos).
- **input_features** (`torch.FloatTensor` of shape `(batch_size, sequence_length, feature_dim)`, *optional*) --
  The tensors corresponding to the input audio features. Audio features can be obtained using
  [Gemma4UnifiedAudioFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedAudioFeatureExtractor). See [Gemma4UnifiedAudioFeatureExtractor.__call__()](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedAudioFeatureExtractor.__call__) for details ([Gemma4UnifiedProcessor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedProcessor) uses
  [Gemma4UnifiedAudioFeatureExtractor](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedAudioFeatureExtractor) for processing audios).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **input_features_mask** (`torch.FloatTensor]` of shape `(num_images, seq_length)`) --
  The attention mask for the input audio.
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **image_position_ids** (`torch.LongTensor` of shape `(batch_size, max_patches, 2)`, *optional*) --
  2D patch position coordinates from the image processor, with `(-1, -1)` indicating padding.
  Passed through to the vision encoder for positional embedding computation.
- **video_position_ids** (`torch.LongTensor` of shape `(num_videos, num_frames, max_patches, 2)`, *optional*) --
  2D patch position coordinates from the video processor, with `(-1, -1)` indicating padding.
  Passed through to the vision encoder for positional embedding computation.
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
- **mm_token_type_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens matching each modality. For example text (0), image (1), video (2).
  Multimodal token type ids can be obtained using [AutoProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoProcessor). See [ProcessorMixin.__call__()](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

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
  This is useful when using packed tensor format (single dimension for batch and sequence length).`Gemma4UnifiedCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `Gemma4UnifiedCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Gemma4UnifiedConfig](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedConfig)) and inputs.
The [Gemma4UnifiedForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/gemma4_unified#transformers.Gemma4UnifiedForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.text_config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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
  image_hidden_states of the model produced by the vision encoder after projecting last hidden state.
- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  audio_hidden_states of the model produced by the audio encoder and after projecting the last hidden state.
- **shared_kv_states** (`dict`, *optional*) -- Dictionary mapping layer type strings to tuples of (key_states, value_states) tensors.
  Used to pass shared KV states between layers during KV sharing.

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, Gemma4UnifiedForConditionalGeneration

>>> model = Gemma4UnifiedForConditionalGeneration.from_pretrained("google/gemma-4-12B-it")
>>> processor = AutoProcessor.from_pretrained("google/gemma-4-12B-it")

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
