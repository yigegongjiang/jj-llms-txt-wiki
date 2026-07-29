# LW-DETR

[LW-DETR](https://huggingface.co/papers/2407.17140) proposes a light-weight Detection Transformer (DETR) architecture designed to compete with and surpass the dominant YOLO series for real-time object detection. It achieves a new state-of-the-art balance between speed (latency) and accuracy (mAP) by combining recent transformer advances with efficient design choices.

The LW-DETR architecture is characterized by its simple and efficient structure: a plain ViT Encoder, a Projector, and a shallow DETR Decoder.
It enhances the DETR architecture for efficiency and speed using the following core modifications:
1. Efficient ViT Encoder: Uses a plain ViT with interleaved window/global attention and a window-major organization to drastically reduce attention complexity and latency.
2. Richer Input: Aggregates multi-level features from the encoder and uses a C2f Projector (YOLOv8) to pass two-scale features ($1/8$ and $1/32$).
3. Faster Decoder: Employs a shallow 3-layer DETR decoder with deformable cross-attention for lower latency and faster convergence.
4. Optimized Queries: Uses a mixed-query scheme combining learnable content queries and generated spatial queries.

You can find all the available LW DETR checkpoints under the [AnnaZhang](https://huggingface.co/AnnaZhang) organization.
The original code can be found [here](https://github.com/Atten4Vis/LW-DETR).

> [!TIP]
> This model was contributed by [stevenbucaille](https://huggingface.co/stevenbucaille).
>
> Click on the LW-DETR models in the right sidebar for more examples of how to apply LW-DETR to different object detection tasks.

The example below demonstrates how to perform object detection with the [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) and the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python

from transformers import pipeline

pipeline = pipeline(
    "object-detection",
    model="AnnaZhang/lwdetr_small_60e_coco",
    device_map=0
)

pipeline("http://images.cocodataset.org/val2017/000000039769.jpg")
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForObjectDetection

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(requests.get(url, stream=True).raw)

image_processor = AutoImageProcessor.from_pretrained("AnnaZhang/lwdetr_small_60e_coco")
model = AutoModelForObjectDetection.from_pretrained("AnnaZhang/lwdetr_small_60e_coco", device_map="auto")

# prepare image for the model
inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

results = image_processor.post_process_object_detection(outputs, target_sizes=torch.tensor([image.size[::-1]]), threshold=0.3)

for result in results:
    for score, label_id, box in zip(result["scores"], result["labels"], result["boxes"]):
        score, label = score.item(), label_id.item()
        box = [round(i, 2) for i in box.tolist()]
        print(f"{model.config.id2label[label]}: {score:.2f} {box}")
```

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with LwDetr.

- Scripts for finetuning [LwDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrForObjectDetection) with [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer) or [Accelerate](https://huggingface.co/docs/accelerate/index) can be found [here](https://github.com/huggingface/transformers/tree/main/examples/pytorch/object-detection).
- See also: [Object detection task guide](../tasks/object_detection).

## LwDetrConfig[[transformers.LwDetrConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **projector_scale_factors** (`list[float]`, *optional*, defaults to `[]`) --
  Scale factors for the feature pyramid network. Each scale factor determines the resolution of features
  at different levels. Supported values are 0.5, 1.0, and 2.0.
- **hidden_expansion** (`float`, *optional*, defaults to 0.5) --
  Expansion factor for hidden dimensions in the projector layers.
- **c2f_num_blocks** (`int`, *optional*, defaults to 3) --
  Number of blocks in the C2F layer.
- **activation_function** (`str`, *optional*, defaults to `"silu"`) --
  The non-linear activation function in the projector. Supported values are `"silu"`, `"relu"`, `"gelu"`.
- **batch_norm_eps** (`float`, *optional*, defaults to 1e-05) --
  The epsilon value for batch normalization layers.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **decoder_ffn_dim** (`int`, *optional*, defaults to 2048) --
  Dimension of the "intermediate" (often named feed-forward) layer in decoder.
- **decoder_n_points** (`int`, *optional*, defaults to 4) --
  The number of sampled keys in each feature level for each attention head in the decoder.
- **decoder_layers** (`int`, *optional*, defaults to `3`) --
  Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.
- **decoder_self_attention_heads** (`int`, *optional*, defaults to 8) --
  Number of attention heads for each attention layer in the decoder self-attention.
- **decoder_cross_attention_heads** (`int`, *optional*, defaults to 16) --
  Number of attention heads for each attention layer in the decoder cross-attention.
- **decoder_activation_function** (`str`, *optional*, defaults to `"relu"`) --
  The non-linear activation function in the decoder. Supported values are `"relu"`, `"silu"`, `"gelu"`.
- **num_queries** (`int`, *optional*, defaults to 300) --
  Number of object queries, i.e. detection slots. This is the maximal number of objects
  [LwDetrModel](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrModel) can detect in a single image.
- **attention_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for activations inside the fully connected layer.
- **group_detr** (`int`, *optional*, defaults to 13) --
  Number of groups for Group DETR attention mechanism, which helps reduce computational complexity.
- **init_std** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **disable_custom_kernels** (`bool`, *optional*, defaults to `True`) --
  Disable the use of custom CUDA and CPU kernels. This option is necessary for the ONNX export, as custom
  kernels are not supported by PyTorch ONNX export.
- **class_cost** (`Union[int, float]`, *optional*, defaults to `2`) --
  Relative weight of the classification error in the Hungarian matching cost.
- **bbox_cost** (`Union[int, float]`, *optional*, defaults to `5`) --
  Relative weight of the L1 bounding box error in the Hungarian matching cost.
- **giou_cost** (`Union[int, float]`, *optional*, defaults to `2`) --
  Relative weight of the generalized IoU loss in the Hungarian matching cost.
- **class_loss_coefficient** (`float`, *optional*, defaults to 1) --
  Relative weight of the classification loss in the Hungarian matching cost.
- **dice_loss_coefficient** (`Union[int, float]`, *optional*, defaults to `1`) --
  Relative weight of the dice loss in the panoptic segmentation loss.
- **bbox_loss_coefficient** (`Union[int, float]`, *optional*, defaults to `5`) --
  Relative weight of the L1 bounding box loss in the panoptic segmentation loss.
- **giou_loss_coefficient** (`Union[int, float]`, *optional*, defaults to `2`) --
  Relative weight of the generalized IoU loss in the panoptic segmentation loss.
- **eos_coefficient** (`float`, *optional*, defaults to `0.1`) --
  Relative classification weight of the 'no-object' class in the object detection loss.
- **focal_alpha** (`float`, *optional*, defaults to `0.25`) --
  Alpha parameter in the focal loss.
- **auxiliary_loss** (`bool`, *optional*, defaults to `True`) --
  Whether auxiliary decoding losses (losses at each decoder layer) are to be used.
- **d_model** (`int`, *optional*, defaults to `256`) --
  Size of the encoder layers and the pooler layer.

This is the configuration class to store the configuration of a LwDetrModel. It is used to instantiate a Lw Detr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [AnnaZhang/lwdetr_small_60e_coco](https://huggingface.co/AnnaZhang/lwdetr_small_60e_coco)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import LwDetrConfig, LwDetrModel

>>> # Initializing a LW-DETR AnnaZhang/lwdetr_small_60e_coco style configuration
>>> configuration = LwDetrConfig()

>>> # Initializing a model (with random weights) from the AnnaZhang/lwdetr_small_60e_coco style configuration
>>> model = LwDetrModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## LwDetrViTConfig[[transformers.LwDetrViTConfig]]

- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **mlp_ratio** (`int`, *optional*, defaults to `4`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `256`) --
  The size (resolution) of each image.
- **pretrain_image_size** (`int`, *optional*, defaults to 224) --
  The size (resolution) of each image during pretraining.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **window_block_indices** (`list[int]`, *optional*, defaults to `[]`) --
  List of indices of blocks that should have window attention instead of regular global self-attention.
- **use_absolute_position_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to add absolute position embeddings to the patch embeddings.
- **cae_init_values** (`float`, *optional*, defaults to 0.1) --
  Initialization value for CAE parameters when `use_cae` is enabled.
- **num_windows** (`int`, *optional*, defaults to 16) --
  Number of windows for window-based attention. Must be a perfect square and the image size must be
  divisible by the square root of this value. This enables efficient window-major feature map organization.

This is the configuration class to store the configuration of a LwDetrModel. It is used to instantiate a Lw Detr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [AnnaZhang/lwdetr_small_60e_coco](https://huggingface.co/AnnaZhang/lwdetr_small_60e_coco)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import LwDetrViTConfig, LwDetrViTModel

>>> # Initializing a LW-DETR ViT configuration
>>> configuration = LwDetrViTConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = LwDetrViTModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## LwDetrModel[[transformers.LwDetrModel]]

- **config** ([LwDetrConfig](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare LW Detr Model (consisting of a backbone and decoder Transformer) outputting raw
hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [DeformableDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrImageProcessor). See `DeformableDetrImageProcessor.__call__()` for details (`processor_class` uses
  [DeformableDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrImageProcessor) for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)`LwDetrModelOutput` or `tuple(torch.FloatTensor)`A `LwDetrModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LwDetrConfig](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrConfig)) and inputs.
The [LwDetrModel](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **init_reference_points** (`torch.FloatTensor` of shape  `(batch_size, num_queries, 4)`) -- Initial reference points sent through the Transformer decoder.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, hidden_size)`) -- Stacked intermediate hidden states (output of each layer of the decoder).
- **intermediate_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate reference points (reference points of each layer of the decoder).
- **enc_outputs_class** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Predicted bounding boxes scores where the top `config.two_stage_num_proposals` scoring bounding boxes are
  picked as region proposals in the first stage. Output of bounding box binary classification (i.e.
  foreground and background).
- **enc_outputs_coord_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the first stage.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **cross_attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, DeformableDetrModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("AnnaZhang/lwdetr_small_60e_coco")
>>> model = DeformableDetrModel.from_pretrained("AnnaZhang/lwdetr_small_60e_coco")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)

>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 300, 256]
```

## LwDetrForObjectDetection[[transformers.LwDetrForObjectDetection]]

- **config** ([LwDetrConfig](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

LW DETR Model (consisting of a backbone and decoder Transformer) with object detection heads on
top, for tasks such as COCO detection.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [DeformableDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrImageProcessor). See `DeformableDetrImageProcessor.__call__()` for details (`processor_class` uses
  [DeformableDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrImageProcessor) for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **labels** (`list[Dict]` of len `(batch_size,)`, *optional*) --
  Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the
  following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch
  respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes
  in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.</paramsdesc><rettype>`LwDetrObjectDetectionOutput` or `tuple(torch.FloatTensor)`</rettype><retdesc>A `LwDetrObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LwDetrConfig](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrConfig)) and inputs.
The [LwDetrForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrForObjectDetection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` are provided)) -- Total loss as a linear combination of a negative log-likelihood (cross-entropy) for class prediction and a
  bounding box loss. The latter is defined as a linear combination of the L1 loss and the generalized
  scale-invariant IoU loss.
- **loss_dict** (`Dict`, *optional*) -- A dictionary containing the individual losses. Useful for logging.
- **logits** (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes + 1)`) -- Classification logits (including no-object) for all queries.
- **pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- Normalized boxes coordinates for all queries, represented as (center_x, center_y, width, height). These
  values are normalized in [0, 1], relative to the size of each individual image in the batch (disregarding
  possible padding). You can use `~DeformableDetrProcessor.post_process_object_detection` to retrieve the
  unnormalized bounding boxes.
- **auxiliary_outputs** (`list[Dict]`, *optional*) -- Optional, only returned when auxiliary losses are activated (i.e. `config.auxiliary_loss` is set to `True`)
  and labels are provided. It is a list of dictionaries containing the two above keys (`logits` and
  `pred_boxes`) for each decoder layer.
- **init_reference_points** (`torch.FloatTensor` of shape  `(batch_size, num_queries, 4)`) -- Initial reference points sent through the Transformer decoder.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, hidden_size)`) -- Stacked intermediate hidden states (output of each layer of the decoder).
- **intermediate_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate reference points (reference points of each layer of the decoder).
- **enc_outputs_class** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Predicted bounding boxes scores where the top `config.two_stage_num_proposals` scoring bounding boxes are
  picked as region proposals in the first stage. Output of bounding box binary classification (i.e.
  foreground and background).
- **enc_outputs_coord_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the first stage.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **cross_attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, LwDetrForObjectDetection
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("AnnaZhang/lwdetr_small_60e_coco")
>>> model = LwDetrForObjectDetection.from_pretrained("AnnaZhang/lwdetr_small_60e_coco")

>>> inputs = image_processor(images=image, return_tensors="pt")
>>> outputs = model(**inputs)

>>> # convert outputs (bounding boxes and class logits) to Pascal VOC format (xmin, ymin, xmax, ymax)
>>> target_sizes = torch.tensor([image.size[::-1]])
>>> results = image_processor.post_process_object_detection(outputs, threshold=0.5, target_sizes=target_sizes)[
...     0
... ]
>>> for score, label, box in zip(results["scores"], results["labels"], results["boxes"]):
...     box = [round(i, 2) for i in box.tolist()]
...     print(
...         f"Detected {model.config.id2label[label.item()]} with confidence "
...         f"{round(score.item(), 3)} at location {box}"
...     )
Detected cat with confidence 0.8 at location [16.5, 52.84, 318.25, 470.78]
Detected cat with confidence 0.789 at location [342.19, 24.3, 640.02, 372.25]
Detected remote with confidence 0.633 at location [40.79, 72.78, 176.76, 117.25]
```

## LwDetrViTBackbone[[transformers.LwDetrViTBackbone]]

- **config** ([LwDetrViTBackbone](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrViTBackbone)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Lw Detr backbone.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [DeformableDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrImageProcessor). See `DeformableDetrImageProcessor.__call__()` for details (`processor_class` uses
  [DeformableDetrImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deformable_detr#transformers.DeformableDetrImageProcessor) for processing images).`BackboneOutput` or `tuple(torch.FloatTensor)`A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([LwDetrConfig](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrConfig)) and inputs.
The [LwDetrViTBackbone](/docs/transformers/v5.14.0/en/model_doc/lw_detr#transformers.LwDetrViTBackbone) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **feature_maps** (`tuple(torch.FloatTensor)` of shape `(batch_size, num_channels, height, width)`) -- Feature maps of the stages.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, sequence_length, hidden_size)` or `(batch_size, num_channels, height, width)`,
  depending on the backbone.

  Hidden-states of the model at the output of each stage plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Only applicable if the backbone uses attention.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import LwDetrViTConfig, LwDetrViTBackbone
>>> import torch

>>> config = LwDetrViTConfig()
>>> model = LwDetrViTBackbone(config)

>>> pixel_values = torch.randn(1, 3, 224, 224)

>>> with torch.no_grad():
...     outputs = model(pixel_values)

>>> feature_maps = outputs.feature_maps
>>> list(feature_maps[-1].shape)
[1, 768, 14, 14]
```
