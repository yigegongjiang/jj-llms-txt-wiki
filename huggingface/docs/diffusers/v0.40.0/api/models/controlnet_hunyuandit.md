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
controlnet = HunyuanDiT2DControlNetModel.from_pretrained("Tencent-Hunyuan/HunyuanDiT-v1.1-ControlNet-Diffusers-Pose", dtype=torch.float16)
```

## HunyuanDiT2DControlNetModel[[diffusers.HunyuanDiT2DControlNetModel]]

#### diffusers.HunyuanDiT2DControlNetModel[[diffusers.HunyuanDiT2DControlNetModel]]

```python
diffusers.HunyuanDiT2DControlNetModel(conditioning_channels: int = 3, num_attention_heads: int = 16, attention_head_dim: int = 88, in_channels: int | None = None, patch_size: int | None = None, activation_fn: str = 'gelu-approximate', sample_size = 32, hidden_size = 1152, transformer_num_layers: int = 40, mlp_ratio: float = 4.0, cross_attention_dim: int = 1024, cross_attention_dim_t5: int = 2048, pooled_projection_dim: int = 1024, text_len: int = 77, text_len_t5: int = 256, use_style_cond_and_image_meta_size: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/controlnets/controlnet_hunyuan.py#L40)

#### forward[[diffusers.HunyuanDiT2DControlNetModel.forward]]

```python
forward(hidden_states, timestep, controlnet_cond: Tensor, conditioning_scale: float = 1.0, encoder_hidden_states = None, text_embedding_mask = None, encoder_hidden_states_t5 = None, text_embedding_mask_t5 = None, image_meta_size = None, style = None, image_rotary_emb = None, return_dict = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/controlnets/controlnet_hunyuan.py#L215)

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

The [HunyuanDiT2DControlNetModel](/docs/diffusers/v0.40.0/en/api/models/controlnet_hunyuandit#diffusers.HunyuanDiT2DControlNetModel) forward method.

#### set_attn_processor[[diffusers.HunyuanDiT2DControlNetModel.set_attn_processor]]

```python
set_attn_processor(processor: diffusers.models.attention_processor.AttnProcessor | diffusers.models.attention_processor.CustomDiffusionAttnProcessor | diffusers.models.attention_processor.AttnAddedKVProcessor | diffusers.models.attention_processor.AttnAddedKVProcessor2_0 | diffusers.models.attention_processor.JointAttnProcessor2_0 | diffusers.models.attention_processor.PAGJointAttnProcessor2_0 | diffusers.models.attention_processor.PAGCFGJointAttnProcessor2_0 | diffusers.models.attention_processor.FusedJointAttnProcessor2_0 | diffusers.models.attention_processor.AllegroAttnProcessor2_0 | diffusers.models.attention_processor.AuraFlowAttnProcessor2_0 | diffusers.models.attention_processor.FusedAuraFlowAttnProcessor2_0 | diffusers.models.attention_processor.FluxAttnProcessor2_0 | diffusers.models.attention_processor.FluxAttnProcessor2_0_NPU | diffusers.models.attention_processor.FusedFluxAttnProcessor2_0 | diffusers.models.attention_processor.FusedFluxAttnProcessor2_0_NPU | diffusers.models.attention_processor.CogVideoXAttnProcessor2_0 | diffusers.models.attention_processor.FusedCogVideoXAttnProcessor2_0 | diffusers.models.attention_processor.XFormersAttnAddedKVProcessor | diffusers.models.attention_processor.XFormersAttnProcessor | diffusers.models.attention_processor.XLAFlashAttnProcessor2_0 | diffusers.models.attention_processor.AttnProcessorNPU | diffusers.models.attention_processor.AttnProcessor2_0 | diffusers.models.attention_processor.MochiVaeAttnProcessor2_0 | diffusers.models.attention_processor.MochiAttnProcessor2_0 | diffusers.models.attention_processor.StableAudioAttnProcessor2_0 | diffusers.models.attention_processor.HunyuanAttnProcessor2_0 | diffusers.models.attention_processor.FusedHunyuanAttnProcessor2_0 | diffusers.models.attention_processor.PAGHunyuanAttnProcessor2_0 | diffusers.models.attention_processor.PAGCFGHunyuanAttnProcessor2_0 | diffusers.models.attention_processor.LuminaAttnProcessor2_0 | diffusers.models.attention_processor.FusedAttnProcessor2_0 | diffusers.models.attention_processor.CustomDiffusionXFormersAttnProcessor | diffusers.models.attention_processor.CustomDiffusionAttnProcessor2_0 | diffusers.models.attention_processor.SlicedAttnProcessor | diffusers.models.attention_processor.SlicedAttnAddedKVProcessor | diffusers.models.attention_processor.SanaLinearAttnProcessor2_0 | diffusers.models.attention_processor.PAGCFGSanaLinearAttnProcessor2_0 | diffusers.models.attention_processor.PAGIdentitySanaLinearAttnProcessor2_0 | diffusers.models.attention_processor.SanaMultiscaleLinearAttention | diffusers.models.attention_processor.SanaMultiscaleAttnProcessor2_0 | diffusers.models.attention_processor.SanaMultiscaleAttentionProjection | diffusers.models.attention_processor.IPAdapterAttnProcessor | diffusers.models.attention_processor.IPAdapterAttnProcessor2_0 | diffusers.models.attention_processor.IPAdapterXFormersAttnProcessor | diffusers.models.attention_processor.SD3IPAdapterJointAttnProcessor2_0 | diffusers.models.attention_processor.PAGIdentitySelfAttnProcessor2_0 | diffusers.models.attention_processor.PAGCFGIdentitySelfAttnProcessor2_0 | diffusers.models.attention_processor.LoRAAttnProcessor | diffusers.models.attention_processor.LoRAAttnProcessor2_0 | diffusers.models.attention_processor.LoRAXFormersAttnProcessor | diffusers.models.attention_processor.LoRAAttnAddedKVProcessor | dict[str, diffusers.models.attention_processor.AttnProcessor | diffusers.models.attention_processor.CustomDiffusionAttnProcessor | diffusers.models.attention_processor.AttnAddedKVProcessor | diffusers.models.attention_processor.AttnAddedKVProcessor2_0 | diffusers.models.attention_processor.JointAttnProcessor2_0 | diffusers.models.attention_processor.PAGJointAttnProcessor2_0 | diffusers.models.attention_processor.PAGCFGJointAttnProcessor2_0 | diffusers.models.attention_processor.FusedJointAttnProcessor2_0 | diffusers.models.attention_processor.AllegroAttnProcessor2_0 | diffusers.models.attention_processor.AuraFlowAttnProcessor2_0 | diffusers.models.attention_processor.FusedAuraFlowAttnProcessor2_0 | diffusers.models.attention_processor.FluxAttnProcessor2_0 | diffusers.models.attention_processor.FluxAttnProcessor2_0_NPU | diffusers.models.attention_processor.FusedFluxAttnProcessor2_0 | diffusers.models.attention_processor.FusedFluxAttnProcessor2_0_NPU | diffusers.models.attention_processor.CogVideoXAttnProcessor2_0 | diffusers.models.attention_processor.FusedCogVideoXAttnProcessor2_0 | diffusers.models.attention_processor.XFormersAttnAddedKVProcessor | diffusers.models.attention_processor.XFormersAttnProcessor | diffusers.models.attention_processor.XLAFlashAttnProcessor2_0 | diffusers.models.attention_processor.AttnProcessorNPU | diffusers.models.attention_processor.AttnProcessor2_0 | diffusers.models.attention_processor.MochiVaeAttnProcessor2_0 | diffusers.models.attention_processor.MochiAttnProcessor2_0 | diffusers.models.attention_processor.StableAudioAttnProcessor2_0 | diffusers.models.attention_processor.HunyuanAttnProcessor2_0 | diffusers.models.attention_processor.FusedHunyuanAttnProcessor2_0 | diffusers.models.attention_processor.PAGHunyuanAttnProcessor2_0 | diffusers.models.attention_processor.PAGCFGHunyuanAttnProcessor2_0 | diffusers.models.attention_processor.LuminaAttnProcessor2_0 | diffusers.models.attention_processor.FusedAttnProcessor2_0 | diffusers.models.attention_processor.CustomDiffusionXFormersAttnProcessor | diffusers.models.attention_processor.CustomDiffusionAttnProcessor2_0 | diffusers.models.attention_processor.SlicedAttnProcessor | diffusers.models.attention_processor.SlicedAttnAddedKVProcessor | diffusers.models.attention_processor.SanaLinearAttnProcessor2_0 | diffusers.models.attention_processor.PAGCFGSanaLinearAttnProcessor2_0 | diffusers.models.attention_processor.PAGIdentitySanaLinearAttnProcessor2_0 | diffusers.models.attention_processor.SanaMultiscaleLinearAttention | diffusers.models.attention_processor.SanaMultiscaleAttnProcessor2_0 | diffusers.models.attention_processor.SanaMultiscaleAttentionProjection | diffusers.models.attention_processor.IPAdapterAttnProcessor | diffusers.models.attention_processor.IPAdapterAttnProcessor2_0 | diffusers.models.attention_processor.IPAdapterXFormersAttnProcessor | diffusers.models.attention_processor.SD3IPAdapterJointAttnProcessor2_0 | diffusers.models.attention_processor.PAGIdentitySelfAttnProcessor2_0 | diffusers.models.attention_processor.PAGCFGIdentitySelfAttnProcessor2_0 | diffusers.models.attention_processor.LoRAAttnProcessor | diffusers.models.attention_processor.LoRAAttnProcessor2_0 | diffusers.models.attention_processor.LoRAXFormersAttnProcessor | diffusers.models.attention_processor.LoRAAttnAddedKVProcessor])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/controlnets/controlnet_hunyuan.py#L141)

**Parameters:**

processor (`dict` of `AttentionProcessor` or only `AttentionProcessor`) : The instantiated processor class or a dictionary of processor classes that will be set as the processor for **all** `Attention` layers. If `processor` is a dict, the key needs to define the path to the corresponding cross attention processor. This is strongly recommended when setting trainable attention processors.

Sets the attention processor to use to compute attention.
