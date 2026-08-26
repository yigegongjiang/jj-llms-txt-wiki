# PP-DocLayoutV3

## Overview

**PP-DocLayoutV3** is a unified and high-efficiency model designed for comprehensive layout analysis. It addresses the challenges of complex physical distortions—such as skewing, curving, and adverse lighting—by integrating instance segmentation and reading order prediction into a single, end-to-end framework.

## Model Architecture

PP-DocLayoutV3 evolves from a traditional detection-based approach to a robust instance segmentation architecture built upon the RT-DETR framework. Instead of simple bounding boxes, it utilizes a mask-based detection head to predict pixel-accurate segments for layout elements. 

Unlike its predecessor, PP-DocLayoutV3 eliminates decoupled stages by embedding a Global Pointer Mechanism directly within the Transformer decoder layers. This allows the model to concurrently output classification labels, precise masks, and logical reading orders in a single forward pass, significantly reducing latency while enhancing parsing precision on complex document layouts.

## Usage

### Single input inference

The example below demonstrates how to generate text with PP-DocLayoutV3 using [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel).

```python
import requests
from PIL import Image

from transformers import pipeline

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/layout_demo.jpg", stream=True).raw)
layout_detector = pipeline("object-detection", model="PaddlePaddle/PP-DocLayoutV3_safetensors")
results = layout_detector(image)
for idx, res in enumerate(results):
    print(f"Order {idx + 1}: {res}")
```

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForObjectDetection

model_path = "PaddlePaddle/PP-DocLayoutV3_safetensors"
model = AutoModelForObjectDetection.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/layout_demo.jpg", stream=True).raw)
inputs = image_processor(images=image, return_tensors="pt").to(model.device)

outputs = model(**inputs)
results = image_processor.post_process_object_detection(outputs, target_sizes=[image.size[::-1]])
for result in results:
    for idx, (score, label_id, box, polygon_points) in enumerate(zip(result["scores"], result["labels"], result["boxes"], result["polygon_points"])):
        score, label = score.item(), label_id.item()
        box = [round(i, 2) for i in box.tolist()]
        print(f"Order {idx + 1}: {model.config.id2label[label]}, score: {score:.2f}, box: {box}, polygon_points: {polygon_points}")
```

### Batched inference

PP-DocLayoutV3 also supports batched inference. Here is how you can do it with PP-DocLayoutV3 using [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel):

```python
import requests
from PIL import Image

from transformers import pipeline

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/layout_demo.jpg", stream=True).raw)
layout_detector = pipeline("object-detection", model="PaddlePaddle/PP-DocLayoutV3_safetensors")
results = layout_detector([image, image])
for result in results:
    print("result:")
    for idx, res in enumerate(result):
        print(f"Order {idx + 1}: {res}")
```

```python
import requests
from PIL import Image

from transformers import AutoImageProcessor, AutoModelForObjectDetection

model_path = "PaddlePaddle/PP-DocLayoutV3_safetensors"
model = AutoModelForObjectDetection.from_pretrained(model_path, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_path)

image = Image.open(requests.get("https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/layout_demo.jpg", stream=True).raw)
inputs = image_processor(images=[image, image], return_tensors="pt").to(model.device)
target_sizes = [image.size[::-1], image.size[::-1]]

outputs = model(**inputs)
results = image_processor.post_process_object_detection(outputs, target_sizes=target_sizes)
for result in results:
    print("result:")
    for idx, (score, label_id, box, polygon_points) in enumerate(zip(result["scores"], result["labels"], result["boxes"], result["polygon_points"])):
        score, label = score.item(), label_id.item()
        box = [round(i, 2) for i in box.tolist()]
        print(f"Order {idx + 1}: {model.config.id2label[label]}, score: {score:.2f}, box: {box}, polygon_points: {polygon_points}")
