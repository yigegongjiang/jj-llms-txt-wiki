# DAB-DETR

## Overview

The DAB-DETR model was proposed in [DAB-DETR: Dynamic Anchor Boxes are Better Queries for DETR](https://huggingface.co/papers/2201.12329) by Shilong Liu, Feng Li, Hao Zhang, Xiao Yang, Xianbiao Qi, Hang Su, Jun Zhu, Lei Zhang.
DAB-DETR is an enhanced variant of Conditional DETR. It utilizes dynamically updated anchor boxes to provide both a reference query point (x, y) and a reference anchor size (w, h), improving cross-attention computation. This new approach achieves 45.7% AP when trained for 50 epochs with a single ResNet-50 model as the backbone.

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/dab_detr_convergence_plot.png"
alt="drawing" width="600"/>

The abstract from the paper is the following:

*We present in this paper a novel query formulation using dynamic anchor boxes
for DETR (DEtection TRansformer) and offer a deeper understanding of the role
of queries in DETR. This new formulation directly uses box coordinates as queries
in Transformer decoders and dynamically updates them layer-by-layer. Using box
coordinates not only helps using explicit positional priors to improve the query-to-feature similarity and eliminate the slow training convergence issue in DETR,
but also allows us to modulate the positional attention map using the box width
and height information. Such a design makes it clear that queries in DETR can be
implemented as performing soft ROI pooling layer-by-layer in a cascade manner.
As a result, it leads to the best performance on MS-COCO benchmark among
the DETR-like detection models under the same setting, e.g., AP 45.7% using
ResNet50-DC5 as backbone trained in 50 epochs. We also conducted extensive
experiments to confirm our analysis and verify the effectiveness of our methods.*

This model was contributed by [davidhajdu](https://huggingface.co/davidhajdu).
The original code can be found [here](https://github.com/IDEA-Research/DAB-DETR).

## How to Get Started with the Model

Use the code below to get started with the model.

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForObjectDetection

url = 'http://images.cocodataset.org/val2017/000000039769.jpg'
image = Image.open(requests.get(url, stream=True).raw)

image_processor = AutoImageProcessor.from_pretrained("IDEA-Research/dab-detr-resnet-50")
model = AutoModelForObjectDetection.from_pretrained("IDEA-Research/dab-detr-resnet-50", device_map="auto")

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

This should output

```text
cat: 0.87 [14.7, 49.39, 320.52, 469.28]
remote: 0.86 [41.08, 72.37, 173.39, 117.2]
cat: 0.86 [344.45, 19.43, 639.85, 367.86]
remote: 0.61 [334.27, 75.93, 367.92, 188.81]
couch: 0.59 [-0.04, 1.34, 639.9, 477.09]
```

There are three other ways to instantiate a DAB-DETR model (depending on what you prefer):

Option 1: Instantiate DAB-DETR with pre-trained weights for entire model

```python
from transformers import DabDetrForObjectDetection

model = DabDetrForObjectDetection.from_pretrained("IDEA-Research/dab-detr-resnet-50", device_map="auto")
```

Option 2: Instantiate DAB-DETR with randomly initialized weights for Transformer, but pre-trained weights for backbone

```python
from transformers import DabDetrConfig, DabDetrForObjectDetection

config = DabDetrConfig()
model = DabDetrForObjectDetection(config)
```

Option 3: Instantiate DAB-DETR with randomly initialized weights for backbone + Transformer

```py
config = DabDetrConfig()
model = DabDetrForObjectDetection(config)
```

## DabDetrConfig[[transformers.DabDetrConfig]]

#### transformers.DabDetrConfig[[transformers.DabDetrConfig]]

```python
transformers.DabDetrConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, num_queries: int = 300, encoder_layers: int = 6, encoder_ffn_dim: int = 2048, encoder_attention_heads: int = 8, decoder_layers: int = 6, decoder_ffn_dim: int = 2048, decoder_attention_heads: int = 8, activation_function: str = 'prelu', hidden_size: int = 256, dropout: float | int = 0.1, attention_dropout: float | int = 0.0, activation_dropout: float | int = 0.0, init_std: float = 0.02, init_xavier_std: float = 1.0, auxiliary_loss: bool = False, dilation: bool = False, class_cost: int = 2, bbox_cost: int = 5, giou_cost: int = 2, cls_loss_coefficient: int = 2, bbox_loss_coefficient: int = 5, giou_loss_coefficient: int = 2, focal_alpha: float = 0.25, temperature_height: int = 20, temperature_width: int = 20, query_dim: int = 4, random_refpoints_xy: bool = False, keep_query_pos: bool = False, num_patterns: int = 0, normalize_before: bool = False, sine_position_embedding_scale: float | None = None, initializer_bias_prior_prob: float | None = None, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dab_detr/configuration_dab_detr.py#L26)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

backbone_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The configuration of the backbone model.

num_queries (`int`, *optional*, defaults to 300) : Number of object queries, i.e. detection slots. This is the maximal number of objects [DabDetrModel](/docs/transformers/v5.15.1/en/model_doc/dab-detr#transformers.DabDetrModel) can detect in a single image. For COCO, we recommend 100 queries.

encoder_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

encoder_ffn_dim (`int`, *optional*, defaults to `2048`) : Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.

encoder_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer encoder.

decoder_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

decoder_ffn_dim (`int`, *optional*, defaults to `2048`) : Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.

decoder_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

activation_function (`str`, *optional*, defaults to `prelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_size (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The ratio for all dropout layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for activations inside the fully connected layer.

init_std (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

init_xavier_std (`float`, *optional*, defaults to `1.0`) : The scaling factor used for the Xavier initialization of the cross-attention weights.

auxiliary_loss (`bool`, *optional*, defaults to `False`) : Whether auxiliary decoding losses (losses at each decoder layer) are to be used.

dilation (`bool`, *optional*, defaults to `False`) : Whether to replace stride with dilation in the last convolutional block (DC5). Only supported when `use_timm_backbone` = `True`.

class_cost (`int`, *optional*, defaults to `2`) : Relative weight of the classification error in the Hungarian matching cost.

bbox_cost (`int`, *optional*, defaults to `5`) : Relative weight of the L1 bounding box error in the Hungarian matching cost.

giou_cost (`int`, *optional*, defaults to `2`) : Relative weight of the generalized IoU loss in the Hungarian matching cost.

cls_loss_coefficient (`int`, *optional*, defaults to `2`) : Relative weight of the classification loss in the panoptic segmentation loss.

bbox_loss_coefficient (`int`, *optional*, defaults to `5`) : Relative weight of the L1 bounding box loss in the panoptic segmentation loss.

giou_loss_coefficient (`int`, *optional*, defaults to `2`) : Relative weight of the generalized IoU loss in the panoptic segmentation loss.

focal_alpha (`float`, *optional*, defaults to `0.25`) : Alpha parameter in the focal loss.

temperature_height (`int`, *optional*, defaults to 20) : Temperature parameter to tune the flatness of positional attention (HEIGHT)

temperature_width (`int`, *optional*, defaults to 20) : Temperature parameter to tune the flatness of positional attention (WIDTH)

query_dim (`int`, *optional*, defaults to 4) : Query dimension parameter represents the size of the output vector.

random_refpoints_xy (`bool`, *optional*, defaults to `False`) : Whether to fix the x and y coordinates of the anchor boxes with random initialization.

keep_query_pos (`bool`, *optional*, defaults to `False`) : Whether to concatenate the projected positional embedding from the object query into the original query (key) in every decoder layer.

num_patterns (`int`, *optional*, defaults to 0) : Number of pattern embeddings.

normalize_before (`bool`, *optional*, defaults to `False`) : Whether we use a normalization layer in the Encoder or not.

sine_position_embedding_scale (`float`, *optional*, defaults to 'None') : Scaling factor applied to the normalized positional encodings.

initializer_bias_prior_prob (`float`, *optional*) : The prior probability used by the bias initializer to initialize biases for `enc_score_head` and `class_embed`. If `None`, `prior_prob` computed as `prior_prob = 1 / (num_labels + 1)` while initializing model weights.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Dab DetrModel. It is used to instantiate a Dab Detr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [IDEA-Research/dab-detr-resnet-50](https://huggingface.co/IDEA-Research/dab-detr-resnet-50)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import DabDetrConfig, DabDetrModel

>>> # Initializing a DAB-DETR IDEA-Research/dab-detr-resnet-50 style configuration
>>> configuration = DabDetrConfig()

>>> # Initializing a model (with random weights) from the IDEA-Research/dab-detr-resnet-50 style configuration
>>> model = DabDetrModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DabDetrModel[[transformers.DabDetrModel]]

#### transformers.DabDetrModel[[transformers.DabDetrModel]]

```python
transformers.DabDetrModel(config: DabDetrConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dab_detr/modeling_dab_detr.py#L1143)

**Parameters:**

config ([DabDetrConfig](/docs/transformers/v5.15.1/en/model_doc/dab-detr#transformers.DabDetrConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare DAB-DETR Model (consisting of a backbone and encoder-decoder Transformer) outputting raw
hidden-states, intermediate hidden states, reference points, output coordinates without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DabDetrModel.forward]]

```python
forward(pixel_values: FloatTensor, pixel_mask: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: typing.Optional[torch.FloatTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dab_detr/modeling_dab_detr.py#L1195)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

pixel_mask (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

decoder_attention_mask (`torch.FloatTensor` of shape `(batch_size, num_queries)`, *optional*) : Not used by default. Can be used to mask object queries.

encoder_outputs (`torch.FloatTensor`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you can choose to directly pass a flattened representation of an image.

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`, *optional*) : Optionally, instead of initializing the queries with a tensor of zeros, you can choose to directly pass an embedded representation.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `DabDetrModelOutput` or `tuple(torch.FloatTensor)`

A `DabDetrModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DabDetrConfig](/docs/transformers/v5.15.1/en/model_doc/dab-detr#transformers.DabDetrConfig)) and inputs.

The [DabDetrModel](/docs/transformers/v5.15.1/en/model_doc/dab-detr#transformers.DabDetrModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the optional initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(config.decoder_layers, batch_size, sequence_length, hidden_size)`, *optional*, returned when `config.auxiliary_loss=True`) -- Intermediate decoder activations, i.e. the output of each decoder layer, each of them gone through a
  layernorm.
- **reference_points** (`torch.FloatTensor` of shape `(config.decoder_layers, batch_size, num_queries, 2 (anchor points))`) -- Reference points (reference points of each layer of the decoder).

Examples:

```python
>>> from transformers import AutoImageProcessor, AutoModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("IDEA-Research/dab-detr-resnet-50")
>>> model = AutoModel.from_pretrained("IDEA-Research/dab-detr-resnet-50")

>>> # prepare image for the model
>>> inputs = image_processor(images=image, return_tensors="pt")

>>> # forward pass
>>> outputs = model(**inputs)

>>> # the last hidden states are the final query embeddings of the Transformer decoder
>>> # these are of shape (batch_size, num_queries, hidden_size)
>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 300, 256]
```

## DabDetrForObjectDetection[[transformers.DabDetrForObjectDetection]]

#### transformers.DabDetrForObjectDetection[[transformers.DabDetrForObjectDetection]]

```python
transformers.DabDetrForObjectDetection(config: DabDetrConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dab_detr/modeling_dab_detr.py#L1415)

**Parameters:**

config ([DabDetrConfig](/docs/transformers/v5.15.1/en/model_doc/dab-detr#transformers.DabDetrConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

DAB_DETR Model (consisting of a backbone and encoder-decoder Transformer) with object detection heads on
top, for tasks such as COCO detection.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DabDetrForObjectDetection.forward]]

```python
forward(pixel_values: FloatTensor, pixel_mask: typing.Optional[torch.LongTensor] = None, decoder_attention_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: typing.Optional[torch.FloatTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, decoder_inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: list[dict] | None = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/dab_detr/modeling_dab_detr.py#L1444)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

pixel_mask (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

decoder_attention_mask (`torch.FloatTensor` of shape `(batch_size, num_queries)`, *optional*) : Not used by default. Can be used to mask object queries.

encoder_outputs (`torch.FloatTensor`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you can choose to directly pass a flattened representation of an image.

decoder_inputs_embeds (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`, *optional*) : Optionally, instead of initializing the queries with a tensor of zeros, you can choose to directly pass an embedded representation.

labels (`list[Dict]` of len `(batch_size,)`, *optional*) : Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `DabDetrObjectDetectionOutput` or `tuple(torch.FloatTensor)`

A `DabDetrObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DabDetrConfig](/docs/transformers/v5.15.1/en/model_doc/dab-detr#transformers.DabDetrConfig)) and inputs.

The [DabDetrForObjectDetection](/docs/transformers/v5.15.1/en/model_doc/dab-detr#transformers.DabDetrForObjectDetection) forward method, overrides the `__call__` special method.

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
  possible padding). You can use `~DabDetrImageProcessor.post_process_object_detection` to retrieve the
  unnormalized bounding boxes.
- **auxiliary_outputs** (`list[Dict]`, *optional*) -- Optional, only returned when auxiliary losses are activated (i.e. `config.auxiliary_loss` is set to `True`)
  and labels are provided. It is a list of dictionaries containing the two above keys (`logits` and
  `pred_boxes`) for each decoder layer.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.
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

Examples:

```python
>>> from transformers import AutoImageProcessor, AutoModelForObjectDetection
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("IDEA-Research/dab-detr-resnet-50")
>>> model = AutoModelForObjectDetection.from_pretrained("IDEA-Research/dab-detr-resnet-50")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> with torch.no_grad():
>>>     outputs = model(**inputs)

>>> # convert outputs (bounding boxes and class logits) to Pascal VOC format (xmin, ymin, xmax, ymax)
>>> target_sizes = torch.tensor([(image.height, image.width)])
>>> results = image_processor.post_process_object_detection(outputs, threshold=0.5, target_sizes=target_sizes)[0]
>>> for score, label, box in zip(results["scores"], results["labels"], results["boxes"]):
...     box = [round(i, 2) for i in box.tolist()]
...     print(
...         f"Detected {model.config.id2label[label.item()]} with confidence "
...         f"{round(score.item(), 3)} at location {box}"
...     )
Detected remote with confidence 0.833 at location [38.31, 72.1, 177.63, 118.45]
Detected cat with confidence 0.831 at location [9.2, 51.38, 321.13, 469.0]
Detected cat with confidence 0.804 at location [340.3, 16.85, 642.93, 370.95]
Detected remote with confidence 0.683 at location [334.48, 73.49, 366.37, 190.01]
Detected couch with confidence 0.535 at location [0.52, 1.19, 640.35, 475.1]
```
