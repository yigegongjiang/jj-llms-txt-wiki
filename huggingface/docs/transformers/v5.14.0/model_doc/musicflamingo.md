# Music Flamingo

## Overview

Music Flamingo is a fully open large audio–language model designed for robust understanding and reasoning over music. It builds upon the [Audio Flamingo 3](./audioflamingo3) architecture by including **Rotary Time Embeddings (RoTE)**, which injects temporal position information to enable the model to handle audio sequences up to 20 minutes (1200 seconds).

The model checkpoint is available at: [nvidia/music-flamingo-2601-hf](https://huggingface.co/nvidia/music-flamingo-2601-hf)

Highlights:

- Unified audio encoder across speech, sound, and music.
- **Rotary Time Embeddings (RoTE)** for enhanced temporal modeling, enabling support for **up to 20 minutes of audio**.
- **Extended long-audio support via windowing and post-pool alignment (up to 20 minutes maximum).** The model processes audio in 30-second windows with a hard limit of 40 windows (20 minutes total). Audio longer than 20 minutes will be truncated.
- Special sound boundary tokens (`<|sound_bos|>` and `<|sound_eos|>`) for improved audio sequence modeling.
- Deterministic fusion that preserves sequence length by replacing audio placeholder tokens with audio embeddings.

This model was contributed by [Lasha Koroshinadze](https://huggingface.co/lashahub) and [Eric Bezzam](https://huggingface.co/bezzam).

### Paper

[Music Flamingo: Scaling Music Understanding in Audio Language Models](https://huggingface.co/papers/2511.10289)  
S. Ghosh, A. Goel, L. Koroshinadze, S. Lee, Z. Kong, J. F. Santos, R. Duraiswami, D. Manocha, W. Ping, M. Shoeybi, B. Catanzaro  
NVIDIA and University of Maryland  
Project: https://research.nvidia.com/labs/adlr/MF/

## Usage

### Audio Instruct Mode

The model supports audio-text instructions, including multi-turn interactions, all processed in batches.

➡️ audio + text instruction

```python
from transformers import AutoProcessor, MusicFlamingoForConditionalGeneration

model_id = "nvidia/music-flamingo-2601-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = MusicFlamingoForConditionalGeneration.from_pretrained(model_id, device_map="auto")

conversation = [
    {
        "role": "user",
        "content": [
            {"type": "text", "text": "Describe this track in full detail - tell me the genre, tempo, and key, then dive into the instruments, production style, and overall mood it creates."},
            {"type": "audio", "path": "https://huggingface.co/datasets/nvidia/AudioSkills/resolve/main/assets/song_1.mp3"},
        ],
    }
]

inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    add_generation_prompt=True,
    return_dict=True,
).to(model.device)
inputs["input_features"] = inputs["input_features"].to(model.dtype)

outputs = model.generate(**inputs, max_new_tokens=500)

decoded_outputs = processor.batch_decode(outputs[:, inputs.input_ids.shape[1]:], skip_special_tokens=True)
print(decoded_outputs)
```

➡️ multi-turn:

```python
from transformers import AutoProcessor, MusicFlamingoForConditionalGeneration

model_id = "nvidia/music-flamingo-2601-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = MusicFlamingoForConditionalGeneration.from_pretrained(model_id, device_map="auto")

conversation = [
    {
        "role": "user",
        "content": [
            {
                "type": "text",
                "text": "Write a rich caption that blends the technical details (genre, BPM, key, chords, mix) with how the song feels emotionally and dynamically as it unfolds.",
            },
            {"type": "audio", "path": "https://huggingface.co/datasets/nvidia/AudioSkills/resolve/main/assets/song_1.mp3"},
        ],
    },
    {
        "role": "assistant",
        "content": [{"type": "text", "text": "This energetic Eurodance anthem at 150 BPM in E major combines bright synth arpeggios with a punchy four-on-the-floor beat..."}],
    },
    {
        "role": "user",
        "content": [
            {"type": "text", "text": "What instruments stand out the most?"},
        ],
    },
]

inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    add_generation_prompt=True,
    return_dict=True,
).to(model.device)
inputs["input_features"] = inputs["input_features"].to(model.dtype)

outputs = model.generate(**inputs, max_new_tokens=500)

decoded_outputs = processor.batch_decode(outputs[:, inputs.input_ids.shape[1]:], skip_special_tokens=True)
print(decoded_outputs)
```

➡️ batched inference!

```python
from transformers import AutoProcessor, MusicFlamingoForConditionalGeneration

model_id = "nvidia/music-flamingo-2601-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = MusicFlamingoForConditionalGeneration.from_pretrained(model_id, device_map="auto")

conversations = [
    [
        {
            "role": "user",
            "content": [
                {"type": "text", "text": "Describe this track in full detail - tell me the genre, tempo, and key, then dive into the instruments, production style, and overall mood it creates."},
                {
                    "type": "audio",
                    "path": "https://huggingface.co/datasets/nvidia/AudioSkills/resolve/main/assets/song_1.mp3",
                },
            ],
        }
    ],
    [
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Generate a structured lyric sheet from the input music.",
                },
                {"type": "audio", "path": "https://huggingface.co/datasets/nvidia/AudioSkills/resolve/main/assets/song_2.mp3"},
            ],
        }
    ],
]

inputs = processor.apply_chat_template(
    conversations,
    tokenize=True,
    add_generation_prompt=True,
    return_dict=True,
).to(model.device)
inputs["input_features"] = inputs["input_features"].to(model.dtype)

outputs = model.generate(**inputs, max_new_tokens=500)

decoded_outputs = processor.batch_decode(outputs[:, inputs.input_ids.shape[1]:], skip_special_tokens=True)
print(decoded_outputs)
```

➡️ Training:

```python
from transformers import AutoProcessor, MusicFlamingoForConditionalGeneration

model_id = "nvidia/music-flamingo-2601-hf"
processor = AutoProcessor.from_pretrained(model_id)
model = MusicFlamingoForConditionalGeneration.from_pretrained(model_id, device_map="auto")
model.train()

conversation = [
    [
        {
            "role": "user",
            "content": [
                {"type": "text", "text": "Break the track down like a critic - list its tempo, key, and chordal motion, then explain the textures, dynamics, and emotional impact of the performance."},
                {"type": "audio", "path": "https://huggingface.co/datasets/nvidia/AudioSkills/resolve/main/assets/song_1.mp3"},
            ],
        },
        {
            "role": "assistant",
            "content": [{"type": "text", "text": "This Eurodance track operates at 150 BPM in E major, with harmonic movement centering on the I-vi-IV-V family. The production features layered synth arpeggios, a four-on-the-floor kick pattern, and a mezzo-soprano lead vocal with bright timbre. Dynamically, the track builds through verses into an anthemic chorus with full synth orchestration and backing vocals, creating an uplifting, euphoric atmosphere characteristic of late 2000s dance-pop."}],
        }
    ],
    [
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Describe this song from both a technical and artistic lens: mention tempo, harmony, and instrumentation, but also mood, lyrical themes, and structure.",
                },
                {"type": "audio", "path": "https://huggingface.co/datasets/nvidia/AudioSkills/resolve/main/assets/song_2.mp3"},
            ],
        },
        {
            "role": "assistant",
            "content": [{"type": "text", "text": "This electronic pop track combines upbeat production with playful lyrical themes centered around late-night pizza cravings. The structure follows a verse-chorus format with recurring melodic motifs and rhythmic patterns that emphasize the celebratory, lighthearted mood of the piece."}],
        }

    ]
]

inputs = processor.apply_chat_template(
    conversation,
    tokenize=True,
    add_generation_prompt=True,
    return_dict=True,
    processor_kwargs={"output_labels": True},
).to(model.device)
inputs["input_features"] = inputs["input_features"].to(model.dtype)

loss = model(**inputs).loss
loss.backward()
```

## How the model works

### Architecture

* **Audio Encoder**
  Whisper-style feature extractor + encoder → average-pool over time (stride 2) → LayerNorm.
  Produces per-frame hidden states at the post-pool rate.

* **Rotary Time Embeddings (RoTE)**
  Applied to the encoder output to inject temporal position information, enabling the model to handle audio sequences up to 20 minutes (1200 seconds). RoTE uses 2D axial rotary embeddings for batch and time dimensions with time-based angle modulation.

* **MusicFlamingoMultiModalProjector**
  A small MLP that maps encoder features to the language model's hidden size.

* **MusicFlamingoForConditionalGeneration**
  A causal language model that accepts text embeddings where each audio placeholder token slot is replaced, in place, by an audio frame embedding. Uses special boundary tokens (`<|sound_bos|>` and `<|sound_eos|>`) to mark audio sequences. No sequence-length change is introduced by fusion.

### Processor-level alignment

1. Each raw waveform is split into fixed-length windows based on the feature extractor’s `chunk_length` (seconds) and `sampling_rate` (Hz).
2. For each window, the processor computes the number of post-pool frames `post_pool_len` that the encoder will output (matching the conv/pool schedule).
3. The processor expands the audio placeholder token by the total number of post-pool frames across all windows.
4. The model later replaces those token positions with the corresponding projected audio embeddings.

## Long audio and windowing

**Important: Maximum audio length is 20 minutes.** Audio longer than this will be truncated.

* The default setup processes 30-second windows at 16 kHz mono.
* **The processor enforces a hard limit of 40 windows per sample, resulting in a maximum of 20 minutes of audio (40 windows × 30 seconds).**
* Rotary Time Embeddings (RoTE) provide position information for sequences up to 20 minutes (1200 seconds).
* For each window:

  * `mel_len` is the padded mel length.
  * A conv stack reduces time as `conv_output_len = (mel_len - 1) // 2 + 1`.
  * Post-pool frames per window: `post_pool_len = (conv_output_len - 2) // 2 + 1`.
  * An audio placeholder token is expanded to the sum of `post_pool_len` across all windows.

## MusicFlamingoConfig[[transformers.MusicFlamingoConfig]]

- **audio_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the audio backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **audio_token_id** (`int`, *optional*, defaults to `151669`) --
  The audio token index used as a placeholder for input audio.
- **projector_hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The activation function used by the multimodal projector.
- **projector_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in the multimodal projector.
- **audio_bos_token_id** (`int`, *optional*, defaults to 151670) --
  The beginning-of-audio token index used to mark the start of audio spans.
- **audio_eos_token_id** (`int`, *optional*, defaults to 151671) --
  The end-of-audio token index used to mark the end of audio spans.
- **audio_frame_step** (`float`, *optional*, defaults to 0.01) --
  Duration in seconds of one input mel frame (trained with hop_length 160 at sampling_rate 16000).
- **rope_parameters** (`dict`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.

This is the configuration class to store the configuration of a MusicFlamingoModel. It is used to instantiate a Musicflamingo
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/music-flamingo-2601-hf](https://huggingface.co/nvidia/music-flamingo-2601-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import MusicFlamingoForConditionalGeneration, MusicFlamingoConfig, AudioFlamingo3EncoderConfig, Qwen2Config

>>> # Initializing an MusicFlamingoEncoder config
>>> audio_config = AudioFlamingo3EncoderConfig()

>>> # Initializing a Qwen2 config
>>> text_config = Qwen2Config()

>>> # Initializing an MusicFlamingo configuration
>>> configuration = MusicFlamingoConfig(audio_config, text_config)

>>> # Initializing a model from the musicflamingo style configuration
>>> model = MusicFlamingoForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MusicFlamingoProcessor[[transformers.MusicFlamingoProcessor]]

'"}, {"name": "audio_bos_token", "val": " = '<|sound_bos|>'"}, {"name": "audio_eos_token", "val": " = '<|sound_eos|>'"}, {"name": "max_audio_len", "val": " = 1200"}]}>
- **feature_extractor** (`feature_extractor_class`) --
  The feature extractor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **audio_token** (`Optional[str]`, *optional*, defaults to `"<sound>"`) --
  Special token used to represent audio inputs in the chat template.
- **audio_bos_token** (`Optional[str]`, *optional*, defaults to `"<|sound_bos|>"`) --
  Special token used to represent the beginning of audio.
- **audio_eos_token** (`Optional[str]`, *optional*, defaults to `"<|sound_eos|>"`) --
  Special token used to represent the end of audio.
- **max_audio_len** (`int`, *optional*, defaults to 1200) --
  Maximum length of audio sequences in seconds. Audio longer than this will be truncated.
Constructs a MusicFlamingoProcessor which wraps a feature extractor and a tokenizer into a single processor.

[MusicFlamingoProcessor](/docs/transformers/v5.14.0/en/model_doc/musicflamingo#transformers.MusicFlamingoProcessor) offers all the functionalities of `feature_extractor_class` and `tokenizer_class`. See the
`~feature_extractor_class` and `~tokenizer_class` for more information.

## MusicFlamingoModel[[transformers.MusicFlamingoModel]]

- **config** ([MusicFlamingoConfig](/docs/transformers/v5.14.0/en/model_doc/musicflamingo#transformers.MusicFlamingoConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The MusicFlamingo model (fine-tuned Whisper encoder, multi-modal projector, Qwen2 language model),
without a language modeling head.

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
  `feature_extractor_class`. See `feature_extractor_class.__call__` for details ([MusicFlamingoProcessor](/docs/transformers/v5.14.0/en/model_doc/musicflamingo#transformers.MusicFlamingoProcessor) uses
  `feature_extractor_class` for processing audios).
- **input_features_mask** (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`) --
  Mask to avoid performing attention on padding feature indices.
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
  `past_key_values`).`MusicFlamingoModelOutputWithPast` or `tuple(torch.FloatTensor)`A `MusicFlamingoModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MusicFlamingoConfig](/docs/transformers/v5.14.0/en/model_doc/musicflamingo#transformers.MusicFlamingoConfig)) and inputs.
The [MusicFlamingoModel](/docs/transformers/v5.14.0/en/model_doc/musicflamingo#transformers.MusicFlamingoModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- Projected audio hidden states.

## MusicFlamingoForConditionalGeneration[[transformers.MusicFlamingoForConditionalGeneration]]

- **config** ([MusicFlamingoConfig](/docs/transformers/v5.14.0/en/model_doc/musicflamingo#transformers.MusicFlamingoConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The MusicFlamingo model which consists of a fine-tuned Whisper encoder, rotary time embedding, a multi-modal projector, and a Qwen2 language model.

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
  `feature_extractor_class`. See `feature_extractor_class.__call__` for details ([MusicFlamingoProcessor](/docs/transformers/v5.14.0/en/model_doc/musicflamingo#transformers.MusicFlamingoProcessor) uses
  `feature_extractor_class` for processing audios).
- **input_features_mask** (`torch.Tensor` of shape `(batch_size, feature_sequence_length)`) --
  Mask to avoid performing attention on padding feature indices.
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
  Labels for computing the masked language modeling loss.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`MusicFlamingoCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `MusicFlamingoCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MusicFlamingoConfig](/docs/transformers/v5.14.0/en/model_doc/musicflamingo#transformers.MusicFlamingoConfig)) and inputs.
The [MusicFlamingoForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/musicflamingo#transformers.MusicFlamingoForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **audio_hidden_states** (`torch.FloatTensor`, *optional*) -- Hidden states of the audio encoder after projection.

Example:

```python
>>> from transformers import MusicFlamingoForConditionalGeneration, AutoProcessor

>>> model_id = "nvidia/audio-flamingo-3-hf"
>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = MusicFlamingoForConditionalGeneration.from_pretrained(model_id, device_map="auto")
```