```

## PPDocLayoutV3ForObjectDetection[[transformers.PPDocLayoutV3ForObjectDetection]]

#### transformers.PPDocLayoutV3ForObjectDetection[[transformers.PPDocLayoutV3ForObjectDetection]]

```python
transformers.PPDocLayoutV3ForObjectDetection(config: PPDocLayoutV3Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_doclayout_v3/modeling_pp_doclayout_v3.py#L2014)

**Parameters:**

config ([PPDocLayoutV3Config](/docs/transformers/v5.15.1/en/model_doc/pp_doclayout_v3#transformers.PPDocLayoutV3Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PP-DocLayoutV3 Model (consisting of a backbone and encoder-decoder) outputs bounding boxes and logits sorted according to reading order,
which are further decoded into scores and classes.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.PPDocLayoutV3ForObjectDetection.forward]]

```python
forward(pixel_values: FloatTensor, pixel_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: typing.Optional[torch.FloatTensor] = None, labels: list[dict] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_doclayout_v3/modeling_pp_doclayout_v3.py#L2032)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

pixel_mask (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

encoder_outputs (`torch.FloatTensor`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

labels (`list[Dict]` of len `(batch_size,)`, *optional*) : Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.

**Returns:** `PPDocLayoutV3ForObjectDetectionOutput` or `tuple(torch.FloatTensor)`

A `PPDocLayoutV3ForObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [PPDocLayoutV3ForObjectDetection](/docs/transformers/v5.15.1/en/model_doc/pp_doclayout_v3#transformers.PPDocLayoutV3ForObjectDetection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **logits** (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes + 1)`) -- Classification logits (including no-object) for all queries.
- **pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- Normalized boxes coordinates for all queries, represented as (center_x, center_y, width, height). These
  values are normalized in [0, 1], relative to the size of each individual image in the batch (disregarding
  possible padding). You can use `post_process_object_detection()` to retrieve the
  unnormalized (absolute) bounding boxes.
- **order_logits** (`tuple` of `torch.FloatTensor` of shape `(batch_size, num_queries, num_queries)`) -- Order logits of the final layer of the decoder.
- **out_masks** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, height, width)`) -- Masks of the final layer of the decoder.
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

>>> model_path = "PaddlePaddle/PP-DocLayoutV3_safetensors"
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
Order 1: text: 0.99 [334.95, 184.78, 897.25, 654.83]
Order 2: paragraph_title: 0.97 [337.28, 683.92, 869.16, 798.35]
Order 3: text: 0.99 [335.75, 842.82, 892.13, 1454.32]
Order 4: text: 0.99 [920.18, 185.28, 1476.38, 464.49]
Order 5: text: 0.98 [920.47, 483.68, 1480.63, 765.72]
Order 6: text: 0.98 [920.62, 846.8, 1482.09, 1220.67]
Order 7: text: 0.97 [920.92, 1239.41, 1469.55, 1378.02]
Order 8: footnote: 0.86 [335.03, 1614.68, 1483.33, 1731.73]
Order 9: footnote: 0.83 [334.64, 1756.74, 1471.78, 1845.69]
Order 10: text: 0.81 [336.8, 1910.52, 661.64, 1939.92]
Order 11: footnote: 0.96 [336.24, 2114.42, 1450.14, 2172.12]
Order 12: number: 0.88 [106.0, 2257.5, 135.84, 2282.18]
Order 13: footer: 0.93 [338.4, 2255.52, 986.15, 2284.37]
```

## PPDocLayoutV3Model[[transformers.PPDocLayoutV3Model]]

#### transformers.PPDocLayoutV3Model[[transformers.PPDocLayoutV3Model]]

```python
transformers.PPDocLayoutV3Model(config: PPDocLayoutV3Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_doclayout_v3/modeling_pp_doclayout_v3.py#L1577)

**Parameters:**

config ([PPDocLayoutV3Config](/docs/transformers/v5.15.1/en/model_doc/pp_doclayout_v3#transformers.PPDocLayoutV3Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

PP-DocLayoutV3 Model (consisting of a backbone and encoder-decoder) outputting raw hidden states without any head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.PPDocLayoutV3Model.forward]]

```python
forward(pixel_values: FloatTensor, pixel_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: typing.Optional[torch.FloatTensor] = None, labels: list[dict] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_doclayout_v3/modeling_pp_doclayout_v3.py#L1714)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

pixel_mask (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

encoder_outputs (`torch.FloatTensor`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

labels (`list[Dict]` of len `(batch_size,)`, *optional*) : Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.

**Returns:** `PPDocLayoutV3ModelOutput` or `tuple(torch.FloatTensor)`

A `PPDocLayoutV3ModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [PPDocLayoutV3Model](/docs/transformers/v5.15.1/en/model_doc/pp_doclayout_v3#transformers.PPDocLayoutV3Model) forward method, overrides the `__call__` special method.

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
- **out_order_logits** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, config.num_queries, config.num_queries)`) -- Stacked order logits (order logits of each layer of the decoder).
- **out_masks** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, config.num_queries, 200, 200)`) -- Stacked masks (masks of each layer of the decoder).

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

## PPDocLayoutV3Config[[transformers.PPDocLayoutV3Config]]

#### transformers.PPDocLayoutV3Config[[transformers.PPDocLayoutV3Config]]

```python
transformers.PPDocLayoutV3Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, initializer_range: float = 0.01, initializer_bias_prior_prob: float | None = None, layer_norm_eps: float = 1e-05, batch_norm_eps: float = 1e-05, tie_word_embeddings: bool = True, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, freeze_backbone_batch_norms: bool = True, encoder_hidden_dim: int = 256, encoder_in_channels: list[int] | tuple[int, ...] = (512, 1024, 2048), feat_strides: list[int] | tuple[int, ...] = (8, 16, 32), encoder_layers: int = 1, encoder_ffn_dim: int = 1024, encoder_attention_heads: int = 8, dropout: float | int = 0.0, activation_dropout: float | int = 0.0, encode_proj_layers: list[int] | tuple[int, ...] = (2,), positional_encoding_temperature: int = 10000, encoder_activation_function: str = 'gelu', activation_function: str = 'silu', eval_size: int | None = None, normalize_before: bool = False, hidden_expansion: float = 1.0, mask_feature_channels: list[int] | tuple[int, ...] = (64, 64), x4_feat_dim: int = 128, d_model: int = 256, num_prototypes: int = 32, label_noise_ratio: float = 0.4, box_noise_scale: float = 0.4, mask_enhanced: bool = True, num_queries: int = 300, decoder_in_channels: list[int] | tuple[int, ...] = (256, 256, 256), decoder_ffn_dim: int = 1024, num_feature_levels: int = 3, decoder_n_points: int = 4, decoder_layers: int = 6, decoder_attention_heads: int = 8, decoder_activation_function: str = 'relu', attention_dropout: float | int = 0.0, num_denoising: int = 100, learn_initial_query: bool = False, anchor_image_size: int | None = None, disable_custom_kernels: bool = True, global_pointer_head_size: int = 64, gp_dropout_value: float | int = 0.1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_doclayout_v3/configuration_pp_doclayout_v3.py#L31)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

initializer_range (`float`, *optional*, defaults to `0.01`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

initializer_bias_prior_prob (`float`, *optional*) : The prior probability used by the bias initializer to initialize biases for `enc_score_head` and `class_embed`. If `None`, `prior_prob` computed as `prior_prob = 1 / (num_labels + 1)` while initializing model weights.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

batch_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the batch normalization layers.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

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

mask_feature_channels (`list[int]`, *optional*, defaults to `[64, 64]`) : The channels of the multi-level features for mask enhancement.

x4_feat_dim (`int`, *optional*, defaults to 128) : The dimension of the x4 feature map.

d_model (`int`, *optional*, defaults to 256) : Dimension of the layers exclude hybrid encoder.

num_prototypes (`int`, *optional*, defaults to 32) : Dimension of the layers exclude mask query head.

label_noise_ratio (`float`, *optional*, defaults to 0.4) : The fraction of denoising labels to which random noise should be added.

box_noise_scale (`float`, *optional*, defaults to 0.4) : Scale or magnitude of noise to be added to the bounding boxes.

mask_enhanced (`bool`, *optional*, defaults to `True`) : Whether to use enhanced masked attention.

num_queries (`int`, *optional*, defaults to 300) : Number of object queries.

decoder_in_channels (`list`, *optional*, defaults to `[256, 256, 256]`) : Multi level features dimension for decoder

decoder_ffn_dim (`int`, *optional*, defaults to 1024) : Dimension of the "intermediate" (often named feed-forward) layer in decoder.

num_feature_levels (`int`, *optional*, defaults to 3) : The number of input feature levels.

decoder_n_points (`int`, *optional*, defaults to 4) : The number of sampled keys in each feature level for each attention head in the decoder.

decoder_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

decoder_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

decoder_activation_function (`str`, *optional*, defaults to `"relu"`) : The non-linear activation function (function or string) in the decoder. If string, `"gelu"`, `"relu"`, `"silu"` and `"gelu_new"` are supported.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

num_denoising (`int`, *optional*, defaults to 100) : The total number of denoising tasks or queries to be used for contrastive denoising.

learn_initial_query (`bool`, *optional*, defaults to `False`) : Indicates whether the initial query embeddings for the decoder should be learned during training

anchor_image_size (`tuple[int, int]`, *optional*) : Height and width of the input image used during evaluation to generate the bounding box anchors. If None, automatic generate anchor is applied.

disable_custom_kernels (`bool`, *optional*, defaults to `True`) : Whether to disable custom kernels.

global_pointer_head_size (`int`, *optional*, defaults to 64) : The size of the global pointer head.

gp_dropout_value (`float`, *optional*, defaults to 0.1) : The dropout probability in the global pointer head.

This is the configuration class to store the configuration of a PPDocLayoutV3Model. It is used to instantiate a Pp Doclayout V3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PP-DocLayoutV3_safetensors](https://huggingface.co/PaddlePaddle/PP-DocLayoutV3_safetensors)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import PPDocLayoutV3Config, PPDocLayoutV3ForObjectDetection

>>> # Initializing a PP-DocLayoutV3 configuration
>>> configuration = PPDocLayoutV3Config()

>>> # Initializing a model (with random weights) from the configuration
>>> model = PPDocLayoutV3ForObjectDetection(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PPDocLayoutV3ImageProcessor[[transformers.PPDocLayoutV3ImageProcessor]]

#### transformers.PPDocLayoutV3ImageProcessor[[transformers.PPDocLayoutV3ImageProcessor]]

```python
transformers.PPDocLayoutV3ImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/pp_doclayout_v3/image_processing_pp_doclayout_v3.py#L39)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 800, 'width': 800}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0, 0, 0]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[1, 1, 1]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models. Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models. Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models. Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors. --

Constructs a PPDocLayoutV3ImageProcessor image processor.

disable_grouping (`bool`, *kwargs*, *optional*):
Whether to disable grouping of images by size to process them individually and not in batches.
If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on
empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157
image_seq_length (`int`, *kwargs*, *optional*):
The number of image tokens to be used for each image in the input.
Added for backward compatibility but this should be set as a processor attribute in future models.

#### preprocess[[transformers.PPDocLayoutV3ImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], *args, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L382)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
