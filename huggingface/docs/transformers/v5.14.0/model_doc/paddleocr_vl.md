# PaddleOCR-VL

## Overview

**Huggingface Hub**: [PaddleOCR-VL](https://huggingface.co/collections/PaddlePaddle/paddleocr-vl) | **Github Repo**: [PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR)

**Official Website**: [Baidu AI Studio](https://aistudio.baidu.com/paddleocr) | **arXiv**: [Technical Report](https://huggingface.co/papers/2510.14528)

**PaddleOCR-VL** is a SOTA and resource-efficient model tailored for document parsing. Its core component is PaddleOCR-VL-0.9B, a compact yet powerful vision-language model (VLM) that integrates a NaViT-style dynamic resolution visual encoder with the ERNIE-4.5-0.3B language model to enable accurate element recognition. This innovative model efficiently supports 109 languages and excels in recognizing complex elements (e.g., text, tables, formulas, and charts), while maintaining minimal resource consumption. Through comprehensive evaluations on widely used public benchmarks and in-house benchmarks, PaddleOCR-VL achieves SOTA performance in both page-level document parsing and element-level recognition. It significantly outperforms existing solutions, exhibits strong competitiveness against top-tier VLMs, and delivers fast inference speeds. These strengths make it highly suitable for practical deployment in real-world scenarios.

### **Core Features**

1. **Compact yet Powerful VLM Architecture:** We present a novel vision-language model that is specifically designed for resource-efficient inference, achieving outstanding performance in element recognition. By integrating a NaViT-style dynamic high-resolution visual encoder with the lightweight ERNIE-4.5-0.3B language model, we significantly enhance the model’s recognition capabilities and decoding efficiency. This integration maintains high accuracy while reducing computational demands, making it well-suited for efficient and practical document processing applications.

2. **SOTA Performance on Document Parsing:** PaddleOCR-VL achieves state-of-the-art performance in both page-level document parsing and element-level recognition. It significantly outperforms existing pipeline-based solutions and exhibiting strong competitiveness against leading vision-language models (VLMs) in document parsing. Moreover, it excels in recognizing complex document elements, such as text, tables, formulas, and charts, making it suitable for a wide range of challenging content types, including handwritten text and historical documents. This makes it highly versatile and suitable for a wide range of document types and scenarios.

3. **Multilingual Support:** PaddleOCR-VL Supports 109 languages, covering major global languages, including but not limited to Chinese, English, Japanese, Latin, and Korean, as well as languages with different scripts and structures, such as Russian (Cyrillic script), Arabic, Hindi (Devanagari script), and Thai. This broad language coverage substantially enhances the applicability of our system to multilingual and globalized document processing scenarios.

### **Model Architecture**

## Usage

### Usage tips

> [!IMPORTANT]
> We currently recommend using the [PaddleOCR official method for inference](https://www.paddleocr.ai/latest/en/version3.x/pipeline_usage/PaddleOCR-VL.html), as it is faster and supports page-level document parsing.
> The example code below only supports element-level recognition.

We have four types of element-level recognition:

- Text recognition, indicated by the prompt `OCR:`.
- Formula recognition, indicated by the prompt `Formula Recognition:`.
- Table recognition, indicated by the prompt `Table Recognition:`.
- Chart recognition, indicated by the prompt `Chart Recognition:`.

The following examples are all based on text recognition, with the prompt `OCR:`.

### Single input inference

The example below demonstrates how to generate text with PaddleOCRVL using [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel).

```python
from transformers import pipeline

pipe = pipeline("image-text-to-text", model="PaddlePaddle/PaddleOCR-VL", dtype="bfloat16")
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/ocr_demo2.jpg"},
            {"type": "text", "text": "OCR:"},
        ]
    }
]
result = pipe(text=messages)
print(result[0]["generated_text"])
```

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("PaddlePaddle/PaddleOCR-VL", dtype="bfloat16", device_map="auto")
processor = AutoProcessor.from_pretrained("PaddlePaddle/PaddleOCR-VL")
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/ocr_demo2.jpg"},
            {"type": "text", "text": "OCR:"},
        ]
    }
]
inputs = processor.apply_chat_template(
	messages,
	add_generation_prompt=True,
	tokenize=True,
	return_dict=True,
	return_tensors="pt",
).to(model.device)

