# AutoencoderOobleck

The Oobleck variational autoencoder (VAE) model with KL loss was introduced in [Stability-AI/stable-audio-tools](https://github.com/Stability-AI/stable-audio-tools) and [Stable Audio Open](https://huggingface.co/papers/2407.14358) by Stability AI. The model is used in 🤗 Diffusers to encode audio waveforms into latents and to decode latent representations into audio waveforms.

The abstract from the paper is:

*Open generative models are vitally important for the community, allowing for fine-tunes and serving as baselines when presenting new models. However, most current text-to-audio models are private and not accessible for artists and researchers to build upon. Here we describe the architecture and training process of a new open-weights text-to-audio model trained with Creative Commons data. Our evaluation shows that the model's performance is competitive with the state-of-the-art across various metrics. Notably, the reported FDopenl3 results (measuring the realism of the generations) showcase its potential for high-quality stereo sound synthesis at 44.1kHz.*

## AutoencoderOobleck[[diffusers.AutoencoderOobleck]]

#### diffusers.AutoencoderOobleck[[diffusers.AutoencoderOobleck]]

```python
diffusers.AutoencoderOobleck(encoder_hidden_size = 128, downsampling_ratios = [2, 4, 4, 8, 8], channel_multiples = [1, 2, 4, 8, 16], decoder_channels = 128, decoder_input_channels = 64, audio_channels = 2, sampling_rate = 44100)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_oobleck.py#L294)

**Parameters:**

encoder_hidden_size (`int`, *optional*, defaults to 128) : Intermediate representation dimension for the encoder.

downsampling_ratios (`list[int]`, *optional*, defaults to `[2, 4, 4, 8, 8]`) : Ratios for downsampling in the encoder. These are used in reverse order for upsampling in the decoder.

channel_multiples (`list[int]`, *optional*, defaults to `[1, 2, 4, 8, 16]`) : Multiples used to determine the hidden sizes of the hidden layers.

decoder_channels (`int`, *optional*, defaults to 128) : Intermediate representation dimension for the decoder.

decoder_input_channels (`int`, *optional*, defaults to 64) : Input dimension for the decoder. Corresponds to the latent dimension.

audio_channels (`int`, *optional*, defaults to 2) : Number of channels in the audio data. Either 1 for mono or 2 for stereo.

sampling_rate (`int`, *optional*, defaults to 44100) : The sampling rate at which the audio waveform should be digitalized expressed in hertz (Hz).

An autoencoder for encoding waveforms into latents and decoding latent representations into waveforms. First
introduced in Stable Audio.

This model inherits from [ModelMixin](/docs/diffusers/v0.40.0/en/api/models/overview#diffusers.ModelMixin). Check the superclass documentation for it's generic methods implemented
for all models (such as downloading or saving).

#### decode[[diffusers.AutoencoderOobleck.decode]]

```python
decode(z: FloatTensor, return_dict: bool = True, generator = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_oobleck.py#L488)

**Parameters:**

z (`torch.Tensor`) : Input batch of latent vectors.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.vae.OobleckDecoderOutput` instead of a plain tuple.

**Returns:** `~models.vae.OobleckDecoderOutput` or `tuple`

If return_dict is True, a `~models.vae.OobleckDecoderOutput` is returned, otherwise a plain `tuple`
is returned.

Decode a batch of images.

#### encode[[diffusers.AutoencoderOobleck.encode]]

```python
encode(x: Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_oobleck.py#L377)

**Parameters:**

x (`torch.Tensor`) : Input batch of images.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.autoencoder_kl.AutoencoderKLOutput` instead of a plain tuple.

**Returns:**

The latent representations of the encoded images. If `return_dict` is True, a
`~models.autoencoder_kl.AutoencoderKLOutput` is returned, otherwise a plain `tuple` is returned.

Encode a batch of images into latents.

#### forward[[diffusers.AutoencoderOobleck.forward]]

```python
forward(sample: Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: typing.Optional[torch.Generator] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_oobleck.py#L517)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `OobleckDecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `~models.vae.OobleckDecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.OobleckDecoderOutput` is returned, otherwise a plain `tuple`
is returned.

## OobleckDecoderOutput[[diffusers.models.autoencoders.autoencoder_oobleck.OobleckDecoderOutput]]

#### diffusers.models.autoencoders.autoencoder_oobleck.OobleckDecoderOutput[[diffusers.models.autoencoders.autoencoder_oobleck.OobleckDecoderOutput]]

```python
diffusers.models.autoencoders.autoencoder_oobleck.OobleckDecoderOutput(sample: Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_oobleck.py#L202)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, audio_channels, sequence_length)`) : The decoded output sample from the last layer of the model.

Output of decoding method.

## AutoencoderOobleckOutput[[diffusers.models.autoencoders.autoencoder_oobleck.AutoencoderOobleckOutput]]

#### diffusers.models.autoencoders.autoencoder_oobleck.AutoencoderOobleckOutput[[diffusers.models.autoencoders.autoencoder_oobleck.AutoencoderOobleckOutput]]

```python
diffusers.models.autoencoders.autoencoder_oobleck.AutoencoderOobleckOutput(latent_dist: OobleckDiagonalGaussianDistribution)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_oobleck.py#L187)

**Parameters:**

latent_dist (`OobleckDiagonalGaussianDistribution`) : Encoded outputs of `Encoder` represented as the mean and standard deviation of `OobleckDiagonalGaussianDistribution`. `OobleckDiagonalGaussianDistribution` allows for sampling latents from the distribution.

Output of AutoencoderOobleck encoding method.
