# SigLIP2

## Overview

[SigLIP2](https://huggingface.co/papers/2502.14786) is a family of multilingual vision-language encoders that builds on the [SigLIP](./siglip) training recipe. It includes decoder-based pretraining, self-distillation, and masked prediction to improve dense prediction tasks (segmentation, depth estimation, etc.). This model is available in two variants:

- NaFlex supports different resolutions and maintains the native image aspect ratio
- FixRes supports fixed resolutions and is backwards compatible with [SigLIP](./siglip)

You can find all the original SigLIP2 checkpoints under the [SigLIP2](https://huggingface.co/collections/google/siglip2-67b5dcef38c175486e240107) collection.

> [!TIP]
> Click on the SigLIP2 models in the right sidebar for more examples of how to apply SigLIP2 to different image and text tasks.

The example below demonstrates zero-shot classification with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

image = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
candidate_labels = ["a Pallas cat", "a lion", "a Siberian tiger"]

pipeline = pipeline(task="zero-shot-image-classification", model="google/siglip2-base-patch16-224", device=0)
pipeline(image, candidate_labels=candidate_labels)
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoModel, AutoProcessor

model = AutoModel.from_pretrained("google/siglip2-base-patch16-224", device_map="auto", attn_implementation="sdpa")
processor = AutoProcessor.from_pretrained("google/siglip2-base-patch16-224")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
candidate_labels = ["a Pallas cat", "a lion", "a Siberian tiger"]

# follows the pipeline prompt template to get same results
texts = [f'This is a photo of {label}.' for label in candidate_labels]

# IMPORTANT: we pass `padding=max_length` and `max_length=64` since the model was trained with this
inputs = processor(text=texts, images=image, padding="max_length", max_length=64, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

logits_per_image = outputs.logits_per_image
probs = torch.sigmoid(logits_per_image)
print(f"{probs[0][0]:.1%} that image 0 is '{candidate_labels[0]}'")
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoModel, AutoProcessor

model = AutoModel.from_pretrained("google/siglip2-base-patch16-naflex", device_map="auto", attn_implementation="sdpa")
processor = AutoProcessor.from_pretrained("google/siglip2-base-patch16-naflex")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
candidate_labels = ["a Pallas cat", "a lion", "a Siberian tiger"]
texts = [f'This is a photo of {label}.' for label in candidate_labels]

# default value for `max_num_patches` is 256, but you can increase resulted image resolution providing higher values e.g. `max_num_patches=512`
inputs = processor(text=texts, images=image, padding="max_length", max_num_patches=256, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

logits_per_image = outputs.logits_per_image
probs = torch.sigmoid(logits_per_image)
print(f"{probs[0][0]:.1%} that image 0 is '{candidate_labels[0]}'")
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to only quantize the weights to int4.

```python
import requests
import torch
from PIL import Image

from transformers import AutoModel, AutoProcessor, BitsAndBytesConfig

bnb_config = BitsAndBytesConfig(load_in_4bit=True)
model = AutoModel.from_pretrained("google/siglip2-base-patch16-224", quantization_config=bnb_config, device_map="auto", attn_implementation="sdpa")
processor = AutoProcessor.from_pretrained("google/siglip2-base-patch16-224")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
candidate_labels = ["a Pallas cat", "a lion", "a Siberian tiger"]

# follows the pipeline prompt template to get same results
texts = [f'This is a photo of {label}.' for label in candidate_labels]

# IMPORTANT: we pass `padding=max_length` and `max_length=64` since the model was trained with this
inputs = processor(text=texts, images=image, padding="max_length", max_length=64, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

logits_per_image = outputs.logits_per_image
probs = torch.sigmoid(logits_per_image)
print(f"{probs[0][0]:.1%} that image 0 is '{candidate_labels[0]}'")
```
## Text embeddings and retrieval

SigLIP2 can be used to generate text embeddings for retrieval or similarity-based tasks (for example, product or caption retrieval).

For best results, the same text preprocessing used during training must be applied. When loading SigLIP2 checkpoints via [AutoProcessor], this preprocessing is handled automatically by the processor.

### Default text preprocessing (handled automatically)

For SigLIP2 models, the processor applies the following defaults for text inputs:
- **Lowercasing** all input text
- **Fixed padding and truncation**: `padding="max_length"`, `max_length=64`, `truncation=True`

These defaults ensure consistent and correct text embeddings. Overriding them may lead to degraded retrieval quality.

### Example: computing text embeddings

```python
import torch

from transformers import AutoModel, AutoProcessor

model_id = "google/siglip2-so400m-patch14-384"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModel.from_pretrained(model_id).eval( device_map="auto")

texts = [
    "HOME084 Timbangan Badan Digital Kaca Transparan 28CM Body Scale Personal Scale",
    "26cm Timbangan Badan digital personal scale weight",
    "33cm Timbangan Badan digital personal scale weight",
]

# NOTE: lowercasing and padding/truncation to length 64 are applied automatically by the processor pipeline.
inputs = processor(text=texts, return_tensors="pt").to(model.device)

with torch.no_grad():
    text_features = model.get_text_features(**inputs)

# Normalize embeddings for cosine similarity
text_features = text_features / text_features.norm(p=2, dim=-1, keepdim=True)
```
### Text-only usage: Siglip2Tokenizer

If you are encoding text without a processor (for example, via [AutoTokenizer]), use [Siglip2Tokenizer].

- Siglip2Tokenizer applies lowercasing at the tokenizer backend level (matching SigLIP2 training time normalization), while keeping the same tokenization as the original tokenizer.

- When using the tokenizer directly, you should explicitly apply the same padding/truncation settings as used during training (e.g. max_length=64):

```python
from transformers import Siglip2Tokenizer

model_id = "google/siglip2-so400m-patch14-384"
tokenizer = Siglip2Tokenizer.from_pretrained(model_id)

inputs = tokenizer(
    ["HELLO WORLD"],
    padding="max_length",
    truncation=True,
    max_length=64,
    return_tensors="pt",
)
```
## Notes

- Training is supported for DDP and FSDP on single-node multi-accelerator setups. However, it does not use [torch.distributed](https://pytorch.org/tutorials/beginner/dist_overview.html) utilities which may limit the scalability of batch size.
- When using the standalone [GemmaTokenizerFast](/docs/transformers/v5.14.0/en/model_doc/gemma#transformers.GemmaTokenizer) make sure to pass `padding="max_length"` and `max_length=64` as that's how the model was trained.
- Model was trained with *lowercased* text, so make sure your text labels are preprocessed the same way.
- To get the same results as the [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline), a prompt template of `"This is a photo of {label}."` should be passed to the processor.
- The NaFlex variant processes different types of images at the appropriate resolution (using a larger resolution to process document images for example), while also minimizing the impact of aspect ratio distortion for certain inference tasks like OCR.

   NaFlex resizes the input image so the height and width are multiples of the patch size after resizing. It keeps the aspect ratio distortion as low as possible and produces a sequence length of at most the desired target sequence length (`max_num_patches`). After resizing, the image is split into a sequence of patches and a mask with padding information is added.
- Toggle the `attn_implementation` parameter to either `"sdpa"` or `"flash_attention_2"` to use a more memory-efficient attention.

    ```py
    # pip install -U flash-attn --no-build-isolation

    from transformers import SiglipModel

    model = SiglipModel.from_pretrained(
        "google/siglip2-so400m-patch14-384",
        attn_implementation="flash_attention_2",
        device_map="auto",
    )
    ```

## Siglip2Config[[transformers.Siglip2Config]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).

This is the configuration class to store the configuration of a Siglip2Model. It is used to instantiate a Siglip2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/siglip2-base-patch16-naflex](https://huggingface.co/google/siglip2-base-patch16-naflex)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Siglip2Config, Siglip2Model

>>> # Initializing a Siglip2Config with google/siglip2-base-patch16-224 style configuration
>>> configuration = Siglip2Config()

>>> # Initializing a Siglip2Model (with random weights) from the google/siglip2-base-patch16-224 style configuration
>>> model = Siglip2Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a Siglip2Config from a Siglip2TextConfig and a Siglip2VisionConfig
>>> from transformers import Siglip2TextConfig, Siglip2VisionConfig

>>> # Initializing a Siglip2Text and Siglip2Vision configuration
>>> config_text = Siglip2TextConfig()
>>> config_vision = Siglip2VisionConfig()

>>> config = Siglip2Config(text_config=config_text, vision_config=config_vision)
```

## Siglip2TextConfig[[transformers.Siglip2TextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `32000`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **max_position_embeddings** (`int`, *optional*, defaults to `64`) --
  The maximum sequence length that this model might ever be used with.
- **hidden_act** (`str`, *optional*, defaults to `gelu_pytorch_tanh`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **pad_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `49406`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `49407`) --
  Token id used for end-of-stream in the vocabulary.
- **projection_size** (`int`, *optional*) --
  Dimensionality of text and vision projection layers.

This is the configuration class to store the configuration of a Siglip2Model. It is used to instantiate a Siglip2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/siglip2-base-patch16-naflex](https://huggingface.co/google/siglip2-base-patch16-naflex)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Siglip2TextConfig, Siglip2TextModel

>>> # Initializing a Siglip2TextConfig with google/siglip2-base-patch16-224 style configuration
>>> configuration = Siglip2TextConfig()

>>> # Initializing a Siglip2TextModel (with random weights) from the google/siglip2-base-patch16-224 style configuration
>>> model = Siglip2TextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Siglip2VisionConfig[[transformers.Siglip2VisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **hidden_act** (`str`, *optional*, defaults to `gelu_pytorch_tanh`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **num_patches** (`int`, *optional*, defaults to 256) --
  The number of patches in the image with the size of (`patch_size`, `patch_size`).
  The image is resized to fill maximum of this number of patches, and to preserve
  the aspect ratio. In case the resulted number of patches is lower, the image is
  padded in "patch" dimension.

This is the configuration class to store the configuration of a Siglip2Model. It is used to instantiate a Siglip2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/siglip2-base-patch16-naflex](https://huggingface.co/google/siglip2-base-patch16-naflex)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Siglip2VisionConfig, Siglip2VisionModel

>>> # Initializing a Siglip2VisionConfig with google/siglip2-base-patch16-naflex style configuration
>>> configuration = Siglip2VisionConfig()

>>> # Initializing a Siglip2VisionModel (with random weights) from the google/siglip2-base-patch16-naflex style configuration
>>> model = Siglip2VisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Siglip2ImageProcessor[[transformers.Siglip2ImageProcessor]]

- **patch_size** (`int`, *kwargs*, *optional*, defaults to `self.patch_size`) --
  The size (resolution) of each patch the image will be split to.
- **max_num_patches** (`int`, *kwargs*, *optional*, defaults to `self.max_num_patches`) --
  The image will be resized to have at most this number of patches,
  and then padded in "patch" dimension to match this number exactly.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Siglip2ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`int`, *kwargs*, *optional*, defaults to `self.patch_size`) --
  The size (resolution) of each patch the image will be split to.
- **max_num_patches** (`int`, *kwargs*, *optional*, defaults to `self.max_num_patches`) --
  The image will be resized to have at most this number of patches,
  and then padded in "patch" dimension to match this number exactly.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Siglip2ImageProcessorPil[[transformers.Siglip2ImageProcessorPil]]

- **patch_size** (`int`, *kwargs*, *optional*, defaults to `self.patch_size`) --
  The size (resolution) of each patch the image will be split to.
- **max_num_patches** (`int`, *kwargs*, *optional*, defaults to `self.max_num_patches`) --
  The image will be resized to have at most this number of patches,
  and then padded in "patch" dimension to match this number exactly.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Siglip2ImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`int`, *kwargs*, *optional*, defaults to `self.patch_size`) --
  The size (resolution) of each patch the image will be split to.
- **max_num_patches** (`int`, *kwargs*, *optional*, defaults to `self.max_num_patches`) --
  The image will be resized to have at most this number of patches,
  and then padded in "patch" dimension to match this number exactly.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Siglip2Processor[[transformers.Siglip2Processor]]

- **image_processor** (`Siglip2ImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`Siglip2Tokenizer`) --
  The tokenizer is a required input.
Constructs a Siglip2Processor which wraps a image processor and a tokenizer into a single processor.

[Siglip2Processor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Processor) offers all the functionalities of [Siglip2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ImageProcessor) and [Siglip2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Tokenizer). See the
[~Siglip2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ImageProcessor) and [~Siglip2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Tokenizer) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **videos** (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) --
  Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If
  passing in videos with pixel values between 0 and 1, set `do_rescale=False`.
- **audio** (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) --
  The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor.
  In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels,
  and T is the sample length of the audio.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.

## Siglip2Model[[transformers.Siglip2Model]]

- **config** ([Siglip2Config](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Siglip2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Siglip2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ImageProcessor). See `Siglip2ImageProcessor.__call__()` for details ([Siglip2Processor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Processor) uses
  [Siglip2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ImageProcessor) for processing images).
- **pixel_attention_mask** (`torch.Tensor` of shape `(batch_size, image_size, image_size)`, *optional*) --
  Mask to avoid performing attention on padding pixel indices.
- **spatial_shapes** (`torch.LongTensor` of shape `(batch_size, 2)`) --
  Tensor containing the spatial dimensions (height, width) of the input images.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **return_loss** (`bool`, *optional*) --
  Whether or not to return the contrastive loss.`Siglip2Output` or `tuple(torch.FloatTensor)`A `Siglip2Output` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Siglip2Config](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Config)) and inputs.
The [Siglip2Model](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Contrastive loss for image-text similarity.
- **logits_per_image** (`torch.FloatTensor` of shape `(image_batch_size, text_batch_size)`) -- The scaled dot product scores between `image_embeds` and `text_embeds`. This represents the image-text
  similarity scores.
- **logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, image_batch_size)`) -- The scaled dot product scores between `text_embeds` and `image_embeds`. This represents the text-image
  similarity scores.
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The text embeddings obtained by applying the projection layer to the pooled output of [Siglip2TextModel](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2TextModel).
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The image embeddings obtained by applying the projection layer to the pooled output of [Siglip2VisionModel](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2VisionModel).
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [Siglip2TextModel](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2TextModel).
- **vision_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [Siglip2VisionModel](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2VisionModel).

Examples:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, AutoModel
>>> import torch

>>> model = AutoModel.from_pretrained("google/siglip2-base-patch16-224")
>>> processor = AutoProcessor.from_pretrained("google/siglip2-base-patch16-224")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> texts = ["a photo of 2 cats", "a photo of 2 dogs"]
>>> # important: we pass `padding=max_length` since the model was trained with this
>>> inputs = processor(text=texts, images=image, padding="max_length", return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> logits_per_image = outputs.logits_per_image
>>> probs = torch.sigmoid(logits_per_image) # these are the probabilities
>>> print(f"{probs[0][0]:.1%} that image 0 is '{texts[0]}'")
31.9% that image 0 is 'a photo of 2 cats'
```

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "position_ids", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_ids** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Siglip2Config](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Config)) and inputs.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoTokenizer, AutoModel
>>> import torch

>>> model = AutoModel.from_pretrained("google/siglip2-base-patch16-224")
>>> tokenizer = AutoTokenizer.from_pretrained("google/siglip2-base-patch16-224")

>>> # important: make sure to set padding="max_length" as that's how the model was trained
>>> inputs = tokenizer(["a photo of a cat", "a photo of a dog"], padding="max_length", return_tensors="pt")
>>> with torch.no_grad():
...     text_features = model.get_text_features(**inputs)
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Siglip2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ImageProcessor). See `Siglip2ImageProcessor.__call__()` for details ([Siglip2Processor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Processor) uses
  [Siglip2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ImageProcessor) for processing images).
- **pixel_attention_mask** (`torch.Tensor` of shape `(batch_size, image_size, image_size)`, *optional*) --
  Mask to avoid performing attention on padding pixel indices.
- **spatial_shapes** (`torch.LongTensor` of shape `(batch_size, 2)`) --
  Tensor containing the spatial dimensions (height, width) of the input images.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Siglip2Config](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Config)) and inputs.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> import torch