outputs = model.generate(**inputs, max_new_tokens=100)
result = processor.decode(outputs[0][inputs["input_ids"].shape[-1]:-1])
print(result)
```

### Batched inference

PaddleOCRVL also supports batched inference. We advise users to use `padding_side="left"` when computing batched generation as it leads to more accurate results. Here is how you can do it with PaddleOCRVL using [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel):

```python
from transformers import pipeline

pipe = pipeline("image-text-to-text", model="PaddlePaddle/PaddleOCR-VL", dtype="bfloat16")
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/ocr_demo2.jpg"},
            {"type": "text", "text": "OCR:"},
        ]
    }
]
result = pipe(text=[messages, messages])
print(result[0][0]["generated_text"])
print(result[1][0]["generated_text"])
```

```python
from transformers import AutoModelForImageTextToText, AutoProcessor

model = AutoModelForImageTextToText.from_pretrained("PaddlePaddle/PaddleOCR-VL", dtype="bfloat16", device_map="auto")
processor = AutoProcessor.from_pretrained("PaddlePaddle/PaddleOCR-VL")
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/ocr_demo2.jpg"},
            {"type": "text", "text": "OCR:"},
        ]
    }
]
batch_messages = [messages, messages]
inputs = processor.apply_chat_template(
	batch_messages,
	add_generation_prompt=True,
	tokenize=True,
	return_dict=True,
	return_tensors="pt",
    padding=True,
    padding_side='left',
).to(model.device)

generated_ids = model.generate(**inputs, max_new_tokens=100)
generated_ids_trimmed = [out_ids[len(in_ids) :] for in_ids, out_ids in zip(inputs.input_ids, generated_ids)]
result = processor.batch_decode(generated_ids_trimmed, skip_special_tokens=True, clean_up_tokenization_spaces=False)
print(result)
```

### Using Flash Attention 2

Flash Attention 2 is an even faster, optimized version of the previous optimization, please refer to the [FlashAttention](https://huggingface.co/docs/transformers/perf_infer_gpu_one#flashattention).

For example:

```shell
pip install flash-attn --no-build-isolation
```

```python
from transformers import AutoModelForImageTextToText

model = AutoModelForImageTextToText.from_pretrained("PaddlePaddle/PaddleOCR-VL", dtype="bfloat16", attn_implementation="flash_attention_2", device_map="auto")
```

## PaddleOCRVLForConditionalGeneration[[transformers.PaddleOCRVLForConditionalGeneration]]

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
- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PaddleOCRVLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLImageProcessor). See `PaddleOCRVLImageProcessor.__call__()` for details ([PaddleOCRVLProcessor](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLProcessor) uses
  [PaddleOCRVLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLImageProcessor) for processing images).
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.
- **mm_token_type_ids** (`torch.IntTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens matching each modality. For example text (0), image (1), video (2).
  Multimodal token type ids can be obtained using [AutoProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoProcessor). See [ProcessorMixin.__call__()](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.

- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`PaddleOCRVLCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `PaddleOCRVLCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PaddleOCRVLConfig](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLConfig)) and inputs.
The [PaddleOCRVLForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLForConditionalGeneration) forward method, overrides the `__call__` special method.

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
- **rope_deltas** (`torch.LongTensor` of shape `(batch_size, )`, *optional*) -- The rope index difference between sequence length and multimodal rope.
  The attribute is deprecated and will be removed in v5.20, use `model.base_model.rope_deltas` instead.

Example:

