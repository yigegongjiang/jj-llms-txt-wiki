# PP-DocLayoutV2

## Overview

**PP-DocLayoutV2** is a dedicated lightweight model for layout analysis, focusing specifically on element detection, classification, and reading order prediction. 

## Model Architecture

PP-DocLayoutV2 is composed of two sequentially connected networks. The first is an RT-DETR-based detection model that performs layout element detection and classification. The detected bounding boxes and class labels are then passed to a subsequent pointer network, which is responsible for ordering these layout elements.

## Usage

### Single input inference

The example below demonstrates how to generate text with PP-DocLayoutV2 using [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
import requests
from PIL import Image

from transformers import pipeline

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/layout_demo.jpg", stream=True).raw)
layout_detector = pipeline("object-detection", model="PaddlePaddle/PP-DocLayoutV2_safetensors")
result = layout_detector(image)
print(result)
```

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForObjectDetection

model_path = "PaddlePaddle/PP-DocLayoutV2_safetensors"
model = AutoModelForObjectDetection.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)
image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/layout_demo.jpg", stream=True).raw)
inputs = image_processor(images=image, return_tensors="pt").to(model.device)

outputs = model(**inputs)
results = image_processor.post_process_object_detection(outputs, target_sizes=[image.size[::-1]])
for result in results:
    print(result["scores"])
    print(result["labels"])
    print(result["boxes"])
    for idx, (score, label_id, box) in enumerate(zip(result["scores"], result["labels"], result["boxes"])):
        score, label = score.item(), label_id.item()
        box = [round(i, 2) for i in box.tolist()]
        print(f"Order {idx + 1}: {model.config.id2label[label]}: {score:.2f} {box}")
```

### Batched inference

Here is how you can do it with PP-DocLayoutV2 using [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel):

```python
import requests
from PIL import Image

from transformers import pipeline

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/layout_demo.jpg", stream=True).raw)
layout_detector = pipeline("object-detection", model="PaddlePaddle/PP-DocLayoutV2_safetensors")
result = layout_detector([image, image])
print(result[0])
print(result[1])
```

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForObjectDetection

model_path = "PaddlePaddle/PP-DocLayoutV2_safetensors"
model = AutoModelForObjectDetection.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/layout_demo.jpg", stream=True).raw)
inputs = image_processor(images=[image, image], return_tensors="pt").to(model.device)
target_sizes = [image.size[::-1], image.size[::-1]]

outputs = model(**inputs)
results = image_processor.post_process_object_detection(outputs, target_sizes=target_sizes)
for result in results:
    print("result:")
    for idx, (score, label_id, box) in enumerate(zip(result["scores"], result["labels"], result["boxes"])):
        score, label = score.item(), label_id.item()
        box = [round(i, 2) for i in box.tolist()]
        print(f"Order {idx + 1}: {model.config.id2label[label]}: {score:.2f} {box}")
```

## PPDocLayoutV2Config[[transformers.PPDocLayoutV2Config]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **initializer_range** (`float`, *optional*, defaults to `0.01`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **initializer_bias_prior_prob** (`float`, *optional*) --
  The prior probability used by the bias initializer to initialize biases for `enc_score_head` and `class_embed`.
  If `None`, `prior_prob` computed as `prior_prob = 1 / (num_labels + 1)` while initializing model weights.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **batch_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the batch normalization layers.
- **backbone_config** (`Union[~configuration_utils.PreTrainedConfig, dict]`, *optional*) --
  The configuration of the backbone model.
- **freeze_backbone_batch_norms** (`bool`, *optional*, defaults to `True`) --
  Whether to freeze the batch normalization layers in the backbone.
- **encoder_hidden_dim** (`int`, *optional*, defaults to 256) --
  Dimension of the layers in hybrid encoder.
- **encoder_in_channels** (`list`, *optional*, defaults to `[512, 1024, 2048]`) --
  Multi level features input for encoder.
- **feat_strides** (`list[int]`, *optional*, defaults to `[8, 16, 32]`) --
  Strides used in each feature map.
- **encoder_layers** (`int`, *optional*, defaults to 1) --
  Total of layers to be used by the encoder.
- **encoder_ffn_dim** (`int`, *optional*, defaults to `1024`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.
- **encoder_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer encoder.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for activations inside the fully connected layer.
- **encode_proj_layers** (`list[int]`, *optional*, defaults to `[2]`) --
  Indexes of the projected layers to be used in the encoder.
- **positional_encoding_temperature** (`int`, *optional*, defaults to 10000) --
  The temperature parameter used to create the positional encodings.
- **encoder_activation_function** (`str`, *optional*, defaults to `"gelu"`) --
  The non-linear activation function (function or string) in the encoder and pooler. If string, `"gelu"`,
  `"relu"`, `"silu"` and `"gelu_new"` are supported.
- **activation_function** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **eval_size** (`tuple[int, int]`, *optional*) --
  Height and width used to computes the effective height and width of the position embeddings after taking
  into account the stride.
- **normalize_before** (`bool`, *optional*, defaults to `False`) --
  Determine whether to apply layer normalization in the transformer encoder layer before self-attention and
  feed-forward modules.
- **hidden_expansion** (`float`, *optional*, defaults to 1.0) --
  Expansion ratio to enlarge the dimension size of RepVGGBlock and CSPRepLayer.
- **d_model** (`int`, *optional*, defaults to `256`) --
  Size of the encoder layers and the pooler layer.
- **num_queries** (`int`, *optional*, defaults to 300) --
  Number of object queries.
- **decoder_in_channels** (`list`, *optional*, defaults to `[256, 256, 256]`) --
  Multi level features dimension for decoder
- **decoder_ffn_dim** (`int`, *optional*, defaults to 1024) --
  Dimension of the "intermediate" (often named feed-forward) layer in decoder.
- **num_feature_levels** (`int`, *optional*, defaults to 3) --
  The number of input feature levels.
- **decoder_n_points** (`int`, *optional*, defaults to 4) --
  The number of sampled keys in each feature level for each attention head in the decoder.
- **decoder_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.
- **decoder_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **decoder_activation_function** (`str`, *optional*, defaults to `"relu"`) --
  The non-linear activation function (function or string) in the decoder. If string, `"gelu"`,
  `"relu"`, `"silu"` and `"gelu_new"` are supported.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **num_denoising** (`int`, *optional*, defaults to 100) --
  The total number of denoising tasks or queries to be used for contrastive denoising.
- **label_noise_ratio** (`float`, *optional*, defaults to 0.5) --
  The fraction of denoising labels to which random noise should be added.
- **box_noise_scale** (`float`, *optional*, defaults to 1.0) --
  Scale or magnitude of noise to be added to the bounding boxes.
- **learn_initial_query** (`bool`, *optional*, defaults to `False`) --
  Indicates whether the initial query embeddings for the decoder should be learned during training
- **anchor_image_size** (`tuple[int, int]`, *optional*) --
  Height and width of the input image used during evaluation to generate the bounding box anchors. If None, automatic generate anchor is applied.
- **disable_custom_kernels** (`bool`, *optional*, defaults to `True`) --
  Whether to disable custom kernels.
- **class_thresholds** (`list[float]`, *optional*) --
  The thresholds for each label.
- **class_order** (`list[int]`, *optional*) --
  The priority for each label.
- **reading_order_config** (`dict`, *optional*) --
  The configuration of a `PPDocLayoutV2ReadingOrder`.

This is the configuration class to store the configuration of a Pp Doclayout V2Model. It is used to instantiate a Pp Doclayout V2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PP-DocLayoutV2_safetensors](https://huggingface.co/PaddlePaddle/PP-DocLayoutV2_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import PPDocLayoutV2Config, PPDocLayoutV2ForObjectDetection

>>> # Initializing a PP-DocLayoutV2 configuration
>>> configuration = PPDocLayoutV2Config()

>>> # Initializing a model (with random weights) from the configuration
>>> model = PPDocLayoutV2ForObjectDetection(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PPDocLayoutV2ForObjectDetection[[transformers.PPDocLayoutV2ForObjectDetection]]

- **config** ([PPDocLayoutV2Config](/docs/transformers/v5.14.0/en/model_doc/pp_doclayout_v2#transformers.PPDocLayoutV2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PP-DocLayoutV2 Model (consisting of a backbone and encoder-decoder) outputting bounding boxes, logits and order_logits to be further
decoded into scores, classes and their reading order.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses
  `image_processor_class` for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **encoder_outputs** (`torch.FloatTensor`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you
  can choose to directly pass a flattened representation of an image.
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`, *optional*) --
  Optionally, instead of initializing the queries with a tensor of zeros, you can choose to directly pass an
  embedded representation.
