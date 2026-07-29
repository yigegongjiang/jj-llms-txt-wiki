# KimiK-2.5, KimiK-2.6, KimiK-2.7

This model class supports all three different releases: KimiK-2.5,KimiK-2.6, KimiK-2.7

## Overview

Kimi K2.5 is an open-source, native multimodal agentic model that advances practical capabilities in long-horizon coding, coding-driven design, proactive autonomous execution, and swarm-based task orchestration. The model was proposed in [Kimi K2.5: Visual Agentic Intelligence](https://www.kimi.com/en/blog/kimi-k2-5) and further improved in [Kimi K2.6: Advancing Open-Source Coding](Kimi K2.5: Visual Agentic Intelligence).

Kimi K2.5 achieves significant improvements on complex, end-to-end coding tasks, generalizing robustly across programming languages (Rust, Go, Python) and domains spanning front-end, DevOps, and performance optimization. The model is capable of transforming simple prompts and visual inputs into production-ready interfaces and lightweight full-stack workflows, generating structured layouts, interactive elements, and rich animations with deliberate aesthetic precision.

This model was contributed by [RaushanTurganbay](https://huggingface.co/RaushanTurganbay).
The offical checkpoints are [moonshotai/Kimi-K2.5](https://huggingface.co/moonshotai/Kimi-K2.5), [moonshotai/Kimi-K2.6](https://huggingface.co/moonshotai/Kimi-K2.6) and [moonshotai/Kimi-K2.7-Code](https://huggingface.co/moonshotai/Kimi-K2.7-Code).

## Usage examples

Note that the repositories don't yet have the correct fast tokenizer uploaded. You can get the converted processor and tokenizer from [RaushanTurganbay/kimi2.7-processor](https://huggingface.co/RaushanTurganbay/kimi2.7-processor)

 

```python
import os
import torch
from transformers import AutoProcessor, AutoTokenizer, AutoModelForImageTextToText
from transformers.distributed.configuration_utils import DistributedConfig

distributed_config = DistributedConfig(enable_expert_parallel=True)

processor = AutoProcessor.from_pretrained('moonshotai/Kimi-K2.6')
model = AutoModelForImageTextToText.from_pretrained(
    'moonshotai/Kimi-K2.6',
    distributed_config=distributed_config,
)

messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "image": "https://www.ilankelman.org/stopsigns/australia.jpg"},
            {"type": "text", "text": "What is shown in this image?"},
        ],
    }
]

inputs = processor.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_tensors="pt",
    return_dict=True,
).to(device=model.device, dtype=model.dtype)

generated_ids = model.generate(**inputs, max_new_tokens=64)
generated_text = processor.batch_decode(generated_ids[:, inputs["input_ids"].shape[-1]:], skip_special_tokens=True)[0]
print(generated_text)

```

## Kimi_K25ImageProcessor[[transformers.Kimi_K25ImageProcessor]]

- **max_patches** (`int`, *kwargs*, *optional*, defaults to `16384`) --
  The max limit to resize resize the image.
- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a Kimi_K25ImageProcessor image processor.

- **height** (`int`) --
  Height of the input image.
- **width** (`int`) --
  Width of the input image.
- **images_kwargs** (`dict`, *optional*) --
  Any kwargs to override defaults of the image processor.`int`Number of image patches per image.

A utility that returns number of image patches for a given image size.

Note: Do not remove this method! It is used by vLLM to infer the number of patches and placeholders
without an image input.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **max_patches** (`int`, *kwargs*, *optional*, defaults to `16384`) --
  The max limit to resize resize the image.
- **patch_size** (`int`, *kwargs*, *optional*, defaults to 14) --
  The spatial patch size of the vision encoder.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.`~image_processing_base.BatchFeature`- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Kimi_K25Processor[[transformers.Kimi_K25Processor]]

- **image_processor** (`Kimi_K25ImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`TokenizersBackend`) --
  The tokenizer is a required input.
- **video_processor** (`Kimi_K25VideoProcessor`) --
  The video processor is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
Constructs a Kimi_K25Processor which wraps a image processor, a tokenizer, and a video processor into a single processor.

[Kimi_K25Processor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) offers all the functionalities of [Kimi_K25ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor), [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend), and [Kimi_K25VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor). See the
[~Kimi_K25ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor), [~TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend), and [~Kimi_K25VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor) for more information.

- **generated_outputs** (`torch.Tensor` or `np.ndarray`) --
  The output of the model `generate` function. The output is expected to be a tensor of shape `(batch_size, sequence_length)`
  or `(sequence_length,)`.
- **skip_special_tokens** (`bool`, *optional*, defaults to `True`) --
  Whether or not to remove special tokens in the output. Argument passed to the tokenizer's `batch_decode` method.
- **clean_up_tokenization_spaces** (`bool`, *optional*, defaults to `False`) --
  Whether or not to clean up the tokenization spaces. Argument passed to the tokenizer's `batch_decode` method.
- ****kwargs** --
  Additional arguments to be passed to the tokenizer's `batch_decode method`.`list[str]`The decoded text.

Post-process the output of the model to decode the text.

## Kimi_K25VideoProcessor[[transformers.Kimi_K25VideoProcessor]]

## Kimi_K25Config[[transformers.Kimi_K25Config]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **projection_hidden_size** (`int`, *optional*, defaults to `1152`) --
  The output hidden size for multimodal projector.
- **projection_layer_norm_eps** (`float`, *optional*, defaults to `1e-5`) --
  Layer norm epsilon for projector.
- **image_token_id** (`int`, *optional*, defaults to `163605`) --
  The image token index used as a placeholder for input images.
- **video_token_id** (`int`, *optional*, defaults to `163840`) --
  The video token index used as a placeholder for input videos.
- **vision_start_token_id** (`int`, *optional*, defaults to `163602`) --
  Token ID that marks the start of a visual segment in the multimodal input sequence.
- **vision_end_token_id** (`int`, *optional*, defaults to `163604`) --
  Token ID that marks the end of a visual segment in the multimodal input sequence.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Kimi_K25Model. It is used to instantiate a Kimi K25
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [moonshotai/Kimi-K2.6](https://huggingface.co/moonshotai/Kimi-K2.6)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Kimi_K25VisionConfig[[transformers.Kimi_K25VisionConfig]]

- **patch_size** (`int`, *optional*, defaults to `14`) --
  The size (resolution) of each patch.
- **pos_emb_height** (`int`, *optional*) --
  Initial position embedding height.
- **pos_emb_width** (`int`, *optional*) --
  Initial position embedding width.
- **pos_emb_time** (`int`, *optional*) --
  Initial position embedding time dimension.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_hidden_layers** (`int`, *optional*, defaults to `27`) --
  Number of hidden layers in the Transformer decoder.
- **hidden_size** (`int`, *optional*, defaults to `1152`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `4304`) --
  Dimension of the MLP representations.
- **hidden_act** (`str`, *optional*, defaults to `gelu_pytorch_tanh`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **merge_kernel_size** (`tuple[int] | list[int]`, *optional*) --
  Kernel size for patch merging.
- **rope_parameters** (`dict`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **max_position_embeddings** (`int`, *optional*) --
  The maximum sequence length that this model might ever be used with.

This is the configuration class to store the configuration of a Kimi_K25Model. It is used to instantiate a Kimi K25
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [moonshotai/Kimi-K2.6](https://huggingface.co/moonshotai/Kimi-K2.6)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Kimi_K25PreTrainedModel[[transformers.Kimi_K25PreTrainedModel]]

- **config** ([PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

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

## Kimi_K25VisionModel[[transformers.Kimi_K25VisionModel]]

)>"}, {"name": "grid_thw", "val": ": )>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Kimi_K25ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor). See `Kimi_K25ImageProcessor.__call__()` for details ([Kimi_K25Processor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) uses
  [Kimi_K25ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor) for processing images).
- **grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.
The [Kimi_K25VisionModel](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25VisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

)>"}, {"name": "grid_thw", "val": ": )>"}]}>
- **hidden_states** (`torch.Tensor` of shape `(total_patches, hidden_dim)`) --
  Concatenated patch embeddings for all clips in the batch. `total_patches` equals
  the sum of `t * h * w` over all entries in `grid_thw`.
- **grid_thw** (`torch.Tensor` of shape `(batch_size, 3)`) --
  Temporal and spatial grid dimensions for each clip, where each row is
  `(num_frames, grid_height, grid_width)`. `grid_height` and `grid_width` must be
  divisible by `kernel_height` and `kernel_width` respectively.`torch.Tensor` of shape `(total_merged_patches, kernel_height * kernel_width, hidden_dim)`Temporally pooled patch embeddings. `total_merged_patches` equals the sum of
`(h // kernel_height) * (w // kernel_width)` over all clips.

Merges temporal frames by spatially pooling patch embeddings across time.

For each video clip defined by `grid_thw`, the method reshapes the flat patch sequence
into a `(T, H, W)` grid, averages over the temporal dimension, then rearranges spatial
patches into groups of `kernel_height * kernel_width` — matching the merged-token layout
expected by downstream layers.

## Kimi_K25Model[[transformers.Kimi_K25Model]]

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
  `past_key_values`).
- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Kimi_K25ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor). See `Kimi_K25ImageProcessor.__call__()` for details ([Kimi_K25Processor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) uses
  [Kimi_K25ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor) for processing images).
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.
- **pixel_values_videos** (`torch.Tensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [Kimi_K25VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor). See `Kimi_K25VideoProcessor.__call__()` for details ([Kimi_K25Processor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) uses
  [Kimi_K25VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor) for processing videos).
- **video_grid_thw** (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) --
  The temporal, height and width of feature shape of each video in LLM.`Kimi_K25ModelOutputWithPast` or `tuple(torch.FloatTensor)`A `Kimi_K25ModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Kimi_K25Config](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Config)) and inputs.
The [Kimi_K25Model](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Model) forward method, overrides the `__call__` special method.

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

## Kimi_K25ForConditionalGeneration[[transformers.Kimi_K25ForConditionalGeneration]]

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
  [Kimi_K25ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor). See `Kimi_K25ImageProcessor.__call__()` for details ([Kimi_K25Processor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) uses
  [Kimi_K25ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25ImageProcessor) for processing images).
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.
- **pixel_values_videos** (`torch.Tensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`, *optional*) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [Kimi_K25VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor). See `Kimi_K25VideoProcessor.__call__()` for details ([Kimi_K25Processor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Processor) uses
  [Kimi_K25VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25VideoProcessor) for processing videos).
- **video_grid_thw** (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) --
  The temporal, height and width of feature shape of each video in LLM.
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`Kimi_K25CausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `Kimi_K25CausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Kimi_K25Config](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Config)) and inputs.
The [Kimi_K25ForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25ForConditionalGeneration) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoProcessor, Kimi_K25ForConditionalGeneration