```python
>>> from transformers import AutoProcessor, PaddleOCRVLForConditionalGeneration

>>> model = PaddleOCRVLForConditionalGeneration.from_pretrained("PaddlePaddle/PaddleOCR-VL", dtype="bfloat16")
>>> processor = AutoProcessor.from_pretrained("PaddlePaddle/PaddleOCR-VL")

>>> messages = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "image": "https://paddle-model-ecology.bj.bcebos.com/paddlex/imgs/demo_image/ocr_demo.jpg",
            },
            {"type": "text", "text": "OCR:"},
        ],
    }
]

>>> inputs = processor.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_dict=True,
    return_tensors="pt"
).to(model.device)

>>> # Generate
>>> generated_ids = model.generate(**inputs, max_new_tokens=1024)
>>> generated_ids_trimmed = [out_ids[len(in_ids) :] for in_ids, out_ids in zip(inputs.input_ids, generated_ids)]
>>> output_text = processor.batch_decode(generated_ids_trimmed, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
>>> print(output_text)
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images.
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PaddleOCRVLConfig](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLConfig)) and inputs.

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
>>> from transformers import AutoProcessor, PaddleOCRVLForConditionalGeneration

>>> model = PaddleOCRVLForConditionalGeneration.from_pretrained("PaddlePaddle/PaddleOCR-VL")
>>> processor = AutoProcessor.from_pretrained("PaddlePaddle/PaddleOCR-VL")

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

## PaddleOCRVLConfig[[transformers.PaddleOCRVLConfig]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **image_token_id** (`int`, *optional*, defaults to `100295`) --
  The image token index used as a placeholder for input images.
- **video_token_id** (`int`, *optional*, defaults to `100296`) --
  The video token index used as a placeholder for input videos.
- **vision_start_token_id** (`int`, *optional*, defaults to `101305`) --
  Token ID that marks the start of a visual segment in the multimodal input sequence.
- **vision_end_token_id** (`int`, *optional*, defaults to `101306`) --
  Token ID that marks the end of a visual segment in the multimodal input sequence.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a PaddleOCRVLModel. It is used to instantiate a Paddleocr Vl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PaddleOCR-VL](https://huggingface.co/PaddlePaddle/PaddleOCR-VL)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PaddleOCRVLForConditionalGeneration, PaddleOCRVLConfig

>>> # Initializing a PaddleOCRVL style configuration
>>> configuration = PaddleOCRVLConfig()

>>> # Initializing a model from the PaddleOCRVL style configuration
>>> model = PaddleOCRVLForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PaddleOCRVisionConfig[[transformers.PaddleOCRVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `1152`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `4304`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `27`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`int`, *optional*, defaults to `384`) --
  The size (resolution) of each image.
- **patch_size** (`int`, *optional*, defaults to `14`) --
  The size (resolution) of each patch.
- **hidden_act** (`str`, *optional*, defaults to `gelu_pytorch_tanh`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **spatial_merge_size** (`int`, *optional*, defaults to `2`) --
  The size of the spatial merge window used to reduce the number of visual tokens by merging neighboring patches.

This is the configuration class to store the configuration of a PaddleOCRVLModel. It is used to instantiate a Paddleocr Vl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PaddleOCR-VL](https://huggingface.co/PaddlePaddle/PaddleOCR-VL)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PaddleOCRVisionConfig, PaddleOCRVisionModel

>>> # Initializing a PaddleOCRVisionConfig with PaddlePaddle/PaddleOCR-VL style configuration
>>> configuration = PaddleOCRVisionConfig()

>>> # Initializing a PaddleOCRVisionModel (with random weights) from the PaddlePaddle/PaddleOCR-VL style configuration
>>> model = PaddleOCRVisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PaddleOCRTextConfig[[transformers.PaddleOCRTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `103424`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `18`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*, defaults to `2`) --
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
- **max_position_embeddings** (`int`, *optional*, defaults to `131072`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **use_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in any of the projections including mlp and attention for example.
- **head_dim** (`int`, *optional*, defaults to `128`) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads

This is the configuration class to store the configuration of a PaddleOCRVLModel. It is used to instantiate a Paddleocr Vl
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [PaddlePaddle/PaddleOCR-VL](https://huggingface.co/PaddlePaddle/PaddleOCR-VL)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import PaddleOCRTextModel, PaddleOCRTextConfig

>>> # Initializing a PaddleOCRText 0.3B style configuration
>>> configuration = PaddleOCRTextConfig()

>>> # Initializing a model from the 0.3B style configuration
>>> model = PaddleOCRTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## PaddleOCRTextModel[[transformers.PaddleOCRTextModel]]

- **config** ([PaddleOCRTextConfig](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Paddleocr Vl Text Model outputting raw hidden-states without any specific head on to.

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
elements depending on the configuration ([PaddleOCRVLConfig](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLConfig)) and inputs.
The [PaddleOCRTextModel](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRTextModel) forward method, overrides the `__call__` special method.

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

## PaddleOCRVisionModel[[transformers.PaddleOCRVisionModel]]

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, sequence_length, image_channels, patch_size, patch_size)`) --
  The tensors corresponding to the input images.
