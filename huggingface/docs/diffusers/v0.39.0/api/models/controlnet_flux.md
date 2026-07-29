# FluxControlNetModel

FluxControlNetModel is an implementation of ControlNet for Flux.1.

The ControlNet model was introduced in [Adding Conditional Control to Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.05543) by Lvmin Zhang, Anyi Rao, Maneesh Agrawala. It provides a greater degree of control over text-to-image generation by conditioning the model on additional inputs such as edge maps, depth maps, segmentation maps, and keypoints for pose detection.

The abstract from the paper is:

*We present ControlNet, a neural network architecture to add spatial conditioning controls to large, pretrained text-to-image diffusion models. ControlNet locks the production-ready large diffusion models, and reuses their deep and robust encoding layers pretrained with billions of images as a strong backbone to learn a diverse set of conditional controls. The neural architecture is connected with "zero convolutions" (zero-initialized convolution layers) that progressively grow the parameters from zero and ensure that no harmful noise could affect the finetuning. We test various conditioning controls, eg, edges, depth, segmentation, human pose, etc, with Stable Diffusion, using single or multiple conditions, with or without prompts. We show that the training of ControlNets is robust with small (<50k) and large (>1m) datasets. Extensive results show that ControlNet may facilitate wider applications to control image diffusion models.*

## Loading from the original format

By default the [FluxControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_flux#diffusers.FluxControlNetModel) should be loaded with [from_pretrained()](/docs/diffusers/v0.39.0/en/api/models/overview#diffusers.ModelMixin.from_pretrained).

```py
from diffusers import FluxControlNetPipeline
from diffusers.models import FluxControlNetModel, FluxMultiControlNetModel

controlnet = FluxControlNetModel.from_pretrained("InstantX/FLUX.1-dev-Controlnet-Canny")
pipe = FluxControlNetPipeline.from_pretrained("black-forest-labs/FLUX.1-dev", controlnet=controlnet)

controlnet = FluxControlNetModel.from_pretrained("InstantX/FLUX.1-dev-Controlnet-Canny")
controlnet = FluxMultiControlNetModel([controlnet])
pipe = FluxControlNetPipeline.from_pretrained("black-forest-labs/FLUX.1-dev", controlnet=controlnet)
```

## FluxControlNetModel[[diffusers.FluxControlNetModel]]

#### diffusers.FluxControlNetModel[[diffusers.FluxControlNetModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_flux.py#L45)

forwarddiffusers.FluxControlNetModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_flux.py#L157[{"name": "hidden_states", "val": ": Tensor"}, {"name": "controlnet_cond", "val": ": Tensor"}, {"name": "controlnet_mode", "val": ": Tensor = None"}, {"name": "conditioning_scale", "val": ": float = 1.0"}, {"name": "encoder_hidden_states", "val": ": Tensor = None"}, {"name": "pooled_projections", "val": ": Tensor = None"}, {"name": "timestep", "val": ": LongTensor = None"}, {"name": "img_ids", "val": ": Tensor = None"}, {"name": "txt_ids", "val": ": Tensor = None"}, {"name": "guidance", "val": ": Tensor = None"}, {"name": "joint_attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.FloatTensor` of shape `(batch size, channel, height, width)`) --
  Input `hidden_states`.
- **controlnet_cond** (`torch.Tensor`) --
  The conditional input tensor of shape `(batch_size, sequence_length, hidden_size)`.
- **controlnet_mode** (`torch.Tensor`) --
  The mode tensor of shape `(batch_size, 1)`.
- **conditioning_scale** (`float`, defaults to `1.0`) --
  The scale factor for ControlNet outputs.
- **encoder_hidden_states** (`torch.FloatTensor` of shape `(batch size, sequence_len, embed_dims)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **pooled_projections** (`torch.FloatTensor` of shape `(batch_size, projection_dim)`) -- Embeddings projected
  from the embeddings of input conditions.
- **timestep** ( `torch.LongTensor`) --
  Used to indicate denoising step.
- **img_ids** (`torch.Tensor`) --
  Positional ids for the image tokens.
- **txt_ids** (`torch.Tensor`) --
  Positional ids for the text tokens.
- **guidance** (`torch.Tensor`, *optional*) --
  Guidance scale tensor used by guidance-distilled variants of the model.
- **joint_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [FluxTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/flux_transformer#diffusers.FluxTransformer2DModel) forward method.

**Parameters:**

hidden_states (`torch.FloatTensor` of shape `(batch size, channel, height, width)`) : Input `hidden_states`.

controlnet_cond (`torch.Tensor`) : The conditional input tensor of shape `(batch_size, sequence_length, hidden_size)`.

controlnet_mode (`torch.Tensor`) : The mode tensor of shape `(batch_size, 1)`.

conditioning_scale (`float`, defaults to `1.0`) : The scale factor for ControlNet outputs.

encoder_hidden_states (`torch.FloatTensor` of shape `(batch size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

pooled_projections (`torch.FloatTensor` of shape `(batch_size, projection_dim)`) : Embeddings projected from the embeddings of input conditions.

timestep ( `torch.LongTensor`) : Used to indicate denoising step.

img_ids (`torch.Tensor`) : Positional ids for the image tokens.

txt_ids (`torch.Tensor`) : Positional ids for the text tokens.

guidance (`torch.Tensor`, *optional*) : Guidance scale tensor used by guidance-distilled variants of the model.

joint_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

## FluxControlNetOutput[[diffusers.models.controlnets.FluxControlNetOutput]]

#### diffusers.models.controlnets.FluxControlNetOutput[[diffusers.models.controlnets.FluxControlNetOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_flux.py#L40)
