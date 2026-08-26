# WanAnimate2Transformer3DModel

A Diffusion Transformer model for 3D video-like data used in [Wan-Animate-2](https://github.com/Wan-Video/Wan2.2) by the Alibaba Wan Team. It animates a character image with the motion of a driving video through an in-context reference mechanism: each segment first runs a reference pass (`kv_cache_mode="extract"`) that caches every layer's reference K/V, then the denoising passes (`kv_cache_mode="cached"`) attend jointly over the generation tokens and the cached reference tokens through a flex `BlockMask`.

The model can be loaded with the following code snippet.

```python
from diffusers import WanAnimate2Transformer3DModel

transformer = WanAnimate2Transformer3DModel.from_pretrained("Wan-AI/Wan2.2-Animate-2-14B-Diffusers", subfolder="transformer", dtype=torch.bfloat16)
```

## WanAnimate2Transformer3DModel[[diffusers.WanAnimate2Transformer3DModel]]

#### diffusers.WanAnimate2Transformer3DModel[[diffusers.WanAnimate2Transformer3DModel]]

```python
diffusers.WanAnimate2Transformer3DModel(patch_size: tuple = (1, 2, 2), text_len: int = 512, in_dim: int = 36, dim: int = 5120, ffn_dim: int = 13824, freq_dim: int = 256, text_dim: int = 4096, out_dim: int = 16, num_heads: int = 40, num_layers: int = 40, cross_attn_norm: bool = True, eps: float = 1e-06, use_img_emb: bool = True, refer_offset_t: int = 1, refer_offset_h: int = 0, refer_offset_w: int = -1, refer_stride: int = 1)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_wan_animate_2.py#L570)

**Parameters:**

patch_size (*tuple[int]*, defaults to *(1, 2, 2)*) : 3D patch dimensions for video embedding (t_patch, h_patch, w_patch).

text_len (*int*, defaults to *512*) : Fixed length for text embeddings.

in_dim (*int*, defaults to *36*) : The number of channels in the input (2 * latent_channels + 4 for mask channel).

dim (*int*, defaults to *5120*) : The number of channels in the transformer.

ffn_dim (*int*, defaults to *13824*) : Intermediate dimension in feed-forward network.

freq_dim (*int*, defaults to *256*) : Dimension for sinusoidal time embeddings.

text_dim (*int*, defaults to *4096*) : Input dimension for text embeddings.

out_dim (*int*, defaults to *16*) : The number of channels in the output.

num_heads (*int*, defaults to *40*) : The number of attention heads.

num_layers (*int*, defaults to *40*) : The number of layers of transformer blocks to use.

cross_attn_norm (*bool*, defaults to *True*) : Enable cross-attention normalization.

eps (*float*, defaults to *1e-6*) : Epsilon value for normalization layers.

use_img_emb (*bool*, defaults to *True*) : Whether to use CLIP image embedding.

refer_offset_t (*int*, defaults to *1*) : RoPE offset for the temporal dimension of the reference.

refer_offset_h (*int*, defaults to *0*) : RoPE offset for the height dimension of the reference.

refer_offset_w (*int*, defaults to *-1*) : RoPE offset for the width dimension of the reference. -1 means use the generation grid size.

refer_stride (*int*, defaults to *1*) : Stride for RoPE application on the reference.

A Transformer model for video-like data used in the Wan-Animate-2 model.

Wan-Animate-2 uses an in-context attention mechanism with a KV cache: a reference video is first encoded
(`kv_cache_mode="extract"`) to populate a [*WanAnimate2KVCache*], then each denoising step
(`kv_cache_mode="cached"`) attends jointly over the generation tokens and the cached reference K/V through a flex
`BlockMask`. The generation self-attention therefore runs on the `flex` attention backend only; every other
attention in the model works on any backend.

#### forward[[diffusers.WanAnimate2Transformer3DModel.forward]]

```python
forward(hidden_states: list, timestep: Tensor, encoder_hidden_states: list, condition_latents: list, kv_cache: WanAnimate2KVCache, kv_cache_mode: str, seq_len: int, encoder_hidden_states_image: typing.Optional[torch.Tensor] = None, offset_grid_sizes: typing.Optional[torch.Tensor] = None, reference_grid_sizes: typing.Optional[torch.Tensor] = None, origin_len: int | None = None, origin_area: list[int] | None = None, is_uncondtion: bool = False, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_wan_animate_2.py#L766)

**Parameters:**

hidden_states (`list[torch.Tensor]`) : Latents for this pass — the reference latents when `kv_cache_mode="extract"`, the noisy generation latents when `kv_cache_mode="cached"`.

timestep (`torch.Tensor`) : Denoising timestep. Ignored under `kv_cache_mode="extract"`, which uses a fixed timestep of 1.

encoder_hidden_states (`list[torch.Tensor]`) : Text embeddings for this pass.

condition_latents (`list[torch.Tensor]`) : Conditioning latents concatenated to `hidden_states` before patch embedding.

kv_cache (`WanAnimate2KVCache`) : Written under `kv_cache_mode="extract"`, read under `"cached"`.

kv_cache_mode (`str`) : `"extract"` runs the reference pass and populates `kv_cache`; `"cached"` runs a denoising step against the cached reference tokens.

seq_len (`int`) : Token count each sample must hold after patch embedding.

encoder_hidden_states_image (`torch.Tensor`, *optional*) : CLIP image embeddings, used when the model is configured with `use_img_emb`.

offset_grid_sizes (`torch.Tensor`, *optional*) : Patch grid of the reference latents, used to resolve any `refer_offset_*` set to -1. Required under `kv_cache_mode="extract"`; under `"cached"`, `reference_grid_sizes` describes the same grid and is used instead.

reference_grid_sizes (`torch.Tensor`, *optional*) : Patch grid of the reference latents, used for the reference rotary embeddings. Required under `kv_cache_mode="cached"`.

origin_len (`int`, *optional*) : Frame count of the full video, which the in-context block mask is built over. Required under `kv_cache_mode="cached"`.

origin_area (`list[int]`, *optional*) : Spatial size `[height, width]` of the full video, which the in-context block mask is built over. Required under `kv_cache_mode="cached"`.

is_uncondtion (`bool`, *optional*) : Whether this is the unconditional branch of classifier-free guidance.

return_dict (`bool`, *optional*, defaults to `True`) : Whether to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:** [Transformer2DModelOutput](/docs/diffusers/v0.40.0/en/api/models/hunyuan_video15_transformer_3d#diffusers.models.modeling_outputs.Transformer2DModelOutput) or `tuple(list[torch.Tensor])`

The predicted sample per input latent, unpatchified; a plain tuple if `return_dict` is `False`.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
