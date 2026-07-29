# OmniGenTransformer2DModel

A Transformer model that accepts multimodal instructions to generate images for [OmniGen](https://github.com/VectorSpaceLab/OmniGen/).

The abstract from the paper is:

*The emergence of Large Language Models (LLMs) has unified language  generation tasks and revolutionized human-machine interaction.  However, in the realm of image generation, a unified model capable of handling various tasks within a single framework remains largely unexplored. In this work, we introduce OmniGen, a new diffusion model for unified image generation. OmniGen is characterized by the following features: 1) Unification: OmniGen not only demonstrates text-to-image generation capabilities but also inherently supports various downstream tasks, such as image editing, subject-driven generation, and visual conditional generation. 2) Simplicity: The architecture of OmniGen is highly simplified, eliminating the need for additional plugins. Moreover, compared to existing diffusion models, it is more user-friendly and can complete complex tasks end-to-end through instructions without the need for extra intermediate steps, greatly simplifying the image generation workflow. 3) Knowledge Transfer: Benefit from learning in a unified format, OmniGen effectively transfers knowledge across different tasks, manages unseen tasks and domains, and exhibits novel capabilities. We also explore the model’s reasoning capabilities and potential applications of the chain-of-thought mechanism.  This work represents the first attempt at a general-purpose image generation model,  and we will release our resources at https://github.com/VectorSpaceLab/OmniGen to foster future advancements.*

```python
import torch
from diffusers import OmniGenTransformer2DModel

transformer = OmniGenTransformer2DModel.from_pretrained("Shitao/OmniGen-v1-diffusers", subfolder="transformer", torch_dtype=torch.bfloat16)
```

## OmniGenTransformer2DModel[[diffusers.OmniGenTransformer2DModel]]

#### diffusers.OmniGenTransformer2DModel[[diffusers.OmniGenTransformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_omnigen.py#L283)

The Transformer model introduced in OmniGen (https://huggingface.co/papers/2409.11340).

forwarddiffusers.OmniGenTransformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_omnigen.py#L407[{"name": "hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": int | float | torch.FloatTensor"}, {"name": "input_ids", "val": ": Tensor"}, {"name": "input_img_latents", "val": ": list"}, {"name": "input_image_sizes", "val": ": dict"}, {"name": "attention_mask", "val": ": Tensor"}, {"name": "position_ids", "val": ": Tensor"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, in_channels, height, width)`) --
  Input `hidden_states`.
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **input_ids** (`torch.Tensor`) --
  Multimodal text token ids used as conditioning.
- **input_img_latents** (`list` of `torch.Tensor`) --
  List of latents for input images used as conditioning.
- **input_image_sizes** (`dict` of `int` to `list` of `int`) --
  Mapping from sample index to the positions where input image embeddings should be placed in the
  conditioning sequence.
- **attention_mask** (`torch.Tensor`) --
  Attention mask for the joint multimodal sequence.
- **position_ids** (`torch.Tensor`) --
  Position ids used to compute the positional embeddings.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0`~models.transformer_2d.Transformer2DModelOutput` or `tuple`If `return_dict` is True, a `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise
a plain `tuple` is returned.

The [OmniGenTransformer2DModel](/docs/diffusers/v0.39.0/en/api/models/omnigen_transformer#diffusers.OmniGenTransformer2DModel) forward method.

**Parameters:**

in_channels (`int`, defaults to `4`) : The number of channels in the input.

patch_size (`int`, defaults to `2`) : The size of the spatial patches to use in the patch embedding layer.

hidden_size (`int`, defaults to `3072`) : The dimensionality of the hidden layers in the model.

rms_norm_eps (`float`, defaults to `1e-5`) : Eps for RMSNorm layer.

num_attention_heads (`int`, defaults to `32`) : The number of heads to use for multi-head attention.

num_key_value_heads (`int`, defaults to `32`) : The number of heads to use for keys and values in multi-head attention.

intermediate_size (`int`, defaults to `8192`) : Dimension of the hidden layer in FeedForward layers.

num_layers (`int`, default to `32`) : The number of layers of transformer blocks to use.

pad_token_id (`int`, default to `32000`) : The id of the padding token.

vocab_size (`int`, default to `32064`) : The size of the vocabulary of the embedding vocabulary.

rope_base (`int`, default to `10000`) : The default theta value to use when creating RoPE.

rope_scaling (`dict`, optional) : The scaling factors for the RoPE. Must contain `short_factor` and `long_factor`.

pos_embed_max_size (`int`, default to `192`) : The maximum size of the positional embeddings.

time_step_dim (`int`, default to `256`) : Output dimension of timestep embeddings.

flip_sin_to_cos (`bool`, default to `True`) : Whether to flip the sin and cos in the positional embeddings when preparing timestep embeddings.

downscale_freq_shift (`int`, default to `0`) : The frequency shift to use when downscaling the timestep embeddings.

timestep_activation_fn (`str`, default to `silu`) : The activation function to use for the timestep embeddings.

**Returns:**

``~models.transformer_2d.Transformer2DModelOutput` or `tuple``

If `return_dict` is True, a `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise
a plain `tuple` is returned.
