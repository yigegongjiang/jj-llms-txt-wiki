# Hotswapping adapters

The idea of hotswapping an adapter is the following: We can already load multiple adapters, e.g. two LoRAs, at the same time. But sometimes, we want to load one LoRA and then replace its weights in-place with the LoRA weights of another adapter. This is now possible the `hotswap_adapter` function.

In general, this should be faster than deleting one adapter and loading the adapter in its place, which would be the how to achieve the same final outcome without hotswapping. Another advantage of hotswapping is that it prevents re-compilation in case the PEFT model is already compiled using `torch.compile`. This can save quite a lot of time.

## Example without `torch.compile`

```python
import torch
from transformers import AutoModelForCausalLM
from peft import PeftModel
from peft.utils.hotswap import hotswap_adapter

model_id = ...
inputs = ...
device = ...
model = AutoModelForCausalLM.from_pretrained(model_id).to(device)

# load lora 0
model = PeftModel.from_pretrained(model, <path-adapter-0>)
with torch.inference_mode():
    output_adapter_0 = model(inputs)

# replace the "default" lora adapter with the new one
hotswap_adapter(model, <path-adapter-1>, adapter_name="default", torch_device=device)
with torch.inference_mode():
    output_adapter_1 = model(inputs).logits
```

## Example with `torch.compile`

```python
import torch
from transformers import AutoModelForCausalLM
from peft import PeftModel
from peft.utils.hotswap import hotswap_adapter, prepare_model_for_compiled_hotswap

model_id = ...
inputs = ...
device = ...
max_rank = ...  # maximum rank among all LoRA adapters that will be used
model = AutoModelForCausalLM.from_pretrained(model_id).to(device)

# load lora 0
model = PeftModel.from_pretrained(model, <path-adapter-0>)
# Prepare the model to allow hotswapping even if ranks/scalings of 2nd adapter differ.
# You can skip this step if all ranks and scalings are identical.
prepare_model_for_compiled_hotswap(model, target_rank=max_rank)
model = torch.compile(model)
with torch.inference_mode():
    output_adapter_0 = model(inputs)

# replace the "default" lora adapter with the new one
hotswap_adapter(model, <path-adapter-1>, adapter_name="default", torch_device=device)
with torch.inference_mode():
    output_adapter_1 = model(inputs).logits
```

Note that if you want to hotswap weights that were added through `target_parameters`, i.e. that directly target an `nn.Parameter`, re-compilation and/or graph breaks cannot be prevented. Therefore, it is recommended to avoid using `target_parameters` together with compiled models and hotswapping.

## Caveats

Hotswapping works with transformers models and diffusers models. However, there are some caveats:

- Right now, only LoRA is properly supported.
- It only works for the same PEFT method, so no swapping LoRA and LoHa, for example.
- The adapter that is being swapped in must target the same layers as the previous adapter or a subset of those layers. It cannot target new layers. Therefore, if possible, start with the adapter that targets most layers.

## API[[peft.utils.hotswap.hotswap_adapter]]

- **model** ([~PeftModel](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel)) --
  The PEFT model with the loaded adapter.
- **model_name_or_path** (`str`) --
  The name or path of the model to load the new adapter from.
- **adapter_name** (`str`) --
  The name of the adapter to swap, e.g. `"default"`. The name will stay the same after swapping.
- **torch_device** -- (`str`, *optional*, defaults to None):
  The device to load the new adapter onto.
- ****kwargs** (`optional`) --
  Additional keyword arguments used for loading the config and weights.
Substitute old adapter data with new adapter data, keeping the rest the same.

As of now, only LoRA is supported.

This function is useful when you want to replace the loaded adapter with a new adapter. The adapter name will
remain the same, but the weights and other parameters will be swapped out.

If the adapters are incomptabile, e.g. targeting different layers or having different alpha values, an error will
be raised.

Example:

```py
>>> import torch
>>> from transformers import AutoModelForCausalLM
>>> from peft import PeftModel
>>> from peft.utils.hotswap import hotswap_adapter

>>> model_id = ...
>>> inputs = ...
>>> device = ...
>>> model = AutoModelForCausalLM.from_pretrained(model_id).to(device)

>>> # load lora 0
>>> model = PeftModel.from_pretrained(model, "path-adapter-0")
>>> model = torch.compile(model)  # optionally compile the model
>>> with torch.inference_mode():
...     output_adapter_0 = model(inputs)

>>> # replace the "default" lora adapter with the new one
>>> hotswap_adapter(model, "path-adapter-1", adapter_name="default", torch_device=device)
>>> with torch.inference_mode():
...     output_adapter_1 = model(inputs).logits
```

- **model** (`nn.Module`) --
  The model with the loaded adapter.
- **state_dict** (`dict[str, torch.Tensor]`) --
  The state dict of the new adapter, which needs to be compatible (targeting same modules etc.).
- **adapter_name** (`str`) --
  The name of the adapter that should be hot-swapped, e.g. `"default"`. The name will remain the same after
  swapping.
- **config** (`LoraConfig`) --
  The config of the LoRA adapter. This is used to determine the scaling and rank of the adapter.
- **parameter_prefix** (`str`, *optional*, defaults to `"lora_"`) --
  The prefix used to identify the adapter's keys in the state dict. For LoRA, this would be `"lora_"` (the
  default).- ``RuntimeError`` -- 
  If the old and the new adapter are not compatible, a RuntimeError is raised.``RuntimeError``

Swap out the adapter weights from the model with the weights from state_dict.

As of now, only LoRA is supported.

This is a low-level function that assumes that the adapters have been checked for compatibility and that the
state_dict has been correctly mapped to work with PEFT. For a high level function that performs this work for you,
use `hotswap_adapter` instead.
