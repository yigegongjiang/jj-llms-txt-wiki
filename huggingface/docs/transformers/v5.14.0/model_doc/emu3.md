# Emu3

## Overview

The Emu3 model was proposed in [Emu3: Next-Token Prediction is All You Need](https://huggingface.co/papers/2409.18869) by Xinlong Wang, Xiaosong Zhang, Zhengxiong Luo, Quan Sun, Yufeng Cui, Jinsheng Wang, Fan Zhang, Yueze Wang, Zhen Li, Qiying Yu, Yingli Zhao, Yulong Ao, Xuebin Min, Tao Li, Boya Wu, Bo Zhao, Bowen Zhang, Liangdong Wang, Guang Liu, Zheqi He, Xi Yang, Jingjing Liu, Yonghua Lin, Tiejun Huang, Zhongyuan Wang.

Emu3 is a multimodal LLM that uses vector quantization to tokenize images into discrete tokens. Discretized image tokens are later fused with text token ids for image and text generation. The model can additionally generate images by predicting image token ids.

The abstract from the paper is the following:

*While next-token prediction is considered a promising path towards artificial general intelligence, it has struggled to excel in multimodal tasks, which are still dominated by diffusion models (e.g., Stable Diffusion) and compositional approaches (e.g., CLIP combined with LLMs). In this paper, we introduce Emu3, a new suite of state-of-the-art multimodal models trained solely with next-token prediction. By tokenizing images, text, and videos into a discrete space, we train a single transformer from scratch on a mixture of multimodal sequences. Emu3 outperforms several well-established task-specific models in both generation and perception tasks, surpassing flagship models such as SDXL and LLaVA-1.6, while eliminating the need for diffusion or compositional architectures. Emu3 is also capable of generating high-fidelity video via predicting the next token in a video sequence. We simplify complex multimodal model designs by converging on a singular focus: tokens, unlocking great potential for scaling both during training and inference. Our results demonstrate that next-token prediction is a promising path towards building general multimodal intelligence beyond language. We open-source key techniques and models to support further research in this direction.*

Tips:

- We advise users to set `processor.tokenizer.padding_side = "left"` before batched generation as it leads to more accurate results.

- Note that the model has been trained with a specific prompt format for chatting. Use `processor.apply_chat_template(my_conversation_dict)` to correctly format your prompts.

- Emu3 has two different checkpoints for image-generation and text-generation, make sure to use the correct checkpoint when loading the model. To generate an image, it is advised to use `prefix_constraints` so that the generated tokens are sampled only from possible image tokens. See more below for usage examples.

> [!TIP]
> Emu3 implementation in Transformers uses a special image token to indicate where to merge image embeddings. The special image token isn't new and uses one of the reserved tokens: `<|extra_0|>`. You have to add `<image>` to your prompt in the place where the image should be embedded for correct generation.

This model was contributed by [RaushanTurganbay](https://huggingface.co/RaushanTurganbay).
The original code can be found [here](https://github.com/baaivision/Emu3).

## Usage example

### Text generation inference

Here's how to load the model and perform inference in half-precision (`torch.bfloat16`) to generate textual output from text or text and image inputs:

```python
import requests
from PIL import Image

from transformers import Emu3ForConditionalGeneration, Emu3Processor

processor = Emu3Processor.from_pretrained("BAAI/Emu3-Chat-hf")
model = Emu3ForConditionalGeneration.from_pretrained("BAAI/Emu3-Chat-hf", device_map="auto")

# prepare image and text prompt
url = 'http://images.cocodataset.org/val2017/000000039769.jpg'
image = Image.open(requests.get(url, stream=True).raw)
prompt = "What do you see in this image?<image>"

inputs = processor(images=image, text=prompt, return_tensors="pt").to(model.device)

# autoregressively complete prompt
output = model.generate(**inputs, max_new_tokens=50)
print(processor.decode(output[0], skip_special_tokens=True))
```

### Image generation inference

Emu3 can also generate images from textual input. Here is how you can do it:

```python
processor = Emu3Processor.from_pretrained("BAAI/Emu3-Gen-hf")
model = Emu3ForConditionalGeneration.from_pretrained("BAAI/Emu3-Gen-hf", device_map="auto", attn_implementation="flash_attention_2")

inputs = processor(
    text=["a portrait of young girl. masterpiece, film grained, best quality.", "a dog running under the rain"],
    padding=True,
    return_tensors="pt",
    return_for_image_generation=True,
)
inputs = inputs.to(device=model.device)

neg_prompt = "lowres, bad anatomy, bad hands, text, error, missing fingers, extra digit, fewer digits, cropped, worst quality, low quality, normal quality, jpeg artifacts, signature, watermark, username, blurry."
neg_inputs = processor(text=[neg_prompt] * 2, return_tensors="pt").to(device=model.device)

image_sizes = inputs.pop("image_sizes")
HEIGHT, WIDTH = image_sizes[0]
VISUAL_TOKENS = model.vocabulary_mapping.image_tokens

def prefix_allowed_tokens_fn(batch_id, input_ids):
    height, width = HEIGHT, WIDTH
    visual_tokens = VISUAL_TOKENS
    image_wrapper_token_id = torch.tensor([processor.tokenizer.image_wrapper_token_id], device=model.device)
    eoi_token_id = torch.tensor([processor.tokenizer.eoi_token_id], device=model.device)
    eos_token_id = torch.tensor([processor.tokenizer.eos_token_id], device=model.device)
    pad_token_id = torch.tensor([processor.tokenizer.pad_token_id], device=model.device)
    eof_token_id = torch.tensor([processor.tokenizer.eof_token_id], device=model.device)
    eol_token_id = processor.tokenizer.encode("<|extra_200|>", return_tensors="pt").to(model.device)[0]

    position = torch.nonzero(input_ids == image_wrapper_token_id, as_tuple=True)[0][0]
    offset = input_ids.shape[0] - position
    if offset % (width + 1) == 0:
        return (eol_token_id, )
    elif offset == (width + 1) * height + 1:
        return (eof_token_id, )
    elif offset == (width + 1) * height + 2:
        return (eoi_token_id, )
    elif offset == (width + 1) * height + 3:
        return (eos_token_id, )
    elif offset > (width + 1) * height + 3:
        return (pad_token_id, )
    else:
        return visual_tokens

out = model.generate(
    **inputs,
    max_new_tokens=50_000, # make sure to have enough tokens for one image
    prefix_allowed_tokens_fn=prefix_allowed_tokens_fn,
    return_dict_in_generate=True,
    negative_prompt_ids=neg_inputs.input_ids, # indicate for Classifier-Free Guidance
    negative_prompt_attention_mask=neg_inputs.attention_mask,
)

image = model.decode_image_tokens(out.sequences[:, inputs.input_ids.shape[1]: ], height=HEIGHT, width=WIDTH)
images = processor.postprocess(list(image.float()), return_tensors="PIL.Image.Image").to(model.device) # internally we convert to np but it's not supported in bf16 precision
for i, image in enumerate(images['pixel_values']):
    image.save(f"result{i}.png")

```

## Emu3Config[[transformers.Emu3Config]]

- **vq_config** (`Union[dict, ~models.emu3.configuration_emu3.Emu3VQVAEConfig]`, *optional*) --
  Configuration dict of the vector quantize module.
- **text_config** (`Union[dict, ~models.emu3.configuration_emu3.Emu3TextConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vocabulary_map** (`dict`, *optional*) --
  A dictionary containing the vocabulary map from the tokenizer. Used to obtain tokens from the image inputs.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Emu3Model. It is used to instantiate a Emu3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Emu3-community/Emu3-Chat-hf](https://huggingface.co/Emu3-community/Emu3-Chat-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Emu3VQVAEConfig[[transformers.Emu3VQVAEConfig]]

- **codebook_size** (`int`, *optional*, defaults to `32768`) --
  The number of parallel codebooks used by the model.
- **embed_dim** (`int`, *optional*, defaults to 4) --
  Dimension of the quantized vector in codebook.
- **latent_channels** (`int`, *optional*, defaults to `4`) --
  Number of channels for the latent space.
- **double_latent** (`bool`, *optional*, defaults to `False`) --
  Whether to use double z channels.
- **in_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **out_channels** (`int`, *optional*, defaults to 3) --
  Output channel of decoder.
- **temporal_downsample_factor** (`int`, *optional*, defaults to 4) --
  Temporal downsample factor.
- **base_channels** (`int`, *optional*, defaults to 256) --
  Basic channel number of the intermediate blocks.
- **channel_multiplier** (`list[int]`, *optional*, defaults to `[1, 2, 2, 4]`) --
  Channel scaling factor of the intermediate blocks.
- **num_res_blocks** (`int`, *optional*, defaults to 2) --
  Residual block number in each stage.
- **attn_resolutions** (`list[int]`, *optional*, defaults to `[3]`) --
  Stage indices to apply attention.
- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **num_attention_heads** (`int`, *optional*, defaults to `1`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.

This is the configuration class to store the configuration of a Emu3Model. It is used to instantiate a Emu3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Emu3-community/Emu3-Chat-hf](https://huggingface.co/Emu3-community/Emu3-Chat-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import Emu3VQVAE, Emu3VQVAEConfig

>>> # Initializing a video VQ model of Emu3 configuration
>>> configuration = Emu3VQVAEConfig()

>>> # Initializing a model from the Emu3 VQ model style configuration
>>> model = Emu3VQVAE(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Emu3TextConfig[[transformers.Emu3TextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `184622`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `14336`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `32`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `8`) --
  This is the number of key_value heads that should be used to implement Grouped Query Attention. If
  `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if
  `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When
  converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed
  by meanpooling all the original heads within that group. For more details, check out [this
  paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to
  `num_attention_heads`.
- **hidden_act** (`str`, *optional*, defaults to `silu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **max_position_embeddings** (`int`, *optional*, defaults to `9216`) --
  The maximum sequence length that this model might ever be used with.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **pad_token_id** (`int`, *optional*, defaults to `151643`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `151849`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `151850`) --
  Token id used for end-of-stream in the vocabulary.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Emu3Model. It is used to instantiate a Emu3
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Emu3-community/Emu3-Chat-hf](https://huggingface.co/Emu3-community/Emu3-Chat-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Emu3Model, Emu3Config

>>> # Initializing a Emu3-community/Emu3-Chat-hf style configuration
>>> configuration = Emu3Config()

>>> # Initializing a model from the Emu3-community/Emu3-Chat-hf style configuration
>>> model = Emu3Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Emu3Processor[[transformers.Emu3Processor]]

- **image_processor** (`Emu3ImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`GPT2Tokenizer`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
Constructs a Emu3Processor which wraps a image processor and a tokenizer into a single processor.

[Emu3Processor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3Processor) offers all the functionalities of [Emu3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ImageProcessor) and [GPT2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Tokenizer). See the
[~Emu3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ImageProcessor) and [~GPT2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/gpt2#transformers.GPT2Tokenizer) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **return_for_image_generation** (`bool`, *kwargs*, *optional*, defaults to `False`) --
  Whether the processed text is intended for image generation tasks. When `True`, the processor prepares
  inputs for image generation by appending image start tokens and size information to the prompt, and
  images should not be provided. When `False`, the processor prepares inputs for text generation from
  images and text, requiring both inputs to be provided.
- **ratio** (`str`, *kwargs*, *optional*, defaults to `"1 --1"`):
  The ratio of the image to resize the image.
- **image_area** (`int`, *kwargs*, *optional*, defaults to `518400`) --
  The area of the image to resize the image.
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

## Emu3ImageProcessor[[transformers.Emu3ImageProcessor]]

"}, {"name": "do_rescale", "val": ": bool = True"}, {"name": "rescale_factor", "val": ": int | float = 0.00392156862745098"}, {"name": "do_normalize", "val": ": bool = True"}, {"name": "image_mean", "val": ": float | list[float] | None = None"}, {"name": "image_std", "val": ": float | list[float] | None = None"}, {"name": "do_convert_rgb", "val": ": bool = True"}, {"name": "do_pad", "val": ": bool = True"}, {"name": "min_pixels", "val": ": int = 262144"}, {"name": "max_pixels", "val": ": int = 1048576"}, {"name": "spatial_factor", "val": ": int = 8"}, {"name": "**kwargs", "val": ""}]}>
- **do_resize** (`bool`, *optional*, defaults to `True`) --
  Whether to resize the image's (height, width) dimensions.
- **resample** (`PILImageResampling`, *optional*, defaults to `Resampling.BICUBIC`) --
  Resampling filter to use when resizing the image.
- **do_rescale** (`bool`, *optional*, defaults to `True`) --
  Whether to rescale the image by the specified scale `rescale_factor`.
- **rescale_factor** (`int` or `float`, *optional*, defaults to `1/255`) --
  Scale factor to use if rescaling the image.
- **do_normalize** (`bool`, *optional*, defaults to `True`) --
  Whether to normalize the image.
- **image_mean** (`float` or `list[float]`, *optional*, defaults to `[0.48145466, 0.4578275, 0.40821073]`) --
  Mean to use if normalizing the image. This is a float or list of floats for each channel in the image.
- **image_std** (`float` or `list[float]`, *optional*, defaults to `[0.26862954, 0.26130258, 0.27577711]`) --
  Standard deviation to use if normalizing the image. This is a float or list of floats for each channel in the image.
- **do_convert_rgb** (`bool`, *optional*, defaults to `True`) --
  Whether to convert the image to RGB.
- **do_pad** (`bool`, *optional*, defaults to `True`) --
  Whether to pad the image. If `True`, will pad the patch dimension of the images in the batch to the largest
  number of patches in the batch. Padding will be applied to the bottom and right with zeros.
- **min_pixels** (`int`, *optional*, defaults to `512 * 512`) --
  The min pixels of the image to resize the image.
- **max_pixels** (`int`, *optional*, defaults to `1024 * 1024`) --
  The max pixels of the image to resize the image.
- **spatial_factor** (`int`, *optional*, defaults to 8) --
  The spatial downsample factor the image will be downsampled in feature extracting phase

Constructs a Emu3 image processor that dynamically resizes images based on the original images.

"}, {"name": "input_data_format", "val": ": str | transformers.image_utils.ChannelDimension | None = None"}]}>
- **images** (`ImageInput`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **do_resize** (`bool`, *optional*, defaults to `self.do_resize`) --
  Whether to resize the image.
- **size** (`dict[str, int]`, *optional*, defaults to `self.size`) --
  Size of the image after resizing. Shortest edge of the image is resized to size["shortest_edge"], with
  the longest edge resized to keep the input aspect ratio.
- **resample** (`int`, *optional*, defaults to `self.resample`) --
  Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only
  has an effect if `do_resize` is set to `True`.
- **do_rescale** (`bool`, *optional*, defaults to `self.do_rescale`) --
  Whether to rescale the image.
- **rescale_factor** (`float`, *optional*, defaults to `self.rescale_factor`) --
  Rescale factor to rescale the image by if `do_rescale` is set to `True`.
- **do_normalize** (`bool`, *optional*, defaults to `self.do_normalize`) --
  Whether to normalize the image.
- **image_mean** (`float` or `list[float]`, *optional*, defaults to `self.image_mean`) --
  Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.
- **image_std** (`float` or `list[float]`, *optional*, defaults to `self.image_std`) --
  Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to
  `True`.
- **do_convert_rgb** (`bool`, *optional*, defaults to `self.do_convert_rgb`) --
  Whether to convert the image to RGB.
- **do_pad** (`bool`, *optional*, defaults to `True`) --
  Whether to pad the image. If `True`, will pad the patch dimension of the images in the batch to the largest
  number of patches in the batch. Padding will be applied to the bottom and right with zeros.
- **return_tensors** (`str` or `TensorType`, *optional*) --
  The type of tensors to return. Can be one of:
  - Unset: Return a list of `np.ndarray`.
  - `TensorType.PYTORCH` or `'pt'`: Return a batch of type `torch.Tensor`.
  - `TensorType.NUMPY` or `'np'`: Return a batch of type `np.ndarray`.
- **data_format** (`ChannelDimension` or `str`, *optional*, defaults to `ChannelDimension.FIRST`) --
  The channel dimension format for the output image. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format.
  - Unset: Use the channel dimension format of the input image.
- **input_data_format** (`ChannelDimension` or `str`, *optional*) --
  The channel dimension format for the input image. If unset, the channel dimension format is inferred
  from the input image. Can be one of:
  - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format.
  - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format.
  - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

## Emu3VQVAE[[transformers.Emu3VQVAE]]

- **config** ([Emu3VQVAEConfig](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3VQVAEConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The VQ-VAE model used in Emu3 for encoding/decoding images into discrete tokens.
This model follows the "Make-a-scene: Scene-based text-to-image generation with human priors" paper from
[ Oran Gafni, Adam Polyak, Oron Ashual, Shelly Sheynin, Devi Parikh, and Yaniv
Taigman](https://huggingface.co/papers/2203.13131).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.

## Emu3TextModel[[transformers.Emu3TextModel]]

- **config** ([Emu3TextConfig](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3TextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Emu3 Text Model outputting raw hidden-states without any specific head on to.

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
  `past_key_values`).[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Emu3Config](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3Config)) and inputs.
The [Emu3TextModel](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3TextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## Emu3Model[[transformers.Emu3Model]]

- **image_tokens** (`torch.LongTensor` of shape `(batch_size, num_of_tokens)`) --
  The tensors corresponding to the input images.
- **height** (`int`) --
  Height of the generated image before upsampling.
- **width** (`int`) --
  Width of the generated image before upsampling.

Decodes generated image tokens from language model to continuous pixel values
with VQGAN module via upsampling.

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Emu3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ImageProcessor). See `Emu3ImageProcessor.__call__()` for details ([Emu3Processor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3Processor) uses
  [Emu3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ImageProcessor) for processing images).
- **image_sizes** (`torch.LongTensor` of shape `(batch_size, 2)`) --
  The sizes of the images in the batch, being (height, width) for each image. Image sizes can be obtained using
  [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor). See `Emu3ImageProcessor.__call__()` for details ([]`Emu3Processor`] uses
  [Emu3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ImageProcessor) for processing images).
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
  `past_key_values`).[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Emu3Config](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3Config)) and inputs.
The [Emu3Model](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)) --
  The tensors corresponding to the input images.
- **image_sizes** (`torch.LongTensor` of shape `(batch_size, 2)`) --
  The sizes of the images in the batch, being (height, width) for each image.`Emu3VQVAEModelOutput` or `tuple(torch.FloatTensor)`A `Emu3VQVAEModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Emu3Config](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3Config)) and inputs.
Tokenizes images into discrete tokens with VQGAN module and embeds them with text embeddings layer

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
- **image_tokens** (`torch.LongTensor` of shape `(batch_size, config.vocab_size`) -- Indices of the image tokens predicted by the VQ-VAE model.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images.
- **image_sizes** (`torch.LongTensor` of shape `(batch_size, 2)`) --
  The sizes of the images in the batch, being (height, width) for each image.

Tokenizes images into discrete tokens with VQGAN module. Converts
obtained image tokens into BPE tokens and wraps with "boi" and "eoi"
special tokens.

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

## Emu3ForCausalLM[[transformers.Emu3ForCausalLM]]

- **config** ([Emu3ForCausalLM](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ForCausalLM)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Emu3 Model for causal language modeling.

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
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Emu3Config](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3Config)) and inputs.
The [Emu3ForCausalLM](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ForCausalLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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

Example:

```python
>>> from transformers import Emu3Processor, Emu3ForConditionalGeneration
>>> import torch
>>> import httpx
>>> from io import BytesIO
>>> from PIL import Image

>>> model = Emu3ForCausalLM.from_pretrained("BAAI/Emu3-Chat-hf", dtype=torch.bfloat16)
>>> processor = Emu3Processor.from_pretrained("BAAI/Emu3-Chat-hf")

>>> inputs = processor(text=["Can you write me a poem about winter."], return_tensors="pt").to(model.device)

>>> generated_ids = model.generate(**inputs, max_new_tokens=100, do_sample=False)
>>> processor.batch_decode(generated_ids, skip_special_tokens=True)[0]
```

## Emu3ForConditionalGeneration[[transformers.Emu3ForConditionalGeneration]]

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Emu3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ImageProcessor). See `Emu3ImageProcessor.__call__()` for details ([Emu3Processor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3Processor) uses
  [Emu3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ImageProcessor) for processing images).
- **image_sizes** (`torch.LongTensor` of shape `(batch_size, 2)`) --
  The sizes of the images in the batch, being (height, width) for each image. Image sizes can be obtained using
  [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor). See `Emu3ImageProcessor.__call__()` for details ([]`Emu3Processor`] uses
  [Emu3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ImageProcessor) for processing images).
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
  `past_key_values`).
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).[CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`A [CausalLMOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Emu3Config](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3Config)) and inputs.
The [Emu3ForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/emu3#transformers.Emu3ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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

Example:

```python
>>> from transformers import Emu3Processor, Emu3ForConditionalGeneration
>>> import torch
>>> import httpx
>>> from io import BytesIO
>>> from PIL import Image

>>> model = Emu3ForConditionalGeneration.from_pretrained("BAAI/Emu3-Chat-hf", dtype=torch.bfloat16)
>>> processor = Emu3Processor.from_pretrained("BAAI/Emu3-Chat-hf")

>>> conversation = [
...     {
...     "role": "system",
...     "content": [
...         {"type": "text", "text": "You are a helpful assistant."},
...         ],
...     },
...     {
...     "role": "user",
...     "content": [
...         {"type": "image"},
...         {"type": "text", "text": "Please describe the image."},
...         ],
...     },
... ]

>>> prompt = processor.apply_chat_template(conversation, add_generation_prompt=True)
>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=[image], text=[prompt], return_tensors="pt").to(model.device, torch.bfloat16)

>>> generated_ids = model.generate(**inputs, max_new_tokens=100, do_sample=False)
>>> processor.batch_decode(generated_ids, skip_special_tokens=True)[0]
```
