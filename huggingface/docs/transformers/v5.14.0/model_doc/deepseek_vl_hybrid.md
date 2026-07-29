# DeepseekVLHybrid

[Deepseek-VL-Hybrid](https://huggingface.co/papers/2403.05525) was introduced by the DeepSeek AI team. It is a vision-language model (VLM) designed to process both text and images for generating contextually relevant responses. The model leverages [LLaMA](./llama) as its text encoder, while [SigLip](./siglip) is used for encoding low-resolution images and [SAM (Segment Anything Model)](./sam) is incorporated to handle high-resolution image encoding, enhancing the model's ability to process fine-grained visual details. Deepseek-VL-Hybrid is a variant of Deepseek-VL that uses [SAM (Segment Anything Model)](./sam) to handle high-resolution image encoding.

You can find all the original Deepseek-VL-Hybrid checkpoints under the [DeepSeek-community](https://huggingface.co/deepseek-community) organization.

> [!TIP]
> Click on the Deepseek-VL-Hybrid models in the right sidebar for more examples of how to apply Deepseek-VL-Hybrid to different vision and language tasks.

The example below demonstrates how to generate text based on an image with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipe = pipeline(
    task="image-text-to-text",
    model="deepseek-community/deepseek-vl-7b-chat",
    device=0,
)

messages = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg",
            },
            { "type": "text", "text": "Describe this image."},
        ]
    }
]

pipe(text=messages, max_new_tokens=20, return_full_text=False)
```

```python
from transformers import AutoProcessor, DeepseekVLHybridForConditionalGeneration

model = DeepseekVLHybridForConditionalGeneration.from_pretrained(
    "deepseek-community/deepseek-vl-7b-chat",
    device_map="auto",
    attn_implementation="sdpa"
)

processor = AutoProcessor.from_pretrained("deepseek-community/deepseek-vl-7b-chat")

messages = [
    {
        "role":"user",
        "content":[
            {
                "type":"image",
                "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
            },
            {
                "type":"text",
                "text":"Describe this image."
            }
        ]
    }

]

inputs = processor.apply_chat_template(
    messages,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt"
).to(model.device, dtype=model.dtype)

generated_ids = model.generate(**inputs, max_new_tokens=128)
generated_ids_trimmed = [
    out_ids[len(in_ids) :] for in_ids, out_ids in zip(inputs.input_ids, generated_ids)
]
output_text = processor.batch_decode(
    generated_ids_trimmed, skip_special_tokens=True, clean_up_tokenization_spaces=False
)

print(output_text)
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [torchao](../quantization/torchao) to only quantize the weights to int4.

```python
from transformers import DeepseekVLHybridForConditionalGeneration, TorchAoConfig

quantization_config = TorchAoConfig(
    "int4_weight_only",
    group_size=128
)

model = DeepseekVLHybridForConditionalGeneration.from_pretrained(
    "deepseek-community/deepseek-vl-7b-chat",
    device_map="auto",
    quantization_config=quantization_config
)
```

### Notes

- Do inference with multiple images in a single conversation.

    ```py
    import torch
    from transformers import DeepseekVLHybridForConditionalGeneration, AutoProcessor

    model = DeepseekVLHybridForConditionalGeneration.from_pretrained(
        "deepseek-community/deepseek-vl-7b-chat",
        device_map="auto",
        attn_implementation="sdpa"
    )

    processor = AutoProcessor.from_pretrained("deepseek-community/deepseek-vl-7b-chat")

    messages = [
        [
            {
                "role": "user",
                "content": [
                    {"type": "text", "text": "What’s the difference between"},
                    {"type": "image", "url": "http://images.cocodataset.org/val2017/000000039769.jpg"},
                    {"type": "text", "text": " and "},
                    {"type": "image", "url": "https://www.ilankelman.org/stopsigns/australia.jpg"}
                ]
            }
        ],
        [
            {
                "role": "user",
                "content": [
                    {"type": "image", "url": "https://huggingface.co/microsoft/kosmos-2-patch14-224/resolve/main/snowman.jpg"},
                    {"type": "text", "text": "What do you see in this image?"}
                ]
            }
        ]
    ]

    inputs = processor.apply_chat_template(
        messages,
        add_generation_prompt=True,
        padding=True,
        truncation=True,
        tokenize=True,
        return_dict=True,
        return_tensors="pt"
    ).to(model.device, dtype=model.dtype)

    generated_ids = model.generate(**inputs, max_new_tokens=128)
    generated_ids_trimmed = [
        out_ids[len(in_ids) :] for in_ids, out_ids in zip(inputs.input_ids, generated_ids)
    ]
    output_text = processor.batch_decode(
        generated_ids_trimmed, skip_special_tokens=True, clean_up_tokenization_spaces=False
    )

    print(output_text)
    ```

