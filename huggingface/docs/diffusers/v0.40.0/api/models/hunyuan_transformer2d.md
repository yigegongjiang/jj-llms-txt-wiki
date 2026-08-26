# HunyuanDiT2DModel

A Diffusion Transformer model for 2D data from [Hunyuan-DiT](https://github.com/Tencent/HunyuanDiT).

## HunyuanDiT2DModel[[diffusers.HunyuanDiT2DModel]]

#### diffusers.HunyuanDiT2DModel[[diffusers.HunyuanDiT2DModel]]

```python
diffusers.HunyuanDiT2DModel(num_attention_heads: int = 16, attention_head_dim: int = 88, in_channels: int | None = None, patch_size: int | None = None, activation_fn: str = 'gelu-approximate', sample_size = 32, hidden_size = 1152, num_layers: int = 28, mlp_ratio: float = 4.0, learn_sigma: bool = True, cross_attention_dim: int = 1024, norm_type: str = 'layer_norm', cross_attention_dim_t5: int = 2048, pooled_projection_dim: int = 1024, text_len: int = 77, text_len_t5: int = 256, use_style_cond_and_image_meta_size: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/hunyuan_transformer_2d.py#L201)

**Parameters:**

num_attention_heads (`int`, *optional*, defaults to 16) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, *optional*, defaults to 88) : The number of channels in each head.

in_channels (`int`, *optional*) : The number of channels in the input and output (specify if the input is **continuous**).

patch_size (`int`, *optional*) : The size of the patch to use for the input.

activation_fn (`str`, *optional*, defaults to `"geglu"`) : Activation function to use in feed-forward.

sample_size (`int`, *optional*) : The width of the latent images. This is fixed during training since it is used to learn a number of position embeddings.

dropout (`float`, *optional*, defaults to 0.0) : The dropout probability to use.

cross_attention_dim (`int`, *optional*) : The number of dimension in the clip text embedding.

hidden_size (`int`, *optional*) : The size of hidden layer in the conditioning embedding layers.

num_layers (`int`, *optional*, defaults to 1) : The number of layers of Transformer blocks to use.

mlp_ratio (`float`, *optional*, defaults to 4.0) : The ratio of the hidden layer size to the input size.

learn_sigma (`bool`, *optional*, defaults to `True`) : Whether to predict variance.

cross_attention_dim_t5 (`int`, *optional*) : The number dimensions in t5 text embedding.

pooled_projection_dim (`int`, *optional*) : The size of the pooled projection.

text_len (`int`, *optional*) : The length of the clip text embedding.

text_len_t5 (`int`, *optional*) : The length of the T5 text embedding.

use_style_cond_and_image_meta_size (`bool`,  *optional*) : Whether or not to use style condition and image meta size. True for version <=1.1, False for version >= 1.2

HunYuanDiT: Diffusion model with a Transformer backbone.

Inherit ModelMixin and ConfigMixin to be compatible with the sampler StableDiffusionPipeline of diffusers.

#### enable_forward_chunking[[diffusers.HunyuanDiT2DModel.enable_forward_chunking]]

```python
enable_forward_chunking(chunk_size: int | None = None, dim: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/hunyuan_transformer_2d.py#L472)

**Parameters:**

chunk_size (`int`, *optional*) : The chunk size of the feed-forward layers. If not specified, will run feed-forward layer individually over each tensor of dim=`dim`.

dim (`int`, *optional*, defaults to `0`) : The dimension over which the feed-forward computation should be chunked. Choose between dim=0 (batch) or dim=1 (sequence length).

Sets the attention processor to use [feed forward
chunking](https://huggingface.co/blog/reformer#2-chunked-feed-forward-layers).

#### forward[[diffusers.HunyuanDiT2DModel.forward]]

```python
forward(hidden_states, timestep, encoder_hidden_states = None, text_embedding_mask = None, encoder_hidden_states_t5 = None, text_embedding_mask_t5 = None, image_meta_size = None, style = None, image_rotary_emb = None, controlnet_block_samples = None, return_dict = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/hunyuan_transformer_2d.py#L357)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch size, dim, height, width)`) : The input tensor.

timestep ( `torch.LongTensor`, *optional*) : Used to indicate denoising step.

encoder_hidden_states ( `torch.Tensor` of shape `(batch size, sequence len, embed dims)`, *optional*) : Conditional embeddings for cross attention layer. This is the output of `BertModel`.

text_embedding_mask : torch.Tensor An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. This is the output of `BertModel`.

encoder_hidden_states_t5 ( `torch.Tensor` of shape `(batch size, sequence len, embed dims)`, *optional*) : Conditional embeddings for cross attention layer. This is the output of T5 Text Encoder.

text_embedding_mask_t5 : torch.Tensor An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. This is the output of T5 Text Encoder.

image_meta_size (torch.Tensor) : Conditional embedding indicate the image sizes

style : torch.Tensor: Conditional embedding indicate the style

image_rotary_emb (`torch.Tensor`) : The image rotary embeddings to apply on query and key tensors during attention calculation.

controlnet_block_samples (`list` of `torch.Tensor`, *optional*) : A list of tensors that if specified are added to the residuals of transformer blocks.

return_dict : bool Whether to return a dictionary.

The [HunyuanDiT2DModel](/docs/diffusers/v0.40.0/en/api/models/hunyuan_transformer2d#diffusers.HunyuanDiT2DModel) forward method.

#### fuse_qkv_projections[[diffusers.HunyuanDiT2DModel.fuse_qkv_projections]]

```python
fuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/hunyuan_transformer_2d.py#L320)

Enables fused QKV projections. For self-attention modules, all projection matrices (i.e., query, key, value)
are fused. For cross-attention modules, key and value projection matrices are fused.

> [!WARNING] > This API is 🧪 experimental.

#### set_default_attn_processor[[diffusers.HunyuanDiT2DModel.set_default_attn_processor]]

```python
set_default_attn_processor()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/hunyuan_transformer_2d.py#L351)

Disables custom attention processors and sets the default attention implementation.

#### unfuse_qkv_projections[[diffusers.HunyuanDiT2DModel.unfuse_qkv_projections]]

```python
unfuse_qkv_projections()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/hunyuan_transformer_2d.py#L342)

Disables the fused QKV projection if enabled.

> [!WARNING] > This API is 🧪 experimental.
