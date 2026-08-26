# EXAONE 4.5

## Overview

[EXAONE 4.5](https://github.com/LG-AI-EXAONE/EXAONE-4.5) model is the first open-weight vision language model developed by LG AI Research.
Integrating a dedicated visual encoder into the existing EXAONE 4.0 framework, we expand the model's capability toward multimodality.
EXAONE 4.5 features 33 billion parameters in total, including 1.2 billion parameters from the vision encoder. 
EXAONE 4.5 achieves competitive performance in general benchmark while outperforming SOTA models of similar size in document understanding and Korean contextual reasoning, inheriting powerful language capabilities from our previous language models.

EXAONE 4.5 builds on the foundation of EXAONE 4.0 with several key enhancements. The vocabulary size has been expanded to 153,600, and the context window now supports up to 256K tokens. In addition, a Multi-Token Prediction (MTP) mechanism has been introduced, further improving the model's performance.

For more details, please refer to the [technical report](https://huggingface.co/papers/2604.08644), [blog](https://www.lgresearch.ai/blog/view?seq=641) and [GitHub](https://github.com/LG-AI-EXAONE/EXAONE-4.5).

All model weights including quantized version are available at [Huggingface Collections](https://huggingface.co/collections/LGAI-EXAONE/exaone-45).

## Usage tips

> To achieve the expected performance, we recommend using the following configurations:
> - We recommend to use `temperature=1.0`, `top_p=0.95`, `presence_penalty=1.5` for general purpose.
> - We recommend to use `temperature=0.6`, `top_p=0.95`, `presence_penalty=1.5`, `top_k=20` for OCR/document-related tasks, and Korean inputs.
> - We recommend to use `temperature=1.0`, `top_p=0.95` for text-only inputs.
> - Different from EXAONE-4.0, EXAONE 4.5 uses `enable_thinking=True` as default. Thus, you need to set `enable_thinking=False` when you want to use non-reasoning mode.
> - EXAONE 4.5 prefers using `\boxed{}` format to answer the question. We recommend using this format with the corresponding format instruction for better parsing accuracy. 

For tasks that require accurate results, you can run the EXAONE 4.5 model in reasoning mode, whereas for tasks where latency matters more than accuracy, you can run the EXAONE 4.5 model in non-reasoning mode.

Here is the example code for using EXAONE 4.5 model in reasoning mode:

```python
import torch
from transformers import AutoProcessor, AutoModelForImageTextToText
from transformers.image_utils import load_image

model_id = "LGAI-EXAONE/EXAONE-4.5-33B"

processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForImageTextToText.from_pretrained(
    model_id,
    device_map="auto",
)

image_url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = load_image(image_url)

messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "image": image_url},
            {"type": "text", "text": "Describe the image."},
        ],
    }
]

text = processor.apply_chat_template(
    messages,
    tokenize=False,
    add_generation_prompt=True,
    enable_thinking=True,   # default: True
)
inputs = processor(
    text=[text],
    images=[image],
    padding=True,
    return_tensors="pt",
)
inputs = inputs.to(model.device)

generated_ids = model.generate(**inputs, max_new_tokens=64)
generated_text = processor.batch_decode(
    generated_ids[:, inputs["input_ids"].shape[-1]:],
    skip_special_tokens=True,
)[0]
print(generated_text)
```

## Exaone4_5_Config[[transformers.Exaone4_5_Config]]

#### transformers.Exaone4_5_Config[[transformers.Exaone4_5_Config]]

```python
transformers.Exaone4_5_Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, image_token_id: int = 67, video_token_id: int = 68, tie_word_embeddings: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4_5/configuration_exaone4_5.py#L61)

**Parameters:**

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

image_token_id (`int`, *optional*, defaults to `67`) : The image token index used as a placeholder for input images.

video_token_id (`int`, *optional*, defaults to `68`) : The video token index used as a placeholder for input videos.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Exaone4_5_Model. It is used to instantiate a Exaone4 5
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [LGAI-EXAONE/EXAONE-4.5-33B](https://huggingface.co/LGAI-EXAONE/EXAONE-4.5-33B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Exaone4_5_VisionConfig[[transformers.Exaone4_5_VisionConfig]]

#### transformers.Exaone4_5_VisionConfig[[transformers.Exaone4_5_VisionConfig]]

```python
transformers.Exaone4_5_VisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, depth: int = 32, hidden_size: int = 3584, hidden_act: str = 'silu', intermediate_size: int = 3420, num_heads: int = 16, in_channels: int = 3, patch_size: int | list[int] | tuple[int, int] = 14, spatial_merge_size: int = 2, temporal_patch_size: int | list[int] | tuple[int, int] = 2, window_size: int = 112, out_hidden_size: int = 3584, fullatt_block_indexes: list[int] | tuple[int, ...] = (7, 15, 23, 31), initializer_range: float = 0.02, num_key_value_heads: int = 8)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4_5/configuration_exaone4_5.py#L30)

**Parameters:**

depth (`int`, *optional*, defaults to `32`) : Number of Transformer layers in the vision encoder.

hidden_size (`int`, *optional*, defaults to `3584`) : Dimension of the hidden representations.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

intermediate_size (`int`, *optional*, defaults to `3420`) : Dimension of the MLP representations.

num_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

in_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) : The size (resolution) of each patch.

spatial_merge_size (`int`, *optional*, defaults to `2`) : The size of the spatial merge window used to reduce the number of visual tokens by merging neighboring patches.

temporal_patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `2`) : Temporal patch size used in the 3D patch embedding for video inputs.

