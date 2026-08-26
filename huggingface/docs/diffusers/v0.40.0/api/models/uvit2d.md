# UVit2DModel

The [U-ViT](https://hf.co/papers/2301.11093) model is a vision transformer (ViT) based UNet. This model incorporates elements from ViT (considers all inputs such as time, conditions and noisy image patches as tokens) and a UNet (long skip connections between the shallow and deep layers). The skip connection is important for predicting pixel-level features. An additional 3x3 convolutional block is applied prior to the final output to improve image quality.

The abstract from the paper is:

*Currently, applying diffusion models in pixel space of high resolution images is difficult. Instead, existing approaches focus on diffusion in lower dimensional spaces (latent diffusion), or have multiple super-resolution levels of generation referred to as cascades. The downside is that these approaches add additional complexity to the diffusion framework. This paper aims to improve denoising diffusion for high resolution images while keeping the model as simple as possible. The paper is centered around the research question: How can one train a standard denoising diffusion models on high resolution images, and still obtain performance comparable to these alternate approaches? The four main findings are: 1) the noise schedule should be adjusted for high resolution images, 2) It is sufficient to scale only a particular part of the architecture, 3) dropout should be added at specific locations in the architecture, and 4) downsampling is an effective strategy to avoid high resolution feature maps. Combining these simple yet effective techniques, we achieve state-of-the-art on image generation among diffusion models without sampling modifiers on ImageNet.*

## UVit2DModel[[diffusers.UVit2DModel]]

#### diffusers.UVit2DModel[[diffusers.UVit2DModel]]

```python
diffusers.UVit2DModel(hidden_size: int = 1024, use_bias: bool = False, hidden_dropout: float = 0.0, cond_embed_dim: int = 768, micro_cond_encode_dim: int = 256, micro_cond_embed_dim: int = 1280, encoder_hidden_size: int = 768, vocab_size: int = 8256, codebook_size: int = 8192, in_channels: int = 768, block_out_channels: int = 768, num_res_blocks: int = 3, downsample: bool = False, upsample: bool = False, block_num_heads: int = 12, num_hidden_layers: int = 22, num_attention_heads: int = 16, attention_dropout: float = 0.0, intermediate_size: int = 2816, layer_norm_eps: float = 1e-06, ln_elementwise_affine: bool = True, sample_size: int = 64)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/uvit_2d.py#L38)

#### forward[[diffusers.UVit2DModel.forward]]

```python
forward(input_ids, encoder_hidden_states, pooled_text_emb, micro_conds, cross_attention_kwargs = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/uvit_2d.py#L150)

**Parameters:**

input_ids (`torch.LongTensor`) : Token ids of the masked latent image tokens, with shape `(batch_size, height, width)`.

encoder_hidden_states (`torch.Tensor`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

pooled_text_emb (`torch.Tensor`) : Pooled text embeddings used for additional conditioning.

micro_conds (`torch.Tensor`) : Micro-conditioning values that are embedded and combined with `pooled_text_emb`.

cross_attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor`.

#### set_default_attn_processor[[diffusers.UVit2DModel.set_default_attn_processor]]

```python
set_default_attn_processor()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/uvit_2d.py#L226)

Disables custom attention processors and sets the default attention implementation.

## UVit2DConvEmbed[[diffusers.models.unets.uvit_2d.UVit2DConvEmbed]]

#### diffusers.models.unets.uvit_2d.UVit2DConvEmbed[[diffusers.models.unets.uvit_2d.UVit2DConvEmbed]]

```python
diffusers.models.unets.uvit_2d.UVit2DConvEmbed(in_channels, block_out_channels, vocab_size, elementwise_affine, eps, bias)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/uvit_2d.py#L242)

## UVitBlock[[diffusers.models.unets.uvit_2d.UVitBlock]]

#### diffusers.models.unets.uvit_2d.UVitBlock[[diffusers.models.unets.uvit_2d.UVitBlock]]

```python
diffusers.models.unets.uvit_2d.UVitBlock(channels, num_res_blocks: int, hidden_size, hidden_dropout, ln_elementwise_affine, layer_norm_eps, use_bias, block_num_heads, attention_dropout, downsample: bool, upsample: bool)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/uvit_2d.py#L257)

## ConvNextBlock[[diffusers.models.unets.uvit_2d.ConvNextBlock]]

#### diffusers.models.unets.uvit_2d.ConvNextBlock[[diffusers.models.unets.uvit_2d.ConvNextBlock]]

```python
diffusers.models.unets.uvit_2d.ConvNextBlock(channels, layer_norm_eps, ln_elementwise_affine, use_bias, hidden_dropout, hidden_size, res_ffn_factor = 4)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/uvit_2d.py#L356)

## ConvMlmLayer[[diffusers.models.unets.uvit_2d.ConvMlmLayer]]

#### diffusers.models.unets.uvit_2d.ConvMlmLayer[[diffusers.models.unets.uvit_2d.ConvMlmLayer]]

```python
diffusers.models.unets.uvit_2d.ConvMlmLayer(block_out_channels: int, in_channels: int, use_bias: bool, ln_elementwise_affine: bool, layer_norm_eps: float, codebook_size: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/unets/uvit_2d.py#L401)
