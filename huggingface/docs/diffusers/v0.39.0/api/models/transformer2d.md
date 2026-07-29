# Transformer2DModel

A Transformer model for image-like data from [CompVis](https://huggingface.co/CompVis) that is based on the [Vision Transformer](https://huggingface.co/papers/2010.11929) introduced by Dosovitskiy et al. The [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) accepts discrete (classes of vector embeddings) or continuous (actual embeddings) inputs.

When the input is **continuous**:

1. Project the input and reshape it to `(batch_size, sequence_length, feature_dimension)`.
2. Apply the Transformer blocks in the standard way.
3. Reshape to image.

When the input is **discrete**:

> [!TIP]
> It is assumed one of the input classes is the masked latent pixel. The predicted classes of the unnoised image don't contain a prediction for the masked pixel because the unnoised image cannot be masked.

1. Convert input (classes of latent pixels) to embeddings and apply positional embeddings.
2. Apply the Transformer blocks in the standard way.
3. Predict classes of unnoised image.

## Transformer2DModel[[diffusers.Transformer2DModel]]

#### diffusers.Transformer2DModel[[diffusers.Transformer2DModel]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_2d.py#L39)

A 2D Transformer model for image-like data.

forwarddiffusers.Transformer2DModel.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_2d.py#L324[{"name": "hidden_states", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": torch.Tensor | None = None"}, {"name": "timestep", "val": ": torch.LongTensor | None = None"}, {"name": "added_cond_kwargs", "val": ": dict = None"}, {"name": "class_labels", "val": ": torch.LongTensor | None = None"}, {"name": "cross_attention_kwargs", "val": ": dict = None"}, {"name": "attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "encoder_attention_mask", "val": ": torch.Tensor | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.LongTensor` of shape `(batch size, num latent pixels)` if discrete, `torch.Tensor` of shape `(batch size, channel, height, width)` if continuous) --
  Input `hidden_states`.
- **encoder_hidden_states** ( `torch.Tensor` of shape `(batch size, sequence len, embed dims)`, *optional*) --
  Conditional embeddings for cross attention layer. If not given, cross-attention defaults to
  self-attention.
- **timestep** ( `torch.LongTensor`, *optional*) --
  Used to indicate denoising step. Optional timestep to be applied as an embedding in `AdaLayerNorm`.
- **class_labels** ( `torch.LongTensor` of shape `(batch size, num classes)`, *optional*) --
  Used to indicate class labels conditioning. Optional class labels to be applied as an embedding in
  `AdaLayerZeroNorm`.
- **cross_attention_kwargs** ( `dict[str, Any]`, *optional*) --
  A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under
  `self.processor` in
  [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).
- **attention_mask** ( `torch.Tensor`, *optional*) --
  An attention mask of shape `(batch, key_tokens)` is applied to `encoder_hidden_states`. If `1` the mask
  is kept, otherwise if `0` it is discarded. Mask will be converted into a bias, which adds large
  negative values to the attention scores corresponding to "discard" tokens.
- **encoder_attention_mask** ( `torch.Tensor`, *optional*) --
  Cross-attention mask applied to `encoder_hidden_states`. Two formats supported:

  * Mask `(batch, sequence_length)` True = keep, False = discard.
  * Bias `(batch, 1, sequence_length)` 0 = keep, -10000 = discard.

  If `ndim == 2`: will be interpreted as a mask, then converted into a bias consistent with the format
  above. This bias will be added to the cross-attention scores.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [UNet2DConditionOutput](/docs/diffusers/v0.39.0/en/api/models/unet2d-cond#diffusers.models.unets.unet_2d_condition.UNet2DConditionOutput) instead of a plain
  tuple.0If `return_dict` is True, an `Transformer2DModelOutput` is returned,
otherwise a `tuple` where the first element is the sample tensor.

The [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) forward method.

**Parameters:**

num_attention_heads (`int`, *optional*, defaults to 16) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, *optional*, defaults to 88) : The number of channels in each head.

in_channels (`int`, *optional*) : The number of channels in the input and output (specify if the input is **continuous**).

num_layers (`int`, *optional*, defaults to 1) : The number of layers of Transformer blocks to use.

dropout (`float`, *optional*, defaults to 0.0) : The dropout probability to use.

cross_attention_dim (`int`, *optional*) : The number of `encoder_hidden_states` dimensions to use.

sample_size (`int`, *optional*) : The width of the latent images (specify if the input is **discrete**). This is fixed during training since it is used to learn a number of position embeddings.

num_vector_embeds (`int`, *optional*) : The number of classes of the vector embeddings of the latent pixels (specify if the input is **discrete**). Includes the class for the masked latent pixel.

activation_fn (`str`, *optional*, defaults to `"geglu"`) : Activation function to use in feed-forward.

num_embeds_ada_norm ( `int`, *optional*) : The number of diffusion steps used during training. Pass if at least one of the norm_layers is `AdaLayerNorm`. This is fixed during training since it is used to learn a number of embeddings that are added to the hidden states.  During inference, you can denoise for up to but not more steps than `num_embeds_ada_norm`.

attention_bias (`bool`, *optional*) : Configure if the `TransformerBlocks` attention should contain a bias parameter.

**Returns:**

If `return_dict` is True, an `Transformer2DModelOutput` is returned,
otherwise a `tuple` where the first element is the sample tensor.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/modeling_outputs.py#L21)

The output of [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel).

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.39.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.
