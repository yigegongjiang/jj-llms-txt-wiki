# ViTPose

[ViTPose](https://huggingface.co/papers/2204.12484) is a vision transformer-based model for keypoint (pose) estimation. It uses a simple, non-hierarchical [ViT](./vit) backbone and a lightweight decoder head. This architecture simplifies model design, takes advantage of transformer scalability, and can be adapted to different training strategies.

[ViTPose++](https://huggingface.co/papers/2212.04246) improves on ViTPose by incorporating a mixture-of-experts (MoE) module in the backbone and using more diverse pretraining data.

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/vitpose-architecture.png"
alt="drawing" width="600"/>

You can find all ViTPose and ViTPose++ checkpoints under the [ViTPose collection](https://huggingface.co/collections/usyd-community/vitpose-677fcfd0a0b2b5c8f79c4335).

The example below demonstrates pose estimation with the [VitPoseForPoseEstimation](/docs/transformers/v5.14.0/en/model_doc/vitpose#transformers.VitPoseForPoseEstimation) class.

```python
import requests
import supervision as sv
import torch
from PIL import Image

from transformers import AutoProcessor, RTDetrForObjectDetection, VitPoseForPoseEstimation

url = "https://www.fcbarcelona.com/fcbarcelona/photo/2021/01/31/3c55a19f-dfc1-4451-885e-afd14e890a11/mini_2021-01-31-BARCELONA-ATHLETIC-BILBAOI-30.JPG"
image = Image.open(requests.get(url, stream=True).raw)

# Detect humans in the image
person_image_processor = AutoProcessor.from_pretrained("PekingU/rtdetr_r50vd_coco_o365")
person_model = RTDetrForObjectDetection.from_pretrained("PekingU/rtdetr_r50vd_coco_o365", device_map="auto")

inputs = person_image_processor(images=image, return_tensors="pt").to(person_model.device)

with torch.no_grad():
    outputs = person_model(**inputs)

results = person_image_processor.post_process_object_detection(
    outputs, target_sizes=torch.tensor([(image.height, image.width)]), threshold=0.3
)
result = results[0]

# Human label refers 0 index in COCO dataset
person_boxes = result["boxes"][result["labels"] == 0]
person_boxes = person_boxes.cpu().numpy()

# Convert boxes from VOC (x1, y1, x2, y2) to COCO (x1, y1, w, h) format
person_boxes[:, 2] = person_boxes[:, 2] - person_boxes[:, 0]
person_boxes[:, 3] = person_boxes[:, 3] - person_boxes[:, 1]

# Detect keypoints for each person found
image_processor = AutoProcessor.from_pretrained("usyd-community/vitpose-base-simple")
model = VitPoseForPoseEstimation.from_pretrained("usyd-community/vitpose-base-simple", device_map="auto")

inputs = image_processor(image, boxes=[person_boxes], return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

pose_results = image_processor.post_process_pose_estimation(outputs, boxes=[person_boxes])
image_pose_result = pose_results[0]

xy = torch.stack([pose_result['keypoints'] for pose_result in image_pose_result]).cpu().numpy()
scores = torch.stack([pose_result['scores'] for pose_result in image_pose_result]).cpu().numpy()

key_points = sv.KeyPoints(
    xy=xy, confidence=scores
)

edge_annotator = sv.EdgeAnnotator(
    color=sv.Color.GREEN,
    thickness=1
)
vertex_annotator = sv.VertexAnnotator(
    color=sv.Color.RED,
    radius=2
)
annotated_frame = edge_annotator.annotate(
    scene=image.copy(),
    key_points=key_points
)
annotated_frame = vertex_annotator.annotate(
    scene=annotated_frame,
    key_points=key_points
)
annotated_frame
```

    

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [torchao](../quantization/torchao) to only quantize the weights to int4.

```python
# pip install torchao
import requests
import torch
from PIL import Image

from transformers import AutoProcessor, RTDetrForObjectDetection, TorchAoConfig, VitPoseForPoseEstimation

url = "https://www.fcbarcelona.com/fcbarcelona/photo/2021/01/31/3c55a19f-dfc1-4451-885e-afd14e890a11/mini_2021-01-31-BARCELONA-ATHLETIC-BILBAOI-30.JPG"
image = Image.open(requests.get(url, stream=True).raw)

person_image_processor = AutoProcessor.from_pretrained("PekingU/rtdetr_r50vd_coco_o365")
person_model = RTDetrForObjectDetection.from_pretrained("PekingU/rtdetr_r50vd_coco_o365", device_map="auto")

inputs = person_image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = person_model(**inputs)

results = person_image_processor.post_process_object_detection(
    outputs, target_sizes=torch.tensor([(image.height, image.width)]), threshold=0.3
)
result = results[0]

person_boxes = result["boxes"][result["labels"] == 0]
person_boxes = person_boxes.cpu().numpy()

person_boxes[:, 2] = person_boxes[:, 2] - person_boxes[:, 0]
person_boxes[:, 3] = person_boxes[:, 3] - person_boxes[:, 1]

quantization_config = TorchAoConfig("int4_weight_only", group_size=128)

image_processor = AutoProcessor.from_pretrained("usyd-community/vitpose-plus-huge")
model = VitPoseForPoseEstimation.from_pretrained("usyd-community/vitpose-plus-huge", device_map="auto", quantization_config=quantization_config)

inputs = image_processor(image, boxes=[person_boxes], return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

pose_results = image_processor.post_process_pose_estimation(outputs, boxes=[person_boxes])
image_pose_result = pose_results[0]
```

## Notes

- Use [AutoProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoProcessor) to automatically prepare bounding box and image inputs.
- ViTPose is a top-down pose estimator. It uses a object detector to detect individuals first before keypoint prediction.
- ViTPose++ has 6 different MoE expert heads (COCO validation `0`, AiC `1`, MPII `2`, AP-10K `3`, APT-36K `4`, COCO-WholeBody `5`) which supports 6 different datasets. Pass a specific value corresponding to the dataset to the `dataset_index` to indicate which expert to use.

    ```py
    from transformers import AutoProcessor, VitPoseForPoseEstimation

    image_processor = AutoProcessor.from_pretrained("usyd-community/vitpose-plus-base")
    model = VitPoseForPoseEstimation.from_pretrained("usyd-community/vitpose-plus-base", device_map="auto")

    inputs = image_processor(image, boxes=[person_boxes], return_tensors="pt").to(model.device)
    dataset_index = torch.tensor([0], device=device) # must be a tensor of shape (batch_size,)

    with torch.no_grad():
        outputs = model(**inputs, dataset_index=dataset_index)
    ```

- [OpenCV](https://opencv.org/) is an alternative option for visualizing the estimated pose.

    ```py
    # pip install opencv-python
    import math
    import cv2

    def draw_points(image, keypoints, scores, pose_keypoint_color, keypoint_score_threshold, radius, show_keypoint_weight):
        if pose_keypoint_color is not None:
            assert len(pose_keypoint_color) == len(keypoints)
        for kid, (kpt, kpt_score) in enumerate(zip(keypoints, scores)):
            x_coord, y_coord = int(kpt[0]), int(kpt[1])
            if kpt_score > keypoint_score_threshold:
                color = tuple(int(c) for c in pose_keypoint_color[kid])
                if show_keypoint_weight:
                    cv2.circle(image, (int(x_coord), int(y_coord)), radius, color, -1)
                    transparency = max(0, min(1, kpt_score))
                    cv2.addWeighted(image, transparency, image, 1 - transparency, 0, dst=image)
                else:
                    cv2.circle(image, (int(x_coord), int(y_coord)), radius, color, -1)

    def draw_links(image, keypoints, scores, keypoint_edges, link_colors, keypoint_score_threshold, thickness, show_keypoint_weight, stick_width = 2):
        height, width, _ = image.shape
        if keypoint_edges is not None and link_colors is not None:
            assert len(link_colors) == len(keypoint_edges)
            for sk_id, sk in enumerate(keypoint_edges):
                x1, y1, score1 = (int(keypoints[sk[0], 0]), int(keypoints[sk[0], 1]), scores[sk[0]])
                x2, y2, score2 = (int(keypoints[sk[1], 0]), int(keypoints[sk[1], 1]), scores[sk[1]])
                if (
                    x1 > 0
                    and x1 < width
                    and y1 > 0
                    and y1 < height
                    and x2 > 0
                    and x2 < width
                    and y2 > 0
                    and y2 < height
                    and score1 > keypoint_score_threshold
                    and score2 > keypoint_score_threshold
                ):
                    color = tuple(int(c) for c in link_colors[sk_id])
                    if show_keypoint_weight:
                        X = (x1, x2)
                        Y = (y1, y2)
                        mean_x = np.mean(X)
                        mean_y = np.mean(Y)
                        length = ((Y[0] - Y[1]) ** 2 + (X[0] - X[1]) ** 2) ** 0.5
                        angle = math.degrees(math.atan2(Y[0] - Y[1], X[0] - X[1]))
                        polygon = cv2.ellipse2Poly(
                            (int(mean_x), int(mean_y)), (int(length / 2), int(stick_width)), int(angle), 0, 360, 1
                        )
                        cv2.fillConvexPoly(image, polygon, color)
                        transparency = max(0, min(1, 0.5 * (keypoints[sk[0], 2] + keypoints[sk[1], 2])))
                        cv2.addWeighted(image, transparency, image, 1 - transparency, 0, dst=image)
                    else:
                        cv2.line(image, (x1, y1), (x2, y2), color, thickness=thickness)

    # Note: keypoint_edges and color palette are dataset-specific
    keypoint_edges = model.config.edges

    palette = np.array(
        [
            [255, 128, 0],
            [255, 153, 51],
            [255, 178, 102],
            [230, 230, 0],
            [255, 153, 255],
            [153, 204, 255],
            [255, 102, 255],
            [255, 51, 255],
            [102, 178, 255],
            [51, 153, 255],
            [255, 153, 153],
            [255, 102, 102],
            [255, 51, 51],
            [153, 255, 153],
            [102, 255, 102],
            [51, 255, 51],
            [0, 255, 0],
            [0, 0, 255],
            [255, 0, 0],
            [255, 255, 255],
        ]
    )

    link_colors = palette[[0, 0, 0, 0, 7, 7, 7, 9, 9, 9, 9, 9, 16, 16, 16, 16, 16, 16, 16]]
    keypoint_colors = palette[[16, 16, 16, 16, 16, 9, 9, 9, 9, 9, 9, 0, 0, 0, 0, 0, 0]]

    numpy_image = np.array(image)

    for pose_result in image_pose_result:
        scores = np.array(pose_result["scores"])
        keypoints = np.array(pose_result["keypoints"])

        # draw each point on image
        draw_points(numpy_image, keypoints, scores, keypoint_colors, keypoint_score_threshold=0.3, radius=4, show_keypoint_weight=False)

        # draw links
        draw_links(numpy_image, keypoints, scores, keypoint_edges, link_colors, keypoint_score_threshold=0.3, thickness=1, show_keypoint_weight=False)

    pose_image = Image.fromarray(numpy_image)
    pose_image
    ```

## Resources

Refer to resources below to learn more about using ViTPose.

- This [notebook](https://github.com/NielsRogge/Transformers-Tutorials/blob/master/ViTPose/Inference_with_ViTPose_for_body_pose_estimation.ipynb) demonstrates inference and visualization.
- This [Space](https://huggingface.co/spaces/hysts/ViTPose-transformers) demonstrates ViTPose on images and video.

## VitPoseImageProcessor[[transformers.VitPoseImageProcessor]]

- **do_affine_transform** (`bool`, *kwargs*, *optional*) --
  Whether to apply an affine transformation to the input images based on the bounding boxes.
- **normalize_factor** (`float`, *kwargs*, *optional*, defaults to `200.0`) --
  Width and height scale factor used for normalization when computing center and scale from bounding boxes.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a VitPoseImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **boxes** (`list[list[list[float]]]` or `np.ndarray`) --
  List or array of bounding boxes for each image. Each box should be a list of 4 floats representing the
  bounding box coordinates in COCO format (top_left_x, top_left_y, width, height).
- **do_affine_transform** (`bool`, *kwargs*, *optional*) --
  Whether to apply an affine transformation to the input images based on the bounding boxes.
- **normalize_factor** (`float`, *kwargs*, *optional*, defaults to `200.0`) --
  Width and height scale factor used for normalization when computing center and scale from bounding boxes.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`VitPoseEstimatorOutput`) --
  VitPoseForPoseEstimation model outputs.
- **boxes** (`list[list[list[float]]]` or `np.ndarray`) --
  List or array of bounding boxes for each image. Each box should be a list of 4 floats representing the bounding
  box coordinates in COCO format (top_left_x, top_left_y, width, height).
- **kernel_size** (`int`, *optional*, defaults to 11) --
  Gaussian kernel size (K) for modulation.
- **threshold** (`float`, *optional*, defaults to None) --
  Score threshold to keep object detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  `(height, width)` of each image in the batch. If unset, predictions will be resize with the default value.`list[list[Dict]]`A list of dictionaries, each dictionary containing the keypoints and boxes for an image
in the batch as predicted by the model.

Transform the heatmaps into keypoint predictions and transform them back to the image.

## VitPoseImageProcessorPil[[transformers.VitPoseImageProcessorPil]]

- **do_affine_transform** (`bool`, *kwargs*, *optional*) --
  Whether to apply an affine transformation to the input images based on the bounding boxes.
- **normalize_factor** (`float`, *kwargs*, *optional*, defaults to `200.0`) --
  Width and height scale factor used for normalization when computing center and scale from bounding boxes.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a VitPoseImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **boxes** (`list[list[list[float]]]` or `np.ndarray`) --
  List or array of bounding boxes for each image. Each box should be a list of 4 floats representing the
  bounding box coordinates in COCO format (top_left_x, top_left_y, width, height).
- **do_affine_transform** (`bool`, *kwargs*, *optional*) --
  Whether to apply an affine transformation to the input images based on the bounding boxes.
- **normalize_factor** (`float`, *kwargs*, *optional*, defaults to `200.0`) --
  Width and height scale factor used for normalization when computing center and scale from bounding boxes.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`VitPoseEstimatorOutput`) --
  VitPoseForPoseEstimation model outputs.
- **boxes** (`list[list[list[float]]]` or `np.ndarray`) --
  List or array of bounding boxes for each image. Each box should be a list of 4 floats representing the bounding
  box coordinates in COCO format (top_left_x, top_left_y, width, height).
- **kernel_size** (`int`, *optional*, defaults to 11) --
  Gaussian kernel size (K) for modulation.
- **threshold** (`float`, *optional*, defaults to None) --
  Score threshold to keep object detection predictions.
- **target_sizes** (`torch.Tensor` or `list[tuple[int, int]]`, *optional*) --
  Tensor of shape `(batch_size, 2)` or list of tuples (`tuple[int, int]`) containing the target size
  `(height, width)` of each image in the batch. If unset, predictions will be resize with the default value.`list[list[Dict]]`A list of dictionaries, each dictionary containing the keypoints and boxes for an image
in the batch as predicted by the model.

Transform the heatmaps into keypoint predictions and transform them back to the image.

## VitPoseConfig[[transformers.VitPoseConfig]]

- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **scale_factor** (`int`, *optional*, defaults to 4) --
  Factor to upscale the feature maps coming from the ViT backbone.
- **use_simple_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether to use a `VitPoseSimpleDecoder` to decode the feature maps from the backbone into heatmaps. Otherwise it uses `VitPoseClassicDecoder`.

This is the configuration class to store the configuration of a VitposeModel. It is used to instantiate a Vitpose
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [usyd-community/vitpose-base-simple](https://huggingface.co/usyd-community/vitpose-base-simple)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import VitPoseConfig, VitPoseForPoseEstimation

>>> # Initializing a VitPose configuration
>>> configuration = VitPoseConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = VitPoseForPoseEstimation(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## VitPoseForPoseEstimation[[transformers.VitPoseForPoseEstimation]]

- **config** ([VitPoseConfig](/docs/transformers/v5.14.0/en/model_doc/vitpose#transformers.VitPoseConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The VitPose model with a pose estimation head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "dataset_index", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "flip_pairs", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "labels", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [VitPoseImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vitpose#transformers.VitPoseImageProcessor). See `VitPoseImageProcessor.__call__()` for details (`processor_class` uses
  [VitPoseImageProcessor](/docs/transformers/v5.14.0/en/model_doc/vitpose#transformers.VitPoseImageProcessor) for processing images).
- **dataset_index** (`torch.Tensor` of shape `(batch_size,)`) --
  Index to use in the Mixture-of-Experts (MoE) blocks of the backbone.

  This corresponds to the dataset index used during training, e.g. For the single dataset index 0 refers to the corresponding dataset. For the multiple datasets index 0 refers to dataset A (e.g. MPII) and index 1 refers to dataset B (e.g. CrowdPose).
- **flip_pairs** (`torch.tensor`, *optional*) --
  Whether to mirror pairs of keypoints (for example, left ear -- right ear).
- **labels** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.`VitPoseEstimatorOutput` or `tuple(torch.FloatTensor)`A `VitPoseEstimatorOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VitPoseConfig](/docs/transformers/v5.14.0/en/model_doc/vitpose#transformers.VitPoseConfig)) and inputs.
The [VitPoseForPoseEstimation](/docs/transformers/v5.14.0/en/model_doc/vitpose#transformers.VitPoseForPoseEstimation) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Loss is not supported at this moment. See https://github.com/ViTAE-Transformer/ViTPose/tree/main/mmpose/models/losses for further detail.
- **heatmaps** (`torch.FloatTensor` of shape `(batch_size, num_keypoints, height, width)`) -- Heatmaps as predicted by the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, VitPoseForPoseEstimation
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> processor = AutoImageProcessor.from_pretrained("usyd-community/vitpose-base-simple")
>>> model = VitPoseForPoseEstimation.from_pretrained("usyd-community/vitpose-base-simple")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> boxes = [[[412.8, 157.61, 53.05, 138.01], [384.43, 172.21, 15.12, 35.74]]]
>>> inputs = processor(image, boxes=boxes, return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)
>>> heatmaps = outputs.heatmaps
```
