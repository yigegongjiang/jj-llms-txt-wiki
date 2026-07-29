# SuperGlue

[SuperGlue](https://huggingface.co/papers/1911.11763) is a neural network that matches two sets of local features by jointly finding correspondences and rejecting non-matchable points. Assignments are estimated by solving a differentiable optimal transport problem, whose costs are predicted by a graph neural network. SuperGlue introduces a flexible context aggregation mechanism based on attention, enabling it to reason about the underlying 3D scene and feature assignments jointly. Paired with the [SuperPoint model](https://huggingface.co/magic-leap-community/superpoint), it can be used to match two images and estimate the pose between them. This model is useful for tasks such as image matching, homography estimation, etc.

You can find all the original SuperGlue checkpoints under the [Magic Leap Community](https://huggingface.co/magic-leap-community) organization.

> [!TIP]
> This model was contributed by [stevenbucaille](https://huggingface.co/stevenbucaille).
>
> Click on the SuperGlue models in the right sidebar for more examples of how to apply SuperGlue to different computer vision tasks.

The example below demonstrates how to match keypoints between two images with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

keypoint_matcher = pipeline(task="keypoint-matching", model="magic-leap-community/superglue_outdoor")

url_0 = "https://raw.githubusercontent.com/magicleap/SuperGluePretrainedNetwork/refs/heads/master/assets/phototourism_sample_images/united_states_capitol_98169888_3347710852.jpg"
url_1 = "https://raw.githubusercontent.com/magicleap/SuperGluePretrainedNetwork/refs/heads/master/assets/phototourism_sample_images/united_states_capitol_26757027_6717084061.jpg"

results = keypoint_matcher([url_0, url_1], threshold=0.9)
print(results[0])
# {'keypoint_image_0': {'x': ..., 'y': ...}, 'keypoint_image_1': {'x': ..., 'y': ...}, 'score': ...}
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoImageProcessor, AutoModel

url_image1 = "https://raw.githubusercontent.com/magicleap/SuperGluePretrainedNetwork/refs/heads/master/assets/phototourism_sample_images/united_states_capitol_98169888_3347710852.jpg"
image1 = Image.open(requests.get(url_image1, stream=True).raw)
url_image2 = "https://raw.githubusercontent.com/magicleap/SuperGluePretrainedNetwork/refs/heads/master/assets/phototourism_sample_images/united_states_capitol_26757027_6717084061.jpg"
image2 = Image.open(requests.get(url_image2, stream=True).raw)

images = [image1, image2]

processor = AutoImageProcessor.from_pretrained("magic-leap-community/superglue_outdoor")
model = AutoModel.from_pretrained("magic-leap-community/superglue_outdoor", device_map="auto")

inputs = processor(images, return_tensors="pt").to(model.device)
with torch.inference_mode():
    outputs = model(**inputs)

# Post-process to get keypoints and matches
image_sizes = [[(image.height, image.width) for image in images]]
processed_outputs = processor.post_process_keypoint_matching(outputs, image_sizes, threshold=0.2)
```

## Notes

- SuperGlue performs feature matching between two images simultaneously, requiring pairs of images as input.

    ```python
    from transformers import AutoImageProcessor, AutoModel
    import torch
    from PIL import Image
    import requests

    processor = AutoImageProcessor.from_pretrained("magic-leap-community/superglue_outdoor")
    model = AutoModel.from_pretrained("magic-leap-community/superglue_outdoor", device_map="auto")

    # SuperGlue requires pairs of images
    images = [image1, image2]
    inputs = processor(images, return_tensors="pt").to(model.device)
    with torch.inference_mode():
        outputs = model(**inputs)

    # Extract matching information
    keypoints0 = outputs.keypoints0  # Keypoints in first image
    keypoints1 = outputs.keypoints1  # Keypoints in second image
    matches = outputs.matches        # Matching indices
    matching_scores = outputs.matching_scores  # Confidence scores
    ```

- The model outputs matching indices, keypoints, and confidence scores for each match.
- For better visualization and analysis, use the [SuperGlueImageProcessor.post_process_keypoint_matching()](/docs/transformers/v5.14.0/en/model_doc/superglue#transformers.SuperGlueImageProcessor.post_process_keypoint_matching) method to get matches in a more readable format.

    ```py
    # Process outputs for visualization
    image_sizes = [[(image.height, image.width) for image in images]]
    processed_outputs = processor.post_process_keypoint_matching(outputs, image_sizes, threshold=0.2)

    for i, output in enumerate(processed_outputs):
        print(f"For the image pair {i}")
        for keypoint0, keypoint1, matching_score in zip(
                output["keypoints0"], output["keypoints1"], output["matching_scores"]
        ):
            print(f"Keypoint at {keypoint0.numpy()} matches with keypoint at {keypoint1.numpy()} with score {matching_score}")
    ```

- Visualize the matches between the images using the built-in plotting functionality.

    ```py
    # Easy visualization using the built-in plotting method
    processor.visualize_keypoint_matching(images, processed_outputs)
    ```

    

## Resources

- Refer to the [original SuperGlue repository](https://github.com/magicleap/SuperGluePretrainedNetwork) for more examples and implementation details.

## SuperGlueConfig[[transformers.SuperGlueConfig]]

- **keypoint_detector_config** (`Union[AutoConfig, dict]`,  *optional*, defaults to `SuperPointConfig`) --
  The config object or dictionary of the keypoint detector.
- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **keypoint_encoder_sizes** (`list[int]`, *optional*, defaults to `[32, 64, 128, 256]`) --
  The sizes of the keypoint encoder layers.
- **gnn_layers_types** (`list[str]`, *optional*, defaults to `['self', 'cross', 'self', 'cross', 'self', 'cross', 'self', 'cross', 'self', 'cross', 'self', 'cross', 'self', 'cross', 'self', 'cross', 'self', 'cross']`) --
  The types of the GNN layers. Must be either 'self' or 'cross'.
- **num_attention_heads** (`int`, *optional*, defaults to `4`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **sinkhorn_iterations** (`int`, *optional*, defaults to 100) --
  The number of Sinkhorn iterations.
- **matching_threshold** (`float`, *optional*, defaults to 0.0) --
  The matching threshold.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **is_decoder** (`bool`, *optional*, defaults to `False`) --
  Whether the model is used as a decoder or not. If `False`, the model is used as an encoder.
- **attention_probs_dropout_prob** (`Union[int, float]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.

This is the configuration class to store the configuration of a SuperglueModel. It is used to instantiate a Superglue
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [magic-leap-community/superglue_indoor](https://huggingface.co/magic-leap-community/superglue_indoor)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:
```python
>>> from transformers import SuperGlueConfig, SuperGlueModel

>>> # Initializing a SuperGlue superglue style configuration
>>> configuration = SuperGlueConfig()

>>> # Initializing a model from the superglue style configuration
>>> model = SuperGlueModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SuperGlueImageProcessor[[transformers.SuperGlueImageProcessor]]

- **do_grayscale** (`bool`, *kwargs*, *optional*, defaults to `self.do_grayscale`) --
  Whether to convert the image to grayscale. Can be overridden by `do_grayscale` in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a SuperGlueImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **do_grayscale** (`bool`, *kwargs*, *optional*, defaults to `self.do_grayscale`) --
  Whether to convert the image to grayscale. Can be overridden by `do_grayscale` in the `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`SuperGlueKeypointMatchingOutput`) --
  Raw outputs of the model.
- **target_sizes** (`torch.Tensor` or `list[tuple[tuple[int, int]]]`, *optional*) --
  Tensor of shape `(batch_size, 2, 2)` or list of tuples of tuples (`tuple[int, int]`) containing the
  target size `(height, width)` of each image in the batch. This must be the original image size (before
  any processing).
- **threshold** (`float`, *optional*, defaults to `0.0`) --
  Threshold to filter out the matches with low scores.`list[Dict]`A list of dictionaries, each dictionary containing the keypoints in the first and second image
of the pair, the matching scores and the matching indices.

Converts the raw output of `SuperGlueKeypointMatchingOutput` into lists of keypoints, scores and descriptors
with coordinates absolute to the original image sizes.

- **images** --
  Image pairs to plot. Same as `EfficientLoFTRImageProcessor.preprocess`. Expects either a list of 2
  images or a list of list of 2 images list with pixel values ranging from 0 to 255.
- **keypoint_matching_output** (List[Dict[str, torch.Tensor]]]) --
  A post processed keypoint matching output`List[PIL.Image.Image]`A list of PIL images, each containing the image pairs side by side with the detected
keypoints as well as the matching between them.

Plots the image pairs side by side with the detected keypoints as well as the matching between them.

## SuperGlueImageProcessorPil[[transformers.SuperGlueImageProcessorPil]]

- **do_grayscale** (`bool`, *kwargs*, *optional*, defaults to `self.do_grayscale`) --
  Whether to convert the image to grayscale. Can be overridden by `do_grayscale` in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a SuperGlueImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **do_grayscale** (`bool`, *kwargs*, *optional*, defaults to `self.do_grayscale`) --
  Whether to convert the image to grayscale. Can be overridden by `do_grayscale` in the `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

- **outputs** (`SuperGlueKeypointMatchingOutput`) --
  Raw outputs of the model.
- **target_sizes** (`torch.Tensor` or `list[tuple[tuple[int, int]]]`, *optional*) --
  Tensor of shape `(batch_size, 2, 2)` or list of tuples of tuples (`tuple[int, int]`) containing the
  target size `(height, width)` of each image in the batch. This must be the original image size (before
  any processing).
- **threshold** (`float`, *optional*, defaults to `0.0`) --
  Threshold to filter out the matches with low scores.`list[Dict]`A list of dictionaries, each dictionary containing the keypoints in the first and second image
of the pair, the matching scores and the matching indices.

Converts the raw output of `SuperGlueKeypointMatchingOutput` into lists of keypoints, scores and descriptors
with coordinates absolute to the original image sizes.

- **images** (`ImageInput`) --
  Image pairs to plot. Same as `SuperGlueImageProcessor.preprocess`. Expects either a list of 2
  images or a list of list of 2 images list with pixel values ranging from 0 to 255.
- **keypoint_matching_output** (List[Dict[str, torch.Tensor]]]) --
  A post processed keypoint matching output`List[PIL.Image.Image]`A list of PIL images, each containing the image pairs side by side with the detected
keypoints as well as the matching between them.

Plots the image pairs side by side with the detected keypoints as well as the matching between them.

## SuperGlueForKeypointMatching[[transformers.SuperGlueForKeypointMatching]]

- **config** ([SuperGlueConfig](/docs/transformers/v5.14.0/en/model_doc/superglue#transformers.SuperGlueConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SuperGlue model taking images as inputs and outputting the matching of them.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [SuperGlueImageProcessor](/docs/transformers/v5.14.0/en/model_doc/superglue#transformers.SuperGlueImageProcessor). See `SuperGlueImageProcessor.__call__()` for details (`processor_class` uses
  [SuperGlueImageProcessor](/docs/transformers/v5.14.0/en/model_doc/superglue#transformers.SuperGlueImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`SuperGlueKeypointMatchingOutput` or `tuple(torch.FloatTensor)`A `SuperGlueKeypointMatchingOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SuperGlueConfig](/docs/transformers/v5.14.0/en/model_doc/superglue#transformers.SuperGlueConfig)) and inputs.
The [SuperGlueForKeypointMatching](/docs/transformers/v5.14.0/en/model_doc/superglue#transformers.SuperGlueForKeypointMatching) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*) -- Loss computed during training.
- **matches** (`torch.FloatTensor` of shape `(batch_size, 2, num_matches)`) -- Index of keypoint matched in the other image.
- **matching_scores** (`torch.FloatTensor` of shape `(batch_size, 2, num_matches)`) -- Scores of predicted matches.
- **keypoints** (`torch.FloatTensor` of shape `(batch_size, num_keypoints, 2)`) -- Absolute (x, y) coordinates of predicted keypoints in a given image.
- **mask** (`torch.IntTensor` of shape `(batch_size, num_keypoints)`) -- Mask indicating which values in matches and matching_scores are keypoint matching information.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of each stage) of shape `(batch_size, 2, num_channels,
  num_keypoints)`, returned when `output_hidden_states=True` is passed or when
  `config.output_hidden_states=True`)
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, 2, num_heads, num_keypoints,
  num_keypoints)`, returned when `output_attentions=True` is passed or when `config.output_attentions=True`)

Examples:

```python
>>> from transformers import AutoImageProcessor, AutoModel
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "https://github.com/magicleap/SuperGluePretrainedNetwork/blob/master/assets/phototourism_sample_images/london_bridge_78916675_4568141288.jpg?raw=true"
>>> with httpx.stream("GET", url) as response:
...     image_1 = Image.open(BytesIO(response.read()))

>>> url = "https://github.com/magicleap/SuperGluePretrainedNetwork/blob/master/assets/phototourism_sample_images/london_bridge_19481797_2295892421.jpg?raw=true"
>>> with httpx.stream("GET", url) as response:
...     image_2 = Image.open(BytesIO(response.read()))

>>> images = [image_1, image_2]

>>> processor = AutoImageProcessor.from_pretrained("magic-leap-community/superglue_outdoor")
>>> model = AutoModel.from_pretrained("magic-leap-community/superglue_outdoor")

>>> with torch.no_grad():
>>>     inputs = processor(images, return_tensors="pt")
>>>     outputs = model(**inputs)
```
