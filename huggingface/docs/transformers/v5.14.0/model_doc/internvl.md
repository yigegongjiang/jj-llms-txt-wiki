# InternVL

The InternVL3 family of Visual Language Models was introduced in [InternVL3: Exploring Advanced Training and Test-Time Recipes for Open-Source Multimodal Models](https://huggingface.co/papers/2504.10479).

The abstract from the paper is the following:

*We introduce InternVL3, a significant advancement in the InternVL series featuring a native multimodal pre-training paradigm. Rather than adapting a text-only large language model (LLM) into a multimodal large language model (MLLM) that supports visual inputs, InternVL3 jointly acquires multimodal and linguistic capabilities from both diverse multimodal data and pure-text corpora during a single pre-training stage. This unified training paradigm effectively addresses the complexities and alignment challenges commonly encountered in conventional post-hoc training pipelines for MLLMs. To further improve performance and scalability, InternVL3 incorporates variable visual position encoding (V2PE) to support extended multimodal contexts, employs advanced post-training techniques such as supervised fine-tuning (SFT) and mixed preference optimization (MPO), and adopts test-time scaling strategies alongside an optimized training infrastructure. Extensive empirical evaluations demonstrate that InternVL3 delivers superior performance across a wide range of multi-modal tasks. In particular, InternVL3-78B achieves a score of 72.2 on the MMMU benchmark, setting a new state-of-the-art among open-source MLLMs. Its capabilities remain highly competitive with leading proprietary models, including ChatGPT-4o, Claude 3.5 Sonnet, and Gemini 2.5 Pro, while also maintaining strong pure-language proficiency. In pursuit of open-science principles, we will publicly release both the training data and model weights to foster further research and development in next-generation MLLMs.*

 Overview of InternVL3 models architecture, which is the same as InternVL2.5. Taken from the original checkpoint. 

 Comparison of InternVL3 performance on OpenCompass against other SOTA VLLMs. Taken from the original checkpoint. 

This model was contributed by [yonigozlan](https://huggingface.co/yonigozlan).
The original code can be found [here](https://github.com/OpenGVLab/InternVL).

## Usage example

### Inference with Pipeline

Here is how you can use the `image-text-to-text` pipeline to perform inference with the `InternVL3` models in just a few lines of code:

```python
from transformers import pipeline

messages = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "image": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/bee.jpg",
            },
            {"type": "text", "text": "Describe this image."},
        ],
    },
]

pipe = pipeline("image-text-to-text", model="OpenGVLab/InternVL3-1B-hf")
outputs = pipe(text=messages, max_new_tokens=50, return_full_text=False)
outputs[0]["generated_text"]
'The image showcases a vibrant scene of nature, featuring several flowers and a bee. \n\n1. **Foreground Flowers**: \n   - The primary focus is on a large, pink cosmos flower with a prominent yellow center. The petals are soft and slightly r'
```

### Inference on a single image

This example demonstrates how to perform inference on a single image with the InternVL models using chat templates.

> [!NOTE]
> Note that the model has been trained with a specific prompt format for chatting. Use `processor.apply_chat_template(my_conversation_dict)` to correctly format your prompts.

```python

from transformers import AutoModelForImageTextToText, AutoProcessor

model_checkpoint = "OpenGVLab/InternVL3-1B-hf"
processor = AutoProcessor.from_pretrained(model_checkpoint)
model = AutoModelForImageTextToText.from_pretrained(model_checkpoint, device_map="auto")

messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": "http://images.cocodataset.org/val2017/000000039769.jpg"},
            {"type": "text", "text": "Please describe the image explicitly."},
        ],
    }
]

inputs = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(model.device)

generate_ids = model.generate(**inputs, max_new_tokens=50)
decoded_output = processor.decode(generate_ids[0, inputs["input_ids"].shape[1] :], skip_special_tokens=True)

decoded_output
'The image shows two cats lying on a pink blanket. The cat on the left is a tabby with a mix of brown, black, and white fur, and it appears to be sleeping with its head resting on the blanket. The cat on the'
```

### Text-only generation

This example shows how to generate text using the InternVL model without providing any image input.

```python

from transformers import AutoModelForImageTextToText, AutoProcessor

model_checkpoint = "OpenGVLab/InternVL3-1B-hf"
processor = AutoProcessor.from_pretrained(model_checkpoint)
model = AutoModelForImageTextToText.from_pretrained(model_checkpoint, device_map="auto")

messages = [
    {
        "role": "user",
        "content": [
            {"type": "text", "text": "Write a haiku"},
        ],
    }
]

inputs = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(model.device)

generate_ids = model.generate(**inputs, max_new_tokens=50)
decoded_output = processor.decode(generate_ids[0, inputs["input_ids"].shape[1] :], skip_special_tokens=True)

print(decoded_output)
"Whispers of dawn,\nSilent whispers of the night,\nNew day's light begins."
```

### Batched image and text inputs

InternVL models also support batched image and text inputs.

```python

from transformers import AutoModelForImageTextToText, AutoProcessor

model_checkpoint = "OpenGVLab/InternVL3-1B-hf"
processor = AutoProcessor.from_pretrained(model_checkpoint)
model = AutoModelForImageTextToText.from_pretrained(model_checkpoint, device_map="auto")

messages = [
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
                {"type": "image", "url": "https://www.ilankelman.org/stopsigns/australia.jpg"},
                {"type": "text", "text": "Describe this image"},
            ],
        },
    ],
]

inputs = processor.apply_chat_template(messages, padding=True, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(model.device)

output = model.generate(**inputs, max_new_tokens=25)

decoded_outputs = processor.batch_decode(output, skip_special_tokens=True)
decoded_outputs
["user\n\nWrite a haiku for this image\nassistant\nSilky lake,  \nWooden pier,  \nNature's peace.",
 'user\n\nDescribe this image\nassistant\nThe image shows a street scene with a traditional Chinese archway, known as a "Chinese Gate" or "Chinese Gate of']
```

### Batched multi-image input

This implementation of the InternVL models supports batched text-images inputs with different number of images for each text.

```python
from transformers import AutoProcessor, AutoModelForImageTextToText
import torch

model_checkpoint = "OpenGVLab/InternVL3-1B-hf"
processor = AutoProcessor.from_pretrained(model_checkpoint)
model = AutoModelForImageTextToText.from_pretrained(model_checkpoint, device_map="auto")

messages = [
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
                {"type": "image", "url": "https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg"},
                {"type": "image", "url": "https://thumbs.dreamstime.com/b/golden-gate-bridge-san-francisco-purple-flowers-california-echium-candicans-36805947.jpg"},
                {"type": "text", "text": "These images depict two different landmarks. Can you identify them?"},
            ],
        },
    ],
]

inputs = processor.apply_chat_template(messages, padding=True, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(model.device)

output = model.generate(**inputs, max_new_tokens=25)

decoded_outputs = processor.batch_decode(output, skip_special_tokens=True)
decoded_outputs
["user\n\nWrite a haiku for this image\nassistant\nSilky lake,  \nWooden pier,  \nNature's peace.",
 'user\n\n\nThese images depict two different landmarks. Can you identify them?\nassistant\nYes, these images depict the Statue of Liberty and the Golden Gate Bridge.']
```

### Video input

InternVL models can also handle video inputs. Here is an example of how to perform inference on a video input using chat templates.

```python
from transformers import AutoModelForImageTextToText, AutoProcessor, BitsAndBytesConfig

model_checkpoint = "OpenGVLab/InternVL3-8B-hf"
quantization_config = BitsAndBytesConfig(load_in_4bit=True)
processor = AutoProcessor.from_pretrained(model_checkpoint)
model = AutoModelForImageTextToText.from_pretrained(model_checkpoint, quantization_config=quantization_config, device_map="auto")

messages = [
    {
        "role": "user",
        "content": [
            {
                "type": "video",
                "url": "https://huggingface.co/datasets/hf-internal-testing/fixtures_videos/resolve/main/tennis.mp4",
            },
            {"type": "text", "text": "What type of shot is the man performing?"},
        ],
    }
]
inputs = processor.apply_chat_template(
    messages,
    return_tensors="pt",
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    num_frames=8,
).to(model.device)

output = model.generate(**inputs, max_new_tokens=25)

decoded_output = processor.decode(output[0, inputs["input_ids"].shape[1] :], skip_special_tokens=True)
decoded_output
'The man is performing a forehand shot.'
```

### Interleaved image and video inputs

This example showcases how to handle a batch of chat conversations with interleaved image and video inputs using chat template.

```python
from transformers import AutoProcessor, AutoModelForImageTextToText, BitsAndBytesConfig
import torch

model_checkpoint = "OpenGVLab/InternVL3-1B-hf"
processor = AutoProcessor.from_pretrained(model_checkpoint)
model = AutoModelForImageTextToText.from_pretrained(model_checkpoint, device_map="auto")

messages = [
    [
        {
            "role": "user",
            "content": [
                {"type": "image", "url": "https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg"},
                {"type": "image", "url": "https://thumbs.dreamstime.com/b/golden-gate-bridge-san-francisco-purple-flowers-california-echium-candicans-36805947.jpg"},
                {"type": "text", "text": "These images depict two different landmarks. Can you identify them?"},
            ],
        },
    ],
    [
        {
            "role": "user",
            "content": [
                {"type": "video", "url": "https://huggingface.co/datasets/hf-internal-testing/fixtures_videos/resolve/main/tennis.mp4"},
                {"type": "text", "text": "What type of shot is the man performing?"},
            ],
        },
    ],
    [
        {
            "role": "user",
            "content": [
                {"type": "image", "url": "https://llava-vl.github.io/static/images/view.jpg"},
                {"type": "text", "text": "Write a haiku for this image"},
            ],
        },
    ],
]
inputs = processor.apply_chat_template(
    messages,
    padding=True,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
).to(model.device)

outputs = model.generate(**inputs, max_new_tokens=25)

decoded_outputs = processor.batch_decode(outputs, skip_special_tokens=True)
decoded_outputs
['user\n\n\nThese images depict two different landmarks. Can you identify them?\nassistant\nThe images depict the Statue of Liberty and the Golden Gate Bridge.',
 'user\nFrame1: \nFrame2: \nFrame3: \nFrame4: \nFrame5: \nFrame6: \nFrame7: \nFrame8: \nWhat type of shot is the man performing?\nassistant\nA forehand shot',
 "user\n\nWrite a haiku for this image\nassistant\nSilky lake,  \nWooden pier,  \nNature's peace."]
```

## InternVLVisionConfig[[transformers.InternVLVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **use_qk_norm** (`bool`, *optional*, defaults to `False`) --
  Whether to use query-key normalization in the attention.
- **intermediate_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the MLP representations.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **projection_dropout** (`float`, *optional*, defaults to 0.0) --
  Dropout probability for the projection layer.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **norm_type** (`str`, *optional*, defaults to `"layer_norm"`) --
  The type of normalization to use in the encoder. Can be `"layer_norm"` or `"rms_norm"`.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **image_size** (`Union[int, list[int], tuple[int, ...]]`, *optional*, defaults to `(448, 448)`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, ...]]`, *optional*, defaults to `(14, 14)`) --
  The size (resolution) of each patch.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **use_mask_token** (`bool`, *optional*, defaults to `False`) --
  Whether to use a mask token for masked image modeling
- **use_absolute_position_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to use absolute position embeddings.
- **layer_scale_init_value** (`float`, *optional*, defaults to `0.1`) --
  Scale to use in the self-attention layers. 0.1 for base, 1e-6 for large. Set 0 to disable layer scale.
- **use_mean_pooling** (`bool`, *optional*, defaults to `True`) --
  Whether to mean pool the final hidden states of the patches instead of using the final hidden state of the
  CLS token, before applying the classification head.

This is the configuration class to store the configuration of a InternVLModel. It is used to instantiate a Internvl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [OpenGVLab/InternVL3-1B-hf](https://huggingface.co/OpenGVLab/InternVL3-1B-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import InternVLVisionConfig, InternVLVisionModel

>>> # Initializing a InternVLVisionModel OpenGVLab/InternVL3-1B-hf style configuration
>>> configuration = InternVLVisionConfig()

>>> # Initializing a model (with random weights) from the OpenGVLab/InternVL3-1B-hf configuration
>>> model = InternVLVisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## InternVLConfig[[transformers.InternVLConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **image_token_id** (`int`, *optional*, defaults to `151667`) --
  The image token index used as a placeholder for input images.
- **image_seq_length** (`int`, *optional*, defaults to `256`) --
  Sequence length of one image embedding.
- **downsample_ratio** (`float`, *optional*, defaults to 0.5) --
  Factor by which to downsample the image.
- **projector_hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The activation function used by the multimodal projector.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*, defaults to `-1`) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **vision_feature_select_strategy** (`str`, *optional*, defaults to `default`) --
  The feature selection strategy used to select the vision feature from the vision backbone.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a InternVLModel. It is used to instantiate a Internvl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [OpenGVLab/InternVL3-1B-hf](https://huggingface.co/OpenGVLab/InternVL3-1B-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import InternVLForConditionalGeneration, InternVLConfig

>>> # Initializing a InternVL style configuration
>>> configuration = InternVLConfig()

>>> # Initializing a model (with random weights) from the OpenGVLab/InternVL3-1B-hf configuration
>>> model = InternVLForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## InternVLVisionModel[[transformers.InternVLVisionModel]]

- **config** ([InternVLVisionConfig](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLVisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Internvl Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "bool_masked_pos", "val": ": typing.Optional[torch.BoolTensor] = None"}, {"name": "**kwargs", "val": ""}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([InternVLProcessor](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLProcessor) uses
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).
- **bool_masked_pos** (`torch.BoolTensor` of shape `(batch_size, num_patches)`, *optional*) --
  Boolean masked positions. Indicates which patches are masked (1) and which aren't (0).`InternVLVisionModelOutputWithPooling` or `tuple(torch.FloatTensor)`A `InternVLVisionModelOutputWithPooling` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([InternVLConfig](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLConfig)) and inputs.
The [InternVLVisionModel](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Average of the last layer hidden states of the patch tokens (excluding the *[CLS]* token) if
  *config.use_mean_pooling* is set to True. If set to False, then the final hidden state of the *[CLS]* token
  will be returned.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## InternVLModel[[transformers.InternVLModel]]

- **config** ([InternVLConfig](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The InternVL model which consists of a vision backbone and a language model, without a language modeling head.

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
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([InternVLProcessor](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLProcessor) uses
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
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **vision_feature_select_strategy** (`str`, *optional*) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`.`InternVLModelOutputWithPast` or `tuple(torch.FloatTensor)`A `InternVLModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([InternVLConfig](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLConfig)) and inputs.
The [InternVLModel](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLModel) forward method, overrides the `__call__` special method.

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

- **pixel_values** (`torch.FloatTensor]` of shape `(batch_size, channels, height, width)`) --
  The tensors corresponding to the input images.
- **vision_feature_layer** (`int` or `list[int]`) --
  Layer index or list of layer indices to extract features from.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **vision_feature_select_strategy** (`str`, *optional*) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([InternVLConfig](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLConfig)) and inputs.
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

## InternVLForConditionalGeneration[[transformers.InternVLForConditionalGeneration]]

- **config** ([InternVLConfig](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The INTERNVL model which consists of a vision backbone and a language model.

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
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([InternVLProcessor](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLProcessor) uses
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
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **vision_feature_select_strategy** (`str`, *optional*) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`.
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).
- **image_sizes** (`torch.Tensor` of shape `(batch_size, 2)`, *optional*) --
  The sizes of the images in the batch, being (height, width) for each image.`InternVLCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `InternVLCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([InternVLConfig](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLConfig)) and inputs.
