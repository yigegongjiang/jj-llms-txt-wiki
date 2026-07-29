# Normalization layers

Customized normalization layers for supporting various models in 🤗 Diffusers.

## AdaLayerNorm[[diffusers.models.normalization.AdaLayerNorm]]

#### diffusers.models.normalization.AdaLayerNorm[[diffusers.models.normalization.AdaLayerNorm]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L27)

Norm layer modified to incorporate timestep embeddings.

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`, *optional*) : The size of the embeddings dictionary.

output_dim (`int`, *optional*) --

norm_elementwise_affine (`bool`, defaults to `False) --

norm_eps (`bool`, defaults to `False`) --

chunk_dim (`int`, defaults to `0`) --

## AdaLayerNormZero[[diffusers.models.normalization.AdaLayerNormZero]]

#### diffusers.models.normalization.AdaLayerNormZero[[diffusers.models.normalization.AdaLayerNormZero]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L130)

Norm layer adaptive layer norm zero (adaLN-Zero).

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`) : The size of the embeddings dictionary.

## AdaLayerNormSingle[[diffusers.models.normalization.AdaLayerNormSingle]]

#### diffusers.models.normalization.AdaLayerNormSingle[[diffusers.models.normalization.AdaLayerNormSingle]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L235)

Norm layer adaptive layer norm single (adaLN-single).

As proposed in PixArt-Alpha (see: https://huggingface.co/papers/2310.00426; Section 2.3).

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

use_additional_conditions (`bool`) : To use additional conditions for normalization or not.

## AdaGroupNorm[[diffusers.models.normalization.AdaGroupNorm]]

#### diffusers.models.normalization.AdaGroupNorm[[diffusers.models.normalization.AdaGroupNorm]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L269)

GroupNorm layer modified to incorporate timestep embeddings.

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`) : The size of the embeddings dictionary.

num_groups (`int`) : The number of groups to separate the channels into.

act_fn (`str`, *optional*, defaults to `None`) : The activation function to use.

eps (`float`, *optional*, defaults to `1e-5`) : The epsilon value to use for numerical stability.

## AdaLayerNormContinuous[[diffusers.models.normalization.AdaLayerNormContinuous]]

#### diffusers.models.normalization.AdaLayerNormContinuous[[diffusers.models.normalization.AdaLayerNormContinuous]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L307)

Adaptive normalization layer with a norm layer (layer_norm or rms_norm).

**Parameters:**

embedding_dim (`int`) : Embedding dimension to use during projection.

conditioning_embedding_dim (`int`) : Dimension of the input condition.

elementwise_affine (`bool`, defaults to `True`) : Boolean flag to denote if affine transformation should be applied.

eps (`float`, defaults to 1e-5) : Epsilon factor.

bias (`bias`, defaults to `True`) : Boolean flag to denote if bias should be use.

norm_type (`str`, defaults to `"layer_norm"`) : Normalization layer to use. Values supported: "layer_norm", "rms_norm".

## RMSNorm[[diffusers.models.normalization.RMSNorm]]

#### diffusers.models.normalization.RMSNorm[[diffusers.models.normalization.RMSNorm]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L510)

RMS Norm as introduced in https://huggingface.co/papers/1910.07467 by Zhang et al.

**Parameters:**

dim (`int`) : Number of dimensions to use for `weights`. Only effective when `elementwise_affine` is True.

eps (`float`) : Small value to use when calculating the reciprocal of the square-root.

elementwise_affine (`bool`, defaults to `True`) : Boolean flag to denote if affine transformation should be applied.

bias (`bool`, defaults to False) : If also training the `bias` param.

## GlobalResponseNorm[[diffusers.models.normalization.GlobalResponseNorm]]

#### diffusers.models.normalization.GlobalResponseNorm[[diffusers.models.normalization.GlobalResponseNorm]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L600)

Global response normalization as introduced in ConvNeXt-v2 (https://huggingface.co/papers/2301.00808).

**Parameters:**

dim (`int`) : Number of dimensions to use for the `gamma` and `beta`.

## LuminaLayerNormContinuous[[diffusers.models.normalization.LuminaLayerNormContinuous]]
#### diffusers.models.normalization.LuminaLayerNormContinuous[[diffusers.models.normalization.LuminaLayerNormContinuous]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L354)

## SD35AdaLayerNormZeroX[[diffusers.models.normalization.SD35AdaLayerNormZeroX]]
#### diffusers.models.normalization.SD35AdaLayerNormZeroX[[diffusers.models.normalization.SD35AdaLayerNormZeroX]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L96)

Norm layer adaptive layer norm zero (AdaLN-Zero).

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`) : The size of the embeddings dictionary.

## AdaLayerNormZeroSingle[[diffusers.models.normalization.AdaLayerNormZeroSingle]]
#### diffusers.models.normalization.AdaLayerNormZeroSingle[[diffusers.models.normalization.AdaLayerNormZeroSingle]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L173)

Norm layer adaptive layer norm zero (adaLN-Zero).

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`) : The size of the embeddings dictionary.

## LuminaRMSNormZero[[diffusers.models.normalization.LuminaRMSNormZero]]
#### diffusers.models.normalization.LuminaRMSNormZero[[diffusers.models.normalization.LuminaRMSNormZero]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L205)

Norm layer adaptive RMS normalization zero.

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

## LpNorm[[diffusers.models.normalization.LpNorm]]
#### diffusers.models.normalization.LpNorm[[diffusers.models.normalization.LpNorm]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L620)

## CogView3PlusAdaLayerNormZeroTextImage[[diffusers.models.normalization.CogView3PlusAdaLayerNormZeroTextImage]]
#### diffusers.models.normalization.CogView3PlusAdaLayerNormZeroTextImage[[diffusers.models.normalization.CogView3PlusAdaLayerNormZeroTextImage]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L403)

Norm layer adaptive layer norm zero (adaLN-Zero).

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`) : The size of the embeddings dictionary.

## CogVideoXLayerNormZero[[diffusers.models.normalization.CogVideoXLayerNormZero]]
#### diffusers.models.normalization.CogVideoXLayerNormZero[[diffusers.models.normalization.CogVideoXLayerNormZero]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L448)

## MochiRMSNormZero[[diffusers.models.transformers.transformer_mochi.MochiRMSNormZero]]
#### diffusers.models.transformers.transformer_mochi.MochiRMSNormZero[[diffusers.models.transformers.transformer_mochi.MochiRMSNormZero]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/transformers/transformer_mochi.py#L88)

Adaptive RMS Norm used in Mochi.

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

## MochiRMSNorm[[diffusers.models.normalization.MochiRMSNorm]]
#### diffusers.models.normalization.MochiRMSNorm[[diffusers.models.normalization.MochiRMSNorm]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/normalization.py#L572)
