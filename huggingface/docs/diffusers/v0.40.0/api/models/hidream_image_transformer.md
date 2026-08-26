# HiDreamImageTransformer2DModel

A Transformer model for image-like data from [HiDream-I1](https://huggingface.co/HiDream-ai).

The model can be loaded with the following code snippet.

```python
from diffusers import HiDreamImageTransformer2DModel

transformer = HiDreamImageTransformer2DModel.from_pretrained("HiDream-ai/HiDream-I1-Full", subfolder="transformer", dtype=torch.bfloat16)
```

## Loading GGUF quantized checkpoints for HiDream-I1

GGUF checkpoints for the `HiDreamImageTransformer2DModel` can  be loaded using `~FromOriginalModelMixin.from_single_file`

```python
import torch
from diffusers import GGUFQuantizationConfig, HiDreamImageTransformer2DModel

ckpt_path = "https://huggingface.co/city96/HiDream-I1-Dev-gguf/blob/main/hidream-i1-dev-Q2_K.gguf"
transformer = HiDreamImageTransformer2DModel.from_single_file(
    ckpt_path,
    quantization_config=GGUFQuantizationConfig(compute_dtype=torch.bfloat16),
    dtype=torch.bfloat16
)
```

## HiDreamImageTransformer2DModel[[diffusers.HiDreamImageTransformer2DModel]]

#### diffusers.HiDreamImageTransformer2DModel[[diffusers.HiDreamImageTransformer2DModel]]

```python
diffusers.HiDreamImageTransformer2DModel(patch_size: int | None = None, in_channels: int = 64, out_channels: int | None = None, num_layers: int = 16, num_single_layers: int = 32, attention_head_dim: int = 128, num_attention_heads: int = 20, caption_channels: list = None, text_emb_dim: int = 2048, num_routed_experts: int = 4, num_activated_experts: int = 2, axes_dims_rope: tuple = (32, 32), max_resolution: tuple = (128, 128), llama_layers: list = None, force_inference_output: bool = False)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_hidream_image.py#L602)

#### forward[[diffusers.HiDreamImageTransformer2DModel.forward]]

```python
forward(hidden_states: Tensor, timesteps: LongTensor = None, encoder_hidden_states_t5: Tensor = None, encoder_hidden_states_llama3: Tensor = None, pooled_embeds: Tensor = None, img_ids: typing.Optional[torch.Tensor] = None, img_sizes: list[tuple[int, int]] | None = None, hidden_states_masks: typing.Optional[torch.Tensor] = None, attention_kwargs: dict[str, typing.Any] | None = None, return_dict: bool = True, **kwargs)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/transformers/transformer_hidream_image.py#L773)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(batch_size, in_channels, height, width)` or `(batch_size, patch_height * patch_width, patch_size * patch_size * channels)`) : Input `hidden_states`.

timesteps (`torch.LongTensor`) : Used to indicate denoising step.

encoder_hidden_states_t5 (`torch.Tensor`) : Conditional embeddings computed from the T5 text encoder.

encoder_hidden_states_llama3 (`torch.Tensor`) : Conditional embeddings computed from the Llama3 text encoder.

pooled_embeds (`torch.Tensor`) : Pooled text embeddings used for additional conditioning.

img_ids (`torch.Tensor`, *optional*) : Image position ids for the patched hidden states.

img_sizes (`list` of `tuple` of `int`, *optional*) : Per-sample patch grid sizes used to unpatchify the output.

hidden_states_masks (`torch.Tensor`, *optional*) : Mask over patched `hidden_states`.

attention_kwargs (`dict`, *optional*) : A kwargs dictionary that if specified is passed along to the `AttentionProcessor` as defined under `self.processor` in [diffusers.models.attention_processor](https://github.com/huggingface/diffusers/blob/main/src/diffusers/models/attention_processor.py).

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `~models.transformer_2d.Transformer2DModelOutput` instead of a plain tuple.

**Returns:**

If `return_dict` is True, an `~models.transformer_2d.Transformer2DModelOutput` is returned, otherwise a
`tuple` where the first element is the sample tensor.

The [HiDreamImageTransformer2DModel](/docs/diffusers/v0.40.0/en/api/models/hidream_image_transformer#diffusers.HiDreamImageTransformer2DModel) forward method.

## Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

#### diffusers.models.modeling_outputs.Transformer2DModelOutput[[diffusers.models.modeling_outputs.Transformer2DModelOutput]]

```python
diffusers.models.modeling_outputs.Transformer2DModelOutput(sample: torch.Tensor)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L21)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)` or `(batch size, num_vector_embeds - 1, num_latent_pixels)` if [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel) is discrete) : The hidden states output conditioned on the `encoder_hidden_states` input. If discrete, returns probability distributions for the unnoised latent pixels.

The output of [Transformer2DModel](/docs/diffusers/v0.40.0/en/api/models/transformer2d#diffusers.Transformer2DModel).
