# LyCORIS

[LyCORIS](https://hf.co/papers/2309.14859) (Lora beYond Conventional methods, Other Rank adaptation Implementations for Stable diffusion) are LoRA-like matrix decomposition adapters that modify the cross-attention layer of the UNet. The [LoHa](loha) and [LoKr](lokr) methods inherit from the `Lycoris` classes here.

## LycorisConfig[[peft.tuners.lycoris_utils.LycorisConfig]]

#### peft.tuners.lycoris_utils.LycorisConfig[[peft.tuners.lycoris_utils.LycorisConfig]]

```python
peft.tuners.lycoris_utils.LycorisConfig(task_type: Optional[Union[str, TaskType]] = None, peft_type: Optional[Union[str, PeftType]] = None, auto_mapping: Optional[dict] = None, peft_version: Optional[str] = None, base_model_name_or_path: Optional[str] = None, revision: Optional[str] = None, inference_mode: bool = False, rank_pattern: Optional[dict] = <factory>, alpha_pattern: Optional[dict] = <factory>)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/lycoris_utils.py#L35)

A base config for LyCORIS like adapters

## LycorisLayer[[peft.tuners.lycoris_utils.LycorisLayer]]

#### peft.tuners.lycoris_utils.LycorisLayer[[peft.tuners.lycoris_utils.LycorisLayer]]

```python
peft.tuners.lycoris_utils.LycorisLayer(base_layer: nn.Module)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/lycoris_utils.py#L60)

A base layer for LyCORIS like adapters

#### merge[[peft.tuners.lycoris_utils.LycorisLayer.merge]]

```python
merge(safe_merge: bool = False, adapter_names: Optional[list[str]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/lycoris_utils.py#L114)

**Parameters:**

safe_merge (`bool`, *optional*) : If `True`, the merge operation will be performed in a copy of the original weights and check for NaNs before merging the weights. This is useful if you want to check if the merge operation will produce NaNs. Defaults to `False`.

adapter_names (`List[str]`, *optional*) : The list of adapter names that should be merged. If `None`, all active adapters will be merged. Defaults to `None`.

Merge the active adapter weights into the base weights

#### unmerge[[peft.tuners.lycoris_utils.LycorisLayer.unmerge]]

```python
unmerge()
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/lycoris_utils.py#L168)

This method unmerges all merged adapter layers from the base weights.

## LycorisTuner[[peft.tuners.lycoris_utils.LycorisTuner]]

#### peft.tuners.lycoris_utils.LycorisTuner[[peft.tuners.lycoris_utils.LycorisTuner]]

```python
peft.tuners.lycoris_utils.LycorisTuner(model, peft_config: Union[PeftConfig, dict[str, PeftConfig]], adapter_name: str, low_cpu_mem_usage: bool = False, state_dict: Optional[dict[str, torch.Tensor]] = None)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/tuners/lycoris_utils.py#L194)

**Parameters:**

model (`torch.nn.Module`) : The model to be adapted.

config ([LoraConfig](/docs/peft/v0.20.0/en/package_reference/lora#peft.LoraConfig)) : The configuration of the Lora model.

adapter_name (`str`) : The name of the adapter, defaults to `"default"`.

low_cpu_mem_usage (`bool`, `optional`, defaults to `False`) : Create empty adapter weights on meta device. Useful to speed up the loading process.

A base tuner for LyCORIS like adapters
