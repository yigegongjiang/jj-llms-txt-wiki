# Pixtral

[Pixtral](https://huggingface.co/papers/2410.07073) is a multimodal model trained to understand natural images and documents. It accepts images in their natural resolution and aspect ratio without resizing or padding due to it's 2D RoPE embeddings. In addition, Pixtral has a long 128K token context window for processing a large number of images. Pixtral couples a 400M vision encoder with a 12B Mistral Nemo decoder.

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/pixtral_architecture.webp"
alt="drawing" width="600"/>

 Pixtral architecture. Taken from the blog post. 

You can find all the original Pixtral checkpoints under the [Mistral AI](https://huggingface.co/mistralai/models?search=pixtral) organization.

> [!TIP]
> This model was contributed by [amyeroberts](https://huggingface.co/amyeroberts) and [ArthurZ](https://huggingface.co/ArthurZ).
> Click on the Pixtral models in the right sidebar for more examples of how to apply Pixtral to different vision and language tasks.

```python
import torch
from transformers import AutoProcessor, LlavaForConditionalGeneration

model_id = "mistral-community/pixtral-12b"
model = LlavaForConditionalGeneration.from_pretrained(model_id, device_map="auto")
processor = AutoProcessor.from_pretrained(model_id)

url_dog = "https://picsum.photos/id/237/200/300"
url_mountain = "https://picsum.photos/seed/picsum/200/300"

chat = [
    {
      "role": "user", "content": [
        {"type": "text", "content": "Can this animal"}, 
        {"type": "image", "url": url_dog}, 
        {"type": "text", "content": "live here?"}, 
        {"type": "image", "url" : url_mountain}
      ]
    }
]

inputs = processor.apply_chat_template(chat, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors"pt").to(model.device)
generate_ids = model.generate(**inputs, max_new_tokens=500)
output = processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to quantize the model to 4-bits.

```python
import requests
import torch
from PIL import Image

from transformers import AutoProcessor, BitsAndBytesConfig, LlavaForConditionalGeneration

model_id = "mistral-community/pixtral-12b"

quantization_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_compute_dtype=torch.bfloat16
)

model = LlavaForConditionalGeneration.from_pretrained(
    model_id,
    quantization_config=quantization_config,
    device_map="auto"
)
processor = AutoProcessor.from_pretrained(model_id)

dog_url = "https://picsum.photos/id/237/200/300"
mountain_url = "https://picsum.photos/seed/picsum/200/300"
dog_image = Image.open(requests.get(dog_url, stream=True).raw)
mountain_image = Image.open(requests.get(mountain_url, stream=True).raw)

chat = [
    {
      "role": "user", "content": [
        {"type": "text", "text": "Can this animal"},
        {"type": "image"},
        {"type": "text", "text": "live here?"},
        {"type": "image"}
      ]
    }
]

prompt = processor.apply_chat_template(chat, tokenize=False, add_generation_prompt=True)
inputs = processor(text=prompt, images=[dog_image, mountain_image], return_tensors="pt").to(model.device)

inputs["pixel_values"] = inputs["pixel_values"].to(model.dtype)
inputs = {k: v.to(model.device) for k, v in inputs.items()}

generate_ids = model.generate(**inputs, max_new_tokens=100)
output = processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)
print(output)
```

## Notes

- Pixtral uses [PixtralVisionModel](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralVisionModel) as the vision encoder and [MistralForCausalLM](/docs/transformers/v5.14.0/en/model_doc/mistral#transformers.MistralForCausalLM)  for its language decoder.
- The model internally replaces `[IMG]` token placeholders with image embeddings.

    ```py
    "<s>[INST][IMG]\nWhat are the things I should be cautious about when I visit this place?[/INST]"
    ```

    The `[IMG]` tokens are replaced with a number of `[IMG]` tokens that depend on the height and width of each image. Each row of the image is separated by a `[IMG_BREAK]` token and each image is separated by a `[IMG_END]` token. Use the `~Processor.apply_chat_template` method to handle these tokens for you.

## PixtralVisionConfig[[transformers.PixtralVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
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
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a PixtralVisionModel. It is used to instantiate a Pixtral
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [mistral-labs/pixtral-12b](https://huggingface.co/mistral-labs/pixtral-12b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PixtralVisionModel, PixtralVisionConfig

>>> # Initializing a Pixtral-12B style configuration
>>> config = PixtralVisionConfig()

>>> # Initializing a model (with randomly initialized weights) from the configuration
>>> model = PixtralVisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MistralCommonBackend[[transformers.MistralCommonBackend]]

## PixtralVisionModel[[transformers.PixtralVisionModel]]

- **config** ([PixtralVisionModel](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralVisionModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Pixtral Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "image_sizes", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor). See `PixtralImageProcessor.__call__()` for details ([PixtralProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralProcessor) uses
  [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor) for processing images).
- **image_sizes** (`torch.Tensor` of shape `(batch_size, 2)`, *optional*) --
  The sizes of the images in the batch, being (height, width) for each image.[BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`A [BaseModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PixtralVisionConfig](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralVisionConfig)) and inputs.
The [PixtralVisionModel](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## PixtralImageProcessor[[transformers.PixtralImageProcessor]]

- **patch_size** (`Union[dict[str, *kwargs*, int], int]` *optional*, defaults to `{"height" -- 16, "width": 16}`):
  Size of the patches in the model, used to calculate the output image size.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PixtralImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`Union[dict[str, *kwargs*, int], int]` *optional*, defaults to `{"height" -- 16, "width": 16}`):
  Size of the patches in the model, used to calculate the output image size.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## PixtralImageProcessorPil[[transformers.PixtralImageProcessorPil]]

- **patch_size** (`Union[dict[str, *kwargs*, int], int]` *optional*, defaults to `{"height" -- 16, "width": 16}`):
  Size of the patches in the model, used to calculate the output image size.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PixtralImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`Union[dict[str, *kwargs*, int], int]` *optional*, defaults to `{"height" -- 16, "width": 16}`):
  Size of the patches in the model, used to calculate the output image size.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## PixtralProcessor[[transformers.PixtralProcessor]]

- **image_processor** (`PixtralImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`TokenizersBackend`) --
  The tokenizer is a required input.
- **patch_size** (`int`, *optional*, defaults to 16) --
  Patch size from the vision tower.
- **spatial_merge_size** (`int`, *optional*, defaults to 1) --
  The downsampling factor for the spatial merge operation.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **image_token** (`str`, *optional*, defaults to `"[IMG]"`) --
  Special token used to denote image location.
- **image_break_token** (`str`, *optional*, defaults to `"[IMG_BREAK]"`) --
  Special token used to denote the end of a line of pixels in an image.
- **image_end_token** (`str`, *optional*, defaults to `"[IMG_END]"`) --
  Special token used to denote the end of an image input.
Constructs a PixtralProcessor which wraps a image processor and a tokenizer into a single processor.

[PixtralProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralProcessor) offers all the functionalities of [PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor) and [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~PixtralImageProcessor](/docs/transformers/v5.14.0/en/model_doc/pixtral#transformers.PixtralImageProcessor) and [~TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
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