- **grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.

## PaddleOCRVLImageProcessor[[transformers.PaddleOCRVLImageProcessor]]

- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- **temporal_patch_size** (`int`, *kwargs*, *optional*, defaults to 1) --
  The temporal patch size of the vision encoder.
- **merge_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The merge size of the vision encoder to llm encoder.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PaddleOCRVLImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- **temporal_patch_size** (`int`, *kwargs*, *optional*, defaults to 1) --
  The temporal patch size of the vision encoder.
- **merge_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The merge size of the vision encoder to llm encoder.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## PaddleOCRVLImageProcessorPil[[transformers.PaddleOCRVLImageProcessorPil]]

- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- **temporal_patch_size** (`int`, *kwargs*, *optional*, defaults to 1) --
  The temporal patch size of the vision encoder.
- **merge_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The merge size of the vision encoder to llm encoder.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a PaddleOCRVLImageProcessor image processor.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- **temporal_patch_size** (`int`, *kwargs*, *optional*, defaults to 1) --
  The temporal patch size of the vision encoder.
- **merge_size** (`int`, *kwargs*, *optional*, defaults to 2) --
  The merge size of the vision encoder to llm encoder.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## PaddleOCRVLModel[[transformers.PaddleOCRVLModel]]

- **config** ([PaddleOCRVLConfig](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Paddleocr Vl Model outputting raw hidden-states without any specific head on top.

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
- **past_key_values** (`list[torch.FloatTensor]`, *optional*) --
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
- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [PaddleOCRVLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLImageProcessor). See `PaddleOCRVLImageProcessor.__call__()` for details ([PaddleOCRVLProcessor](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLProcessor) uses
  [PaddleOCRVLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLImageProcessor) for processing images).
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.
- **mm_token_type_ids** (`torch.IntTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens matching each modality. For example text (0), image (1), video (2).
  Multimodal token type ids can be obtained using [AutoProcessor](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoProcessor). See [ProcessorMixin.__call__()](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessorMixin.__call__) for details.`PaddleOCRVLModelOutputWithPast` or `tuple(torch.FloatTensor)`A `PaddleOCRVLModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PaddleOCRVLConfig](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLConfig)) and inputs.
The [PaddleOCRVLModel](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLModel) forward method, overrides the `__call__` special method.

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
- **rope_deltas** (`torch.LongTensor` of shape `(batch_size, )`, *optional*) -- The rope index difference between sequence length and multimodal rope.
  The attribute is deprecated and will be removed in v5.20, use `model.base_model.rope_deltas` instead.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images.
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([PaddleOCRVLConfig](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLConfig)) and inputs.

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

- **input_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you provide
  it.
- **mm_token_type_ids** (`torch.IntTensor` of shape `(batch_size, sequence_length)`) --
  Token type ids matching each modality to a different value in the input sequence, i.e. text (0), image (1), video (2).
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.
- **video_grid_thw** (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) --
  The temporal, height and width of feature shape of each video in LLM.
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.position_ids (`torch.LongTensor` of shape `(3, batch_size, sequence_length)`)
mrope_position_deltas (`torch.Tensor` of shape `(batch_size)`)

Calculate the 3D rope index based on image and video's sizes. The utility expects a `vision + text`
sequence and will error out otherwise. For pure text sequence, please rely on model's auto-inferred
position ids. In a mixed vision + text sequence, vision tokens use 3D RoPE (temporal, height, width)
while text tokens use standard 1D RoPE.

Example:
Temporal patches: 3; Height patches: 2; Width patches: 2
Each vision input results in (temporal x height × width) positions. Here: 3 x 2 × 2 = 12 positions total.

Temporal position IDs are spaced by:
`interval = tokens_per_second * temporal_patch_size / fps`

If fps = 1; tokens_per_second = 25; temporal_patch_size = 2, temporal IDs increase by 50 for each temporal patch:
`[0, 0, 0, 0, 50, 50, 50, 50, 100, 100, 100, 100]`

Height IDs repeat per row: `[0, 0, 1, 1, ...]`
Width IDs alternate per column: `[0, 1, 0, 1, ...]`
Text tokens follow standard 1D RoPE and the position IDs grow consequently with a step of `1`

- **start_position** (`int`) --
  Offset added to all computed positional indices.
- **grid_thw** (`Sequence[int]` or `torch.Tensor` of shape `(3,)`) --
  The (T, H, W) grid representing the feature layout of the current image or video after patch embedding.
- **temp_merge_size** (`int`, *optional*) --
  Factor by which the temporal dimension is reduced in the backbone. The temporal grid size is divided
  by this value. Defaults to 1.
- **spatial_merge_size** (`int`, *optional*) --
  Factor by which the spatial dimensions (H and W) are reduced in the backbone. Both H and W are divided
  by this value. Defaults to 1.
- **time_interval** (`int`, *optional*) --
  Spacing factor applied between consecutive temporal position indices.Defaults to 1.
- **device** (`str` or `torch.device`, *optional*) --
  Device on which the resulting tensor is allocated. If `None`, uses the current default device.torch.LongTensor of shape (3, sequence_length)Positional indices for temporal, height, and width dimensions,
flattened into sequence form and offset by `start_position`.

Compute 3D positional indices for vision tokens derived from a single image or video input.

The positions are generated from the input grid defined by temporal (T), height (H), and
width (W) dimensions. Temporal and spatial dimensions can be downscaled according to the
merge sizes used in the vision backbone. The resulting positions are offset by `start_position`.

## PaddleOCRVLProcessor[[transformers.PaddleOCRVLProcessor]]

- **image_processor** ([PaddleOCRVLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLImageProcessor), *optional*) --
  The image processor is a required input.
- **tokenizer** (`LLamaTokenizerFast`, *optional*) --
  The tokenizer is a required input.
- **chat_template** (`str`, *optional*) -- A Jinja template which will be used to convert lists of messages
  in a chat into a tokenizable string.

[PaddleOCRVLProcessor](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLProcessor) offers all the functionalities of [PaddleOCRVLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLImageProcessor) and `LLamaTokenizerFast`. See the
[__call__()](/docs/transformers/v5.14.0/en/model_doc/paddleocr_vl#transformers.PaddleOCRVLProcessor.__call__) and [decode()](/docs/transformers/v5.14.0/en/model_doc/donut#transformers.DonutProcessor.decode) for more information.

- **images** (`PIL.Image.Image`, `np.ndarray`, `torch.Tensor`, `List[PIL.Image.Image]`, `List[np.ndarray]`, `List[torch.Tensor]`) --
  The image or batch of images to be prepared. Each image can be a PIL image, NumPy array or PyTorch
  tensor. Both channels-first and channels-last formats are supported.
- **text** (`str`, `List[str]`, `List[List[str]]`) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set
  `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:
  - `'tf'`: Return TensorFlow `tf.constant` objects.
  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
  - `'jax'`: Return JAX `jnp.ndarray` objects.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:

- **input_ids** -- List of token ids to be fed to a model. Returned when `text` is not `None`.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).
- **pixel_values** -- Pixel values to be fed to a model. Returned when `images` is not `None`.
- **image_grid_thw** -- List of image 3D grid in LLM. Returned when `images` is not `None`.

## PaddleOCRVisionTransformer[[transformers.PaddleOCRVisionTransformer]]

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, sequence_length, patch_size * patch_size * image_channels)`) --
  The tensors corresponding to the input images.
- **attention_mask** (`torch.Tensor`, *optional*) --
  The attention_mask used in forward function shape [batch_size X sequence_length] if not None.
- **grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.
