# Stable Audio

Stable Audio was proposed in [Stable Audio Open](https://huggingface.co/papers/2407.14358) by Zach Evans et al. . it takes a text prompt as input and predicts the corresponding sound or music sample.

Stable Audio Open generates variable-length (up to 47s) stereo audio at 44.1kHz from text prompts. It comprises three components: an autoencoder that compresses waveforms into a manageable sequence length, a T5-based text embedding for text conditioning, and a transformer-based diffusion (DiT) model that operates in the latent space of the autoencoder.

Stable Audio is trained on a corpus of around 48k audio recordings, where around 47k are from Freesound and the rest are from the Free Music Archive (FMA). All audio files are licensed under CC0, CC BY, or CC Sampling+. This data is used to train the autoencoder and the DiT.

The abstract of the paper is the following:
*Open generative models are vitally important for the community, allowing for fine-tunes and serving as baselines when presenting new models. However, most current text-to-audio models are private and not accessible for artists and researchers to build upon. Here we describe the architecture and training process of a new open-weights text-to-audio model trained with Creative Commons data. Our evaluation shows that the model's performance is competitive with the state-of-the-art across various metrics. Notably, the reported FDopenl3 results (measuring the realism of the generations) showcase its potential for high-quality stereo sound synthesis at 44.1kHz.*

This pipeline was contributed by [Yoach Lacombe](https://huggingface.co/ylacombe). The original codebase can be found at [Stability-AI/stable-audio-tools](https://github.com/Stability-AI/stable-audio-tools).

## Tips

When constructing a prompt, keep in mind:

* Descriptive prompt inputs work best; use adjectives to describe the sound (for example, "high quality" or "clear") and make the prompt context specific where possible (e.g. "melodic techno with a fast beat and synths" works better than "techno").
* Using a *negative prompt* can significantly improve the quality of the generated audio. Try using a negative prompt of "low quality, average quality".

During inference:

* The _quality_ of the generated audio sample can be controlled by the `num_inference_steps` argument; higher steps give higher quality audio at the expense of slower inference.
* Multiple waveforms can be generated in one go: set `num_waveforms_per_prompt` to a value greater than 1 to enable. Automatic scoring will be performed between the generated waveforms and prompt text, and the audios ranked from best to worst accordingly.

## Quantization

Quantization helps reduce the memory requirements of very large models by storing model weights in a lower precision data type. However, quantization may have varying impact on video quality depending on the video model.

Refer to the [Quantization](../../quantization/overview) overview to learn more about supported quantization backends and selecting a quantization backend that supports your use case. The example below demonstrates how to load a quantized [StableAudioPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/stable_audio#diffusers.StableAudioPipeline) for inference with bitsandbytes.

```py
import torch
from diffusers import BitsAndBytesConfig as DiffusersBitsAndBytesConfig, StableAudioDiTModel, StableAudioPipeline
from diffusers.utils import export_to_video
from transformers import BitsAndBytesConfig as BitsAndBytesConfig, T5EncoderModel

quant_config = BitsAndBytesConfig(load_in_8bit=True)
text_encoder_8bit = T5EncoderModel.from_pretrained(
    "stabilityai/stable-audio-open-1.0",
    subfolder="text_encoder",
    quantization_config=quant_config,
    dtype=torch.float16,
)

quant_config = DiffusersBitsAndBytesConfig(load_in_8bit=True)
transformer_8bit = StableAudioDiTModel.from_pretrained(
    "stabilityai/stable-audio-open-1.0",
    subfolder="transformer",
    quantization_config=quant_config,
    dtype=torch.float16,
)

pipeline = StableAudioPipeline.from_pretrained(
    "stabilityai/stable-audio-open-1.0",
    text_encoder=text_encoder_8bit,
    transformer=transformer_8bit,
    dtype=torch.float16,
    device_map="balanced",
)

prompt = "The sound of a hammer hitting a wooden surface."
negative_prompt = "Low quality."
audio = pipeline(
    prompt,
    negative_prompt=negative_prompt,
    num_inference_steps=200,
    audio_end_in_s=10.0,
    num_waveforms_per_prompt=3,
    generator=generator,
).audios

output = audio[0].T.float().cpu().numpy()
sf.write("hammer.wav", output, pipeline.vae.sampling_rate)
```

## StableAudioPipeline[[diffusers.StableAudioPipeline]]

#### diffusers.StableAudioPipeline[[diffusers.StableAudioPipeline]]

```python
diffusers.StableAudioPipeline(vae: AutoencoderOobleck, text_encoder: T5EncoderModel, projection_model: StableAudioProjectionModel, tokenizer: T5Tokenizer, transformer: StableAudioDiTModel, scheduler: EDMDPMSolverMultistepScheduler)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio/pipeline_stable_audio.py#L78)

**Parameters:**

vae ([AutoencoderOobleck](/docs/diffusers/v0.40.0/en/api/models/autoencoder_oobleck#diffusers.AutoencoderOobleck)) : Variational Auto-Encoder (VAE) model to encode and decode images to and from latent representations.

text_encoder ([T5EncoderModel](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/t5#transformers.T5EncoderModel)) : Frozen text-encoder. StableAudio uses the encoder of [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel), specifically the [google-t5/t5-base](https://huggingface.co/google-t5/t5-base) variant.

projection_model (`StableAudioProjectionModel`) : A trained model used to linearly project the hidden-states from the text encoder model and the start and end seconds. The projected hidden-states from the encoder and the conditional seconds are concatenated to give the input to the transformer model.

tokenizer ([T5Tokenizer](https://huggingface.co/docs/transformers/v5.15.1/en/model_doc/t5#transformers.T5Tokenizer)) : Tokenizer to tokenize text for the frozen text-encoder.

transformer ([StableAudioDiTModel](/docs/diffusers/v0.40.0/en/api/models/stable_audio_transformer#diffusers.StableAudioDiTModel)) : A `StableAudioDiTModel` to denoise the encoded audio latents.

scheduler ([EDMDPMSolverMultistepScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/edm_multistep_dpm_solver#diffusers.EDMDPMSolverMultistepScheduler)) : A scheduler to be used in combination with `transformer` to denoise the encoded audio latents.

Pipeline for text-to-audio generation using StableAudio.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.40.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

#### __call__[[diffusers.StableAudioPipeline.__call__]]

```python
__call__(prompt: str | list[str] = None, audio_end_in_s: float | None = None, audio_start_in_s: float | None = 0.0, num_inference_steps: int = 100, guidance_scale: float = 7.0, negative_prompt: str | list[str] | None = None, num_waveforms_per_prompt: int | None = 1, eta: float = 0.0, generator: typing.Union[torch.Generator, list[torch.Generator], NoneType] = None, latents: typing.Optional[torch.Tensor] = None, initial_audio_waveforms: typing.Optional[torch.Tensor] = None, initial_audio_sampling_rate: typing.Optional[torch.Tensor] = None, prompt_embeds: typing.Optional[torch.Tensor] = None, negative_prompt_embeds: typing.Optional[torch.Tensor] = None, attention_mask: typing.Optional[torch.LongTensor] = None, negative_attention_mask: typing.Optional[torch.LongTensor] = None, return_dict: bool = True, callback: typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None, callback_steps: int | None = 1, output_type: str | None = 'pt')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/pipelines/stable_audio/pipeline_stable_audio.py#L462)

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide audio generation. If not defined, you need to pass `prompt_embeds`.

audio_end_in_s (`float`, *optional*, defaults to 47.55) : Audio end index in seconds.

audio_start_in_s (`float`, *optional*, defaults to 0) : Audio start index in seconds.

num_inference_steps (`int`, *optional*, defaults to 100) : The number of denoising steps. More denoising steps usually lead to a higher quality audio at the expense of slower inference.

guidance_scale (`float`, *optional*, defaults to 7.0) : A higher guidance scale value encourages the model to generate audio that is closely linked to the text `prompt` at the expense of lower sound quality. Guidance scale is enabled when `guidance_scale > 1`.

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts to guide what to not include in audio generation. If not defined, you need to pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale < 1`).

num_waveforms_per_prompt (`int`, *optional*, defaults to 1) : The number of waveforms to generate per prompt.

eta (`float`, *optional*, defaults to 0.0) : Corresponds to parameter eta (η) from the [DDIM](https://huggingface.co/papers/2010.02502) paper. Only applies to the [DDIMScheduler](/docs/diffusers/v0.40.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), and is ignored in other schedulers.

generator (`torch.Generator` or `list[torch.Generator]`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make generation deterministic.

latents (`torch.Tensor`, *optional*) : Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for audio generation. Can be used to tweak the same generation with different prompts. If not provided, a latents tensor is generated by sampling using the supplied random `generator`.

initial_audio_waveforms (`torch.Tensor`, *optional*) : Optional initial audio waveforms to use as the initial audio waveform for generation. Must be of shape `(batch_size, num_channels, audio_length)` or `(batch_size, audio_length)`, where `batch_size` corresponds to the number of prompts passed to the model.

initial_audio_sampling_rate (`int`, *optional*) : Sampling rate of the `initial_audio_waveforms`, if they are provided. Must be the same as the model.

prompt_embeds (`torch.Tensor`, *optional*) : Pre-computed text embeddings from the text encoder model. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be computed from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-computed negative text embeddings from the text encoder model. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be computed from `negative_prompt` input argument.

attention_mask (`torch.LongTensor`, *optional*) : Pre-computed attention mask to be applied to the `prompt_embeds`. If not provided, attention mask will be computed from `prompt` input argument.

negative_attention_mask (`torch.LongTensor`, *optional*) : Pre-computed attention mask to be applied to the `negative_text_audio_duration_embeds`.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) instead of a plain tuple.

callback (`Callable`, *optional*) : A function that calls every `callback_steps` steps during inference. The function is called with the following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.

callback_steps (`int`, *optional*, defaults to 1) : The frequency at which the `callback` function is called. If not specified, the callback is called at every step.

output_type (`str`, *optional*, defaults to `"pt"`) : The output format of the generated audio. Choose between `"np"` to return a NumPy `np.ndarray` or `"pt"` to return a PyTorch `torch.Tensor` object. Set to `"latent"` to return the latent diffusion model (LDM) output.

**Returns:** [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.40.0/en/api/pipelines/stable_diffusion/image_variation#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated audio.

The call function to the pipeline for generation.

Examples:
```py
>>> import scipy
>>> import torch
>>> import soundfile as sf
>>> from diffusers import StableAudioPipeline

>>> repo_id = "stabilityai/stable-audio-open-1.0"
>>> pipe = StableAudioPipeline.from_pretrained(repo_id, torch_dtype=torch.float16)
>>> pipe = pipe.to("cuda")

>>> # define the prompts
>>> prompt = "The sound of a hammer hitting a wooden surface."
>>> negative_prompt = "Low quality."

>>> # set the seed for generator
>>> generator = torch.Generator("cuda").manual_seed(0)

>>> # run the generation
>>> audio = pipe(
...     prompt,
...     negative_prompt=negative_prompt,
...     num_inference_steps=200,
...     audio_end_in_s=10.0,
...     num_waveforms_per_prompt=3,
...     generator=generator,
... ).audios

>>> output = audio[0].T.float().cpu().numpy()
>>> sf.write("hammer.wav", output, pipe.vae.sampling_rate)
```