## DeepseekVLHybridConfig[[transformers.DeepseekVLHybridConfig]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **image_token_id** (`int`, *optional*, defaults to `100015`) --
  The image token index used as a placeholder for input images.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **high_res_vision_config** (`Union[AutoConfig, dict]`,  *optional*, defaults to `SamVisionConfig`) --
  The config object or dictionary of the high resolution vision backbone.

This is the configuration class to store the configuration of a DeepseekVLHybridModel. It is used to instantiate a Deepseek Vl Hybrid
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-community/deepseek-vl-7b-chat](https://huggingface.co/deepseek-community/deepseek-vl-7b-chat)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import DeepseekVLHybridConfig, DeepseekVLHybridModel

>>> # Initializing a DeepseekVLHybrid deepseek-community/deepseek-vl-7b-chat style configuration
>>> configuration = DeepseekVLHybridConfig()

>>> # Initializing a model (with random weights) from the deepseek-community/deepseek-vl-7b-chat style configuration
>>> model = DeepseekVLHybridModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## DeepseekVLHybridProcessor[[transformers.DeepseekVLHybridProcessor]]

- **image_processor** (`DeepseekVLHybridImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`TokenizersBackend`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **num_image_tokens** (`int`, *optional*, defaults to 576) --
  The number of special image tokens used as placeholders for visual content in text sequences.
Constructs a DeepseekVLHybridProcessor which wraps a image processor and a tokenizer into a single processor.

[DeepseekVLHybridProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridProcessor) offers all the functionalities of [DeepseekVLHybridImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridImageProcessor) and [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~DeepseekVLHybridImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridImageProcessor) and [~TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
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

## DeepseekVLHybridImageProcessor[[transformers.DeepseekVLHybridImageProcessor]]

- **min_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The minimum allowed size for the resized image. Ensures that neither the height nor width
  falls below this value after resizing.
  high_res_size (`dict`, *optional*, defaults to `{"height": 1024, "width": 1024}`):
  Size of the high resolution output image after resizing. Can be overridden by the `high_res_size` parameter in the `preprocess`
  method.
- **high_res_resample** (`PILImageResampling`, *kwargs*, *optional*, defaults to `Resampling.BICUBIC`) --
  Resampling filter to use if resizing the image. Only has an effect if `do_resize` is set to `True`. Can be
  overridden by the `high_res_resample` parameter in the `preprocess` method.
- **high_res_image_mean** (`float`, *kwargs* or `list[float]`, *optional*, defaults to `OPENAI_CLIP_MEAN`) --
  Mean to use if normalizing the high resolution image. This is a float or list of floats the length of the number of
  channels in the image. Can be overridden by the `high_res_image_mean` parameter in the `preprocess` method.
- **high_res_image_std** (`float`, *kwargs* or `list[float]`, *optional*, defaults to `OPENAI_CLIP_STD`) --
  Standard deviation to use if normalizing the high resolution image. This is a float or list of floats the length of the
  number of channels in the image. Can be overridden by the `high_res_image_std` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a DeepseekVLHybridImageProcessor image processor.

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

## DeepseekVLHybridImageProcessorPil[[transformers.DeepseekVLHybridImageProcessorPil]]

- **min_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The minimum allowed size for the resized image. Ensures that neither the height nor width
  falls below this value after resizing.
  high_res_size (`dict`, *optional*, defaults to `{"height": 1024, "width": 1024}`):
  Size of the high resolution output image after resizing. Can be overridden by the `high_res_size` parameter in the `preprocess`
  method.
- **high_res_resample** (`PILImageResampling`, *kwargs*, *optional*, defaults to `Resampling.BICUBIC`) --
  Resampling filter to use if resizing the image. Only has an effect if `do_resize` is set to `True`. Can be
  overridden by the `high_res_resample` parameter in the `preprocess` method.
- **high_res_image_mean** (`float`, *kwargs* or `list[float]`, *optional*, defaults to `OPENAI_CLIP_MEAN`) --
  Mean to use if normalizing the high resolution image. This is a float or list of floats the length of the number of
  channels in the image. Can be overridden by the `high_res_image_mean` parameter in the `preprocess` method.
- **high_res_image_std** (`float`, *kwargs* or `list[float]`, *optional*, defaults to `OPENAI_CLIP_STD`) --
  Standard deviation to use if normalizing the high resolution image. This is a float or list of floats the length of the
  number of channels in the image. Can be overridden by the `high_res_image_std` parameter in the `preprocess` method.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a DeepseekVLHybridImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **min_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The minimum allowed size for the resized image. Ensures that neither the height nor width
  falls below this value after resizing.
  high_res_size (`dict`, *optional*, defaults to `{"height": 1024, "width": 1024}`):
  Size of the high resolution output image after resizing. Can be overridden by the `high_res_size` parameter in the `preprocess`
  method.
- **high_res_resample** (`PILImageResampling`, *kwargs*, *optional*, defaults to `Resampling.BICUBIC`) --
  Resampling filter to use if resizing the image. Only has an effect if `do_resize` is set to `True`. Can be
  overridden by the `high_res_resample` parameter in the `preprocess` method.
- **high_res_image_mean** (`float`, *kwargs* or `list[float]`, *optional*, defaults to `OPENAI_CLIP_MEAN`) --
  Mean to use if normalizing the high resolution image. This is a float or list of floats the length of the number of
  channels in the image. Can be overridden by the `high_res_image_mean` parameter in the `preprocess` method.
- **high_res_image_std** (`float`, *kwargs* or `list[float]`, *optional*, defaults to `OPENAI_CLIP_STD`) --
  Standard deviation to use if normalizing the high resolution image. This is a float or list of floats the length of the
  number of channels in the image. Can be overridden by the `high_res_image_std` parameter in the `preprocess` method.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## DeepseekVLHybridModel[[transformers.DeepseekVLHybridModel]]

- **config** ([DeepseekVLHybridModel](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridModel)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Deepseek Vl Hybrid Model outputting raw hidden-states without any specific head on top.

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
  [DeepseekVLHybridImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridImageProcessor). See `DeepseekVLHybridImageProcessor.__call__()` for details ([DeepseekVLHybridProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridProcessor) uses
  [DeepseekVLHybridImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridImageProcessor) for processing images).
- **high_res_pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size), *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor).
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).`DeepseekVLHybridBaseModelOutputWithPast` or `tuple(torch.FloatTensor)`A `DeepseekVLHybridBaseModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekVLHybridConfig](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridConfig)) and inputs.
The [DeepseekVLHybridModel](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridModel) forward method, overrides the `__call__` special method.

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
  [DeepseekVLHybridImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridImageProcessor). See `DeepseekVLHybridImageProcessor.__call__()` for details ([DeepseekVLHybridProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridProcessor) uses
  [DeepseekVLHybridImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridImageProcessor) for processing images).
