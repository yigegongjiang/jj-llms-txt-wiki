# Tuners

A tuner (or adapter) is a module that can be plugged into a `torch.nn.Module`. `BaseTuner` base class for other tuners and provides shared methods and attributes for preparing an adapter configuration and replacing a target module with the adapter module. `BaseTunerLayer` is a base class for adapter layers. It offers methods and attributes for managing adapters such as activating and disabling adapters.

## BaseTuner[[peft.tuners.tuners_utils.BaseTuner]]

- **model** (`torch.nn.Module`) --
  The model to which the adapter tuner layers will be attached.
- **forward** (`Callable`) --
  The forward method of the model.
- **peft_config** (`Union[`PeftConfig`, dict[str, PeftConfig]]`) --
  The adapter configuration object, it should be a dictionary of `str` to `PeftConfig` objects. One can also
  pass a PeftConfig object and a new adapter will be created with the default name `adapter` or create a new
  dictionary with a key `adapter_name` and a value of that peft config.
- **config** (`dict[str, Any]`) --
  The model configuration object, it should be a dictionary of `str` to `Any` objects.
- **targeted_module_names** (`list[str]`) --
  The list of module names that were actually adapted. Can be useful to inspect if you want to quickly
  double-check that the `config.target_modules` were specified correctly.
- **targeted_parameter_names** (`list[str]`) --
  The list of parameter names that were actually adapted. Can be useful to inspect if you want to quickly
  double-check that the `config.target_parameters` were specified correctly.
- **prefix** (`str`) --
  The PEFT-method specific unique prefix. E.g. `"lora_"` for LoRA.

A base tuner model that provides the common methods and attributes for all tuners that are injectable into a
torch.nn.Module

For adding a new Tuner class, one needs to overwrite the following methods:

- **_prepare_adapter_config**:
  A private method to eventually prepare the adapter config, for example in case the field `target_modules` is
  missing.
- **_create_and_replace**:
  A private method to create and replace the target module with the adapter module.
- **_check_target_module_exists**:
  A private helper method to check if the passed module's key name matches any of the target modules in the
  adapter_config.

The easiest is to check what is done in the `peft.tuners.lora.LoraModel` class.

- **adapter_name** (str) -- Name of the adapter to be deleted.

Deletes an existing adapter.

Disable all adapters in-place.

When disabling all adapters, the model output corresponds to the output of the base model.

Enable all adapters in-place

- **model** (`nn.Module`) --
  Model to get the config from.
- **default** (`dict|None`, *optional*) --:
  What to return if model does not have a config attribute.

This method gets the config from a model in dictionary form. If model has not attribute config, then this
method returns a default config.

- **model** (`nn.Module`) --
  The model to be tuned.
- **adapter_name** (`str`) --
  The adapter name.
- **autocast_adapter_dtype** (`bool`, *optional*) --
  Whether to autocast the adapter dtype. Defaults to `True`.
- **low_cpu_mem_usage** (`bool`, `optional`, defaults to `False`) --
  Create empty adapter weights on meta device. Useful to speed up the loading process.
- **state_dict** (`dict`, *optional*, defaults to `None`) --
  If a state_dict is passed here, the adapters will be injected based on the entries of the state_dict.
  This can be useful when the exact `target_modules` of the PEFT method is unknown, for instance because
  the checkpoint was created without meta data. Note that the values from the state_dict are not used,
  only the keys are used to determine the correct layers that should be adapted.

Creates adapter layers and replaces the target modules with the adapter layers. This method is called under the
hood by `peft.mapping.get_peft_model` if a non-prompt tuning adapter class is passed.

The corresponding PEFT config is directly retrieved from the `peft_config` attribute of the BaseTuner class.

- **adapter_names** (`list[str]`, *optional*) --
  The list of adapter names that should be merged. If `None`, all active adapters will be merged.
  Defaults to `None`.
- **safe_merge** (`bool`, *optional*) --
  If `True`, the merge operation will be performed in a copy of the original weights and check for NaNs
  before merging the weights. This is useful if you want to check if the merge operation will produce
  NaNs. Defaults to `False`.

