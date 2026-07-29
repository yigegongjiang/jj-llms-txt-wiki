# StableCascadeUNet

A UNet model from the [Stable Cascade pipeline](../pipelines/stable_cascade).

## StableCascadeUNet[[diffusers.models.StableCascadeUNet]]

#### diffusers.models.StableCascadeUNet[[diffusers.models.StableCascadeUNet]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_stable_cascade.py#L136)

forwarddiffusers.models.StableCascadeUNet.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_stable_cascade.py#L538[{"name": "sample", "val": ""}, {"name": "timestep_ratio", "val": ""}, {"name": "clip_text_pooled", "val": ""}, {"name": "clip_text", "val": " = None"}, {"name": "clip_img", "val": " = None"}, {"name": "effnet", "val": " = None"}, {"name": "pixels", "val": " = None"}, {"name": "sca", "val": " = None"}, {"name": "crp", "val": " = None"}, {"name": "return_dict", "val": " = True"}]- **sample** (`torch.Tensor`) -- The noisy input sample.
- **timestep_ratio** (`torch.Tensor`) --
  Timestep ratio used to compute the timestep embedding.
- **clip_text_pooled** (`torch.Tensor`) --
  Pooled CLIP text embeddings.
- **clip_text** (`torch.Tensor`, *optional*) --
  Sequence-level CLIP text embeddings.
- **clip_img** (`torch.Tensor`, *optional*) --
  CLIP image embeddings.
- **effnet** (`torch.Tensor`, *optional*) --
  EfficientNet feature map used as additional conditioning.
- **pixels** (`torch.Tensor`, *optional*) --
  Pixel-level conditioning tensor. If `None`, a tensor of zeros is used.
- **sca** (`torch.Tensor`, *optional*) --
  Optional `sca` conditioning value used to build the timestep embedding.
- **crp** (`torch.Tensor`, *optional*) --
  Optional `crp` conditioning value used to build the timestep embedding.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `StableCascadeUNetOutput` instead of a plain tuple.0

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
