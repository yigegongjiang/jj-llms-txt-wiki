# RADIO

[RADIO](https://huggingface.co/papers/2312.06709) (Reduce All Domains Into One) is a family of vision foundation models from NVIDIA trained by multi-teacher distillation (e.g. CLIP, DINOv2, SAM) into a single ViT backbone. It produces both an image-level `summary` embedding and dense spatial `features`, and supports variable input resolutions through a Cropped Position Embedding (CPE) patch generator.

The example below demonstrates how to extract image features with the [RadioModel](/docs/transformers/v5.15.1/en/model_doc/radio#transformers.RadioModel) class.

```python
import requests
import torch
from PIL import Image

from transformers import CLIPImageProcessor, RadioModel

hf_repo = "nvidia/C-RADIOv4-H"

device = torch.accelerator.current_accelerator().type if torch.accelerator.is_available() else "cpu"

model = RadioModel.from_pretrained(hf_repo)
model.eval().to(device)

image_processor = CLIPImageProcessor(
    size={"height": 224, "width": 224}, do_resize=True, do_center_crop=False, do_normalize=False
)

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw).convert("RGB")
pixel_values = image_processor(images=image, return_tensors="pt").pixel_values
pixel_values = pixel_values.to(device)

with torch.no_grad():
    outputs = model(pixel_values)

summary = outputs.summary    # (1, 2560) image-level embedding
features = outputs.features   # (1, 196, 1280) dense spatial features
```

## RadioConfig[[transformers.RadioConfig]]

#### transformers.RadioConfig[[transformers.RadioConfig]]

```python
transformers.RadioConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 1280, num_hidden_layers: int = 32, num_attention_heads: int = 16, mlp_ratio: float = 4.0, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06, attention_probs_dropout_prob: float = 0.0, hidden_dropout_prob: float = 0.0, drop_path_rate: float = 0.0, use_swiglu_ffn: bool = False, qkv_bias: bool = True, layerscale_value: float = 1.0, num_channels: int = 3, patch_size: int = 16, image_size: int = 224, max_img_size: int = 2048, num_cls_tokens: int = 3, num_registers: int = 7, summary_idxs: list[int] | None = None, norm_mean: list[float] | tuple[float, float, float] = (0.48145466, 0.4578275, 0.40821073), norm_std: list[float] | tuple[float, float, float] = (0.26862954, 0.26130258, 0.27577711), initializer_range: float = 0.02)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/radio/configuration_radio.py#L28)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `1280`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

mlp_ratio (`float`, *optional*, defaults to 4.0) : Ratio of the hidden size of the MLP relative to `hidden_size`.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

attention_probs_dropout_prob (`float`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

hidden_dropout_prob (`float`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

drop_path_rate (`float`, *optional*, defaults to `0.0`) : Drop path rate for the patch fusion.

use_swiglu_ffn (`bool`, *optional*, defaults to `False`) : Whether to use a SwiGLU feed-forward network in the encoder layers instead of the standard MLP.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

layerscale_value (`float`, *optional*, defaults to 1.0) : Initial value for the LayerScale parameters. C-RADIO has no LayerScale; the default of `1.0` makes the (inherited) LayerScale an identity operation.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

patch_size (`int`, *optional*, defaults to `16`) : The size (resolution) of each patch.

image_size (`int`, *optional*, defaults to `224`) : The size (resolution) of each image.

max_img_size (`int`, *optional*, defaults to 2048) : Maximum supported image size (in pixels) used to size the position embedding table of the CPE patch generator.

num_cls_tokens (`int`, *optional*, defaults to 3) : Number of learned class (summary) tokens prepended to the patch sequence.

num_registers (`int`, *optional*, defaults to 7) : Number of learned register tokens prepended to the patch sequence.

summary_idxs (`list[int]`, *optional*, defaults to `[0, 1]`) : Indices of the class tokens to gather and flatten into the `summary` output embedding.

norm_mean (`tuple[float, float, float]`, *optional*, defaults to `OPENAI_CLIP_MEAN`) : Per-channel mean used by the input conditioner to normalize pixel values.

norm_std (`tuple[float, float, float]`, *optional*, defaults to `OPENAI_CLIP_STD`) : Per-channel standard deviation used by the input conditioner to normalize pixel values.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a RadioModel. It is used to instantiate a Radio
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [nvidia/C-RADIOv4-H](https://huggingface.co/nvidia/C-RADIOv4-H)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## RadioModel[[transformers.RadioModel]]

#### transformers.RadioModel[[transformers.RadioModel]]

```python
transformers.RadioModel(config: RadioConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/radio/modeling_radio.py#L413)

**Parameters:**

config ([RadioConfig](/docs/transformers/v5.15.1/en/model_doc/radio#transformers.RadioConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Radio Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.RadioModel.forward]]

```python
forward(pixel_values: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/radio/modeling_radio.py#L433)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

**Returns:** `RadioModelOutput` or `tuple(torch.FloatTensor)`

A `RadioModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([RadioConfig](/docs/transformers/v5.15.1/en/model_doc/radio#transformers.RadioConfig)) and inputs.

The [RadioModel](/docs/transformers/v5.15.1/en/model_doc/radio#transformers.RadioModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **summary** (`torch.FloatTensor` of shape `(batch_size, num_summary_idxs * hidden_size)`) -- Flattened summary embedding, gathered from the cls tokens selected by `config.summary_idxs`.
- **features** (`torch.FloatTensor` of shape `(batch_size, num_patches, hidden_size)`) -- Dense spatial patch features.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Full token sequence (prefix tokens + patches) from the final encoder layer.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True`) -- Tuple of `(batch_size, sequence_length, hidden_size)` tensors, one for the embedding output plus one for
  each encoder layer.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True`) -- Tuple of `(batch_size, num_heads, sequence_length, sequence_length)` attention weights, one per layer.
