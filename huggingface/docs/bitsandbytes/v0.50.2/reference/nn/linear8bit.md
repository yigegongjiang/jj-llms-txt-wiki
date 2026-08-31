# LLM.int8()
[LLM.int8()](https://hf.co/papers/2208.07339) is a quantization method that aims to make large language model inference more accessible without significant degradation. Unlike naive 8-bit quantization, which can result in loss of critical information and accuracy, LLM.int8() dynamically adapts to ensure sensitive components of the computation retain higher precision when needed. The key is to extract the outliers from the inputs and weights and multiply them in 16-bit. All other values are multiplied in 8-bit before being dequantized back to 16-bits. The outputs from the 16-bit and 8-bit multiplication are combined to produce the final output.

[Further Resources](../../explanations/resources#llm-int8)

## Linear8bitLt[[bitsandbytes.nn.Linear8bitLt]]

#### bitsandbytes.nn.Linear8bitLt[[bitsandbytes.nn.Linear8bitLt]]

```python
bitsandbytes.nn.Linear8bitLt(input_features: int, output_features: int, bias = True, has_fp16_weights = True, threshold = 0.0, index = None, device = None)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/nn/modules.py#L1018)

This class is the base module for the [LLM.int8()](https://arxiv.org/abs/2208.07339) algorithm.
To read more about it, have a look at the paper.

In order to quantize a linear layer one should first load the original fp16 / bf16 weights into
the Linear8bitLt module, then call `int8_module.to("cuda")` to quantize the fp16 weights.

Example:

```python
import torch
import torch.nn as nn

import bitsandbytes as bnb
from bitsandbytes.nn import Linear8bitLt

fp16_model = nn.Sequential(
    nn.Linear(64, 64),
    nn.Linear(64, 64)
)

int8_model = nn.Sequential(
    Linear8bitLt(64, 64, has_fp16_weights=False),
    Linear8bitLt(64, 64, has_fp16_weights=False)
)

int8_model.load_state_dict(fp16_model.state_dict())
int8_model = int8_model.to(0) # Quantization happens here
```

#### __init__[[bitsandbytes.nn.Linear8bitLt.__init__]]

```python
__init__(input_features: int, output_features: int, bias = True, has_fp16_weights = True, threshold = 0.0, index = None, device = None)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/nn/modules.py#L1050)

**Parameters:**

input_features (*int*) : Number of input features of the linear layer.

output_features (*int*) : Number of output features of the linear layer.

bias (*bool*, defaults to *True*) : Whether the linear class uses the bias term as well.

has_fp16_weights (*bool*, defaults to *True*) : If False, weights are quantized to int8 on `.to(device)`. If True, weights remain in fp16 and are quantized on-the-fly during each forward pass.

threshold (*float*, defaults to *0.0*) : Outlier threshold for mixed-precision decomposition (LLM.int8()). During the forward pass, activation columns where any value exceeds this threshold are computed in fp16, while the remaining columns use int8. This operates on **activations** (inputs), not on weight values. Set to 0.0 to disable mixed-precision decomposition and quantize all columns to int8.

index : Indices for weight reordering (used internally).

device : Device to initialize the layer on.

Initialize Linear8bitLt class.

## Int8Params[[bitsandbytes.nn.Int8Params]]

#### bitsandbytes.nn.Int8Params[[bitsandbytes.nn.Int8Params]]

```python
bitsandbytes.nn.Int8Params(data: typing.Optional[torch.Tensor] = None, requires_grad = True, has_fp16_weights = False, CB: typing.Optional[torch.Tensor] = None, SCB: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/bitsandbytes/nn/modules.py#L719)

#### [[bitsandbytes.nn.Int8Params.__init__]]

```python
<lambda>(*args, **kwargs)
```

[Source](https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.2/doc_builder/mock_imports.py#L251)
