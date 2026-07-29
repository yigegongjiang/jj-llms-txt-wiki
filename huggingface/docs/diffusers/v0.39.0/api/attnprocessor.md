# Attention Processor

An attention processor is a class for applying different types of attention mechanisms.

## AttnProcessor[[diffusers.models.attention_processor.AttnProcessor]]

#### diffusers.models.attention_processor.AttnProcessor[[diffusers.models.attention_processor.AttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L1103)

Default processor for performing attention-related computations.

#### diffusers.models.attention_processor.AttnProcessor2_0[[diffusers.models.attention_processor.AttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2696)

Processor for implementing scaled dot-product attention (enabled by default if you're using PyTorch 2.0).

#### diffusers.models.attention_processor.AttnAddedKVProcessor[[diffusers.models.attention_processor.AttnAddedKVProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L1279)

Processor for performing attention-related computations with extra learnable key and value matrices for the text
encoder.

#### diffusers.models.attention_processor.AttnAddedKVProcessor2_0[[diffusers.models.attention_processor.AttnAddedKVProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L1346)

Processor for performing scaled dot-product attention (enabled by default if you're using PyTorch 2.0), with extra
learnable key and value matrices for the text encoder.

#### diffusers.models.attention_processor.AttnProcessorNPU[[diffusers.models.attention_processor.AttnProcessorNPU]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2582)

Processor for implementing flash attention using torch_npu. Torch_npu supports only fp16 and bf16 data types. If
fp32 is used, F.scaled_dot_product_attention will be used for computation, but the acceleration effect on NPU is
not significant.

#### diffusers.models.attention_processor.FusedAttnProcessor2_0[[diffusers.models.attention_processor.FusedAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L3668)

Processor for implementing scaled dot-product attention (enabled by default if you're using PyTorch 2.0). It uses
fused projection layers. For self-attention modules, all projection matrices (i.e., query, key, value) are fused.
For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is currently 🧪 experimental in nature and can change in future.

## Allegro[[diffusers.models.attention_processor.AllegroAttnProcessor2_0]]

#### diffusers.models.attention_processor.AllegroAttnProcessor2_0[[diffusers.models.attention_processor.AllegroAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L1993)

Processor for implementing scaled dot-product attention (enabled by default if you're using PyTorch 2.0). This is
used in the Allegro model. It applies a normalization layer and rotary embedding on the query and key vector.

## AuraFlow[[diffusers.models.attention_processor.AuraFlowAttnProcessor2_0]]

#### diffusers.models.attention_processor.AuraFlowAttnProcessor2_0[[diffusers.models.attention_processor.AuraFlowAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2087)

Attention processor used typically in processing Aura Flow.

#### diffusers.models.attention_processor.FusedAuraFlowAttnProcessor2_0[[diffusers.models.attention_processor.FusedAuraFlowAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2180)

Attention processor used typically in processing Aura Flow with fused projections.

## CogVideoX[[diffusers.models.attention_processor.CogVideoXAttnProcessor2_0]]

#### diffusers.models.attention_processor.CogVideoXAttnProcessor2_0[[diffusers.models.attention_processor.CogVideoXAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2277)

Processor for implementing scaled dot-product attention for the CogVideoX model. It applies a rotary embedding on
query and key vectors, but does not include spatial normalization.

#### diffusers.models.attention_processor.FusedCogVideoXAttnProcessor2_0[[diffusers.models.attention_processor.FusedCogVideoXAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2346)

Processor for implementing scaled dot-product attention for the CogVideoX model. It applies a rotary embedding on
query and key vectors, but does not include spatial normalization.

## DreamLite[[diffusers.models.unets.unet_dreamlite.DreamLiteAttnProcessor2_0]]

#### diffusers.models.unets.unet_dreamlite.DreamLiteAttnProcessor2_0[[diffusers.models.unets.unet_dreamlite.DreamLiteAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/unet_dreamlite.py#L285)

Processor for implementing scaled dot-product attention with Grouped Query Attention (GQA / MQA) support.

Identical to `AttnProcessor2_0` except the key/value reshape branch correctly handles `attn.kv_heads != attn.heads` by reshaping K/V to `kv_heads` and then `repeat_interleave`-ing them up to `attn.heads`. This is
required by the DreamLite UNet, which combines GQA with `qk_norm` — a combination the default
`AttnProcessor2_0` does not handle. SDPA is delegated to `dispatch_attention_fn` so any of the
diffusers attention backends (native PyTorch SDPA, FlashAttention, etc.) can be used.

## CrossFrameAttnProcessor[[diffusers.pipelines.deprecated.text_to_video_synthesis.pipeline_text_to_video_zero.CrossFrameAttnProcessor]]

#### diffusers.pipelines.deprecated.text_to_video_synthesis.pipeline_text_to_video_zero.CrossFrameAttnProcessor[[diffusers.pipelines.deprecated.text_to_video_synthesis.pipeline_text_to_video_zero.CrossFrameAttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/pipelines/deprecated/text_to_video_synthesis/pipeline_text_to_video_zero.py#L62)

Cross frame attention processor. Each frame attends the first frame.

**Parameters:**

batch_size : The number that represents actual batch size, other than the frames. For example, calling unet with a single prompt and num_images_per_prompt=1, batch_size should be equal to 2, due to classifier-free guidance.

## Custom Diffusion[[diffusers.models.attention_processor.CustomDiffusionAttnProcessor]]

#### diffusers.models.attention_processor.CustomDiffusionAttnProcessor[[diffusers.models.attention_processor.CustomDiffusionAttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L1175)

Processor for implementing attention for the Custom Diffusion method.

**Parameters:**

train_kv (`bool`, defaults to `True`) : Whether to newly train the key and value matrices corresponding to the text features.

train_q_out (`bool`, defaults to `True`) : Whether to newly train query matrices corresponding to the latent image features.

hidden_size (`int`, *optional*, defaults to `None`) : The hidden size of the attention layer.

cross_attention_dim (`int`, *optional*, defaults to `None`) : The number of channels in the `encoder_hidden_states`.

out_bias (`bool`, defaults to `True`) : Whether to include the bias parameter in `train_q_out`.

dropout (`float`, *optional*, defaults to 0.0) : The dropout probability to use.

#### diffusers.models.attention_processor.CustomDiffusionAttnProcessor2_0[[diffusers.models.attention_processor.CustomDiffusionAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L3886)

Processor for implementing attention for the Custom Diffusion method using PyTorch 2.0’s memory-efficient scaled
dot-product attention.

**Parameters:**

train_kv (`bool`, defaults to `True`) : Whether to newly train the key and value matrices corresponding to the text features.

train_q_out (`bool`, defaults to `True`) : Whether to newly train query matrices corresponding to the latent image features.

hidden_size (`int`, *optional*, defaults to `None`) : The hidden size of the attention layer.

cross_attention_dim (`int`, *optional*, defaults to `None`) : The number of channels in the `encoder_hidden_states`.

out_bias (`bool`, defaults to `True`) : Whether to include the bias parameter in `train_q_out`.

dropout (`float`, *optional*, defaults to 0.0) : The dropout probability to use.

#### diffusers.models.attention_processor.CustomDiffusionXFormersAttnProcessor[[diffusers.models.attention_processor.CustomDiffusionXFormersAttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L3770)

Processor for implementing memory efficient attention using xFormers for the Custom Diffusion method.

**Parameters:**

train_kv (`bool`, defaults to `True`) : Whether to newly train the key and value matrices corresponding to the text features.

train_q_out (`bool`, defaults to `True`) : Whether to newly train query matrices corresponding to the latent image features.

hidden_size (`int`, *optional*, defaults to `None`) : The hidden size of the attention layer.

cross_attention_dim (`int`, *optional*, defaults to `None`) : The number of channels in the `encoder_hidden_states`.

out_bias (`bool`, defaults to `True`) : Whether to include the bias parameter in `train_q_out`.

dropout (`float`, *optional*, defaults to 0.0) : The dropout probability to use.

attention_op (`Callable`, *optional*, defaults to `None`) : The base [operator](https://facebookresearch.github.io/xformers/components/ops.html#xformers.ops.AttentionOpBase) to use as the attention operator. It is recommended to set to `None`, and allow xFormers to choose the best operator.

## Flux[[diffusers.models.attention_processor.FluxAttnProcessor2_0]]

#### diffusers.models.attention_processor.FluxAttnProcessor2_0[[diffusers.models.attention_processor.FluxAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5505)

#### diffusers.models.attention_processor.FusedFluxAttnProcessor2_0[[diffusers.models.attention_processor.FusedFluxAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5529)

#### diffusers.models.attention_processor.FluxSingleAttnProcessor2_0[[diffusers.models.attention_processor.FluxSingleAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5515)

Processor for implementing scaled dot-product attention (enabled by default if you're using PyTorch 2.0).

## Hunyuan[[diffusers.models.attention_processor.HunyuanAttnProcessor2_0]]

#### diffusers.models.attention_processor.HunyuanAttnProcessor2_0[[diffusers.models.attention_processor.HunyuanAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L3124)

Processor for implementing scaled dot-product attention (enabled by default if you're using PyTorch 2.0). This is
used in the HunyuanDiT model. It applies a s normalization layer and rotary embedding on query and key vector.

#### diffusers.models.attention_processor.FusedHunyuanAttnProcessor2_0[[diffusers.models.attention_processor.FusedHunyuanAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L3222)

Processor for implementing scaled dot-product attention (enabled by default if you're using PyTorch 2.0) with fused
projection layers. This is used in the HunyuanDiT model. It applies a s normalization layer and rotary embedding on
query and key vector.

#### diffusers.models.attention_processor.PAGHunyuanAttnProcessor2_0[[diffusers.models.attention_processor.PAGHunyuanAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L3325)

Processor for implementing scaled dot-product attention (enabled by default if you're using PyTorch 2.0). This is
used in the HunyuanDiT model. It applies a normalization layer and rotary embedding on query and key vector. This
variant of the processor employs [Pertubed Attention Guidance](https://huggingface.co/papers/2403.17377).

#### diffusers.models.attention_processor.PAGCFGHunyuanAttnProcessor2_0[[diffusers.models.attention_processor.PAGCFGHunyuanAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L3448)

Processor for implementing scaled dot-product attention (enabled by default if you're using PyTorch 2.0). This is
used in the HunyuanDiT model. It applies a normalization layer and rotary embedding on query and key vector. This
variant of the processor employs [Pertubed Attention Guidance](https://huggingface.co/papers/2403.17377).

## IdentitySelfAttnProcessor2_0[[diffusers.models.attention_processor.PAGIdentitySelfAttnProcessor2_0]]

#### diffusers.models.attention_processor.PAGIdentitySelfAttnProcessor2_0[[diffusers.models.attention_processor.PAGIdentitySelfAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5043)

Processor for implementing PAG using scaled dot-product attention (enabled by default if you're using PyTorch 2.0).
PAG reference: https://huggingface.co/papers/2403.17377

#### diffusers.models.attention_processor.PAGCFGIdentitySelfAttnProcessor2_0[[diffusers.models.attention_processor.PAGCFGIdentitySelfAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5142)

Processor for implementing PAG using scaled dot-product attention (enabled by default if you're using PyTorch 2.0).
PAG reference: https://huggingface.co/papers/2403.17377

## IP-Adapter[[diffusers.models.attention_processor.IPAdapterAttnProcessor]]

#### diffusers.models.attention_processor.IPAdapterAttnProcessor[[diffusers.models.attention_processor.IPAdapterAttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L4208)

Attention processor for Multiple IP-Adapters.

**Parameters:**

hidden_size (`int`) : The hidden size of the attention layer.

cross_attention_dim (`int`) : The number of channels in the `encoder_hidden_states`.

num_tokens (`int`, `tuple[int]` or `list[int]`, defaults to `(4,)`) : The context length of the image features.

scale (`float` or list`float`, defaults to 1.0) : the weight scale of image prompt.

#### diffusers.models.attention_processor.IPAdapterAttnProcessor2_0[[diffusers.models.attention_processor.IPAdapterAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L4408)

Attention processor for IP-Adapter for PyTorch 2.0.

**Parameters:**

hidden_size (`int`) : The hidden size of the attention layer.

cross_attention_dim (`int`) : The number of channels in the `encoder_hidden_states`.

num_tokens (`int`, `tuple[int]` or `list[int]`, defaults to `(4,)`) : The context length of the image features.

scale (`float` or `list[float]`, defaults to 1.0) : the weight scale of image prompt.

#### diffusers.models.attention_processor.SD3IPAdapterJointAttnProcessor2_0[[diffusers.models.attention_processor.SD3IPAdapterJointAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L4872)

Attention processor for IP-Adapter used typically in processing the SD3-like self-attention projections, with
additional image-based information and timestep embeddings.

**Parameters:**

hidden_size (`int`) : The number of hidden channels.

ip_hidden_states_dim (`int`) : The image feature dimension.

head_dim (`int`) : The number of head channels.

timesteps_emb_dim (`int`, defaults to 1280) : The number of input channels for timestep embedding.

scale (`float`, defaults to 0.5) : IP-Adapter scale.

## JointAttnProcessor2_0[[diffusers.models.attention_processor.JointAttnProcessor2_0]]

#### diffusers.models.attention_processor.JointAttnProcessor2_0[[diffusers.models.attention_processor.JointAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L1422)

Attention processor used typically in processing the SD3-like self-attention projections.

#### diffusers.models.attention_processor.PAGJointAttnProcessor2_0[[diffusers.models.attention_processor.PAGJointAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L1508)

Attention processor used typically in processing the SD3-like self-attention projections.

#### diffusers.models.attention_processor.PAGCFGJointAttnProcessor2_0[[diffusers.models.attention_processor.PAGCFGJointAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L1664)

Attention processor used typically in processing the SD3-like self-attention projections.

#### diffusers.models.attention_processor.FusedJointAttnProcessor2_0[[diffusers.models.attention_processor.FusedJointAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L1829)

Attention processor used typically in processing the SD3-like self-attention projections.

## LoRA[[diffusers.models.attention_processor.LoRAAttnProcessor]]

#### diffusers.models.attention_processor.LoRAAttnProcessor[[diffusers.models.attention_processor.LoRAAttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5305)

Processor for implementing attention with LoRA.

#### diffusers.models.attention_processor.LoRAAttnProcessor2_0[[diffusers.models.attention_processor.LoRAAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5314)

Processor for implementing attention with LoRA (enabled by default if you're using PyTorch 2.0).

#### diffusers.models.attention_processor.LoRAAttnAddedKVProcessor[[diffusers.models.attention_processor.LoRAAttnAddedKVProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5332)

Processor for implementing attention with LoRA with extra learnable key and value matrices for the text encoder.

#### diffusers.models.attention_processor.LoRAXFormersAttnProcessor[[diffusers.models.attention_processor.LoRAXFormersAttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5323)

Processor for implementing attention with LoRA using xFormers.

## Lumina-T2X[[diffusers.models.attention_processor.LuminaAttnProcessor2_0]]

#### diffusers.models.attention_processor.LuminaAttnProcessor2_0[[diffusers.models.attention_processor.LuminaAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L3572)

Processor for implementing scaled dot-product attention (enabled by default if you're using PyTorch 2.0). This is
used in the LuminaNextDiT model. It applies a s normalization layer and rotary embedding on query and key vector.

## Mochi[[diffusers.models.attention_processor.MochiAttnProcessor2_0]]

#### diffusers.models.attention_processor.MochiAttnProcessor2_0[[diffusers.models.attention_processor.MochiAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L998)

Attention processor used in Mochi.

#### diffusers.models.attention_processor.MochiVaeAttnProcessor2_0[[diffusers.models.attention_processor.MochiVaeAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2906)

Attention processor used in Mochi VAE.

## Sana[[diffusers.models.attention_processor.SanaLinearAttnProcessor2_0]]

#### diffusers.models.attention_processor.SanaLinearAttnProcessor2_0[[diffusers.models.attention_processor.SanaLinearAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5341)

Processor for implementing scaled dot-product linear attention.

#### diffusers.models.attention_processor.SanaMultiscaleAttnProcessor2_0[[diffusers.models.attention_processor.SanaMultiscaleAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5245)

Processor for implementing multiscale quadratic attention.

#### diffusers.models.attention_processor.PAGCFGSanaLinearAttnProcessor2_0[[diffusers.models.attention_processor.PAGCFGSanaLinearAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5393)

Processor for implementing scaled dot-product linear attention.

#### diffusers.models.attention_processor.PAGIdentitySanaLinearAttnProcessor2_0[[diffusers.models.attention_processor.PAGIdentitySanaLinearAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5448)

Processor for implementing scaled dot-product linear attention.

## Stable Audio[[diffusers.models.attention_processor.StableAudioAttnProcessor2_0]]

#### diffusers.models.attention_processor.StableAudioAttnProcessor2_0[[diffusers.models.attention_processor.StableAudioAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2991)

Processor for implementing scaled dot-product attention (enabled by default if you're using PyTorch 2.0). This is
used in the Stable Audio model. It applies rotary embedding on query and key vector, and allows MHA, GQA or MQA.

## SlicedAttnProcessor[[diffusers.models.attention_processor.SlicedAttnProcessor]]

#### diffusers.models.attention_processor.SlicedAttnProcessor[[diffusers.models.attention_processor.SlicedAttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L4000)

Processor for implementing sliced attention.

**Parameters:**

slice_size (`int`, *optional*) : The number of steps to compute attention. Uses as many slices as `attention_head_dim // slice_size`, and `attention_head_dim` must be a multiple of the `slice_size`.

#### diffusers.models.attention_processor.SlicedAttnAddedKVProcessor[[diffusers.models.attention_processor.SlicedAttnAddedKVProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L4087)

Processor for implementing sliced attention with extra learnable key and value matrices for the text encoder.

**Parameters:**

slice_size (`int`, *optional*) : The number of steps to compute attention. Uses as many slices as `attention_head_dim // slice_size`, and `attention_head_dim` must be a multiple of the `slice_size`.

## XFormersAttnProcessor[[diffusers.models.attention_processor.XFormersAttnProcessor]]

#### diffusers.models.attention_processor.XFormersAttnProcessor[[diffusers.models.attention_processor.XFormersAttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2488)

Processor for implementing memory efficient attention using xFormers.

**Parameters:**

attention_op (`Callable`, *optional*, defaults to `None`) : The base [operator](https://facebookresearch.github.io/xformers/components/ops.html#xformers.ops.AttentionOpBase) to use as the attention operator. It is recommended to set to `None`, and allow xFormers to choose the best operator.

#### diffusers.models.attention_processor.XFormersAttnAddedKVProcessor[[diffusers.models.attention_processor.XFormersAttnAddedKVProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2417)

Processor for implementing memory efficient attention using xFormers.

**Parameters:**

attention_op (`Callable`, *optional*, defaults to `None`) : The base [operator](https://facebookresearch.github.io/xformers/components/ops.html#xformers.ops.AttentionOpBase) to use as the attention operator. It is recommended to set to `None`, and allow xFormers to choose the best operator.

## XLAFlashAttnProcessor2_0[[diffusers.models.attention_processor.XLAFlashAttnProcessor2_0]]

#### diffusers.models.attention_processor.XLAFlashAttnProcessor2_0[[diffusers.models.attention_processor.XLAFlashAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L2790)

Processor for implementing scaled dot-product attention with pallas flash attention kernel if using `torch_xla`.

## XFormersJointAttnProcessor[[diffusers.models.attention_processor.XFormersJointAttnProcessor]]

#### diffusers.models.attention_processor.XFormersJointAttnProcessor[[diffusers.models.attention_processor.XFormersJointAttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L1908)

Processor for implementing memory efficient attention using xFormers.

**Parameters:**

attention_op (`Callable`, *optional*, defaults to `None`) : The base [operator](https://facebookresearch.github.io/xformers/components/ops.html#xformers.ops.AttentionOpBase) to use as the attention operator. It is recommended to set to `None`, and allow xFormers to choose the best operator.

## IPAdapterXFormersAttnProcessor[[diffusers.models.attention_processor.IPAdapterXFormersAttnProcessor]]

#### diffusers.models.attention_processor.IPAdapterXFormersAttnProcessor[[diffusers.models.attention_processor.IPAdapterXFormersAttnProcessor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L4640)

Attention processor for IP-Adapter using xFormers.

**Parameters:**

hidden_size (`int`) : The hidden size of the attention layer.

cross_attention_dim (`int`) : The number of channels in the `encoder_hidden_states`.

num_tokens (`int`, `tuple[int]` or `list[int]`, defaults to `(4,)`) : The context length of the image features.

scale (`float` or `list[float]`, defaults to 1.0) : the weight scale of image prompt.

attention_op (`Callable`, *optional*, defaults to `None`) : The base [operator](https://facebookresearch.github.io/xformers/components/ops.html#xformers.ops.AttentionOpBase) to use as the attention operator. It is recommended to set to `None`, and allow xFormers to choose the best operator.

## FluxIPAdapterJointAttnProcessor2_0[[diffusers.models.attention_processor.FluxIPAdapterJointAttnProcessor2_0]]

#### diffusers.models.attention_processor.FluxIPAdapterJointAttnProcessor2_0[[diffusers.models.attention_processor.FluxIPAdapterJointAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5539)

## XLAFluxFlashAttnProcessor2_0[[diffusers.models.attention_processor.XLAFluxFlashAttnProcessor2_0]]

#### diffusers.models.attention_processor.XLAFluxFlashAttnProcessor2_0[[diffusers.models.attention_processor.XLAFluxFlashAttnProcessor2_0]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/attention_processor.py#L5579)

Processor for implementing scaled dot-product attention with pallas flash attention kernel if using `torch_xla`.