>>> from transformers import AutoProcessor, AutoModel
>>> from transformers.image_utils import load_image

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = load_image(url)

>>> model = AutoModel.from_pretrained("google/siglip2-base-patch16-224")
>>> processor = AutoProcessor.from_pretrained("google/siglip2-base-patch16-224")

>>> inputs = processor(images=image, return_tensors="pt")

>>> with torch.no_grad():
...     image_features = model.get_image_features(**inputs)
```

## Siglip2TextModel[[transformers.Siglip2TextModel]]

- **config** ([Siglip2TextConfig](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2TextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The text model from Siglip2 without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Siglip2Config](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Config)) and inputs.
The [Siglip2TextModel](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2TextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoTokenizer, Siglip2TextModel

>>> model = Siglip2TextModel.from_pretrained("google/siglip2-base-patch16-224")
>>> tokenizer = AutoTokenizer.from_pretrained("google/siglip2-base-patch16-224")

>>> # important: make sure to set padding="max_length" as that's how the model was trained
>>> inputs = tokenizer(["a photo of a cat", "a photo of a dog"], padding="max_length", return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooled_output = outputs.pooler_output  # pooled (EOS token) states
```

## Siglip2VisionModel[[transformers.Siglip2VisionModel]]

- **config** ([Siglip2VisionConfig](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2VisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The vision model from Siglip2 without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "spatial_shapes", "val": ": LongTensor"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Siglip2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ImageProcessor). See `Siglip2ImageProcessor.__call__()` for details ([Siglip2Processor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Processor) uses
  [Siglip2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ImageProcessor) for processing images).
- **pixel_attention_mask** (`torch.Tensor` of shape `(batch_size, image_size, image_size)`, *optional*) --
  Mask to avoid performing attention on padding pixel indices.
- **spatial_shapes** (`torch.LongTensor` of shape `(batch_size, 2)`) --
  Tensor containing the spatial dimensions (height, width) of the input images.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Siglip2Config](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Config)) and inputs.
