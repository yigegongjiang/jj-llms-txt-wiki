# SanaVideoTransformer3DModel

A Diffusion Transformer model for 3D data (video) from [SANA-Video: Efficient Video Generation with Block Linear Diffusion Transformer](https://huggingface.co/papers/2509.24695) from NVIDIA and MIT HAN Lab, by Junsong Chen, Yuyang Zhao, Jincheng Yu, Ruihang Chu, Junyu Chen, Shuai Yang, Xianbang Wang, Yicheng Pan, Daquan Zhou, Huan Ling, Haozhe Liu, Hongwei Yi, Hao Zhang, Muyang Li, Yukang Chen, Han Cai, Sanja Fidler, Ping Luo, Song Han, Enze Xie.

The abstract from the paper is:

*We introduce SANA-Video, a small diffusion model that can efficiently generate videos up to 720x1280 resolution and minute-length duration. SANA-Video synthesizes high-resolution, high-quality and long videos with strong text-video alignment at a remarkably fast speed, deployable on RTX 5090 GPU. Two core designs ensure our efficient, effective and long video generation: (1) Linear DiT: We leverage linear attention as the core operation, which is more efficient than vanilla attention given the large number of tokens processed in video generation. (2) Constant-Memory KV cache for Block Linear Attention: we design block-wise autoregressive approach for long video generation by employing a constant-memory state, derived from the cumulative properties of linear attention. This KV cache provides the Linear DiT with global context at a fixed memory cost, eliminating the need for a traditional KV cache and enabling efficient, minute-long video generation. In addition, we explore effective data filters and model training strategies, narrowing the training cost to 12 days on 64 H100 GPUs, which is only 1% of the cost of MovieGen. Given its low cost, SANA-Video achieves competitive performance compared to modern state-of-the-art small diffusion models (e.g., Wan 2.1-1.3B and SkyReel-V2-1.3B) while being 16x faster in measured latency. Moreover, SANA-Video can be deployed on RTX 5090 GPUs with NVFP4 precision, accelerating the inference speed of generating a 5-second 720p video from 71s to 29s (2.4x speedup). In summary, SANA-Video enables low-cost, high-quality video generation.*

The model can be loaded with the following code snippet.

```python
from diffusers import SanaVideoTransformer3DModel
import torch

transformer = SanaVideoTransformer3DModel.from_pretrained("Efficient-Large-Model/SANA-Video_2B_480p_diffusers", subfolder="transformer", torch_dtype=torch.bfloat16)
```

## SanaVideoTransformer3DModel[[diffusers.SanaVideoTransformer3DModel]]

#### diffusers.SanaVideoTransformer3DModel[[diffusers.SanaVideoTransformer3DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_sana_video.py#L457)

A 3D Transformer model introduced in [Sana-Video](https://huggingface.co/papers/2509.24695) family of models.

forwarddiffusers.SanaVideoTransformer3DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_sana_video.py#L573[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": Tensor"}, {"name": "guidance", "val": ": torch.Tensor | None = None"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "controlnet_block_samples", "val": ": tuple[torch.Tensor] | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, in_channels, num_frames, height, width)`) --
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

The [SanaVideoTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/sana_video_transformer3d#diffusers.SanaVideoTransformer3DModel) forward method.

**Parameters:**

in_channels (`int`, defaults to `16`) : The number of channels in the input.

out_channels (`int`, *optional*, defaults to `16`) : The number of channels in the output.

num_attention_heads (`int`, defaults to `20`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `112`) : The number of channels in each head.

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

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
