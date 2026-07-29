# VidEoMT

## Overview

The VidEoMT model was proposed in [Your ViT is Secretly Also a Video Segmentation Model](https://huggingface.co/papers/2602.17807) by Narges Norouzi, Idil Esen Zulfikar, Niccolò Cavagnero, Tommie Kerssies, Bastian Leibe, Gijs Dubbelman, Daan de Geus. Video Encoder-only Mask Transformer (VidEoMT) is a lightweight encoder-only model for online video segmentation built on a plain [Vision Transformer (ViT)](vit). It is a minimal extension of [EoMT](./eomt) to video which performs both spatial and temporal reasoning within the ViT encoder, without relying on dedicated tracking modules or heavy task-specific heads.

The abstract from the paper is the following:

*Existing online video segmentation models typically combine a per-frame segmenter with complex specialized tracking modules. While effective, these modules introduce significant architectural complexity and computational overhead. Recent studies suggest that plain Vision Transformer (ViT) encoders, when scaled with sufficient capacity and large-scale pre-training, can conduct accurate image segmentation without requiring specialized modules. Motivated by this observation, we propose the Video Encoder-only Mask Transformer (VidEoMT), a simple encoder-only video segmentation model that eliminates the need for dedicated tracking modules. To enable temporal modeling in an encoder-only ViT, VidEoMT introduces a lightweight query propagation mechanism that carries information across frames by reusing queries from the previous frame. To balance this with adaptability to new content, it employs a query fusion strategy that combines the propagated queries with a set of temporally-agnostic learned queries. As a result, VidEoMT attains the benefits of a tracker without added complexity, achieving competitive accuracy while being 5x--10x faster, running at up to 160 FPS with a ViT-L backbone.*

Tips:

- VidEoMT currently only supports a DINOv2 backbone (with register tokens). Available model sizes are ViT-S, ViT-B, and ViT-L.
- The model accepts video input as a 5D tensor of shape `(batch_size, num_frames, 3, height, width)`.
- VidEoMT supports three video segmentation tasks: **instance**, **semantic**, and **panoptic** segmentation, each with a dedicated post-processing method on the video processor.

This model was contributed by [nielsr](https://huggingface.co/nielsr).
The original code can be found [here](https://github.com/tue-mps/videomt).

## Architecture Info

VidEoMT builds on [EoMT](./eomt), which repurposes a plain DINOv2-pretrained Vision Transformer with **register tokens** as a segmentation model. EoMT introduces learned **object queries** and a lightweight **mask prediction head** directly inside the ViT encoder, eliminating the need for task-specific decoders.

VidEoMT extends this to video with two key additions:

1. **Query propagation**: object queries from the previous frame are carried forward to the next frame through a linear projection (`query_updater`), enabling temporal reasoning without a dedicated tracker.
2. **Query fusion**: the propagated queries are added to a set of temporally-agnostic learned queries, allowing the model to adapt to new objects appearing in the video.

The early encoder layers process all frames independently (in parallel), while the final blocks operate per-frame with the fused queries, producing per-frame mask and class predictions.

## Usage Examples

Use the Hugging Face implementation of VidEoMT for inference with pre-trained models. The examples below reuse the public `tue-mps/videomt-dinov2-small-ytvis2019` checkpoint to demonstrate video instance, semantic, and panoptic post-processing on a sample video.

### Video Instance Segmentation

```python
import matplotlib.pyplot as plt
import numpy as np
import torch

from transformers import AutoModelForUniversalSegmentation, AutoVideoProcessor
from transformers.video_utils import load_video

model_id = "tue-mps/videomt-dinov2-small-ytvis2019"
processor = AutoVideoProcessor.from_pretrained(model_id)
model = AutoModelForUniversalSegmentation.from_pretrained(model_id, device_map="auto")

video_url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/videos/pexels-allan-mas-5362370.mp4"
# Sample 8 frames to keep the example lightweight.
video_frames, _ = load_video(video_url, num_frames=8)

inputs = processor(videos=[video_frames], return_tensors="pt").to(model.device)

with torch.inference_mode():
    outputs = model(**inputs)

original_height, original_width = video_frames[0].shape[:2]
target_sizes = [(original_height, original_width)] * len(video_frames)

results = processor.post_process_instance_segmentation(
    outputs,
    target_sizes=target_sizes,
)

fig, axes = plt.subplots(2, 4, figsize=(16, 8))
for idx, (ax, frame, result) in enumerate(zip(axes.flatten(), video_frames, results)):
    ax.imshow(frame)
    seg = result["segmentation"].cpu().numpy()
    masked = np.ma.masked_where(seg == -1, seg)
    ax.imshow(masked, alpha=0.6, cmap="tab20")
    ax.set_title(f"Frame {idx}")
    ax.axis("off")
plt.suptitle("Video Instance Segmentation")
plt.tight_layout()
plt.show()
```

### Video Semantic Segmentation

```python
import matplotlib.pyplot as plt
import torch

from transformers import AutoModelForUniversalSegmentation, AutoVideoProcessor
from transformers.video_utils import load_video

model_id = "tue-mps/videomt-dinov2-small-ytvis2019"
processor = AutoVideoProcessor.from_pretrained(model_id)
model = AutoModelForUniversalSegmentation.from_pretrained(model_id, device_map="auto")

video_url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/videos/pexels-allan-mas-5362370.mp4"
# Sample 8 frames to keep the example lightweight.
video_frames, _ = load_video(video_url, num_frames=8)

inputs = processor(videos=[video_frames], return_tensors="pt").to(model.device)

with torch.inference_mode():
    outputs = model(**inputs)

original_height, original_width = video_frames[0].shape[:2]
target_sizes = [(original_height, original_width)] * len(video_frames)

preds = processor.post_process_semantic_segmentation(
    outputs,
    target_sizes=target_sizes,
)

fig, axes = plt.subplots(2, 4, figsize=(16, 8))
for idx, (ax, frame, seg_map) in enumerate(zip(axes.flatten(), video_frames, preds)):
    ax.imshow(frame)
    ax.imshow(seg_map.cpu().numpy(), alpha=0.6, cmap="tab20")
    ax.set_title(f"Frame {idx}")
    ax.axis("off")
plt.suptitle("Video Semantic Segmentation")
plt.tight_layout()
plt.show()
```

### Video Panoptic Segmentation

```python
import matplotlib.pyplot as plt
import numpy as np
import torch

from transformers import AutoModelForUniversalSegmentation, AutoVideoProcessor
from transformers.video_utils import load_video

model_id = "tue-mps/videomt-dinov2-small-ytvis2019"
processor = AutoVideoProcessor.from_pretrained(model_id)
model = AutoModelForUniversalSegmentation.from_pretrained(model_id, device_map="auto")

video_url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/videos/pexels-allan-mas-5362370.mp4"
# Sample 8 frames to keep the example lightweight.
video_frames, _ = load_video(video_url, num_frames=8)

inputs = processor(videos=[video_frames], return_tensors="pt").to(model.device)

with torch.inference_mode():
    outputs = model(**inputs)

original_height, original_width = video_frames[0].shape[:2]
target_sizes = [(original_height, original_width)] * len(video_frames)

results = processor.post_process_panoptic_segmentation(
    outputs,
    target_sizes=target_sizes,
)

fig, axes = plt.subplots(2, 4, figsize=(16, 8))
for idx, (ax, frame, result) in enumerate(zip(axes.flatten(), video_frames, results)):
    ax.imshow(frame)
    seg = result["segmentation"].cpu().numpy()
    masked = np.ma.masked_where(seg == -1, seg)
    ax.imshow(masked, alpha=0.6, cmap="tab20")
    ax.set_title(f"Frame {idx}")
    ax.axis("off")
plt.suptitle("Video Panoptic Segmentation")
plt.tight_layout()
plt.show()
```

## VideomtVideoProcessor[[transformers.VideomtVideoProcessor]]

- **outputs** (`VideomtForUniversalSegmentationOutput`) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple[int, int]]`) --
  List of `(height, width)` tuples corresponding to the requested final size of each prediction.
  Length should match the number of frames in the output.
- **return_segmentation_scores** (`bool`, *optional*, defaults to `False`) --
  Whether to return segmentation scores alongside the segmentation map. When `True`, each element of
  the returned list is a `SemanticSegmentationPostProcessorOutput` with fields `segmentation`
  (class IDs, shape `(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`).`list[torch.Tensor]` or `list[SemanticSegmentationPostProcessorOutput]`When
`return_segmentation_scores=False` (default), a list of length `batch_size` where each item is a
segmentation map of shape `(height, width)` with class IDs. When `return_segmentation_scores=True`,
a list of `SemanticSegmentationPostProcessorOutput` with fields `segmentation` (class IDs, shape
`(height, width)`) and `segmentation_scores` (shape `(num_classes, height, width)`). In both cases,
`(height, width)` corresponds to the target size.

Converts the output of [VideomtForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/videomt#transformers.VideomtForUniversalSegmentation) into semantic segmentation predictions.

- **outputs** (`VideomtForUniversalSegmentationOutput`) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple[int, int]]`) --
  List of `(height, width)` tuples corresponding to the requested final size of each prediction.
  Length should match the number of frames in the output.
- **threshold** (`float`, *optional*, defaults to 0.5) --
  Minimum combined score to keep an instance.`list[dict]`A list of dicts (one per frame), each containing:
- `"segmentation"` -- A `torch.Tensor` of shape `(height, width)` with instance IDs (or -1 for background).
- `"segments_info"` -- A list of dicts with `"id"`, `"label_id"`, and `"score"` for each instance.

Converts the output of [VideomtForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/videomt#transformers.VideomtForUniversalSegmentation) into instance segmentation predictions.

- **outputs** (`VideomtForUniversalSegmentationOutput`) --
  Raw outputs of the model.
- **target_sizes** (`list[tuple[int, int]]`) --
  List of `(height, width)` tuples corresponding to the requested final size of each prediction.
  Length should match the number of frames in the output.
- **threshold** (`float`, *optional*, defaults to 0.8) --
  Minimum score to keep a predicted segment.
- **mask_threshold** (`float`, *optional*, defaults to 0.5) --
  Threshold for binarizing mask probabilities.
- **overlap_mask_area_threshold** (`float`, *optional*, defaults to 0.8) --
  Overlap threshold to merge masks into a single segment.
- **label_ids_to_fuse** (`set[int]`, *optional*) --
  Label IDs that should be fused across disconnected regions.`list[dict]`A list of dicts (one per frame), each containing:
- `"segmentation"` -- A `torch.Tensor` of shape `(height, width)` with segment IDs (or -1 for background).
- `"segments_info"` -- A list of dicts with `"id"`, `"label_id"`, and `"score"` for each segment.

Converts the output of [VideomtForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/videomt#transformers.VideomtForUniversalSegmentation) into panoptic segmentation predictions.

## VideomtConfig[[transformers.VideomtConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `640`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **mlp_ratio** (`int`, *optional*, defaults to `4`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **layerscale_value** (`float`, *optional*, defaults to 1.0) --
  Initial value for the LayerScale parameter.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  Drop path rate for the patch fusion.
- **num_upscale_blocks** (`int`, *optional*, defaults to 2) --
  Number of upsampling blocks used in the decoder or segmentation head.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **use_swiglu_ffn** (`bool`, *optional*, defaults to `False`) --
  Whether to use the SwiGLU feedforward neural network.
- **num_blocks** (`int`, *optional*, defaults to 4) --
  Number of feature blocks or stages in the architecture.
- **no_object_weight** (`float`, *optional*, defaults to 0.1) --
  Loss weight for the 'no object' class in panoptic/instance segmentation.
- **class_weight** (`float`, *optional*, defaults to 2.0) --
  Loss weight for classification targets.
- **mask_weight** (`float`, *optional*, defaults to 5.0) --
  Loss weight for mask prediction.
- **dice_weight** (`float`, *optional*, defaults to `5.0`) --
  Relative weight of the dice loss in the panoptic segmentation loss.
- **train_num_points** (`int`, *optional*, defaults to 12544) --
  Number of points to sample for mask loss computation during training.
- **oversample_ratio** (`float`, *optional*, defaults to 3.0) --
  Oversampling ratio used in point sampling for mask training.
- **importance_sample_ratio** (`float`, *optional*, defaults to 0.75) --
  Ratio of points to sample based on importance during training.
- **num_queries** (`int`, *optional*, defaults to 200) --
  Number of object queries in the Transformer.
- **num_register_tokens** (`int`, *optional*, defaults to 4) --
  Number of learnable register tokens added to the transformer input.

This is the configuration class to store the configuration of a VideomtModel. It is used to instantiate a Videomt
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [tue-mps/videomt-dinov2-small-ytvis2019](https://huggingface.co/tue-mps/videomt-dinov2-small-ytvis2019)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import VideomtConfig, VideomtForUniversalSegmentation

>>> # Initialize configuration
>>> config = VideomtConfig()

>>> # Initialize model
>>> model = VideomtForUniversalSegmentation(config)

>>> # Access config
>>> config = model.config
```

## VideomtPreTrainedModel[[transformers.VideomtPreTrainedModel]]

- **config** ([PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

## VideomtForUniversalSegmentation[[transformers.VideomtForUniversalSegmentation]]

- **config** ([VideomtConfig](/docs/transformers/v5.14.0/en/model_doc/videomt#transformers.VideomtConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Videomt Model with head on top for instance/semantic/panoptic segmentation.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>] | None = None"}, {"name": "class_labels", "val": ": list[)>] | None = None"}, {"name": "patch_offsets", "val": ": list[)>] | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values_videos** (`torch.Tensor`, *optional*) --
  Video inputs of shape `(batch_size, num_frames, num_channels, height, width)`.
- **mask_labels** (`list[torch.Tensor]`, *optional*) --
  Not supported for 5D video inputs.
- **class_labels** (`list[torch.LongTensor]`, *optional*) --
  Not supported for 5D video inputs.
- **patch_offsets** (`list[torch.Tensor]`, *optional*) --
  Unused for video inputs and only kept for modular compatibility.`VideomtForUniversalSegmentationOutput` or `tuple(torch.FloatTensor)`A `VideomtForUniversalSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VideomtConfig](/docs/transformers/v5.14.0/en/model_doc/videomt#transformers.VideomtConfig)) and inputs.
The [VideomtForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/videomt#transformers.VideomtForUniversalSegmentation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.Tensor`, *optional*) -- The computed loss, returned when labels are present.
- **class_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, num_labels + 1)` representing the proposed classes for each
  query. Note the `+ 1` is needed because we incorporate the null class.
- **masks_queries_logits** (`torch.FloatTensor`, *optional*) -- A tensor of shape `(batch_size, num_queries, height, width)` representing the proposed masks for each
  query.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Last hidden states (final feature map) of the last layer.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each stage) of
  shape `(batch_size, sequence_length, hidden_size)`. Hidden-states all layers of the model.
- **attentions** (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `tuple(torch.FloatTensor)` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Self and Cross Attentions weights from transformer decoder.
