# StableCascadeUNet

A UNet model from the [Stable Cascade pipeline](../pipelines/stable_cascade).

## StableCascadeUNet[[diffusers.models.StableCascadeUNet]]

#### diffusers.models.StableCascadeUNet[[diffusers.models.StableCascadeUNet]]

```python
diffusers.models.StableCascadeUNet(in_channels: int = 16, out_channels: int = 16, timestep_ratio_embedding_dim: int = 64, patch_size: int = 1, conditioning_dim: int = 2048, block_out_channels: tuple = (2048, 2048), num_attention_heads: tuple = (32, 32), down_num_layers_per_block: tuple = (8, 24), up_num_layers_per_block: tuple = (24, 8), down_blocks_repeat_mappers: tuple[int] | None = (1, 1), up_blocks_repeat_mappers: tuple[int] | None = (1, 1), block_types_per_layer: tuple = (('SDCascadeResBlock', 'SDCascadeTimestepBlock', 'SDCascadeAttnBlock'), ('SDCascadeResBlock', 'SDCascadeTimestepBlock', 'SDCascadeAttnBlock')), clip_text_in_channels: int | None = None, clip_text_pooled_in_channels = 1280, clip_image_in_channels: int | None = None, clip_seq = 4, effnet_in_channels: int | None = None, pixel_mapper_in_channels: int | None = None, kernel_size = 3, dropout: float | tuple[float] = (0.1, 0.1), self_attn: bool | tuple[bool] = True, timestep_conditioning_type: tuple = ('sca', 'crp'), switch_level: tuple[bool] | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_stable_cascade.py#L136)

#### forward[[diffusers.models.StableCascadeUNet.forward]]

```python
forward(sample, timestep_ratio, clip_text_pooled, clip_text = None, clip_img = None, effnet = None, pixels = None, sca = None, crp = None, return_dict = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/unet_stable_cascade.py#L538)

**Parameters:**

sample (`torch.Tensor`) : The noisy input sample.

timestep_ratio (`torch.Tensor`) : Timestep ratio used to compute the timestep embedding.

clip_text_pooled (`torch.Tensor`) : Pooled CLIP text embeddings.

clip_text (`torch.Tensor`, *optional*) : Sequence-level CLIP text embeddings.

clip_img (`torch.Tensor`, *optional*) : CLIP image embeddings.

effnet (`torch.Tensor`, *optional*) : EfficientNet feature map used as additional conditioning.

pixels (`torch.Tensor`, *optional*) : Pixel-level conditioning tensor. If `None`, a tensor of zeros is used.

sca (`torch.Tensor`, *optional*) : Optional `sca` conditioning value used to build the timestep embedding.

crp (`torch.Tensor`, *optional*) : Optional `crp` conditioning value used to build the timestep embedding.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `StableCascadeUNetOutput` instead of a plain tuple.
