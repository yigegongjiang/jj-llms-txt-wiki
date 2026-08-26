# MiniMaxH3Transformer3DModel

A Diffusion Transformer model for joint video and audio generation, introduced in [MiniMax-H3](https://huggingface.co/MiniMaxAI/MiniMax-H3) by MiniMax.

MiniMax-H3 runs a single stack of blocks over **one packed 1-D sequence** that holds the text conditioning, the conditioning image and video rows, the audio rows and the target video rows at once. Attention is full self-attention over that sequence, so there is no cross-attention and no per-modality block weights. Modality-specific behaviour comes only from the two input patch projections, the per-row modality tag that selects the AdaLN modulation parameters, and the two output heads.

Building the packed layout is the caller's job, which is why the forward signature takes the layout apart from the latents: the `(t, h, w)` position grid, the per-row modality tags, the per-row timestep indices and the three index tensors that address the video, audio and text rows. [MiniMaxH3Blocks](/docs/diffusers/v0.40.0/en/api/pipelines/minimax_h3#diffusers.MiniMaxH3Blocks) and `MiniMaxH3Ref2VABlocks` build all of it.

A layout that carries padding rows (tag `-1`) needs a masked attention backend, since those rows are kept in their own attention document by a boolean mask; a padless sequence needs no mask and keeps every backend available.

One repository holds both released checkpoint partitions, so the subfolder is what selects the task: `transformer/` for the text and keyframe tasks, `transformer_ref/` for the omni-reference task.

```python
import torch
from diffusers import MiniMaxH3Transformer3DModel

transformer = MiniMaxH3Transformer3DModel.from_pretrained(
    "MiniMaxAI/MiniMax-H3", subfolder="transformer", dtype=torch.bfloat16
).to("cuda")
```

The checkpoint is mixed precision: the two input patch projections, the timestep MLP and the two output heads are float32 while the block stack is bfloat16. `from_pretrained` keeps that layout through `_keep_in_fp32_modules`, so pass `dtype=torch.bfloat16` and let it place the float32 modules rather than casting the model with `.to(torch.bfloat16)` afterwards.

## MiniMaxH3Transformer3DModel[[diffusers.MiniMaxH3Transformer3DModel]]

#### diffusers.MiniMaxH3Transformer3DModel[[diffusers.MiniMaxH3Transformer3DModel]]

```python
diffusers.MiniMaxH3Transformer3DModel(num_attention_heads: int = 56, attention_head_dim: int = 128, hidden_size: int = 5376, num_layers: int = 50, num_refiner_layers: int = 2, ffn_dim: int = 14336, in_channels: int = 24, audio_in_channels: int = 32, patch_size: tuple = (1, 2, 2), text_dim: int = 5120, freq_dim: int = 256, time_embed_hidden_dim: int = 5376, time_embed_dim: int = 2688, rope_freq_dim: int = 16, rope_theta: float = 10000.0, norm_eps: float = 1e-05, qk_norm_eps: float = 1e-05, final_norm_eps: float = 1e-05)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_minimax_h3.py#L376)

**Parameters:**

num_attention_heads (`int`, defaults to `56`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `128`) : The number of channels in each attention head. Note that `num_attention_heads * attention_head_dim` is *larger* than `hidden_size` in MiniMax-H3.

hidden_size (`int`, defaults to `5376`) : The number of channels of the packed sequence (the residual stream).

num_layers (`int`, defaults to `50`) : The number of transformer blocks.

num_refiner_layers (`int`, defaults to `2`) : The number of token refiner blocks applied to the projected text stream.

ffn_dim (`int`, defaults to `14336`) : The inner dimension of the SwiGLU feed-forward layers.

in_channels (`int`, defaults to `24`) : The number of channels of the video latents.

audio_in_channels (`int`, defaults to `32`) : The number of channels of the audio latents.

patch_size (`tuple[int, int, int]`, defaults to `(1, 2, 2)`) : The `(t, h, w)` patch used to pack the video latents into rows.

text_dim (`int`, defaults to `5120`) : The number of channels of the text conditioning produced by the text encoder.

freq_dim (`int`, defaults to `256`) : The dimension of the sinusoidal timestep embedding. Timesteps are consumed unscaled in `[0, 1]`.

time_embed_hidden_dim (`int`, defaults to `5376`) : The inner dimension of the timestep MLP.

time_embed_dim (`int`, defaults to `2688`) : The output dimension of the timestep MLP, i.e. the input of every AdaLN projection.

rope_freq_dim (`int`, defaults to `16`) : The number of rotary frequencies per axis. The `(t, h, w)` axes share one `inv_freq` buffer of this length and `2 * 3 * rope_freq_dim` of the `attention_head_dim` channels are rotated.

rope_theta (`float`, defaults to `10000.0`) : The base of the rotary frequency schedule the `rope.inv_freq` buffer is computed from.

norm_eps (`float`, defaults to `1e-5`) : Epsilon of the pre-attention and pre-feed-forward norms.

qk_norm_eps (`float`, defaults to `1e-5`) : Epsilon of the per-head query/key norms.

final_norm_eps (`float`, defaults to `1e-5`) : Epsilon of the token refiner output norm and of `norm_out`.

A Transformer model for joint video + audio generation, introduced in MiniMax-H3.

MiniMax-H3 runs a single stack of blocks over **one packed 1-D sequence** that holds the text condition, the
conditioning image / video rows, the audio rows and the target video rows. Attention is full self-attention over
that sequence; there is no cross-attention and no per-modality block weights. Modality-specific behaviour comes
only from the two input patch projections, the per-row AdaLN modality tag, and the two output heads.

The caller is responsible for building the packed layout: patchifying the video latents, ordering the rows, and
producing the `(t, h, w)` position grid, the per-row modality tags and the per-row timestep indices. The sequence
carries no padding — the reference implementation pads it to a multiple of 64 for FlashAttention and splits the
tail off with `cu_seqlens = [0, used, S]`, which this port has no use for — so attention runs unmasked over one
document and every attention backend stays available.

The batch axis is a pure replication axis: the structural arguments (`timestep`, `timestep_indices`, `token_tags`,
`position_ids` and the three index tensors) describe one packed layout that every batch item shares, and each item
is a single attention document.

#### forward[[diffusers.MiniMaxH3Transformer3DModel.forward]]

```python
forward(hidden_states: Tensor, audio_hidden_states: Tensor, encoder_hidden_states: Tensor, timestep: Tensor, timestep_indices: Tensor, token_tags: Tensor, position_ids: Tensor, video_indices: Tensor, audio_indices: Tensor, text_indices: Tensor, attention_kwargs: dict[str, typing.Any] | None = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_minimax_h3.py#L561)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, num_video_tokens, in_channels * prod(patch_size))`) : Patchified video latent rows — conditioning rows and target rows — ordered as they appear in the packed sequence, i.e. matching `video_indices`.

audio_hidden_states (`torch.Tensor` of shape `(batch_size, num_audio_tokens, audio_in_channels)`) : Audio latent rows, ordered to match `audio_indices`.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, num_text_tokens, text_dim)`) : Text conditioning, ordered to match `text_indices`.

timestep (`torch.Tensor` of shape `(num_timesteps,)`) : The *distinct* timestep values present in the packed sequence, in `[0, 1]` and unscaled. One forward serves rows at different noise levels (target video, target audio, conditioning rows).

timestep_indices (`torch.Tensor` of shape `(seq_len,)`) : For every row of the packed sequence, the index of its timestep in `timestep`.

token_tags (`torch.Tensor` of shape `(seq_len,)`) : For every row of the packed sequence, its modality: `0` video, `1` text, `2` audio.

position_ids (`torch.Tensor` of shape `(seq_len, 3)`) : The `(t, h, w)` rotary coordinates of every row of the packed sequence.

video_indices (`torch.Tensor` of shape `(num_video_tokens,)`) : Positions of the video rows in the packed sequence.

audio_indices (`torch.Tensor` of shape `(num_audio_tokens,)`) : Positions of the audio rows in the packed sequence.

text_indices (`torch.Tensor` of shape `(num_text_tokens,)`) : Positions of the text rows in the packed sequence.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that, if specified, may carry a `scale` entry which is applied to the LoRA layers.

return_dict (`bool`, defaults to `True`) : Whether to return a `MiniMaxH3TransformerOutput` instead of a plain tuple.

**Returns:** `MiniMaxH3TransformerOutput` or `tuple`

The video velocity of shape `(batch_size, num_video_tokens, in_channels * prod(patch_size))` and the
audio velocity of shape `(batch_size, num_audio_tokens, audio_in_channels)`, in the row order of
`video_indices` and `audio_indices`.

## MiniMaxH3TransformerOutput[[diffusers.models.transformers.transformer_minimax_h3.MiniMaxH3TransformerOutput]]

#### diffusers.models.transformers.transformer_minimax_h3.MiniMaxH3TransformerOutput[[diffusers.models.transformers.transformer_minimax_h3.MiniMaxH3TransformerOutput]]

```python
diffusers.models.transformers.transformer_minimax_h3.MiniMaxH3TransformerOutput(sample: Tensor, audio_sample: typing.Optional[torch.Tensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_minimax_h3.py#L41)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_video_tokens, in_channels * prod(patch_size))`) : The video velocity prediction for the rows addressed by `video_indices`, in the same order. Conditioning rows are returned unmasked — masking them out before the scheduler step is the caller's job.

audio_sample (`torch.Tensor` of shape `(batch_size, num_audio_tokens, audio_in_channels)`, defaults to `None`) : The audio velocity prediction for the rows addressed by `audio_indices`, in the same order. `forward` always populates it; it only defaults to `None` so that the output can be rebuilt from a plain dict of its fields, which is how the accelerate offload hooks move a `BaseOutput` back to the input device.

The output of [MiniMaxH3Transformer3DModel](/docs/diffusers/v0.40.0/en/api/models/minimax_h3_transformer3d#diffusers.MiniMaxH3Transformer3DModel).
