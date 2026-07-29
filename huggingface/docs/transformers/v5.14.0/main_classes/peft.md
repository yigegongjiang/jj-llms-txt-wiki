# PEFT[[transformers.integrations.PeftAdapterMixin]]

The [PeftAdapterMixin](/docs/transformers/v5.14.0/en/main_classes/peft#transformers.integrations.PeftAdapterMixin) provides functions from the [PEFT](https://huggingface.co/docs/peft/index) library for managing adapters with Transformers. This mixin supports all non-prompt-learning PEFT methods (LoRA, IA3, AdaLoRA, and others). Prefix tuning methods (prompt tuning, prompt learning) aren't supported because they can't be injected into a torch module.

A class containing all functions for loading and using adapters weights that are supported in PEFT library. For
more details about adapters and injecting them on a transformer-based model, check out the documentation of PEFT
library: https://huggingface.co/docs/peft/index

Currently supported PEFT methods are all non-prompt learning methods (LoRA, IA³, etc.). Other PEFT models such as
prompt tuning, prompt learning are out of scope as these adapters are not "injectable" into a torch module. For
using these methods, please refer to the usage guide of PEFT library.

With this mixin, if the correct PEFT version is installed (>= 0.19.1), it is possible to:

- Load an adapter stored on a local path or in a remote Hub repository, and inject it in the model
- Attach new adapters in the model and train them with Trainer or by your own.
- Attach multiple adapters and iteratively activate / deactivate them
- Activate / deactivate all adapters from the model.
- Get the `state_dict` of the active adapter.

- **peft_model_id** (`str`, *optional*) --
  The identifier of the model to look for on the Hub, or a local path to the saved adapter config file
  and adapter weights.
- **adapter_name** (`str`, *optional*) --
  The adapter name to use. If not set, will use the name "default".
- **load_config** (`LoadStateDictConfig`, *optional*) --
  A load configuration to reuse when pulling adapter weights, typically from `from_pretrained`.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional `LoadStateDictConfig` fields passed as keyword arguments.
- **peft_config** (`dict[str, Any]`, *optional*) --
  The configuration of the adapter to add, supported adapters are all non-prompt learning configs (LoRA,
  IA³, etc). This argument is used in case users directly pass PEFT state dicts.
- **adapter_state_dict** (`dict[str, torch.Tensor]`, *optional*) --
  The state dict of the adapter to load. This argument is used in case users directly pass PEFT state
  dicts.
- **low_cpu_mem_usage** (`bool`, *optional*, defaults to `False`) --
  Reduce memory usage while loading the PEFT adapter. This should also speed up the loading process.
- **is_trainable** (`bool`, *optional*, defaults to `False`) --
  Whether the adapter should be trainable or not. If `False`, the adapter will be frozen and can only be
  used for inference.
- **hotswap**  -- (`"auto"` or `bool`, *optional*, defaults to `"auto"`)
  Whether to substitute an existing (LoRA) adapter with the newly loaded adapter in-place. This means
  that, instead of loading an additional adapter, this will take the existing adapter weights and replace
  them with the weights of the new adapter. This can be faster and more memory efficient. However, the
  main advantage of hotswapping is that when the model is compiled with torch.compile, loading the new
  adapter does not require recompilation of the model. When using hotswapping, the passed `adapter_name`
  should be the name of an already loaded adapter.

  If the new adapter and the old adapter have different ranks and/or LoRA alphas (i.e. scaling), you need
  to call an additional method before loading the adapter:

```py
model = AutoModel.from_pretrained(...)
max_rank = ...  # the highest rank among all LoRAs that you want to load
# call *before* compiling and loading the LoRA adapter
model.enable_peft_hotswap(target_rank=max_rank)
model.load_adapter(file_name_1, adapter_name="default")
# optionally compile the model now
model = torch.compile(model, ...)
output_1 = model(...)
# now you can hotswap the 2nd adapter, use the same name as for the 1st
# hotswap is activated by default since enable_peft_hotswap was called
model.load_adapter(file_name_2, adapter_name="default")
output_2 = model(...)
```

  By default, hotswap is disabled and requires passing `hotswap=True`. If you called
  `enable_peft_hotswap` first, it is enabled. You can still manually disable it in that case by passing
  `hotswap=False`.

  Note that hotswapping comes with a couple of limitations documented here:
  https://huggingface.co/docs/peft/main/en/package_reference/hotswap
- **adapter_kwargs** (`dict[str, Any]`, *optional*) --
  Additional keyword arguments passed along to the `from_pretrained` method of the adapter config and
  `find_adapter_config_file` method.

Load adapter weights from file or remote Hub folder. If you are not familiar with adapters and PEFT methods, we
invite you to read more about them on PEFT official documentation: https://huggingface.co/docs/peft

Requires PEFT to be installed as a backend to load the adapter weights.

- **adapter_config** (`~peft.PeftConfig`) --
  The configuration of the adapter to add, supported adapters are non-prompt learning methods (LoRA,
  IA³, etc.).
- **adapter_name** (`str`, *optional*, defaults to `"default"`) --
  The name of the adapter to add. If no name is passed, a default name is assigned to the adapter.

If you are not familiar with adapters and PEFT methods, we invite you to read more about them on the PEFT
official documentation: https://huggingface.co/docs/peft

Adds a fresh new adapter to the current model for training purpose. If no adapter name is passed, a default
name is assigned to the adapter to follow the convention of PEFT library (in PEFT we use "default" as the
default adapter name).

Note that the newly added adapter is not automatically activated. To activate it, use `model.set_adapter`.

- **adapter_name** (`Union[list[str], str]`) --
  The name of the adapter to set. Can be also a list of strings to set multiple adapters.

If you are not familiar with adapters and PEFT methods, we invite you to read more about them on the PEFT
official documentation: https://huggingface.co/docs/peft

Sets a specific adapter by forcing the model to use a that adapter and disable the other adapters.

If you are not familiar with adapters and PEFT methods, we invite you to read more about them on the PEFT
official documentation: https://huggingface.co/docs/peft

Disable all adapters that are attached to the model. This leads to inferring with the base model only.

If you are not familiar with adapters and PEFT methods, we invite you to read more about them on the PEFT
official documentation: https://huggingface.co/docs/peft

Enable adapters that are attached to the model.

- **target_rank** (`int`, *optional*, defaults to `128`) --
  The highest rank among all the adapters that will be loaded.
- **check_compiled** (`str`, *optional*, defaults to `"error"`) --
  How to handle the case when the model is already compiled, which should generally be avoided. The
  options are:
  - "error" (default): raise an error
  - "warn": issue a warning
  - "ignore": do nothing
Enables the possibility to hotswap PEFT adapters with different ranks, or, if the model is compiled, without
triggering recompilation.

Right now, hotswapping is only supported for LoRA.

Calling this method is only required when hotswapping adapters and if the model is compiled or if the ranks of
the loaded adapters differ. If the ranks are all identical and the model is not compiled, hotswapping works
without calling this method first.

If you are not familiar with adapters and PEFT methods, we invite you to read more about them on the PEFT
official documentation: https://huggingface.co/docs/peft

Gets the current active adapters of the model. In case of multi-adapter inference (combining multiple adapters
for inference) returns the list of all active adapters so that users can deal with them accordingly.

For previous PEFT versions (that does not support multi-adapter inference), `module.active_adapter` will return
a single string.

- **adapter_name** (`str`, *optional*) --
  The name of the adapter to get the state dict from. If no name is passed, the active adapter is used.
- **state_dict** (nested dictionary of `torch.Tensor`, *optional*) --
  The state dictionary of the model. Will default to `self.state_dict()`, but can be used if special
  precautions need to be taken when recovering the state dictionary of a model (like when using model
  parallelism).

If you are not familiar with adapters and PEFT methods, we invite you to read more about them on the PEFT
official documentation: https://huggingface.co/docs/peft

Gets the adapter state dict that should only contain the weights tensors of the specified adapter_name adapter.
If no adapter_name is passed, the active adapter is used.

- **adapter_names** (`Union[list[str], str]`) --
  The name(s) of the adapter(s) to delete.

Delete a PEFT adapter from the underlying model.
