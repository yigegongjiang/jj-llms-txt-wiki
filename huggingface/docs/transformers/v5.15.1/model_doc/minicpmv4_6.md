# MiniCPM-V

[MiniCPM-V](https://huggingface.co/papers/2509.18154) is a series of efficient multimodal large language models developed by [OpenBMB](https://github.com/OpenBMB). The MiniCPM-V 4.6 architecture uses a [SigLIP](siglip) vision encoder with a window-attention merger and a [Qwen3.5](qwen3_5) language model backbone, supporting both 4x and 16x visual downsampling modes.

This model was contributed by [OpenBMB](https://huggingface.co/openbmb).
The original code can be found [here](https://github.com/OpenBMB/MiniCPM-V).

## Usage example

### Inference with Pipeline

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

pipe = pipeline("image-text-to-text", model="openbmb/MiniCPM-V-4_6")
outputs = pipe(text=messages, max_new_tokens=50, return_full_text=False)
outputs[0]["generated_text"]
```

### Inference on a single image

> [!NOTE]
> The model has been trained with a specific prompt format for chatting. Use `processor.apply_chat_template(my_conversation_dict)` to correctly format your prompts.

```python
from transformers import AutoProcessor, AutoModelForImageTextToText

model_checkpoint = "openbmb/MiniCPM-V-4_6"
processor = AutoProcessor.from_pretrained(model_checkpoint)
model = AutoModelForImageTextToText.from_pretrained(model_checkpoint, device_map="auto")

messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
            {"type": "text", "text": "Describe this image."},
        ],
    }
]

inputs = processor.apply_chat_template(
    messages, add_generation_prompt=True, tokenize=True,
    return_dict=True, return_tensors="pt",
).to(model.device, dtype=model.dtype)

