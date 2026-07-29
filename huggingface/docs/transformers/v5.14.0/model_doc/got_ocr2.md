# GOT-OCR2

## Overview

The GOT-OCR2 model was proposed in [General OCR Theory: Towards OCR-2.0 via a Unified End-to-end Model](https://huggingface.co/papers/2409.01704) by Haoran Wei, Chenglong Liu, Jinyue Chen, Jia Wang, Lingyu Kong, Yanming Xu, Zheng Ge, Liang Zhao, Jianjian Sun, Yuang Peng, Chunrui Han, Xiangyu Zhang.

The abstract from the paper is the following:

*Traditional OCR systems (OCR-1.0) are increasingly unable to meet people’snusage due to the growing demand for intelligent processing of man-made opticalncharacters. In this paper, we collectively refer to all artificial optical signals (e.g., plain texts, math/molecular formulas, tables, charts, sheet music, and even geometric shapes) as "characters" and propose the General OCR Theory along with an excellent model, namely GOT, to promote the arrival of OCR-2.0. The GOT, with 580M parameters, is a unified, elegant, and end-to-end model, consisting of a high-compression encoder and a long-contexts decoder. As an OCR-2.0 model, GOT can handle all the above "characters" under various OCR tasks. On the input side, the model supports commonly used scene- and document-style images in slice and whole-page styles. On the output side, GOT can generate plain or formatted results (markdown/tikz/smiles/kern) via an easy prompt. Besides, the model enjoys interactive OCR features, i.e., region-level recognition guided by coordinates or colors. Furthermore, we also adapt dynamic resolution and multipage OCR technologies to GOT for better practicality. In experiments, we provide sufficient results to prove the superiority of our model.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/got_ocr_overview.png"
alt="drawing" width="600"/>

 GOT-OCR2 training stages. Taken from the original paper. 

Tips:

GOT-OCR2 works on a wide range of tasks, including plain document OCR, scene text OCR, formatted document OCR, and even OCR for tables, charts, mathematical formulas, geometric shapes, molecular formulas and sheet music. While this implementation of the model will only output plain text, the outputs can be further processed to render the desired format, with packages like `pdftex`, `mathpix`, `matplotlib`, `tikz`, `verovio` or `pyecharts`.
The model can also be used for interactive OCR, where the user can specify the region to be recognized by providing the coordinates or the color of the region's bounding box.

This model was contributed by [yonigozlan](https://huggingface.co/yonigozlan).
The original code can be found [here](https://github.com/Ucas-HaoranWei/GOT-OCR2.0).

## Usage example

### Plain text inference

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", device_map="auto")
processor = AutoProcessor.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", use_fast=True)

image = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/image_ocr.jpg"
inputs = processor(image, return_tensors="pt", device=device).to(model.device)

generate_ids = model.generate(
    **inputs,
    do_sample=False,
    tokenizer=processor.tokenizer,
    stop_strings="<|im_end|>",
    max_new_tokens=4096,
)

processor.decode(generate_ids[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
"R&D QUALITY IMPROVEMENT\nSUGGESTION/SOLUTION FORM\nName/Phone Ext. : (...)"
```

### Plain text inference batched

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", device_map="auto")
processor = AutoProcessor.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", use_fast=True)

image1 = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/multi_box.png"
image2 = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/image_ocr.jpg"

inputs = processor([image1, image2], return_tensors="pt", device=device).to(model.device)

generate_ids = model.generate(
    **inputs,
    do_sample=False,
    tokenizer=processor.tokenizer,
    stop_strings="<|im_end|>",
    max_new_tokens=4,
)

processor.batch_decode(generate_ids[:, inputs["input_ids"].shape[1] :], skip_special_tokens=True)
["Reducing the number", "R&D QUALITY"]
```

### Formatted text inference

GOT-OCR2 can also generate formatted text, such as markdown or LaTeX. Here is an example of how to generate formatted text:

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", device_map="auto")
processor = AutoProcessor.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", use_fast=True)

image = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/latex.png"
inputs = processor(image, return_tensors="pt", format=True, device=device).to(model.device)

generate_ids = model.generate(
    **inputs,
    do_sample=False,
    tokenizer=processor.tokenizer,
    stop_strings="<|im_end|>",
    max_new_tokens=4096,
)

processor.decode(generate_ids[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
"\\author{\nHanwen Jiang* \\(\\quad\\) Arjun Karpur \\({ }^{\\dagger} \\quad\\) Bingyi Cao \\({ }^{\\dagger} \\quad\\) (...)"
```

### Inference on multiple pages

Although it might be reasonable in most cases to use a “for loop” for multi-page processing, some text data with formatting across several pages make it necessary to process all pages at once. GOT introduces a multi-page OCR (without “for loop”) feature, where multiple pages can be processed by the model at once, with the output being one continuous text.
Here is an example of how to process multiple pages at once:

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", device_map="auto")
processor = AutoProcessor.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", use_fast=True)

image1 = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/page1.png"
image2 = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/page2.png"
inputs = processor([image1, image2], return_tensors="pt", multi_page=True, format=True, device=device).to(model.device)

generate_ids = model.generate(
    **inputs,
    do_sample=False,
    tokenizer=processor.tokenizer,
    stop_strings="<|im_end|>",
    max_new_tokens=4096,
)

processor.decode(generate_ids[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
"\\title{\nGeneral OCR Theory: Towards OCR-2.0 via a Unified End-to-end Model\n}\n\\author{\nHaoran Wei (...)"
```

### Inference on cropped patches

GOT supports a 1024×1024 input resolution, which is sufficient for most OCR tasks, such as scene OCR or processing A4-sized PDF pages. However, certain scenarios, like horizontally stitched two-page PDFs commonly found in academic papers or images with unusual aspect ratios, can lead to accuracy issues when processed as a single image. To address this, GOT can dynamically crop an image into patches, process them all at once, and merge the results for better accuracy with such inputs.
Here is an example of how to process cropped patches:

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", device_map="auto")
processor = AutoProcessor.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", use_fast=True)

image = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/one_column.png"
inputs = processor(image, return_tensors="pt", format=True, crop_to_patches=True, max_patches=3, device=device).to(model.device)

generate_ids = model.generate(
    **inputs,
    do_sample=False,
    tokenizer=processor.tokenizer,
    stop_strings="<|im_end|>",
    max_new_tokens=4096,
)

processor.decode(generate_ids[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
"on developing architectural improvements to make learnable matching methods generalize.\nMotivated by the above observations, (...)"
```

### Inference on a specific region

GOT supports interactive OCR, where the user can specify the region to be recognized by providing the coordinates or the color of the region's bounding box. Here is an example of how to process a specific region:

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", device_map="auto")
processor = AutoProcessor.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", use_fast=True)

image = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/multi_box.png"
inputs = processor(image, return_tensors="pt", color="green", device=device).to(model.device) # or box=[x1, y1, x2, y2] for coordinates (image pixels)

generate_ids = model.generate(
    **inputs,
    do_sample=False,
    tokenizer=processor.tokenizer,
    stop_strings="<|im_end|>",
    max_new_tokens=4096,
)

processor.decode(generate_ids[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
"You should keep in mind what features from the module should be used, especially \nwhen you’re planning to sell a template."
```

### Inference on general OCR data example: sheet music

Although this implementation of the model will only output plain text, the outputs can be further processed to render the desired format, with packages like `pdftex`, `mathpix`, `matplotlib`, `tikz`, `verovio` or `pyecharts`.
Here is an example of how to process sheet music:

```python
import verovio

from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", device_map="auto")
processor = AutoProcessor.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf", use_fast=True)

image = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/sheet_music.png"
inputs = processor(image, return_tensors="pt", format=True, device=device).to(model.device)

generate_ids = model.generate(
    **inputs,
    do_sample=False,
    tokenizer=processor.tokenizer,
    stop_strings="<|im_end|>",
    max_new_tokens=4096,
)

outputs = processor.decode(generate_ids[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
tk = verovio.toolkit()
tk.loadData(outputs)
tk.setOptions(
    {
        "pageWidth": 2100,
        "pageHeight": 800,
        "footer": "none",
        "barLineWidth": 0.5,
        "beamMaxSlope": 15,
        "staffLineWidth": 0.2,
        "spacingStaff": 6,
    }
)
tk.getPageCount()
svg = tk.renderToSVG()
svg = svg.replace('overflow="inherit"', 'overflow="visible"')
with open("output.svg", "w") as f:
    f.write(svg)
```

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/sheet_music.svg"
alt="drawing" width="600"/>

## GotOcr2Config[[transformers.GotOcr2Config]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **image_token_index** (`int`, *optional*, defaults to `151859`) --
  The image token index used as a placeholder for input images.
- **image_seq_length** (`int`, *optional*, defaults to `576`) --
  Sequence length of one image embedding.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a GotOcr2Model. It is used to instantiate a Got Ocr2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam-vit-huge](https://huggingface.co/facebook/sam-vit-huge)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import GotOcr2ForConditionalGeneration, GotOcr2Config

>>> # Initializing a GotOcr2 style configuration
>>> configuration = GotOcr2Config()

>>> # Initializing a model from the Qwen2-VL-7B style configuration
>>> model = GotOcr2ForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## GotOcr2VisionConfig[[transformers.GotOcr2VisionConfig]]

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
- **use_abs_pos** (`bool`, *optional*, defaults to `True`) --
  Whether to use absolute position embedding.
- **use_rel_pos** (`bool`, *optional*, defaults to `True`) --
  Whether to use relative position embedding.
- **window_size** (`int`, *optional*, defaults to 14) --
  Window size for relative position.
- **global_attn_indexes** (`list[int]`, *optional*, defaults to `[2, 5, 8, 11]`) --
  The indexes of the global attention layers.
- **mlp_dim** (`int`, *optional*, defaults to 3072) --
  The dimensionality of the MLP layer in the Transformer encoder.

This is the configuration class to store the configuration of a GotOcr2Model. It is used to instantiate a Got Ocr2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/sam-vit-huge](https://huggingface.co/facebook/sam-vit-huge)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## GotOcr2ImageProcessor[[transformers.GotOcr2ImageProcessor]]

- **crop_to_patches** (`bool`, *kwargs*, *optional*, defaults to `self.crop_to_patches`) --
  Whether to crop the image to patches. Can be overridden by the `crop_to_patches` parameter in the
  `preprocess` method.
- **min_patches** (`int`, *kwargs*, *optional*, defaults to `self.min_patches`) --
  The minimum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `min_patches` parameter in the `preprocess` method.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to `self.max_patches`) --
  The maximum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `max_patches` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a GotOcr2ImageProcessor image processor.

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

## GotOcr2ImageProcessorPil[[transformers.GotOcr2ImageProcessorPil]]

- **crop_to_patches** (`bool`, *kwargs*, *optional*, defaults to `self.crop_to_patches`) --
  Whether to crop the image to patches. Can be overridden by the `crop_to_patches` parameter in the
  `preprocess` method.
- **min_patches** (`int`, *kwargs*, *optional*, defaults to `self.min_patches`) --
  The minimum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `min_patches` parameter in the `preprocess` method.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to `self.max_patches`) --
  The maximum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is
  set to `True`. Can be overridden by the `max_patches` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a GotOcr2ImageProcessor image processor.

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

## GotOcr2Processor[[transformers.GotOcr2Processor]]

- **image_processor** (`GotOcr2ImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`TokenizersBackend`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
Constructs a GotOcr2Processor which wraps a image processor and a tokenizer into a single processor.

[GotOcr2Processor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Processor) offers all the functionalities of [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) and [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) and [~TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **format** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to request formatted output from the OCR model. When enabled, the model is instructed to return
  structured and formatted text output rather than raw OCR results.
- **crop_to_patches** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether to crop images into patches before processing. When enabled, large images are divided into
  smaller patches for more efficient OCR processing.
- **min_patches** (`int`, *kwargs*, *optional*, defaults to `1`) --
  Minimum number of patches to generate when cropping images. This ensures that even small images are
  processed with at least this many patches.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to `12`) --
  Maximum number of patches to generate when cropping images. Large images will be divided into at most
  this many patches to control computational complexity.
- **box** (`list`, *kwargs*, `tuple[float, float]`, or `tuple[float, float, float, float]`, *optional*) --
  Bounding box coordinates for OCR region of interest. Can be specified as a single box `[x1, y1, x2, y2]`
  or a list of boxes. Coordinates are normalized to the range [0, 1000] based on the image dimensions.
  If not provided, OCR is performed on the entire image.
- **color** (`str`, *kwargs*, *optional*) --
  Color filter specification for OCR. When provided, the OCR query is prefixed with the color information
  to focus on text of a specific color (e.g., "red", "blue").
- **num_image_tokens** (`int`, *kwargs*, *optional*, defaults to `256`) --
  Number of image tokens (patches) to use per image. This controls the resolution of the image representation
  passed to the model. Higher values provide more detail but increase computational cost.
- **multi_page** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether the input consists of multi-page documents. When enabled, images can be provided as nested lists
  where each inner list represents a page, and OCR is performed across all pages with appropriate handling
  of page boundaries.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:

- **input_ids** -- List of token ids to be fed to a model. Returned when `text` is not `None`.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).
- **pixel_values** -- Pixel values to be fed to a model. Returned when `images` is not `None`.

## GotOcr2Model[[transformers.GotOcr2Model]]

- **config** ([GotOcr2Config](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The GotOcr2 model which consists of a vision backbone and a language model, without a language modeling head.

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
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([GotOcr2Processor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Processor) uses
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`GotOcr2ModelOutputWithPast` or `tuple(torch.FloatTensor)`A `GotOcr2ModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GotOcr2Config](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Config)) and inputs.
The [GotOcr2Model](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([GotOcr2Processor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Processor) uses
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GotOcr2Config](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Config)) and inputs.
Obtains image last hidden states from the vision tower and apply multimodal projection.

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

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

## GotOcr2ForConditionalGeneration[[transformers.GotOcr2ForConditionalGeneration]]

- **config** ([GotOcr2Config](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The GOT_OCR2 model which consists of a vision backbone and a language model.

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
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([GotOcr2Processor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Processor) uses
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **past_key_values** (`~cache_utils.Cache`, *optional*) --
  Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values`
  returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.

  Only [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).
  If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.

  The model will output the same cache format that is fed as input.

  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't
  have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids`
  of shape `(batch_size, sequence_length)`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`GotOcr2CausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `GotOcr2CausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GotOcr2Config](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Config)) and inputs.
The [GotOcr2ForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, GotOcr2ForConditionalGeneration, TextStreamer

>>> model = GotOcr2ForConditionalGeneration.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf").to("cuda")
>>> processor = AutoProcessor.from_pretrained("stepfun-ai/GOT-OCR-2.0-hf")

>>> url = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/multi_box.png"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(image, return_tensors="pt", color="green").to("cuda")

>>> # Generate
>>> streamer = TextStreamer(processor.tokenizer, skip_prompt=True, skip_special_tokens=True)
>>> generate_ids = model.generate(
...     **inputs,
...     do_sample=False,
...     tokenizer = processor.tokenizer,
...     stop_strings='<|im_end|>',
...     streamer=streamer,
...     max_new_tokens=4096,
... )
"You should keep in mind what features from the module should be used, especially
when you're planning to sell a template."
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([GotOcr2Processor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Processor) uses
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([GotOcr2Config](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2Config)) and inputs.

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

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, GotOcr2ForConditionalGeneration

>>> model = GotOcr2ForConditionalGeneration.from_pretrained("facebook/sam-vit-huge")
>>> processor = AutoProcessor.from_pretrained("facebook/sam-vit-huge")

>>> messages = [
...     {
...         "role": "user", "content": [
...             {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Where is the cat standing?"},
...         ]
...     },
... ]

>>> inputs = processor.apply_chat_template(
...     messages,
...     tokenize=True,
...     return_dict=True,
...     return_tensors="pt",
...     add_generation_prompt=True
... )
>>> # Generate
>>> generate_ids = model.generate(**inputs)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True)[0]
```
