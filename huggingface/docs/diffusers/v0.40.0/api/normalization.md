# Normalization layers

Customized normalization layers for supporting various models in 🤗 Diffusers.

## AdaLayerNorm[[diffusers.models.normalization.AdaLayerNorm]]

#### diffusers.models.normalization.AdaLayerNorm[[diffusers.models.normalization.AdaLayerNorm]]

```python
diffusers.models.normalization.AdaLayerNorm(embedding_dim: int, num_embeddings: int | None = None, output_dim: int | None = None, norm_elementwise_affine: bool = False, norm_eps: float = 1e-05, chunk_dim: int = 0)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L27)

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`, *optional*) : The size of the embeddings dictionary.

output_dim (`int`, *optional*) --

norm_elementwise_affine (`bool`, defaults to `False) --

norm_eps (`bool`, defaults to `False`) --

chunk_dim (`int`, defaults to `0`) --

Norm layer modified to incorporate timestep embeddings.

## AdaLayerNormZero[[diffusers.models.normalization.AdaLayerNormZero]]

#### diffusers.models.normalization.AdaLayerNormZero[[diffusers.models.normalization.AdaLayerNormZero]]

```python
diffusers.models.normalization.AdaLayerNormZero(embedding_dim: int, num_embeddings: int | None = None, norm_type = 'layer_norm', bias = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L130)

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`) : The size of the embeddings dictionary.

Norm layer adaptive layer norm zero (adaLN-Zero).

## AdaLayerNormSingle[[diffusers.models.normalization.AdaLayerNormSingle]]

#### diffusers.models.normalization.AdaLayerNormSingle[[diffusers.models.normalization.AdaLayerNormSingle]]

```python
diffusers.models.normalization.AdaLayerNormSingle(embedding_dim: int, use_additional_conditions: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L235)

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

use_additional_conditions (`bool`) : To use additional conditions for normalization or not.

Norm layer adaptive layer norm single (adaLN-single).

As proposed in PixArt-Alpha (see: https://huggingface.co/papers/2310.00426; Section 2.3).

## AdaGroupNorm[[diffusers.models.normalization.AdaGroupNorm]]

#### diffusers.models.normalization.AdaGroupNorm[[diffusers.models.normalization.AdaGroupNorm]]

```python
diffusers.models.normalization.AdaGroupNorm(embedding_dim: int, out_dim: int, num_groups: int, act_fn: str | None = None, eps: float = 1e-05)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L269)

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`) : The size of the embeddings dictionary.

num_groups (`int`) : The number of groups to separate the channels into.

act_fn (`str`, *optional*, defaults to `None`) : The activation function to use.

eps (`float`, *optional*, defaults to `1e-5`) : The epsilon value to use for numerical stability.

GroupNorm layer modified to incorporate timestep embeddings.

## AdaLayerNormContinuous[[diffusers.models.normalization.AdaLayerNormContinuous]]

#### diffusers.models.normalization.AdaLayerNormContinuous[[diffusers.models.normalization.AdaLayerNormContinuous]]

```python
diffusers.models.normalization.AdaLayerNormContinuous(embedding_dim: int, conditioning_embedding_dim: int, elementwise_affine = True, eps = 1e-05, bias = True, norm_type = 'layer_norm')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L307)

**Parameters:**

embedding_dim (`int`) : Embedding dimension to use during projection.

conditioning_embedding_dim (`int`) : Dimension of the input condition.

elementwise_affine (`bool`, defaults to `True`) : Boolean flag to denote if affine transformation should be applied.

eps (`float`, defaults to 1e-5) : Epsilon factor.

bias (`bias`, defaults to `True`) : Boolean flag to denote if bias should be use.

norm_type (`str`, defaults to `"layer_norm"`) : Normalization layer to use. Values supported: "layer_norm", "rms_norm".

Adaptive normalization layer with a norm layer (layer_norm or rms_norm).

## RMSNorm[[diffusers.models.normalization.RMSNorm]]

#### diffusers.models.normalization.RMSNorm[[diffusers.models.normalization.RMSNorm]]

```python
diffusers.models.normalization.RMSNorm(dim, eps: float, elementwise_affine: bool = True, bias: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L510)

**Parameters:**

dim (`int`) : Number of dimensions to use for `weights`. Only effective when `elementwise_affine` is True.

eps (`float`) : Small value to use when calculating the reciprocal of the square-root.

elementwise_affine (`bool`, defaults to `True`) : Boolean flag to denote if affine transformation should be applied.

bias (`bool`, defaults to False) : If also training the `bias` param.

RMS Norm as introduced in https://huggingface.co/papers/1910.07467 by Zhang et al.

## GlobalResponseNorm[[diffusers.models.normalization.GlobalResponseNorm]]

#### diffusers.models.normalization.GlobalResponseNorm[[diffusers.models.normalization.GlobalResponseNorm]]

```python
diffusers.models.normalization.GlobalResponseNorm(dim)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L600)

**Parameters:**

dim (`int`) : Number of dimensions to use for the `gamma` and `beta`.

Global response normalization as introduced in ConvNeXt-v2 (https://huggingface.co/papers/2301.00808).

## LuminaLayerNormContinuous[[diffusers.models.normalization.LuminaLayerNormContinuous]]

#### diffusers.models.normalization.LuminaLayerNormContinuous[[diffusers.models.normalization.LuminaLayerNormContinuous]]

```python
diffusers.models.normalization.LuminaLayerNormContinuous(embedding_dim: int, conditioning_embedding_dim: int, elementwise_affine = True, eps = 1e-05, bias = True, norm_type = 'layer_norm', out_dim: int | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L354)

## SD35AdaLayerNormZeroX[[diffusers.models.normalization.SD35AdaLayerNormZeroX]]

#### diffusers.models.normalization.SD35AdaLayerNormZeroX[[diffusers.models.normalization.SD35AdaLayerNormZeroX]]

```python
diffusers.models.normalization.SD35AdaLayerNormZeroX(embedding_dim: int, norm_type: str = 'layer_norm', bias: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L96)

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`) : The size of the embeddings dictionary.

