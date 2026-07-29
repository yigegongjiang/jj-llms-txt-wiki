# PriorTransformer

The Prior Transformer was originally introduced in [Hierarchical Text-Conditional Image Generation with CLIP Latents](https://huggingface.co/papers/2204.06125) by Ramesh et al. It is used to predict CLIP image embeddings from CLIP text embeddings; image embeddings are predicted through a denoising diffusion process.

The abstract from the paper is:

*Contrastive models like CLIP have been shown to learn robust representations of images that capture both semantics and style. To leverage these representations for image generation, we propose a two-stage model: a prior that generates a CLIP image embedding given a text caption, and a decoder that generates an image conditioned on the image embedding. We show that explicitly generating image representations improves image diversity with minimal loss in photorealism and caption similarity. Our decoders conditioned on image representations can also produce variations of an image that preserve both its semantics and style, while varying the non-essential details absent from the image representation. Moreover, the joint embedding space of CLIP enables language-guided image manipulations in a zero-shot fashion. We use diffusion models for the decoder and experiment with both autoregressive and diffusion models for the prior, finding that the latter are computationally more efficient and produce higher-quality samples.*

## PriorTransformer[[diffusers.PriorTransformer]]

#### diffusers.PriorTransformer[[diffusers.PriorTransformer]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/prior_transformer.py#L34)

A Prior Transformer model.

forwarddiffusers.PriorTransformer.forwardhttps://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/prior_transformer.py#L183[{"name": "hidden_states", "val": ""}, {"name": "timestep", "val": ": torch.Tensor | float | int"}, {"name": "proj_embedding", "val": ": Tensor"}, {"name": "encoder_hidden_states", "val": ": torch.Tensor | None = None"}, {"name": "attention_mask", "val": ": torch.BoolTensor | None = None"}, {"name": "return_dict", "val": ": bool = True"}]- **hidden_states** (`torch.Tensor` of shape `(batch_size, embedding_dim)`) --
  The currently predicted image embeddings.
- **timestep** (`torch.LongTensor`) --
  Current denoising step.
- **proj_embedding** (`torch.Tensor` of shape `(batch_size, embedding_dim)`) --
  Projected embedding vector the denoising process is conditioned on.
- **encoder_hidden_states** (`torch.Tensor` of shape `(batch_size, num_embeddings, embedding_dim)`) --
  Hidden states of the text embeddings the denoising process is conditioned on.
- **attention_mask** (`torch.BoolTensor` of shape `(batch_size, num_embeddings)`) --
  Text mask for the text embeddings.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not to return a [PriorTransformerOutput](/docs/diffusers/v0.39.0/en/api/models/prior_transformer#diffusers.models.transformers.prior_transformer.PriorTransformerOutput) instead of
  a plain tuple.0[PriorTransformerOutput](/docs/diffusers/v0.39.0/en/api/models/prior_transformer#diffusers.models.transformers.prior_transformer.PriorTransformerOutput) or `tuple`If return_dict is True, a [PriorTransformerOutput](/docs/diffusers/v0.39.0/en/api/models/prior_transformer#diffusers.models.transformers.prior_transformer.PriorTransformerOutput) is
returned, otherwise a tuple is returned where the first element is the sample tensor.

The [PriorTransformer](/docs/diffusers/v0.39.0/en/api/models/prior_transformer#diffusers.PriorTransformer) forward method.

**Parameters:**

num_attention_heads (`int`, *optional*, defaults to 32) : The number of heads to use for multi-head attention.

attention_head_dim (`int`, *optional*, defaults to 64) : The number of channels in each head.

num_layers (`int`, *optional*, defaults to 20) : The number of layers of Transformer blocks to use.

embedding_dim (`int`, *optional*, defaults to 768) : The dimension of the model input `hidden_states`

num_embeddings (`int`, *optional*, defaults to 77) : The number of embeddings of the model input `hidden_states`

additional_embeddings (`int`, *optional*, defaults to 4) : The number of additional tokens appended to the projected `hidden_states`. The actual length of the used `hidden_states` is `num_embeddings + additional_embeddings`.

dropout (`float`, *optional*, defaults to 0.0) : The dropout probability to use.

time_embed_act_fn (`str`, *optional*, defaults to 'silu') : The activation function to use to create timestep embeddings.

norm_in_type (`str`, *optional*, defaults to None) : The normalization layer to apply on hidden states before passing to Transformer blocks. Set it to `None` if normalization is not needed.

embedding_proj_norm_type (`str`, *optional*, defaults to None) : The normalization layer to apply on the input `proj_embedding`. Set it to `None` if normalization is not needed.

encoder_hid_proj_type (`str`, *optional*, defaults to `linear`) : The projection layer to apply on the input `encoder_hidden_states`. Set it to `None` if `encoder_hidden_states` is `None`.

added_emb_type (`str`, *optional*, defaults to `prd`) : Additional embeddings to condition the model. Choose from `prd` or `None`. if choose `prd`, it will prepend a token indicating the (quantized) dot product between the text embedding and image embedding as proposed in the unclip paper https://huggingface.co/papers/2204.06125 If it is `None`, no additional embeddings will be prepended.

time_embed_dim (`int, *optional*, defaults to None) : The dimension of timestep embeddings. If None, will be set to `num_attention_heads * attention_head_dim`

embedding_proj_dim (`int`, *optional*, default to None) : The dimension of `proj_embedding`. If None, will be set to `embedding_dim`.

clip_embed_dim (`int`, *optional*, default to None) : The dimension of the output. If None, will be set to `embedding_dim`.

**Returns:**

`[PriorTransformerOutput](/docs/diffusers/v0.39.0/en/api/models/prior_transformer#diffusers.models.transformers.prior_transformer.PriorTransformerOutput) or `tuple``

If return_dict is True, a [PriorTransformerOutput](/docs/diffusers/v0.39.0/en/api/models/prior_transformer#diffusers.models.transformers.prior_transformer.PriorTransformerOutput) is
returned, otherwise a tuple is returned where the first element is the sample tensor.
#### set_default_attn_processor[[diffusers.PriorTransformer.set_default_attn_processor]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/prior_transformer.py#L168)

Disables custom attention processors and sets the default attention implementation.

## PriorTransformerOutput[[diffusers.models.transformers.prior_transformer.PriorTransformerOutput]]

#### diffusers.models.transformers.prior_transformer.PriorTransformerOutput[[diffusers.models.transformers.prior_transformer.PriorTransformerOutput]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/prior_transformer.py#L22)

The output of [PriorTransformer](/docs/diffusers/v0.39.0/en/api/models/prior_transformer#diffusers.PriorTransformer).

**Parameters:**

predicted_image_embedding (`torch.Tensor` of shape `(batch_size, embedding_dim)`) : The predicted CLIP image embedding conditioned on the CLIP text embedding input.
