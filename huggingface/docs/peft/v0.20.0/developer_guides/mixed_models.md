# Mixed adapter types

Normally, it isn't possible to mix different adapter types in 🤗 PEFT. You can create a PEFT model with two different LoRA adapters (which can have different config options), but it is not possible to combine a LoRA and LoHa adapter. With [PeftMixedModel](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftMixedModel) however, this works as long as the adapter types are compatible. The main purpose of allowing mixed adapter types is to combine trained adapters for inference. While it is possible to train a mixed adapter model, this has not been tested and is not recommended.

To load different adapter types into a PEFT model, use [PeftMixedModel](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftMixedModel) instead of [PeftModel](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel):

```py
from peft import PeftMixedModel

base_model = ...  # load the base model, e.g. from transformers
# load first adapter, which will be called "default"
peft_model = PeftMixedModel.from_pretrained(base_model, <path_to_adapter1>)
peft_model.load_adapter(<path_to_adapter2>, adapter_name="other")
peft_model.set_adapter(["default", "other"])
```

The [set_adapter()](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftMixedModel.set_adapter) method is necessary to activate both adapters, otherwise only the first adapter would be active. You can keep adding more adapters by calling [add_adapter()](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel.add_adapter) repeatedly.

[PeftMixedModel](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftMixedModel) does not support saving and loading mixed adapters. The adapters should already be trained, and loading the model requires a script to be run each time.

## Tips

- Not all adapter types can be combined. See [`peft.tuners.mixed.COMPATIBLE_TUNER_TYPES`](https://github.com/huggingface/peft/blob/1c1c7fdaa6e6abaa53939b865dee1eded82ad032/src/peft/tuners/mixed/model.py#L35) for a list of compatible types. An error will be raised if you try to combine incompatible adapter types.
- It is possible to mix multiple adapters of the same type which can be useful for combining adapters with very different configs.
- If you want to combine a lot of different adapters, the most performant way to do it is to consecutively add the same adapter types. For example, add LoRA1, LoRA2, LoHa1, LoHa2 in this order, instead of LoRA1, LoHa1, LoRA2, and LoHa2. While the order can affect the output, there is no inherently *best* order, so it is best to choose the fastest one.
