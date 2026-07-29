# KOSMOS-2.5

The Kosmos-2.5 model was proposed in [KOSMOS-2.5: A Multimodal Literate Model](https://huggingface.co/papers/2309.11419/) by Microsoft.

The abstract from the paper is the following:

*We present Kosmos-2.5, a multimodal literate model for machine reading of text-intensive images. Pre-trained on large-scale text-intensive images, Kosmos-2.5 excels in two distinct yet cooperative transcription tasks: (1) generating spatially-aware text blocks, where each block of text is assigned its spatial coordinates within the image, and (2) producing structured text output that captures styles and structures into the markdown format. This unified multimodal literate capability is achieved through a shared Transformer architecture, task-specific prompts, and flexible text representations. We evaluate Kosmos-2.5 on end-to-end document-level text recognition and image-to-markdown text generation. Furthermore, the model can be readily adapted for any text-intensive image understanding task with different prompts through supervised fine-tuning, making it a general-purpose tool for real-world applications involving text-rich images. This work also paves the way for the future scaling of multimodal large language models.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/kosmos2_5_ocr.png"
alt="drawing" width="600"/>

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/kosmos2_5_md.png"
alt="drawing" width="600"/>

 Overview of tasks that KOSMOS-2.5 can handle. Taken from the original paper. 

The examples below demonstrates how to generate with [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel), for both Markdown and OCR tasks.

```python
import requests
from PIL import Image

from transformers import AutoProcessor, Kosmos2_5ForConditionalGeneration

repo = "microsoft/kosmos-2.5"
model = Kosmos2_5ForConditionalGeneration.from_pretrained(repo, device_map="auto")
processor = AutoProcessor.from_pretrained(repo)

# sample image
url = "https://huggingface.co/microsoft/kosmos-2.5/resolve/main/receipt_00008.png"
image = Image.open(requests.get(url, stream=True).raw)

prompt = "<md>"
inputs = processor(text=prompt, images=image, return_tensors="pt").to(model.device)

height, width = inputs.pop("height"), inputs.pop("width")
raw_width, raw_height = image.size
scale_height = raw_height / height
scale_width = raw_width / width

inputs = {k: v.to(model.device) if v is not None else None for k, v in inputs.items()}
inputs["flattened_patches"] = inputs["flattened_patches"].to(dtype)
generated_ids = model.generate(
    **inputs,
    max_new_tokens=1024,
)

generated_text = processor.batch_decode(generated_ids, skip_special_tokens=True)
print(generated_text[0])
```

```python
import re

import requests
from PIL import Image, ImageDraw

from transformers import AutoProcessor, Kosmos2_5ForConditionalGeneration

repo = "microsoft/kosmos-2.5"
model = Kosmos2_5ForConditionalGeneration.from_pretrained(repo, device_map="auto")
processor = AutoProcessor.from_pretrained(repo)

# sample image
url = "https://huggingface.co/microsoft/kosmos-2.5/resolve/main/receipt_00008.png"
image = Image.open(requests.get(url, stream=True).raw)

# bs = 1
prompt = "<ocr>"
inputs = processor(text=prompt, images=image, return_tensors="pt").to(model.device)
height, width = inputs.pop("height"), inputs.pop("width")
raw_width, raw_height = image.size
scale_height = raw_height / height
scale_width = raw_width / width

# bs > 1, batch generation
# inputs = processor(text=[prompt, prompt], images=[image,image], return_tensors="pt").to(model.device)
# height, width = inputs.pop("height"), inputs.pop("width")
# raw_width, raw_height = image.size
# scale_height = raw_height / height[0]
# scale_width = raw_width / width[0]

inputs = {k: v.to(model.device) if v is not None else None for k, v in inputs.items()}
inputs["flattened_patches"] = inputs["flattened_patches"].to(dtype)
generated_ids = model.generate(
    **inputs,
    max_new_tokens=1024,
)

generated_text = processor.batch_decode(generated_ids, skip_special_tokens=True)
def post_process(y, scale_height, scale_width):
    y = y.replace(prompt, "")
    if "<md>" in prompt:
        return y
    pattern = r"<bbox><x_\d+><y_\d+><x_\d+><y_\d+></bbox>"
    bboxs_raw = re.findall(pattern, y)
    lines = re.split(pattern, y)[1:]
    bboxs = [re.findall(r"\d+", i) for i in bboxs_raw]
    bboxs = [[int(j) for j in i] for i in bboxs]
    info = ""
    for i in range(len(lines)):
        box = bboxs[i]
        x0, y0, x1, y1 = box
        if not (x0 >= x1 or y0 >= y1):
            x0 = int(x0 * scale_width)
            y0 = int(y0 * scale_height)
            x1 = int(x1 * scale_width)
            y1 = int(y1 * scale_height)
            info += f"{x0},{y0},{x1},{y0},{x1},{y1},{x0},{y1},{lines[i]}"
    return info

output_text = post_process(generated_text[0], scale_height, scale_width)
print(output_text)

draw = ImageDraw.Draw(image)
lines = output_text.split("\n")
for line in lines:
    # draw the bounding box
    line = list(line.split(","))
    if len(line) < 8:
        continue
    line = list(map(int, line[:8]))
    draw.polygon(line, outline="red")
image.save("output.png")
```

## Chat version

The authors also released Kosmos-2.5 Chat, which is a chat version optimized for document understanding. You can use it like so:

```python
import requests
import torch
from PIL import Image

from transformers import AutoProcessor, Kosmos2_5ForConditionalGeneration

repo = "microsoft/kosmos-2.5-chat"
dtype = torch.bfloat16

model = Kosmos2_5ForConditionalGeneration.from_pretrained(repo,
                                                          device_map="auto",
                                                          attn_implementation="flash_attention_2")
processor = AutoProcessor.from_pretrained(repo)

# sample image
url = "https://huggingface.co/microsoft/kosmos-2.5/resolve/main/receipt_00008.png"

image = Image.open(requests.get(url, stream=True).raw)

question = "What is the sub total of the receipt?"
template = "<md>A chat between a curious user and an artificial intelligence assistant. The assistant gives helpful, detailed, and polite answers to the user's questions. USER: {} ASSISTANT:"
prompt = template.format(question)
inputs = processor(text=prompt, images=image, return_tensors="pt").to(model.device)

height, width = inputs.pop("height"), inputs.pop("width")
raw_width, raw_height = image.size
scale_height = raw_height / height
scale_width = raw_width / width

inputs = {k: v.to(model.device) if v is not None else None for k, v in inputs.items()}
inputs["flattened_patches"] = inputs["flattened_patches"].to(dtype)
generated_ids = model.generate(
    **inputs,
    max_new_tokens=1024,
)

generated_text = processor.batch_decode(generated_ids, skip_special_tokens=True)
print(generated_text[0])
```

## Kosmos2_5Config[[transformers.Kosmos2_5Config]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **latent_query_num** (`int`, *optional*, defaults to 2048) --
  The number of latent query tokens that represent the image features used in the text decoder component.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Kosmos2 5Model. It is used to instantiate a Kosmos2 5
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/kosmos-2.5](https://huggingface.co/microsoft/kosmos-2.5)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Kosmos2_5TextConfig[[transformers.Kosmos2_5TextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `108481`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **max_position_embeddings** (`int`, *optional*, defaults to `4096`) --
  The maximum sequence length that this model might ever be used with.
- **embed_dim** (`int`, *optional*, defaults to `1536`) --
  Dimensionality of the embeddings and hidden states.
- **layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **ffn_dim** (`int`, *optional*, defaults to `6144`) --
  Dimension of the MLP representations.
- **attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **activation_function** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for activations inside the fully connected layer.
- **layerdrop** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The LayerDrop probability. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for
  more details.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **init_std** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **scale_embedding** (`bool`, *optional*, defaults to `True`) --
  Whether to scale embeddings by dividing by sqrt(d_model).
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **pad_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.

This is the configuration class to store the configuration of a Kosmos2 5Model. It is used to instantiate a Kosmos2 5
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/kosmos-2.5](https://huggingface.co/microsoft/kosmos-2.5)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Kosmos2_5VisionConfig[[transformers.Kosmos2_5VisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1536`) --
  Dimension of the hidden representations.
- **patch_embed_hidden_size** (`int`, *optional*, defaults to 768) --
  Dimensionality of the input patch_embedding layer in the Transformer encoder.
- **intermediate_size** (`int`, *optional*, defaults to `3968`) --
  Dimension of the MLP representations.
- **head_dim** (`int`, *optional*, defaults to `64`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **num_hidden_layers** (`int`, *optional*, defaults to `18`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `24`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **dense_act_fn** (`str` or `function`, *optional*, defaults to `"gelu_new"`) --
  The non-linear activation function (function or string) in the encoder and pooler. If string, `"gelu"`,
  `"relu"`, `"selu"` and `"gelu_new"` ``"gelu"` are supported.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **dropout_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **max_num_patches** (`int`, *optional*, defaults to 4096) --
  Maximum sequence length (here number of patches) supported by the model.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Kosmos2 5Model. It is used to instantiate a Kosmos2 5
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/kosmos-2.5](https://huggingface.co/microsoft/kosmos-2.5)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Kosmos2_5VisionConfig, Kosmos2_5VisionModel

>>> # Initializing a Kosmos2_5VisionConfig with microsoft/kosmos-2.5 style configuration
>>> configuration = Kosmos2_5VisionConfig()

>>> # Initializing a Kosmos2_5VisionModel (with random weights) from the microsoft/kosmos-2.5 style configuration
>>> model = Kosmos2_5VisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Kosmos2_5ImageProcessor[[transformers.Kosmos2_5ImageProcessor]]

- **patch_size** (`Dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" -- 16, "width": 16}`):
  The patch size to use for the image. According to Kosmos2_5 paper and code, the patch size is 16x16.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to 4096) --
  The maximum number of patches to extract from the image as per the
  [KOSMOS 2.5 paper](https://huggingface.co/papers/2309.11419).
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a image_processor_class image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`Dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" -- 16, "width": 16}`):
  The patch size to use for the image. According to Kosmos2_5 paper and code, the patch size is 16x16.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to 4096) --
  The maximum number of patches to extract from the image as per the
  [KOSMOS 2.5 paper](https://huggingface.co/papers/2309.11419).
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Kosmos2_5ImageProcessorPil[[transformers.Kosmos2_5ImageProcessorPil]]

- **patch_size** (`Dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" -- 16, "width": 16}`):
  The patch size to use for the image. According to Kosmos2_5 paper and code, the patch size is 16x16.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to 4096) --
  The maximum number of patches to extract from the image as per the
  [KOSMOS 2.5 paper](https://huggingface.co/papers/2309.11419).
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a image_processor_class image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`Dict[str, *kwargs*, int]`, *optional*, defaults to `{"height" -- 16, "width": 16}`):
  The patch size to use for the image. According to Kosmos2_5 paper and code, the patch size is 16x16.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to 4096) --
  The maximum number of patches to extract from the image as per the
  [KOSMOS 2.5 paper](https://huggingface.co/papers/2309.11419).
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Kosmos2_5Processor[[transformers.Kosmos2_5Processor]]

- **image_processor** (`image_processor_class`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **num_image_tokens** (`int`, *optional*, defaults to 2048) --
  Number of image tokens used as a placeholder.
Constructs a Kosmos2_5Processor which wraps a image processor and a tokenizer into a single processor.

[Kosmos2_5Processor](/docs/transformers/v5.14.0/en/model_doc/kosmos2_5#transformers.Kosmos2_5Processor) offers all the functionalities of `image_processor_class` and `tokenizer_class`. See the
`~image_processor_class` and `~tokenizer_class` for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Kosmos2_5Model[[transformers.Kosmos2_5Model]]

- **config** ([Kosmos2_5Config](/docs/transformers/v5.14.0/en/model_doc/kosmos2_5#transformers.Kosmos2_5Config)) -- Model configuration class with all the parameters of the model.
  Initializing with a config file does not load the weights associated with the model, only the
  configuration. Check out the [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

KOSMOS-2.5 Model for generating text and image features. The model consists of a vision encoder and a language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you provide
  it.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **flattened_patches** (`torch.FloatTensor` of shape `(batch_size, max_patches, 2 + patch_height * patch_width * image_channels)`) --
  Flattened patches of the images. `flattened_patches` can be obtained using [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor). See
  `Kosmos2_5ImageProcessor.__call__()` for details.
- **width** (`torch.FloatTensor` of shape `(batch_size,)`) --
  The original width (before resizing) of each image in the batch. This can be obtained using
  [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor). See `Kosmos2_5ImageProcessor.__call__()` for details.
- **height** (`torch.FloatTensor` of shape `(batch_size,)`) --
  The original height (before resizing) of each image in the batch. This can be obtained using
  [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor). See `Kosmos2_5ImageProcessor.__call__()` for details.
- **image_embeds_position_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to indicate the location in a sequence to insert the image features . Mask values selected in `[0,
  1]`:

  - 1 for places where to put the image features,
  - 0 for places that are not for image features (i.e. for text tokens).

- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)

- **past_key_values** (`Cache` of length `config.n_layers` with each tuple having 4 tensors of shape `(batch_size, num_heads, sequence_length - 1, embed_size_per_head)`) --
  Contains precomputed key and value hidden states of the attention blocks. Can be used to speed up decoding.

  If `past_key_values` are used, the user can optionally input only the last `decoder_input_ids` (those that
  don't have their past key value states given to this model) of shape `(batch_size, 1)` instead of all
  `decoder_input_ids` of shape `(batch_size, sequence_length)`.
- **image_embeds** -- (`torch.FloatTensor` of shape `(batch_size, latent_query_num, hidden_size)`, *optional*):
  Sequence of hidden-states at the output of `Kosmos2ImageToTextProjection`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0,
  config.max_position_embeddings - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.`transformers.models.kosmos2_5.modeling_kosmos2_5.Kosmos2_5ModelOutput` or `tuple(torch.FloatTensor)`A `transformers.models.kosmos2_5.modeling_kosmos2_5.Kosmos2_5ModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`<class 'transformers.models.kosmos2_5.configuration_kosmos2_5.Kosmos2_5Config'>`) and inputs.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **width** (`torch.FloatTensor` of shape `(batch_size,)`) -- The original width (before resizing) of each image in the batch.
- **height** (`torch.FloatTensor` of shape `(batch_size,)`) -- The original height (before resizing) of each image in the batch.
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, latent_query_num, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of `Kosmos2ImageToTextProjection`.
- **projection_attentions** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights given by `Kosmos2ImageToTextProjection`, after the attention softmax, used to compute
  the weighted average in the self-attention heads.
- **vision_model_output(`BaseModelOutputWithPooling`,** *optional*) -- The output of the `Kosmos2VisionModel`.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
The [Kosmos2_5Model](/docs/transformers/v5.14.0/en/model_doc/kosmos2_5#transformers.Kosmos2_5Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Examples:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, Kosmos2_5Model

>>> model = Kosmos2_5Model.from_pretrained("microsoft/kosmos2.5")
>>> processor = AutoProcessor.from_pretrained("microsoft/kosmos2.5")

>>> url = "https://huggingface.co/microsoft/kosmos2.5/resolve/main/snowman.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> text = (
...     "<grounding> An image of<phrase> a snowman</phrase><object><patch_index_0044><patch_index_0863>"
...     "</object> warming himself by<phrase> a fire</phrase><object><patch_index_0005><patch_index_0911>"
...     "</object>"
... )

>>> inputs = processor(text=text, images=image, return_tensors="pt", add_eos_token=True)

>>> last_hidden_state = model(
...     pixel_values=inputs["pixel_values"],
...     input_ids=inputs["input_ids"],
...     attention_mask=inputs["attention_mask"],
...     image_embeds_position_mask=inputs["image_embeds_position_mask"],
... ).last_hidden_state
>>> list(last_hidden_state.shape)
[1, 91, 2048]
```

## Kosmos2_5ForConditionalGeneration[[transformers.Kosmos2_5ForConditionalGeneration]]

- **config** ([Kosmos2_5Config](/docs/transformers/v5.14.0/en/model_doc/kosmos2_5#transformers.Kosmos2_5Config)) -- Model configuration class with all the parameters of the model.
  Initializing with a config file does not load the weights associated with the model, only the
  configuration. Check out the [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

KOSMOS-2.5 Model for generating text and bounding boxes given an image. The model consists of a vision encoder and a
language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you provide
  it.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **flattened_patches** (`torch.FloatTensor` of shape `(batch_size, max_patches, 2 + patch_height * patch_width * image_channels)`) --
  Flattened patches of the images. `flattened_patches` can be obtained using [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor). See
  `Kosmos2_5ImageProcessor.__call__()` for details.
- **width** (`torch.FloatTensor` of shape `(batch_size,)`) --
  The original width (before resizing) of each image in the batch. This can be obtained using
  [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor). See `Kosmos2_5ImageProcessor.__call__()` for details.
- **height** (`torch.FloatTensor` of shape `(batch_size,)`) --
  The original height (before resizing) of each image in the batch. This can be obtained using
  [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor). See `Kosmos2_5ImageProcessor.__call__()` for details.
- **image_embeds_position_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to indicate the location in a sequence to insert the image features . Mask values selected in `[0,
  1]`:

  - 1 for places where to put the image features,
  - 0 for places that are not for image features (i.e. for text tokens).

- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)

- **past_key_values** (`Cache` of length `config.n_layers` with each tuple having 4 tensors of shape `(batch_size, num_heads, sequence_length - 1, embed_size_per_head)`) --
  Contains precomputed key and value hidden states of the attention blocks. Can be used to speed up decoding.

  If `past_key_values` are used, the user can optionally input only the last `decoder_input_ids` (those that
  don't have their past key value states given to this model) of shape `(batch_size, 1)` instead of all
  `decoder_input_ids` of shape `(batch_size, sequence_length)`.
- **image_embeds** -- (`torch.FloatTensor` of shape `(batch_size, latent_query_num, hidden_size)`, *optional*):
  Sequence of hidden-states at the output of `Kosmos2ImageToTextProjection`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0,
  config.max_position_embeddings - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.

- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the left-to-right language modeling loss (next word prediction). Indices should be in
  `[-100, 0, ..., config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are
  ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]``transformers.models.kosmos2_5.modeling_kosmos2_5.Kosmos2_5ForConditionalGenerationModelOutput` or `tuple(torch.FloatTensor)`A `transformers.models.kosmos2_5.modeling_kosmos2_5.Kosmos2_5ForConditionalGenerationModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration (`<class 'transformers.models.kosmos2_5.configuration_kosmos2_5.Kosmos2_5Config'>`) and inputs.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **width** (`torch.FloatTensor` of shape `(batch_size,)`) -- The original width (before resizing) of each image in the batch.
- **height** (`torch.FloatTensor` of shape `(batch_size,)`) -- The original height (before resizing) of each image in the batch.
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, latent_query_num, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of `Kosmos2ImageToTextProjection`.
- **projection_attentions** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights given by `Kosmos2ImageToTextProjection`, after the attention softmax, used to compute
  the weighted average in the self-attention heads.
- **vision_model_output(`BaseModelOutputWithPooling`,** *optional*) -- The output of the `Kosmos2VisionModel`.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
The [Kosmos2_5ForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/kosmos2_5#transformers.Kosmos2_5ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Examples:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> import torch
>>> from transformers import AutoProcessor, Kosmos2_5ForConditionalGeneration

>>> repo = "microsoft/kosmos-2.5"
>>> device = "cuda:0"
>>> dtype = torch.bfloat16 # torch.float16
>>> model = Kosmos2_5ForConditionalGeneration.from_pretrained(repo, device_map=device, dtype=dtype)
>>> processor = AutoProcessor.from_pretrained(repo)

>>> url = "https://huggingface.co/microsoft/kosmos-2.5/resolve/main/receipt_00008.png"

>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> prompt = "<ocr>" # <md>

>>> inputs = processor(text=prompt, images=image, return_tensors="pt")
>>> height, width = inputs.pop("height"), inputs.pop("width")
>>> inputs = {k: v.to(device) if v is not None else None for k, v in inputs.items()}
>>> inputs["flattened_patches"] = inputs["flattened_patches"].to(dtype)

>>> generated_ids = model.generate(**inputs,max_new_tokens=1024)
>>> generated_text = processor.batch_decode(generated_ids, skip_special_tokens=True)[0]
>>> generated_text
'<ocr><bbox><x_53><y_573><x_69><y_606></bbox>1\n<bbox><x_79><y_573><x_464><y_612></bbox>[REG] BLACK SAKURA\n<bbox><x_690><y_569><x_810><y_606></bbox>45,455\n<bbox><x_53><y_614><x_69><y_648></bbox>1\n<bbox><x_79><y_614><x_468><y_650></bbox>COOKIE DOH SAUCES\n<bbox><x_788><y_609><x_812><y_644></bbox>0\n<bbox><x_50><y_658><x_69><y_693></bbox>1\n<bbox><x_79><y_658><x_358><y_693></bbox>NATA DE COCO\n<bbox><x_790><y_652><x_814><y_687></bbox>0\n<bbox><x_31><y_742><x_820><y_781></bbox>Sub Total 45,455\n<bbox><x_27><y_781><x_822><y_827></bbox>PB1 (10%) 4,545\n<bbox><x_27><y_826><x_824><y_872></bbox>Rounding 0\n<bbox><x_24><y_872><x_827><y_921></bbox>Total 50,000\n<bbox><x_17><y_1056><x_836><y_1108></bbox>Card Payment 50,000\n'
```