The [InternVLForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLForConditionalGeneration) forward method, overrides the `__call__` special method.

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
>>> import torch
>>> from transformers import AutoProcessor, AutoModelForImageTextToText

>>> torch_device = "cuda"
>>> processor = AutoProcessor.from_pretrained("OpenGVLab/InternVL3-1B-hf")
>>> model = AutoModelForImageTextToText.from_pretrained(
...     "OpenGVLab/InternVL3-1B-hf", dtype=torch.bfloat16, device_map=torch_device
... )

>>> messages = [
...     {
...         "role": "user",
...         "content": [
...             {
...                 "type": "image",
...                 "url": "https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg",
...             },
...             {
...                 "type": "image",
...                 "url": "https://thumbs.dreamstime.com/b/golden-gate-bridge-san-francisco-purple-flowers-california-echium-candicans-36805947.jpg",
...             },
...             {"type": "text", "text": "These images depict two different landmarks. Can you identify them?"},
...         ],
...     },
... ]

>>> inputs = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(torch_device)
>>> generate_ids = model.generate(**inputs, max_new_tokens=200)
>>> print(processor.decode(generate_ids[0, inputs["input_ids"].shape[1] :], skip_special_tokens=True))
The images depict the Statue of Liberty and the Golden Gate Bridge.
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor). See `GotOcr2ImageProcessor.__call__()` for details ([InternVLProcessor](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLProcessor) uses
  [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor) for processing images).
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **vision_feature_select_strategy** (`str`, *optional*) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([InternVLConfig](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLConfig)) and inputs.

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
>>> from transformers import AutoProcessor, InternVLForConditionalGeneration

