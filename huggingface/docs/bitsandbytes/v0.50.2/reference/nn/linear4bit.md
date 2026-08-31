# 4-bit quantization

[QLoRA](https://hf.co/papers/2305.14314) is a finetuning method that quantizes a model to 4-bits and adds a set of low-rank adaptation (LoRA) weights to the model and tuning them through the quantized weights. This method also introduces a new data type, 4-bit NormalFloat (`LinearNF4`) in addition to the standard Float4 data type (`LinearFP4`). `LinearNF4` is a quantization data type for normally distributed data and can improve performance.

## Linear4bit[[bitsandbytes.nn.Linear4bit]]

#### bitsandbytes.nn.Linear4bit[[bitsandbytes.nn.Linear4bit]]

```python
bitsandbytes.nn.Linear4bit(input_features, output_features, bias = True, compute_dtype = None, compress_statistics = True, quant_type = 'fp4', quant_storage = torch.uint8, device = None)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/nn/modules.py#L504)

This class is the base module for the 4-bit quantization algorithm presented in [QLoRA](https://arxiv.org/abs/2305.14314).
QLoRA 4-bit linear layers uses blockwise k-bit quantization under the hood, with the possibility of selecting various
compute datatypes such as FP4 and NF4.

In order to quantize a linear layer one should first load the original fp16 / bf16 weights into
the Linear4bit module, then call `quantized_module.to("cuda")` to quantize the fp16 / bf16 weights.

Example:

```python
import torch
import torch.nn as nn

import bitsandbytes as bnb
from bitsandbytes.nn import Linear4bit

fp16_model = nn.Sequential(
    nn.Linear(64, 64),
    nn.Linear(64, 64)
)

quantized_model = nn.Sequential(
    Linear4bit(64, 64),
    Linear4bit(64, 64)
)

quantized_model.load_state_dict(fp16_model.state_dict())
quantized_model = quantized_model.to(0) # Quantization happens here
```

#### __init__[[bitsandbytes.nn.Linear4bit.__init__]]

```python
__init__(input_features, output_features, bias = True, compute_dtype = None, compress_statistics = True, quant_type = 'fp4', quant_storage = torch.uint8, device = None)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/nn/modules.py#L537)

**Parameters:**

input_features (`str`) : Number of input features of the linear layer.

output_features (`str`) : Number of output features of the linear layer.

bias (`bool`, defaults to `True`) : Whether the linear class uses the bias term as well.

Initialize Linear4bit class.

## LinearFP4[[bitsandbytes.nn.LinearFP4]]

#### bitsandbytes.nn.LinearFP4[[bitsandbytes.nn.LinearFP4]]

```python
bitsandbytes.nn.LinearFP4(input_features, output_features, bias = True, compute_dtype = None, compress_statistics = True, quant_storage = torch.uint8, device = None)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/nn/modules.py#L640)

Implements the FP4 data type.

#### __init__[[bitsandbytes.nn.LinearFP4.__init__]]

```python
__init__(input_features, output_features, bias = True, compute_dtype = None, compress_statistics = True, quant_storage = torch.uint8, device = None)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/nn/modules.py#L645)

**Parameters:**

input_features (`str`) : Number of input features of the linear layer.

output_features (`str`) : Number of output features of the linear layer.

bias (`bool`, defaults to `True`) : Whether the linear class uses the bias term as well.

## LinearNF4[[bitsandbytes.nn.LinearNF4]]

#### bitsandbytes.nn.LinearNF4[[bitsandbytes.nn.LinearNF4]]

```python
bitsandbytes.nn.LinearNF4(input_features, output_features, bias = True, compute_dtype = None, compress_statistics = True, quant_storage = torch.uint8, device = None)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/nn/modules.py#L676)

Implements the NF4 data type.

Constructs a quantization data type where each bin has equal area under a standard normal distribution N(0, 1) that
is normalized into the range [-1, 1].

For more information read the paper: QLoRA: Efficient Finetuning of Quantized LLMs (https://arxiv.org/abs/2305.14314)

Implementation of the NF4 data type in bitsandbytes can be found in the `create_normal_map` function in
the `functional.py` file: https://github.com/TimDettmers/bitsandbytes/blob/main/bitsandbytes/functional.py#L236.

#### __init__[[bitsandbytes.nn.LinearNF4.__init__]]

```python
__init__(input_features, output_features, bias = True, compute_dtype = None, compress_statistics = True, quant_storage = torch.uint8, device = None)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/nn/modules.py#L688)

**Parameters:**

input_features (`str`) : Number of input features of the linear layer.

output_features (`str`) : Number of output features of the linear layer.

bias (`bool`, defaults to `True`) : Whether the linear class uses the bias term as well.

## Params4bit[[bitsandbytes.nn.Params4bit]]

#### bitsandbytes.nn.Params4bit[[bitsandbytes.nn.Params4bit]]

```python
bitsandbytes.nn.Params4bit(data: typing.Optional[torch.Tensor] = None, requires_grad = False, quant_state: typing.Optional[bitsandbytes.functional.QuantState] = None, blocksize: typing.Optional[int] = None, compress_statistics: bool = True, quant_type: str = 'fp4', quant_storage: dtype = torch.uint8, module: typing.Optional[ForwardRef('Linear4bit')] = None, bnb_quantized: bool = False, **kwargs)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/nn/modules.py#L213)

#### [[bitsandbytes.nn.Params4bit.__init__]]

```python
<lambda>(*args, **kwargs)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/doc_builder/mock_imports.py#L251)