>>> model = Kimi_K25ForConditionalGeneration.from_pretrained("moonshotai/Kimi-K2.6")
>>> processor = AutoProcessor.from_pretrained("moonshotai/Kimi-K2.6")

>>> messages = [
    {
        "role": "user",
        "content": [
            {
                "type": "image",
                "image": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg",
            },
            {"type": "text", "text": "Describe the image."},
        ],
    }
]

>>> inputs = processor.apply_chat_template(
    messages,
    tokenize=True,
    add_generation_prompt=True,
    return_dict=True,
    return_tensors="pt",
)

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
elements depending on the configuration ([Kimi_K25Config](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Config)) and inputs.

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
>>> from transformers import AutoProcessor, Kimi_K25ForConditionalGeneration

>>> model = Kimi_K25ForConditionalGeneration.from_pretrained("moonshotai/Kimi-K2.6")
>>> processor = AutoProcessor.from_pretrained("moonshotai/Kimi-K2.6")

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

- **pixel_values_videos** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input videos.
- **video_grid_thw** (`torch.LongTensor` of shape `(num_videos, 3)`, *optional*) --
  The temporal, height and width of feature shape of each video in LLM.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Kimi_K25Config](/docs/transformers/v5.14.0/en/model_doc/kimi_k25#transformers.Kimi_K25Config)) and inputs.

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
>>> from transformers import AutoProcessor, Kimi_K25ForConditionalGeneration

>>> model = Kimi_K25ForConditionalGeneration.from_pretrained("moonshotai/Kimi-K2.6")
>>> processor = AutoProcessor.from_pretrained("moonshotai/Kimi-K2.6")

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