This method merges the adapter layers into the base model.

Merging adapters can lead to a speed up of the forward pass. A copy of the adapter weights is still kept in
memory, which is required to unmerge the adapters. In order to merge the adapter weights without keeping them
in memory, please call `merge_and_unload`.

- **progressbar** (`bool`) --
  whether to show a progressbar indicating the unload and merge process (default: False).
- **safe_merge** (`bool`) --
  whether to activate the safe merging check to check if there is any potential Nan in the adapter
  weights.
- **adapter_names** (`List[str]`, *optional*) --
  The list of adapter names that should be merged. If None, all active adapters will be merged. Defaults
  to `None`.

This method merges the adapter layers into the base model.

This is needed if someone wants to use the base model as a standalone model. The returned model has the same
architecture as the original base model.

It is important to assign the returned model to a variable and use it, this is not an in-place operation!

Example:

```py
>>> from transformers import AutoModelForCausalLM
>>> from peft import PeftModel

>>> model_id = ...
>>> base_model = AutoModelForCausalLM.from_pretrained(model_id)
>>> peft_model_id = ...
>>> model = PeftModel.from_pretrained(base_model, peft_model_id)
>>> merged_model = model.merge_and_unload()
```

- **adapter_name** (str, list[str]) --
  The name(s) of the adapter(s) to set as active
- **inference_mode** (bool, optional) --
  Whether the activated adapter should be frozen (i.e. `requires_grad=False`). Default is False.
Set the active adapter(s).

- **adapter_names** (`str` or `Sequence[str]`) --
  The name of the adapter(s) whose gradients should be enabled/disabled.
- **requires_grad** (`bool`, *optional*) --
  Whether to enable (`True`, default) or disable (`False`).

Enable or disable gradients on the given adapter(s).

Whether it is possible for the adapter of this model to be converted to LoRA.

Normally, this works if the PEFT method is additive, i.e. W' = W_base + delta_weight.

Return the base model by removing all the PEFT modules.

It is important to assign the returned model to a variable and use it, this is not an in-place operation!

This method unmerges all merged adapter layers from the base model.

## BaseTunerLayer[[peft.tuners.tuners_utils.BaseTunerLayer]]

- **is_pluggable** (`bool`, *optional*) --
  Whether the adapter layer can be plugged to any pytorch module
- **active_adapters** (Union[List`str`, `str`], *optional*) --
  The name of the active adapter.

A tuner layer mixin that provides the common methods and attributes for all tuners.

- **adapter_name** (`str`) -- The name of the adapter to delete

Delete an adapter from the layer

This should be called on all adapter layers, or else we will get an inconsistent state.

This method will also set a new active adapter if the deleted adapter was an active adapter. It is important
that the new adapter is chosen in a deterministic way, so that the same adapter is chosen on all layers.

- **enabled** (bool) -- True to enable adapters, False to disable adapters
Toggle the enabling and disabling of adapters

Takes care of setting the requires_grad flag for the adapter weights.

(Recursively) get the base_layer.

This is necessary for the case that the tuner layer wraps another tuner layer.

Return the weight of the base layer.

This takes care of potentially dequantizing the weight if it is quantized.

- **adapter_names** (`str` or `list[str]`) --
  The name(s) of the adapter(s) to set as active.
- **inference_mode** (bool, optional) --
  Whether the activated adapter should be frozen (i.e. `requires_grad=False`). Default is False.
Set the active adapter(s).

Additionally, this function will set the specified adapter to trainable (i.e., requires_grad=True) unless
inference_mode is True.

Sets the base weight of the base layer to the new tensor

This works also with quantized weights.

- **adapter_names** (`str` or `Sequence[str]`) --
  The name of the adapter(s) whose gradients should be enabled/disabled.
- **requires_grad** (`bool`, *optional*) --
  Whether to enable (`True`, default) or disable (`False`).

Enable or disable gradients on the given adapter(s).

Whether it is possible for this layer type to be converted to LoRA.

Normally, this works if the PEFT method is additive, i.e. W' = W_base + delta_weight.
