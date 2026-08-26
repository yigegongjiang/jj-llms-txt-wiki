# D-FINE

## Overview

The D-FINE model was proposed in [D-FINE: Redefine Regression Task in DETRs as Fine-grained Distribution Refinement](https://huggingface.co/papers/2410.13842) by
Yansong Peng, Hebei Li, Peixi Wu, Yueyi Zhang, Xiaoyan Sun, Feng Wu

The abstract from the paper is the following:

*We introduce D-FINE, a powerful real-time object detector that achieves outstanding localization precision by redefining the bounding box regression task in DETR models. D-FINE comprises two key components: Fine-grained Distribution Refinement (FDR) and Global Optimal Localization Self-Distillation (GO-LSD).
FDR transforms the regression process from predicting fixed coordinates to iteratively refining probability distributions, providing a fine-grained intermediate representation that significantly enhances localization accuracy. GO-LSD is a bidirectional optimization strategy that transfers localization knowledge from refined distributions to shallower layers through self-distillation, while also simplifying the residual prediction tasks for deeper layers. Additionally, D-FINE incorporates lightweight optimizations in computationally intensive modules and operations, achieving a better balance between speed and accuracy. Specifically, D-FINE-L / X achieves 54.0% / 55.8% AP on the COCO dataset at 124 / 78 FPS on an NVIDIA T4 GPU. When pretrained on Objects365, D-FINE-L / X attains 57.1% / 59.3% AP, surpassing all existing real-time detectors. Furthermore, our method significantly enhances the performance of a wide range of DETR models by up to 5.3% AP with negligible extra parameters and training costs. Our code and pretrained models: this https URL.*

This model was contributed by [VladOS95-cyber](https://github.com/VladOS95-cyber).
The original code can be found [here](https://github.com/Peterande/D-FINE).

## Usage tips

```python
import torch

from transformers import AutoImageProcessor, DFineForObjectDetection
from transformers.image_utils import load_image

url = 'http://images.cocodataset.org/val2017/000000039769.jpg'
image = load_image(url)

image_processor = AutoImageProcessor.from_pretrained("ustc-community/dfine_x_coco")
model = DFineForObjectDetection.from_pretrained("ustc-community/dfine_x_coco", device_map="auto")

inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

results = image_processor.post_process_object_detection(outputs, target_sizes=[(image.height, image.width)], threshold=0.5)

for result in results:
    for score, label_id, box in zip(result["scores"], result["labels"], result["boxes"]):
        score, label = score.item(), label_id.item()
        box = [round(i, 2) for i in box.tolist()]
        print(f"{model.config.id2label[label]}: {score:.2f} {box}")
cat: 0.96 [344.49, 23.4, 639.84, 374.27]
cat: 0.96 [11.71, 53.52, 316.64, 472.33]
remote: 0.95 [40.46, 73.7, 175.62, 117.57]
sofa: 0.92 [0.59, 1.88, 640.25, 474.74]
remote: 0.89 [333.48, 77.04, 370.77, 187.3]
```

## DFineConfig[[transformers.DFineConfig]]

#### transformers.DFineConfig[[transformers.DFineConfig]]

```python
transformers.DFineConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, initializer_range: float = 0.01, initializer_bias_prior_prob: float | None = None, layer_norm_eps: float = 1e-05, batch_norm_eps: float = 1e-05, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, freeze_backbone_batch_norms: bool = True, encoder_hidden_dim: int = 256, encoder_in_channels: list[int] | tuple[int, ...] = (512, 1024, 2048), feat_strides: list[int] | tuple[int, ...] = (8, 16, 32), encoder_layers: int = 1, encoder_ffn_dim: int = 1024, encoder_attention_heads: int = 8, dropout: float | int = 0.0, activation_dropout: float | int = 0.0, encode_proj_layers: list[int] | tuple[int, ...] = (2,), positional_encoding_temperature: int = 10000, encoder_activation_function: str = 'gelu', activation_function: str = 'silu', eval_size: int | None = None, normalize_before: bool = False, hidden_expansion: float = 1.0, d_model: int = 256, num_queries: int = 300, decoder_in_channels: list[int] | tuple[int, ...] = (256, 256, 256), decoder_ffn_dim: int = 1024, num_feature_levels: int = 3, decoder_n_points: int | list[int] = 4, decoder_layers: int = 6, decoder_attention_heads: int = 8, decoder_activation_function: str = 'relu', attention_dropout: float | int = 0.0, num_denoising: int = 100, label_noise_ratio: float = 0.5, box_noise_scale: float = 1.0, learn_initial_query: bool = False, anchor_image_size: int | list[int] | None = None, with_box_refine: bool = True, matcher_alpha: float = 0.25, matcher_gamma: float = 2.0, matcher_class_cost: float = 2.0, matcher_bbox_cost: float = 5.0, matcher_giou_cost: float = 2.0, use_focal_loss: bool = True, auxiliary_loss: bool = True, focal_loss_alpha: float = 0.75, focal_loss_gamma: float = 2.0, weight_loss_vfl: float = 1.0, weight_loss_bbox: float = 5.0, weight_loss_giou: float = 2.0, weight_loss_fgl: float = 0.15, weight_loss_ddf: float = 1.5, eos_coefficient: float = 0.0001, eval_idx: int = -1, layer_scale: int | float = 1.0, max_num_bins: int = 32, reg_scale: float = 4.0, depth_mult: float = 1.0, top_prob_values: int = 4, lqe_hidden_dim: int = 64, lqe_layers: int = 2, decoder_offset_scale: float = 0.5, decoder_method: str = 'default', up: float = 0.5, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/d_fine/configuration_d_fine.py#L32)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

initializer_range (`float`, *optional*, defaults to `0.01`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

initializer_bias_prior_prob (`float`, *optional*) : The prior probability used by the bias initializer to initialize biases for `enc_score_head` and `class_embed`. If `None`, `prior_prob` computed as `prior_prob = 1 / (num_labels + 1)` while initializing model weights.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

batch_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the batch normalization layers.

backbone_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The configuration of the backbone model.

freeze_backbone_batch_norms (`bool`, *optional*, defaults to `True`) : Whether to freeze the batch normalization layers in the backbone.

encoder_hidden_dim (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

encoder_in_channels (`list`, *optional*, defaults to `[512, 1024, 2048]`) : Multi level features input for encoder.

feat_strides (`list[int]`, *optional*, defaults to `[8, 16, 32]`) : Strides used in each feature map.

encoder_layers (`int`, *optional*, defaults to `1`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

encoder_ffn_dim (`int`, *optional*, defaults to `1024`) : Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.

encoder_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer encoder.

dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The ratio for all dropout layers.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for activations inside the fully connected layer.

encode_proj_layers (`list[int]`, *optional*, defaults to `[2]`) : Indexes of the projected layers to be used in the encoder.

positional_encoding_temperature (`int`, *optional*, defaults to 10000) : The temperature parameter used to create the positional encodings.

encoder_activation_function (`str`, *optional*, defaults to `"gelu"`) : The non-linear activation function (function or string) in the encoder and pooler. If string, `"gelu"`, `"relu"`, `"silu"` and `"gelu_new"` are supported.

activation_function (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

eval_size (`tuple[int, int]`, *optional*) : Height and width used to computes the effective height and width of the position embeddings after taking into account the stride.

normalize_before (`bool`, *optional*, defaults to `False`) : Determine whether to apply layer normalization in the transformer encoder layer before self-attention and feed-forward modules.

hidden_expansion (`float`, *optional*, defaults to 1.0) : Expansion ratio to enlarge the dimension size of RepVGGBlock and CSPRepLayer.

d_model (`int`, *optional*, defaults to `256`) : Size of the encoder layers and the pooler layer.

num_queries (`int`, *optional*, defaults to 300) : Number of object queries.

decoder_in_channels (`list`, *optional*, defaults to `[256, 256, 256]`) : Multi level features dimension for decoder

decoder_ffn_dim (`int`, *optional*, defaults to `1024`) : Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.

num_feature_levels (`int`, *optional*, defaults to 3) : The number of input feature levels.

decoder_n_points (`int`, *optional*, defaults to 4) : The number of sampled keys in each feature level for each attention head in the decoder.

decoder_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

decoder_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

decoder_activation_function (`str`, *optional*, defaults to `"relu"`) : The non-linear activation function (function or string) in the decoder. If string, `"gelu"`, `"relu"`, `"silu"` and `"gelu_new"` are supported.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

num_denoising (`int`, *optional*, defaults to 100) : The total number of denoising tasks or queries to be used for contrastive denoising.

label_noise_ratio (`float`, *optional*, defaults to 0.5) : The fraction of denoising labels to which random noise should be added.

box_noise_scale (`float`, *optional*, defaults to 1.0) : Scale or magnitude of noise to be added to the bounding boxes.

learn_initial_query (`bool`, *optional*, defaults to `False`) : Indicates whether the initial query embeddings for the decoder should be learned during training

anchor_image_size (`tuple[int, int]`, *optional*) : Height and width of the input image used during evaluation to generate the bounding box anchors. If None, automatic generate anchor is applied.

with_box_refine (`bool`, *optional*, defaults to `True`) : Whether to apply iterative bounding box refinement, where each decoder layer refines the bounding boxes based on the predictions from the previous layer.

matcher_alpha (`float`, *optional*, defaults to 0.25) : Parameter alpha used by the Hungarian Matcher.

matcher_gamma (`float`, *optional*, defaults to 2.0) : Parameter gamma used by the Hungarian Matcher.

matcher_class_cost (`float`, *optional*, defaults to 2.0) : The relative weight of the class loss used by the Hungarian Matcher.

matcher_bbox_cost (`float`, *optional*, defaults to 5.0) : The relative weight of the bounding box loss used by the Hungarian Matcher.

matcher_giou_cost (`float`, *optional*, defaults to 2.0) : The relative weight of the giou loss used by the Hungarian Matcher.

use_focal_loss (`bool`, *optional*, defaults to `True`) : Parameter informing if focal loss should be used.

auxiliary_loss (`bool`, *optional*, defaults to `True`) : Whether auxiliary decoding losses (losses at each decoder layer) are to be used.

focal_loss_alpha (`float`, *optional*, defaults to 0.75) : Parameter alpha used to compute the focal loss.

focal_loss_gamma (`float`, *optional*, defaults to 2.0) : Parameter gamma used to compute the focal loss.

weight_loss_vfl (`float`, *optional*, defaults to 1.0) : Relative weight of the varifocal loss in the object detection loss.

weight_loss_bbox (`float`, *optional*, defaults to 5.0) : Relative weight of the L1 bounding box loss in the object detection loss.

weight_loss_giou (`float`, *optional*, defaults to 2.0) : Relative weight of the generalized IoU loss in the object detection loss.

weight_loss_fgl (`float`, *optional*, defaults to 0.15) : Relative weight of the fine-grained localization loss in the object detection loss.

weight_loss_ddf (`float`, *optional*, defaults to 1.5) : Relative weight of the decoupled distillation focal loss in the object detection loss.

eos_coefficient (`float`, *optional*, defaults to `0.0001`) : Relative classification weight of the 'no-object' class in the object detection loss.

eval_idx (`int`, *optional*, defaults to -1) : Index of the decoder layer to use for evaluation. If negative, counts from the end (e.g., -1 means use the last layer). This allows for early prediction in the decoder stack while still training later layers.

layer_scale (`float`, *optional*, defaults to `1.0`) : Scaling factor for the hidden dimension in later decoder layers. Used to adjust the model capacity after the evaluation layer.

max_num_bins (`int`, *optional*, defaults to 32) : Maximum number of bins for the distribution-guided bounding box refinement. Higher values allow for more fine-grained localization but increase computation.

reg_scale (`float`, *optional*, defaults to 4.0) : Scale factor for the regression distribution. Controls the range and granularity of the bounding box refinement process.

depth_mult (`float`, *optional*, defaults to 1.0) : Multiplier for the number of blocks in RepNCSPELAN4 layers. Used to scale the model's depth while maintaining its architecture.

top_prob_values (`int`, *optional*, defaults to 4) : Number of top probability values to consider from each corner's distribution.

lqe_hidden_dim (`int`, *optional*, defaults to 64) : Hidden dimension size for the Location Quality Estimator (LQE) network.

lqe_layers (`int`, *optional*, defaults to 2) : Number of layers in the Location Quality Estimator MLP.

decoder_offset_scale (`float`, *optional*, defaults to 0.5) : Offset scale used in deformable attention.

decoder_method (`str`, *optional*, defaults to `"default"`) : The method to use for the decoder: `"default"` or `"discrete"`.

up (`float`, *optional*, defaults to 0.5) : Controls the upper bounds of the Weighting Function.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a DFineModel. It is used to instantiate a D Fine
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ustc-community/dfine-xlarge-coco](https://huggingface.co/ustc-community/dfine-xlarge-coco)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## DFineModel[[transformers.DFineModel]]

#### transformers.DFineModel[[transformers.DFineModel]]

```python
transformers.DFineModel(config: DFineConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/d_fine/modeling_d_fine.py#L1538)

**Parameters:**

config ([DFineConfig](/docs/transformers/v5.15.1/en/model_doc/d_fine#transformers.DFineConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

RT-DETR Model (consisting of a backbone and encoder-decoder) outputting raw hidden states without any head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DFineModel.forward]]

```python
forward(pixel_values: FloatTensor, pixel_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: typing.Optional[torch.FloatTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: list[dict] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/d_fine/modeling_d_fine.py#L1664)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

pixel_mask (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

encoder_outputs (`torch.FloatTensor`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you can choose to directly pass a flattened representation of an image.

labels (`list[Dict]` of len `(batch_size,)`, *optional*) : Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.

**Returns:** `DFineModelOutput` or `tuple(torch.FloatTensor)`

A `DFineModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [DFineModel](/docs/transformers/v5.15.1/en/model_doc/d_fine#transformers.DFineModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, hidden_size)`) -- Stacked intermediate hidden states (output of each layer of the decoder).
- **intermediate_logits** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, sequence_length, config.num_labels)`) -- Stacked intermediate logits (logits of each layer of the decoder).
- **intermediate_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate reference points (reference points of each layer of the decoder).
- **intermediate_predicted_corners** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate predicted corners (predicted corners of each layer of the decoder).
- **initial_reference_points** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- Initial reference points used for the first decoder layer.
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **init_reference_points** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- Initial reference points sent through the Transformer decoder.
- **enc_topk_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`) -- Predicted bounding boxes scores where the top `config.two_stage_num_proposals` scoring bounding boxes are
  picked as region proposals in the encoder stage. Output of bounding box binary classification (i.e.
  foreground and background).
- **enc_topk_bboxes** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`) -- Logits of predicted bounding boxes coordinates in the encoder stage.
- **enc_outputs_class** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Predicted bounding boxes scores where the top `config.two_stage_num_proposals` scoring bounding boxes are
  picked as region proposals in the first stage. Output of bounding box binary classification (i.e.
  foreground and background).
- **enc_outputs_coord_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the first stage.
- **denoising_meta_values** (`dict`, *optional*) -- Extra dictionary for the denoising related values.

Examples:

```python
>>> from transformers import AutoImageProcessor, DFineModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("PekingU/DFine_r50vd")
>>> model = DFineModel.from_pretrained("PekingU/DFine_r50vd")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)

>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 300, 256]
```

## DFineForObjectDetection[[transformers.DFineForObjectDetection]]

#### transformers.DFineForObjectDetection[[transformers.DFineForObjectDetection]]

```python
transformers.DFineForObjectDetection(config: DFineConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/d_fine/modeling_d_fine.py#L1948)

**Parameters:**

config ([DFineConfig](/docs/transformers/v5.15.1/en/model_doc/d_fine#transformers.DFineConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

RT-DETR Model (consisting of a backbone and encoder-decoder) outputting bounding boxes and logits to be further
decoded into scores and classes.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DFineForObjectDetection.forward]]

```python
forward(pixel_values: FloatTensor, pixel_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: typing.Optional[torch.FloatTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: list[dict] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/d_fine/modeling_d_fine.py#L1987)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

pixel_mask (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

encoder_outputs (`torch.FloatTensor`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`list[dict]` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

**Returns:** `DFineObjectDetectionOutput` or `tuple(torch.FloatTensor)`

A `DFineObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [DFineForObjectDetection](/docs/transformers/v5.15.1/en/model_doc/d_fine#transformers.DFineForObjectDetection) forward method, overrides the `__call__` special method.

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
  possible padding). You can use `~DFineImageProcessor.post_process_object_detection` to retrieve the
  unnormalized (absolute) bounding boxes.
- **auxiliary_outputs** (`list[Dict]`, *optional*) -- Optional, only returned when auxiliary losses are activated (i.e. `config.auxiliary_loss` is set to `True`)
  and labels are provided. It is a list of dictionaries containing the two above keys (`logits` and
  `pred_boxes`) for each decoder layer.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, hidden_size)`) -- Stacked intermediate hidden states (output of each layer of the decoder).
- **intermediate_logits** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, config.num_labels)`) -- Stacked intermediate logits (logits of each layer of the decoder).
- **intermediate_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate reference points (reference points of each layer of the decoder).
- **intermediate_predicted_corners** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate predicted corners (predicted corners of each layer of the decoder).
- **initial_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked initial reference points (initial reference points of each layer of the decoder).
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **init_reference_points** (`torch.FloatTensor` of shape  `(batch_size, num_queries, 4)`) -- Initial reference points sent through the Transformer decoder.
- **enc_topk_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the encoder.
- **enc_topk_bboxes** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the encoder.
- **enc_outputs_class** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Predicted bounding boxes scores where the top `config.two_stage_num_proposals` scoring bounding boxes are
  picked as region proposals in the first stage. Output of bounding box binary classification (i.e.
  foreground and background).