>>> model = InternVLForConditionalGeneration.from_pretrained("OpenGVLab/InternVL3-1B-hf")
>>> processor = AutoProcessor.from_pretrained("OpenGVLab/InternVL3-1B-hf")

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

## InternVLProcessor[[transformers.InternVLProcessor]]

- **image_processor** (`GotOcr2ImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`Qwen2Tokenizer`) --
  The tokenizer is a required input.
- **video_processor** (`InternVLVideoProcessor`) --
  The video processor is a required input.
- **image_seq_length** (`int`, *optional*, defaults to 256) --
  The number of image token to use per image patch. it should be set so that:
  image_seq_length = (config.image_size // config.patch_size) ** 2 * (config.scale_factor**2)
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
Constructs a InternVLProcessor which wraps a image processor, a tokenizer, and a video processor into a single processor.

[InternVLProcessor](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLProcessor) offers all the functionalities of [GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor), [Qwen2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer), and [InternVLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLVideoProcessor). See the
[~GotOcr2ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/got_ocr2#transformers.GotOcr2ImageProcessor), [~Qwen2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer), and [~InternVLVideoProcessor](/docs/transformers/v5.14.0/en/model_doc/internvl#transformers.InternVLVideoProcessor) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **videos** (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) --
  Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If
  passing in videos with pixel values between 0 and 1, set `do_rescale=False`.
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

## InternVLVideoProcessor[[transformers.InternVLVideoProcessor]]

- **metadata** (`VideoMetadata`) --
  Metadata of the video containing information about total duration, fps and total number of frames.
- **num_frames** (`int`, *optional*) --
  Maximum number of frames to sample. Defaults to `self.num_frames`.
- **fps** (`int` or `float`, *optional*) --
  Target frames to sample per second. Defaults to `self.fps`.
- **initial_shift** (`bool`, `float` or `int`, defaults to `self.initial_shift`) --
  The initial shift to apply when sampling frames. If `True`, the shift is set so that frames are sampled from the middle of the video.np.ndarrayIndices to sample video frames.

Default sampling function which uniformly samples the desired number of frames between 0 and total number of frames.
If `fps` is passed along with metadata, `fps` frames per second are sampled uniformty. Arguments `num_frames`
and `fps` are mutually exclusive.
