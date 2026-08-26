# OmDet-Turbo

## Overview

The OmDet-Turbo model was proposed in [Real-time Transformer-based Open-Vocabulary Detection with Efficient Fusion Head](https://huggingface.co/papers/2403.06892) by Tiancheng Zhao, Peng Liu, Xuan He, Lu Zhang, Kyusong Lee. OmDet-Turbo incorporates components from RT-DETR and introduces a swift multimodal fusion module to achieve real-time open-vocabulary object detection capabilities while maintaining high accuracy. The base model achieves performance of up to 100.2 FPS and 53.4 AP on COCO zero-shot.

The abstract from the paper is the following:

*End-to-end transformer-based detectors (DETRs) have shown exceptional performance in both closed-set and open-vocabulary object detection (OVD) tasks through the integration of language modalities. However, their demanding computational requirements have hindered their practical application in real-time object detection (OD) scenarios. In this paper, we scrutinize the limitations of two leading models in the OVDEval benchmark, OmDet and Grounding-DINO, and introduce OmDet-Turbo. This novel transformer-based real-time OVD model features an innovative Efficient Fusion Head (EFH) module designed to alleviate the bottlenecks observed in OmDet and Grounding-DINO. Notably, OmDet-Turbo-Base achieves a 100.2 frames per second (FPS) with TensorRT and language cache techniques applied. Notably, in zero-shot scenarios on COCO and LVIS datasets, OmDet-Turbo achieves performance levels nearly on par with current state-of-the-art supervised models. Furthermore, it establishes new state-of-the-art benchmarks on ODinW and OVDEval, boasting an AP of 30.1 and an NMS-AP of 26.86, respectively. The practicality of OmDet-Turbo in industrial applications is underscored by its exceptional performance on benchmark datasets and superior inference speed, positioning it as a compelling choice for real-time object detection tasks.*

 OmDet-Turbo architecture overview. Taken from the original paper. 

This model was contributed by [yonigozlan](https://huggingface.co/yonigozlan).
The original code can be found [here](https://github.com/om-ai-lab/OmDet).

## Usage tips

One unique property of OmDet-Turbo compared to other zero-shot object detection models, such as [Grounding DINO](grounding-dino), is the decoupled classes and prompt embedding structure that allows caching of text embeddings. This means that the model needs both classes and task as inputs, where classes is a list of objects we want to detect and task is the grounded text used to guide open-vocabulary detection. This approach limits the scope of the open-vocabulary detection and makes the decoding process faster.

[OmDetTurboProcessor](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboProcessor) is used to prepare the classes, task and image triplet. The task input is optional, and when not provided, it will default to `"Detect [class1], [class2], [class3], ..."`. To process the results from the model, one can use `post_process_grounded_object_detection` from [OmDetTurboProcessor](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboProcessor). Notably, this function takes in the input classes, as unlike other zero-shot object detection models, the decoupling of classes and task embeddings means that no decoding of the predicted class embeddings is needed in the post-processing step, and the predicted classes can be matched to the inputted ones directly.

## Usage example

### Single image inference

Here's how to load the model and prepare the inputs to perform zero-shot object detection on a single image:

```python
import torch
import requests
from PIL import Image

from transformers import AutoProcessor, OmDetTurboForObjectDetection

processor = AutoProcessor.from_pretrained("omlab/omdet-turbo-swin-tiny-hf")
model = OmDetTurboForObjectDetection.from_pretrained("omlab/omdet-turbo-swin-tiny-hf", device_map="auto")

url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = Image.open(requests.get(url, stream=True).raw)
text_labels = ["cat", "remote"]
inputs = processor(image, text=text_labels, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

# convert outputs (bounding boxes and class logits)
results = processor.post_process_grounded_object_detection(
    outputs,
    target_sizes=[(image.height, image.width)],
    text_labels=text_labels,
    threshold=0.3,
    nms_threshold=0.3,
)
result = results[0]
boxes, scores, text_labels = result["boxes"], result["scores"], result["text_labels"]
for box, score, text_label in zip(boxes, scores, text_labels):
    box = [round(i, 2) for i in box.tolist()]
    print(f"Detected {text_label} with confidence {round(score.item(), 3)} at location {box}")
Detected remote with confidence 0.768 at location [39.89, 70.35, 176.74, 118.04]
Detected cat with confidence 0.72 at location [11.6, 54.19, 314.8, 473.95]
Detected remote with confidence 0.563 at location [333.38, 75.77, 370.7, 187.03]
Detected cat with confidence 0.552 at location [345.15, 23.95, 639.75, 371.67]
```

### Multi image inference

OmDet-Turbo can perform batched multi-image inference, with support for different text prompts and classes in the same batch:

```python
import torch
import requests
from io import BytesIO
from PIL import Image
from transformers import AutoProcessor, OmDetTurboForObjectDetection

processor = AutoProcessor.from_pretrained("omlab/omdet-turbo-swin-tiny-hf")
model = OmDetTurboForObjectDetection.from_pretrained("omlab/omdet-turbo-swin-tiny-hf", device_map="auto")

url1 = "http://images.cocodataset.org/val2017/000000039769.jpg"
image1 = Image.open(BytesIO(requests.get(url1).content)).convert("RGB")
text_labels1 = ["cat", "remote"]
task1 = "Detect {}.".format(", ".join(text_labels1))

url2 = "http://images.cocodataset.org/train2017/000000257813.jpg"
image2 = Image.open(BytesIO(requests.get(url2).content)).convert("RGB")
text_labels2 = ["boat"]
task2 = "Detect everything that looks like a boat."

url3 = "https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg"
image3 = Image.open(BytesIO(requests.get(url3).content)).convert("RGB")
text_labels3 = ["statue", "trees"]
task3 = "Focus on the foreground, detect statue and trees."

inputs = processor(
    images=[image1, image2, image3],
    text=[text_labels1, text_labels2, text_labels3],
    task=[task1, task2, task3],
    return_tensors="pt",
)

with torch.no_grad():
    outputs = model(**inputs)

# convert outputs (bounding boxes and class logits)
results = processor.post_process_grounded_object_detection(
    outputs,
    text_labels=[text_labels1, text_labels2, text_labels3],
    target_sizes=[(image.height, image.width) for image in [image1, image2, image3]],
    threshold=0.2,
    nms_threshold=0.3,
)

for i, result in enumerate(results):
    for score, text_label, box in zip(
        result["scores"], result["text_labels"], result["boxes"]
    ):
        box = [round(i, 1) for i in box.tolist()]
        print(
            f"Detected {text_label} with confidence "
            f"{round(score.item(), 2)} at location {box} in image {i}"
        )
Detected remote with confidence 0.77 at location [39.9, 70.4, 176.7, 118.0] in image 0
Detected cat with confidence 0.72 at location [11.6, 54.2, 314.8, 474.0] in image 0
Detected remote with confidence 0.56 at location [333.4, 75.8, 370.7, 187.0] in image 0
Detected cat with confidence 0.55 at location [345.2, 24.0, 639.8, 371.7] in image 0
Detected boat with confidence 0.32 at location [146.9, 219.8, 209.6, 250.7] in image 1
Detected boat with confidence 0.3 at location [319.1, 223.2, 403.2, 238.4] in image 1
Detected boat with confidence 0.27 at location [37.7, 220.3, 84.0, 235.9] in image 1
Detected boat with confidence 0.22 at location [407.9, 207.0, 441.7, 220.2] in image 1
Detected statue with confidence 0.73 at location [544.7, 210.2, 651.9, 502.8] in image 2
Detected trees with confidence 0.25 at location [3.9, 584.3, 391.4, 785.6] in image 2
Detected trees with confidence 0.25 at location [1.4, 621.2, 118.2, 787.8] in image 2
Detected statue with confidence 0.2 at location [428.1, 205.5, 767.3, 759.5] in image 2

```

## OmDetTurboConfig[[transformers.OmDetTurboConfig]]

#### transformers.OmDetTurboConfig[[transformers.OmDetTurboConfig]]

```python
transformers.OmDetTurboConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, apply_layernorm_after_vision_backbone: bool = True, image_size: int | list[int] | tuple[int, int] = 640, disable_custom_kernels: bool = False, layer_norm_eps: float = 1e-05, batch_norm_eps: float = 1e-05, init_std: float = 0.02, text_projection_in_dim: int = 512, text_projection_out_dim: int = 512, task_encoder_hidden_dim: int = 1024, class_embed_dim: int = 512, class_distance_type: typing.Literal['cosine', 'dot'] = 'cosine', num_queries: int = 900, csp_activation: str = 'silu', conv_norm_activation: str = 'gelu', encoder_feedforward_activation: str = 'relu', encoder_feedforward_dropout: float | int = 0.0, encoder_dropout: float | int = 0.0, hidden_expansion: int = 1, encoder_hidden_dim: int = 256, vision_features_channels: list[int] | tuple[int, ...] = (256, 256, 256), encoder_in_channels: list[int] | tuple[int, ...] = (192, 384, 768), encoder_projection_indices: list[int] | tuple[int, ...] = (2,), encoder_attention_heads: int = 8, encoder_dim_feedforward: int = 2048, encoder_layers: int = 1, positional_encoding_temperature: int = 10000, num_feature_levels: int = 3, decoder_hidden_dim: int = 256, decoder_num_heads: int = 8, decoder_num_layers: int = 6, decoder_activation: str = 'relu', decoder_dim_feedforward: int = 2048, decoder_num_points: int = 4, decoder_dropout: float | int = 0.0, eval_size: int | None = None, learn_initial_query: bool = False, cache_size: int = 100)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/omdet_turbo/configuration_omdet_turbo.py#L31)

**Parameters:**

is_encoder_decoder (`bool`, *optional*, defaults to `True`) : Whether the model is used as an encoder/decoder or not.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

backbone_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The configuration of the backbone model.

apply_layernorm_after_vision_backbone (`bool`, *optional*, defaults to `True`) : Whether to apply layer normalization on the feature maps of the vision backbone output.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `640`) : The size (resolution) of each image.

disable_custom_kernels (`bool`, *optional*, defaults to `False`) : Whether to disable custom kernels.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

batch_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the batch normalization layers.

init_std (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

text_projection_in_dim (`int`, *optional*, defaults to 512) : The input dimension for the text projection.

text_projection_out_dim (`int`, *optional*, defaults to 512) : The output dimension for the text projection.

task_encoder_hidden_dim (`int`, *optional*, defaults to 1024) : The feedforward dimension for the task encoder.

class_embed_dim (`int`, *optional*, defaults to 512) : The dimension of the classes embeddings.

class_distance_type (`str`, *optional*, defaults to `"cosine"`) : The type of distance to compare predicted classes to projected classes embeddings. Can be `"cosine"` or `"dot"`.

num_queries (`int`, *optional*, defaults to 900) : The number of queries.

csp_activation (`str`, *optional*, defaults to `"silu"`) : The activation function of the Cross Stage Partial (CSP) networks of the encoder.

conv_norm_activation (`str`, *optional*, defaults to `"gelu"`) : The activation function of the ConvNormLayer layers of the encoder.

encoder_feedforward_activation (`str`, *optional*, defaults to `"relu"`) : The activation function for the feedforward network of the encoder.

encoder_feedforward_dropout (`float`, *optional*, defaults to 0.0) : The dropout rate following the activation of the encoder feedforward network.

encoder_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The ratio for all dropout layers.

hidden_expansion (`int`, *optional*, defaults to 1) : The hidden expansion of the CSP networks in the encoder.

encoder_hidden_dim (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

vision_features_channels (`tuple(int)`, *optional*, defaults to `[256, 256, 256]`) : The projected vision features channels used as inputs for the decoder.

encoder_in_channels (`List(int)`, *optional*, defaults to `[192, 384, 768]`) : The input channels for the encoder.

encoder_projection_indices (`List(int)`, *optional*, defaults to `[2]`) : The indices of the input features projected by each layers.

encoder_attention_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer encoder.

encoder_dim_feedforward (`int`, *optional*, defaults to 2048) : The feedforward dimension for the encoder.

encoder_layers (`int`, *optional*, defaults to `1`) : Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.

positional_encoding_temperature (`int`, *optional*, defaults to 10000) : The positional encoding temperature in the encoder.

num_feature_levels (`int`, *optional*, defaults to 3) : The number of feature levels for the multi-scale deformable attention module of the decoder.

decoder_hidden_dim (`int`, *optional*, defaults to `256`) : Dimension of the hidden representations.

decoder_num_heads (`int`, *optional*, defaults to `8`) : Number of attention heads for each attention layer in the Transformer decoder.

decoder_num_layers (`int`, *optional*, defaults to `6`) : Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.

decoder_activation (`str`, *optional*, defaults to `"relu"`) : The activation function for the decoder.

decoder_dim_feedforward (`int`, *optional*, defaults to 2048) : The feedforward dimension for the decoder.

decoder_num_points (`int`, *optional*, defaults to 4) : The number of points sampled in the decoder multi-scale deformable attention module.

decoder_dropout (`float`, *optional*, defaults to 0.0) : The dropout rate for the decoder.

eval_size (`tuple[int, int]`, *optional*) : Height and width used to computes the effective height and width of the position embeddings after taking into account the stride (see RTDetr).

learn_initial_query (`bool`, *optional*, defaults to `False`) : Whether to learn the initial query.

cache_size (`int`, *optional*, defaults to 100) : The cache size for the classes and prompts caches.

This is the configuration class to store the configuration of a Omdet TurboModel. It is used to instantiate a Omdet Turbo
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [omlab/omdet-turbo-swin-tiny-hf](https://huggingface.co/omlab/omdet-turbo-swin-tiny-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import OmDetTurboConfig, OmDetTurboForObjectDetection

>>> # Initializing a OmDet-Turbo omlab/omdet-turbo-swin-tiny-hf style configuration
>>> configuration = OmDetTurboConfig()

>>> # Initializing a model (with random weights) from the omlab/omdet-turbo-swin-tiny-hf style configuration
>>> model = OmDetTurboForObjectDetection(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## OmDetTurboProcessor[[transformers.OmDetTurboProcessor]]

#### transformers.OmDetTurboProcessor[[transformers.OmDetTurboProcessor]]

```python
transformers.OmDetTurboProcessor(image_processor, tokenizer)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/omdet_turbo/processing_omdet_turbo.py#L190)

**Parameters:**

image_processor (`DetrImageProcessor`) : The image processor is a required input.

tokenizer (`CLIPTokenizer`) : The tokenizer is a required input.

Constructs a OmDetTurboProcessor which wraps a image processor and a tokenizer into a single processor.

[OmDetTurboProcessor](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboProcessor) offers all the functionalities of [DetrImageProcessor](/docs/transformers/v5.15.1/en/model_doc/detr#transformers.DetrImageProcessor) and [CLIPTokenizer](/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer). See the
[~DetrImageProcessor](/docs/transformers/v5.15.1/en/model_doc/detr#transformers.DetrImageProcessor) and [~CLIPTokenizer](/docs/transformers/v5.15.1/en/model_doc/clip#transformers.CLIPTokenizer) for more information.

#### __call__[[transformers.OmDetTurboProcessor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: list[str] | list[list[str]] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/omdet_turbo/processing_omdet_turbo.py#L196)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

task (`str`, *kwargs*, `list[str]`, `TextInput`, or `PreTokenizedInput`, *optional*) : The detection task description(s) to encode. If not provided, a default task description is generated from the `text` input (e.g., "Detect {text}."). Can be a single string, a list of strings (one per image), or pre-tokenized input. The task description guides the model on what objects to detect in the images.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** `~feature_extraction_utils.BatchFeature`

- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the __call__/pad methods ('input_values', 'attention_mask',
  etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **skip_tensor_conversion** (`list[str]` or `set[str]`, *optional*) -- List or set of keys that should NOT be converted to tensors, even when `tensor_type` is specified.

#### post_process_grounded_object_detection[[transformers.OmDetTurboProcessor.post_process_grounded_object_detection]]

```python
post_process_grounded_object_detection(outputs: OmDetTurboObjectDetectionOutput, text_labels: list[str] | list[list[str]] | None = None, threshold: float = 0.3, nms_threshold: float = 0.5, target_sizes: transformers.utils.generic.TensorType | list[tuple] | None = None, max_num_det: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/omdet_turbo/processing_omdet_turbo.py#L266)

**Parameters:**

outputs (`OmDetTurboObjectDetectionOutput`) : Raw outputs of the model.

text_labels (Union[list[str], list[list[str]]], *optional*) : The input classes names. If not provided, `text_labels` will be set to `None` in `outputs`.

threshold (float, defaults to 0.3) : Only return detections with a confidence score exceeding this threshold.

nms_threshold (float, defaults to 0.5) : The threshold to use for box non-maximum suppression. Value in [0, 1].

target_sizes (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) : Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size `(height, width)` of each image in the batch. If unset, predictions will not be resized.

max_num_det (`int`, *optional*) : The maximum number of detections to return.

**Returns:** `list[Dict]`

A list of dictionaries, each dictionary containing the scores, classes and boxes for an image
in the batch as predicted by the model.

Converts the raw output of [OmDetTurboForObjectDetection](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format and get the associated text class.

## OmDetTurboForObjectDetection[[transformers.OmDetTurboForObjectDetection]]

#### transformers.OmDetTurboForObjectDetection[[transformers.OmDetTurboForObjectDetection]]

```python
transformers.OmDetTurboForObjectDetection(config: OmDetTurboConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/omdet_turbo/modeling_omdet_turbo.py#L1464)

**Parameters:**

config ([OmDetTurboConfig](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

OmDetTurbo Model (consisting of a vision and a text backbone, and encoder-decoder architecture) outputting
bounding boxes and classes scores for tasks such as COCO detection.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.OmDetTurboForObjectDetection.forward]]

```python
forward(pixel_values: FloatTensor, classes_input_ids: LongTensor, classes_attention_mask: LongTensor, tasks_input_ids: LongTensor, tasks_attention_mask: LongTensor, classes_structure: LongTensor, labels: typing.Optional[torch.LongTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/omdet_turbo/modeling_omdet_turbo.py#L1494)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [DetrImageProcessor](/docs/transformers/v5.15.1/en/model_doc/detr#transformers.DetrImageProcessor). See `DetrImageProcessor.__call__()` for details ([OmDetTurboProcessor](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboProcessor) uses [DetrImageProcessor](/docs/transformers/v5.15.1/en/model_doc/detr#transformers.DetrImageProcessor) for processing images).

classes_input_ids (`torch.LongTensor` of shape `(total_classes (>= batch_size), sequence_length)`) : Indices of input classes sequence tokens in the vocabulary of the language model. Several classes can be provided for each tasks, thus the tokenized classes are flattened and the structure of the classes is provided in the `classes_structure` argument.  Indices can be obtained using [OmDetTurboProcessor](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboProcessor). See [OmDetTurboProcessor.__call__()](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboProcessor.__call__) for details.  [What are input IDs?](../glossary#input-ids)

classes_attention_mask (`torch.BoolTensor` of shape `(total_classes (>= batch_size), num_classes, sequence_length)`) : Attention mask for the classes. This is a binary mask that indicates which tokens should be attended to, and which should not.

tasks_input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input tasks sequence tokens in the vocabulary of the language model.  Indices can be obtained using [OmDetTurboProcessor](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboProcessor). See [OmDetTurboProcessor.__call__()](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboProcessor.__call__) for details.  [What are input IDs?](../glossary#input-ids)

tasks_attention_mask (`torch.BoolTensor` of shape `(batch_size, sequence_length)`) : Attention mask for the tasks. This is a binary mask that indicates which tokens should be attended to, and which should not.

classes_structure (`torch.LongTensor` of shape `(batch_size)`) : Structure of the classes. This tensor indicates the number of classes for each task.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `OmDetTurboObjectDetectionOutput` or `tuple(torch.FloatTensor)`

A `OmDetTurboObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([OmDetTurboConfig](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboConfig)) and inputs.

The [OmDetTurboForObjectDetection](/docs/transformers/v5.15.1/en/model_doc/omdet-turbo#transformers.OmDetTurboForObjectDetection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor`, *optional*) -- The loss value.
- **decoder_coord_logits** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- The predicted coordinates logits of the objects.
- **decoder_class_logits** (`torch.FloatTensor` of shape `(batch_size, num_queries, num_classes)`) -- The predicted class of the objects.
- **init_reference_points** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- The initial reference points.
- **intermediate_reference_points** (`tuple[tuple[torch.FloatTensor]]`, *optional*) -- The intermediate reference points.
- **encoder_coord_logits** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- The predicted coordinates of the objects from the encoder.
- **encoder_class_logits** (`tuple[torch.FloatTensor]`, *optional*) -- The predicted class of the objects from the encoder.
- **encoder_extracted_states** (`torch.FloatTensor`, *optional*) -- The extracted states from the Feature Pyramid Network (FPN) and Path Aggregation Network (PAN) of the encoder.
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of shape
  `(batch_size, sequence_length, hidden_size)`. Hidden-states of the model at the output of each layer
  plus the initial embedding outputs.
- **decoder_attentions** (`tuple[tuple[torch.FloatTensor]]`, *optional*) -- Tuple of tuples of `torch.FloatTensor` (one for attention for each layer) of shape `(batch_size, num_heads,
  sequence_length, sequence_length)`. Attentions weights after the attention softmax, used to compute the
  weighted average in the self-attention, cross-attention and multi-scale deformable attention heads.
- **encoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of shape
  `(batch_size, sequence_length, hidden_size)`. Hidden-states of the model at the output of each layer
  plus the initial embedding outputs.
- **encoder_attentions** (`tuple[tuple[torch.FloatTensor]]`, *optional*) -- Tuple of tuples of `torch.FloatTensor` (one for attention for each layer) of shape `(batch_size, num_heads,
  sequence_length, sequence_length)`. Attentions weights after the attention softmax, used to compute the
  weighted average in the self-attention, cross-attention and multi-scale deformable attention heads.
- **classes_structure** (`torch.LongTensor`, *optional*) -- The number of queried classes for each image.

Examples:

```python
>>> import httpx
>>> from io import BytesIO
>>> from PIL import Image

>>> from transformers import AutoProcessor, OmDetTurboForObjectDetection

>>> processor = AutoProcessor.from_pretrained("omlab/omdet-turbo-swin-tiny-hf")
>>> model = OmDetTurboForObjectDetection.from_pretrained("omlab/omdet-turbo-swin-tiny-hf")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> classes = ["cat", "remote"]
>>> task = "Detect {}.".format(", ".join(classes))
>>> inputs = processor(image, text=classes, task=task, return_tensors="pt")

>>> outputs = model(**inputs)

>>> # convert outputs (bounding boxes and class logits)
>>> results = processor.post_process_grounded_object_detection(
...     outputs,
...     classes=classes,
...     target_sizes=[image.size[::-1]],
...     score_threshold=0.3,
...     nms_threshold=0.3,
>>> )[0]
>>> for score, class_name, box in zip(results["scores"], results["classes"], results["boxes"]):
...     box = [round(i, 1) for i in box.tolist()]
...     print(
...         f"Detected {class_name} with confidence "
...         f"{round(score.item(), 2)} at location {box}"
...     )
Detected remote with confidence 0.76 at location [39.9, 71.3, 176.5, 117.9]
Detected cat with confidence 0.72 at location [345.1, 22.5, 639.7, 371.9]
Detected cat with confidence 0.65 at location [12.7, 53.8, 315.5, 475.3]
Detected remote with confidence 0.57 at location [333.4, 75.6, 370.7, 187.0]
```
