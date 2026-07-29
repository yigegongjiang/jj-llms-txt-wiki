# 4-bit quantization

[QLoRA](https://hf.co/papers/2305.14314) is a finetuning method that quantizes a model to 4-bits and adds a set of low-rank adaptation (LoRA) weights to the model and tuning them through the quantized weights. This method also introduces a new data type, 4-bit NormalFloat (`LinearNF4`) in addition to the standard Float4 data type (`LinearFP4`). `LinearNF4` is a quantization data type for normally distributed data and can improve performance.

## Linear4bit[[bitsandbytes.nn.Linear4bit]]

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

- **input_features** (`str`) --
  Number of input features of the linear layer.
- **output_features** (`str`) --
  Number of output features of the linear layer.
- **bias** (`bool`, defaults to `True`) --
  Whether the linear class uses the bias term as well.

Initialize Linear4bit class.

## LinearFP4[[bitsandbytes.nn.LinearFP4]]

Implements the FP4 data type.

- **input_features** (`str`) --
  Number of input features of the linear layer.
- **output_features** (`str`) --
  Number of output features of the linear layer.
- **bias** (`bool`, defaults to `True`) --
  Whether the linear class uses the bias term as well.

## LinearNF4[[bitsandbytes.nn.LinearNF4]]

Implements the NF4 data type.

Constructs a quantization data type where each bin has equal area under a standard normal distribution N(0, 1) that
is normalized into the range [-1, 1].

For more information read the paper: QLoRA: Efficient Finetuning of Quantized LLMs (https://arxiv.org/abs/2305.14314)

Implementation of the NF4 data type in bitsandbytes can be found in the `create_normal_map` function in
the `functional.py` file: https://github.com/TimDettmers/bitsandbytes/blob/main/bitsandbytes/functional.py#L236.

- **input_features** (`str`) --
  Number of input features of the linear layer.
- **output_features** (`str`) --
  Number of output features of the linear layer.
- **bias** (`bool`, defaults to `True`) --
  Whether the linear class uses the bias term as well.

## Params4bit[[bitsandbytes.nn.Params4bit]]

"} anchor={"bitsandbytes.nn.Params4bit.__init__"} source={"https://github.com/bitsandbytes-foundation/bitsandbytes/blob/v0.50.0/doc_builder/mock_imports.py#L251"} parameters={[{"name": "*args", "val": ""}, {"name": "**kwargs", "val": ""}]}>
