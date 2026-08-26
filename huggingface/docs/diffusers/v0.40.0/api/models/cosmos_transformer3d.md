# CosmosTransformer3DModel

A Diffusion Transformer model for 3D video-like data was introduced in [Cosmos World Foundation Model Platform for Physical AI](https://huggingface.co/papers/2501.03575) by NVIDIA.

The model can be loaded with the following code snippet.

```python
from diffusers import CosmosTransformer3DModel

transformer = CosmosTransformer3DModel.from_pretrained("nvidia/Cosmos-1.0-Diffusion-7B-Text2World", subfolder="transformer", dtype=torch.bfloat16)
```

## CosmosTransformer3DModel[[diffusers.CosmosTransformer3DModel]]

#### diffusers.CosmosTransformer3DModel[[diffusers.CosmosTransformer3DModel]]

```python
diffusers.CosmosTransformer3DModel(in_channels: int = 16, out_channels: int = 16, num_attention_heads: int = 32, attention_head_dim: int = 128, num_layers: int = 28, mlp_ratio: float = 4.0, text_embed_dim: int = 1024, adaln_lora_dim: int = 256, max_size: tuple = (128, 240, 240), patch_size: tuple = (1, 2, 2), rope_scale: tuple = (2.0, 1.0, 1.0), concat_padding_mask: bool = True, extra_pos_embed_type: str | None = 'learnable', use_crossattn_projection: bool = False, crossattn_proj_in_channels: int = 1024, encoder_hidden_states_channels: int = 1024, controlnet_block_every_n: int | None = None, img_context_dim_in: int | None = None, img_context_num_tokens: int = 256, img_context_dim_out: int = 2048)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_cosmos.py#L554)

**Parameters:**

in_channels (`int`, defaults to `16`) : The number of channels in the input.

out_channels (`int`, defaults to `16`) : The number of channels in the output.

num_attention_heads (`int`, defaults to `32`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `128`) : The number of channels in each attention head.

num_layers (`int`, defaults to `28`) : The number of layers of transformer blocks to use.

mlp_ratio (`float`, defaults to `4.0`) : The ratio of the hidden layer size to the input size in the feedforward network.

text_embed_dim (`int`, defaults to `4096`) : Input dimension of text embeddings from the text encoder.

adaln_lora_dim (`int`, defaults to `256`) : The hidden dimension of the Adaptive LayerNorm LoRA layer.

max_size (`tuple[int, int, int]`, defaults to `(128, 240, 240)`) : The maximum size of the input latent tensors in the temporal, height, and width dimensions.

patch_size (`tuple[int, int, int]`, defaults to `(1, 2, 2)`) : The patch size to use for patchifying the input latent tensors in the temporal, height, and width dimensions.

rope_scale (`tuple[float, float, float]`, defaults to `(2.0, 1.0, 1.0)`) : The scaling factor to use for RoPE in the temporal, height, and width dimensions.

concat_padding_mask (`bool`, defaults to `True`) : Whether to concatenate the padding mask to the input latent tensors.

extra_pos_embed_type (`str`, *optional*, defaults to `learnable`) : The type of extra positional embeddings to use. Can be one of `None` or `learnable`.

controlnet_block_every_n (`int`, *optional*) : Interval between transformer blocks that should receive control residuals (for example, `7` to inject after every seventh block). Required for Cosmos Transfer2.5.

img_context_dim_in (`int`, *optional*) : The dimension of the input image context feature vector, i.e. it is the D in [B, N, D].

img_context_num_tokens (`int`) : The number of tokens in the image context feature vector, i.e. it is the N in [B, N, D]. If `img_context_dim_in` is not provided, then this parameter is ignored.

img_context_dim_out (`int`) : The output dimension of the image context projection layer. If `img_context_dim_in` is not provided, then this parameter is ignored.

A Transformer model for video-like data used in [Cosmos](https://github.com/NVIDIA/Cosmos).

#### forward[[diffusers.CosmosTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, timestep: Tensor, encoder_hidden_states: Tensor, block_controlnet_hidden_states: list[torch.Tensor] | None = None, attention_mask: typing.Optional[torch.Tensor] = None, fps: int | None = None, condition_mask: typing.Optional[torch.Tensor] = None, padding_mask: typing.Optional[torch.Tensor] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_cosmos.py#L688)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)`) : Input `hidden_states`.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

block_controlnet_hidden_states (`list` of `torch.Tensor`, *optional*) : A list of tensors that if specified are added to the residuals of transformer blocks.

attention_mask (`torch.Tensor`, *optional*) : Mask applied to `encoder_hidden_states` during attention.

fps (`int`, *optional*) : Frames per second of the input video used to compute the rotary positional embeddings.

condition_mask (`torch.Tensor`, *optional*) : Mask channel concatenated to `hidden_states` to indicate the conditioning region.

padding_mask (`torch.Tensor`, *optional*) : Padding mask concatenated to `hidden_states` when `concat_padding_mask` is enabled.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [CosmosTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/cosmos_transformer3d#diffusers.CosmosTransformer3DModel) forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
