# Activation functions

Customized activation functions for supporting various models in 🤗 Diffusers.

## GELU[[diffusers.models.activations.GELU]]

#### diffusers.models.activations.GELU[[diffusers.models.activations.GELU]]

```python
diffusers.models.activations.GELU(dim_in: int, dim_out: int, approximate: str = 'none', bias: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/activations.py#L65)

**Parameters:**

dim_in (`int`) : The number of channels in the input.

dim_out (`int`) : The number of channels in the output.

approximate (`str`, *optional*, defaults to `"none"`) : If `"tanh"`, use tanh approximation.

bias (`bool`, defaults to True) : Whether to use a bias in the linear layer.

GELU activation function with tanh approximation support with `approximate="tanh"`.

## GEGLU[[diffusers.models.activations.GEGLU]]

#### diffusers.models.activations.GEGLU[[diffusers.models.activations.GEGLU]]

```python
diffusers.models.activations.GEGLU(dim_in: int, dim_out: int, bias: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/activations.py#L93)

**Parameters:**

dim_in (`int`) : The number of channels in the input.

dim_out (`int`) : The number of channels in the output.

bias (`bool`, defaults to True) : Whether to use a bias in the linear layer.

A [variant](https://huggingface.co/papers/2002.05202) of the gated linear unit activation function.

## ApproximateGELU[[diffusers.models.activations.ApproximateGELU]]

#### diffusers.models.activations.ApproximateGELU[[diffusers.models.activations.ApproximateGELU]]

```python
diffusers.models.activations.ApproximateGELU(dim_in: int, dim_out: int, bias: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/activations.py#L149)

**Parameters:**

dim_in (`int`) : The number of channels in the input.

dim_out (`int`) : The number of channels in the output.

bias (`bool`, defaults to True) : Whether to use a bias in the linear layer.

The approximate form of the Gaussian Error Linear Unit (GELU). For more details, see section 2 of this
[paper](https://huggingface.co/papers/1606.08415).

## SwiGLU[[diffusers.models.activations.SwiGLU]]

#### diffusers.models.activations.SwiGLU[[diffusers.models.activations.SwiGLU]]

```python
diffusers.models.activations.SwiGLU(dim_in: int, dim_out: int, bias: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/activations.py#L126)

**Parameters:**

dim_in (`int`) : The number of channels in the input.

dim_out (`int`) : The number of channels in the output.

bias (`bool`, defaults to True) : Whether to use a bias in the linear layer.

A [variant](https://huggingface.co/papers/2002.05202) of the gated linear unit activation function. It's similar to
`GEGLU` but uses SiLU / Swish instead of GeLU.

## FP32SiLU[[diffusers.models.activations.FP32SiLU]]

#### diffusers.models.activations.FP32SiLU[[diffusers.models.activations.FP32SiLU]]

```python
diffusers.models.activations.FP32SiLU()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/activations.py#L53)

SiLU activation function with input upcasted to torch.float32.

## LinearActivation[[diffusers.models.activations.LinearActivation]]

#### diffusers.models.activations.LinearActivation[[diffusers.models.activations.LinearActivation]]

```python
diffusers.models.activations.LinearActivation(dim_in: int, dim_out: int, bias: bool = True, activation: str = 'silu')
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/activations.py#L169)
