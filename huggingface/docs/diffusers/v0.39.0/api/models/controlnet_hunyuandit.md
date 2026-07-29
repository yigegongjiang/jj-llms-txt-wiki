# HunyuanDiT2DControlNetModel

HunyuanDiT2DControlNetModel is an implementation of ControlNet for [Hunyuan-DiT](https://huggingface.co/papers/2405.08748).

ControlNet was introduced in [Adding Conditional Control to Text-to-Image Diffusion Models](https://huggingface.co/papers/2302.05543) by Lvmin Zhang, Anyi Rao, and Maneesh Agrawala.

With a ControlNet model, you can provide an additional control image to condition and control Hunyuan-DiT generation. For example, if you provide a depth map, the ControlNet model generates an image that'll preserve the spatial information from the depth map. It is a more flexible and accurate way to control the image generation process.

The abstract from the paper is:

*We present ControlNet, a neural network architecture to add spatial conditioning controls to large, pretrained text-to-image diffusion models. ControlNet locks the production-ready large diffusion models, and reuses their deep and robust encoding layers pretrained with billions of images as a strong backbone to learn a diverse set of conditional controls. The neural architecture is connected with "zero convolutions" (zero-initialized convolution layers) that progressively grow the parameters from zero and ensure that no harmful noise could affect the finetuning. We test various conditioning controls, eg, edges, depth, segmentation, human pose, etc, with Stable Diffusion, using single or multiple conditions, with or without prompts. We show that the training of ControlNets is robust with small (<50k) and large (>1m) datasets. Extensive results show that ControlNet may facilitate wider applications to control image diffusion models.*

This code is implemented by Tencent Hunyuan Team. You can find pre-trained checkpoints for Hunyuan-DiT ControlNets on [Tencent Hunyuan](https://huggingface.co/Tencent-Hunyuan).

## Example For Loading HunyuanDiT2DControlNetModel

```py
from diffusers import HunyuanDiT2DControlNetModel
import torch
controlnet = HunyuanDiT2DControlNetModel.from_pretrained("Tencent-Hunyuan/HunyuanDiT-v1.1-ControlNet-Diffusers-Pose", torch_dtype=torch.float16)
```

## HunyuanDiT2DControlNetModel[[diffusers.HunyuanDiT2DControlNetModel]]

#### diffusers.HunyuanDiT2DControlNetModel[[diffusers.HunyuanDiT2DControlNetModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_hunyuan.py#L40)

forwarddiffusers.HunyuanDiT2DControlNetModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_hunyuan.py#L215[{"name": "hidden_states", "val": ""}, {"name": "timestep", "val": ""}, {"name": "controlnet_cond", "val": ": Tensor"}, {"name": "conditioning_scale", "val": ": float = 1.0"}, {"name": "encoder_hidden_states", "val": " = None"}, {"name": "text_embedding_mask", "val": " = None"}, {"name": "encoder_hidden_states_t5", "val": " = None"}, {"name": "text_embedding_mask_t5", "val": " = None"}, {"name": "image_meta_size", "val": " = None"}, {"name": "style", "val": " = None"}, {"name": "image_rotary_emb", "val": " = None"}, {"name": "return_dict", "val": " = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch size, dim, height, width)`) --
  The input tensor.
- **timestep** ( `torch.LongTensor`, *optional*) --
  Used to indicate denoising step.
- **controlnet_cond** ( `torch.Tensor` ) --
  The conditioning input to ControlNet.
- **conditioning_scale** ( `float` ) --
  Indicate the conditioning scale.
- **encoder_hidden_states** ( `torch.Tensor` of shape `(batch size, sequence len, embed dims)`, *optional*) --
  Conditional embeddings for cross attention layer. This is the output of `BertModel`.
- **text_embedding_mask** -- torch.Tensor
  An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. This is the output
  of `BertModel`.
- **encoder_hidden_states_t5** ( `torch.Tensor` of shape `(batch size, sequence len, embed dims)`, *optional*) --
  Conditional embeddings for cross attention layer. This is the output of T5 Text Encoder.
- **text_embedding_mask_t5** -- torch.Tensor
  An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. This is the output
  of T5 Text Encoder.
- **image_meta_size** (torch.Tensor) --
  Conditional embedding indicate the image sizes
- **style** -- torch.Tensor:
  Conditional embedding indicate the style
- **image_rotary_emb** (`torch.Tensor`) --
  The image rotary embeddings to apply on query and key tensors during attention calculation.
- **return_dict** -- bool
  Whether to return a dictionary.0

The [HunyuanDiT2DControlNetModel](/docs/diffusers/v0.39.0/en/api/models/controlnet_hunyuandit#diffusers.HunyuanDiT2DControlNetModel) forward method.

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch size, dim, height, width)`) : The input tensor.

timestep ( `torch.LongTensor`, *optional*) : Used to indicate denoising step.

controlnet_cond ( `torch.Tensor` ) : The conditioning input to ControlNet.

conditioning_scale ( `float` ) : Indicate the conditioning scale.

encoder_hidden_states ( `torch.Tensor` of shape `(batch size, sequence len, embed dims)`, *optional*) : Conditional embeddings for cross attention layer. This is the output of `BertModel`.

text_embedding_mask : torch.Tensor An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. This is the output of `BertModel`.

encoder_hidden_states_t5 ( `torch.Tensor` of shape `(batch size, sequence len, embed dims)`, *optional*) : Conditional embeddings for cross attention layer. This is the output of T5 Text Encoder.

text_embedding_mask_t5 : torch.Tensor An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. This is the output of T5 Text Encoder.

image_meta_size (torch.Tensor) : Conditional embedding indicate the image sizes

style : torch.Tensor: Conditional embedding indicate the style

image_rotary_emb (`torch.Tensor`) : The image rotary embeddings to apply on query and key tensors during attention calculation.

return_dict : bool Whether to return a dictionary.
#### set_attn_processor[[diffusers.HunyuanDiT2DControlNetModel.set_attn_processor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/controlnets/controlnet_hunyuan.py#L141)

Sets the attention processor to use to compute attention.

**Parameters:**

processor (`dict` of `AttentionProcessor` or only `AttentionProcessor`) : The instantiated processor class or a dictionary of processor classes that will be set as the processor for **all** `Attention` layers. If `processor` is a dict, the key needs to define the path to the corresponding cross attention processor. This is strongly recommended when setting trainable attention processors.
