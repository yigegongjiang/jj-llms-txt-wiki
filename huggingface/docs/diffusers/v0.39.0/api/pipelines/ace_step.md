# ACE-Step 1.5

ACE-Step 1.5 was introduced in [ACE-Step 1.5: Pushing the Boundaries of Open-Source Music Generation](https://arxiv.org/abs/2602.00744) by the ACE-Step Team (ACE Studio and StepFun). It is an open-source music foundation model that generates commercial-grade stereo music with lyrics from text prompts.

ACE-Step 1.5 generates variable-length stereo audio at 48 kHz (10 seconds to 10 minutes) from text prompts and optional lyrics. The full system pairs a Language Model planner with a Diffusion Transformer (DiT) synthesizer; this pipeline wraps the DiT half of that stack, and consists of three components: an [AutoencoderOobleck](/docs/diffusers/v0.39.0/en/api/models/autoencoder_oobleck#diffusers.AutoencoderOobleck) VAE that compresses waveforms into 25 Hz stereo latents, a Qwen3-based text encoder for prompt and lyric conditioning, and an [AceStepTransformer1DModel](/docs/diffusers/v0.39.0/en/api/models/ace_step_transformer#diffusers.AceStepTransformer1DModel) DiT that operates in the VAE latent space using flow matching.

The model supports 50+ languages for lyrics — including English, Chinese, Japanese, Korean, French, German, Spanish, Italian, Portuguese, and Russian — and runs on consumer GPUs (under 4 GB of VRAM when offloaded).

This pipeline was contributed by the [ACE-Step Team](https://github.com/ace-step). The original codebase can be found at [ace-step/ACE-Step-1.5](https://github.com/ace-step/ACE-Step-1.5).

## Variants

ACE-Step 1.5 ships three DiT checkpoints that share the same transformer architecture but differ in guidance behavior; the pipeline auto-detects turbo checkpoints from the loaded transformer config and ignores CFG guidance for those guidance-distilled weights.

| Variant | CFG | Default steps | Default `guidance_scale` | Default `shift` | HF repo |
|---------|:---:|:-------------:|:------------------------:|:---------------:|---------|
| `turbo` (guidance-distilled) | off | 8 | ignored | 3.0 | [`ACE-Step/acestep-v15-xl-turbo-diffusers`](https://huggingface.co/ACE-Step/acestep-v15-xl-turbo-diffusers) |
| `base` | on | 8 | 7.0 | 3.0 | [`ACE-Step/acestep-v15-base`](https://huggingface.co/ACE-Step/acestep-v15-base) |
| `sft` | on | 8 | 7.0 | 3.0 | [`ACE-Step/acestep-v15-sft`](https://huggingface.co/ACE-Step/acestep-v15-sft) |

Base and SFT use the learned `null_condition_emb` for classifier-free guidance (APG, not vanilla CFG). Users commonly override `num_inference_steps` to 30–60 on base/sft for higher quality.

## Tips

When constructing a prompt, keep in mind:

* Descriptive prompt inputs work best; use adjectives to describe the music style, instruments, mood, and tempo.
* The prompt should describe the overall musical characteristics (e.g., "upbeat pop song with electric guitar and drums").
* Lyrics should be structured with tags like `[verse]`, `[chorus]`, `[bridge]`, etc.

During inference:

* `num_inference_steps`, `guidance_scale`, and `shift` default to the values shown above. For turbo checkpoints, `guidance_scale > 1.0` is ignored with a warning because guidance is distilled into the weights.
* The `audio_duration` parameter controls the length of the generated music in seconds.
* The `vocal_language` parameter should match the language of the lyrics.
* `pipe.sample_rate` and `pipe.latents_per_second` are sourced from the VAE config (48000 Hz and 25 fps for the released checkpoints).
* For audio-to-audio tasks, pass `src_audio` and `reference_audio` as preprocessed stereo tensors at `pipe.sample_rate`.
* `flash` and `flash_hub` use FlashAttention's native sliding-window support for ACE-Step's self-attention and expect unpadded text batches. If a batched prompt contains padding, use `flash_varlen` or `flash_varlen_hub` instead. Single-prompt inference with `padding="longest"` is normally unpadded.

```python
import torch
import soundfile as sf
from diffusers import AceStepPipeline

pipe = AceStepPipeline.from_pretrained("ACE-Step/acestep-v15-xl-turbo-diffusers", torch_dtype=torch.bfloat16)
pipe = pipe.to("cuda")

audio = pipe(
    prompt="A beautiful piano piece with soft melodies and gentle rhythm",
    lyrics="[verse]\nSoft notes in the morning light\nDancing through the air so bright\n[chorus]\nMusic fills the air tonight\nEvery note feels just right",
    audio_duration=30.0,
).audios

sf.write("output.wav", audio[0].T.cpu().float().numpy(), pipe.sample_rate)
```

## AceStepPipeline[[diffusers.AceStepPipeline]]
#### diffusers.AceStepPipeline[[diffusers.AceStepPipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ace_step/pipeline_ace_step.py#L132)

Pipeline for text-to-music generation using ACE-Step 1.5.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

The pipeline uses flow matching with a custom timestep schedule for the diffusion process. The turbo model variant
uses 8 inference steps by default.

Supported task types:
- `"text2music"`: Generate music from text prompts and lyrics.
- `"cover"`: Generate audio from source audio / semantic codes with timbre transfer from reference audio.
- `"repaint"`: Regenerate a section of existing audio while keeping the rest.
- `"extract"`: Extract a specific track (e.g., vocals, drums) from audio.
- `"lego"`: Generate a specific track based on audio context.
- `"complete"`: Complete an input audio with additional tracks.

__call__diffusers.AceStepPipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ace_step/pipeline_ace_step.py#L779[{"name": "prompt", "val": ": typing.Union[str, typing.List[str]] = None"}, {"name": "lyrics", "val": ": typing.Union[str, typing.List[str]] = ''"}, {"name": "audio_duration", "val": ": float = 60.0"}, {"name": "vocal_language", "val": ": typing.Union[str, typing.List[str]] = 'en'"}, {"name": "num_inference_steps", "val": ": int = 8"}, {"name": "guidance_scale", "val": ": float = 7.0"}, {"name": "shift", "val": ": float = 3.0"}, {"name": "generator", "val": ": typing.Union[torch._C.Generator, typing.List[torch._C.Generator], NoneType] = None"}, {"name": "latents", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "output_type", "val": ": typing.Optional[str] = 'pt'"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback", "val": ": typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None"}, {"name": "callback_steps", "val": ": typing.Optional[int] = 1"}, {"name": "callback_on_step_end", "val": ": typing.Optional[typing.Callable[..., dict]] = None"}, {"name": "callback_on_step_end_tensor_inputs", "val": ": typing.List[str] = ('latents',)"}, {"name": "instruction", "val": ": typing.Optional[str] = None"}, {"name": "max_text_length", "val": ": int = 256"}, {"name": "max_lyric_length", "val": ": int = 2048"}, {"name": "bpm", "val": ": typing.Optional[int] = None"}, {"name": "keyscale", "val": ": typing.Optional[str] = None"}, {"name": "timesignature", "val": ": typing.Optional[str] = None"}, {"name": "task_type", "val": ": str = 'text2music'"}, {"name": "track_name", "val": ": typing.Optional[str] = None"}, {"name": "complete_track_classes", "val": ": typing.Optional[typing.List[str]] = None"}, {"name": "src_audio", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "reference_audio", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "audio_codes", "val": ": typing.Union[str, typing.List[str], NoneType] = None"}, {"name": "repainting_start", "val": ": typing.Optional[float] = None"}, {"name": "repainting_end", "val": ": typing.Optional[float] = None"}, {"name": "audio_cover_strength", "val": ": float = 1.0"}, {"name": "cfg_interval_start", "val": ": float = 0.0"}, {"name": "cfg_interval_end", "val": ": float = 1.0"}, {"name": "timesteps", "val": ": typing.Optional[typing.List[float]] = None"}]- **prompt** (`str` or `List[str]`, *optional*) --
  The prompt or prompts to guide music generation. Describes the style, genre, instruments, etc.
- **lyrics** (`str` or `List[str]`, *optional*, defaults to `""`) --
  The lyrics text for the music. Supports structured lyrics with tags like `[verse]`, `[chorus]`, etc.
- **audio_duration** (`float`, *optional*, defaults to 60.0) --
  Duration of the generated audio in seconds.
- **vocal_language** (`str` or `List[str]`, *optional*, defaults to `"en"`) --
  Language code for the lyrics (e.g., `"en"`, `"zh"`, `"ja"`).
- **num_inference_steps** (`int`, *optional*, defaults to 8) --
  The number of denoising steps. The turbo model is designed for 8 steps.
- **guidance_scale** (`float`, *optional*, defaults to 7.0) --
  Guidance scale for classifier-free guidance. A value of 1.0 disables CFG.
- **shift** (`float`, *optional*, defaults to 3.0) --
  Shift parameter for the timestep schedule (1.0, 2.0, or 3.0).
- **generator** (`torch.Generator` or `List[torch.Generator]`, *optional*) --
  A generator to make generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noise latents of shape `(batch_size, latent_length, acoustic_dim)`.
- **output_type** (`str`, *optional*, defaults to `"pt"`) --
  Output format. `"pt"` for PyTorch tensor, `"np"` for NumPy array, `"latent"` for raw latents.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether to return an `AudioPipelineOutput` or a plain tuple.
- **callback** (`Callable`, *optional*) --
  A function called every `callback_steps` steps with `(step, timestep, latents)`.
- **callback_steps** (`int`, *optional*, defaults to 1) --
  Frequency of the callback function.
- **callback_on_step_end** (`Callable`, *optional*) --
  A function that is called at the end of each denoising step during inference. The function is called
  with the following arguments: `callback_on_step_end(self: DiffusionPipeline, step: int, timestep: int,
  callback_kwargs: Dict)`. `callback_kwargs` will include a list of all tensors as specified by
  `callback_on_step_end_tensor_inputs`.
- **callback_on_step_end_tensor_inputs** (`List`, *optional*) --
  The list of tensor inputs for the `callback_on_step_end` function. The tensors specified in the list
  will be passed as `callback_kwargs` argument. You will only be able to include variables listed in the
  `._callback_tensor_inputs` attribute of your pipeline class.
- **instruction** (`str`, *optional*) --
  Custom instruction text for the generation task. If not provided, it is auto-generated based on
  `task_type`.
- **max_text_length** (`int`, *optional*, defaults to 256) --
  Maximum token length for text prompt encoding.
- **max_lyric_length** (`int`, *optional*, defaults to 2048) --
  Maximum token length for lyrics encoding.
- **bpm** (`int`, *optional*) --
  BPM (beats per minute) for music metadata. If `None`, the model estimates it.
- **keyscale** (`str`, *optional*) --
  Musical key (e.g., `"C major"`, `"A minor"`). If `None`, the model estimates it.
- **timesignature** (`str`, *optional*) --
  Time signature (e.g., `"4"` for 4/4, `"3"` for 3/4). If `None`, the model estimates it.
- **task_type** (`str`, *optional*, defaults to `"text2music"`) --
  The generation task type. One of `"text2music"`, `"cover"`, `"repaint"`, `"extract"`, `"lego"`,
  `"complete"`.
- **track_name** (`str`, *optional*) --
  Track name for `"extract"` or `"lego"` tasks (e.g., `"vocals"`, `"drums"`).
- **complete_track_classes** (`List[str]`, *optional*) --
  Track classes for the `"complete"` task.
- **src_audio** (`torch.Tensor`, *optional*) --
  Source audio tensor of shape `[channels, samples]` at 48kHz for audio-to-audio tasks (repaint, lego,
  cover, extract, complete). The audio is encoded through the VAE to produce source latents.
- **reference_audio** (`torch.Tensor`, *optional*) --
  Reference audio tensor of shape `[channels, samples]` at 48kHz for timbre conditioning. Used to extract
  timbre features for style transfer.
- **audio_codes** (`str` or `List[str]`, *optional*) --
  Audio semantic code strings (e.g. `"..."`). When provided, the task
  is automatically switched to `"cover"` mode and the registered ACE-Step audio tokenizer / detokenizer
  modules decode the 5 Hz codes into 25 Hz acoustic conditioning.
- **repainting_start** (`float`, *optional*) --
  Start time in seconds for the repaint region (for `"repaint"` and `"lego"` tasks).
- **repainting_end** (`float`, *optional*) --
  End time in seconds for the repaint region. Use `-1` or `None` for until end.
- **audio_cover_strength** (`float`, *optional*, defaults to 1.0) --
  Strength of audio cover blending (0.0 to 1.0). When 0[AudioPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/audioldm2#diffusers.AudioPipelineOutput) or `tuple`If `return_dict` is `True`, an `AudioPipelineOutput` is returned, otherwise a tuple with the generated
audio.

The call function to the pipeline for music generation.

Examples:
```py
>>> import torch
>>> import soundfile as sf
>>> from diffusers import AceStepPipeline

>>> pipe = AceStepPipeline.from_pretrained(
...     "ACE-Step/acestep-v15-xl-turbo-diffusers", torch_dtype=torch.bfloat16
... )
>>> pipe = pipe.to("cuda")

>>> # Text-to-music generation with metadata
>>> audio = pipe(
...     prompt="A beautiful piano piece with soft melodies",
...     lyrics="[verse]\nSoft notes in the morning light\n[chorus]\nMusic fills the air tonight",
...     audio_duration=30.0,
...     num_inference_steps=8,
...     bpm=120,
...     keyscale="C major",
...     timesignature="4",
... ).audios

>>> # Save the generated audio
>>> sf.write("output.wav", audio[0, 0].cpu().numpy(), 48000)

>>> # Repaint task: regenerate a section of existing stereo 48kHz audio
>>> src_audio, sr = sf.read("input.wav")
>>> src_audio = torch.from_numpy(src_audio).float().T
>>> audio = pipe(
...     prompt="Epic rock guitar solo",
...     lyrics="",
...     task_type="repaint",
...     src_audio=src_audio,
...     repainting_start=10.0,
...     repainting_end=20.0,
... ).audios

>>> # Cover task with reference audio for timbre transfer
>>> ref_audio, sr = sf.read("reference.wav")
>>> ref_audio = torch.from_numpy(ref_audio).float().T
>>> audio = pipe(
...     prompt="Pop song with bright vocals",
...     lyrics="[verse]\nHello world",
...     task_type="cover",
...     reference_audio=ref_audio,
...     audio_cover_strength=0.8,
... ).audios
```

**Parameters:**

vae ([AutoencoderOobleck](/docs/diffusers/v0.39.0/en/api/models/autoencoder_oobleck#diffusers.AutoencoderOobleck)) : Variational Auto-Encoder (VAE) model to encode and decode audio waveforms to and from latent representations.

text_encoder ([AutoModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/auto#transformers.AutoModel)) : Text encoder model (e.g., Qwen3-Embedding-0.6B) for encoding text prompts and lyrics.

tokenizer ([AutoTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/auto#transformers.AutoTokenizer)) : Tokenizer for the text encoder.

transformer ([AceStepTransformer1DModel](/docs/diffusers/v0.39.0/en/api/models/ace_step_transformer#diffusers.AceStepTransformer1DModel)) : The Diffusion Transformer (DiT) model for denoising audio latents.

condition_encoder (`AceStepConditionEncoder`) : Condition encoder that combines text, lyric, and timbre embeddings for cross-attention.

scheduler ([FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler)) : Flow-matching Euler scheduler. ACE-Step feeds the DiT timesteps in `[0, 1]`, so the scheduler is configured with `num_train_timesteps=1` and `shift=1.0` — the pipeline computes its shifted / turbo sigma schedule itself and passes it via `set_timesteps(sigmas=...)`.

**Returns:**

`[AudioPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/audioldm2#diffusers.AudioPipelineOutput) or `tuple``

If `return_dict` is `True`, an `AudioPipelineOutput` is returned, otherwise a tuple with the generated
audio.
#### check_inputs[[diffusers.AceStepPipeline.check_inputs]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ace_step/pipeline_ace_step.py#L227)

Validate user-facing arguments before we start allocating noise tensors.
#### encode_prompt[[diffusers.AceStepPipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ace_step/pipeline_ace_step.py#L396)

Encode text prompts and lyrics into embeddings.

Text prompts are encoded through the full text encoder model to produce contextual hidden states. Lyrics are
only passed through the text encoder's embedding layer (token lookup), since the lyric encoder in the condition
encoder handles the contextual encoding.

**Parameters:**

prompt (`str` or `List[str]`) : Text caption(s) describing the music.

lyrics (`str` or `List[str]`) : Lyric text(s).

device (`torch.device`) : Device for tensors.

vocal_language (`str` or `List[str]`, *optional*, defaults to `"en"`) : Language code(s) for lyrics.

audio_duration (`float`, *optional*, defaults to 60.0) : Duration of the audio in seconds.

instruction (`str`, *optional*) : Instruction text for generation.

bpm (`int`, *optional*) : BPM (beats per minute) for metadata.

keyscale (`str`, *optional*) : Musical key (e.g., `"C major"`).

timesignature (`str`, *optional*) : Time signature (e.g., `"4"` for 4/4).

max_text_length (`int`, *optional*, defaults to 256) : Maximum token length for text prompts.

max_lyric_length (`int`, *optional*, defaults to 2048) : Maximum token length for lyrics.

**Returns:**

Tuple of `(text_hidden_states, text_attention_mask, lyric_hidden_states, lyric_attention_mask)`.
#### prepare_latents[[diffusers.AceStepPipeline.prepare_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ace_step/pipeline_ace_step.py#L501)

Prepare initial noise latents for the flow matching process.

**Parameters:**

batch_size (`int`) : Number of samples to generate.

audio_duration (`float`) : Duration of audio in seconds.

dtype (`torch.dtype`) : Data type for the latents.

device (`torch.device`) : Device for the latents.

generator (`torch.Generator` or `List[torch.Generator]`, *optional*) : Random number generator(s).

latents (`torch.Tensor`, *optional*) : Pre-generated latents.

**Returns:**

Noise latents of shape `(batch_size, latent_length, acoustic_dim)`.
#### prepare_reference_audio_latents[[diffusers.AceStepPipeline.prepare_reference_audio_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ace_step/pipeline_ace_step.py#L575)

Process reference audio into acoustic latents for the timbre encoder.

The reference audio is repeated/cropped to 30 seconds (3 segments of 10 seconds each from front, middle, and
back), encoded through the VAE, and then transposed for the timbre encoder.

**Parameters:**

reference_audio (`torch.Tensor`) : Reference audio tensor of shape `[channels, samples]` at `self.sample_rate`.

batch_size (`int`) : Batch size.

device (`torch.device`) : Target device.

dtype (`torch.dtype`) : Target dtype.

**Returns:**

Tuple of `(refer_audio_acoustic, refer_audio_order_mask)`.
#### prepare_src_latents[[diffusers.AceStepPipeline.prepare_src_latents]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/ace_step/pipeline_ace_step.py#L628)

Prepare source latents for text-to-music and audio-to-audio tasks.

**Parameters:**

src_audio (`torch.Tensor`, *optional*) : Source audio tensor of shape `[channels, samples]` at `self.sample_rate`.

audio_codes (`str` or `List[str]`, *optional*) : Audio semantic code strings.

latent_length (`int`, *optional*) : Target latent length when no source audio or audio codes are given.

device (`torch.device`) : Target device.

dtype (`torch.dtype`) : Target dtype.

batch_size (`int`) : Batch size.

task_type (`str`) : Current task type.

**Returns:**

Tuple of `(src_latents, latent_length)` where `src_latents` has shape `[batch, T, D]`.