- **labels** (`list[Dict]` of len `(batch_size,)`, *optional*) --
  Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the
  following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch
  respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes
  in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.</paramsdesc><rettype>`PPDocLayoutV2ForObjectDetectionOutput` or `tuple(torch.FloatTensor)`</rettype><retdesc>A `PPDocLayoutV2ForObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [PPDocLayoutV2ForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/pp_doclayout_v2#transformers.PPDocLayoutV2ForObjectDetection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **logits** (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes + 1)`) -- Classification logits (including no-object) for all queries.
- **pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- Normalized boxes coordinates for all queries, represented as (center_x, center_y, width, height). These
  values are normalized in [0, 1], relative to the size of each individual image in the batch (disregarding
  possible padding). You can use [post_process_object_detection()](/docs/transformers/v5.14.0/en/model_doc/rt_detr#transformers.RTDetrImageProcessor.post_process_object_detection) to retrieve the
  unnormalized (absolute) bounding boxes.
- **order_logits** (`tuple` of `torch.FloatTensor` of shape `(batch_size, num_queries, num_queries)`) -- Order logits for all queries. The first dimension of each tensor is the batch size. The second dimension is the number of queries.
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

Examples:

```python
>>> from transformers import AutoModelForObjectDetection, AutoImageProcessor
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> import torch

>>> url = "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/layout_demo.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> model_path = "PaddlePaddle/PP-DocLayoutV2_safetensors"
>>> image_processor = AutoImageProcessor.from_pretrained(model_path)
>>> model = AutoModelForObjectDetection.from_pretrained(model_path)

>>> # prepare image for the model
>>> inputs = image_processor(images=[image], return_tensors="pt")

>>> # forward pass
>>> outputs = model(**inputs)

>>> # convert outputs (bounding boxes and class logits) to Pascal VOC format (xmin, ymin, xmax, ymax)
>>> results = image_processor.post_process_object_detection(outputs, target_sizes=torch.tensor([image.size[::-1]]))

>>> # print outputs
>>> for result in results:
...     for idx, (score, label_id, box) in enumerate(zip(result["scores"], result["labels"], result["boxes"])):
...         score, label = score.item(), label_id.item()
...         box = [round(i, 2) for i in box.tolist()]
...         print(f"Order {idx + 1}: {model.config.id2label[label]}: {score:.2f} {box}")
Order 1: text: 0.99 [335.39, 184.26, 896.49, 654.48]
Order 2: paragraph_title: 0.97 [337.14, 683.49, 869.42, 798.27]
Order 3: text: 0.99 [335.71, 843.04, 891.17, 1454.15]
Order 4: text: 0.99 [920.42, 185.53, 1476.39, 464.25]
Order 5: text: 0.98 [920.62, 483.75, 1480.52, 765.34]
Order 6: text: 0.98 [920.58, 846.75, 1481.94, 1220.53]
Order 7: text: 0.97 [921.12, 1239.27, 1468.87, 1377.33]
Order 8: footnote: 0.82 [334.58, 1614.67, 1483.84, 1731.61]
Order 9: text: 0.51 [334.58, 1614.67, 1483.84, 1731.61]
Order 10: footnote: 0.83 [334.7, 1757.26, 1471.07, 1845.33]
Order 11: text: 0.87 [336.65, 1910.28, 661.33, 1939.92]
Order 12: footnote: 0.95 [336.16, 2114.52, 1450.28, 2171.74]
Order 13: number: 0.87 [106.04, 2257.37, 136.05, 2281.98]
Order 14: footer: 0.93 [338.6, 2255.94, 985.67, 2283.57]
```

## PPDocLayoutV2Model[[transformers.PPDocLayoutV2Model]]

- **config** ([PPDocLayoutV2Config](/docs/transformers/v5.14.0/en/model_doc/pp_doclayout_v2#transformers.PPDocLayoutV2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PP-DocLayoutV2 Model (consisting of a backbone and encoder-decoder) outputting raw hidden states without any head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses
  `image_processor_class` for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **encoder_outputs** (`torch.FloatTensor`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you
  can choose to directly pass a flattened representation of an image.
- **labels** (`list[Dict]` of len `(batch_size,)`, *optional*) --
  Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the
  following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch
  respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes
  in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.</paramsdesc><rettype>`PPDocLayoutV2ModelOutput` or `tuple(torch.FloatTensor)`</rettype><retdesc>A `PPDocLayoutV2ModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.
The [PPDocLayoutV2Model](/docs/transformers/v5.14.0/en/model_doc/pp_doclayout_v2#transformers.PPDocLayoutV2Model) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, PPDocLayoutV2Model
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("PekingU/PPDocLayoutV2_r50vd")
>>> model = PPDocLayoutV2Model.from_pretrained("PekingU/PPDocLayoutV2_r50vd")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)

>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 300, 256]
```

## PPDocLayoutV2ReadingOrder[[transformers.PPDocLayoutV2ReadingOrder]]

- **config** ([PPDocLayoutV2ReadingOrder](/docs/transformers/v5.14.0/en/model_doc/pp_doclayout_v2#transformers.PPDocLayoutV2ReadingOrder)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PP-DocLayoutV2 ReadingOrder Model. This model consists of an encoder and a GlobalPointer head.
It takes layout features as input and outputs logits representing the relative ordering relationships
between elements, which are used to determine the final reading sequence.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **boxes** (`torch.Tensor` of shape `(batch_size, sequence_length, 4)`) --
  Bounding box coordinates of the detected layout elements **in [0, 1000] scale**.
  Format is `[x_min, y_min, x_max, y_max]`.
  The tensor usually contains sorted valid boxes followed by zero-padding.
- **labels** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  The **remapped** class indices for each layout element.
  These are not necessarily the raw detection class IDs, but indices mapped via
  `config.class_order` (e.g., mapping text/title/figure to specific reading-order category IDs).
- **mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Boolean or Binary mask indicating valid detected elements after threshold filtering.
  - True: Valid layout element.
  - False: Padding/Empty element.
  Used to determine the sequence length (`num_pred`) for the pointer mechanism.
The [PPDocLayoutV2ReadingOrder](/docs/transformers/v5.14.0/en/model_doc/pp_doclayout_v2#transformers.PPDocLayoutV2ReadingOrder) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

## PPDocLayoutV2ImageProcessor[[transformers.PPDocLayoutV2ImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PPDocLayoutV2ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
