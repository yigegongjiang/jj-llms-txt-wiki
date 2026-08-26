# SAM-HQ

## Overview

SAM-HQ (High-Quality Segment Anything Model) was proposed in [Segment Anything in High Quality](https://huggingface.co/papers/2306.01567) by Lei Ke, Mingqiao Ye, Martin Danelljan, Yifan Liu, Yu-Wing Tai, Chi-Keung Tang, Fisher Yu.

The model is an enhancement to the original SAM model that produces significantly higher quality segmentation masks while maintaining SAM's original promptable design, efficiency, and zero-shot generalizability.

![example image](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/sam-output.png)

SAM-HQ introduces several key improvements over the original SAM model:

1. High-Quality Output Token: A learnable token injected into SAM's mask decoder for higher quality mask prediction
2. Global-local Feature Fusion: Combines features from different stages of the model for improved mask details
3. Training Data: Uses a carefully curated dataset of 44K high-quality masks instead of SA-1B
4. Efficiency: Adds only 0.5% additional parameters while significantly improving mask quality
5. Zero-shot Capability: Maintains SAM's strong zero-shot performance while improving accuracy

The abstract from the paper is the following:

*The recent Segment Anything Model (SAM) represents a big leap in scaling up segmentation models, allowing for powerful zero-shot capabilities and flexible prompting. Despite being trained with 1.1 billion masks, SAM's mask prediction quality falls short in many cases, particularly when dealing with objects that have intricate structures. We propose HQ-SAM, equipping SAM with the ability to accurately segment any object, while maintaining SAM's original promptable design, efficiency, and zero-shot generalizability. Our careful design reuses and preserves the pre-trained model weights of SAM, while only introducing minimal additional parameters and computation. We design a learnable High-Quality Output Token, which is injected into SAM's mask decoder and is responsible for predicting the high-quality mask. Instead of only applying it on mask-decoder features, we first fuse them with early and final ViT features for improved mask details. To train our introduced learnable parameters, we compose a dataset of 44K fine-grained masks from several sources. HQ-SAM is only trained on the introduced dataset of 44k masks, which takes only 4 hours on 8 GPUs.*

Tips:

- SAM-HQ produces higher quality masks than the original SAM model, particularly for objects with intricate structures and fine details
- The model predicts binary masks with more accurate boundaries and better handling of thin structures
- Like SAM, the model performs better with input 2D points and/or input bounding boxes
- You can prompt multiple points for the same image and predict a single high-quality mask
- The model maintains SAM's zero-shot generalization capabilities
- SAM-HQ only adds ~0.5% additional parameters compared to SAM
- Fine-tuning the model is not supported yet

This model was contributed by [sushmanth](https://huggingface.co/sushmanth).
The original code can be found [here](https://github.com/SysCV/SAM-HQ).

Below is an example on how to run mask generation given an image and a 2D point:

```python
import requests
import torch
from PIL import Image

from transformers import SamHQModel, SamHQProcessor

model = SamHQModel.from_pretrained("syscv-community/sam-hq-vit-base", device_map="auto")
processor = SamHQProcessor.from_pretrained("syscv-community/sam-hq-vit-base")

img_url = "https://huggingface.co/ybelkada/segment-anything/resolve/main/assets/car.png"
raw_image = Image.open(requests.get(img_url, stream=True).raw).convert("RGB")
input_points = [[[450, 600]]]  # 2D location of a window in the image

inputs = processor(raw_image, input_points=input_points, return_tensors="pt").to(model.device)
with torch.no_grad():
    outputs = model(**inputs)

masks = processor.image_processor.post_process_masks(
    outputs.pred_masks.cpu(), inputs["original_sizes"].cpu(), inputs["reshaped_input_sizes"].cpu()
)
scores = outputs.iou_scores
```

You can also process your own masks alongside the input images in the processor to be passed to the model:

```python
import requests
import torch
from PIL import Image

from transformers import SamHQModel, SamHQProcessor

model = SamHQModel.from_pretrained("syscv-community/sam-hq-vit-base", device_map="auto")
processor = SamHQProcessor.from_pretrained("syscv-community/sam-hq-vit-base")

img_url = "https://huggingface.co/ybelkada/segment-anything/resolve/main/assets/car.png"
raw_image = Image.open(requests.get(img_url, stream=True).raw).convert("RGB")
mask_url = "https://huggingface.co/ybelkada/segment-anything/resolve/main/assets/car.png"
segmentation_map = Image.open(requests.get(mask_url, stream=True).raw).convert("1")
input_points = [[[450, 600]]]  # 2D location of a window in the image

inputs = processor(raw_image, input_points=input_points, segmentation_maps=segmentation_map, return_tensors="pt").to(model.device)
with torch.no_grad():
    outputs = model(**inputs)

masks = processor.image_processor.post_process_masks(
    outputs.pred_masks.cpu(), inputs["original_sizes"].cpu(), inputs["reshaped_input_sizes"].cpu()
)
scores = outputs.iou_scores
```

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with SAM-HQ:

- Demo notebook for using the model (coming soon)
- Paper implementation and code: [SAM-HQ GitHub Repository](https://github.com/SysCV/SAM-HQ)

## SamHQConfig[[transformers.SamHQConfig]]

#### transformers.SamHQConfig[[transformers.SamHQConfig]]

```python
transformers.SamHQConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, prompt_encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, mask_decoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, initializer_range: float = 0.02, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam_hq/configuration_sam_hq.py#L153)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

prompt_encoder_config (Union[`dict`, `SamHQPromptEncoderConfig`], *optional*) : Dictionary of configuration options used to initialize [SamHQPromptEncoderConfig](/docs/transformers/v5.15.1/en/model_doc/sam_hq#transformers.SamHQPromptEncoderConfig).

mask_decoder_config (Union[`dict`, `SamHQMaskDecoderConfig`], *optional*) : Dictionary of configuration options used to initialize [SamHQMaskDecoderConfig](/docs/transformers/v5.15.1/en/model_doc/sam_hq#transformers.SamHQMaskDecoderConfig).

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a SamHQModel. It is used to instantiate a Sam Hq
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [syscv-community/sam-hq-vit-base](https://huggingface.co/syscv-community/sam-hq-vit-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## SamHQVisionConfig[[transformers.SamHQVisionConfig]]

#### transformers.SamHQVisionConfig[[transformers.SamHQVisionConfig]]

```python
transformers.SamHQVisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, output_channels: int = 256, num_hidden_layers: int = 12, num_attention_heads: int = 12, num_channels: int = 3, image_size: int | list[int] | tuple[int, int] = 1024, patch_size: int | list[int] | tuple[int, int] = 16, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06, attention_dropout: float | int = 0.0, initializer_range: float = 1e-10, qkv_bias: bool = True, mlp_ratio: float = 4.0, use_abs_pos: bool = True, use_rel_pos: bool = True, window_size: int = 14, global_attn_indexes: list[int] | tuple[int, ...] = (2, 5, 8, 11), num_pos_feats: int = 128, mlp_dim: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam_hq/configuration_sam_hq.py#L54)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

output_channels (`int`, *optional*, defaults to 256) : Dimensionality of the output channels in the Patch Encoder.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1024`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `1e-10`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

mlp_ratio (`float`, *optional*, defaults to `4.0`) : Ratio of the MLP hidden dim to the embedding dim.

use_abs_pos (`bool`, *optional*, defaults to `True`) : Whether to use absolute position embeddings.

use_rel_pos (`bool`, *optional*, defaults to `True`) : Whether to use relative position embedding.

window_size (`int`, *optional*, defaults to 14) : Window size for relative position.

global_attn_indexes (`list[int]`, *optional*, defaults to `[2, 5, 8, 11]`) : The indexes of the global attention layers.

num_pos_feats (`int`, *optional*, defaults to 128) : The dimensionality of the position embedding.

mlp_dim (`int`, *optional*) : The dimensionality of the MLP layer in the Transformer encoder. If `None`, defaults to `mlp_ratio * hidden_size`.

This is the configuration class to store the configuration of a SamHQModel. It is used to instantiate a Sam Hq
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [syscv-community/sam-hq-vit-base](https://huggingface.co/syscv-community/sam-hq-vit-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     SamHQVisionConfig,
...     SamHQVisionModel,
... )

>>> # Initializing a SamHQVisionConfig with `"facebook/sam_hq-vit-huge"` style configuration
>>> configuration = SamHQVisionConfig()

>>> # Initializing a SamHQVisionModel (with random weights) from the `"facebook/sam_hq-vit-huge"` style configuration
>>> model = SamHQVisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SamHQMaskDecoderConfig[[transformers.SamHQMaskDecoderConfig]]

#### transformers.SamHQMaskDecoderConfig[[transformers.SamHQMaskDecoderConfig]]

```python
transformers.SamHQMaskDecoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 256, hidden_act: str = 'relu', mlp_dim: int = 2048, num_hidden_layers: int = 2, num_attention_heads: int = 8, attention_downsample_rate: int = 2, num_multimask_outputs: int = 3, iou_head_depth: int = 3, iou_head_hidden_dim: int = 256, layer_norm_eps: float = 1e-06, vit_dim: int = 768)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam_hq/configuration_sam_hq.py#L119)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

mlp_dim (`int`, *optional*, defaults to 2048) : Dimensionality of the "intermediate" (i.e., feed-forward) layer in the Transformer encoder.

num_hidden_layers (`int`, *optional*, defaults to `2`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

attention_downsample_rate (`int`, *optional*, defaults to 2) : The downsampling rate of the attention layer.

num_multimask_outputs (`int`, *optional*, defaults to 3) : The number of outputs from the `SamMaskDecoder` module. In the Segment Anything paper, this is set to 3.

iou_head_depth (`int`, *optional*, defaults to 3) : The number of layers in the IoU head module.

iou_head_hidden_dim (`int`, *optional*, defaults to 256) : The dimensionality of the hidden states in the IoU head module.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

vit_dim (`int`, *optional*, defaults to 768) : Dimensionality of the Vision Transformer (ViT) used in the `SamHQMaskDecoder` module.

This is the configuration class to store the configuration of a SamHQModel. It is used to instantiate a Sam Hq
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [syscv-community/sam-hq-vit-base](https://huggingface.co/syscv-community/sam-hq-vit-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## SamHQPromptEncoderConfig[[transformers.SamHQPromptEncoderConfig]]

#### transformers.SamHQPromptEncoderConfig[[transformers.SamHQPromptEncoderConfig]]

```python
transformers.SamHQPromptEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 256, image_size: int | list[int] | tuple[int, int] = 1024, patch_size: int | list[int] | tuple[int, int] = 16, mask_input_channels: int = 16, num_point_embeddings: int = 4, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam_hq/configuration_sam_hq.py#L29)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1024`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

mask_input_channels (`int`, *optional*, defaults to 16) : The number of channels to be fed to the `MaskDecoder` module.

num_point_embeddings (`int`, *optional*, defaults to 4) : The number of point embeddings to be used.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

This is the configuration class to store the configuration of a SamHQModel. It is used to instantiate a Sam Hq
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [syscv-community/sam-hq-vit-base](https://huggingface.co/syscv-community/sam-hq-vit-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## SamHQProcessor[[transformers.SamHQProcessor]]

#### transformers.SamHQProcessor[[transformers.SamHQProcessor]]

```python
transformers.SamHQProcessor(image_processor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam_hq/processing_sam_hq.py#L83)

**Parameters:**

image_processor (`SamImageProcessor`) : The image processor is a required input.

Constructs a SamHQProcessor which wraps a image processor into a single processor.

[SamHQProcessor](/docs/transformers/v5.15.1/en/model_doc/sam_hq#transformers.SamHQProcessor) offers all the functionalities of [SamImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam#transformers.SamImageProcessor). See the
[~SamImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam#transformers.SamImageProcessor) for more information.

#### __call__[[transformers.SamHQProcessor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam_hq/processing_sam_hq.py#L93)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

segmentation_maps (`ImageInput`, *kwargs*, *optional*) : Ground truth segmentation maps to process alongside the input images. These maps are used for training or evaluation purposes and are resized and normalized to match the processed image dimensions.

input_points (`NestedList`, *kwargs*, *optional*) : Input points for prompt-based segmentation. Should be a nested list with structure `[image_level, object_level, point_level, [x, y]]` where each point is specified as `[x, y]` coordinates in the original image space. Points are normalized to the target image size before being passed to the model.

input_labels (`NestedList`, *kwargs*, *optional*) : Labels for the input points, indicating whether each point is a foreground (1) or background (0) point. Should be a nested list with structure `[image_level, object_level, point_level]`. Must have the same structure as `input_points` (excluding the coordinate dimension).

input_boxes (`NestedList`, *kwargs*, *optional*) : Bounding boxes for prompt-based segmentation. Should be a nested list with structure `[image_level, box_level, [x1, y1, x2, y2]]` where each box is specified as `[x1, y1, x2, y2]` coordinates in the original image space. Boxes are normalized to the target image size before being passed to the model.

point_pad_value (`int`, *kwargs*, *optional*, defaults to `None`) : The value used for padding input points when batching sequences of different lengths. This value marks padded positions and is preserved during coordinate normalization to distinguish real points from padding. If `None`, the default pad value from the processor configuration is used.

mask_size (`dict[str, *kwargs*, int]`, *optional*) : Dictionary specifying the target mask size with keys `"height"` and `"width"`. This determines the resolution of the output segmentation masks generated by the model.

mask_pad_size (`dict[str, *kwargs*, int]`, *optional*) : Dictionary specifying the padding size for masks with keys `"height"` and `"width"`. This is used when batching masks of different sizes to ensure consistent dimensions.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** `~feature_extraction_utils.BatchFeature`

- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the __call__/pad methods ('input_values', 'attention_mask',
  etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **skip_tensor_conversion** (`list[str]` or `set[str]`, *optional*) -- List or set of keys that should NOT be converted to tensors, even when `tensor_type` is specified.

## SamHQVisionModel[[transformers.SamHQVisionModel]]

#### transformers.SamHQVisionModel[[transformers.SamHQVisionModel]]

```python
transformers.SamHQVisionModel(config: SamHQVisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam_hq/modeling_sam_hq.py#L1075)

**Parameters:**

config ([SamHQVisionConfig](/docs/transformers/v5.15.1/en/model_doc/sam_hq#transformers.SamHQVisionConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The vision model from SamHQ without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SamHQVisionModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam_hq/modeling_sam_hq.py#L1087)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [SamImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam#transformers.SamImageProcessor). See `SamImageProcessor.__call__()` for details ([SamHQProcessor](/docs/transformers/v5.15.1/en/model_doc/sam_hq#transformers.SamHQProcessor) uses [SamImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam#transformers.SamImageProcessor) for processing images).

**Returns:** `SamHQVisionEncoderOutput` or `tuple(torch.FloatTensor)`

A `SamHQVisionEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SamHQConfig](/docs/transformers/v5.15.1/en/model_doc/sam_hq#transformers.SamHQConfig)) and inputs.

The [SamHQVisionModel](/docs/transformers/v5.15.1/en/model_doc/sam_hq#transformers.SamHQVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim)` *optional* returned when model is initialized with `with_projection=True`) -- The image embeddings obtained by applying the projection layer to the pooler_output.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **intermediate_embeddings** (`list(torch.FloatTensor)`, *optional*) -- A list of intermediate embeddings collected from certain blocks within the model, typically those without
  windowed attention. Each element in the list is of shape `(batch_size, sequence_length, hidden_size)`.
  This is specific to SAM-HQ and not present in base SAM.

## SamHQModel[[transformers.SamHQModel]]

#### transformers.SamHQModel[[transformers.SamHQModel]]

```python
transformers.SamHQModel(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam_hq/modeling_sam_hq.py#L1233)

**Parameters:**

config ([SamHQModel](/docs/transformers/v5.15.1/en/model_doc/sam_hq#transformers.SamHQModel)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Segment Anything Model HQ (SAM-HQ) for generating masks, given an input image and optional 2D location and bounding boxes.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SamHQModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.FloatTensor] = None, input_points: typing.Optional[torch.FloatTensor] = None, input_labels: typing.Optional[torch.LongTensor] = None, input_boxes: typing.Optional[torch.FloatTensor] = None, input_masks: typing.Optional[torch.LongTensor] = None, image_embeddings: typing.Optional[torch.FloatTensor] = None, multimask_output: bool = True, hq_token_only: bool = False, attention_similarity: typing.Optional[torch.FloatTensor] = None, target_embedding: typing.Optional[torch.FloatTensor] = None, intermediate_embeddings: list[torch.FloatTensor] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sam_hq/modeling_sam_hq.py#L1317)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [SamImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam#transformers.SamImageProcessor). See `SamImageProcessor.__call__()` for details ([SamHQProcessor](/docs/transformers/v5.15.1/en/model_doc/sam_hq#transformers.SamHQProcessor) uses [SamImageProcessor](/docs/transformers/v5.15.1/en/model_doc/sam#transformers.SamImageProcessor) for processing images).

input_points (`torch.FloatTensor` of shape `(batch_size, num_points, 2)`) : Input 2D spatial points, this is used by the prompt encoder to encode the prompt. Generally yields to much better results. The points can be obtained by passing a list of list of list to the processor that will create corresponding `torch` tensors of dimension 4. The first dimension is the image batch size, the second dimension is the point batch size (i.e. how many segmentation masks do we want the model to predict per input point), the third dimension is the number of points per segmentation mask (it is possible to pass multiple points for a single mask), and the last dimension is the x (vertical) and y (horizontal) coordinates of the point. If a different number of points is passed either for each image, or for each mask, the processor will create "PAD" points that will correspond to the (0, 0) coordinate, and the computation of the embedding will be skipped for these points using the labels.

input_labels (`torch.LongTensor` of shape `(batch_size, point_batch_size, num_points)`) : Input labels for the points, this is used by the prompt encoder to encode the prompt. According to the official implementation, there are 3 types of labels  - `1`: the point is a point that contains the object of interest - `0`: the point is a point that does not contain the object of interest - `-1`: the point corresponds to the background  We added the label:  - `-10`: the point is a padding point, thus should be ignored by the prompt encoder  The padding labels should be automatically done by the processor.

input_boxes (`torch.FloatTensor` of shape `(batch_size, num_boxes, 4)`) : Input boxes for the points, this is used by the prompt encoder to encode the prompt. Generally yields to much better generated masks. The boxes can be obtained by passing a list of list of list to the processor, that will generate a `torch` tensor, with each dimension corresponding respectively to the image batch size, the number of boxes per image and the coordinates of the top left and bottom right point of the box. In the order (`x1`, `y1`, `x2`, `y2`):  - `x1`: the x coordinate of the top left point of the input box - `y1`: the y coordinate of the top left point of the input box - `x2`: the x coordinate of the bottom right point of the input box - `y2`: the y coordinate of the bottom right point of the input box

input_masks (`torch.FloatTensor` of shape `(batch_size, image_size, image_size)`) : SAM_HQ model also accepts segmentation masks as input. The mask will be embedded by the prompt encoder to generate a corresponding embedding, that will be fed later on to the mask decoder. These masks needs to be manually fed by the user, and they need to be of shape (`batch_size`, `image_size`, `image_size`).

image_embeddings (`torch.FloatTensor` of shape `(batch_size, output_channels, window_size, window_size)`) : Image embeddings, this is used by the mask decder to generate masks and iou scores. For more memory efficient computation, users can first retrieve the image embeddings using the `get_image_embeddings` method, and then feed them to the `forward` method instead of feeding the `pixel_values`.

multimask_output (`bool`, *optional*) : In the original implementation and paper, the model always outputs 3 masks per image (or per point / per bounding box if relevant). However, it is possible to just output a single mask, that corresponds to the "best" mask, by specifying `multimask_output=False`.

hq_token_only (`bool`, *optional*, defaults to `False`) : Whether to use only the HQ token path for mask generation. When False, combines both standard and HQ paths. This is specific to SAM-HQ's architecture.

attention_similarity (`torch.FloatTensor`, *optional*) : Attention similarity tensor, to be provided to the mask decoder for target-guided attention in case the model is used for personalization as introduced in [PerSAM](https://huggingface.co/papers/2305.03048).

target_embedding (`torch.FloatTensor`, *optional*) : Embedding of the target concept, to be provided to the mask decoder for target-semantic prompting in case the model is used for personalization as introduced in [PerSAM](https://huggingface.co/papers/2305.03048).

intermediate_embeddings (`List[torch.FloatTensor]`, *optional*) : Intermediate embeddings from vision encoder's non-windowed blocks, used by SAM-HQ for enhanced mask quality. Required when providing pre-computed image_embeddings instead of pixel_values.

**Returns:**

`list[dict[str, torch.Tensor]]`

The [SamHQModel](/docs/transformers/v5.15.1/en/model_doc/sam_hq#transformers.SamHQModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoModel, AutoProcessor

>>> model = AutoModel.from_pretrained("sushmanth/sam_hq_vit_b")
>>> processor = AutoProcessor.from_pretrained("sushmanth/sam_hq_vit_b")

>>> url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/sam-car.png"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read())).convert("RGB")
>>> input_points = [[[400, 650]]]  # 2D location of a window on the car
>>> inputs = processor(images=image, input_points=input_points, return_tensors="pt")

>>> # Get high-quality segmentation mask
>>> outputs = model(**inputs)

>>> # For high-quality mask only
>>> outputs = model(**inputs, hq_token_only=True)

>>> # Postprocess masks
>>> masks = processor.post_process_masks(
...     outputs.pred_masks, inputs["original_sizes"], inputs["reshaped_input_sizes"]
... )
```
