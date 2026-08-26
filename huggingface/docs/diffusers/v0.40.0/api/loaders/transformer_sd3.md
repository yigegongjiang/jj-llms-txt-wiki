# SD3Transformer2D

This class is useful when *only* loading weights into a [SD3Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/sd3_transformer2d#diffusers.SD3Transformer2DModel). If you need to load weights into the text encoder or a text encoder and SD3Transformer2DModel, check [`SD3LoraLoaderMixin`](lora#diffusers.loaders.SD3LoraLoaderMixin) class instead.

The `SD3Transformer2DLoadersMixin` class currently only loads IP-Adapter weights, but will be used in the future to save weights and load LoRAs.

> [!TIP]
> To learn more about how to load LoRA weights, see the [LoRA](../../tutorials/using_peft_for_inference) loading guide.

## SD3Transformer2DLoadersMixin[[diffusers.loaders.SD3Transformer2DLoadersMixin]]

#### diffusers.loaders.SD3Transformer2DLoadersMixin[[diffusers.loaders.SD3Transformer2DLoadersMixin]]

```python
diffusers.loaders.SD3Transformer2DLoadersMixin()
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/transformer_sd3.py#L27)

Load IP-Adapters and LoRA layers into a `[SD3Transformer2DModel]`.

#### _load_ip_adapter_weights[[diffusers.loaders.SD3Transformer2DLoadersMixin._load_ip_adapter_weights]]

```python
_load_ip_adapter_weights(state_dict: dict, low_cpu_mem_usage: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/loaders/transformer_sd3.py#L157)

**Parameters:**

state_dict (`Dict`) : State dict with keys "ip_adapter", which contains parameters for attention processors, and "image_proj", which contains parameters for image projection net.

low_cpu_mem_usage (`bool`, *optional*, defaults to `True` if torch version >= 1.9.0 else `False`) : Speed up model loading only loading the pretrained weights and not initializing the weights. This also tries to not use more than 1x model size in CPU memory (including peak memory) while loading the model. Only supported for PyTorch >= 1.9.0. If you are using an older version of PyTorch, setting this argument to `True` will raise an error.

Sets IP-Adapter attention processors, image projection, and loads state_dict.
