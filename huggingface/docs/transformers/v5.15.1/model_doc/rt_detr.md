# RT-DETR

## Overview

The RT-DETR model was proposed in [DETRs Beat YOLOs on Real-time Object Detection](https://huggingface.co/papers/2304.08069) by Wenyu Lv, Yian Zhao, Shangliang Xu, Jinman Wei, Guanzhong Wang, Cheng Cui, Yuning Du, Qingqing Dang, Yi Liu.

RT-DETR is an object detection model that stands for "Real-Time DEtection Transformer." This model is designed to perform object detection tasks with a focus on achieving real-time performance while maintaining high accuracy. Leveraging the transformer architecture, which has gained significant popularity in various fields of deep learning, RT-DETR processes images to identify and locate multiple objects within them.

The abstract from the paper is the following:

*Recently, end-to-end transformer-based detectors (DETRs) have achieved remarkable performance. However, the issue of the high computational cost of DETRs has not been effectively addressed, limiting their practical application and preventing them from fully exploiting the benefits of no post-processing, such as non-maximum suppression (NMS). In this paper, we first analyze the influence of NMS in modern real-time object detectors on inference speed, and establish an end-to-end speed benchmark. To avoid the inference delay caused by NMS, we propose a Real-Time DEtection TRansformer (RT-DETR), the first real-time end-to-end object detector to our best knowledge. Specifically, we design an efficient hybrid encoder to efficiently process multi-scale features by decoupling the intra-scale interaction and cross-scale fusion, and propose IoU-aware query selection to improve the initialization of object queries. In addition, our proposed detector supports flexibly adjustment of the inference speed by using different decoder layers without the need for retraining, which facilitates the practical application of real-time object detectors. Our RT-DETR-L achieves 53.0% AP on COCO val2017 and 114 FPS on T4 GPU, while RT-DETR-X achieves 54.8% AP and 74 FPS, outperforming all YOLO detectors of the same scale in both speed and accuracy. Furthermore, our RT-DETR-R50 achieves 53.1% AP and 108 FPS, outperforming DINO-Deformable-DETR-R50 by 2.2% AP in accuracy and by about 21 times in FPS.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/rt_detr_overview.png"
alt="drawing" width="600"/>

 RT-DETR performance relative to YOLO models. Taken from the original paper. 

The model version was contributed by [rafaelpadilla](https://huggingface.co/rafaelpadilla) and [sangbumchoi](https://github.com/SangbumChoi). The original code can be found [here](https://github.com/lyuwenyu/RT-DETR/).

## Usage tips

Initially, an image is processed using a pre-trained convolutional neural network, specifically a Resnet-D variant as referenced in the original code. This network extracts features from the final three layers of the architecture. Following this, a hybrid encoder is employed to convert the multi-scale features into a sequential array of image features. Then, a decoder, equipped with auxiliary prediction heads is used to refine the object queries. This process facilitates the direct generation of bounding boxes, eliminating the need for any additional post-processing to acquire the logits and coordinates for the bounding boxes. The model is meant to be used on images resized to a size 640x640 with the corresponding ImageProcessor. Reshaping to other sizes will generally degrade performance.

```python
import requests
import torch
from PIL import Image

from transformers import RTDetrForObjectDetection, RTDetrImageProcessor

url = 'http://images.cocodataset.org/val2017/000000039769.jpg'
image = Image.open(requests.get(url, stream=True).raw)

image_processor = RTDetrImageProcessor.from_pretrained("PekingU/rtdetr_r50vd")
model = RTDetrForObjectDetection.from_pretrained("PekingU/rtdetr_r50vd", device_map="auto")

inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

results = image_processor.post_process_object_detection(outputs, target_sizes=torch.tensor([(image.height, image.width)]), threshold=0.3)

for result in results:
    for score, label_id, box in zip(result["scores"], result["labels"], result["boxes"]):
        score, label = score.item(), label_id.item()
        box = [round(i, 2) for i in box.tolist()]
        print(f"{model.config.id2label[label]}: {score:.2f} {box}")
sofa: 0.97 [0.14, 0.38, 640.13, 476.21]
cat: 0.96 [343.38, 24.28, 640.14, 371.5]
cat: 0.96 [13.23, 54.18, 318.98, 472.22]
remote: 0.95 [40.11, 73.44, 175.96, 118.48]
remote: 0.92 [333.73, 76.58, 369.97, 186.99]
```

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with RT-DETR.

- Scripts for finetuning [RTDetrForObjectDetection](/docs/transformers/v5.15.1/en/model_doc/rt_detr#transformers.RTDetrForObjectDetection) with [Trainer](/docs/transformers/v5.15.1/en/main_classes/trainer#transformers.Trainer) or [Accelerate](https://huggingface.co/docs/accelerate/index) can be found [here](https://github.com/huggingface/transformers/tree/main/examples/pytorch/object-detection).
- See also: [Object detection task guide](../tasks/object_detection).
- Notebooks regarding inference and fine-tuning RT-DETR on a custom dataset can be found [here](https://github.com/NielsRogge/Transformers-Tutorials/tree/master/RT-DETR). 🌎

## RTDetrConfig[[transformers.RTDetrConfig]]

#### transformers.RTDetrConfig[[transformers.RTDetrConfig]]

```python
transformers.RTDetrConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, is_encoder_decoder: bool = True, initializer_range: float = 0.01, initializer_bias_prior_prob: float | None = None, layer_norm_eps: float = 1e-05, batch_norm_eps: float = 1e-05, backbone_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, freeze_backbone_batch_norms: bool = True, encoder_hidden_dim: int = 256, encoder_in_channels: list[int] | tuple[int, ...] = (512, 1024, 2048), feat_strides: list[int] | tuple[int, ...] = (8, 16, 32), encoder_layers: int = 1, encoder_ffn_dim: int = 1024, encoder_attention_heads: int = 8, dropout: float | int = 0.0, activation_dropout: float | int = 0.0, encode_proj_layers: list[int] | tuple[int, ...] = (2,), positional_encoding_temperature: int = 10000, encoder_activation_function: str = 'gelu', activation_function: str = 'silu', eval_size: int | None = None, normalize_before: bool = False, hidden_expansion: float = 1.0, d_model: int = 256, num_queries: int = 300, decoder_in_channels: list[int] | tuple[int, ...] = (256, 256, 256), decoder_ffn_dim: int = 1024, num_feature_levels: int = 3, decoder_n_points: int = 4, decoder_layers: int = 6, decoder_attention_heads: int = 8, decoder_activation_function: str = 'relu', attention_dropout: float | int = 0.0, num_denoising: int = 100, label_noise_ratio: float = 0.5, box_noise_scale: float = 1.0, learn_initial_query: bool = False, anchor_image_size: int | list[int] | None = None, disable_custom_kernels: bool = True, with_box_refine: bool = True, matcher_alpha: float = 0.25, matcher_gamma: float = 2.0, matcher_class_cost: float = 2.0, matcher_bbox_cost: float = 5.0, matcher_giou_cost: float = 2.0, use_focal_loss: bool = True, auxiliary_loss: bool = True, focal_loss_alpha: float = 0.75, focal_loss_gamma: float = 2.0, weight_loss_vfl: float = 1.0, weight_loss_bbox: float = 5.0, weight_loss_giou: float = 2.0, eos_coefficient: float = 0.0001)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/configuration_rt_detr.py#L26)

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

disable_custom_kernels (`bool`, *optional*, defaults to `True`) : Whether to disable custom kernels.

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

eos_coefficient (`float`, *optional*, defaults to `0.0001`) : Relative classification weight of the 'no-object' class in the object detection loss.

This is the configuration class to store the configuration of a RTDetrModel. It is used to instantiate a Rt Detr
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PekingU/rtdetr_r50vd](https://huggingface.co/PekingU/rtdetr_r50vd)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import RTDetrConfig, RTDetrModel

>>> # Initializing a RT-DETR configuration
>>> configuration = RTDetrConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = RTDetrModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## RTDetrResNetConfig[[transformers.RTDetrResNetConfig]]

#### transformers.RTDetrResNetConfig[[transformers.RTDetrResNetConfig]]

```python
transformers.RTDetrResNetConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, num_channels: int = 3, embedding_size: int = 64, hidden_sizes: list[int] | tuple[int, ...] = (256, 512, 1024, 2048), depths: list[int] | tuple[int, ...] = (3, 4, 6, 3), layer_type: str = 'bottleneck', hidden_act: str = 'relu', downsample_in_first_stage: bool = False, downsample_in_bottleneck: bool = False, _out_features: list[str] | None = None, _out_indices: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/configuration_rt_detr_resnet.py#L25)

**Parameters:**

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

embedding_size (`int`, *optional*, defaults to `64`) : Dimensionality of the embeddings and hidden states.

hidden_sizes (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(256, 512, 1024, 2048)`) : Dimensionality (hidden size) at each stage of the model.

depths (`list[int]`, *optional*, defaults to `[3, 4, 6, 3]`) : Depth (number of layers) for each stage.

layer_type (`str`, *optional*, defaults to `"bottleneck"`) : The layer to use, it can be either `"basic"` (used for smaller models, like resnet-18 or resnet-34) or `"bottleneck"` (used for larger models like resnet-50 and above).

hidden_act (`str`, *optional*, defaults to `"relu"`) : The non-linear activation function in each block. If string, `"gelu"`, `"relu"`, `"selu"` and `"gelu_new"` are supported.

downsample_in_first_stage (`bool`, *optional*, defaults to `False`) : If `True`, the first stage will downsample the inputs using a `stride` of 2.

downsample_in_bottleneck (`bool`, *optional*, defaults to `False`) : If `True`, the first conv 1x1 in ResNetBottleNeckLayer will downsample the inputs using a `stride` of 2.

This is the configuration class to store the configuration of a Rt Detr ResnetModel. It is used to instantiate a Rt Detr Resnet
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/resnet-50](https://huggingface.co/microsoft/resnet-50)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import RTDetrResNetConfig, RTDetrResnetBackbone

>>> # Initializing a ResNet resnet-50 style configuration
>>> configuration = RTDetrResNetConfig()

>>> # Initializing a model (with random weights) from the resnet-50 style configuration
>>> model = RTDetrResnetBackbone(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## RTDetrImageProcessor[[transformers.RTDetrImageProcessor]]

#### transformers.RTDetrImageProcessor[[transformers.RTDetrImageProcessor]]

```python
transformers.RTDetrImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/image_processing_rt_detr.py#L128)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 640, 'width': 640}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.485, 0.456, 0.406]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.229, 0.224, 0.225]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

format (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) : Data format of the annotations. One of "coco_detection" or "coco_panoptic".

do_convert_annotations (`bool`, *kwargs*, *optional*, defaults to `True`) : Controls whether to convert the annotations to the format expected by the RT_DETR model. Converts the bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`. Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.

Constructs a RTDetrImageProcessor image processor.

#### preprocess[[transformers.RTDetrImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], annotations: dict[str, int | str | list[dict]] | list[dict[str, int | str | list[dict]]] | None = None, return_segmentation_masks: bool | None = None, masks_path: str | pathlib.Path | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/image_processing_rt_detr.py#L357)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

annotations (`AnnotationType` or `list[AnnotationType]`, *optional*) : Annotations to transform according to the padding that is applied to the images.

return_segmentation_masks (`bool`, *optional*, defaults to `self.return_segmentation_masks`) : Whether to return segmentation masks.

masks_path (`str` or `pathlib.Path`, *optional*) : Path to the directory containing the segmentation masks.

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

format (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) : Data format of the annotations. One of "coco_detection" or "coco_panoptic".

do_convert_annotations (`bool`, *kwargs*, *optional*, defaults to `True`) : Controls whether to convert the annotations to the format expected by the RT_DETR model. Converts the bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`. Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_object_detection[[transformers.RTDetrImageProcessor.post_process_object_detection]]

```python
post_process_object_detection(outputs, threshold: float = 0.5, target_sizes: transformers.utils.generic.TensorType | list[tuple] = None, use_focal_loss: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/image_processing_rt_detr.py#L482)

**Parameters:**

outputs (`DetrObjectDetectionOutput`) : Raw outputs of the model.

threshold (`float`, *optional*, defaults to 0.5) : Score threshold to keep object detection predictions.

target_sizes (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) : Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size `(height, width)` of each image in the batch. If unset, predictions will not be resized.

use_focal_loss (`bool` defaults to `True`) : Variable informing if the focal loss was used to predict the outputs. If `True`, a sigmoid is applied to compute the scores of each detection, otherwise, a softmax function is used.

**Returns:** `list[Dict]`

A list of dictionaries, each dictionary containing the scores, labels and boxes for an image
in the batch as predicted by the model.

Converts the raw output of [DetrForObjectDetection](/docs/transformers/v5.15.1/en/model_doc/detr#transformers.DetrForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format. Only supports PyTorch.

## RTDetrImageProcessorPil[[transformers.RTDetrImageProcessorPil]]

#### transformers.RTDetrImageProcessorPil[[transformers.RTDetrImageProcessorPil]]

```python
transformers.RTDetrImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/image_processing_pil_rt_detr.py#L130)

#### preprocess[[transformers.RTDetrImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], annotations: dict[str, int | str | list[dict]] | list[dict[str, int | str | list[dict]]] | None = None, return_segmentation_masks: bool | None = None, masks_path: str | pathlib.Path | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/image_processing_pil_rt_detr.py#L379)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

annotations (`AnnotationType` or `list[AnnotationType]`, *optional*) : Annotations to transform according to the padding that is applied to the images.

return_segmentation_masks (`bool`, *optional*, defaults to `self.return_segmentation_masks`) : Whether to return segmentation masks.

masks_path (`str` or `pathlib.Path`, *optional*) : Path to the directory containing the segmentation masks.

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

format (`str`, *kwargs*, *optional*, defaults to `AnnotationFormat.COCO_DETECTION`) : Data format of the annotations. One of "coco_detection" or "coco_panoptic".

do_convert_annotations (`bool`, *kwargs*, *optional*, defaults to `True`) : Controls whether to convert the annotations to the format expected by the RT_DETR model. Converts the bounding boxes to the format `(center_x, center_y, width, height)` and in the range `[0, 1]`. Can be overridden by the `do_convert_annotations` parameter in the `preprocess` method.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

#### post_process_object_detection[[transformers.RTDetrImageProcessorPil.post_process_object_detection]]

```python
post_process_object_detection(outputs, threshold: float = 0.5, target_sizes: transformers.utils.generic.TensorType | list[tuple] = None, use_focal_loss: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/image_processing_pil_rt_detr.py#L507)

**Parameters:**

outputs (`DetrObjectDetectionOutput`) : Raw outputs of the model.

threshold (`float`, *optional*, defaults to 0.5) : Score threshold to keep object detection predictions.

target_sizes (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) : Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size `(height, width)` of each image in the batch. If unset, predictions will not be resized.

use_focal_loss (`bool` defaults to `True`) : Variable informing if the focal loss was used to predict the outputs. If `True`, a sigmoid is applied to compute the scores of each detection, otherwise, a softmax function is used.

**Returns:** `list[Dict]`

A list of dictionaries, each dictionary containing the scores, labels and boxes for an image
in the batch as predicted by the model.

Converts the raw output of [DetrForObjectDetection](/docs/transformers/v5.15.1/en/model_doc/detr#transformers.DetrForObjectDetection) into final bounding boxes in (top_left_x, top_left_y,
bottom_right_x, bottom_right_y) format. Only supports PyTorch.

## RTDetrModel[[transformers.RTDetrModel]]

#### transformers.RTDetrModel[[transformers.RTDetrModel]]

```python
transformers.RTDetrModel(config: RTDetrConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/modeling_rt_detr.py#L1422)

**Parameters:**

config ([RTDetrConfig](/docs/transformers/v5.15.1/en/model_doc/rt_detr#transformers.RTDetrConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

RT-DETR Model (consisting of a backbone and encoder-decoder) outputting raw hidden states without any head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.RTDetrModel.forward]]

```python
forward(pixel_values: FloatTensor, pixel_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: typing.Optional[torch.FloatTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: list[dict] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/modeling_rt_detr.py#L1542)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

pixel_mask (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

encoder_outputs (`torch.FloatTensor`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you can choose to directly pass a flattened representation of an image.

labels (`list[Dict]` of len `(batch_size,)`, *optional*) : Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.

**Returns:** `RTDetrModelOutput` or `tuple(torch.FloatTensor)`

A `RTDetrModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [RTDetrModel](/docs/transformers/v5.15.1/en/model_doc/rt_detr#transformers.RTDetrModel) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoImageProcessor, RTDetrModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = AutoImageProcessor.from_pretrained("PekingU/rtdetr_r50vd")
>>> model = RTDetrModel.from_pretrained("PekingU/rtdetr_r50vd")

>>> inputs = image_processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)

>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 300, 256]
```

## RTDetrForObjectDetection[[transformers.RTDetrForObjectDetection]]

#### transformers.RTDetrForObjectDetection[[transformers.RTDetrForObjectDetection]]

```python
transformers.RTDetrForObjectDetection(config: RTDetrConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/modeling_rt_detr.py#L1748)

**Parameters:**

config ([RTDetrConfig](/docs/transformers/v5.15.1/en/model_doc/rt_detr#transformers.RTDetrConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

RT-DETR Model (consisting of a backbone and encoder-decoder) outputting bounding boxes and logits to be further
decoded into scores and classes.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.RTDetrForObjectDetection.forward]]

```python
forward(pixel_values: FloatTensor, pixel_mask: typing.Optional[torch.LongTensor] = None, encoder_outputs: typing.Optional[torch.FloatTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: list[dict] | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/modeling_rt_detr.py#L1769)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

pixel_mask (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) : Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:  - 1 for pixels that are real (i.e. **not masked**), - 0 for pixels that are padding (i.e. **masked**).  [What are attention masks?](../glossary#attention-mask)

encoder_outputs (`torch.FloatTensor`, *optional*) : Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`) `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing the flattened feature map (output of the backbone + projection layer), you can choose to directly pass a flattened representation of an image.

labels (`list[Dict]` of len `(batch_size,)`, *optional*) : Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.

**Returns:** `RTDetrObjectDetectionOutput` or `tuple(torch.FloatTensor)`

A `RTDetrObjectDetectionOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`None`) and inputs.

The [RTDetrForObjectDetection](/docs/transformers/v5.15.1/en/model_doc/rt_detr#transformers.RTDetrForObjectDetection) forward method, overrides the `__call__` special method.

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
  possible padding). You can use [post_process_object_detection()](/docs/transformers/v5.15.1/en/model_doc/rt_detr#transformers.RTDetrImageProcessor.post_process_object_detection) to retrieve the
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

Examples:

```python
>>> from transformers import RTDetrImageProcessor, RTDetrForObjectDetection
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> import torch

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> image_processor = RTDetrImageProcessor.from_pretrained("PekingU/rtdetr_r50vd")
>>> model = RTDetrForObjectDetection.from_pretrained("PekingU/rtdetr_r50vd")

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
>>> results = image_processor.post_process_object_detection(outputs, threshold=0.9, target_sizes=target_sizes)[
...     0
... ]

>>> for score, label, box in zip(results["scores"], results["labels"], results["boxes"]):
...     box = [round(i, 2) for i in box.tolist()]
...     print(
...         f"Detected {model.config.id2label[label.item()]} with confidence "
...         f"{round(score.item(), 3)} at location {box}"
...     )
Detected sofa with confidence 0.97 at location [0.14, 0.38, 640.13, 476.21]
Detected cat with confidence 0.96 at location [343.38, 24.28, 640.14, 371.5]
Detected cat with confidence 0.958 at location [13.23, 54.18, 318.98, 472.22]
Detected remote with confidence 0.951 at location [40.11, 73.44, 175.96, 118.48]
Detected remote with confidence 0.924 at location [333.73, 76.58, 369.97, 186.99]
```

## RTDetrResNetBackbone[[transformers.RTDetrResNetBackbone]]

#### transformers.RTDetrResNetBackbone[[transformers.RTDetrResNetBackbone]]

```python
transformers.RTDetrResNetBackbone(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/modeling_rt_detr_resnet.py#L349)

**Parameters:**

config ([RTDetrResNetBackbone](/docs/transformers/v5.15.1/en/model_doc/rt_detr#transformers.RTDetrResNetBackbone)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

ResNet backbone, to be used with frameworks like RTDETR.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.RTDetrResNetBackbone.forward]]

```python
forward(pixel_values: Tensor, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/rt_detr/modeling_rt_detr_resnet.py#L362)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using `image_processor_class`. See `image_processor_class.__call__` for details (`processor_class` uses `image_processor_class` for processing images).

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** `BackboneOutput` or `tuple(torch.FloatTensor)`

A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([RTDetrResNetConfig](/docs/transformers/v5.15.1/en/model_doc/rt_detr#transformers.RTDetrResNetConfig)) and inputs.

The [RTDetrResNetBackbone](/docs/transformers/v5.15.1/en/model_doc/rt_detr#transformers.RTDetrResNetBackbone) forward method, overrides the `__call__` special method.

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
>>> from transformers import RTDetrResNetConfig, RTDetrResNetBackbone
>>> import torch
precation import deprecate_kwarg
precation import deprecate_kwarg
precation import deprecate_kwarg
utils.deprecation import deprecate_kwarg
utils.deprecation import deprecate_kwarg

>>> config = RTDetrResNetConfig()
>>> model = RTDetrResNetBackbone(config)

>>> pixel_values = torch.randn(1, 3, 224, 224)

>>> with torch.no_grad():
...     outputs = model(pixel_values)

>>> feature_maps = outputs.feature_maps
>>> list(feature_maps[-1].shape)
[1, 2048, 7, 7]
```
