# LTX2VideoTransformer3DModel

A Diffusion Transformer model for 3D data from [LTX](https://huggingface.co/Lightricks/LTX-2) was introduced by Lightricks.

The model can be loaded with the following code snippet.

```python
from diffusers import LTX2VideoTransformer3DModel

transformer = LTX2VideoTransformer3DModel.from_pretrained("Lightricks/LTX-2", subfolder="transformer", torch_dtype=torch.bfloat16).to("cuda")
```

## LTX2VideoTransformer3DModel[[diffusers.LTX2VideoTransformer3DModel]]

#### diffusers.LTX2VideoTransformer3DModel[[diffusers.LTX2VideoTransformer3DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_ltx2.py#L1062)

A Transformer model for video-like data used in [LTX](https://huggingface.co/Lightricks/LTX-Video).

forwarddiffusers.LTX2VideoTransformer3DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_ltx2.py#L1321[{"name": "hidden_states", "val": ": Tensor"}, {"name": "audio_hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "audio_encoder_hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": LongTensor"}, {"name": "audio_timestep", "val": ": torch.LongTensor | None = None"}, {"name": "sigma", "val": ": torch.Tensor | None = None"}, {"name": "audio_sigma", "val": ": torch.Tensor | None = None"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "audio_encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "num_frames", "val": ": int | None = None"}, {"name": "height", "val": ": int | None = None"}, {"name": "width", "val": ": int | None = None"}, {"name": "fps", "val": ": float = 24.0"}, {"name": "audio_num_frames", "val": ": int | None = None"}, {"name": "video_coords", "val": ": torch.Tensor | None = None"}, {"name": "audio_coords", "val": ": torch.Tensor | None = None"}, {"name": "isolate_modalities", "val": ": bool = False"}, {"name": "spatio_temporal_guidance_blocks", "val": ": list[int] | None = None"}, {"name": "perturbation_mask", "val": ": torch.Tensor | None = None"}, {"name": "use_cross_timestep", "val": ": bool = False"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "video_self_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor`) --
  Input patchified video latents of shape `(batch_size, num_video_tokens, in_channels)`.
- **audio_hidden_states** (`torch.Tensor`) --
  Input patchified audio latents of shape `(batch_size, num_audio_tokens, audio_in_channels)`.
- **encoder_hidden_states** (`torch.Tensor`) --
  Input video text embeddings of shape `(batch_size, text_seq_len, self.config.caption_channels)`.
- **audio_encoder_hidden_states** (`torch.Tensor`) --
  Input audio text embeddings of shape `(batch_size, text_seq_len, self.config.caption_channels)`.
- **timestep** (`torch.Tensor`) --
  Input timestep of shape `(batch_size, num_video_tokens)`. These should already be scaled by
  `self.config.timestep_scale_multiplier`.
- **audio_timestep** (`torch.Tensor`, *optional*) --
  Input timestep of shape `(batch_size,)` or `(batch_size, num_audio_tokens)` for audio modulation
  params. This is only used by certain pipelines such as the I2V pipeline.
- **sigma** (`torch.Tensor`, *optional*) --
  Input scaled timestep of shape (batch_size,). Used for video prompt cross attention modulation in
  models such as LTX-2.3.
- **audio_sigma** (`torch.Tensor`, *optional*) --
  Input scaled timestep of shape (batch_size,). Used for audio prompt cross attention modulation in
  models such as LTX-2.3. If `sigma` is supplied but `audio_sigma` is not, `audio_sigma` will be set to
  the provided `sigma` value.
- **encoder_attention_mask** (`torch.Tensor`, *optional*) --
  Optional multiplicative text attention mask of shape `(batch_size, text_seq_len)`.
- **audio_encoder_attention_mask** (`torch.Tensor`, *optional*) --
  Optional multiplicative text attention mask of shape `(batch_size, text_seq_len)` for audio modeling.
- **num_frames** (`int`, *optional*) --
  The number of latent video frames. Used if calculating the video coordinates for RoPE.
- **height** (`int`, *optional*) --
  The latent video height. Used if calculating the video coordinates for RoPE.
- **width** (`int`, *optional*) --
  The latent video width. Used if calculating the video coordinates for RoPE.
- **fps** -- (`float`, *optional*, defaults to `24.0`):
  The desired frames per second of the generated video. Used if calculating the video coordinates for
  RoPE.
- **audio_num_frames** -- (`int`, *optional*):
  The number of latent audio frames. Used if calculating the audio coordinates for RoPE.
- **video_coords** (`torch.Tensor`, *optional*) --
  The video coordinates to be used when calculating the rotary positional embeddings (RoPE) of shape
  `(batch_size, 3, num_video_tokens, 2)`. If not supplied, this will be calculated inside `forward`.
- **audio_coords** (`torch.Tensor`, *optional*) --
  The audio coordinates to be used when calculating the rotary positional embeddings (RoPE) of shape
  `(batch_size, 1, num_audio_tokens, 2)`. If not supplied, this will be calculated inside `forward`.
- **isolate_modalities** (`bool`, *optional*, defaults to `False`) --
  Whether to isolate each modality by turning off cross-modality (audio-to-video and video-to-audio)
  cross attention (for all blocks). Use for modality guidance in LTX-2.3.
- **spatio_temporal_guidance_blocks** (`list[int]`, *optional*, defaults to `None`) --
  The transformer block indices at which to apply spatio-temporal guidance (STG), which shortcuts the
  self-attention operations by simply using the values rather than the full scaled dot-product attention
  (SDPA) operation. If `None` or empty, STG will not be applied to any block.
- **perturbation_mask** (`torch.Tensor`, *optional*) --
  Perturbation mask for STG of shape `(batch_size,)` or `(batch_size, 1, 1)`. Should be 0 at batch
  elements where STG should be applied and 1 elsewhere. If STG is being used but `peturbation_mask` is
  not supplied, will default to applying STG (perturbing) all batch elements.
- **use_cross_timestep** (`bool` *optional*, defaults to `False`) --
  Whether to use the cross modality (audio is the cross modality of video, and vice versa) sigma when
  calculating the cross attention modulation parameters. `True` is the newer (e.g. LTX-2.3) behavior;
  `False` is the legacy LTX-2.0 behavior.
- **attention_kwargs** (`dict[str, Any]`, *optional*) --
  Optional dict of keyword args to be passed to the attention processor.
- **video_self_attention_mask** (`torch.Tensor`, *optional*) --
  Optional multiplicative self-attention mask of shape `(batch_size, num_video_tokens, num_video_tokens)`
  applied to the video self-attention in each transformer block. Values in `[0, 1]` where `1` means full
  attention and `0` means masked. Used e.g. by the IC-LoRA pipeline to control attention strength between
  noisy tokens and appended reference tokens. Audio self-attention is not affected.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether to return a dict-like structured output of type `AudioVisualModelOutput` or a tuple.0`AudioVisualModelOutput` or `tuple`If `return_dict` is `True`, returns a structured output of type `AudioVisualModelOutput`, otherwise a
`tuple` is returned where the first element is the denoised video latent patch sequence and the second
element is the denoised audio latent patch sequence.

Forward pass for LTX-2.0 audiovisual video transformer.

**Parameters:**

in_channels (`int`, defaults to `128`) : The number of channels in the input.

out_channels (`int`, defaults to `128`) : The number of channels in the output.

patch_size (`int`, defaults to `1`) : The size of the spatial patches to use in the patch embedding layer.

patch_size_t (`int`, defaults to `1`) : The size of the tmeporal patches to use in the patch embedding layer.

num_attention_heads (`int`, defaults to `32`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `64`) : The number of channels in each head.

cross_attention_dim (`int`, defaults to `2048 `) : The number of channels for cross attention heads.

num_layers (`int`, defaults to `28`) : The number of layers of Transformer blocks to use.

activation_fn (`str`, defaults to `"gelu-approximate"`) : Activation function to use in feed-forward.

qk_norm (`str`, defaults to `"rms_norm_across_heads"`) : The normalization layer to use.

**Returns:**

``AudioVisualModelOutput` or `tuple``

If `return_dict` is `True`, returns a structured output of type `AudioVisualModelOutput`, otherwise a
`tuple` is returned where the first element is the denoised video latent patch sequence and the second
element is the denoised audio latent patch sequence.
