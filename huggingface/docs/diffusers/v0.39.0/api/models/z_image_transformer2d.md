# ZImageTransformer2DModel

A Transformer model for image-like data from [Z-Image](https://huggingface.co/Tongyi-MAI/Z-Image-Turbo).

## ZImageTransformer2DModel[[diffusers.ZImageTransformer2DModel]]

#### diffusers.ZImageTransformer2DModel[[diffusers.ZImageTransformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_z_image.py#L359)

forwarddiffusers.ZImageTransformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_z_image.py#L894[{"name": "x", "val": ": list"}, {"name": "t", "val": ""}, {"name": "cap_feats", "val": ": list"}, {"name": "return_dict", "val": ": bool = True"}, {"name": "controlnet_block_samples", "val": ": dict[int, torch.Tensor] | None = None"}, {"name": "siglip_feats", "val": ": list[list[torch.Tensor]] | None = None"}, {"name": "image_noise_mask", "val": ": list[list[int]] | None = None"}, {"name": "patch_size", "val": ": int = 2"}, {"name": "f_patch_size", "val": ": int = 1"}]- **x** (`list` of `torch.Tensor` or nested `list` of `torch.Tensor`) --
  Input latents. A flat list when running in standard mode, or a nested list when running in omni mode.
- **t** (`torch.Tensor`) --
  Used to indicate denoising step.
- **cap_feats** (`list` of `torch.Tensor` or nested `list` of `torch.Tensor`) --
  Conditional caption embeddings (embeddings computed from the input conditions such as prompts) to use.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.
- **controlnet_block_samples** (`dict` of `int` to `torch.Tensor`, *optional*) --
  A mapping from block index to tensor that if specified are added to the residuals of transformer
  blocks.
- **siglip_feats** (`list` of `list` of `torch.Tensor`, *optional*) --
  Optional SigLIP image features used as additional conditioning.
- **image_noise_mask** (`list` of `list` of `int`, *optional*) --
  Per-image noise masks indicating noisy vs. clean tokens in omni mode.
- **patch_size** (`int`, *optional*, defaults to 2) --
  Spatial patch size used to patchify the input latents.
- **f_patch_size** (`int`, *optional*, defaults to 1) --
  Temporal patch size used to patchify the input latents.0

The [ZImageTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/z_image_transformer2d#diffusers.ZImageTransformer2DModel) forward method.

Flow: patchify -> t_embed -> x_embed -> x_refine -> cap_embed -> cap_refine
-> [siglip_embed -> siglip_refine] -> build_unified -> main_layers -> final_layer -> unpatchify

**Parameters:**

x (`list` of `torch.Tensor` or nested `list` of `torch.Tensor`) : Input latents. A flat list when running in standard mode, or a nested list when running in omni mode.

t (`torch.Tensor`) : Used to indicate denoising step.

cap_feats (`list` of `torch.Tensor` or nested `list` of `torch.Tensor`) : Conditional caption embeddings (embeddings computed from the input conditions such as prompts) to use.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

controlnet_block_samples (`dict` of `int` to `torch.Tensor`, *optional*) : A mapping from block index to tensor that if specified are added to the residuals of transformer blocks.

siglip_feats (`list` of `list` of `torch.Tensor`, *optional*) : Optional SigLIP image features used as additional conditioning.

image_noise_mask (`list` of `list` of `int`, *optional*) : Per-image noise masks indicating noisy vs. clean tokens in omni mode.

patch_size (`int`, *optional*, defaults to 2) : Spatial patch size used to patchify the input latents.

f_patch_size (`int`, *optional*, defaults to 1) : Temporal patch size used to patchify the input latents.
#### patchify_and_embed[[diffusers.ZImageTransformer2DModel.patchify_and_embed]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_z_image.py#L588)

Patchify for basic mode: single image per batch item.
#### patchify_and_embed_omni[[diffusers.ZImageTransformer2DModel.patchify_and_embed_omni]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_z_image.py#L625)

Patchify for omni mode: multiple images per batch item with noise masks.
