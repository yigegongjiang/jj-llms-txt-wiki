# UVit2DModel

The [U-ViT](https://hf.co/papers/2301.11093) model is a vision transformer (ViT) based UNet. This model incorporates elements from ViT (considers all inputs such as time, conditions and noisy image patches as tokens) and a UNet (long skip connections between the shallow and deep layers). The skip connection is important for predicting pixel-level features. An additional 3x3 convolutional block is applied prior to the final output to improve image quality.

The abstract from the paper is:

*Currently, applying diffusion models in pixel space of high resolution images is difficult. Instead, existing approaches focus on diffusion in lower dimensional spaces (latent diffusion), or have multiple super-resolution levels of generation referred to as cascades. The downside is that these approaches add additional complexity to the diffusion framework. This paper aims to improve denoising diffusion for high resolution images while keeping the model as simple as possible. The paper is centered around the research question: How can one train a standard denoising diffusion models on high resolution images, and still obtain performance comparable to these alternate approaches? The four main findings are: 1) the noise schedule should be adjusted for high resolution images, 2) It is sufficient to scale only a particular part of the architecture, 3) dropout should be added at specific locations in the architecture, and 4) downsampling is an effective strategy to avoid high resolution feature maps. Combining these simple yet effective techniques, we achieve state-of-the-art on image generation among diffusion models without sampling modifiers on ImageNet.*

## UVit2DModel[[diffusers.UVit2DModel]]

#### diffusers.UVit2DModel[[diffusers.UVit2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/uvit_2d.py#L38)

forwarddiffusers.UVit2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/uvit_2d.py#L150[{"name": "input_ids", "val": ""}, {"name": "encoder_hidden_states", "val": ""}, {"name": "pooled_text_emb", "val": ""}, {"name": "micro_conds", "val": ""}, {"name": "cross_attention_kwargs", "val": " = None"}]- **input_ids** (`torch.LongTensor`) --
  Token ids of the masked latent image tokens, with shape `(batch_size, height, width)`.
- **encoder_hidden_states** (`torch.Tensor`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **pooled_text_emb** (`torch.Tensor`) --
  Pooled text embeddings used for additional conditioning.
- **micro_conds** (`torch.Tensor`) --
  Micro-conditioning values that are embedded and combined with `pooled_text_emb`.
- **cross_attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor`.0

**Parameters:**

input_ids (`torch.LongTensor`) : Token ids of the masked latent image tokens, with shape `(batch_size, height, width)`.

encoder_hidden_states (`torch.Tensor`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

pooled_text_emb (`torch.Tensor`) : Pooled text embeddings used for additional conditioning.

micro_conds (`torch.Tensor`) : Micro-conditioning values that are embedded and combined with `pooled_text_emb`.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor`.
#### set_default_attn_processor[[diffusers.UVit2DModel.set_default_attn_processor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/uvit_2d.py#L226)

Disables custom attention processors and sets the default attention implementation.

## UVit2DConvEmbed[[diffusers.models.unets.uvit_2d.UVit2DConvEmbed]]

#### diffusers.models.unets.uvit_2d.UVit2DConvEmbed[[diffusers.models.unets.uvit_2d.UVit2DConvEmbed]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/uvit_2d.py#L242)

## UVitBlock[[diffusers.models.unets.uvit_2d.UVitBlock]]

#### diffusers.models.unets.uvit_2d.UVitBlock[[diffusers.models.unets.uvit_2d.UVitBlock]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/uvit_2d.py#L257)

## ConvNextBlock[[diffusers.models.unets.uvit_2d.ConvNextBlock]]

#### diffusers.models.unets.uvit_2d.ConvNextBlock[[diffusers.models.unets.uvit_2d.ConvNextBlock]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/uvit_2d.py#L356)

## ConvMlmLayer[[diffusers.models.unets.uvit_2d.ConvMlmLayer]]

#### diffusers.models.unets.uvit_2d.ConvMlmLayer[[diffusers.models.unets.uvit_2d.ConvMlmLayer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/unets/uvit_2d.py#L401)
