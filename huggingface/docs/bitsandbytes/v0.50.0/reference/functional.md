# Overview
The `bitsandbytes.functional` API provides the low-level building blocks for the library's features.

## When to Use `bitsandbytes.functional`

* When you need direct control over quantized operations and their parameters.
* To build custom layers or operations leveraging low-bit arithmetic.
* To integrate with other ecosystem tooling.
* For experimental or research purposes requiring non-standard quantization or performance optimizations.

## LLM.int8()[[bitsandbytes.functional.int8_linear_matmul]]

- **A** (`torch.Tensor`) -- The first matrix operand with the data type `torch.int8`.
- **B** (`torch.Tensor`) -- The second matrix operand with the data type `torch.int8`.
- **out** (`torch.Tensor`, *optional*) -- A pre-allocated tensor used to store the result.
- **dtype** (`torch.dtype`, *optional*) -- The expected data type of the output. Defaults to `torch.int32`.`torch.Tensor`The result of the operation.- ``NotImplementedError`` -- The operation is not supported in the current environment.
- ``RuntimeError`` -- Raised when the cannot be completed for any other reason.</raises><raisederrors>``NotImplementedError`` or ``RuntimeError``
Performs an 8-bit integer matrix multiplication.

A linear transformation is applied such that `out = A @ B.T`. When possible, integer tensor core hardware is
utilized to accelerate the operation.

- **A** (`torch.Tensor` with dtype `torch.int32`) -- The result of a quantized int8 matrix multiplication.
- **row_stats** (`torch.Tensor`) -- The row-wise quantization statistics for the lhs operand of the matrix multiplication.
- **col_stats** (`torch.Tensor`) -- The column-wise quantization statistics for the rhs operand of the matrix multiplication.
- **out** (`torch.Tensor`, *optional*) -- A pre-allocated tensor to store the output of the operation.
- **bias** (`torch.Tensor`, *optional*) -- An optional bias vector to add to the result.`torch.Tensor`The dequantized result with an optional bias, with dtype `torch.float16`.
Performs dequantization on the result of a quantized int8 matrix multiplication.

- **A** (`torch.Tensor` with dtype `torch.int8`) -- The quantized int8 tensor.
- **stats** (`torch.Tensor` with dtype `torch.float32`) -- The row-wise quantization statistics.`torch.Tensor` with dtype `torch.float32`The dequantized tensor.
Dequantizes a tensor with dtype `torch.int8` to `torch.float32`.

- **A** (`torch.Tensor` with dtype `torch.float16`) -- The input tensor.
- **threshold** (`float`, *optional*) --
  An optional threshold for sparse decomposition of outlier features.

  No outliers are held back when 0.0. Defaults to 0.0.`Tuple[torch.Tensor, torch.Tensor, Optional[torch.Tensor]]`A tuple containing the quantized tensor and relevant statistics.
- `torch.Tensor` with dtype `torch.int8`: The quantized data.
- `torch.Tensor` with dtype `torch.float32`: The quantization scales.
- `torch.Tensor` with dtype `torch.int32`, *optional*: A list of column indices which contain outlier features.
Quantizes a tensor with dtype `torch.float16` to `torch.int8` in accordance to the `LLM.int8()` algorithm.

