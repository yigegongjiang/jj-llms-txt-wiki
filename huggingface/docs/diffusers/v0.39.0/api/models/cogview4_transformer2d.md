# CogView4Transformer2DModel

A Diffusion Transformer model for 2D data from [CogView4]()

The model can be loaded with the following code snippet.

```python
from diffusers import CogView4Transformer2DModel

transformer = CogView4Transformer2DModel.from_pretrained("THUDM/CogView4-6B", subfolder="transformer", torch_dtype=torch.bfloat16).to("cuda")
```

## CogView4Transformer2DModel[[diffusers.CogView4Transformer2DModel]]

#### diffusers.CogView4Transformer2DModel[[diffusers.CogView4Transformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_cogview4.py#L615)

forwarddiffusers.CogView4Transformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_cogview4.py#L702[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": LongTensor"}, {"name": "original_size", "val": ": Tensor"}, {"name": "target_size", "val": ": Tensor"}, {"name": "crop_coords", "val": ": Tensor"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "image_rotary_emb", "val": ": tuple[torch.Tensor, torch.Tensor] | list[tuple[torch.Tensor, torch.Tensor]] | None = None"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, in_channels, height, width)`) --
  Input `hidden_states`.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **original_size** (`torch.Tensor`) --
  Original image size conditioning.
- **target_size** (`torch.Tensor`) --
  Target image size conditioning.
- **crop_coords** (`torch.Tensor`) --
  Crop coordinates conditioning.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.
- **attention_mask** (`torch.Tensor`, *optional*) --
  Mask applied to attention scores.
- **image_rotary_emb** (`tuple` of `torch.Tensor`, *optional*) --
  Pre-computed rotary positional embeddings.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [CogView4Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/cogview4_transformer2d#diffusers.CogView4Transformer2DModel) forward method.

**Parameters:**

patch_size (`int`, defaults to `2`) : The size of the patches to use in the patch embedding layer.

in_channels (`int`, defaults to `16`) : The number of channels in the input.

num_layers (`int`, defaults to `30`) : The number of layers of Transformer blocks to use.

attention_head_dim (`int`, defaults to `40`) : The number of channels in each head.

num_attention_heads (`int`, defaults to `64`) : The number of heads to use for multi-head attention.

out_channels (`int`, defaults to `16`) : The number of channels in the output.

text_embed_dim (`int`, defaults to `4096`) : Input dimension of text embeddings from the text encoder.

time_embed_dim (`int`, defaults to `512`) : Output dimension of timestep embeddings.

condition_dim (`int`, defaults to `256`) : The embedding dimension of the input SDXL-style resolution conditions (original_size, target_size, crop_coords).

pos_embed_max_size (`int`, defaults to `128`) : The maximum resolution of the positional embeddings, from which slices of shape `H x W` are taken and added to input patched latents, where `H` and `W` are the latent height and width respectively. A value of 128 means that the maximum supported height and width for image generation is `128 * vae_scale_factor * patch_size => 128 * 8 * 2 => 2048`.

sample_size (`int`, defaults to `128`) : The base resolution of input latents. If height/width is not provided during generation, this value is used to determine the resolution as `sample_size * vae_scale_factor => 128 * 8 => 1024`

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