The [Siglip2VisionModel](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2VisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state of the first token of the sequence (classification token) after further processing
  through the layers used for the auxiliary pretraining task. E.g. for BERT-family of models, this returns
  the classification token after processing through a linear layer and a tanh activation function. The linear
  layer weights are trained from the next sentence prediction (classification) objective during pretraining.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> import httpx
>>> from io import BytesIO
>>> from PIL import Image
>>> from transformers import AutoProcessor, Siglip2VisionModel

>>> model = Siglip2VisionModel.from_pretrained("google/siglip2-base-patch16-224")
>>> processor = AutoProcessor.from_pretrained("google/siglip2-base-patch16-224")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooled_output = outputs.pooler_output  # pooled features
```

## Siglip2ForImageClassification[[transformers.Siglip2ForImageClassification]]

- **config** ([Siglip2Config](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Siglip2 vision encoder with an image classification head on top (a linear layer on top of the pooled final hidden states of
the patch tokens) e.g. for ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Siglip2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ImageProcessor). See `Siglip2ImageProcessor.__call__()` for details ([Siglip2Processor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Processor) uses
  [Siglip2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ImageProcessor) for processing images).
- **pixel_attention_mask** (`torch.Tensor` of shape `(batch_size, image_size, image_size)`, *optional*) --
  Mask to avoid performing attention on padding pixel indices.
- **spatial_shapes** (`torch.LongTensor` of shape `(batch_size, 2)`) --
  Tensor containing the spatial dimensions (height, width) of the input images.
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the image classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).[ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or `tuple(torch.FloatTensor)`A [ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Siglip2Config](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2Config)) and inputs.
The [Siglip2ForImageClassification](/docs/transformers/v5.14.0/en/model_doc/siglip2#transformers.Siglip2ForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, Siglip2ForImageClassification
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> torch.manual_seed(3)
>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> # note: we are loading a `Siglip2Model` from the hub here,
>>> # so the head will be randomly initialized, hence the predictions will be random if seed is not set above.
>>> image_processor = AutoImageProcessor.from_pretrained("google/siglip2-base-patch16-224")
>>> model = Siglip2ForImageClassification.from_pretrained("google/siglip2-base-patch16-224")

>>> inputs = image_processor(images=image, return_tensors="pt")
>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> # model predicts one of the two classes
>>> predicted_class_idx = logits.argmax(-1).item()
>>> print("Predicted class:", model.config.id2label[predicted_class_idx])
Predicted class: LABEL_1
```

## Siglip2Tokenizer[[transformers.Siglip2Tokenizer]]

'"}, {"name": "bos_token", "val": ": str = ''"}, {"name": "eos_token", "val": ": str = ''"}, {"name": "pad_token", "val": ": str = ''"}, {"name": "mask_token", "val": ": str = ''"}, {"name": "**kwargs", "val": ""}]}>

Gemma tokenizer + SigLIP2 training default: lowercase normalization.

- **text** (`str`, `list[str]`, `list[list[str]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set
  `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).
- **text_pair** (`str`, `list[str]`, `list[list[str]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set
  `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).
- **text_target** (`str`, `list[str]`, `list[list[str]]`, *optional*) --
  The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a
  list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized),
  you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).
