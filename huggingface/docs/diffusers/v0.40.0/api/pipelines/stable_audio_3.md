# Stable Audio 3

Stable Audio 3 (SA3) is a text-to-audio model from [Stability AI](https://stability.ai/) that generates high-quality
stereo audio at 44.1 kHz. It uses a rectified-flow DiT conditioned on two signals:

* **Text** — encoded by a frozen T5Gemma encoder and injected via cross-attention.
* **Duration** — a float (seconds) embedded by [StableAudio3DurationEmbedder](/docs/diffusers/v0.40.0/en/api/pipelines/stable_audio_3#diffusers.StableAudio3DurationEmbedder) and used as a global conditioning
  vector for adaptive layer normalisation.

Audio is decoded by the SAME (Semantically-Aligned Music Encoder) autoencoder, [AutoencoderSAME](/docs/diffusers/v0.40.0/en/api/models/autoencoder_same#diffusers.AutoencoderSAME).

Both checkpoints use [FlowMatchEulerDiscreteScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/flow_match_euler_discrete#diffusers.FlowMatchEulerDiscreteScheduler) with a log-SNR-uniform sigma schedule, differing only in
`stochastic_sampling` and the default step count:

| Checkpoint | `diffusion_objective` | `stochastic_sampling` | `num_inference_steps` |
|---|---|---|---|
| `stable-audio-3-medium-base` | `rectified_flow` | `False` (deterministic Euler) | **100** (not distilled) |
| `stable-audio-3-medium` (distilled) | `rf_denoiser` | `True` (ping-pong re-noise) | **8** (distilled for 8 steps) |

The correct scheduler config is baked into each converted checkpoint, so `num_inference_steps` defaults to the right
value when you leave it unset. Only pass it to override.

Original codebase: [Stability-AI/stable-audio-3](https://github.com/Stability-AI/stable-audio-3).

## Converting original checkpoints

The Stability AI checkpoints are not published in diffusers format, so convert them locally. The script downloads the
checkpoint's `model_config.json` and selects the scheduler from its `diffusion_objective`:

```bash
python scripts/convert_stable_audio_3_to_diffusers.py \
  --checkpoint_path stabilityai/stable-audio-3-medium-base \
  --text_encoder_repo google/t5gemma-b-b-ul2 \
  --output_dir /tmp/sa3-diffusers-euler \
  --dtype float32
```

> [!TIP]
> `stable-audio-3-medium-base` is a **gated** repo. Run `hf auth login` with an account that has access before
> converting, otherwise the download fails with a 401.

## Usage example

Load the converted checkpoint from its local output directory (install
[`soundfile`](https://pypi.org/project/soundfile/) with `pip install soundfile`):

```py
import torch
import soundfile as sf
from diffusers import StableAudio3Pipeline

pipe = StableAudio3Pipeline.from_pretrained("/tmp/sa3-diffusers-euler", torch_dtype=torch.float32)
pipe = pipe.to("cuda")

generator = torch.Generator("cuda").manual_seed(0)
audio = pipe(
    "A gentle piano melody with soft strings in a concert hall",
    duration=10.0,  # seconds; latent length is computed automatically
    generator=generator,
).audios

sf.write("sa3_output.wav", audio[0].T.cpu().float().numpy(), samplerate=44100)
```

The pipeline is also registered with [AutoPipelineForText2Audio](/docs/diffusers/v0.40.0/en/api/pipelines/auto_pipeline#diffusers.AutoPipelineForText2Audio), which resolves the checkpoint to
`StableAudio3Pipeline` automatically:

```py
from diffusers import AutoPipelineForText2Audio

pipe = AutoPipelineForText2Audio.from_pretrained("/tmp/sa3-diffusers-euler", torch_dtype=torch.float32)
```

> [!NOTE]
> The examples use a local path because `stabilityai/stable-audio-3-medium` and `stable-audio-3-medium-base` are not
> yet published in diffusers format (loading by repo id returns a 404). Once published, the repo id works in place of
> the local path.

## Tips

* Use `torch.float32` on CPU or MPS (Apple Silicon) — `torch.float16` on MPS produces noise.
* The distilled model (`stable-audio-3-medium`) is **adversarially distilled** — guidance is baked into the weights.
  Leave `guidance_scale=1.0` (the default) and don't pass a `negative_prompt` for that checkpoint; both only do
  something useful for the non-distilled `stable-audio-3-medium-base` checkpoint.
* `silence_padding_duration` (default `0.0`) adds silent headroom at the end of the latent sequence. Leave it at `0.0`
  unless the model is trained to mask that padding — otherwise the extra frames drain output energy and the result
  gets quiet.
* Set `num_waveforms_per_prompt > 1` to generate multiple clips per prompt.

## StableAudio3Pipeline[[diffusers.StableAudio3Pipeline]]

#### diffusers.StableAudio3Pipeline[[diffusers.StableAudio3Pipeline]]

```python
diffusers.StableAudio3Pipeline(vae: AutoencoderSAME, text_encoder: T5GemmaEncoderModel, tokenizer: GemmaTokenizer, duration_embedder: StableAudio3DurationEmbedder, transformer: StableAudio3DiTModel, scheduler: FlowMatchEulerDiscreteScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3.py#L157)

**Parameters:**

vae ([*AutoencoderSAME*]) : SAME autoencoder used to encode and decode audio latents.

text_encoder ([*~transformers.T5GemmaEncoderModel*]) : Frozen T5Gemma text encoder (`google/t5gemma-b-b-ul2`).

tokenizer ([*~transformers.GemmaTokenizerFast*]) : Tokenizer for the text encoder.

duration_embedder ([*StableAudio3DurationEmbedder*]) : Maps `duration` in seconds to a global conditioning vector for AdaLN in each DiT block.

transformer ([*StableAudio3DiTModel*]) : The rectified-flow velocity-prediction DiT.

scheduler ([*FlowMatchEulerDiscreteScheduler*]) : Scheduler for the iterative denoising loop. The production (distilled) SA3 Medium checkpoint uses *stochastic_sampling=True* for exactly 8 ping-pong steps; the non-distilled base checkpoint uses *stochastic_sampling=False* for ~100 deterministic Euler steps.

Pipeline for text-to-audio generation using Stable Audio 3.

SA3 uses a distilled rectified-flow DiT with ping-pong sampling. Classifier-free guidance (`guidance_scale` /
`negative_prompt`) is unnecessary for the distilled checkpoint (leave `guidance_scale=1.0`, the default) but is
meaningful for the non-distilled `stable-audio-3-medium-base` checkpoint.

This model inherits from [*DiffusionPipeline*]. Check the superclass documentation for the generic methods
implemented for all pipelines.

#### __call__[[diffusers.StableAudio3Pipeline.__call__]]

```python
__call__(prompt: typing.Union[str, typing.List[str], NoneType] = None, duration: float = 10.0, num_inference_steps: typing.Optional[int] = None, logsnr_min: float = -6.2, logsnr_max: float = 2.0, silence_padding_duration: float = 0.0, guidance_scale: float = 1.0, negative_prompt: typing.Union[str, typing.List[str], NoneType] = None, num_waveforms_per_prompt: int = 1, generator: typing.Union[torch.Generator, typing.List[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, encoder_attention_mask: typing.Optional[torch.LongTensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, negative_encoder_attention_mask: typing.Optional[torch.LongTensor] = None, return_dict: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int, dict], dict]] = None, callback_on_step_end_tensor_inputs: typing.List[str] = ['latents'], output_type: str = 'pt')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3.py#L398)

**Parameters:**

prompt (*str* or *list[str]*, *optional*) : Text prompt(s). Pass `prompt_embeds` instead to skip tokenization and encoding.

duration (*float*, defaults to 10.0) : Requested output duration in seconds.

num_inference_steps (*int*, *optional*) : Number of denoising steps. When `None` (default), the step count is chosen from the scheduler's *stochastic_sampling* config: **8** when *True* (the distilled ping-pong-style checkpoint) and **100** when *False* (the non-distilled base checkpoint). Pass an explicit value to override.

logsnr_min (*float*, defaults to -6.2) : Minimum log-SNR value for the noise schedule — maps to the high-noise start of the schedule.

logsnr_max (*float*, defaults to 2.0) : Maximum log-SNR value for the noise schedule — maps to the low-noise end of the schedule.

silence_padding_duration (*float*, defaults to 0.0) : Extra seconds of latent context generated beyond the target content, giving the model headroom at the boundary; the output is trimmed back to *duration*. Defaults to 0.0 (disabled). Increase only if the model is trained/distilled to mask this padding — otherwise the extra frames drain output energy.

guidance_scale (*float*, defaults to 1.0) : Classifier-free guidance scale. `1.0` disables guidance (the default, and the only sensible value for the distilled SA3 Medium checkpoint, whose CFG is baked into the weights). Values `> 1.0` are meaningful for the non-distilled `stable-audio-3-medium-base` checkpoint; higher values follow the prompt more closely at the cost of diversity.

negative_prompt (*str* or *list[str]*, *optional*) : Prompt(s) describing what to steer away from when `guidance_scale > 1.0`. Defaults to an empty string (unconditional) when `guidance_scale > 1.0` and neither this nor *negative_prompt_embeds* is given. Ignored when `guidance_scale &amp;lt;= 1.0`.

num_waveforms_per_prompt (*int*, defaults to 1) : Number of waveforms to generate per prompt.

generator (*torch.Generator* or *list[torch.Generator]*, *optional*) : For deterministic generation and reproducible re-noise in the ping-pong loop.

latents (*torch.Tensor*, *optional*) : Pre-generated starting latents. If `None` a fresh Gaussian tensor is sampled.

prompt_embeds (*torch.Tensor*, *optional*) : Pre-computed text embeddings `(batch, seq_len, 768)`.

encoder_attention_mask (*torch.LongTensor*, *optional*) : Boolean mask for pre-computed embeddings.

negative_prompt_embeds (*torch.Tensor*, *optional*) : Pre-computed negative text embeddings, as an alternative to *negative_prompt*.

negative_encoder_attention_mask (*torch.LongTensor*, *optional*) : Boolean mask for pre-computed negative embeddings.

return_dict (*bool*, defaults to *True*) : Return an *AudioPipelineOutput* or a plain tuple.

callback_on_step_end (*Callable*, *optional*) : Called at the end of each denoising step with *(self, step_idx, timestep, callback_kwargs)*, where *callback_kwargs* contains the tensors listed in *callback_on_step_end_tensor_inputs*. Must return a dict with the (optionally modified) tensors to use for the rest of the loop.

callback_on_step_end_tensor_inputs (*list[str]*, defaults to *["latents"]*) : The tensors passed to *callback_on_step_end*. Must be a subset of *self._callback_tensor_inputs*.

output_type (*str*, defaults to `"pt"`) : `"pt"` for a PyTorch tensor, `"np"` for a NumPy array, or `"latent"` to skip decoding and return the raw latents.

**Returns:** [*~pipelines.AudioPipelineOutput*] or *tuple*

`.audios` is a tensor / array of shape `(batch * num_waveforms_per_prompt, audio_channels, samples)`.

Generate audio from a text prompt.

Examples:
```py
>>> import torch
>>> import soundfile as sf
>>> from diffusers import StableAudio3Pipeline

>>> pipe = StableAudio3Pipeline.from_pretrained("stabilityai/stable-audio-3-medium", torch_dtype=torch.float16)
>>> pipe = pipe.to("cuda")

>>> generator = torch.Generator("cuda").manual_seed(0)
>>> audio = pipe(
...     "A gentle piano melody with soft strings in a concert hall",
...     duration=10.0,
...     generator=generator,
... ).audios

>>> sf.write("output.wav", audio[0].T.cpu().float().numpy(), samplerate=pipe.vae.config.sampling_rate)
```

#### encode_duration[[diffusers.StableAudio3Pipeline.encode_duration]]

```python
encode_duration(duration: float, device: device, num_waveforms_per_prompt: int, batch_size: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3.py#L289)

**Parameters:**

duration : Duration in seconds, applied to every sample in the batch.

device : Target device.

num_waveforms_per_prompt : Tile factor.

batch_size : Number of prompts.

**Returns:**

`(batch * num_waveforms_per_prompt, output_dim)` tensor.

Embed the duration value into the global conditioning vector.

#### encode_prompt[[diffusers.StableAudio3Pipeline.encode_prompt]]

```python
encode_prompt(prompt: typing.Union[str, typing.List[str], NoneType], device: device, num_waveforms_per_prompt: int, prompt_embeds: typing.Optional[torch.Tensor] = None, encoder_attention_mask: typing.Optional[torch.LongTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3.py#L223)

**Parameters:**

prompt : Text prompt or list of prompts.  Ignored when `prompt_embeds` is provided.

device : Target device.

num_waveforms_per_prompt : How many output waveforms to generate per prompt; conditioning tensors are tiled accordingly.

prompt_embeds : Pre-computed text embeddings `(batch, seq_len, hidden_size)`.

encoder_attention_mask : Boolean mask `(batch, seq_len)` for pre-computed embeddings; `1` = real token, `0` = pad.

**Returns:**

`(prompt_embeds, encoder_attention_mask)` both tiled to `batch * num_waveforms_per_prompt`.

Encode text prompt(s) into cross-attention conditioning tensors.

#### prepare_cross_attention[[diffusers.StableAudio3Pipeline.prepare_cross_attention]]

```python
prepare_cross_attention(prompt_embeds: Tensor, encoder_attention_mask: Tensor, global_hidden_states: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3.py#L313)

**Returns:**

`(context, context_mask)` of shapes `(batch, T_text + 1, dim)` and `(batch, T_text + 1)`.

Build the cross-attention context by appending the duration embedding as an extra token.

SA3 routes the `seconds_total` conditioner to *both* the global (AdaLN) input and the cross-attention context
(`cross_attention_cond_ids = ["prompt", "seconds_total"]`). The duration embedding is concatenated after the
text tokens, and the attention mask is extended with one valid entry.

## StableAudio3InpaintPipeline[[diffusers.StableAudio3InpaintPipeline]]

#### diffusers.StableAudio3InpaintPipeline[[diffusers.StableAudio3InpaintPipeline]]

```python
diffusers.StableAudio3InpaintPipeline(vae: AutoencoderSAME, text_encoder: T5GemmaEncoderModel, tokenizer: GemmaTokenizer, duration_embedder: StableAudio3DurationEmbedder, transformer: StableAudio3DiTModel, scheduler: FlowMatchEulerDiscreteScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3_inpaint.py#L158)

**Parameters:**

vae ([*AutoencoderSAME*]) : SAME autoencoder used to encode and decode audio latents.

text_encoder ([*~transformers.T5GemmaEncoderModel*]) : Frozen T5Gemma text encoder (`google/t5gemma-b-b-ul2`).

tokenizer ([*~transformers.GemmaTokenizerFast*]) : Tokenizer for the text encoder.

duration_embedder ([*StableAudio3DurationEmbedder*]) : Maps `duration` in seconds to a global conditioning vector for AdaLN in each DiT block.

transformer ([*StableAudio3DiTModel*]) : The rectified-flow velocity-prediction DiT.

scheduler ([*FlowMatchEulerDiscreteScheduler*]) : Scheduler for the iterative denoising loop. The production (distilled) SA3 Medium checkpoint uses *stochastic_sampling=True* for exactly 8 ping-pong steps; the non-distilled base checkpoint uses *stochastic_sampling=False* for ~100 deterministic Euler steps.

Audio inpainting pipeline for Stable Audio 3.

Shares its text-to-audio logic with [*StableAudio3Pipeline*] (kept in sync via *# Copied from*). When `audio` and
`mask` are provided, encodes the reference audio with the frozen SAME encoder and injects `masked_latent ∥ mask` as local-additive conditioning into each DiT block via the transformer's `local_add_cond` pathway
(`to_local_embed`).

Call signature extension (see `__call__`):
audio (*torch.Tensor* of shape `(batch, channels, samples)`):
Reference audio waveform at `vae.config.sampling_rate` Hz.
mask (*torch.Tensor* of shape `(batch, 1, latent_length)`):
Per-frame binary mask in latent space. `1` = preserve original audio; `0` = region to be inpainted.

#### __call__[[diffusers.StableAudio3InpaintPipeline.__call__]]

```python
__call__(prompt: typing.Union[str, typing.List[str], NoneType] = None, duration: float = 10.0, audio: typing.Optional[torch.Tensor] = None, mask: typing.Optional[torch.Tensor] = None, mask_start_seconds: typing.Union[float, typing.List[float], NoneType] = None, mask_end_seconds: typing.Union[float, typing.List[float], NoneType] = None, num_inference_steps: typing.Optional[int] = None, logsnr_min: float = -6.2, logsnr_max: float = 2.0, silence_padding_duration: float = 0.0, num_waveforms_per_prompt: int = 1, generator: typing.Union[torch.Generator, typing.List[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, encoder_attention_mask: typing.Optional[torch.LongTensor] = None, return_dict: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int, dict], dict]] = None, callback_on_step_end_tensor_inputs: typing.List[str] = ['latents'], output_type: str = 'pt')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3_inpaint.py#L455)

**Parameters:**

prompt (*str* or *list[str]*, *optional*) : Text prompt(s).

duration (*float*, defaults to 10.0) : Output duration in seconds. Should match the reference audio.

audio (*torch.Tensor*, *optional*) : Reference waveform `(batch, channels, samples)` at `vae.config.sampling_rate` Hz. Required for inpainting.

mask (*torch.Tensor*, *optional*) : Per-frame latent-space mask `(batch, 1, L)` with 0 = inpaint region, 1 = preserve. Either `mask` or `mask_start_seconds` / `mask_end_seconds` must be provided.

mask_start_seconds (*float* or *list[float]*, *optional*) : Start time(s) of the inpaint region in seconds.

mask_end_seconds (*float* or *list[float]*, *optional*) : End time(s) of the inpaint region (must pair with `mask_start_seconds`).

num_inference_steps (*int*, *optional*) : Number of denoising steps. When `None` (default), the step count is chosen from the scheduler's *stochastic_sampling* config, matching [*StableAudio3Pipeline*].

logsnr_min (*float*, defaults to -6.2) : Minimum log-SNR value for the noise schedule — maps to the high-noise start of the schedule.

logsnr_max (*float*, defaults to 2.0) : Maximum log-SNR value for the noise schedule — maps to the low-noise end of the schedule.

silence_padding_duration (*float*, defaults to 0.0) : Extra latent headroom after the target content.

num_waveforms_per_prompt (*int*, defaults to 1) : Waveforms per prompt.

generator : RNG for reproducibility.

latents : Pre-generated starting noise (`None` → sample fresh).

prompt_embeds : Pre-computed text embeddings.

encoder_attention_mask : Mask for pre-computed embeddings.

return_dict (*bool*, defaults to *True*) : Return *AudioPipelineOutput* or tuple.

callback_on_step_end (*Callable*, *optional*) : Called at the end of each denoising step with *(self, step_idx, timestep, callback_kwargs)*. Must return a dict with the (optionally modified) tensors to use for the rest of the loop.

callback_on_step_end_tensor_inputs (*list[str]*, defaults to *["latents"]*) : The tensors passed to *callback_on_step_end*. Must be a subset of *self._callback_tensor_inputs*.

output_type (*str*, defaults to `"pt"`) : `"pt"` / `"np"` / `"latent"`.

**Returns:**

[*~pipelines.AudioPipelineOutput*] with `.audios`.

Generate inpainted audio conditioned on a text prompt and reference.

Examples:
```py
>>> import torch
>>> import soundfile as sf
>>> import torchaudio
>>> from diffusers import StableAudio3InpaintPipeline

>>> pipe = StableAudio3InpaintPipeline.from_pretrained(
...     "stabilityai/stable-audio-3-medium", torch_dtype=torch.float16
... )
>>> pipe = pipe.to("cuda")

>>> audio, sr = torchaudio.load("reference.wav")
>>> audio = torchaudio.functional.resample(audio, sr, pipe.vae.config.sampling_rate).unsqueeze(0).to("cuda")

>>> generator = torch.Generator("cuda").manual_seed(0)
>>> audio = pipe(
...     "A gentle piano melody with soft strings in a concert hall",
...     duration=10.0,
...     audio=audio,
...     mask_start_seconds=4.0,
...     mask_end_seconds=6.0,
...     generator=generator,
... ).audios

>>> sf.write("output.wav", audio[0].T.cpu().float().numpy(), samplerate=pipe.vae.config.sampling_rate)
```

#### encode_duration[[diffusers.StableAudio3InpaintPipeline.encode_duration]]

```python
encode_duration(duration: float, device: device, num_waveforms_per_prompt: int, batch_size: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3_inpaint.py#L290)

**Parameters:**

duration : Duration in seconds, applied to every sample in the batch.

device : Target device.

num_waveforms_per_prompt : Tile factor.

batch_size : Number of prompts.

**Returns:**

`(batch * num_waveforms_per_prompt, output_dim)` tensor.

Embed the duration value into the global conditioning vector.

#### encode_prompt[[diffusers.StableAudio3InpaintPipeline.encode_prompt]]

```python
encode_prompt(prompt: typing.Union[str, typing.List[str], NoneType], device: device, num_waveforms_per_prompt: int, prompt_embeds: typing.Optional[torch.Tensor] = None, encoder_attention_mask: typing.Optional[torch.LongTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3_inpaint.py#L223)

**Parameters:**

prompt : Text prompt or list of prompts.  Ignored when `prompt_embeds` is provided.

device : Target device.

num_waveforms_per_prompt : How many output waveforms to generate per prompt; conditioning tensors are tiled accordingly.

prompt_embeds : Pre-computed text embeddings `(batch, seq_len, hidden_size)`.

encoder_attention_mask : Boolean mask `(batch, seq_len)` for pre-computed embeddings; `1` = real token, `0` = pad.

**Returns:**

`(prompt_embeds, encoder_attention_mask)` both tiled to `batch * num_waveforms_per_prompt`.

Encode text prompt(s) into cross-attention conditioning tensors.

#### prepare_cross_attention[[diffusers.StableAudio3InpaintPipeline.prepare_cross_attention]]

```python
prepare_cross_attention(prompt_embeds: Tensor, encoder_attention_mask: Tensor, global_hidden_states: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3_inpaint.py#L315)

**Returns:**

`(context, context_mask)` of shapes `(batch, T_text + 1, dim)` and `(batch, T_text + 1)`.

Build the cross-attention context by appending the duration embedding as an extra token.

SA3 routes the `seconds_total` conditioner to *both* the global (AdaLN) input and the cross-attention context
(`cross_attention_cond_ids = ["prompt", "seconds_total"]`). The duration embedding is concatenated after the
text tokens, and the attention mask is extended with one valid entry.

## StableAudio3AudioToAudioPipeline[[diffusers.StableAudio3AudioToAudioPipeline]]

Generates a variation of a reference audio clip: the whole reference is noised to `init_noise_level` and denoised
from there, unlike [StableAudio3InpaintPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/stable_audio_3#diffusers.StableAudio3InpaintPipeline)'s per-frame local-additive conditioning which preserves specific
frames exactly.

#### diffusers.StableAudio3AudioToAudioPipeline[[diffusers.StableAudio3AudioToAudioPipeline]]

```python
diffusers.StableAudio3AudioToAudioPipeline(vae: AutoencoderSAME, text_encoder: T5GemmaEncoderModel, tokenizer: GemmaTokenizer, duration_embedder: StableAudio3DurationEmbedder, transformer: StableAudio3DiTModel, scheduler: FlowMatchEulerDiscreteScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3_audio2audio.py#L154)

**Parameters:**

vae ([*AutoencoderSAME*]) : SAME autoencoder used to encode and decode audio latents.

text_encoder ([*~transformers.T5GemmaEncoderModel*]) : Frozen T5Gemma text encoder (`google/t5gemma-b-b-ul2`).

tokenizer ([*~transformers.GemmaTokenizerFast*]) : Tokenizer for the text encoder.

duration_embedder ([*StableAudio3DurationEmbedder*]) : Maps `duration` in seconds to a global conditioning vector for AdaLN in each DiT block.

transformer ([*StableAudio3DiTModel*]) : The rectified-flow velocity-prediction DiT.

scheduler ([*FlowMatchEulerDiscreteScheduler*]) : Scheduler for the iterative denoising loop. The production (distilled) SA3 Medium checkpoint uses *stochastic_sampling=True* for exactly 8 ping-pong steps; the non-distilled base checkpoint uses *stochastic_sampling=False* for ~100 deterministic Euler steps.

Audio-to-audio variation pipeline for Stable Audio 3.

Shares its text-to-audio logic with [*StableAudio3Pipeline*] (kept in sync via *# Copied from*). Encodes the
reference audio with the frozen SAME encoder, mixes it with fresh noise according to `init_noise_level`, and
denoises from there — the whole signal is noised/denoised globally, unlike [*StableAudio3InpaintPipeline*]'s
per-frame local-additive conditioning.

Call signature extension (see `__call__`):
audio (*torch.Tensor* of shape `(batch, channels, samples)`):
Reference audio waveform at `vae.config.sampling_rate` Hz.
init_noise_level (*float*):
How much noise to mix into the reference before denoising. `1.0` = full noise (equivalent to
text-to-audio); lower values retain more of the reference.

#### __call__[[diffusers.StableAudio3AudioToAudioPipeline.__call__]]

```python
__call__(prompt: typing.Union[str, typing.List[str], NoneType] = None, duration: float = 10.0, audio: typing.Optional[torch.Tensor] = None, init_noise_level: float = 1.0, num_inference_steps: typing.Optional[int] = None, logsnr_min: float = -6.2, logsnr_max: float = 2.0, silence_padding_duration: float = 0.0, num_waveforms_per_prompt: int = 1, generator: typing.Union[torch.Generator, typing.List[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, encoder_attention_mask: typing.Optional[torch.LongTensor] = None, return_dict: bool = True, callback_on_step_end: typing.Optional[typing.Callable[[int, int, dict], dict]] = None, callback_on_step_end_tensor_inputs: typing.List[str] = ['latents'], output_type: str = 'pt')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3_audio2audio.py#L424)

**Parameters:**

prompt (*str* or *list[str]*, *optional*) : Text prompt(s).

duration (*float*, defaults to 10.0) : Output duration in seconds. Should match the reference audio.

audio (*torch.Tensor*, *optional*) : Reference waveform `(batch, channels, samples)` at `vae.config.sampling_rate` Hz. Required.

init_noise_level (*float*, defaults to 1.0) : Noise level (in `(0, 1]`) mixed into the reference before denoising: `x_start = (1 - init_noise_level) * reference_latents + init_noise_level * noise`. `1.0` discards the reference entirely (equivalent to [*StableAudio3Pipeline*]); lower values retain progressively more of the reference's structure while still running the full step count.

num_inference_steps (*int*, *optional*) : Number of denoising steps. When `None` (default), the step count is chosen from the scheduler's *stochastic_sampling* config, matching [*StableAudio3Pipeline*].

logsnr_min (*float*, defaults to -6.2) : Minimum log-SNR value for the noise schedule — maps to the high-noise start of the schedule.

logsnr_max (*float*, defaults to 2.0) : Maximum log-SNR value for the noise schedule — maps to the low-noise end of the schedule.

silence_padding_duration (*float*, defaults to 0.0) : Extra latent headroom after the target content.

num_waveforms_per_prompt (*int*, defaults to 1) : Waveforms per prompt.

generator : RNG for reproducibility.

latents : Pre-generated starting noise (`None` → sample fresh).

prompt_embeds : Pre-computed text embeddings.

encoder_attention_mask : Mask for pre-computed embeddings.

return_dict (*bool*, defaults to *True*) : Return *AudioPipelineOutput* or tuple.

callback_on_step_end (*Callable*, *optional*) : Called at the end of each denoising step with *(self, step_idx, timestep, callback_kwargs)*. Must return a dict with the (optionally modified) tensors to use for the rest of the loop.

callback_on_step_end_tensor_inputs (*list[str]*, defaults to *["latents"]*) : The tensors passed to *callback_on_step_end*. Must be a subset of *self._callback_tensor_inputs*.

output_type (*str*, defaults to `"pt"`) : `"pt"` / `"np"` / `"latent"`.

**Returns:**

[*~pipelines.AudioPipelineOutput*] with `.audios`.

Generate an audio variation conditioned on a text prompt and a reference waveform.

Examples:
```py
>>> import torch
>>> import soundfile as sf
>>> import torchaudio
>>> from diffusers import StableAudio3AudioToAudioPipeline

>>> pipe = StableAudio3AudioToAudioPipeline.from_pretrained(
...     "stabilityai/stable-audio-3-medium", torch_dtype=torch.float16
... )
>>> pipe = pipe.to("cuda")

>>> audio, sr = torchaudio.load("reference.wav")
>>> audio = torchaudio.functional.resample(audio, sr, pipe.vae.config.sampling_rate).unsqueeze(0).to("cuda")

>>> generator = torch.Generator("cuda").manual_seed(0)
>>> audio = pipe(
...     "A gentle piano melody with soft strings in a concert hall",
...     duration=10.0,
...     audio=audio,
...     init_noise_level=0.6,
...     generator=generator,
... ).audios

>>> sf.write("output.wav", audio[0].T.cpu().float().numpy(), samplerate=pipe.vae.config.sampling_rate)
```

#### encode_duration[[diffusers.StableAudio3AudioToAudioPipeline.encode_duration]]

```python
encode_duration(duration: float, device: device, num_waveforms_per_prompt: int, batch_size: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3_audio2audio.py#L287)

**Parameters:**

duration : Duration in seconds, applied to every sample in the batch.

device : Target device.

num_waveforms_per_prompt : Tile factor.

batch_size : Number of prompts.

**Returns:**

`(batch * num_waveforms_per_prompt, output_dim)` tensor.

Embed the duration value into the global conditioning vector.

#### encode_prompt[[diffusers.StableAudio3AudioToAudioPipeline.encode_prompt]]

```python
encode_prompt(prompt: typing.Union[str, typing.List[str], NoneType], device: device, num_waveforms_per_prompt: int, prompt_embeds: typing.Optional[torch.Tensor] = None, encoder_attention_mask: typing.Optional[torch.LongTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3_audio2audio.py#L220)

**Parameters:**

prompt : Text prompt or list of prompts.  Ignored when `prompt_embeds` is provided.

device : Target device.

num_waveforms_per_prompt : How many output waveforms to generate per prompt; conditioning tensors are tiled accordingly.

prompt_embeds : Pre-computed text embeddings `(batch, seq_len, hidden_size)`.

encoder_attention_mask : Boolean mask `(batch, seq_len)` for pre-computed embeddings; `1` = real token, `0` = pad.

**Returns:**

`(prompt_embeds, encoder_attention_mask)` both tiled to `batch * num_waveforms_per_prompt`.

Encode text prompt(s) into cross-attention conditioning tensors.

#### prepare_cross_attention[[diffusers.StableAudio3AudioToAudioPipeline.prepare_cross_attention]]

```python
prepare_cross_attention(prompt_embeds: Tensor, encoder_attention_mask: Tensor, global_hidden_states: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/pipeline_stable_audio_3_audio2audio.py#L312)

**Returns:**

`(context, context_mask)` of shapes `(batch, T_text + 1, dim)` and `(batch, T_text + 1)`.

Build the cross-attention context by appending the duration embedding as an extra token.

SA3 routes the `seconds_total` conditioner to *both* the global (AdaLN) input and the cross-attention context
(`cross_attention_cond_ids = ["prompt", "seconds_total"]`). The duration embedding is concatenated after the
text tokens, and the attention mask is extended with one valid entry.

## StableAudio3DurationEmbedder[[diffusers.StableAudio3DurationEmbedder]]

#### diffusers.StableAudio3DurationEmbedder[[diffusers.StableAudio3DurationEmbedder]]

```python
diffusers.StableAudio3DurationEmbedder(output_dim: int = 768, fourier_dim: int = 256, min_val: float = 0.0, max_val: float = 384.0, min_freq: float = 0.5, max_freq: float = 10000.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/modeling_stable_audio_3.py#L43)

**Parameters:**

output_dim (*int*, defaults to 768) : Dimension of the output embedding. Must match the DiT's `global_cond_dim`.

fourier_dim (*int*, defaults to 256) : Internal Fourier feature dimension (must be even).

min_val (*float*, defaults to 0.0) : Minimum duration value for normalization clamping.

max_val (*float*, defaults to 384.0) : Maximum duration value for normalization clamping. Values above this are clamped. 384 seconds is the production SA3 Medium upper bound for the `seconds_total` conditioner.

min_freq (*float*, defaults to 0.5) : Minimum frequency for the exponential Fourier basis.

max_freq (*float*, defaults to 10000.0) : Maximum frequency for the exponential Fourier basis.

Embeds a duration value (in seconds) into a global conditioning vector for the Stable Audio 3 DiT (used as the
`global_hidden_states` AdaLN input).

Replicates `NumberConditioner(fourier_features_type="expo")` from the SA3 reference implementation.

#### forward[[diffusers.StableAudio3DurationEmbedder.forward]]

```python
forward(seconds: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio_3/modeling_stable_audio_3.py#L90)

**Parameters:**

seconds (`torch.Tensor` of shape `(batch,)`) : Duration values in seconds.

**Returns:**

`torch.Tensor` of shape `(batch, output_dim)` — duration embeddings.
