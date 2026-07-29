# Functions for PEFT integration

A collection of functions that could be useful for non-PeftModel models, e.g. transformers or diffusers integration

The functions provided here can be considered "public API" of PEFT and hence are safe to be used by packages that provide PEFT integrations.

## Cast the adapter weight dtypes[[peft.tuners.tuners_utils.cast_adapter_dtype]]

- **adapter_name** (`str`) --
  The adapter name.
- **autocast_adapter_dtype** (`bool`, *optional*) --
  Whether to autocast the adapter dtype. Defaults to `True`.

A helper method to cast the adapter weights to the correct dtype.

Currently, this only upcasts float dtypes to float32.

## Delete the PEFT adapter from model[[peft.tuners.tuners_utils.delete_adapter]]

"}]}>
- **model** (`nn.Module`) --
  The model from which the adapter should be deleted.
- **adapter_name** (str) --
  The name of the adapter to be deleted.
- **prefix** (str) --
  The prefix of the PEFT method, e.g. "lora_" for LoRA.
- **layer_cls** (type, optional) --
  The class of the adapter layer. Defaults to `BaseTunerLayer`.new_adapter (list[str] | None)The name of remaining adapter(s) after deletion, or `None` if there are no active adapters left. Use this
to set the new active adapter of the model if necessary.

Delete an existing PEFT adapter.

Note: This function does not delete the PEFT config on the model, if there is one. It will also not completely
purge the PEFT layers if the last PEFT adapter is deleted. For this, consider using `model.unload()` if using a
PEFT model instance, or just reloading the base model.

## Get the state dict of the PEFT adapter[[peft.get_peft_model_state_dict]]

- **model** ([PeftModel](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel)) -- The Peft model. When using torch.nn.DistributedDataParallel, DeepSpeed or FSDP,
  the model should be the underlying model/unwrapped model (i.e. model.module).
- **state_dict** (`dict`, *optional*, defaults to `None`) --
  The state dict of the model. If not provided, the state dict of the passed model will be used.
- **adapter_name** (`str`, *optional*, defaults to `"default"`) --
  The name of the adapter whose state dict should be returned.
- **unwrap_compiled** (`bool`, *optional*, defaults to `False`) --
  Whether to unwrap the model if torch.compile was used.
- **save_embedding_layers** (`Union[bool, str]`, , *optional*, defaults to `auto`) --
  If `True`, save the embedding layers in addition to adapter weights. If `auto`, checks the common embedding
  layers `peft.utils.other.EMBEDDING_LAYER_NAMES` in config's `target_modules` when available. Based on it
  sets the boolean flag. This only works for 🤗 transformers models.

Get the state dict of the given adapter of the PEFT model.

This only includes the PEFT parameters, not the parameters of the base model. Thus the returned `state_dict` is
generally small compared to the full model size. To retrieve the full `state_dict`, just call `model.state_dict()`.

Note that the adapter name is removed from the `state_dict`, as this is just an arbitrary name that can be changed
when loading the adapter. So e.g. if the adapter name is `'default'` and the original key is
`'model.q_proj.lora_A.default.weight'`, the returned key will be `'model.q_proj.lora_A.weight'`. Use this function
in conjunction with [set_peft_model_state_dict()](/docs/peft/v0.20.0/en/package_reference/functional#peft.set_peft_model_state_dict) to take care of the adapter name when loading weights.

## Inject a PEFT adapter into the model based on a PEFT config[[peft.inject_adapter_in_model]]

- **peft_config** (`PeftConfig`) --
  Configuration object containing the parameters of the PEFT model.
- **model** (`torch.nn.Module`) --
  The input model where the adapter will be injected.
- **adapter_name** (`str`, `optional`, defaults to `"default"`) --
  The name of the adapter to be injected, if not provided, the default adapter name is used ("default").
- **low_cpu_mem_usage** (`bool`, `optional`, defaults to `False`) --
  Create empty adapter weights on meta device. Useful to speed up the loading process.
- **state_dict** (`dict`, *optional*, defaults to `None`) --
  If a `state_dict` is passed here, the adapters will be injected based on the entries of the state_dict.
  This can be useful when the exact `target_modules` of the PEFT method is unknown, for instance because the
  checkpoint was created without meta data. Note that the values from the `state_dict` are not used, only the
  keys are used to determine the correct layers that should be adapted.

Create PEFT layers and inject them into the model in-place.

Currently the API does not support prompt learning methods and adaption prompt.

This function is similar to [get_peft_model()](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.get_peft_model) but it does not return a [PeftModel](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel) instance. Instead, it returns
the original, mutated instance of the passed model.

## Set the active PEFT adapter(s) of the model[[peft.tuners.tuners_utils.set_adapter]]

"}]}>
- **model** (`nn.Module`) --
  The model on which the adapter(s) should be set.
- **adapter_name** (str, list[str]) --
  The name(s) of the adapter(s) to set as active
- **inference_mode** (bool, optional) --
  Whether the activated adapter should be frozen (i.e. `requires_grad=False`). Default is False.
- **layer_cls** (type, optional) --
  The class of the adapter layer. Defaults to `BaseTunerLayer`.
Set the active PEFT adapter(s) of the model.

Active adapters are those adapters that participate in the forward pass. Use this function if you want to switch
between multiple PEFT adapters.

## Set the `requires_grad` attribute of the specified adapters[[peft.tuners.tuners_utils.set_requires_grad]]

- **model** (`nn.Module`) --
  The model whose adapter gradient requirements should be updated.
- **adapter_names** (`str` or `Sequence[str]`) --
  The name of the adapter(s) whose gradients should be enabled/disabled.
- **requires_grad** (`bool`, *optional*) --
  Whether to enable (`True`, default) or disable (`False`).

Enable or disable gradients on the given adapter(s).

## Load the weights of the PEFT state dict into the model[[peft.set_peft_model_state_dict]]

- **model** ([PeftModel](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel)) --
  The Peft model.
- **peft_model_state_dict** (`dict`) --
  The state dict of the Peft model.
- **adapter_name** (`str`, *optional*, defaults to `"default"`) --
  The name of the adapter whose state dict should be set.
- **ignore_mismatched_sizes** (`bool`, *optional*, defaults to `False`) --
  Whether to ignore mismatched in the state dict.
- **low_cpu_mem_usage** (`bool`, `optional`, defaults to `False`) --
  This argument must be `True` if the `model` was loaded with adapter weights on the meta device, e.g. after
  calling `inject_adapter_in_model` with `low_cpu_mem_usage=True`. Otherwise, leave it as `False`.load_result (`_IncompatibleKeys`)
A named tuple with `missing_keys` and `unexpected_keys` fields.

Set the state dict of the PEFT model.

Given a PEFT `state_dict` (as returned by [get_peft_model_state_dict()](/docs/peft/v0.20.0/en/package_reference/functional#peft.get_peft_model_state_dict)), insert the weights into the model. The
model needs to have the PEFT adapters already in place (e.g. via [inject_adapter_in_model()](/docs/peft/v0.20.0/en/package_reference/functional#peft.inject_adapter_in_model)).

Setting the adapter weights also takes care of re-inserting the adapter name. This name may be a different name
than the one originally used to train the adapter.
