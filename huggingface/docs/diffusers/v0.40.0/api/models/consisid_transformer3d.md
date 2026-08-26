# ConsisIDTransformer3DModel

A Diffusion Transformer model for 3D data from [ConsisID](https://github.com/PKU-YuanGroup/ConsisID) was introduced in [Identity-Preserving Text-to-Video Generation by Frequency Decomposition](https://huggingface.co/papers/2411.17440) by Peking University & University of Rochester & etc.

The model can be loaded with the following code snippet.

```python
from diffusers import ConsisIDTransformer3DModel

transformer = ConsisIDTransformer3DModel.from_pretrained("BestWishYsh/ConsisID-preview", subfolder="transformer", dtype=torch.bfloat16).to("cuda")
```

## ConsisIDTransformer3DModel[[diffusers.ConsisIDTransformer3DModel]]

#### diffusers.ConsisIDTransformer3DModel[[diffusers.ConsisIDTransformer3DModel]]

```python
diffusers.ConsisIDTransformer3DModel(num_attention_heads: int = 30, attention_head_dim: int = 64, in_channels: int = 16, out_channels: int | None = 16, flip_sin_to_cos: bool = True, freq_shift: int = 0, time_embed_dim: int = 512, text_embed_dim: int = 4096, num_layers: int = 30, dropout: float = 0.0, attention_bias: bool = True, sample_width: int = 90, sample_height: int = 60, sample_frames: int = 49, patch_size: int = 2, temporal_compression_ratio: int = 4, max_text_seq_length: int = 226, activation_fn: str = 'gelu-approximate', timestep_activation_fn: str = 'silu', norm_elementwise_affine: bool = True, norm_eps: float = 1e-05, spatial_interpolation_scale: float = 1.875, temporal_interpolation_scale: float = 1.0, use_rotary_positional_embeddings: bool = False, use_learned_positional_embeddings: bool = False, is_train_face: bool = False, is_kps: bool = False, cross_attn_interval: int = 2, cross_attn_dim_head: int = 128, cross_attn_num_heads: int = 16, LFE_id_dim: int = 1280, LFE_vit_dim: int = 1024, LFE_depth: int = 10, LFE_dim_head: int = 64, LFE_num_heads: int = 16, LFE_num_id_token: int = 5, LFE_num_querie: int = 32, LFE_output_dim: int = 2048, LFE_ff_mult: int = 4, LFE_num_scale: int = 5, local_face_scale: float = 1.0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/consisid_transformer_3d.py#L351)

**Parameters:**

num_attention_heads (`int`, defaults to `30`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `64`) : The number of channels in each head.

in_channels (`int`, defaults to `16`) : The number of channels in the input.

out_channels (`int`, *optional*, defaults to `16`) : The number of channels in the output.

flip_sin_to_cos (`bool`, defaults to `True`) : Whether to flip the sin to cos in the time embedding.

time_embed_dim (`int`, defaults to `512`) : Output dimension of timestep embeddings.

text_embed_dim (`int`, defaults to `4096`) : Input dimension of text embeddings from the text encoder.

num_layers (`int`, defaults to `30`) : The number of layers of Transformer blocks to use.

dropout (`float`, defaults to `0.0`) : The dropout probability to use.

attention_bias (`bool`, defaults to `True`) : Whether to use bias in the attention projection layers.

sample_width (`int`, defaults to `90`) : The width of the input latents.

sample_height (`int`, defaults to `60`) : The height of the input latents.

sample_frames (`int`, defaults to `49`) : The number of frames in the input latents. Note that this parameter was incorrectly initialized to 49 instead of 13 because ConsisID processed 13 latent frames at once in its default and recommended settings, but cannot be changed to the correct value to ensure backwards compatibility. To create a transformer with K latent frames, the correct value to pass here would be: ((K - 1) * temporal_compression_ratio + 1).

patch_size (`int`, defaults to `2`) : The size of the patches to use in the patch embedding layer.

temporal_compression_ratio (`int`, defaults to `4`) : The compression ratio across the temporal dimension. See documentation for `sample_frames`.

max_text_seq_length (`int`, defaults to `226`) : The maximum sequence length of the input text embeddings.

activation_fn (`str`, defaults to `"gelu-approximate"`) : Activation function to use in feed-forward.

timestep_activation_fn (`str`, defaults to `"silu"`) : Activation function to use when generating the timestep embeddings.

norm_elementwise_affine (`bool`, defaults to `True`) : Whether to use elementwise affine in normalization layers.

norm_eps (`float`, defaults to `1e-5`) : The epsilon value to use in normalization layers.

spatial_interpolation_scale (`float`, defaults to `1.875`) : Scaling factor to apply in 3D positional embeddings across spatial dimensions.

temporal_interpolation_scale (`float`, defaults to `1.0`) : Scaling factor to apply in 3D positional embeddings across temporal dimensions.

is_train_face (`bool`, defaults to `False`) : Whether to use enable the identity-preserving module during the training process. When set to `True`, the model will focus on identity-preserving tasks.

is_kps (`bool`, defaults to `False`) : Whether to enable keypoint for global facial extractor. If `True`, keypoints will be in the model.

cross_attn_interval (`int`, defaults to `2`) : The interval between cross-attention layers in the Transformer architecture. A larger value may reduce the frequency of cross-attention computations, which can help reduce computational overhead.

cross_attn_dim_head (`int`, optional, defaults to `128`) : The dimensionality of each attention head in the cross-attention layers of the Transformer architecture. A larger value increases the capacity to attend to more complex patterns, but also increases memory and computation costs.

cross_attn_num_heads (`int`, optional, defaults to `16`) : The number of attention heads in the cross-attention layers. More heads allow for more parallel attention mechanisms, capturing diverse relationships between different components of the input, but can also increase computational requirements.

LFE_id_dim (`int`, optional, defaults to `1280`) : The dimensionality of the identity vector used in the Local Facial Extractor (LFE). This vector represents the identity features of a face, which are important for tasks like face recognition and identity preservation across different frames.

LFE_vit_dim (`int`, optional, defaults to `1024`) : The dimension of the vision transformer (ViT) output used in the Local Facial Extractor (LFE). This value dictates the size of the transformer-generated feature vectors that will be processed for facial feature extraction.

LFE_depth (`int`, optional, defaults to `10`) : The number of layers in the Local Facial Extractor (LFE). Increasing the depth allows the model to capture more complex representations of facial features, but also increases the computational load.

LFE_dim_head (`int`, optional, defaults to `64`) : The dimensionality of each attention head in the Local Facial Extractor (LFE). This parameter affects how finely the model can process and focus on different parts of the facial features during the extraction process.

LFE_num_heads (`int`, optional, defaults to `16`) : The number of attention heads in the Local Facial Extractor (LFE). More heads can improve the model's ability to capture diverse facial features, but at the cost of increased computational complexity.

LFE_num_id_token (`int`, optional, defaults to `5`) : The number of identity tokens used in the Local Facial Extractor (LFE). This defines how many identity-related tokens the model will process to ensure face identity preservation during feature extraction.

LFE_num_querie (`int`, optional, defaults to `32`) : The number of query tokens used in the Local Facial Extractor (LFE). These tokens are used to capture high-frequency face-related information that aids in accurate facial feature extraction.

LFE_output_dim (`int`, optional, defaults to `2048`) : The output dimension of the Local Facial Extractor (LFE). This dimension determines the size of the feature vectors produced by the LFE module, which will be used for subsequent tasks such as face recognition or tracking.

LFE_ff_mult (`int`, optional, defaults to `4`) : The multiplication factor applied to the feed-forward network's hidden layer size in the Local Facial Extractor (LFE). A higher value increases the model's capacity to learn more complex facial feature transformations, but also increases the computation and memory requirements.

LFE_num_scale (`int`, optional, defaults to `5`) : The number of different scales visual feature. A higher value increases the model's capacity to learn more complex facial feature transformations, but also increases the computation and memory requirements.

local_face_scale (`float`, defaults to `1.0`) : A scaling factor used to adjust the importance of local facial features in the model. This can influence how strongly the model focuses on high frequency face-related content.

A Transformer model for video-like data in [ConsisID](https://github.com/PKU-YuanGroup/ConsisID).

#### forward[[diffusers.ConsisIDTransformer3DModel.forward]]

```python
forward(hidden_states: Tensor, encoder_hidden_states: Tensor, timestep: typing.Union[int, float, torch.LongTensor], timestep_cond: typing.Optional[torch.Tensor] = None, image_rotary_emb: tuple[torch.Tensor, torch.Tensor] | None = None, attention_kwargs: dict[str, typing.Any] | None = None, id_cond: typing.Optional[torch.Tensor] = None, id_vit_hidden: typing.Optional[torch.Tensor] = None, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/consisid_transformer_3d.py#L623)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, num_frames, channels, height, width)`) : Input `hidden_states`.

encoder_hidden_states (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) : Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.

timestep (`torch.LongTensor`) : Used to indicate denoising step.

timestep_cond (`torch.Tensor`, *optional*) : Conditional embeddings for timestep. If provided, the embeddings will be summed with the samples passed through the `self.time_embedding` layer to obtain the final timestep embeddings.

image_rotary_emb (`tuple` of `torch.Tensor`, *optional*) : Pre-computed rotary positional embeddings.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

id_cond (`torch.Tensor`, *optional*) : The face embedding extracted by the local facial extractor used for identity conditioning.

id_vit_hidden (`torch.Tensor`, *optional*) : The ViT hidden states extracted from face images used for identity conditioning.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [ConsisIDTransformer3DModel](/docs/diffusers/v0.40.0/en/api/models/consisid_transformer3d#diffusers.ConsisIDTransformer3DModel) forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