- **enc_outputs_coord_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.with_box_refine=True` and `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the first stage.
- **denoising_meta_values** (`dict`, *optional*) -- Extra dictionary for the denoising related values

Example:

```python
>>> import torch
>>> from transformers.image_utils import load_image
>>> from transformers import AutoImageProcessor, DFineForObjectDetection

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = load_image(url)

>>> image_processor = AutoImageProcessor.from_pretrained("ustc-community/dfine-xlarge-coco")
>>> model = DFineForObjectDetection.from_pretrained("ustc-community/dfine-xlarge-coco")

>>> # prepare image for the model
>>> inputs = image_processor(images=image, return_tensors="pt")

>>> # forward pass
>>> outputs = model(**inputs)

>>> logits = outputs.logits
>>> list(logits.shape)
[1, 300, 80]

>>> boxes = outputs.pred_boxes
>>> list(boxes.shape)
[1, 300, 4]

>>> # convert outputs (bounding boxes and class logits) to Pascal VOC format (xmin, ymin, xmax, ymax)
>>> target_sizes = torch.tensor([image.size[::-1]])
>>> results = image_processor.post_process_object_detection(outputs, threshold=0.9, target_sizes=target_sizes)
>>> result = results[0]  # first image in batch

>>> for score, label, box in zip(result["scores"], result["labels"], result["boxes"]):
...     box = [round(i, 2) for i in box.tolist()]
...     print(
...         f"Detected {model.config.id2label[label.item()]} with confidence "
...         f"{round(score.item(), 3)} at location {box}"
...     )
Detected cat with confidence 0.958 at location [344.49, 23.4, 639.84, 374.27]
Detected cat with confidence 0.956 at location [11.71, 53.52, 316.64, 472.33]
Detected remote with confidence 0.947 at location [40.46, 73.7, 175.62, 117.57]
Detected sofa with confidence 0.918 at location [0.59, 1.88, 640.25, 474.74]
```
