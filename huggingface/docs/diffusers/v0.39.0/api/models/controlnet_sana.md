# SanaControlNetModel

The ControlNet model was introduced in [Adding Conditional Control to Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.05543) by Lvmin Zhang, Anyi Rao, Maneesh Agrawala. It provides a greater degree of control over text-to-image generation by conditioning the model on additional inputs such as edge maps, depth maps, segmentation maps, and keypoints for pose detection.

The abstract from the paper is:

*We present ControlNet, a neural network architecture to add spatial conditioning controls to large, pretrained text-to-image diffusion models. ControlNet locks the production-ready large diffusion models, and reuses their deep and robust encoding layers pretrained with billions of images as a strong backbone to learn a diverse set of conditional controls. The neural architecture is connected with "zero convolutions" (zero-initialized convolution layers) that progressively grow the parameters from zero and ensure that no harmful noise could affect the finetuning. We test various conditioning controls, eg, edges, depth, segmentation, human pose, etc, with Stable Diffusion, using single or multiple conditions, with or without prompts. We show that the training of ControlNets is robust with small (<50k) and large (>1m) datasets. Extensive results show that ControlNet may facilitate wider applications to control image diffusion models.*

This model was contributed by [ishan24](https://huggingface.co/ishan24). ❤️
The original codebase can be found at [NVlabs/Sana](https://github.com/NVlabs/Sana), and you can find official ControlNet checkpoints on [Efficient-Large-Model's](https://huggingface.co/Efficient-Large-Model) Hub profile.

## SanaControlNetModel[[diffusers.SanaControlNetModel]]
#### diffusers.SanaControlNetModel[[diffusers.SanaControlNetModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_sana.py#L41)

forwarddiffusers.SanaControlNetModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_sana.py#L120[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": LongTensor"}, {"name": "controlnet_cond", "val": ": Tensor"}, {"name": "conditioning_scale", "val": ": float = 1.0"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, channel, height, width)`) --
  Input `hidden_states`.
- **encoder_hidden_states** (`torch.Tensor`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **controlnet_cond** (`torch.Tensor`) --
  The conditional input tensor for the ControlNet.
- **conditioning_scale** (`float`, *optional*, defaults to `1.0`) --
  The scale factor for ControlNet outputs.
- **encoder_attention_mask** (`torch.Tensor`, *optional*) --
  Attention mask applied to `encoder_hidden_states`.
- **attention_mask** (`torch.Tensor`, *optional*) --
  Attention mask applied to `hidden_states`.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0`~models.transformer_2d.Transformer2DModelOutput` or `tuple`If `return_dict` is True, a `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise
a plain `tuple` is returned.

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, channel, height, width)`) : Input `hidden_states`.

encoder_hidden_states (`torch.Tensor`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

controlnet_cond (`torch.Tensor`) : The conditional input tensor for the ControlNet.

conditioning_scale (`float`, *optional*, defaults to `1.0`) : The scale factor for ControlNet outputs.

encoder_attention_mask (`torch.Tensor`, *optional*) : Attention mask applied to `encoder_hidden_states`.

attention_mask (`torch.Tensor`, *optional*) : Attention mask applied to `hidden_states`.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

``~models.transformer_2d.Transformer2DModelOutput` or `tuple``

If `return_dict` is True, a `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise
a plain `tuple` is returned.

## SanaControlNetOutput[[diffusers.models.controlnets.controlnet_sana.SanaControlNetOutput]]
#### diffusers.models.controlnets.controlnet_sana.SanaControlNetOutput[[diffusers.models.controlnets.controlnet_sana.SanaControlNetOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_sana.py#L37)