output = model.generate(**inputs, max_new_tokens=100)
decoded_output = processor.decode(output[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
print(decoded_output)
```

### Downsampling mode

MiniCPM-V 4.6 supports two visual downsampling modes:

- **16x** (default): More aggressive downsampling, fewer visual tokens, faster inference.
- **4x**: Less downsampling, more visual tokens, better for detail-rich tasks.

You can change the downsampling mode at runtime by passing `downsample_mode` via `processor_kwargs` and to `model.generate`:

```python
inputs = processor.apply_chat_template(
    messages, add_generation_prompt=True, tokenize=True,
    return_dict=True, return_tensors="pt",
    processor_kwargs={"downsample_mode": "4x"},
).to(model.device, dtype=model.dtype)

output = model.generate(**inputs, max_new_tokens=100, downsample_mode="4x")
```

### Thinking mode

The model supports a thinking mode controlled by `enable_thinking` in the chat template. When enabled, the model generates internal reasoning before providing the final answer:

```python
inputs = processor.apply_chat_template(
    messages, add_generation_prompt=True, tokenize=True,
    return_dict=True, return_tensors="pt",
    enable_thinking=True,
).to(model.device, dtype=model.dtype)

output = model.generate(**inputs, max_new_tokens=1024)
```

To disable thinking (default for evaluation):

```python
inputs = processor.apply_chat_template(
    messages, add_generation_prompt=True, tokenize=True,
    return_dict=True, return_tensors="pt",
    enable_thinking=False,
).to(model.device, dtype=model.dtype)
```

### Image processing backend

MiniCPM-V 4.6 provides two image processing backends:

- **torchvision** (default): Uses `torchvision.transforms` for image resizing.
- **pil**: Uses `PIL.Image.resize`, matching the original implementation.

To use the PIL backend:

```python
from transformers import AutoProcessor, AutoImageProcessor

processor = AutoProcessor.from_pretrained(model_checkpoint)
processor.image_processor = AutoImageProcessor.from_pretrained(model_checkpoint, backend="pil")
```

### Video inference

MiniCPM-V 4.6 supports video understanding.

```python
messages = [
    {
        "role": "user",
        "content": [
            {"type": "video", "video": "path/to/video.mp4"},
            {"type": "text", "text": "Describe what happens in this video."},
        ],
    }
]

inputs = processor.apply_chat_template(
    messages, add_generation_prompt=True, tokenize=True,
    return_dict=True, return_tensors="pt",
).to(model.device, dtype=model.dtype)

output = model.generate(**inputs, max_new_tokens=200)
decoded_output = processor.decode(output[0, inputs["input_ids"].shape[1]:], skip_special_tokens=True)
print(decoded_output)
```

If you already have the rendered prompt string, you can call `processor(text=..., videos=[...])` directly instead.

## MiniCPMV4_6Config[[transformers.MiniCPMV4_6Config]]

#### transformers.MiniCPMV4_6Config[[transformers.MiniCPMV4_6Config]]

```python
transformers.MiniCPMV4_6Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, insert_layer_id: int = 6, image_size: int = 448, drop_vision_last_layer: bool = False, image_token_id: int | None = None, video_token_id: int | None = None, tie_word_embeddings: bool = False, downsample_mode: str = '16x', merge_kernel_size: tuple[int, int] | list[int] = (2, 2), merger_times: int = 1)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/configuration_minicpmv4_6.py#L66)

**Parameters:**

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

vision_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the vision backbone.

insert_layer_id (`int`, *optional*, defaults to 6) : Vision encoder layer index after which the window-attention merger is applied.

image_size (`int`, *optional*, defaults to 448) : Base resolution for image preprocessing.

drop_vision_last_layer (`bool`, *optional*, defaults to `False`) : Whether to drop the last layer of the vision encoder.

image_token_id (`int`, *optional*) : Token id used as the image placeholder.

video_token_id (`int`, *optional*) : Token id used as the video placeholder.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

downsample_mode (`str`, *optional*, defaults to `"16x"`) : Visual token downsampling ratio. `"4x"` keeps 4× more tokens.

merge_kernel_size (`tuple[int, int]`, *optional*, defaults to `(2, 2)`) : Kernel size `(h, w)` for merging adjacent visual patches in the Merger.

merger_times (`int`, *optional*, defaults to 1) : Number of iterative merge rounds in the Merger.

This is the configuration class to store the configuration of a MiniCPMV4_6Model. It is used to instantiate a Minicpmv4 6
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [openbmb/MiniCPM-V-4.6](https://huggingface.co/openbmb/MiniCPM-V-4.6)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## MiniCPMV4_6VisionConfig[[transformers.MiniCPMV4_6VisionConfig]]

#### transformers.MiniCPMV4_6VisionConfig[[transformers.MiniCPMV4_6VisionConfig]]

```python
transformers.MiniCPMV4_6VisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, intermediate_size: int = 3072, num_hidden_layers: int = 12, num_attention_heads: int = 12, num_channels: int = 3, image_size: int | list[int] | tuple[int, int] = 224, patch_size: int | list[int] | tuple[int, int] = 16, hidden_act: str = 'gelu_pytorch_tanh', layer_norm_eps: float = 1e-06, attention_dropout: float | int = 0.0, insert_layer_id: int = 6, window_kernel_size: tuple[int, int] | list[int] = (2, 2))
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/configuration_minicpmv4_6.py#L31)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

hidden_act (`str`, *optional*, defaults to `gelu_pytorch_tanh`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

insert_layer_id (`int`, *optional*, defaults to 6) : Vision encoder layer index after which the window-attention merger is applied.

window_kernel_size (`tuple[int, int]`, *optional*, defaults to `(2, 2)`) : Window size `(h, w)` for the intermediate window-attention merger.

This is the configuration class to store the configuration of a MiniCPMV4_6Model. It is used to instantiate a Minicpmv4 6
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [openbmb/MiniCPM-V-4.6](https://huggingface.co/openbmb/MiniCPM-V-4.6)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## MiniCPMV4_6Model[[transformers.MiniCPMV4_6Model]]

#### transformers.MiniCPMV4_6Model[[transformers.MiniCPMV4_6Model]]

```python
transformers.MiniCPMV4_6Model(config: MiniCPMV4_6Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/modeling_minicpmv4_6.py#L596)

**Parameters:**

config ([MiniCPMV4_6Config](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The MiniCPMV4_6 model which consists of a vision backbone and a language model, without a language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.MiniCPMV4_6Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, target_sizes: typing.Optional[torch.IntTensor] = None, pixel_values_videos: typing.Optional[torch.FloatTensor] = None, target_sizes_videos: typing.Optional[torch.IntTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: list[torch.FloatTensor] | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, downsample_mode: str | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/modeling_minicpmv4_6.py#L663)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor`, *optional*) : Pixel value patches for images, NaViT-packed.

target_sizes (`torch.IntTensor`, *optional*) : Height and width (in patches) for each image.

pixel_values_videos (`torch.FloatTensor`, *optional*) : Pixel value patches for video frames, NaViT-packed.

target_sizes_videos (`torch.IntTensor`, *optional*) : Height and width (in patches) for each video frame.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`list[torch.FloatTensor]`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

downsample_mode (`str`, *optional*) : `"4x"` keeps 4x more visual tokens; default `"16x"` applies full merge.

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniCPMV4_6Config](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6Config)) and inputs.

The [MiniCPMV4_6Model](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6Model) forward method, overrides the `__call__` special method.

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

#### get_image_features[[transformers.MiniCPMV4_6Model.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, target_sizes: IntTensor, downsample_mode: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/modeling_minicpmv4_6.py#L605)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [MiniCPMV4_6ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6ImageProcessor). See `MiniCPMV4_6ImageProcessor.__call__()` for details ([MiniCPMV4_6Processor](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6Processor) uses [MiniCPMV4_6ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6ImageProcessor) for processing images).

target_sizes (`torch.IntTensor` of shape `(num_images, 2)`) : Height and width (in patches) of each image.

downsample_mode (`str`, *optional*) : When set to `"4x"` the intermediate `vit_merger` is skipped so that each image keeps `4×` more visual tokens. Default `"16x"` mode applies the full merge pipeline.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniCPMV4_6Config](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6Config)) and inputs.

Extract image features: vision encoder, insert merger, then MLP merger.

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

## MiniCPMV4_6ForConditionalGeneration[[transformers.MiniCPMV4_6ForConditionalGeneration]]

#### transformers.MiniCPMV4_6ForConditionalGeneration[[transformers.MiniCPMV4_6ForConditionalGeneration]]

```python
transformers.MiniCPMV4_6ForConditionalGeneration(config: MiniCPMV4_6Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/modeling_minicpmv4_6.py#L761)

#### forward[[transformers.MiniCPMV4_6ForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, target_sizes: typing.Optional[torch.IntTensor] = None, pixel_values_videos: typing.Optional[torch.FloatTensor] = None, target_sizes_videos: typing.Optional[torch.IntTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: list[torch.FloatTensor] | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, downsample_mode: str | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/modeling_minicpmv4_6.py#L771)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor`, *optional*) : Pixel value patches for images, NaViT-packed.

target_sizes (`torch.IntTensor`, *optional*) : Height and width (in patches) for each image.

pixel_values_videos (`torch.FloatTensor`, *optional*) : Pixel value patches for video frames, NaViT-packed.

target_sizes_videos (`torch.IntTensor`, *optional*) : Height and width (in patches) for each video frame.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`list[torch.FloatTensor]`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

downsample_mode (`str`, *optional*) : `"4x"` keeps 4x more visual tokens; default `"16x"` applies full merge.

**Returns:** [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or `tuple(torch.FloatTensor)`

A [CausalLMOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniCPMV4_6Config](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6Config)) and inputs.

The [MiniCPMV4_6ForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6ForConditionalGeneration) forward method, overrides the `__call__` special method.

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
>>> from PIL import Image
>>> from transformers import AutoProcessor, MiniCPMV4_6ForConditionalGeneration

>>> model = MiniCPMV4_6ForConditionalGeneration.from_pretrained("openbmb/MiniCPM-V-4.6")
>>> processor = AutoProcessor.from_pretrained("openbmb/MiniCPM-V-4.6")

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

#### get_image_features[[transformers.MiniCPMV4_6ForConditionalGeneration.get_image_features]]

```python
get_image_features(*args, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/modeling_minicpmv4_6.py#L831)

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MiniCPMV4_6Config](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6Config)) and inputs.

Extract image features: vision encoder, insert merger, then MLP merger.

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
>>> from transformers import AutoProcessor, MiniCPMV4_6ForConditionalGeneration

>>> model = MiniCPMV4_6ForConditionalGeneration.from_pretrained("openbmb/MiniCPM-V-4.6")
>>> processor = AutoProcessor.from_pretrained("openbmb/MiniCPM-V-4.6")

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

## MiniCPMV4_6Processor[[transformers.MiniCPMV4_6Processor]]

#### transformers.MiniCPMV4_6Processor[[transformers.MiniCPMV4_6Processor]]

```python
transformers.MiniCPMV4_6Processor(image_processor = None, video_processor = None, tokenizer = None, chat_template = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/processing_minicpmv4_6.py#L42)

**Parameters:**

image_processor (`MiniCPMV4_6ImageProcessor`) : The image processor is a required input.

video_processor (`MiniCPMV4_6VideoProcessor`) : The video processor is a required input.

tokenizer (`TokenizersBackend`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a MiniCPMV4_6Processor which wraps a image processor, a video processor, and a tokenizer into a single processor.

[MiniCPMV4_6Processor](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6Processor) offers all the functionalities of [MiniCPMV4_6ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6ImageProcessor), [MiniCPMV4_6VideoProcessor](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6VideoProcessor), and [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~MiniCPMV4_6ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6ImageProcessor), [~MiniCPMV4_6VideoProcessor](/docs/transformers/v5.15.1/en/model_doc/minicpmv4_6#transformers.MiniCPMV4_6VideoProcessor), and [~TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

#### __call__[[transformers.MiniCPMV4_6Processor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: str | list[str] | list[list[str]] | None = None, videos: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/processing_minicpmv4_6.py#L65)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

videos (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) : Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If passing in videos with pixel values between 0 and 1, set `do_rescale=False`.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

## MiniCPMV4_6ImageProcessor[[transformers.MiniCPMV4_6ImageProcessor]]

#### transformers.MiniCPMV4_6ImageProcessor[[transformers.MiniCPMV4_6ImageProcessor]]

```python
transformers.MiniCPMV4_6ImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/image_processing_minicpmv4_6.py#L68)

**Parameters:**

do_convert_rgb (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to convert the image to RGB.

do_resize (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to resize the image.

size (*Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]*, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to default to a square image when resizing, if size is an int.

crop_size (*Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]*, *kwargs*) : Size of the output image after applying *center_crop*.

resample (*Annotated[Union[int, PILImageResampling, NoneType], None]*, *kwargs*, defaults to *Resampling.BICUBIC*) : Resampling filter to use if resizing the image. This can be one of the enum *PILImageResampling*. Only has an effect if *do_resize* is set to *True*.

do_rescale (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to rescale the image.

rescale_factor (*float*, *kwargs*, *optional*, defaults to *0.00392156862745098*) : Rescale factor to rescale the image by if *do_rescale* is set to *True*.

do_normalize (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to normalize the image.

image_mean (*Union[float, list[float], tuple[float, ...]]*, *kwargs*, *optional*, defaults to *[0.5, 0.5, 0.5]*) : Image mean to use for normalization. Only has an effect if *do_normalize* is set to *True*.

image_std (*Union[float, list[float], tuple[float, ...]]*, *kwargs*, *optional*, defaults to *[0.5, 0.5, 0.5]*) : Image standard deviation to use for normalization. Only has an effect if *do_normalize* is set to *True*.

do_pad (*bool*, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (*Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]*, *kwargs*) : The size in *&amp;lcub;"height": int, "width" int}* to pad the images to. Must be larger than any image size provided for preprocessing. If *pad_size* is not provided, images will be padded to the largest height and width in the batch. Applied only when *do_pad=True.*

do_center_crop (*bool*, *kwargs*, *optional*) : Whether to center crop the image.

data_format (*Union[str, ~image_utils.ChannelDimension]*, *kwargs*, *optional*) : Only *ChannelDimension.FIRST* is supported. Added for compatibility with slow processors.

input_data_format (*Union[str, ~image_utils.ChannelDimension]*, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - *"channels_first"* or *ChannelDimension.FIRST*: image in (num_channels, height, width) format. - *"channels_last"* or *ChannelDimension.LAST*: image in (height, width, num_channels) format. - *"none"* or *ChannelDimension.NONE*: image in (height, width) format.

device (*Annotated[Union[str, torch.device, NoneType], None]*, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (*Annotated[str | ~utils.generic.TensorType | None, None]*, *kwargs*) : Returns stacked tensors if set to *'pt'*, otherwise returns a list of tensors.

disable_grouping (*bool*, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (*int*, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

max_slice_nums (*int*, *kwargs*, *optional*, defaults to 9) : Maximum number of slices when splitting a high-resolution image.

scale_resolution (*int*, *kwargs*, *optional*, defaults to 448) : Target resolution for individual slices.

patch_size (*int*, *kwargs*, *optional*, defaults to 14) : Spatial patch size of the vision encoder.

slice_mode (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to split images into multiple slices for higher resolution.

downsample_mode (*str*, *kwargs*, *optional*, defaults to *"16x"*) : Visual token downsampling mode. *"16x"* applies full merge; *"4x"* keeps 4x more tokens.

use_image_id (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to prepend an image-id tag (`<image_id>N</image_id>`) before each image placeholder. Consumed by the Processor for placeholder generation, not by the image processing pipeline itself.

Constructs a MiniCPMV4_6ImageProcessor image processor.

#### preprocess[[transformers.MiniCPMV4_6ImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/image_processing_minicpmv4_6.py#L168)

**Parameters:**

images (*Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set *do_rescale=False*.

do_convert_rgb (*bool*, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (*bool*, *kwargs*, *optional*) : Whether to resize the image.

size (*Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]*, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (*bool*, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (*Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]*, *kwargs*) : Size of the output image after applying *center_crop*.

resample (*Annotated[Union[int, PILImageResampling, NoneType], None]*, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum *PILImageResampling*. Only has an effect if *do_resize* is set to *True*.

do_rescale (*bool*, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (*float*, *kwargs*, *optional*) : Rescale factor to rescale the image by if *do_rescale* is set to *True*.

do_normalize (*bool*, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (*Union[float, list[float], tuple[float, ...]]*, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if *do_normalize* is set to *True*.

image_std (*Union[float, list[float], tuple[float, ...]]*, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if *do_normalize* is set to *True*.

do_pad (*bool*, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (*Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]*, *kwargs*) : The size in *&amp;lcub;"height": int, "width" int}* to pad the images to. Must be larger than any image size provided for preprocessing. If *pad_size* is not provided, images will be padded to the largest height and width in the batch. Applied only when *do_pad=True.*

do_center_crop (*bool*, *kwargs*, *optional*) : Whether to center crop the image.

data_format (*Union[str, ~image_utils.ChannelDimension]*, *kwargs*, *optional*) : Only *ChannelDimension.FIRST* is supported. Added for compatibility with slow processors.

input_data_format (*Union[str, ~image_utils.ChannelDimension]*, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - *"channels_first"* or *ChannelDimension.FIRST*: image in (num_channels, height, width) format. - *"channels_last"* or *ChannelDimension.LAST*: image in (height, width, num_channels) format. - *"none"* or *ChannelDimension.NONE*: image in (height, width) format.

device (*Annotated[Union[str, torch.device, NoneType], None]*, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (*Annotated[str | ~utils.generic.TensorType | None, None]*, *kwargs*) : Returns stacked tensors if set to *'pt'*, otherwise returns a list of tensors.

disable_grouping (*bool*, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (*int*, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

max_slice_nums (*int*, *kwargs*, *optional*, defaults to 9) : Maximum number of slices when splitting a high-resolution image.

scale_resolution (*int*, *kwargs*, *optional*, defaults to 448) : Target resolution for individual slices.

patch_size (*int*, *kwargs*, *optional*, defaults to 14) : Spatial patch size of the vision encoder.

slice_mode (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to split images into multiple slices for higher resolution.

downsample_mode (*str*, *kwargs*, *optional*, defaults to *"16x"*) : Visual token downsampling mode. *"16x"* applies full merge; *"4x"* keeps 4x more tokens.

use_image_id (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to prepend an image-id tag (`<image_id>N</image_id>`) before each image placeholder. Consumed by the Processor for placeholder generation, not by the image processing pipeline itself.

**Returns:** `*~image_processing_base.BatchFeature*`

- **data** (*dict*) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (*Union[None, str, TensorType]*, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## MiniCPMV4_6ImageProcessorPil[[transformers.MiniCPMV4_6ImageProcessorPil]]

#### transformers.MiniCPMV4_6ImageProcessorPil[[transformers.MiniCPMV4_6ImageProcessorPil]]

```python
transformers.MiniCPMV4_6ImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/image_processing_pil_minicpmv4_6.py#L69)

**Parameters:**

- ****kwargs** (`MiniCPMV4_6ImageProcessorPilKwargs`, *optional*) : Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class for the complete list of supported arguments.

Constructs a MiniCPMV4_6ImageProcessor image processor.

#### preprocess[[transformers.MiniCPMV4_6ImageProcessorPil.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/image_processing_pil_minicpmv4_6.py#L167)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

- ****kwargs** (`MiniCPMV4_6ImageProcessorPilKwargs`, *optional*) : Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class for the complete list of supported arguments.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## MiniCPMV4_6VideoProcessor[[transformers.MiniCPMV4_6VideoProcessor]]

#### transformers.MiniCPMV4_6VideoProcessor[[transformers.MiniCPMV4_6VideoProcessor]]

```python
transformers.MiniCPMV4_6VideoProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/video_processing_minicpmv4_6.py#L84)

**Parameters:**

do_convert_rgb (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to convert the image to RGB.

do_resize (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to resize the image.

size (*Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]*, *kwargs*, defaults to *None*) : Describes the maximum input dimensions to the model.

default_to_square (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to default to a square image when resizing, if size is an int.

resample (*Annotated[Union[int, PILImageResampling, NoneType], None]*, *kwargs*, defaults to *Resampling.BICUBIC*) : Resampling filter to use if resizing the image. This can be one of the enum *PILImageResampling*. Only has an effect if *do_resize* is set to *True*.

do_rescale (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to rescale the image.

rescale_factor (*float*, *kwargs*, *optional*, defaults to *0.00392156862745098*) : Rescale factor to rescale the image by if *do_rescale* is set to *True*.

do_normalize (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to normalize the image.

image_mean (*Union[float, list[float], tuple[float, ...]]*, *kwargs*, *optional*, defaults to *[0.5, 0.5, 0.5]*) : Image mean to use for normalization. Only has an effect if *do_normalize* is set to *True*.

image_std (*Union[float, list[float], tuple[float, ...]]*, *kwargs*, *optional*, defaults to *[0.5, 0.5, 0.5]*) : Image standard deviation to use for normalization. Only has an effect if *do_normalize* is set to *True*.

do_center_crop (*bool*, *kwargs*, *optional*, defaults to *None*) : Whether to center crop the image.

do_pad (*bool*, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

crop_size (*Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]*, *kwargs*, defaults to *None*) : Size of the output image after applying *center_crop*.

data_format (*Union[str, ~image_utils.ChannelDimension]*, *kwargs*, *optional*) : Only *ChannelDimension.FIRST* is supported. Added for compatibility with slow processors.

input_data_format (*Union[str, ~image_utils.ChannelDimension]*, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - *"channels_first"* or *ChannelDimension.FIRST*: image in (num_channels, height, width) format. - *"channels_last"* or *ChannelDimension.LAST*: image in (height, width, num_channels) format. - *"none"* or *ChannelDimension.NONE*: image in (height, width) format.

device (*Annotated[Union[str, torch.device, NoneType], None]*, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

do_sample_frames (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to sample frames from the video before processing or to process the whole video.

video_metadata (*Annotated[~video_utils.VideoMetadata | dict | list[dict | ~video_utils.VideoMetadata] | list[list[dict | ~video_utils.VideoMetadata]] | None, None]*, *kwargs*, defaults to *None*) : Metadata of the video containing information about total duration, fps and total number of frames. It will be used to sample frames from video or compute timestamps. Don't pass any metadata unless you are trying to decode the video manually before processing

fps (*Annotated[int | float | None, None]*, *kwargs*, defaults to *None*) : Target frames to sample per second when *do_sample_frames=True*.

num_frames (*Annotated[int | None, None]*, *kwargs*, defaults to *None*) : Maximum number of frames to sample when *do_sample_frames=True*.

return_metadata (*bool*, *kwargs*, *optional*, defaults to *False*) : Whether to return video metadata or not. Video metadats is an object containing info about video duration, fps, decoding backend, etc.

return_tensors (*Annotated[str | ~utils.generic.TensorType | None, None]*, *kwargs*) : Returns stacked tensors if set to *'pt'*, otherwise returns a list of tensors.

max_num_frames (*int*, *kwargs*, *optional*, defaults to 128) : Maximum number of main frames to sample per video.

stack_frames (*int*, *kwargs*, *optional*, defaults to 1) : Sub-frames per second to stack.  `1` disables stacking.

max_slice_nums (*int*, *kwargs*, *optional*, defaults to 9) : Maximum number of slices when splitting a high-resolution image.

scale_resolution (*int*, *kwargs*, *optional*, defaults to 448) : Target resolution for individual slices.

patch_size (*int*, *kwargs*, *optional*, defaults to 14) : Spatial patch size of the vision encoder.

slice_mode (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to split images into multiple slices for higher resolution.

downsample_mode (*str*, *kwargs*, *optional*, defaults to *"16x"*) : Visual token downsampling mode. *"16x"* applies full merge; *"4x"* keeps 4x more tokens.

use_image_id (*bool*, *kwargs*, *optional*, defaults to *True*) : Whether to prepend an image-id tag (`<image_id>N</image_id>`) before each image placeholder. Consumed by the Processor for placeholder generation, not by the image processing pipeline itself.

Constructs a MiniCPMV4_6VideoProcessor video processor.

#### preprocess[[transformers.MiniCPMV4_6VideoProcessor.preprocess]]

```python
preprocess(videos: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]]], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/minicpmv4_6/video_processing_minicpmv4_6.py#L294)

**Parameters:**

videos (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`) : Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If passing in videos with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

do_sample_frames (`bool`, *kwargs*, *optional*) : Whether to sample frames from the video before processing or to process the whole video.

video_metadata (`Annotated[~video_utils.VideoMetadata | dict | list[dict | ~video_utils.VideoMetadata] | list[list[dict | ~video_utils.VideoMetadata]] | None, None]`, *kwargs*) : Metadata of the video containing information about total duration, fps and total number of frames. It will be used to sample frames from video or compute timestamps. Don't pass any metadata unless you are trying to decode the video manually before processing

fps (`Annotated[int | float | None, None]`, *kwargs*) : Target frames to sample per second when `do_sample_frames=True`.

num_frames (`Annotated[int | None, None]`, *kwargs*) : Maximum number of frames to sample when `do_sample_frames=True`.

return_metadata (`bool`, *kwargs*, *optional*) : Whether to return video metadata or not. Video metadats is an object containing info about video duration, fps, decoding backend, etc.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.
