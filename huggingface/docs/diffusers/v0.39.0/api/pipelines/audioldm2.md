# AudioLDM 2

AudioLDM 2 was proposed in [AudioLDM 2: Learning Holistic Audio Generation with Self-supervised Pretraining](https://huggingface.co/papers/2308.05734) by Haohe Liu et al. AudioLDM 2 takes a text prompt as input and predicts the corresponding audio. It can generate text-conditional sound effects, human speech and music.

Inspired by [Stable Diffusion](https://huggingface.co/docs/diffusers/api/pipelines/stable_diffusion/overview), AudioLDM 2 is a text-to-audio _latent diffusion model (LDM)_ that learns continuous audio representations from text embeddings. Two text encoder models are used to compute the text embeddings from a prompt input: the text-branch of [CLAP](https://huggingface.co/docs/transformers/main/en/model_doc/clap) and the encoder of [Flan-T5](https://huggingface.co/docs/transformers/main/en/model_doc/flan-t5). These text embeddings are then projected to a shared embedding space by an [AudioLDM2ProjectionModel](https://huggingface.co/docs/diffusers/main/api/pipelines/audioldm2#diffusers.AudioLDM2ProjectionModel). A [GPT2](https://huggingface.co/docs/transformers/main/en/model_doc/gpt2) _language model (LM)_ is used to auto-regressively predict eight new embedding vectors, conditional on the projected CLAP and Flan-T5 embeddings. The generated embedding vectors and Flan-T5 text embeddings are used as cross-attention conditioning in the LDM. The [UNet](https://huggingface.co/docs/diffusers/main/en/api/pipelines/audioldm2#diffusers.AudioLDM2UNet2DConditionModel) of AudioLDM 2 is unique in the sense that it takes **two** cross-attention embeddings, as opposed to one cross-attention conditioning, as in most other LDMs.

The abstract of the paper is the following:

*Although audio generation shares commonalities across different types of audio, such as speech, music, and sound effects, designing models for each type requires careful consideration of specific objectives and biases that can significantly differ from those of other types. To bring us closer to a unified perspective of audio generation, this paper proposes a framework that utilizes the same learning method for speech, music, and sound effect generation. Our framework introduces a general representation of audio, called "language of audio" (LOA). Any audio can be translated into LOA based on AudioMAE, a self-supervised pre-trained representation learning model. In the generation process, we translate any modalities into LOA by using a GPT-2 model, and we perform self-supervised audio generation learning with a latent diffusion model conditioned on LOA. The proposed framework naturally brings advantages such as in-context learning abilities and reusable self-supervised pretrained AudioMAE and latent diffusion models. Experiments on the major benchmarks of text-to-audio, text-to-music, and text-to-speech demonstrate state-of-the-art or competitive performance against previous approaches. Our code, pretrained model, and demo are available at [this https URL](https://audioldm.github.io/audioldm2).*

This pipeline was contributed by [sanchit-gandhi](https://huggingface.co/sanchit-gandhi) and [Nguyễn Công Tú Anh](https://github.com/tuanh123789). The original codebase can be
found at [haoheliu/audioldm2](https://github.com/haoheliu/audioldm2).

## Tips

### Choosing a checkpoint

AudioLDM2 comes in three variants. Two of these checkpoints are applicable to the general task of text-to-audio generation. The third checkpoint is trained exclusively on text-to-music generation.

All checkpoints share the same model size for the text encoders and VAE. They differ in the size and depth of the UNet.
See table below for details on the three checkpoints:

| Checkpoint                                                      | Task          | UNet Model Size | Total Model Size | Training Data / h |
|-----------------------------------------------------------------|---------------|-----------------|------------------|-------------------|
| [audioldm2](https://huggingface.co/cvssp/audioldm2)             | Text-to-audio | 350M            | 1.1B             | 1150k             |
| [audioldm2-large](https://huggingface.co/cvssp/audioldm2-large) | Text-to-audio | 750M            | 1.5B             | 1150k             |
| [audioldm2-music](https://huggingface.co/cvssp/audioldm2-music) | Text-to-music | 350M            | 1.1B             | 665k              |
| [audioldm2-gigaspeech](https://huggingface.co/anhnct/audioldm2_gigaspeech) | Text-to-speech | 350M            | 1.1B             |10k              |
| [audioldm2-ljspeech](https://huggingface.co/anhnct/audioldm2_ljspeech) | Text-to-speech | 350M            | 1.1B             |              |

### Constructing a prompt

* Descriptive prompt inputs work best: use adjectives to describe the sound (e.g. "high quality" or "clear") and make the prompt context specific (e.g. "water stream in a forest" instead of "stream").
* It's best to use general terms like "cat" or "dog" instead of specific names or abstract objects the model may not be familiar with.
* Using a **negative prompt** can significantly improve the quality of the generated waveform, by guiding the generation away from terms that correspond to poor quality audio. Try using a negative prompt of "Low quality."

### Controlling inference

* The _quality_ of the predicted audio sample can be controlled by the `num_inference_steps` argument; higher steps give higher quality audio at the expense of slower inference.
* The _length_ of the predicted audio sample can be controlled by varying the `audio_length_in_s` argument.

### Evaluating generated waveforms:

* The quality of the generated waveforms can vary significantly based on the seed. Try generating with different seeds until you find a satisfactory generation.
* Multiple waveforms can be generated in one go: set `num_waveforms_per_prompt` to a value greater than 1. Automatic scoring will be performed between the generated waveforms and prompt text, and the audios ranked from best to worst accordingly.

The following example demonstrates how to construct good music and speech generation using the aforementioned tips: [example](https://huggingface.co/docs/diffusers/main/en/api/pipelines/audioldm2#diffusers.AudioLDM2Pipeline.__call__.example).

> [!TIP]
> Make sure to check out the Schedulers [guide](../../using-diffusers/schedulers) to learn how to explore the tradeoff between scheduler speed and quality, and see the [reuse components across pipelines](../../using-diffusers/loading#reuse-a-pipeline) section to learn how to efficiently load the same components into multiple pipelines.

## AudioLDM2Pipeline[[diffusers.AudioLDM2Pipeline]]
#### diffusers.AudioLDM2Pipeline[[diffusers.AudioLDM2Pipeline]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/pipeline_audioldm2.py#L150)

Pipeline for text-to-audio generation using AudioLDM2.

This model inherits from [DiffusionPipeline](/docs/diffusers/v0.39.0/en/api/pipelines/overview#diffusers.DiffusionPipeline). Check the superclass documentation for the generic methods
implemented for all pipelines (downloading, saving, running on a particular device, etc.).

__call__diffusers.AudioLDM2Pipeline.__call__https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/pipeline_audioldm2.py#L870[{"name": "prompt", "val": ": str | list[str] = None"}, {"name": "transcription", "val": ": str | list[str] = None"}, {"name": "audio_length_in_s", "val": ": float | None = None"}, {"name": "num_inference_steps", "val": ": int = 200"}, {"name": "guidance_scale", "val": ": float = 3.5"}, {"name": "negative_prompt", "val": ": str | list[str] | None = None"}, {"name": "num_waveforms_per_prompt", "val": ": int | None = 1"}, {"name": "eta", "val": ": float = 0.0"}, {"name": "generator", "val": ": torch._C.Generator | list[torch._C.Generator] | None = None"}, {"name": "latents", "val": ": torch.Tensor | None = None"}, {"name": "prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "generated_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "negative_generated_prompt_embeds", "val": ": torch.Tensor | None = None"}, {"name": "attention_mask", "val": ": torch.LongTensor | None = None"}, {"name": "negative_attention_mask", "val": ": torch.LongTensor | None = None"}, {"name": "max_new_tokens", "val": ": int | None = None"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "callback", "val": ": typing.Optional[typing.Callable[[int, int, torch.Tensor], NoneType]] = None"}, {"name": "callback_steps", "val": ": int | None = 1"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "output_type", "val": ": str | None = 'np'"}]- **prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide audio generation. If not defined, you need to pass `prompt_embeds`.
- **transcription** (`str` or `list[str]`, *optional*) --\
  The transcript for text to speech.
- **audio_length_in_s** (`int`, *optional*, defaults to 10.24) --
  The length of the generated audio sample in seconds.
- **num_inference_steps** (`int`, *optional*, defaults to 200) --
  The number of denoising steps. More denoising steps usually lead to a higher quality audio at the
  expense of slower inference.
- **guidance_scale** (`float`, *optional*, defaults to 3.5) --
  A higher guidance scale value encourages the model to generate audio that is closely linked to the text
  `prompt` at the expense of lower sound quality. Guidance scale is enabled when `guidance_scale > 1`.
- **negative_prompt** (`str` or `list[str]`, *optional*) --
  The prompt or prompts to guide what to not include in audio generation. If not defined, you need to
  pass `negative_prompt_embeds` instead. Ignored when not using guidance (`guidance_scale  1`, then automatic
  scoring is performed between the generated outputs and the text prompt. This scoring ranks the
  generated waveforms based on their cosine similarity with the text input in the joint text-audio
  embedding space.
- **eta** (`float`, *optional*, defaults to 0.0) --
  Corresponds to parameter eta (η) from the [DDIM](https://huggingface.co/papers/2010.02502) paper. Only
  applies to the [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), and is ignored in other schedulers.
- **generator** (`torch.Generator` or `list[torch.Generator]`, *optional*) --
  A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make
  generation deterministic.
- **latents** (`torch.Tensor`, *optional*) --
  Pre-generated noisy latents sampled from a Gaussian distribution, to be used as inputs for spectrogram
  generation. Can be used to tweak the same generation with different prompts. If not provided, a latents
  tensor is generated by sampling using the supplied random `generator`.
- **prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings. Can be used to easily tweak text inputs (prompt weighting). If not
  provided, text embeddings are generated from the `prompt` input argument.
- **negative_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings. Can be used to easily tweak text inputs (prompt weighting). If
  not provided, `negative_prompt_embeds` are generated from the `negative_prompt` input argument.
- **generated_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated text embeddings from the GPT2 language model. Can be used to easily tweak text inputs,
  *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input
  argument.
- **negative_generated_prompt_embeds** (`torch.Tensor`, *optional*) --
  Pre-generated negative text embeddings from the GPT2 language model. Can be used to easily tweak text
  inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be computed from
  `negative_prompt` input argument.
- **attention_mask** (`torch.LongTensor`, *optional*) --
  Pre-computed attention mask to be applied to the `prompt_embeds`. If not provided, attention mask will
  be computed from `prompt` input argument.
- **negative_attention_mask** (`torch.LongTensor`, *optional*) --
  Pre-computed attention mask to be applied to the `negative_prompt_embeds`. If not provided, attention
  mask will be computed from `negative_prompt` input argument.
- **max_new_tokens** (`int`, *optional*, defaults to None) --
  Number of new tokens to generate with the GPT2 language model. If not provided, number of tokens will
  be taken from the config of the model.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) instead of a
  plain tuple.
- **callback** (`Callable`, *optional*) --
  A function that calls every `callback_steps` steps during inference. The function is called with the
  following arguments: `callback(step: int, timestep: int, latents: torch.Tensor)`.
- **callback_steps** (`int`, *optional*, defaults to 1) --
  The frequency at which the `callback` function is called. If not specified, the callback is called at
  every step.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined in
  [`self.processor`](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **output_type** (`str`, *optional*, defaults to `"np"`) --
  The output format of the generated audio. Choose between `"np"` to return a NumPy `np.ndarray` or
  `"pt"` to return a PyTorch `torch.Tensor` object. Set to `"latent"` to return the latent diffusion
  model (LDM) output.0[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple`If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated audio.

The call function to the pipeline for generation.

Examples:
```py
>>> import scipy
>>> import torch
>>> from diffusers import AudioLDM2Pipeline

>>> repo_id = "cvssp/audioldm2"
>>> pipe = AudioLDM2Pipeline.from_pretrained(repo_id, torch_dtype=torch.float16)
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
...     audio_length_in_s=10.0,
...     num_waveforms_per_prompt=3,
...     generator=generator,
... ).audios

>>> # save the best audio sample (index 0) as a .wav file
>>> scipy.io.wavfile.write("techno.wav", rate=16000, data=audio[0])
```

```
#Using AudioLDM2 for Text To Speech
>>> import scipy
>>> import torch
>>> from diffusers import AudioLDM2Pipeline

>>> repo_id = "anhnct/audioldm2_gigaspeech"
>>> pipe = AudioLDM2Pipeline.from_pretrained(repo_id, torch_dtype=torch.float16)
>>> pipe = pipe.to("cuda")

>>> # define the prompts
>>> prompt = "A female reporter is speaking"
>>> transcript = "wish you have a good day"

>>> # set the seed for generator
>>> generator = torch.Generator("cuda").manual_seed(0)

>>> # run the generation
>>> audio = pipe(
...     prompt,
...     transcription=transcript,
...     num_inference_steps=200,
...     audio_length_in_s=10.0,
...     num_waveforms_per_prompt=2,
...     generator=generator,
...     max_new_tokens=512,          #Must set max_new_tokens equa to 512 for TTS
... ).audios

>>> # save the best audio sample (index 0) as a .wav file
>>> scipy.io.wavfile.write("tts.wav", rate=16000, data=audio[0])
```

**Parameters:**

vae ([AutoencoderKL](/docs/diffusers/v0.39.0/en/api/models/autoencoderkl#diffusers.AutoencoderKL)) : Variational Auto-Encoder (VAE) model to encode and decode images to and from latent representations.

text_encoder ([ClapModel](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clap#transformers.ClapModel)) : First frozen text-encoder. AudioLDM2 uses the joint audio-text embedding model [CLAP](https://huggingface.co/docs/transformers/model_doc/clap#transformers.CLAPTextModelWithProjection), specifically the [laion/clap-htsat-unfused](https://huggingface.co/laion/clap-htsat-unfused) variant. The text branch is used to encode the text prompt to a prompt embedding. The full audio-text model is used to rank generated waveforms against the text prompt by computing similarity scores.

text_encoder_2 ([`~transformers.T5EncoderModel`, `~transformers.VitsModel`]) : Second frozen text-encoder. AudioLDM2 uses the encoder of [T5](https://huggingface.co/docs/transformers/model_doc/t5#transformers.T5EncoderModel), specifically the [google/flan-t5-large](https://huggingface.co/google/flan-t5-large) variant. Second frozen text-encoder use for TTS. AudioLDM2 uses the encoder of [Vits](https://huggingface.co/docs/transformers/model_doc/vits#transformers.VitsModel).

projection_model ([AudioLDM2ProjectionModel](/docs/diffusers/v0.39.0/en/api/pipelines/audioldm2#diffusers.AudioLDM2ProjectionModel)) : A trained model used to linearly project the hidden-states from the first and second text encoder models and insert learned SOS and EOS token embeddings. The projected hidden-states from the two text encoders are concatenated to give the input to the language model. A Learned Position Embedding for the Vits hidden-states

language_model ([GPT2Model](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/gpt2#transformers.GPT2Model)) : An auto-regressive language model used to generate a sequence of hidden-states conditioned on the projected outputs from the two text encoders.

tokenizer ([RobertaTokenizer](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/roberta#transformers.RobertaTokenizer)) : Tokenizer to tokenize text for the first frozen text-encoder.

tokenizer_2 ([`~transformers.T5Tokenizer`, `~transformers.VitsTokenizer`]) : Tokenizer to tokenize text for the second frozen text-encoder.

feature_extractor ([ClapFeatureExtractor](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/clap#transformers.ClapFeatureExtractor)) : Feature extractor to pre-process generated audio waveforms to log-mel spectrograms for automatic scoring.

unet ([UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel)) : A `UNet2DConditionModel` to denoise the encoded audio latents.

scheduler ([SchedulerMixin](/docs/diffusers/v0.39.0/en/api/schedulers/overview#diffusers.SchedulerMixin)) : A scheduler to be used in combination with `unet` to denoise the encoded audio latents. Can be one of [DDIMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/ddim#diffusers.DDIMScheduler), [LMSDiscreteScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/lms_discrete#diffusers.LMSDiscreteScheduler), or [PNDMScheduler](/docs/diffusers/v0.39.0/en/api/schedulers/pndm#diffusers.PNDMScheduler).

vocoder ([SpeechT5HifiGan](https://huggingface.co/docs/transformers/v5.12.1/en/model_doc/speecht5#transformers.SpeechT5HifiGan)) : Vocoder of class `SpeechT5HifiGan` to convert the mel-spectrogram latents to the final audio waveform.

**Returns:**

`[StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) or `tuple``

If `return_dict` is `True`, [StableDiffusionPipelineOutput](/docs/diffusers/v0.39.0/en/api/pipelines/stable_diffusion/inpaint#diffusers.pipelines.stable_diffusion.StableDiffusionPipelineOutput) is returned,
otherwise a `tuple` is returned where the first element is a list with the generated audio.
#### disable_vae_slicing[[diffusers.AudioLDM2Pipeline.disable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/pipeline_audioldm2.py#L241)

Disable sliced VAE decoding. If `enable_vae_slicing` was previously enabled, this method will go back to
computing decoding in one step.
#### enable_model_cpu_offload[[diffusers.AudioLDM2Pipeline.enable_model_cpu_offload]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/pipeline_audioldm2.py#L254)

Offloads all models to CPU using accelerate, reducing memory usage with a low impact on performance. Compared
to `enable_sequential_cpu_offload`, this method moves one whole model at a time to the GPU when its `forward`
method is called, and the model remains in GPU until the next model runs. Memory savings are lower than with
`enable_sequential_cpu_offload`, but performance is much better due to the iterative execution of the `unet`.
#### enable_vae_slicing[[diffusers.AudioLDM2Pipeline.enable_vae_slicing]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/pipeline_audioldm2.py#L227)

Enable sliced VAE decoding. When this option is enabled, the VAE will split the input tensor in slices to
compute decoding in several steps. This is useful to save some memory and allow larger batch sizes.
#### encode_prompt[[diffusers.AudioLDM2Pipeline.encode_prompt]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/pipeline_audioldm2.py#L357)

Encodes the prompt into text encoder hidden states.

Example:

```python
>>> import scipy
>>> import torch
>>> from diffusers import AudioLDM2Pipeline

>>> repo_id = "cvssp/audioldm2"
>>> pipe = AudioLDM2Pipeline.from_pretrained(repo_id, torch_dtype=torch.float16)
>>> pipe = pipe.to("cuda")

>>> # Get text embedding vectors
>>> prompt_embeds, attention_mask, generated_prompt_embeds = pipe.encode_prompt(
...     prompt="Techno music with a strong, upbeat tempo and high melodic riffs",
...     device="cuda",
...     do_classifier_free_guidance=True,
... )

>>> # Pass text embeddings to pipeline for text-conditional audio generation
>>> audio = pipe(
...     prompt_embeds=prompt_embeds,
...     attention_mask=attention_mask,
...     generated_prompt_embeds=generated_prompt_embeds,
...     num_inference_steps=200,
...     audio_length_in_s=10.0,
... ).audios[0]

>>> # save generated audio sample
>>> scipy.io.wavfile.write("techno.wav", rate=16000, data=audio)
```

**Parameters:**

prompt (`str` or `list[str]`, *optional*) : prompt to be encoded

transcription (`str` or `list[str]`) : transcription of text to speech

device (`torch.device`) : torch device

num_waveforms_per_prompt (`int`) : number of waveforms that should be generated per prompt

do_classifier_free_guidance (`bool`) : whether to use classifier free guidance or not

negative_prompt (`str` or `list[str]`, *optional*) : The prompt or prompts not to guide the audio generation. If not defined, one has to pass `negative_prompt_embeds` instead. Ignored when not using guidance (i.e., ignored if `guidance_scale` is less than `1`).

prompt_embeds (`torch.Tensor`, *optional*) : Pre-computed text embeddings from the Flan T5 model. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be computed from `prompt` input argument.

negative_prompt_embeds (`torch.Tensor`, *optional*) : Pre-computed negative text embeddings from the Flan T5 model. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be computed from `negative_prompt` input argument.

generated_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated text embeddings from the GPT2 language model. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, text embeddings will be generated from `prompt` input argument.

negative_generated_prompt_embeds (`torch.Tensor`, *optional*) : Pre-generated negative text embeddings from the GPT2 language model. Can be used to easily tweak text inputs, *e.g.* prompt weighting. If not provided, negative_prompt_embeds will be computed from `negative_prompt` input argument.

attention_mask (`torch.LongTensor`, *optional*) : Pre-computed attention mask to be applied to the `prompt_embeds`. If not provided, attention mask will be computed from `prompt` input argument.

negative_attention_mask (`torch.LongTensor`, *optional*) : Pre-computed attention mask to be applied to the `negative_prompt_embeds`. If not provided, attention mask will be computed from `negative_prompt` input argument.

max_new_tokens (`int`, *optional*, defaults to None) : The number of new tokens to generate with the GPT2 language model.

**Returns:**

`prompt_embeds (`torch.Tensor`)`

Text embeddings from the Flan T5 model.
attention_mask (`torch.LongTensor`):
Attention mask to be applied to the `prompt_embeds`.
generated_prompt_embeds (`torch.Tensor`):
Text embeddings generated from the GPT2 language model.
#### generate_language_model[[diffusers.AudioLDM2Pipeline.generate_language_model]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/pipeline_audioldm2.py#L304)

Generates a sequence of hidden-states from the language model, conditioned on the embedding inputs.

**Parameters:**

inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`) : The sequence used as a prompt for the generation.

max_new_tokens (`int`) : Number of new tokens to generate.

model_kwargs (`dict[str, Any]`, *optional*) : Ad hoc parametrization of additional model-specific kwargs that will be forwarded to the `forward` function of the model.

**Returns:**

``inputs_embeds (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`)`

The sequence of generated hidden-states.

## AudioLDM2ProjectionModel[[diffusers.AudioLDM2ProjectionModel]]
#### diffusers.AudioLDM2ProjectionModel[[diffusers.AudioLDM2ProjectionModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/modeling_audioldm2.py#L78)

A simple linear projection model to map two text embeddings to a shared latent space. It also inserts learned
embedding vectors at the start and end of each text embedding sequence respectively. Each variable appended with
`_1` refers to that corresponding to the second text encoder. Otherwise, it is from the first.

forwarddiffusers.AudioLDM2ProjectionModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/modeling_audioldm2.py#L122[{"name": "hidden_states", "val": ": torch.Tensor | None = None"}, {"name": "hidden_states_1", "val": ": torch.Tensor | None = None"}, {"name": "attention_mask", "val": ": torch.LongTensor | None = None"}, {"name": "attention_mask_1", "val": ": torch.LongTensor | None = None"}]

**Parameters:**

text_encoder_dim (`int`) : Dimensionality of the text embeddings from the first text encoder (CLAP).

text_encoder_1_dim (`int`) : Dimensionality of the text embeddings from the second text encoder (T5 or VITS).

langauge_model_dim (`int`) : Dimensionality of the text embeddings from the language model (GPT2).

## AudioLDM2UNet2DConditionModel[[diffusers.AudioLDM2UNet2DConditionModel]]
#### diffusers.AudioLDM2UNet2DConditionModel[[diffusers.AudioLDM2UNet2DConditionModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/modeling_audioldm2.py#L163)

A conditional 2D UNet model that takes a noisy sample, conditional state, and a timestep and returns a sample
shaped output. Compared to the vanilla [UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.UNet2DConditionModel), this variant optionally includes an additional
self-attention layer in each Transformer block, as well as multiple cross-attention layers. It also allows for up
to two cross-attention embeddings, `encoder_hidden_states` and `encoder_hidden_states_1`.

This model inherits from [ModelMixin](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

forwarddiffusers.AudioLDM2UNet2DConditionModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/audioldm2/modeling_audioldm2.py#L612[{"name": "sample", "val": ": Tensor"}, {"name": "timestep", "val": ": torch.Tensor | float | int"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "class_labels", "val": ": torch.Tensor | None = None"}, {"name": "timestep_cond", "val": ": torch.Tensor | None = None"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "cross_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "encoder_hidden_states_1", "val": ": torch.Tensor | None = None"}, {"name": "encoder_attention_mask_1", "val": ": torch.Tensor | None = None"}]- **sample** (`torch.Tensor`) --
  The noisy input tensor with the following shape `(batch, channel, height, width)`.
- **timestep** (`torch.Tensor` or `float` or `int`) -- The number of timesteps to denoise an input.
- **encoder_hidden_states** (`torch.Tensor`) --
  The encoder hidden states with shape `(batch, sequence_length, feature_dim)`.
- **encoder_attention_mask** (`torch.Tensor`) --
  A cross-attention mask of shape `(batch, sequence_length)` is applied to `encoder_hidden_states`. If
  `True` the mask is kept, otherwise if `False` it is discarded. Mask will be converted into a bias,
  which adds large negative values to the attention scores corresponding to "discard" tokens.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [UNet2DConditionOutput](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) instead of a plain
  tuple.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttnProcessor`.
- **encoder_hidden_states_1** (`torch.Tensor`, *optional*) --
  A second set of encoder hidden states with shape `(batch, sequence_length_2, feature_dim_2)`. Can be
  used to condition the model on a different set of embeddings to `encoder_hidden_states`.
- **encoder_attention_mask_1** (`torch.Tensor`, *optional*) --
  A cross-attention mask of shape `(batch, sequence_length_2)` is applied to `encoder_hidden_states_1`.
  If `True` the mask is kept, otherwise if `False` it is discarded. Mask will be converted into a bias,
  which adds large negative values to the attention scores corresponding to "discard" tokens.0[UNet2DConditionOutput](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) or `tuple`If `return_dict` is True, an [UNet2DConditionOutput](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) is returned,
otherwise a `tuple` is returned where the first element is the sample tensor.

The [AudioLDM2UNet2DConditionModel](/docs/diffusers/v0.39.0/en/api/pipelines/audioldm2#diffusers.AudioLDM2UNet2DConditionModel) forward method.

**Parameters:**

sample_size (`int` or `tuple[int, int]`, *optional*, defaults to `None`) : Height and width of input/output sample.

in_channels (`int`, *optional*, defaults to 4) : Number of channels in the input sample.

out_channels (`int`, *optional*, defaults to 4) : Number of channels in the output.

flip_sin_to_cos (`bool`, *optional*, defaults to `False`) : Whether to flip the sin to cos in the time embedding.

freq_shift (`int`, *optional*, defaults to 0) : The frequency shift to apply to the time embedding.

down_block_types (`tuple[str]`, *optional*, defaults to `("CrossAttnDownBlock2D", "CrossAttnDownBlock2D", "CrossAttnDownBlock2D", "DownBlock2D")`) : The tuple of downsample blocks to use.

mid_block_type (`str`, *optional*, defaults to `"UNetMidBlock2DCrossAttn"`) : Block type for middle of UNet, it can only be `UNetMidBlock2DCrossAttn` for AudioLDM2.

up_block_types (`tuple[str]`, *optional*, defaults to `("UpBlock2D", "CrossAttnUpBlock2D", "CrossAttnUpBlock2D", "CrossAttnUpBlock2D")`) : The tuple of upsample blocks to use.

only_cross_attention (`bool` or `tuple[bool]`, *optional*, default to `False`) : Whether to include self-attention in the basic transformer blocks, see `BasicTransformerBlock`.

block_out_channels (`tuple[int]`, *optional*, defaults to `(320, 640, 1280, 1280)`) : The tuple of output channels for each block.

layers_per_block (`int`, *optional*, defaults to 2) : The number of layers per block.

downsample_padding (`int`, *optional*, defaults to 1) : The padding to use for the downsampling convolution.

mid_block_scale_factor (`float`, *optional*, defaults to 1.0) : The scale factor to use for the mid block.

act_fn (`str`, *optional*, defaults to `"silu"`) : The activation function to use.

norm_num_groups (`int`, *optional*, defaults to 32) : The number of groups to use for the normalization. If `None`, normalization and activation layers is skipped in post-processing.

norm_eps (`float`, *optional*, defaults to 1e-5) : The epsilon to use for the normalization.

cross_attention_dim (`int` or `tuple[int]`, *optional*, defaults to 1280) : The dimension of the cross attention features.

transformer_layers_per_block (`int` or `tuple[int]`, *optional*, defaults to 1) : The number of transformer blocks of type `BasicTransformerBlock`. Only relevant for `~models.unet_2d_blocks.CrossAttnDownBlock2D`, `~models.unet_2d_blocks.CrossAttnUpBlock2D`, `~models.unet_2d_blocks.UNetMidBlock2DCrossAttn`.

attention_head_dim (`int`, *optional*, defaults to 8) : The dimension of the attention heads.

num_attention_heads (`int`, *optional*) : The number of attention heads. If not defined, defaults to `attention_head_dim`

resnet_time_scale_shift (`str`, *optional*, defaults to `"default"`) : Time scale shift config for ResNet blocks (see `ResnetBlock2D`). Choose from `default` or `scale_shift`.

class_embed_type (`str`, *optional*, defaults to `None`) : The type of class embedding to use which is ultimately summed with the time embeddings. Choose from `None`, `"timestep"`, `"identity"`, `"projection"`, or `"simple_projection"`.

num_class_embeds (`int`, *optional*, defaults to `None`) : Input dimension of the learnable embedding matrix to be projected to `time_embed_dim`, when performing class conditioning with `class_embed_type` equal to `None`.

time_embedding_type (`str`, *optional*, defaults to `positional`) : The type of position embedding to use for timesteps. Choose from `positional` or `fourier`.

time_embedding_dim (`int`, *optional*, defaults to `None`) : An optional override for the dimension of the projected time embedding.

time_embedding_act_fn (`str`, *optional*, defaults to `None`) : Optional activation function to use only once on the time embeddings before they are passed to the rest of the UNet. Choose from `silu`, `mish`, `gelu`, and `swish`.

timestep_post_act (`str`, *optional*, defaults to `None`) : The second activation function to use in timestep embedding. Choose from `silu`, `mish` and `gelu`.

time_cond_proj_dim (`int`, *optional*, defaults to `None`) : The dimension of `cond_proj` layer in the timestep embedding.

conv_in_kernel (`int`, *optional*, default to `3`) : The kernel size of `conv_in` layer.

conv_out_kernel (`int`, *optional*, default to `3`) : The kernel size of `conv_out` layer.

projection_class_embeddings_input_dim (`int`, *optional*) : The dimension of the `class_labels` input when `class_embed_type="projection"`. Required when `class_embed_type="projection"`.

class_embeddings_concat (`bool`, *optional*, defaults to `False`) : Whether to concatenate the time embeddings with the class embeddings.

**Returns:**

`[UNet2DConditionOutput](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) or `tuple``

If `return_dict` is True, an [UNet2DConditionOutput](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) is returned,
otherwise a `tuple` is returned where the first element is the sample tensor.

## AudioPipelineOutput[[diffusers.AudioPipelineOutput]]
#### diffusers.AudioPipelineOutput[[diffusers.AudioPipelineOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/pipeline_utils.py#L136)

Output class for audio pipelines.

**Parameters:**

audios (`np.ndarray`) : List of denoised audio samples of a NumPy array of shape `(batch_size, num_channels, sample_rate)`.