- **high_res_pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size), *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor).`BaseModelOutputWithHighResVisionEncodings` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithHighResVisionEncodings` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekVLHybridConfig](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridConfig)) and inputs.

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
- **high_res_vision_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the high resolution vision model.
- **high_res_vision_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the high resolution vision model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the high resolution vision model at the output of each layer plus the optional initial embedding outputs.
- **high_res_vision_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)` from the high resolution vision model.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## DeepseekVLHybridForConditionalGeneration[[transformers.DeepseekVLHybridForConditionalGeneration]]

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [DeepseekVLHybridImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridImageProcessor). See `DeepseekVLHybridImageProcessor.__call__()` for details ([DeepseekVLHybridProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridProcessor) uses
  [DeepseekVLHybridImageProcessor](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridImageProcessor) for processing images).
- **high_res_pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size), *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [AutoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoImageProcessor).

  labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*):
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).`DeepseekVLHybridCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `DeepseekVLHybridCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekVLHybridConfig](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridConfig)) and inputs.
The [DeepseekVLHybridForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/deepseek_vl_hybrid#transformers.DeepseekVLHybridForConditionalGeneration) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoProcessor, DeepseekVLHybridForConditionalGeneration

>>> model = DeepseekVLHybridForConditionalGeneration.from_pretrained("deepseek-community/deepseek-vl-7b-chat")
>>> processor = AutoProcessor.from_pretrained("deepseek-community/deepseek-vl-7b-chat")

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
