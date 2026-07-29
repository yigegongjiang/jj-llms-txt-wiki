# Janus

## Overview

The Janus Model was originally proposed in [Janus: Decoupling Visual Encoding for Unified Multimodal Understanding and Generation](https://huggingface.co/papers/2410.13848) by DeepSeek AI team and later refined in [Janus-Pro: Unified Multimodal Understanding and Generation with Data and Model Scaling](https://huggingface.co/papers/2501.17811). Janus is a vision-language model that can generate both image and text output, it can also take both images and text as input.

> [!NOTE]
> The model doesn't generate both images and text in an interleaved format. The user has to pass a parameter indicating whether to generate text or image.

The abstract from the original paper is the following:

*In this paper, we introduce Janus, an autoregressive framework that unifies multimodal understanding and generation. Prior research often relies on a single visual encoder for both tasks, such as Chameleon. However, due to the differing levels of information granularity required by multimodal understanding and generation, this approach can lead to suboptimal performance, particularly in multimodal understanding. To address this issue, we decouple visual encoding into separate pathways, while still leveraging a single, unified transformer architecture for processing. The decoupling not only alleviates the conflict between the visual encoder's roles in understanding and generation, but also enhances the framework's flexibility. For instance, both the multimodal understanding and generation components can independently select their most suitable encoding methods. Experiments show that Janus surpasses previous unified model and matches or exceeds the performance of task-specific models. The simplicity, high flexibility, and effectiveness of Janus make it a strong candidate for next-generation unified multimodal models.*

The abstract from the aforementioned `Janus-Pro` paper, released afterwards, is the following:

*In this work, we introduce Janus-Pro, an advanced version of the previous work Janus. Specifically, Janus-Pro incorporates (1) an optimized training strate (2) expanded training data, and (3) scaling to larger model size. With these improvements, Janus-Pro achieves significant advancements in both multimodal understanding and text-to-image instruction-following capabilities, while also enhancing the stability of text-to-image generation. We hope this work will inspire further exploration in the field. Code and models are publicly available.*

This model was contributed by [Yaswanth Gali](https://huggingface.co/yaswanthgali) and [Hugo Silva](https://huggingface.co/hugosilva664).
The original code can be found [here](https://github.com/deepseek-ai/Janus).

## Usage Example

### Single image inference

Here is the example of visual understanding with a single image.

> [!NOTE]
> Note that the model has been trained with a specific prompt format for chatting. Use `processor.apply_chat_template(my_conversation_dict)` to correctly format your prompts.

```python

from transformers import JanusForConditionalGeneration, JanusProcessor

model_id = "deepseek-community/Janus-Pro-1B"
# Prepare Input for generation.
messages = [
    {
        "role": "user",
        "content": [
            {'type':'image', 'url': 'http://images.cocodataset.org/val2017/000000039769.jpg'},
            {'type':"text", "text":"What do you see in this image?."}
        ]
    },
]

# Set generation mode to `text` to perform text generation.
processor = JanusProcessor.from_pretrained(model_id)
model = JanusForConditionalGeneration.from_pretrained(model_id,
            device_map="auto")

inputs = processor.apply_chat_template(
    messages,
    add_generation_prompt=True,
    generation_mode="text",
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
).to(model.device)

output = model.generate(**inputs, max_new_tokens=40,generation_mode='text',do_sample=True)
text = processor.decode(output[0], skip_special_tokens=True)
print(text)
```

### Multi image inference

Janus can perform inference with multiple images as input, where images can belong to the same prompt or different prompts in batched inference, where the model processes many conversations in parallel. Here is how you can do it:

```python

from transformers import JanusForConditionalGeneration, JanusProcessor

model_id = "deepseek-community/Janus-Pro-1B"

image_urls = [
    "http://images.cocodataset.org/val2017/000000039769.jpg",
    "https://www.ilankelman.org/stopsigns/australia.jpg",
    "https://huggingface.co/microsoft/kosmos-2-patch14-224/resolve/main/snowman.jpg"
]

messages = [
    [
        {
            "role": "user",
            "content": [
                {"type": "text", "text": "What’s the difference between"},
                {"type": "image", "url": image_urls[0]},
                {"type": "text", "text": " and "},
                {"type": "image", "url": image_urls[1]}
            ]
        }
    ],
    [
        {
            "role": "user",
            "content": [
                {"type": "image", "url": image_urls[2]},
                {"type": "text", "text": "What do you see in this image?"}
            ]
        }
    ]
]

# Load model and processor
processor = JanusProcessor.from_pretrained(model_id)
model = JanusForConditionalGeneration.from_pretrained(
    model_id, device_map="auto"
)

inputs = processor.apply_chat_template(
    messages,
    add_generation_prompt=True,
    generation_mode="text",
    tokenize=True,
    padding=True,
    return_dict=True,
    return_tensors="pt"
).to(model.device)

# Generate response
output = model.generate(**inputs, max_new_tokens=40, generation_mode='text', do_sample=False)
text = processor.batch_decode(output, skip_special_tokens=True)
print(text)
```

## Text to Image generation

Janus can also generate images given a prompt.

```python
from transformers import JanusForConditionalGeneration, JanusProcessor

# Set generation mode to `image` to prepare inputs for image generation..

model_id = "deepseek-community/Janus-Pro-1B"
processor = JanusProcessor.from_pretrained(model_id)
model = JanusForConditionalGeneration.from_pretrained(model_id,
            device_map="auto")

messages = [
    {
        "role": "user",
        "content": [
            {"type": "text", "text": "A dog running under the rain."},
        ],
     }
]

prompt = processor.apply_chat_template(messages, add_generation_prompt=True)
inputs = processor(text=prompt,generation_mode="image",return_tensors="pt").to(model.device)

# Set num_return_sequence parameter to generate multiple images per prompt.
model.generation_config.num_return_sequences = 2
outputs = model.generate(**inputs,
                         generation_mode="image",
                         do_sample=True,
                         use_cache=True,
                         )
# Perform post-processing on the generated token ids.
decoded_image = model.decode_image_tokens(outputs)
images = processor.postprocess(list(decoded_image.float()),return_tensors="PIL.Image.Image").to(model.device)
# Save the image
for i, image in enumerate(images['pixel_values']):
    image.save(f"result{i}.png")
```

## JanusConfig[[transformers.JanusConfig]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **vq_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  Configuration dict of the vector quantize module.
- **image_token_id** (`int`, *optional*, defaults to `100581`) --
  The image token index used as a placeholder for input images.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a JanusModel. It is used to instantiate a Janus
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-community/Janus-Pro-1B](https://huggingface.co/deepseek-community/Janus-Pro-1B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import JanusForConditionalGeneration, JanusConfig, JanusVisionConfig, JanusVQVAEConfig, LlamaConfig

>>> # Initializing a Janus vision config
>>> vision_config = JanusVisionConfig()

>>> # Initializing a Llama config
>>> text_config = LlamaConfig()

>>> # Initializing a VQ config
>>> vq_config = JanusVQVAEConfig()

>>> # Initializing a Janus Pro 1B style configuration
>>> configuration = JanusConfig(vision_config=vision_config, text_config=text_config, vq_config=vq_config)

>>> # Initializing a model from the Janus Pro 1B style configuration
>>> model = JanusForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## JanusVisionConfig[[transformers.JanusVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `384`) --
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
- **mlp_ratio** (`Union[float, int]`, *optional*, defaults to `4.0`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **attention_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **hidden_dropout_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **projection_dim** (`int`, *optional*, defaults to `2048`) --
  Dimensionality of text and vision projection layers.
- **projection_dropout** (`float`, *optional*, defaults to 0.0) --
  Dropout probability for the projection layer.
- **use_qk_norm** (`bool`, *optional*, defaults to `False`) --
  Whether to use query-key normalization in the attention.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **depth** (`int`, *optional*, defaults to `2`) --
  Number of Transformer layers in the vision encoder.
- **num_image_tokens** (`int`, *optional*, defaults to 576) --
  Number of image tokens.

This is the configuration class to store the configuration of a JanusModel. It is used to instantiate a Janus
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-community/Janus-Pro-1B](https://huggingface.co/deepseek-community/Janus-Pro-1B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## JanusVQVAEConfig[[transformers.JanusVQVAEConfig]]

- **embed_dim** (`int`, *optional*, defaults to `8`) --
  Dimensionality of the embeddings and hidden states.
- **num_embeddings** (`int`, *optional*, defaults to `16384`) --
  Number of codebook embeddings.
- **double_latent** (`bool`, *optional*, defaults to `False`) --
  Whether to use double z channels.
- **latent_channels** (`int`, *optional*, defaults to `256`) --
  Number of channels for the latent space.
- **in_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **base_channels** (`int`, *optional*, defaults to 128) --
  Base channel count.
- **channel_multiplier** (`list[int]`, *optional*, defaults to `[1, 1, 2, 2, 4]`) --
  Channel multipliers for each resolution.
- **num_res_blocks** (`int`, *optional*, defaults to 2) --
  Number of residual blocks.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **num_patches** (`int`, *optional*, defaults to 32) --
  Num of patches the input images can be divided into.
- **out_channels** (`int`, *optional*, defaults to 3) --
  Number of out channels.
- **projection_dim** (`int`, *optional*, defaults to `2048`) --
  Dimensionality of text and vision projection layers.
- **num_hidden_layers** (`int`, *optional*, defaults to `2`) --
  Number of hidden layers in the Transformer decoder.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **image_token_embed_dim** (`int`, *optional*, defaults to 2048) --
  Dimension of image embeddings. It should be same as the dimensionality of text embeddings.

This is the configuration class to store the configuration of a JanusModel. It is used to instantiate a Janus
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-community/Janus-Pro-1B](https://huggingface.co/deepseek-community/Janus-Pro-1B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## JanusProcessor[[transformers.JanusProcessor]]

- **image_processor** (`JanusImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`TokenizersBackend`) --
  The tokenizer is a required input.
- **chat_template** (`str`, *optional*) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **use_default_system_prompt** (`bool`, *optional*, defaults to `False`) --
  Use default system prompt for Text Generation.
- **num_image_tokens** (`int`, *optional*, defaults to `576`) --
  The number of placeholder image tokens needed per one image.
Constructs a JanusProcessor which wraps a image processor and a tokenizer into a single processor.

[JanusProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusProcessor) offers all the functionalities of [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor) and [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor) and [~TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **generation_mode** (`str`, *kwargs*, *optional*, defaults to `"text"`) --
  The generation mode indicating which modality to generate. Can be one of `"text"` or `"image"`. When set
  to `"text"`, the processor prepares inputs for text generation. When set to `"image"`, it prepares inputs
  for image generation by appending image start tokens to the prompt.
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

## JanusImageProcessor[[transformers.JanusImageProcessor]]

- **min_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The minimum allowed size for the resized image. Ensures that neither the height nor width
  falls below this value after resizing.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a JanusImageProcessor image processor.

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

## JanusImageProcessorPil[[transformers.JanusImageProcessorPil]]

- **min_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The minimum allowed size for the resized image. Ensures that neither the height nor width
  falls below this value after resizing.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a JanusImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **min_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The minimum allowed size for the resized image. Ensures that neither the height nor width
  falls below this value after resizing.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## JanusVisionModel[[transformers.JanusVisionModel]]

- **config** ([JanusVisionConfig](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusVisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Janus Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor). See `JanusImageProcessor.__call__()` for details ([JanusProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusProcessor) uses
  [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor) for processing images).
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([JanusConfig](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusConfig)) and inputs.
The [JanusVisionModel](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusVisionModel) forward method, overrides the `__call__` special method.

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

## JanusVQVAE[[transformers.JanusVQVAE]]

- **config** ([JanusVQVAEConfig](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusVQVAEConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The VQ-VAE model used in Janus for encoding/decoding images into discrete tokens.
This model follows the "Make-a-scene: Scene-based text-to-image generation with human priors" paper from
[ Oran Gafni, Adam Polyak, Oron Ashual, Shelly Sheynin, Devi Parikh, and Yaniv
Taigman](https://huggingface.co/papers/2203.13131).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor). See `JanusImageProcessor.__call__()` for details ([JanusProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusProcessor) uses
  [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor) for processing images).`tuple[torch.FloatTensor, torch.FloatTensor]`
The [JanusVQVAE](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusVQVAE) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

## JanusModel[[transformers.JanusModel]]

- **config** ([JanusConfig](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Janus model which consists of a siglip vision backbone, a Llama language model and a VQ model.

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
  [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor). See `JanusImageProcessor.__call__()` for details ([JanusProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusProcessor) uses
  [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor) for processing images).
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
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`JanusBaseModelOutputWithPast` or `tuple(torch.FloatTensor)`A `JanusBaseModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([JanusConfig](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusConfig)) and inputs.
The [JanusModel](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`~cache_utils.Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and optionally if
  `config.is_encoder_decoder=True` in the cross-attention blocks) that can be used (see `past_key_values`
  input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the image embeddings, `(batch_size, num_images,
  sequence_length, hidden_size)`.

  image_hidden_states of the model produced by the vision encoder, and optionally by the perceiver

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor). See `JanusImageProcessor.__call__()` for details ([JanusProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusProcessor) uses
  [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor) for processing images).[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([JanusConfig](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusConfig)) and inputs.

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

## JanusForConditionalGeneration[[transformers.JanusForConditionalGeneration]]

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor). See `JanusImageProcessor.__call__()` for details ([JanusProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusProcessor) uses
  [JanusImageProcessor](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusImageProcessor) for processing images).
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).`JanusCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `JanusCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([JanusConfig](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusConfig)) and inputs.
The [JanusForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/janus#transformers.JanusForConditionalGeneration) forward method, overrides the `__call__` special method.

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
- **image_hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of `torch.FloatTensor` (one for the output of the image embeddings, `(batch_size, num_images,
  sequence_length, hidden_size)`.

  image_hidden_states of the model produced by the vision encoder, and optionally by the perceiver

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, JanusForConditionalGeneration

>>> model = JanusForConditionalGeneration.from_pretrained("deepseek-community/Janus-Pro-1B")
>>> processor = AutoProcessor.from_pretrained("deepseek-community/Janus-Pro-1B")

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