For more information, see the [LLM.int8() paper](https://arxiv.org/abs/2208.07339).

## 4-bit[[bitsandbytes.functional.dequantize_4bit]]

- **A** (`torch.Tensor`) -- The quantized input tensor.
- **quant_state** (`QuantState`, *optional*) --
  The quantization state as returned by `quantize_4bit`.
  Required if `absmax` is not provided.
- **absmax** (`torch.Tensor`, *optional*) --
  A tensor containing the scaling values.
  Required if `quant_state` is not provided and ignored otherwise.
- **out** (`torch.Tensor`, *optional*) -- A tensor to use to store the result.
- **blocksize** (`int`, *optional*) --
  The size of the blocks. Defaults to 64.
  Valid values are 32, 64, 128, 256, 512, 1024, 2048, and 4096.
- **quant_type** (`str`, *optional*) -- The data type to use: `nf4` or `fp4`. Defaults to `fp4`.`torch.Tensor`The dequantized tensor.- ``ValueError`` -- Raised when the input data type or blocksize is not supported.</raises><raisederrors>``ValueError``
Dequantizes a packed 4-bit quantized tensor.

The input tensor is dequantized by dividing it into blocks of `blocksize` values.
The absolute maximum value within these blocks is used for scaling
the non-linear dequantization.

- **A** (`torch.Tensor`) -- The input tensor. Supports `float16`, `bfloat16`, or `float32` datatypes.
- **absmax** (`torch.Tensor`, *optional*) -- A tensor to use to store the absmax values.
- **out** (`torch.Tensor`, *optional*) -- A tensor to use to store the result.
- **blocksize** (`int`, *optional*) --
  The size of the blocks. Defaults to 64.
  Valid values are 32, 64, 128, 256, 512, 1024, 2048, and 4096.
- **compress_statistics** (`bool`, *optional*) -- Whether to additionally quantize the absmax values. Defaults to False.
- **quant_type** (`str`, *optional*) -- The data type to use: `nf4` or `fp4`. Defaults to `fp4`.
- **quant_storage** (`torch.dtype`, *optional*) -- The dtype of the tensor used to store the result. Defaults to `torch.uint8`.Tuple[`torch.Tensor`, `QuantState`]A tuple containing the quantization results.
- `torch.Tensor`: The quantized tensor with packed 4-bit values.
- `QuantState`: The state object used to undo the quantization.- ``ValueError`` -- Raised when the input data type is not supported.</raises><raisederrors>``ValueError``
Quantize tensor A in blocks of 4-bit values.

Quantizes tensor A by dividing it into blocks which are independently quantized.

container for quantization state components to work with Params4bit and similar classes

returns dict of tensors and strings to use in serialization via _save_to_state_dict()
param: packed -- returns dict[str, torch.Tensor] for state_dict fit for safetensors saving

unpacks components of state_dict into QuantState
where necessary, convert into strings, torch.dtype, ints, etc.

qs_dict: based on state_dict, with only relevant keys, striped of prefixes.

item with key `quant_state.bitsandbytes__[nf4/fp4]` may contain minor and non-tensor quant state items.

## Dynamic 8-bit Quantization[[bitsandbytes.functional.dequantize_blockwise]]

Primitives used in the 8-bit optimizer quantization.

For more details see [8-Bit Approximations for Parallelism in Deep Learning](https://arxiv.org/abs/1511.04561)

- **A** (`torch.Tensor`) -- The quantized input tensor.
- **quant_state** (`QuantState`, *optional*) --
  The quantization state as returned by `quantize_blockwise`.
  Required if `absmax` is not provided.
- **absmax** (`torch.Tensor`, *optional*) --
  A tensor containing the scaling values.
  Required if `quant_state` is not provided and ignored otherwise.
- **code** (`torch.Tensor`, *optional*) --
  A mapping describing the low-bit data type. Defaults to a signed 8-bit dynamic type.
  For more details, see  (8-Bit Approximations for Parallelism in Deep Learning)[https://arxiv.org/abs/1511.04561].
  Ignored when `quant_state` is provided.
- **out** (`torch.Tensor`, *optional*) -- A tensor to use to store the result.
- **blocksize** (`int`, *optional*) --
  The size of the blocks. Defaults to 4096.
  Valid values are 64, 128, 256, 512, 1024, 2048, and 4096.
  Ignored when `quant_state` is provided.`torch.Tensor`The dequantized tensor. The datatype is indicated by `quant_state.dtype` and defaults to `torch.float32`.- ``ValueError`` -- Raised when the input data type is not supported.</raises><raisederrors>``ValueError``
Dequantize a tensor in blocks of values.

The input tensor is dequantized by dividing it into blocks of `blocksize` values.
The the absolute maximum value within these blocks is used for scaling
the non-linear dequantization.

- **A** (`torch.Tensor`) -- The input tensor. Supports `float16`, `bfloat16`, or `float32` datatypes.
- **code** (`torch.Tensor`, *optional*) --
  A mapping describing the low-bit data type. Defaults to a signed 8-bit dynamic type.
  For more details, see  (8-Bit Approximations for Parallelism in Deep Learning)[https://arxiv.org/abs/1511.04561].
- **absmax** (`torch.Tensor`, *optional*) -- A tensor to use to store the absmax values.
- **out** (`torch.Tensor`, *optional*) -- A tensor to use to store the result.
- **blocksize** (`int`, *optional*) --
  The size of the blocks. Defaults to 4096.
  Valid values are 64, 128, 256, 512, 1024, 2048, and 4096.
- **nested** (`bool`, *optional*) -- Whether to additionally quantize the absmax values. Defaults to False.`Tuple[torch.Tensor, QuantState]`A tuple containing the quantization results.
- `torch.Tensor`: The quantized tensor.
- `QuantState`: The state object used to undo the quantization.- ``ValueError`` -- Raised when the input data type is not supported.</raises><raisederrors>``ValueError``
Quantize a tensor in blocks of values.

The input tensor is quantized by dividing it into blocks of `blocksize` values.
The the absolute maximum value within these blocks is calculated for scaling
the non-linear quantization.

## Utility[[bitsandbytes.functional.get_ptr]]

- **A** (`Optional[Tensor]`) -- A PyTorch tensor.`Optional[ct.c_void_p]`A pointer to the underlying tensor data.
Gets the memory address of the first element of a tenso