- **text_pair_target** (`str`, `list[str]`, `list[list[str]]`, *optional*) --
  The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a
  list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized),
  you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).
- **tokenizer_kwargs** (`dict[str, Any]`, *optional*) --
  Additional kwargs to pass to the tokenizer. These will be merged with the explicit parameters and
  other kwargs, with explicit parameters taking precedence.

- **add_special_tokens** (`bool`, *optional*, defaults to `True`) --
  Whether or not to add special tokens when encoding the sequences. This will use the underlying
  `PretrainedTokenizerBase.build_inputs_with_special_tokens` function, which defines which tokens are
  automatically added to the input ids. This is useful if you want to add `bos` or `eos` tokens
  automatically.
- **padding** (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) --
  Activates and controls padding. Accepts the following values:

  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single
    sequence is provided).
  - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum
    acceptable input length for the model if that argument is not provided.
  - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different
    lengths).
- **truncation** (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) --
  Activates and controls truncation. Accepts the following values:

  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or
    to the maximum acceptable input length for the model if that argument is not provided. This will
    truncate token by token, removing a token from the longest sequence in the pair if a pair of
    sequences (or a batch of pairs) is provided.
  - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the
    maximum acceptable input length for the model if that argument is not provided. This will only
    truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided.
  - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the
    maximum acceptable input length for the model if that argument is not provided. This will only
    truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided.
  - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths
    greater than the model maximum admissible input size).
