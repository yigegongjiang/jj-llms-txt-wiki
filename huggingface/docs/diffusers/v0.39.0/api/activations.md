# Activation functions

Customized activation functions for supporting various models in 🤗 Diffusers.

## GELU[[diffusers.models.activations.GELU]]

#### diffusers.models.activations.GELU[[diffusers.models.activations.GELU]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/activations.py#L65)

GELU activation function with tanh approximation support with `approximate="tanh"`.

**Parameters:**

dim_in (`int`) : The number of channels in the input.

dim_out (`int`) : The number of channels in the output.

approximate (`str`, *optional*, defaults to `"none"`) : If `"tanh"`, use tanh approximation.

bias (`bool`, defaults to True) : Whether to use a bias in the linear layer.

## GEGLU[[diffusers.models.activations.GEGLU]]

#### diffusers.models.activations.GEGLU[[diffusers.models.activations.GEGLU]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/activations.py#L93)

A [variant](https://huggingface.co/papers/2002.05202) of the gated linear unit activation function.

**Parameters:**

dim_in (`int`) : The number of channels in the input.

dim_out (`int`) : The number of channels in the output.

bias (`bool`, defaults to True) : Whether to use a bias in the linear layer.

## ApproximateGELU[[diffusers.models.activations.ApproximateGELU]]

#### diffusers.models.activations.ApproximateGELU[[diffusers.models.activations.ApproximateGELU]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/activations.py#L149)

The approximate form of the Gaussian Error Linear Unit (GELU). For more details, see section 2 of this
[paper](https://huggingface.co/papers/1606.08415).

**Parameters:**

dim_in (`int`) : The number of channels in the input.

dim_out (`int`) : The number of channels in the output.

bias (`bool`, defaults to True) : Whether to use a bias in the linear layer.

## SwiGLU[[diffusers.models.activations.SwiGLU]]

#### diffusers.models.activations.SwiGLU[[diffusers.models.activations.SwiGLU]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/activations.py#L126)

A [variant](https://huggingface.co/papers/2002.05202) of the gated linear unit activation function. It's similar to
`GEGLU` but uses SiLU / Swish instead of GeLU.

**Parameters:**

dim_in (`int`) : The number of channels in the input.

dim_out (`int`) : The number of channels in the output.

bias (`bool`, defaults to True) : Whether to use a bias in the linear layer.

## FP32SiLU[[diffusers.models.activations.FP32SiLU]]

#### diffusers.models.activations.FP32SiLU[[diffusers.models.activations.FP32SiLU]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/activations.py#L53)

SiLU activation function with input upcasted to torch.float32.

## LinearActivation[[diffusers.models.activations.LinearActivation]]

#### diffusers.models.activations.LinearActivation[[diffusers.models.activations.LinearActivation]]

[Source](https://github.com/huggingface/diffusers/blob/v0.39.0/src/diffusers/models/activations.py#L169)
