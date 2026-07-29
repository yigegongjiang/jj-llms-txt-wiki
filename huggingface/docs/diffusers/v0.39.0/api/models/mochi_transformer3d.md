# MochiTransformer3DModel

A Diffusion Transformer model for 3D video-like data was introduced in [Mochi-1 Preview](https://huggingface.co/genmo/mochi-1-preview) by Genmo.

The model can be loaded with the following code snippet.

```python
from diffusers import MochiTransformer3DModel

transformer = MochiTransformer3DModel.from_pretrained("genmo/mochi-1-preview", subfolder="transformer", torch_dtype=torch.float16).to("cuda")
```

## MochiTransformer3DModel[[diffusers.MochiTransformer3DModel]]

#### diffusers.MochiTransformer3DModel[[diffusers.MochiTransformer3DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_mochi.py#L309)

A Transformer model for video-like data introduced in [Mochi](https://huggingface.co/genmo/mochi-1-preview).

forwarddiffusers.MochiTransformer3DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_mochi.py#L407[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": Tensor"}, {"name": "timestep", "val": ": LongTensor"}, {"name": "encoder_attention_mask", "val": ": Tensor"}, {"name": "attention_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, num_channels, num_frames, height, width)`) --
  Input `hidden_states`.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, sequence_len, embed_dims)`) --
  Conditional embeddings (embeddings computed from the input conditions such as prompts) to use.
- **timestep** (`torch.LongTensor`) --
  Used to indicate denoising step.
- **encoder_attention_mask** (`torch.Tensor`) --
  Mask applied to `encoder_hidden_states` during attention.
- **attention_kwargs** (`dict`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain
  tuple.0`torch.Tensor`The denoised output tensor of shape `(batch_size, out_channels, num_frames, height, width)`.

The [MochiTransformer3DModel](/docs/diffusers/v0.39.0/en/api/models/mochi_transformer3d#diffusers.MochiTransformer3DModel) forward method.

**Parameters:**

patch_size (`int`, defaults to `2`) : The size of the patches to use in the patch embedding layer.

num_attention_heads (`int`, defaults to `24`) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, defaults to `128`) : The number of channels in each head.

num_layers (`int`, defaults to `48`) : The number of layers of Transformer blocks to use.

in_channels (`int`, defaults to `12`) : The number of channels in the input.

out_channels (`int`, *optional*, defaults to `None`) : The number of channels in the output.

qk_norm (`str`, defaults to `"rms_norm"`) : The normalization layer to use.

text_embed_dim (`int`, defaults to `4096`) : Input dimension of text embeddings from the text encoder.

time_embed_dim (`int`, defaults to `256`) : Output dimension of timestep embeddings.

activation_fn (`str`, defaults to `"swiglu"`) : Activation function to use in feed-forward.

max_sequence_length (`int`, defaults to `256`) : The maximum sequence length of text embeddings supported.

**Returns:**

``torch.Tensor``

The denoised output tensor of shape `(batch_size, out_channels, num_frames, height, width)`.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
