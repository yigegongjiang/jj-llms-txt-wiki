# Aya Vision

[Aya Vision](https://huggingface.co/papers/2505.08751) is a family of open-weight multimodal vision-language models from Cohere Labs. It is trained with a synthetic annotation framework that generates high-quality multilingual image captions, improving Aya Vision's generated responses. In addition, a cross-modal model merging technique is used to prevent the model from losing its text capabilities after adding vision capabilities. The model combines a CommandR-7B language model with a SigLIP vision encoder.

You can find all the original Aya Vision checkpoints under the [Aya Vision](https://huggingface.co/collections/CohereLabs/cohere-labs-aya-vision-67c4ccd395ca064308ee1484) collection.

> [!TIP]
> This model was contributed by [saurabhdash](https://huggingface.co/saurabhdash) and [yonigozlan](https://huggingface.co/yonigozlan).
>
> Click on the Aya Vision models in the right sidebar for more examples of how to apply Aya Vision to different image-to-text tasks.

The example below demonstrates how to generate text based on an image with [Pipeline](/docs/transformers/v5.15.1/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipe = pipeline(model="CohereLabs/aya-vision-8b", task="image-text-to-text", device_map="auto")

# Format message with the aya-vision chat template
messages = [
    {"role": "user",
     "content": [
       {"type": "image", "url": "https://media.istockphoto.com/id/458012057/photo/istanbul-turkey.jpg?s=612x612&w=0&k=20&c=qogAOVvkpfUyqLUMr_XJQyq-HkACXyYUSZbKhBlPrxo="},
        {"type": "text", "text": "Bu resimde hangi anıt gösterilmektedir?"},
    ]},
    ]
outputs = pipe(text=messages, max_new_tokens=300, return_full_text=False)

print(outputs)
```

```python
# pip install 'git+https://github.com/huggingface/transformers.git@v4.49.0-Aya Vision'
from transformers import AutoModelForImageTextToText, AutoProcessor

model_id = "CohereLabs/aya-vision-8b"

processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForImageTextToText.from_pretrained(
    model_id, device_map="auto"
)

# Format message with the aya-vision chat template
messages = [
    {"role": "user",
     "content": [
       {"type": "image", "url": "https://pbs.twimg.com/media/Fx7YvfQWYAIp6rZ?format=jpg&name=medium"},
        {"type": "text", "text": "चित्र में लिखा पाठ क्या कहता है?"},
    ]},
    ]

inputs = processor.apply_chat_template(
    messages, padding=True, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt"
).to(model.device)

gen_tokens = model.generate(
    **inputs,
    max_new_tokens=300,
    do_sample=True,
    temperature=0.3,
)

print(processor.tokenizer.decode(gen_tokens[0][inputs.input_ids.shape[1]:], skip_special_tokens=True))
```

Quantization reduces the memory footprint of large models by representing weights at lower precision. Refer to the [Quantization](../quantization/overview) overview for supported backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to only quantize the weights to 4-bits.

```python
import torch

from transformers import AutoModelForImageTextToText, AutoProcessor, BitsAndBytesConfig

bnb_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_compute_dtype=torch.bfloat16,
    bit_use_double_quant=True
)

processor = AutoProcessor.from_pretrained("CohereLabs/aya-vision-32b", use_fast=True)
model = AutoModelForImageTextToText.from_pretrained(
    "CohereLabs/aya-vision-32b",
    quantization_config=bnb_config,
    device_map="auto"
)

inputs = processor.apply_chat_template(
    [
    {"role": "user", "content": [
        {"type": "image", "url": "https://huggingface.co/roschmid/dog-races/resolve/main/images/Border_Collie.jpg"},
        {"type": "text",  "text":"Describe what you see."}
    ]}
    ],
    padding=True,
    add_generation_prompt=True,
    tokenize=True,
    return_tensors="pt"
).to(model.device)

generated = model.generate(**inputs, max_new_tokens=50)
print(processor.tokenizer.decode(generated[0], skip_special_tokens=True))
```

## Notes

- Images are represented with the `<image>` tag in the chat template.

- Use the [apply_chat_template()](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessorMixin.apply_chat_template) method to correctly format inputs.

- The example below demonstrates inference with multiple images.
  
    ```py
    import torch
    from transformers import AutoProcessor, AutoModelForImageTextToText
        
    processor = AutoProcessor.from_pretrained("CohereForAI/aya-vision-8b")
    model = AutoModelForImageTextToText.from_pretrained(
        "CohereForAI/aya-vision-8b", device_map="auto"
    )
    
    messages = [
        {
            "role": "user",
            "content": [
                {
                    "type": "image",
                    "url": "https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg",
                },
                {
                    "type": "image",
                    "url": "https://thumbs.dreamstime.com/b/golden-gate-bridge-san-francisco-purple-flowers-california-echium-candicans-36805947.jpg",
                },
                {
                    "type": "text",
                    "text": "These images depict two different landmarks. Can you identify them?",
                },
            ],
        },
    ]
    
    inputs = processor.apply_chat_template(
        messages, padding=True, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt"
    ).to(model.device)
    
    gen_tokens = model.generate(
        **inputs, 
        max_new_tokens=300, 
        do_sample=True, 
        temperature=0.3,
    )
    
    gen_text = processor.tokenizer.decode(gen_tokens[0][inputs.input_ids.shape[1]:], skip_special_tokens=True)
    print(gen_text)
    ```

- The example below demonstrates inference with batched inputs.
  
    ```py
    import torch
    from transformers import AutoProcessor, AutoModelForImageTextToText
        
    processor = AutoProcessor.from_pretrained(model_id)
    model = AutoModelForImageTextToText.from_pretrained(
        "CohereForAI/aya-vision-8b", device_map="auto"
    )
    
    batch_messages = [
        [
            {
                "role": "user",
                "content": [
                    {"type": "image", "url": "https://llava-vl.github.io/static/images/view.jpg"},
                    {"type": "text", "text": "Write a haiku for this image"},
                ],
            },
        ],
        [
            {
                "role": "user",
                "content": [
                    {
                        "type": "image",
                        "url": "https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg",
                    },
                    {
                        "type": "image",
                        "url": "https://thumbs.dreamstime.com/b/golden-gate-bridge-san-francisco-purple-flowers-california-echium-candicans-36805947.jpg",
                    },
                    {
                        "type": "text",
                        "text": "These images depict two different landmarks. Can you identify them?",
                    },
                ],
            },
        ],
    ]
    
    batch_inputs = processor.apply_chat_template(
        batch_messages, 
        padding=True, 
        add_generation_prompt=True, 
        tokenize=True, 
        return_dict=True, 
        return_tensors="pt"
    ).to(model.device)
    
    batch_outputs = model.generate(
        **batch_inputs,
        max_new_tokens=300,
        do_sample=True,
        temperature=0.3,
    )
    
    for i, output in enumerate(batch_outputs):
        response = processor.tokenizer.decode(
            output[batch_inputs.input_ids.shape[1]:], 
            skip_special_tokens=True
        )
        print(f"Response {i+1}:\n{response}\n")
    ```

## AyaVisionProcessor[[transformers.AyaVisionProcessor]]

#### transformers.AyaVisionProcessor[[transformers.AyaVisionProcessor]]

```python
transformers.AyaVisionProcessor(image_processor = None, tokenizer = None, patch_size: int = 28, img_size: int = 364, image_token = '<image>', downsample_factor: int = 1, start_of_img_token = '<|START_OF_IMG|>', end_of_img_token = '<|END_OF_IMG|>', img_patch_token = '<|IMG_PATCH|>', img_line_break_token = '<|IMG_LINE_BREAK|>', tile_token = 'TILE', tile_global_token = 'TILE_GLOBAL', chat_template = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/aya_vision/processing_aya_vision.py#L34)

**Parameters:**

image_processor (`GotOcr2ImageProcessor`) : The image processor is a required input.

tokenizer (`CohereTokenizer`) : The tokenizer is a required input.

patch_size (`int`, *optional*, defaults to 28) : The size of image patches for tokenization.

img_size (`int`, *optional*, defaults to 364) : The size of the image to be tokenized. This should correspond to the size given to the image processor.

image_token (`str`, *optional*, defaults to `"<image>"`) : The token to be used to represent an image in the text.

downsample_factor (`int`, *optional*, defaults to 1) : The factor by which to scale the patch size.

start_of_img_token (`str`, *optional*, defaults to `"<|START_OF_IMG|>"`) : The token to be used to represent the start of an image in the text.

end_of_img_token (`str`, *optional*, defaults to `"<|END_OF_IMG|>"`) : The token to be used to represent the end of an image in the text.

img_patch_token (`str`, *optional*, defaults to `"<|IMG_PATCH|>"`) : The token to be used to represent an image patch in the text.

img_line_break_token (`str`, *optional*, defaults to `"<|IMG_LINE_BREAK|>"`) : The token to be used to represent a line break in the text.

tile_token (`str`, *optional*, defaults to `"TILE"`) : The token to be used to represent an image patch in the text.

tile_global_token (`str`, *optional*, defaults to `"TILE_GLOBAL"`) : The token to be used to represent the cover image in the text.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a AyaVisionProcessor which wraps a image processor and a tokenizer into a single processor.

[AyaVisionProcessor](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionProcessor) offers all the functionalities of [GotOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) and [CohereTokenizer](/docs/transformers/v5.15.1/en/model_doc/cohere#transformers.CohereTokenizer). See the
[~GotOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) and [~CohereTokenizer](/docs/transformers/v5.15.1/en/model_doc/cohere#transformers.CohereTokenizer) for more information.

#### __call__[[transformers.AyaVisionProcessor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: str | list[str] | list[list[str]] | None = None, videos: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]], NoneType] = None, audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L651)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

videos (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) : Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If passing in videos with pixel values between 0 and 1, set `do_rescale=False`.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

## AyaVisionConfig[[transformers.AyaVisionConfig]]

#### transformers.AyaVisionConfig[[transformers.AyaVisionConfig]]

```python
transformers.AyaVisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, vision_feature_select_strategy: str = 'full', vision_feature_layer: int | list[int] = -1, downsample_factor: int = 2, adapter_layer_norm_eps: float = 1e-06, image_token_index: int = 255036, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/aya_vision/configuration_aya_vision.py#L25)

**Parameters:**

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

vision_feature_select_strategy (`str`, *optional*, defaults to `full`) : The feature selection strategy used to select the vision feature from the vision backbone.

vision_feature_layer (`Union[int, list[int]]`, *optional*, defaults to `-1`) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

downsample_factor (`int`, *optional*, defaults to 2) : The downsample factor to apply to the vision features.

adapter_layer_norm_eps (`float`, *optional*, defaults to 1e-06) : The epsilon value used for layer normalization in the adapter.

image_token_index (`int`, *optional*, defaults to `255036`) : The image token index used as a placeholder for input images.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a AyaVisionModel. It is used to instantiate a Aya Vision
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [CohereForAI/aya-vision-8b](https://huggingface.co/CohereForAI/aya-vision-8b)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## AyaVisionModel[[transformers.AyaVisionModel]]

#### transformers.AyaVisionModel[[transformers.AyaVisionModel]]

```python
transformers.AyaVisionModel(config: AyaVisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/aya_vision/modeling_aya_vision.py#L159)

**Parameters:**

config ([AyaVisionConfig](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The AyaVision model which consists of a vision backbone and a language model, without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.AyaVisionModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/aya_vision/modeling_aya_vision.py#L234)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [GotOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([AyaVisionProcessor](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionProcessor) uses [GotOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `AyaVisionModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `AyaVisionModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AyaVisionConfig](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionConfig)) and inputs.

The [AyaVisionModel](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

#### get_image_features[[transformers.AyaVisionModel.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, output_hidden_states: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/aya_vision/modeling_aya_vision.py#L168)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [GotOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([AyaVisionProcessor](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionProcessor) uses [GotOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AyaVisionConfig](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionConfig)) and inputs.

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

#### get_placeholder_mask[[transformers.AyaVisionModel.get_placeholder_mask]]

```python
get_placeholder_mask(input_ids: LongTensor, inputs_embeds: FloatTensor, image_features: FloatTensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/aya_vision/modeling_aya_vision.py#L210)

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

## AyaVisionForConditionalGeneration[[transformers.AyaVisionForConditionalGeneration]]

#### transformers.AyaVisionForConditionalGeneration[[transformers.AyaVisionForConditionalGeneration]]

```python
transformers.AyaVisionForConditionalGeneration(config: AyaVisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/aya_vision/modeling_aya_vision.py#L291)

**Parameters:**

config ([AyaVisionConfig](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The AYA_VISION model which consists of a vision backbone and a language model.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.AyaVisionForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, labels: typing.Optional[torch.LongTensor] = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, image_sizes: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/aya_vision/modeling_aya_vision.py#L318)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [GotOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([AyaVisionProcessor](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionProcessor) uses [GotOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

image_sizes (`torch.Tensor` of shape `(batch_size, 2)`, *optional*) : The sizes of the images in the batch, being (height, width) for each image.

**Returns:** `AyaVisionCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `AyaVisionCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AyaVisionConfig](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionConfig)) and inputs.

The [AyaVisionForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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
>>> from transformers import AutoProcessor, AyaVisionForConditionalGeneration
>>> import torch

>>> torch_device = "cuda:0"
>>> processor = AutoProcessor.from_pretrained("CohereForAI/aya-vision-8b", use_fast=True)
>>> model = AyaVisionForConditionalGeneration.from_pretrained("CohereForAI/aya-vision-8b", device_map=torch_device)

>>> messages = [
...     {
...         "role": "user",
...         "content": [
...             {
...                 "type": "image",
...                 "url": "https://pbs.twimg.com/media/Fx7YvfQWYAIp6rZ?format=jpg&name=medium",
...             },
...             {"type": "text", "text": "चित्र में लिखा पाठ क्या कहता है?"},
...         ],
...     }
... ]

>>> inputs = processor.apply_chat_template(
...     messages, padding=True, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt", device=torch_device
... ).to(model.device)

>>> gen_tokens = model.generate(**inputs, max_new_tokens=300, do_sample=True, temperature=0.3)
>>> processor.tokenizer.decode(gen_tokens[0][inputs.input_ids.shape[1]:], skip_special_tokens=True)
```

#### get_image_features[[transformers.AyaVisionForConditionalGeneration.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, vision_feature_layer: int | list[int] | None = None, vision_feature_select_strategy: str | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/aya_vision/modeling_aya_vision.py#L303)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [GotOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([AyaVisionProcessor](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionProcessor) uses [GotOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).

vision_feature_layer (`Union[int, list[int]]`, *optional*) : The index of the layer to select the vision feature. If multiple indices are provided, the vision feature of the corresponding indices will be concatenated to form the vision features.

vision_feature_select_strategy (`str`, *optional*) : The feature selection strategy used to select the vision feature from the vision backbone. Can be one of `"default"` or `"full"`.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AyaVisionConfig](/docs/transformers/v5.15.1/en/model_doc/aya_vision#transformers.AyaVisionConfig)) and inputs.

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
>>> from transformers import AutoProcessor, AyaVisionForConditionalGeneration

>>> model = AyaVisionForConditionalGeneration.from_pretrained("CohereForAI/aya-vision-8b")
>>> processor = AutoProcessor.from_pretrained("CohereForAI/aya-vision-8b")

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