Norm layer adaptive layer norm zero (AdaLN-Zero).

## AdaLayerNormZeroSingle[[diffusers.models.normalization.AdaLayerNormZeroSingle]]

#### diffusers.models.normalization.AdaLayerNormZeroSingle[[diffusers.models.normalization.AdaLayerNormZeroSingle]]

```python
diffusers.models.normalization.AdaLayerNormZeroSingle(embedding_dim: int, norm_type = 'layer_norm', bias = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L173)

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`) : The size of the embeddings dictionary.

Norm layer adaptive layer norm zero (adaLN-Zero).

## LuminaRMSNormZero[[diffusers.models.normalization.LuminaRMSNormZero]]

#### diffusers.models.normalization.LuminaRMSNormZero[[diffusers.models.normalization.LuminaRMSNormZero]]

```python
diffusers.models.normalization.LuminaRMSNormZero(embedding_dim: int, norm_eps: float, norm_elementwise_affine: bool)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L205)

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

Norm layer adaptive RMS normalization zero.

## LpNorm[[diffusers.models.normalization.LpNorm]]

#### diffusers.models.normalization.LpNorm[[diffusers.models.normalization.LpNorm]]

```python
diffusers.models.normalization.LpNorm(p: int = 2, dim: int = -1, eps: float = 1e-12)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L620)

## CogView3PlusAdaLayerNormZeroTextImage[[diffusers.models.normalization.CogView3PlusAdaLayerNormZeroTextImage]]

#### diffusers.models.normalization.CogView3PlusAdaLayerNormZeroTextImage[[diffusers.models.normalization.CogView3PlusAdaLayerNormZeroTextImage]]

```python
diffusers.models.normalization.CogView3PlusAdaLayerNormZeroTextImage(embedding_dim: int, dim: int)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L403)

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

num_embeddings (`int`) : The size of the embeddings dictionary.

Norm layer adaptive layer norm zero (adaLN-Zero).

## CogVideoXLayerNormZero[[diffusers.models.normalization.CogVideoXLayerNormZero]]

#### diffusers.models.normalization.CogVideoXLayerNormZero[[diffusers.models.normalization.CogVideoXLayerNormZero]]

```python
diffusers.models.normalization.CogVideoXLayerNormZero(conditioning_dim: int, embedding_dim: int, elementwise_affine: bool = True, eps: float = 1e-05, bias: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L448)

## MochiRMSNormZero[[diffusers.models.transformers.transformer_mochi.MochiRMSNormZero]]

#### diffusers.models.transformers.transformer_mochi.MochiRMSNormZero[[diffusers.models.transformers.transformer_mochi.MochiRMSNormZero]]

```python
diffusers.models.transformers.transformer_mochi.MochiRMSNormZero(embedding_dim: int, hidden_dim: int, eps: float = 1e-05, elementwise_affine: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_mochi.py#L88)

**Parameters:**

embedding_dim (`int`) : The size of each embedding vector.

Adaptive RMS Norm used in Mochi.

## MochiRMSNorm[[diffusers.models.normalization.MochiRMSNorm]]

#### diffusers.models.normalization.MochiRMSNorm[[diffusers.models.normalization.MochiRMSNorm]]

```python
diffusers.models.normalization.MochiRMSNorm(dim, eps: float, elementwise_affine: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/normalization.py#L572)
