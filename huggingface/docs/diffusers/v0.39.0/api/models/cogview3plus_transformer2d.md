# CogView3PlusTransformer2DModel

A Diffusion Transformer model for 2D data from [CogView3Plus](https://github.com/THUDM/CogView3) was introduced in [CogView3: Finer and Faster Text-to-Image Generation via Relay Diffusion](https://huggingface.co/papers/2403.05121) by Tsinghua University & ZhipuAI.

The model can be loaded with the following code snippet.

```python
from diffusers import CogView3PlusTransformer2DModel

transformer = CogView3PlusTransformer2DModel.from_pretrained("THUDM/CogView3Plus-3b", subfolder="transformer", torch_dtype=torch.bfloat16).to("cuda")
```

## CogView3PlusTransformer2DModel[[diffusers.CogView3PlusTransformer2DModel]]

#### diffusers.CogView3PlusTransformer2DModel[[diffusers.CogView3PlusTransformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_cogview3plus.py#L126)

The Transformer model introduced in [CogView3: Finer and Faster Text-to-Image Generation via Relay
Diffusion](https://huggingface.co/papers/2403.05121).

forwarddiffusers.CogView3PlusTransformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_cogview3plus.py#L225[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": LongTensor"}, {"name": "original_size", "val": ": Tensor"}, {"name": "target_size", "val": ": Tensor"}, {"name": "crop_coords", "val": ": Tensor"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor`) --
  Input `hidden_states` of shape `(batch size, channel, height, width)`.
- **encoder_hidden_states** (`torch.Tensor`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) of shape
  `(batch_size, sequence_len, text_embed_dim)`
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **original_size** (`torch.Tensor`) --
  CogView3 uses SDXL-like micro-conditioning for original image size as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **target_size** (`torch.Tensor`) --
  CogView3 uses SDXL-like micro-conditioning for target image size as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **crop_coords** (`torch.Tensor`) --
  CogView3 uses SDXL-like micro-conditioning for crop coordinates as explained in section 2.2 of
  [https://huggingface.co/papers/2307.01952](https://huggingface.co/papers/2307.01952).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0`torch.Tensor` or `~models.transformer_2d.Transformer2DModelOutput`The denoised latents using provided inputs as conditioning.

The [CogView3PlusTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/cogview3plus_transformer2d#diffusers.CogView3PlusTransformer2DModel) forward method.

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

``torch.Tensor` or `~models.transformer_2d.Transformer2DModelOutput``

The denoised latents using provided inputs as conditioning.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
