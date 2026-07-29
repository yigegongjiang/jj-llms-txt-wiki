# SanaTransformer2DModel

A Diffusion Transformer model for 2D data from [SANA: Efficient High-Resolution Image Synthesis with Linear Diffusion Transformers](https://huggingface.co/papers/2410.10629) was introduced from NVIDIA and MIT HAN Lab, by Enze Xie, Junsong Chen, Junyu Chen, Han Cai, Haotian Tang, Yujun Lin, Zhekai Zhang, Muyang Li, Ligeng Zhu, Yao Lu, Song Han.

The abstract from the paper is:

*We introduce Sana, a text-to-image framework that can efficiently generate images up to 4096×4096 resolution. Sana can synthesize high-resolution, high-quality images with strong text-image alignment at a remarkably fast speed, deployable on laptop GPU. Core designs include: (1) Deep compression autoencoder: unlike traditional AEs, which compress images only 8×, we trained an AE that can compress images 32×, effectively reducing the number of latent tokens. (2) Linear DiT: we replace all vanilla attention in DiT with linear attention, which is more efficient at high resolutions without sacrificing quality. (3) Decoder-only text encoder: we replaced T5 with modern decoder-only small LLM as the text encoder and designed complex human instruction with in-context learning to enhance the image-text alignment. (4) Efficient training and sampling: we propose Flow-DPM-Solver to reduce sampling steps, with efficient caption labeling and selection to accelerate convergence. As a result, Sana-0.6B is very competitive with modern giant diffusion model (e.g. Flux-12B), being 20 times smaller and 100+ times faster in measured throughput. Moreover, Sana-0.6B can be deployed on a 16GB laptop GPU, taking less than 1 second to generate a 1024×1024 resolution image. Sana enables content creation at low cost. Code and model will be publicly released.*

The model can be loaded with the following code snippet.

```python
from diffusers import SanaTransformer2DModel

transformer = SanaTransformer2DModel.from_pretrained("Efficient-Large-Model/Sana_1600M_1024px_BF16_diffusers", subfolder="transformer", torch_dtype=torch.bfloat16)
```

## SanaTransformer2DModel[[diffusers.SanaTransformer2DModel]]

#### diffusers.SanaTransformer2DModel[[diffusers.SanaTransformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/sana_transformer.py#L292)

A 2D Transformer model introduced in [Sana](https://huggingface.co/papers/2410.10629) family of models.

forwarddiffusers.SanaTransformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/sana_transformer.py#L417[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": Tensor"}, {"name": "guidance", "val": ": torch.Tensor | None = None"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_block_samples", "val": ": tuple[torch.Tensor] | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, in_channels, height, width)`) --
  Input `hidden_states`.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **guidance** (`torch.Tensor`, *optional*) --
  Guidance scale embedding.
- **encoder_attention_mask** (`torch.Tensor`, *optional*) --
  Cross-attention mask applied to `encoder_hidden_states`.
- **attention_mask** (`torch.Tensor`, *optional*) --
  Self-attention mask applied to `hidden_states`.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **controlnet_block_samples** (`tuple` of `torch.Tensor`, *optional*) --
  A list of tensors that if specified are added to the residuals of transformer blocks.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [SanaTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/sana_transformer2d#diffusers.SanaTransformer2DModel) forward method.

**Parameters:**

in_channels (`int`, defaults to `32`) : The number of channels in the input.

out_channels (`int`, *optional*, defaults to `32`) : The number of channels in the output.

num_attention_heads (`int`, defaults to `70`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `32`) : The number of channels in each head.

num_layers (`int`, defaults to `20`) : The number of layers of Transformer blocks to use.

num_cross_attention_heads (`int`, *optional*, defaults to `20`) : The number of heads to use for cross-attention.

cross_attention_head_dim (`int`, *optional*, defaults to `112`) : The number of channels in each head for cross-attention.

cross_attention_dim (`int`, *optional*, defaults to `2240`) : The number of channels in the cross-attention output.

caption_channels (`int`, defaults to `2304`) : The number of channels in the caption embeddings.

mlp_ratio (`float`, defaults to `2.5`) : The expansion ratio to use in the GLUMBConv layer.

dropout (`float`, defaults to `0.0`) : The dropout probability.

attention_bias (`bool`, defaults to `False`) : Whether to use bias in the attention layer.

sample_size (`int`, defaults to `32`) : The base size of the input latent.

patch_size (`int`, defaults to `1`) : The size of the patches to use in the patch embedding layer.

norm_elementwise_affine (`bool`, defaults to `False`) : Whether to use elementwise affinity in the normalization layer.

norm_eps (`float`, defaults to `1e-6`) : The epsilon value for the normalization layer.

qk_norm (`str`, *optional*, defaults to `None`) : The normalization to use for the query and key.

timestep_scale (`float`, defaults to `1.0`) : The scale to use for the timesteps.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
