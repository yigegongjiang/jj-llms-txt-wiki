# ChronoEditTransformer3DModel

A Diffusion Transformer model for 3D video-like data from [ChronoEdit: Towards Temporal Reasoning for Image Editing and World Simulation](https://huggingface.co/papers/2510.04290) from NVIDIA and University of Toronto, by Jay Zhangjie Wu, Xuanchi Ren, Tianchang Shen, Tianshi Cao, Kai He, Yifan Lu, Ruiyuan Gao, Enze Xie, Shiyi Lan, Jose M. Alvarez, Jun Gao, Sanja Fidler, Zian Wang, Huan Ling.

> **TL;DR:** ChronoEdit reframes image editing as a video generation task, using input and edited images as start/end frames to leverage pretrained video models with temporal consistency. A temporal reasoning stage introduces reasoning tokens to ensure physically plausible edits and visualize the editing trajectory.

The model can be loaded with the following code snippet.

```python
from diffusers import ChronoEditTransformer3DModel

transformer = ChronoEditTransformer3DModel.from_pretrained("nvidia/ChronoEdit-14B-Diffusers", subfolder="transformer", torch_dtype=torch.bfloat16)
```

## ChronoEditTransformer3DModel[[diffusers.ChronoEditTransformer3DModel]]

#### diffusers.ChronoEditTransformer3DModel[[diffusers.ChronoEditTransformer3DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_chronoedit.py#L523)

A Transformer model for video-like data used in the ChronoEdit model.

forwarddiffusers.ChronoEditTransformer3DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_chronoedit.py#L644[{"name": "hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": LongTensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states_image", "val": ": torch.Tensor | None = None"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)`) --
  Input `hidden_states`.
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **encoder_hidden_states_image** (`torch.Tensor`, *optional*) --
  Conditional image embeddings for image-conditioned generation.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [ChronoEditTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/chronoedit_transformer_3d#diffusers.ChronoEditTransformer3DModel) forward method.

**Parameters:**

patch_size (`tuple[int]`, defaults to `(1, 2, 2)`) : 3D patch dimensions for video embedding (t_patch, h_patch, w_patch).

num_attention_heads (`int`, defaults to `40`) : Fixed length for text embeddings.

attention_head_dim (`int`, defaults to `128`) : The number of channels in each head.

in_channels (`int`, defaults to `16`) : The number of channels in the input.

out_channels (`int`, defaults to `16`) : The number of channels in the output.

text_dim (`int`, defaults to `512`) : Input dimension for text embeddings.

freq_dim (`int`, defaults to `256`) : Dimension for sinusoidal time embeddings.

ffn_dim (`int`, defaults to `13824`) : Intermediate dimension in feed-forward network.

num_layers (`int`, defaults to `40`) : The number of layers of transformer blocks to use.

window_size (`tuple[int]`, defaults to `(-1, -1)`) : Window size for local attention (-1 indicates global attention).

cross_attn_norm (`bool`, defaults to `True`) : Enable cross-attention normalization.

qk_norm (`bool`, defaults to `True`) : Enable query/key normalization.

eps (`float`, defaults to `1e-6`) : Epsilon value for normalization layers.

add_img_emb (`bool`, defaults to `False`) : Whether to use img_emb.

added_kv_proj_dim (`int`, *optional*, defaults to `None`) : The number of channels to use for the added key and value projections. If `None`, no projection is used.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