window_size (`int`, *optional*, defaults to 11) : Size of windows.

out_hidden_size (`int`, *optional*, defaults to 3584) : The output hidden size of the vision model.

fullatt_block_indexes (`int`, *optional*, defaults to `[7, 15, 23, 31]`) : Indices of layers with full attention

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

num_key_value_heads (`int`, *optional*, defaults to `8`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

This is the configuration class to store the configuration of a Exaone4_5_Model. It is used to instantiate a Exaone4 5
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [LGAI-EXAONE/EXAONE-4.5-33B](https://huggingface.co/LGAI-EXAONE/EXAONE-4.5-33B)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Exaone4_5_Processor[[transformers.Exaone4_5_Processor]]

#### transformers.Exaone4_5_Processor[[transformers.Exaone4_5_Processor]]

```python
transformers.Exaone4_5_Processor(image_processor = None, tokenizer = None, video_processor = None, chat_template = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4_5/processing_exaone4_5.py#L36)

**Parameters:**

image_processor (`Qwen2VLImageProcessor`) : The image processor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

video_processor (`Qwen2VLVideoProcessor`) : The video processor is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a Exaone4_5_Processor which wraps a image processor, a tokenizer, and a video processor into a single processor.

[Exaone4_5_Processor](/docs/transformers/v5.15.1/en/model_doc/exaone4_5#transformers.Exaone4_5_Processor) offers all the functionalities of [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor), `tokenizer_class`, and [Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor). See the
[~Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor), `~tokenizer_class`, and [~Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor) for more information.

#### post_process_image_text_to_text[[transformers.Exaone4_5_Processor.post_process_image_text_to_text]]

```python
post_process_image_text_to_text(generated_outputs, skip_special_tokens = True, clean_up_tokenization_spaces = False, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4_5/processing_exaone4_5.py#L102)

**Parameters:**

generated_outputs (`torch.Tensor` or `np.ndarray`) : The output of the model `generate` function. The output is expected to be a tensor of shape `(batch_size, sequence_length)` or `(sequence_length,)`.

skip_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to remove special tokens in the output. Argument passed to the tokenizer's `batch_decode` method.

clean_up_tokenization_spaces (`bool`, *optional*, defaults to `False`) : Whether or not to clean up the tokenization spaces. Argument passed to the tokenizer's `batch_decode` method.

- ****kwargs** : Additional arguments to be passed to the tokenizer's `batch_decode method`.

**Returns:** `list[str]`

The decoded text.

Post-process the output of the model to decode the text.

## Exaone4_5_VisionModel[[transformers.Exaone4_5_VisionModel]]

#### transformers.Exaone4_5_VisionModel[[transformers.Exaone4_5_VisionModel]]

```python
transformers.Exaone4_5_VisionModel(config: Exaone4_5_VisionConfig, *inputs, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4_5/modeling_exaone4_5.py#L485)

#### forward[[transformers.Exaone4_5_VisionModel.forward]]

```python
forward(hidden_states: Tensor, grid_thw: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4_5/modeling_exaone4_5.py#L544)

**Parameters:**

hidden_states (`torch.Tensor` of shape `(seq_len, hidden_size)`) : The final hidden states of the model.

grid_thw (`torch.Tensor` of shape `(num_images_or_videos, 3)`) : The temporal, height and width of feature shape of each image in LLM.

**Returns:** `torch.Tensor`

hidden_states.

## Exaone4_5_Model[[transformers.Exaone4_5_Model]]

#### transformers.Exaone4_5_Model[[transformers.Exaone4_5_Model]]

```python
transformers.Exaone4_5_Model(config: Exaone4_5_Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4_5/modeling_exaone4_5.py#L613)

**Parameters:**

config ([Exaone4_5_Config](/docs/transformers/v5.15.1/en/model_doc/exaone4_5#transformers.Exaone4_5_Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Exaone4 5 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Exaone4_5_Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, pixel_values: typing.Optional[torch.Tensor] = None, pixel_values_videos: typing.Optional[torch.FloatTensor] = None, image_grid_thw: typing.Optional[torch.LongTensor] = None, video_grid_thw: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4_5/modeling_exaone4_5.py#L702)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor). See `Qwen2VLImageProcessor.__call__()` for details ([Exaone4_5_Processor](/docs/transformers/v5.15.1/en/model_doc/exaone4_5#transformers.Exaone4_5_Processor) uses [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor) for processing images).

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor). See `Qwen2VLVideoProcessor.__call__()` for details ([Exaone4_5_Processor](/docs/transformers/v5.15.1/en/model_doc/exaone4_5#transformers.Exaone4_5_Processor) uses [Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor) for processing videos).

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

video_grid_thw (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Exaone4_5_Config](/docs/transformers/v5.15.1/en/model_doc/exaone4_5#transformers.Exaone4_5_Config)) and inputs.

The [Exaone4_5_Model](/docs/transformers/v5.15.1/en/model_doc/exaone4_5#transformers.Exaone4_5_Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

## Exaone4_5_ForConditionalGeneration[[transformers.Exaone4_5_ForConditionalGeneration]]

#### transformers.Exaone4_5_ForConditionalGeneration[[transformers.Exaone4_5_ForConditionalGeneration]]

```python
transformers.Exaone4_5_ForConditionalGeneration(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4_5/modeling_exaone4_5.py#L764)

Main EXAONE 4.5 conditional generation class.

Note: Unlike Qwen2VL, the EXAONE 4.5 vision encoder uses 2D rotary positional embeddings (2D-RoPE)
and adopts a Grouped Query Attention (GQA) structure throughout the multimodal stack.

#### forward[[transformers.Exaone4_5_ForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, pixel_values: typing.Optional[torch.Tensor] = None, pixel_values_videos: typing.Optional[torch.FloatTensor] = None, image_grid_thw: typing.Optional[torch.LongTensor] = None, video_grid_thw: typing.Optional[torch.LongTensor] = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/exaone4_5/modeling_exaone4_5.py#L809)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor). See `Qwen2VLImageProcessor.__call__()` for details ([Exaone4_5_Processor](/docs/transformers/v5.15.1/en/model_doc/exaone4_5#transformers.Exaone4_5_Processor) uses [Qwen2VLImageProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor) for processing images).

pixel_values_videos (`torch.FloatTensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) : The tensors corresponding to the input video. Pixel values for videos can be obtained using [Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor). See `Qwen2VLVideoProcessor.__call__()` for details ([Exaone4_5_Processor](/docs/transformers/v5.15.1/en/model_doc/exaone4_5#transformers.Exaone4_5_Processor) uses [Qwen2VLVideoProcessor](/docs/transformers/v5.15.1/en/model_doc/qwen2_vl#transformers.Qwen2VLVideoProcessor) for processing videos).

image_grid_thw (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) : The temporal, height and width of feature shape of each image in LLM.

video_grid_thw (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) : The temporal, height and width of feature shape of each video in LLM.

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`

A [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Exaone4_5_Config](/docs/transformers/v5.15.1/en/model_doc/exaone4_5#transformers.Exaone4_5_Config)) and inputs.

The [Exaone4_5_ForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/exaone4_5#transformers.Exaone4_5_ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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

Example:

```python
>>> from transformers import AutoProcessor, Exaone4_5_ForConditionalGeneration
>>> import torch

>>> model = Exaone4_5_ForConditionalGeneration.from_pretrained("LGAI-EXAONE/EXAONE-4.5-33B")
>>> processor = AutoProcessor.from_pretrained("LGAI-EXAONE/EXAONE-4.5-33B")

>>> messages = [
...     {
...         "role": "user",
...         "content": [
...             {"type": "image", "image": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
...             {"type": "text", "text": "Describe the image."},
...         ],
...     }
... ]
>>> inputs = processor.apply_chat_template(
...     messages, tokenize=True, add_generation_prompt=True, return_dict=True, return_tensors="pt"
... )
>>> inputs = {k: v.to(model.device) if isinstance(v, torch.Tensor) else v for k, v in inputs.items()}
>>> generated_ids = model.generate(**inputs, max_new_tokens=64)
```
