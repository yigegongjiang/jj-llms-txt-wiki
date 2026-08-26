# MiniMax Music 3

[MiniMax Music 3](https://huggingface.co/MiniMaxAI/MiniMax-Music3) is a music generation model that produces complete
songs up to five minutes long from lyrics and a music description, with expressive vocals and long-range structure.

The model is a hybrid of an autoregressive and a diffusion stage: an 8B Qwen3-based global language model predicts one
semantic audio token per frame while a small depth decoder fills in seven residual RVQ codebooks, and their fused
hidden states condition a 2.4B flow-matching transformer that produces Flow-VAE latents in overlapping chunks. A
DAC-style decoder turns the latents into 44.1 kHz stereo audio.

## Usage

MiniMax Music 3 is available as a modular pipeline.

```py
import soundfile as sf
import torch
from diffusers import ModularPipeline

pipe = ModularPipeline.from_pretrained("MiniMaxAI/MiniMax-Music3")
pipe.load_components(dtype=torch.bfloat16)
pipe.to("cuda")

lyrics = """[verse]
Morning light filtering through the pine
Every quiet street is yours and mine
[chorus]
Softly the world begins to breathe"""

prompt = (
    "Genre: acoustic pop. BPM: 96. Key: C major. Warm and intimate, building gently into the chorus. "
    "Vocals: soft female lead, close and breathy, light stacked harmonies in the chorus. "
    "Arrangement: fingerpicked guitar and soft piano; brushed drums and upright bass enter in the chorus."
)

audio = pipe(
    prompt=prompt,
    lyrics=lyrics,
    audio_duration=60.0,
    generator=torch.Generator("cuda").manual_seed(7),
    output="audios",
)[0]

sf.write("minimax_music3.wav", audio.T, pipe.sampling_rate)
```

## Reduce memory usage

Refer to the [Reduce memory usage](../../optimization/memory) guide for more details about the various memory saving
techniques.

The full pipeline needs ~23 GB of VRAM in bfloat16. With automatic CPU offloading a generation runs in ~22 GB of free
VRAM, and additionally group-offloading the language model fits in 8 GB.

```py
import torch
from diffusers import ComponentsManager, ModularPipeline
from diffusers.hooks.group_offloading import apply_group_offloading

manager = ComponentsManager()
manager.enable_auto_cpu_offload(device="cuda")
pipe = ModularPipeline.from_pretrained("MiniMaxAI/MiniMax-Music3", components_manager=manager)
pipe.load_components(dtype=torch.bfloat16)

# Only needed below ~22 GB of free VRAM — slower, but fits in 8 GB.
apply_group_offloading(
    pipe.language_model, onload_device=torch.device("cuda"), offload_type="leaf_level", use_stream=True
)
```

## Tips

- Structure tags such as `[intro]`, `[verse]`, `[pre-chorus]`, `[chorus]`, `[bridge]`, `[instrumental]`, `[solo]`, and
  `[outro]` must each be on their own line in `lyrics`. Text on the same line as a leading tag is dropped by the
  model's input contract.
- The music description controls the vocals: describe the vocal gender and timbre explicitly (e.g. "warm female
  vocal") or the model may drift instrumental. For fine-grained control, structure the description into global
  metadata (genre, BPM, key, emotional progression), vocal details, and arrangement.
- `audio_duration` is an upper bound — the language model may end the song earlier with a stop token. The
  autoregressive stage generates 25 frames per second of audio and dominates the runtime.
- The classifier-free guidance scale of the flow-matching stage is a guider setting (the reference inference value is
  1.7): swap it with `pipe.update_components(guider=ClassifierFreeGuidance(guidance_scale=...))`.
- The pipeline returns the vocoder's native 44.1 kHz stereo output. The reference server additionally resamples to 32
  kHz; apply your own resampling if you need that exact rate.

## MiniMaxMusic3ModularPipeline[[diffusers.MiniMaxMusic3ModularPipeline]]

#### diffusers.MiniMaxMusic3ModularPipeline[[diffusers.MiniMaxMusic3ModularPipeline]]

```python
diffusers.MiniMaxMusic3ModularPipeline(blocks: diffusers.modular_pipelines.modular_pipeline.ModularPipelineBlocks | None = None, pretrained_model_name_or_path: str | os.PathLike | None = None, components_manager: diffusers.modular_pipelines.components_manager.ComponentsManager | None = None, collection: str | None = None, workflow: str | None = None, modular_config_dict: dict[str, typing.Any] | None = None, config_dict: dict[str, typing.Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/minimax_music3/modular_pipeline.py#L22)

A ModularPipeline for lyrics- and caption-conditioned music generation with MiniMax Music 3.

## MiniMaxMusic3Blocks[[diffusers.MiniMaxMusic3Blocks]]

#### diffusers.MiniMaxMusic3Blocks[[diffusers.MiniMaxMusic3Blocks]]

```python
diffusers.MiniMaxMusic3Blocks()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/modular_pipelines/minimax_music3/modular_blocks_minimax_music3.py#L130)

Modular pipeline for lyrics- and caption-conditioned music generation using MiniMax Music 3. An autoregressive
Qwen3 language model generates per-frame semantic codes and hidden states from the lyrics and the music
description; a flow-matching transformer turns the hidden states into Flow-VAE latents chunk by chunk; and a
DAC-style vocoder decodes them into a stereo waveform at 44.1 kHz.

Components:
tokenizer (`Qwen2Tokenizer`) language_model (`Qwen3ForCausalLM`) rvq_depth_decoder
(`MiniMaxMusic3RVQDepthDecoder`) condition_encoder (`MiniMaxMusic3ConditionEncoder`) transformer
(`MiniMaxMusic3Transformer1DModel`) scheduler (`FlowMatchEulerDiscreteScheduler`) guider
(`ClassifierFreeGuidance`) vocoder (`MiniMaxMusic3Vocoder`)

Inputs:
prompt (`str`):
The music description (genre, mood, vocals, instrumentation, arrangement).
lyrics (`str`):
The lyrics to sing. Structure tags such as `[verse]` or `[chorus]` must each be on their own line; text
on the same line as a leading tag is dropped by the checkpoint's input contract.
audio_duration (`float`, *optional*, defaults to 60.0):
Upper bound on the generated audio length in seconds. The language model may stop earlier. Capped at 9000
frames (six minutes).
generator (`Generator`, *optional*):
Torch generator for deterministic generation.
num_inference_steps (`int`, *optional*, defaults to 30):
Number of flow-matching Euler steps per chunk.
output_type (`str`, *optional*, defaults to np):
Output format: 'np' or 'pt'.

Outputs:
audios (`Tensor | ndarray`):
The generated stereo waveform of shape `(batch, channels, samples)` in `[-1, 1]`.

## MiniMaxMusic3ConditionEncoder[[diffusers.MiniMaxMusic3ConditionEncoder]]

#### diffusers.MiniMaxMusic3ConditionEncoder[[diffusers.MiniMaxMusic3ConditionEncoder]]

```python
diffusers.MiniMaxMusic3ConditionEncoder(condition_hidden_dim: int = 4096, num_condition_layers: int = 8, out_dim: int = 2048, input_sampling_rate: int = 24000, input_hop_length: int = 960, output_sampling_rate: int = 44100, output_hop_length: int = 512)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/condition_embedders/condition_embedder_minimax_music3.py#L23)

Projects the per-frame hidden states of the autoregressive stage onto the Flow-VAE latent timeline.

Each generated frame carries `num_condition_layers` hidden states of size `condition_hidden_dim` (one from the
language model and one per residual codebook step). They are mixed with learned softmax weights, projected, and
resampled from the language-model frame rate to the latent frame rate with nearest-neighbor interpolation.

#### forward[[diffusers.MiniMaxMusic3ConditionEncoder.forward]]

```python
forward(hidden_states: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/condition_embedders/condition_embedder_minimax_music3.py#L48)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch, frames, num_condition_layers * condition_hidden_dim)`) : Concatenated per-frame hidden states from the autoregressive stage.

**Returns:** `torch.Tensor` of shape `(batch, latent_length, out_dim)`

the latent-aligned conditioning sequence.

## MiniMaxMusic3RVQDepthDecoder[[diffusers.MiniMaxMusic3RVQDepthDecoder]]

#### diffusers.MiniMaxMusic3RVQDepthDecoder[[diffusers.MiniMaxMusic3RVQDepthDecoder]]

```python
diffusers.MiniMaxMusic3RVQDepthDecoder(hidden_size: int = 4096, num_layers: int = 4, num_attention_heads: int = 16, intermediate_size: int = 6144, audio_vocab_size: int = 1024, num_codebooks: int = 8, max_position_embeddings: int = 16)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/minimax_music3_rvq_depth_decoder.py#L91)

The local language model of MiniMax Music 3. Within each audio frame it autoregressively predicts the seven
residual RVQ codebooks (c1..c7) from the global language model's hidden state and the frame's semantic code, and
exposes the per-step hidden states that condition the flow-matching transformer.

It also owns the embedding table for the residual codebooks, which the pipeline uses to embed complete frames for
the global language model's feedback loop.

#### forward[[diffusers.MiniMaxMusic3RVQDepthDecoder.forward]]

```python
forward(inputs_embeds: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/minimax_music3_rvq_depth_decoder.py#L127)

**Parameters:**

inputs_embeds (`torch.Tensor` of shape `(batch, steps, hidden_size)`) : Projected depth-sequence embeddings: the global hidden state followed by the embedded codes sampled so far, each passed through `projection`.

**Returns:** `torch.Tensor` of shape `(batch, steps, hidden_size)`

normalized hidden states; the last step feeds the
next codebook head.

## MiniMaxMusic3Vocoder[[diffusers.MiniMaxMusic3Vocoder]]

#### diffusers.MiniMaxMusic3Vocoder[[diffusers.MiniMaxMusic3Vocoder]]

```python
diffusers.MiniMaxMusic3Vocoder(latent_channels: int = 128, decoder_input_dim: int = 1024, decoder_hidden_dim: int = 1536, upsampling_ratios: tuple = (8, 8, 4, 2), sampling_rate: int = 44100)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/minimax_music3_vocoder.py#L71)

The Flow-VAE waveform decoder of MiniMax Music 3 (a DAC-style decoder). It decodes flow-matched latents of shape
`(batch, latent_channels, length)` into stereo waveforms at `sampling_rate`; the two audio channels are decoded as
two folded `latent_channels // 2` streams.

#### forward[[diffusers.MiniMaxMusic3Vocoder.forward]]

```python
forward(latents: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/minimax_music3_vocoder.py#L100)

**Parameters:**

latents (`torch.Tensor` of shape `(batch, latent_channels, length)`) : Flow-matched Flow-VAE latents.

**Returns:** `torch.Tensor` of shape `(batch, 2, samples)`

the stereo waveform in `[-1, 1]`.
