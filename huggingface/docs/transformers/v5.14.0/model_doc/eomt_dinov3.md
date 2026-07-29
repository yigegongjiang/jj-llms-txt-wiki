# EoMT-DINOv3

  

## Overview

The **EoMT-DINOv3** family extends the [Encoder-only Mask Transformer](eomt) architecture with
Vision Transformers that are pre-trained using [DINOv3](dinov3). The update delivers stronger segmentation quality across ADE20K and COCO
benchmarks while preserving the encoder-only design that made EoMT attractive for real-time applications.

Compared to the DINOv2-based models, the DINOv3 variants leverage rotary position embeddings, optional gated MLP blocks
and the latest pre-training recipes from Meta AI. These changes yield measurable performance gains across semantic,
instance and panoptic segmentation tasks, as highlighted in the [DINOv3 model zoo](https://github.com/tue-mps/eomt/blob/master/model_zoo/dinov3.md).

The original EoMT architecture was introduced in the CVPR 2025 Highlight paper *[Your ViT is Secretly an Image
Segmentation Model](https://huggingface.co/papers/2503.19108)* by Tommie Kerssies, Niccolò Cavagnero, Alexander Hermans,
Narges Norouzi, Giuseppe Averta, Bastian Leibe, Gijs Dubbelman and Daan de Geus. The DINOv3 upgrade keeps the same
lightweight segmentation head and query-based inference strategy while swapping the encoder for DINOv3 ViT checkpoints.

Tips:

* The configuration exposes DINOv3-specific knobs such as `rope_theta` and `use_gated_mlp`. Large DINOv3 backbones
  such as `dinov3-vitg14` expect `use_gated_mlp=True`.
* DINOv3 models can operate on a broader range of resolutions thanks to rotary position embeddings. The image processor
  still defaults to square crops but custom sizes can be supplied through `AutoImageProcessor`.
* The pre-trained checkpoints hosted by the TU/e Mobile Perception Systems Lab provide delta weights that should be
  combined with the upstream DINOv3 backbones. The conversion utilities in the
  [official repository](https://github.com/tue-mps/eomt) describe this workflow in detail.

This model was contributed by [nielsr](https://huggingface.co/nielsr).
The original code can be found [here](https://github.com/tue-mps/eomt).

## Usage examples

Below is a minimal example showing how to run panoptic segmentation with a DINOv3-backed EoMT model. The same
image processor can be reused for semantic or instance segmentation simply by swapping the checkpoint.

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForUniversalSegmentation

model_id = "tue-mps/eomt-dinov3-coco-panoptic-base-640"
processor = AutoImageProcessor.from_pretrained(model_id)
model = AutoModelForUniversalSegmentation.from_pretrained(model_id).to("cuda" if torch.cuda.is_available() else "cpu", device_map="auto")

image = Image.open(requests.get("http://images.cocodataset.org/val2017/000000039769.jpg", stream=True).raw)

inputs = processor(images=image, return_tensors="pt").to(model.device)

with torch.inference_mode():
    outputs = model(**inputs)

segmentation = processor.post_process_panoptic_segmentation(outputs, target_sizes=[image.size[::-1]])[0]
list(segmentation.keys())
['segmentation', 'segments_info']
```

## EomtDinov3Config[[transformers.EomtDinov3Config]]

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
- **layerscale_value** (`float`, *optional*, defaults to 1.0) --
  Initial value for the LayerScale parameter.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  Drop path rate for the patch fusion.
- **num_upscale_blocks** (`int`, *optional*, defaults to 2) --
  Number of upsampling blocks used in the decoder or segmentation head.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **num_blocks** (`int`, *optional*, defaults to 4) --
  Number of feature blocks or stages in the architecture.
- **no_object_weight** (`float`, *optional*, defaults to 0.1) --
  Loss weight for the "no object" class in panoptic/instance segmentation.
- **class_weight** (`float`, *optional*, defaults to `2.0`) --
  Relative weight of the classification error in the Hungarian matching cost.
- **mask_weight** (`float`, *optional*, defaults to `5.0`) --
  Relative weight of the focal loss in the panoptic segmentation loss.
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
- **intermediate_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the MLP representations.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **query_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in query projection.
- **key_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use bias in key projection.
- **value_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in value projection.
- **proj_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use bias in output projection.
- **mlp_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.
- **use_gated_mlp** (`bool`, *optional*, defaults to `False`) --
  Whether to use gated MLP layers.
- **pos_embed_shift** (`float`, *optional*) --
  Shift value for position embeddings.
- **pos_embed_jitter** (`float`, *optional*) --
  Jitter value for position embeddings.
- **pos_embed_rescale** (`float`, *optional*, defaults to 2.0) --
  Rescale value for position embeddings.

This is the configuration class to store the configuration of a Eomt Dinov3Model. It is used to instantiate a Eomt Dinov3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [tue-mps/coco_panoptic_eomt_large_640_dinov3](https://huggingface.co/tue-mps/coco_panoptic_eomt_large_640_dinov3)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## EomtDinov3PreTrainedModel[[transformers.EomtDinov3PreTrainedModel]]

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

## EomtDinov3ForUniversalSegmentation[[transformers.EomtDinov3ForUniversalSegmentation]]

- **config** ([EomtDinov3Config](/docs/transformers/v5.14.0/en/model_doc/eomt_dinov3#transformers.EomtDinov3Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The EoMT-DINOv3 model with head on top for instance/semantic/panoptic segmentation.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "mask_labels", "val": ": list[)>] | None = None"}, {"name": "class_labels", "val": ": list[)>] | None = None"}, {"name": "patch_offsets", "val": ": list[)>] | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [EomtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtImageProcessor). See `EomtImageProcessor.__call__()` for details (`processor_class` uses
  [EomtImageProcessor](/docs/transformers/v5.14.0/en/model_doc/eomt#transformers.EomtImageProcessor) for processing images).
- **mask_labels** (`list[torch.Tensor]`, *optional*) --
  list of mask labels of shape `(num_labels, height, width)` to be fed to a model
- **class_labels** (`list[torch.LongTensor]`, *optional*) --
  list of target class labels of shape `(num_labels, height, width)` to be fed to a model. They identify the
  labels of `mask_labels`, e.g. the label of `mask_labels[i][j]` if `class_labels[i][j]`.
- **patch_offsets** (`list[torch.Tensor]`, *optional*) --
  list of tuples indicating the image index and start and end positions of patches for semantic segmentation.`EomtDinov3ForUniversalSegmentationOutput` or `tuple(torch.FloatTensor)`A `EomtDinov3ForUniversalSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([EomtDinov3Config](/docs/transformers/v5.14.0/en/model_doc/eomt_dinov3#transformers.EomtDinov3Config)) and inputs.
The [EomtDinov3ForUniversalSegmentation](/docs/transformers/v5.14.0/en/model_doc/eomt_dinov3#transformers.EomtDinov3ForUniversalSegmentation) forward method, overrides the `__call__` special method.

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
- **patch_offsets** (`list[torch.Tensor]`, *optional*) -- list of tuples indicating the image index and start and end positions of patches for semantic segmentation.
