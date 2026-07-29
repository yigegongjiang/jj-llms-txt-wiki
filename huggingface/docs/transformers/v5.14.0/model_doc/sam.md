# SAM

## Overview

SAM (Segment Anything Model) was proposed in [Segment Anything](https://huggingface.co/papers/2304.02643) by Alexander Kirillov, Eric Mintun, Nikhila Ravi, Hanzi Mao, Chloe Rolland, Laura Gustafson, Tete Xiao, Spencer Whitehead, Alex Berg, Wan-Yen Lo, Piotr Dollar, Ross Girshick.

The model can be used to predict segmentation masks of any object of interest given an input image.

![example image](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/sam-output.png)

The abstract from the paper is the following:

*We introduce the Segment Anything (SA) project: a new task, model, and dataset for image segmentation. Using our efficient model in a data collection loop, we built the largest segmentation dataset to date (by far), with over 1 billion masks on 11M licensed and privacy respecting images. The model is designed and trained to be promptable, so it can transfer zero-shot to new image distributions and tasks. We evaluate its capabilities on numerous tasks and find that its zero-shot performance is impressive -- often competitive with or even superior to prior fully supervised results. We are releasing the Segment Anything Model (SAM) and corresponding dataset (SA-1B) of 1B masks and 11M images at [https://segment-anything.com](https://segment-anything.com) to foster research into foundation models for computer vision.*

Tips:

- The model predicts binary masks that states the presence or not of the object of interest given an image.
- The model predicts much better results if input 2D points and/or input bounding boxes are provided
- You can prompt multiple points for the same image, and predict a single mask.
- Fine-tuning the model is not supported yet
- According to the paper, textual input should be also supported. However, at this time of writing this seems not to be supported according to [the official repository](https://github.com/facebookresearch/segment-anything/issues/4#issuecomment-1497626844).

This model was contributed by [ybelkada](https://huggingface.co/ybelkada) and [ArthurZ](https://huggingface.co/ArthurZ).
The original code can be found [here](https://github.com/facebookresearch/segment-anything).

Below is an example on how to run mask generation given an image and a 2D point:

```python
import requests
import torch
from PIL import Image

from transformers import SamModel, SamProcessor

model = SamModel.from_pretrained("facebook/sam-vit-huge", device_map="auto")
processor = SamProcessor.from_pretrained("facebook/sam-vit-huge")

img_url = "https://huggingface.co/ybelkada/segment-anything/resolve/main/assets/car.png"
raw_image = Image.open(requests.get(img_url, stream=True).raw).convert("RGB")
input_points = [[[450, 600]]]  # 2D location of a window in the image

inputs = processor(raw_image, input_points=input_points, return_tensors="pt").to(model.device)
with torch.no_grad():
    outputs = model(**inputs)

masks = processor.image_processor.post_process_masks(
    outputs.pred_masks.cpu(), inputs["original_sizes"].cpu(), inputs["reshaped_input_sizes"].cpu()
)
scores = outputs.iou_scores
```

You can also process your own masks alongside the input images in the processor to be passed to the model.

```python
import requests
import torch
from PIL import Image

from transformers import SamModel, SamProcessor

model = SamModel.from_pretrained("facebook/sam-vit-huge", device_map="auto")
processor = SamProcessor.from_pretrained("facebook/sam-vit-huge")

img_url = "https://huggingface.co/ybelkada/segment-anything/resolve/main/assets/car.png"
raw_image = Image.open(requests.get(img_url, stream=True).raw).convert("RGB")
mask_url = "https://huggingface.co/ybelkada/segment-anything/resolve/main/assets/car.png"
segmentation_map = Image.open(requests.get(mask_url, stream=True).raw).convert("1")
input_points = [[[450, 600]]]  # 2D location of a window in the image

inputs = processor(raw_image, input_points=input_points, segmentation_maps=segmentation_map, return_tensors="pt").to(model.device)
with torch.no_grad():
    outputs = model(**inputs)

masks = processor.image_processor.post_process_masks(
    outputs.pred_masks.cpu(), inputs["original_sizes"].cpu(), inputs["reshaped_input_sizes"].cpu()
)
scores = outputs.iou_scores
```

## Resources

A list of official Hugging Face and community (indicated by 🌎) resources to help you get started with SAM.

- [Demo notebook](https://github.com/huggingface/notebooks/blob/main/examples/segment_anything.ipynb) for using the model.
- [Demo notebook](https://github.com/huggingface/notebooks/blob/main/examples/automatic_mask_generation.ipynb) for using the automatic mask generation pipeline.
- [Demo notebook](https://github.com/NielsRogge/Transformers-Tutorials/blob/master/SAM/Run_inference_with_MedSAM_using_HuggingFace_Transformers.ipynb) for inference with MedSAM, a fine-tuned version of SAM on the medical domain. 🌎
- [Demo notebook](https://github.com/NielsRogge/Transformers-Tutorials/blob/master/SAM/Fine_tune_SAM_(segment_anything)_on_a_custom_dataset.ipynb) for fine-tuning the model on custom data. 🌎

## SlimSAM

SlimSAM, a pruned version of SAM, was proposed in [0.1% Data Makes Segment Anything Slim](https://huggingface.co/papers/2312.05284) by Zigeng Chen et al. SlimSAM reduces the size of the SAM models considerably while maintaining the same performance.

Checkpoints can be found on the [hub](https://huggingface.co/models?other=slimsam), and they can be used as a drop-in replacement of SAM.

## Grounded SAM

One can combine [Grounding DINO](grounding-dino) with SAM for text-based mask generation as introduced in [Grounded SAM: Assembling Open-World Models for Diverse Visual Tasks](https://huggingface.co/papers/2401.14159). You can refer to this [demo notebook](https://github.com/NielsRogge/Transformers-Tutorials/blob/master/Grounding%20DINO/GroundingDINO_with_Segment_Anything.ipynb) 🌍 for details.

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/grounded_sam.png"
alt="drawing" width="900"/>

 Grounded SAM overview. Taken from the original repository. 

## SamConfig[[transformers.SamConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **prompt_encoder_config** (Union[`dict`, `SamPromptEncoderConfig`], *optional*) --
  Dictionary of configuration options used to initialize [SamPromptEncoderConfig](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamPromptEncoderConfig).
- **mask_decoder_config** (Union[`dict`, `SamMaskDecoderConfig`], *optional*) --
  Dictionary of configuration options used to initialize [SamMaskDecoderConfig](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamMaskDecoderConfig).
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a SamModel. It is used to instantiate a Sam
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam-vit-huge](https://huggingface.co/facebook/sam-vit-huge)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     SamVisionConfig,
...     SamPromptEncoderConfig,
...     SamMaskDecoderConfig,
...     SamModel,
... )

>>> # Initializing a SamConfig with `"facebook/sam-vit-huge"` style configuration
>>> configuration = SamConfig()

>>> # Initializing a SamModel (with random weights) from the `"facebook/sam-vit-huge"` style configuration
>>> model = SamModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a SamConfig from a SamVisionConfig, SamPromptEncoderConfig, and SamMaskDecoderConfig

>>> # Initializing SAM vision, SAM Q-Former and language model configurations
>>> vision_config = SamVisionConfig()
>>> prompt_encoder_config = SamPromptEncoderConfig()
>>> mask_decoder_config = SamMaskDecoderConfig()

>>> config = SamConfig(vision_config, prompt_encoder_config, mask_decoder_config)
```

## SamVisionConfig[[transformers.SamVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **output_channels** (`int`, *optional*, defaults to 256) --
  Dimensionality of the output channels in the Patch Encoder.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1024`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `1e-10`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **mlp_ratio** (`float`, *optional*, defaults to `4.0`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **use_abs_pos** (`bool`, *optional*, defaults to `True`) --
  Whether to use absolute position embeddings.
- **use_rel_pos** (`bool`, *optional*, defaults to `True`) --
  Whether to use relative position embedding.
- **window_size** (`int`, *optional*, defaults to 14) --
  Window size for relative position.
- **global_attn_indexes** (`list[int]`, *optional*, defaults to `[2, 5, 8, 11]`) --
  The indexes of the global attention layers.
- **num_pos_feats** (`int`, *optional*, defaults to 128) --
  The dimensionality of the position embedding.
- **mlp_dim** (`int`, *optional*) --
  The dimensionality of the MLP layer in the Transformer encoder. If `None`, defaults to `mlp_ratio *
  hidden_size`.

This is the configuration class to store the configuration of a SamModel. It is used to instantiate a Sam
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam-vit-huge](https://huggingface.co/facebook/sam-vit-huge)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import (
...     SamVisionConfig,
...     SamVisionModel,
... )

>>> # Initializing a SamVisionConfig with `"facebook/sam-vit-huge"` style configuration
>>> configuration = SamVisionConfig()

>>> # Initializing a SamVisionModel (with random weights) from the `"facebook/sam-vit-huge"` style configuration
>>> model = SamVisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SamMaskDecoderConfig[[transformers.SamMaskDecoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **hidden_act** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **mlp_dim** (`int`, *optional*, defaults to 2048) --
  Dimensionality of the "intermediate" (i.e., feed-forward) layer in the Transformer encoder.
- **num_hidden_layers** (`int`, *optional*, defaults to `2`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **attention_downsample_rate** (`int`, *optional*, defaults to 2) --
  The downsampling rate of the attention layer.
- **num_multimask_outputs** (`int`, *optional*, defaults to 3) --
  The number of outputs from the `SamMaskDecoder` module. In the Segment Anything paper, this is set to 3.
- **iou_head_depth** (`int`, *optional*, defaults to 3) --
  The number of layers in the IoU head module.
- **iou_head_hidden_dim** (`int`, *optional*, defaults to 256) --
  The dimensionality of the hidden states in the IoU head module.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.

This is the configuration class to store the configuration of a SamModel. It is used to instantiate a Sam
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam-vit-huge](https://huggingface.co/facebook/sam-vit-huge)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## SamPromptEncoderConfig[[transformers.SamPromptEncoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1024`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **mask_input_channels** (`int`, *optional*, defaults to 16) --
  The number of channels to be fed to the `MaskDecoder` module.
- **num_point_embeddings** (`int`, *optional*, defaults to 4) --
  The number of point embeddings to be used.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.

This is the configuration class to store the configuration of a SamModel. It is used to instantiate a Sam
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam-vit-huge](https://huggingface.co/facebook/sam-vit-huge)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## SamProcessor[[transformers.SamProcessor]]

- **image_processor** (`SamImageProcessor`) --
  The image processor is a required input.
Constructs a SamProcessor which wraps a image processor into a single processor.

[SamProcessor](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamProcessor) offers all the functionalities of [SamImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamImageProcessor). See the
[~SamImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamImageProcessor) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.`~tokenization_utils_base.BatchEncoding`- **data** (`dict`, *optional*) -- Dictionary of lists/arrays/tensors returned by the `__call__`/`encode_plus`/`batch_encode_plus` methods
  ('input_ids', 'attention_mask', etc.).
- **encoding** (`tokenizers.Encoding` or `Sequence[tokenizers.Encoding]`, *optional*) -- If the tokenizer is a fast tokenizer which outputs additional information like mapping from word/character
  space to token space the `tokenizers.Encoding` instance or list of instance (for batches) hold this
  information.
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
- **prepend_batch_axis** (`bool`, *optional*, defaults to `False`) -- Whether or not to add a batch axis when converting to tensors (see `tensor_type` above). Note that this
  parameter has an effect if the parameter `tensor_type` is set, *otherwise has no effect*.
- **n_sequences** (`int`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## SamImageProcessor[[transformers.SamImageProcessor]]

- **mask_size** (`dict[str, *kwargs*, int]`, *optional*) --
  The size `{"longest_edge": int}` to resize the segmentation maps to.
- **mask_pad_size** (`dict[str, *kwargs*, int]`, *optional*) --
  The size `{"height": int, "width": int}` to pad the segmentation maps to. Must be larger than any segmentation
  map size provided for preprocessing.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a SamImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **mask_size** (`dict[str, *kwargs*, int]`, *optional*) --
  The size `{"longest_edge": int}` to resize the segmentation maps to.
- **mask_pad_size** (`dict[str, *kwargs*, int]`, *optional*) --
  The size `{"height": int, "width": int}` to pad the segmentation maps to. Must be larger than any segmentation
  map size provided for preprocessing.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## SamImageProcessorPil[[transformers.SamImageProcessorPil]]

- **mask_size** (`dict[str, *kwargs*, int]`, *optional*) --
  The size `{"longest_edge": int}` to resize the segmentation maps to.
- **mask_pad_size** (`dict[str, *kwargs*, int]`, *optional*) --
  The size `{"height": int, "width": int}` to pad the segmentation maps to. Must be larger than any segmentation
  map size provided for preprocessing.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a SamImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **segmentation_maps** (`ImageInput`, *optional*) --
  The segmentation maps to preprocess.
- **mask_size** (`dict[str, *kwargs*, int]`, *optional*) --
  The size `{"longest_edge": int}` to resize the segmentation maps to.
- **mask_pad_size** (`dict[str, *kwargs*, int]`, *optional*) --
  The size `{"height": int, "width": int}` to pad the segmentation maps to. Must be larger than any segmentation
  map size provided for preprocessing.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## SamVisionModel[[transformers.SamVisionModel]]

- **config** ([SamVisionConfig](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamVisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The vision model from Sam without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [SamImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamImageProcessor). See `SamImageProcessor.__call__()` for details ([SamProcessor](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamProcessor) uses
  [SamImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamImageProcessor) for processing images).`SamVisionEncoderOutput` or `tuple(torch.FloatTensor)`A `SamVisionEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SamConfig](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamConfig)) and inputs.
The [SamVisionModel](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim)` *optional* returned when model is initialized with `with_projection=True`) -- The image embeddings obtained by applying the projection layer to the pooler_output.
- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## SamModel[[transformers.SamModel]]

- **config** ([SamConfig](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Segment Anything Model (SAM) for generating segmentation masks, given an input image and
input points and labels, boxes, or masks.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [SamImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamImageProcessor). See `SamImageProcessor.__call__()` for details ([SamProcessor](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamProcessor) uses
  [SamImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamImageProcessor) for processing images).
- **input_points** (`torch.FloatTensor` of shape `(batch_size, num_points, 2)`) --
  Input 2D spatial points, this is used by the prompt encoder to encode the prompt. Generally yields to much
  better results. The points can be obtained by passing a list of list of list to the processor that will
  create corresponding `torch` tensors of dimension 4. The first dimension is the image batch size, the
  second dimension is the point batch size (i.e. how many segmentation masks do we want the model to predict
  per input point), the third dimension is the number of points per segmentation mask (it is possible to pass
  multiple points for a single mask), and the last dimension is the x (vertical) and y (horizontal)
  coordinates of the point. If a different number of points is passed either for each image, or for each
  mask, the processor will create "PAD" points that will correspond to the (0, 0) coordinate, and the
  computation of the embedding will be skipped for these points using the labels.
- **input_labels** (`torch.LongTensor` of shape `(batch_size, point_batch_size, num_points)`) --
  Input labels for the points, this is used by the prompt encoder to encode the prompt. According to the
  official implementation, there are 3 types of labels

  - `1`: the point is a point that contains the object of interest
  - `0`: the point is a point that does not contain the object of interest
  - `-1`: the point corresponds to the background

  We added the label:

  - `-10`: the point is a padding point, thus should be ignored by the prompt encoder

  The padding labels should be automatically done by the processor.
- **input_boxes** (`torch.FloatTensor` of shape `(batch_size, num_boxes, 4)`) --
  Input boxes for the points, this is used by the prompt encoder to encode the prompt. Generally yields to
  much better generated masks. The boxes can be obtained by passing a list of list of list to the processor,
  that will generate a `torch` tensor, with each dimension corresponding respectively to the image batch
  size, the number of boxes per image and the coordinates of the top left and bottom right point of the box.
  In the order (`x1`, `y1`, `x2`, `y2`):

  - `x1`: the x coordinate of the top left point of the input box
  - `y1`: the y coordinate of the top left point of the input box
  - `x2`: the x coordinate of the bottom right point of the input box
  - `y2`: the y coordinate of the bottom right point of the input box
- **input_masks** (`torch.FloatTensor` of shape `(batch_size, image_size, image_size)`) --
  SAM model also accepts segmentation masks as input. The mask will be embedded by the prompt encoder to
  generate a corresponding embedding, that will be fed later on to the mask decoder. These masks needs to be
  manually fed by the user, and they need to be of shape (`batch_size`, `image_size`, `image_size`).
- **image_embeddings** (`torch.FloatTensor` of shape `(batch_size, output_channels, window_size, window_size)`) --
  Image embeddings, this is used by the mask decder to generate masks and iou scores. For more memory
  efficient computation, users can first retrieve the image embeddings using the `get_image_embeddings`
  method, and then feed them to the `forward` method instead of feeding the `pixel_values`.
- **multimask_output** (`bool`, *optional*) --
  In the original implementation and paper, the model always outputs 3 masks per image (or per point / per
  bounding box if relevant). However, it is possible to just output a single mask, that corresponds to the
  "best" mask, by specifying `multimask_output=False`.
- **attention_similarity** (`torch.FloatTensor`, *optional*) --
  Attention similarity tensor, to be provided to the mask decoder for target-guided attention in case the
  model is used for personalization as introduced in [PerSAM](https://huggingface.co/papers/2305.03048).
- **target_embedding** (`torch.FloatTensor`, *optional*) --
  Embedding of the target concept, to be provided to the mask decoder for target-semantic prompting in case
  the model is used for personalization as introduced in [PerSAM](https://huggingface.co/papers/2305.03048).`SamImageSegmentationOutput` or `tuple(torch.FloatTensor)`A `SamImageSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SamConfig](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamConfig)) and inputs.
The [SamModel](/docs/transformers/v5.14.0/en/model_doc/sam#transformers.SamModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **iou_scores** (`torch.FloatTensor` of shape `(batch_size, num_masks)`) -- The iou scores of the predicted masks.
- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, num_masks, height, width)`) -- The predicted low resolutions masks. Needs to be post-processed by the processor
- **vision_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the vision model at the output of each layer plus the optional initial embedding outputs.
- **vision_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **mask_decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoModel, AutoProcessor

>>> model = AutoModel.from_pretrained("facebook/sam-vit-base")
>>> processor = AutoProcessor.from_pretrained("facebook/sam-vit-base")

>>> url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/sam-car.png"
>>> with httpx.stream("GET", url) as response:
...     raw_image = Image.open(BytesIO(response.read())).convert("RGB")
>>> input_points = [[[400, 650]]]  # 2D location of a window on the car
>>> inputs = processor(images=raw_image, input_points=input_points, return_tensors="pt")

>>> # Get segmentation mask
>>> outputs = model(**inputs)

>>> # Postprocess masks
>>> masks = processor.post_process_masks(
...     outputs.pred_masks, inputs["original_sizes"], inputs["reshaped_input_sizes"]
... )
```