- **max_length** (`int`, *optional*) --
  Controls the maximum length to use by one of the truncation/padding parameters.

  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length
  is required by one of the truncation/padding parameters. If the model has no specific maximum input
  length (like XLNet) truncation/padding to a maximum length will be deactivated.
- **stride** (`int`, *optional*, defaults to 0) --
  If set to a number along with `max_length`, the overflowing tokens returned when
  `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence
  returned to provide some overlap between truncated and overflowing sequences. The value of this
  argument defines the number of overlapping tokens.
- **is_split_into_words** (`bool`, *optional*, defaults to `False`) --
  Whether or not the input is already pre-tokenized (e.g., split into words). If set to `True`, the
  tokenizer assumes the input is already split into words (for instance, by splitting it on whitespace)
  which it will tokenize. This is useful for NER or token classification.
- **pad_to_multiple_of** (`int`, *optional*) --
  If set will pad the sequence to a multiple of the provided value. Requires `padding` to be activated.
  This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability
  `>= 7.5` (Volta).
- **padding_side** (`str`, *optional*) --
  The side on which the model should have padding applied. Should be selected between ['right', 'left'].
  Default value is picked from the class attribute of the same name.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors instead of list of python integers. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return Numpy `np.ndarray` objects.

- **return_token_type_ids** (`bool`, *optional*) --
  Whether to return token type IDs. If left to the default, will return the token type IDs according to
  the specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are token type IDs?](../glossary#token-type-ids)
- **return_attention_mask** (`bool`, *optional*) --
  Whether to return the attention mask. If left to the default, will return the attention mask according
  to the specific tokenizer's default, defined by the `return_outputs` attribute.

  [What are attention masks?](../glossary#attention-mask)
- **return_overflowing_tokens** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch
  of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead
  of returning overflowing tokens.
- **return_special_tokens_mask** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return special tokens mask information.
- **return_offsets_mapping** (`bool`, *optional*, defaults to `False`) --
  Whether or not to return `(char_start, char_end)` for each token.

  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend), if using
  Python's tokenizer, this method will raise `NotImplementedError`.
- **return_length**  (`bool`, *optional*, defaults to `False`) --
  Whether or not to return the lengths of the encoded inputs.
- **verbose** (`bool`, *optional*, defaults to `True`) --
  Whether or not to print more information and warnings.
- ****kwargs** -- passed to the `self.tokenize()` method[BatchEncoding](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.BatchEncoding)A [BatchEncoding](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields:

- **input_ids** -- List of token ids to be fed to a model.

  [What are input IDs?](../glossary#input-ids)

- **token_type_ids** -- List of token type ids to be fed to a model (when `return_token_type_ids=True` or
  if *"token_type_ids"* is in `self.model_input_names`).

  [What are token type IDs?](../glossary#token-type-ids)

- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names`).

  [What are attention masks?](../glossary#attention-mask)

- **overflowing_tokens** -- List of overflowing tokens sequences (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **num_truncated_tokens** -- Number of tokens truncated (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **special_tokens_mask** -- List of 0s and 1s, with 1 specifying added special tokens and 0 specifying
  regular sequence tokens (when `add_special_tokens=True` and `return_special_tokens_mask=True`).
- **length** -- The length of the inputs (when `return_length=True`)

Main method to tokenize and prepare for the model one or several sequence(s) or one or several pair(s) of
sequences.
