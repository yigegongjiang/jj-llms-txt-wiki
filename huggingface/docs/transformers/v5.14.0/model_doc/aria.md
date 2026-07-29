# Aria

[Aria](https://huggingface.co/papers/2410.05993) is a multimodal mixture-of-experts (MoE) model. The goal of this model is to open-source a training recipe for creating a multimodal native model from scratch. Aria has 3.9B and 3.5B activated parameters per visual and text token respectively. Text is handled by a MoE decoder and visual inputs are handled by a lightweight visual encoder. It is trained in 4 stages, language pretraining, multimodal pretraining, multimodal long-context pretraining, and multimodal post-training.

You can find all the original Aria checkpoints under the [Aria](https://huggingface.co/rhymes-ai?search_models=aria) organization.

> [!TIP]
> Click on the Aria models in the right sidebar for more examples of how to apply Aria to different multimodal tasks.

The example below demonstrates how to generate text based on an image with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipeline = pipeline(
    "image-to-text",
    model="rhymes-ai/Aria",
    device=0,
)
pipeline(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg",
    text="What is shown in this image?"
)
```

```python
import torch

from transformers import AutoModelForCausalLM, AutoProcessor

model = AutoModelForCausalLM.from_pretrained(
    "rhymes-ai/Aria",
    device_map="auto",
    attn_implementation="sdpa"
)

processor = AutoProcessor.from_pretrained("rhymes-ai/Aria")

messages = [
    {
        "role": "user", "content": [
            {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
            {"type": "text", "text": "What is shown in this image?"},
        ]
    },
]

inputs = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(model.device)
inputs = inputs.to(model.device, torch.bfloat16)

output = model.generate(
    **inputs,
    max_new_tokens=15,
    stop_strings=["<|im_end|>"],
    tokenizer=processor.tokenizer,
    do_sample=True,
    temperature=0.9,
)
output_ids = output[0][inputs["input_ids"].shape[1]:]
response = processor.decode(output_ids, skip_special_tokens=True)
print(response)
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [torchao](../quantization/torchao) to only quantize the weights to int4 and the [rhymes-ai/Aria-sequential_mlp](https://huggingface.co/rhymes-ai/Aria-sequential_mlp) checkpoint. This checkpoint replaces grouped GEMM with `torch.nn.Linear` layers for easier quantization.

```python
# pip install torchao
import torch

from transformers import AutoModelForCausalLM, AutoProcessor, TorchAoConfig

quantization_config = TorchAoConfig("int4_weight_only", group_size=128)
model = AutoModelForCausalLM.from_pretrained(
    "rhymes-ai/Aria-sequential_mlp",
    device_map="auto",
    quantization_config=quantization_config
)
processor = AutoProcessor.from_pretrained(
    "rhymes-ai/Aria-sequential_mlp",
)

messages = [
    {
        "role": "user", "content": [
            {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
            {"type": "text", "text": "What is shown in this image?"},
        ]
    },
]

inputs = processor.apply_chat_template(messages, add_generation_prompt=True, tokenize=True, return_dict=True, return_tensors="pt").to(model.device)
inputs = inputs.to(model.device, torch.bfloat16)

output = model.generate(
    **inputs,
    max_new_tokens=15,
    stop_strings=["<|im_end|>"],
    tokenizer=processor.tokenizer,
    do_sample=True,
    temperature=0.9,
)
output_ids = output[0][inputs["input_ids"].shape[1]:]
response = processor.decode(output_ids, skip_special_tokens=True)
print(response)
```

## AriaImageProcessor[[transformers.AriaImageProcessor]]

- **max_image_size** (`int`, *kwargs*, *optional*, defaults to `self.max_image_size`) --
  Maximum image size. Must be either 490 or 980.
- **min_image_size** (`int`, *kwargs*, *optional*, defaults to `self.min_image_size`) --
  Minimum image size. Images smaller than this in any dimension will be scaled up.
- **split_resolutions** (`list[list[int]]`, *kwargs*, *optional*, defaults to `self.split_resolutions`) --
  A list of possible resolutions as (height, width) pairs for splitting high-resolution images into patches.
- **split_image** (`bool`, *kwargs*, *optional*, defaults to `self.split_image`) --
  Whether to split the image into patches using the best matching resolution from `split_resolutions`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a AriaImageProcessor image processor.

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

## AriaImageProcessorPil[[transformers.AriaImageProcessorPil]]

- **max_image_size** (`int`, *kwargs*, *optional*, defaults to `self.max_image_size`) --
  Maximum image size. Must be either 490 or 980.
- **min_image_size** (`int`, *kwargs*, *optional*, defaults to `self.min_image_size`) --
  Minimum image size. Images smaller than this in any dimension will be scaled up.
- **split_resolutions** (`list[list[int]]`, *kwargs*, *optional*, defaults to `self.split_resolutions`) --
  A list of possible resolutions as (height, width) pairs for splitting high-resolution images into patches.
- **split_image** (`bool`, *kwargs*, *optional*, defaults to `self.split_image`) --
  Whether to split the image into patches using the best matching resolution from `split_resolutions`.
- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a AriaImageProcessor image processor.

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

## AriaProcessor[[transformers.AriaProcessor]]

- **image_processor** (`AriaImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`TokenizersBackend`, *optional*) --
  The tokenizer is a required input.
- **chat_template** (`str`, *optional*) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **size_conversion** (`Dict`, *optional*) --
  A dictionary indicating size conversions for images.
Constructs a AriaProcessor which wraps a image processor and a tokenizer into a single processor.

[AriaProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaProcessor) offers all the functionalities of [AriaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaImageProcessor) and [TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~AriaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaImageProcessor) and [~TokenizersBackend](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
- **videos** (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) --
  Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If
  passing in videos with pixel values between 0 and 1, set `do_rescale=False`.
- **audio** (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) --
  The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor.
  In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels,
  and T is the sample length of the audio.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.
- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) --
  Additional processing options for each modality (text, images, videos, audio). Model-specific parameters
  are listed above; see the TypedDict class for the complete list of supported arguments.

## AriaTextConfig[[transformers.AriaTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `32000`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `32`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `32`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_key_value_heads** (`int`, *optional*) --
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
- **max_position_embeddings** (`int`, *optional*, defaults to `2048`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **rms_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the rms normalization layers.
- **use_cache** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return the last key/values attentions (not used by all models). Only
  relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.
- **pad_token_id** (`int`, *optional*, defaults to `2`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **pretraining_tp** (`int`, *optional*, defaults to `1`) --
  Experimental feature. Tensor parallelism rank used during pretraining. Please refer to [this
  document](https://huggingface.co/docs/transformers/main/perf_train_gpu_many#tensor-parallelism) to
  understand more about it. This value is necessary to ensure exact reproducibility of the pretraining
  results. Please refer to [this issue](https://github.com/pytorch/pytorch/issues/76232).
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[int, float]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **mlp_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.
- **head_dim** (`int`, *optional*) --
  The attention head dimension. If None, it will default to hidden_size // num_attention_heads
- **moe_num_experts** (`int`, *optional*, defaults to 8) --
  The number of experts in the MoE layer.
- **moe_topk** (`int`, *optional*, defaults to 2) --
  The number of top experts to route to for each token.
- **moe_num_shared_experts** (`int`, *optional*, defaults to 2) --
  The number of shared experts.

This is the configuration class to store the configuration of a AriaModel. It is used to instantiate a Aria
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [rhymes-ai/Aria](https://huggingface.co/rhymes-ai/Aria)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## AriaConfig[[transformers.AriaConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~models.aria.configuration_aria.AriaTextConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*, defaults to `-1`) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **projector_patch_to_query_dict** (`dict`, *optional*) --
  Mapping of patch sizes to query dimensions.
- **image_token_index** (`int`, *optional*, defaults to `9`) --
  The image token index used as a placeholder for input images.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a AriaModel. It is used to instantiate a Aria
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [rhymes-ai/Aria](https://huggingface.co/rhymes-ai/Aria)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## AriaTextModel[[transformers.AriaTextModel]]

- **config** ([AriaTextConfig](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Aria Text Model outputting raw hidden-states without any specific head on to.

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
elements depending on the configuration ([AriaConfig](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaConfig)) and inputs.
The [AriaTextModel](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaTextModel) forward method, overrides the `__call__` special method.

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

## AriaModel[[transformers.AriaModel]]

- **config** ([AriaConfig](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Aria model which consists of a vision backbone and a language model, without a language modeling head.

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
  [AriaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaImageProcessor). See `AriaImageProcessor.__call__()` for details ([AriaProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaProcessor) uses
  [AriaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaImageProcessor) for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
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
  `past_key_values`).`AriaModelOutputWithPast` or `tuple(torch.FloatTensor)`A `AriaModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AriaConfig](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaConfig)) and inputs.
The [AriaModel](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaModel) forward method, overrides the `__call__` special method.

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

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [AriaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaImageProcessor). See `AriaImageProcessor.__call__()` for details ([AriaProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaProcessor) uses
  [AriaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaImageProcessor) for processing images).
- **pixel_mask** (`torch.FloatTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*, defaults to `-1`) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AriaConfig](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaConfig)) and inputs.
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

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

## AriaTextForCausalLM[[transformers.AriaTextForCausalLM]]

- **config** ([AriaTextConfig](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Aria Model for causal language modeling.

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
elements depending on the configuration ([AriaConfig](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaConfig)) and inputs.
The [AriaTextForCausalLM](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaTextForCausalLM) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, AriaTextForCausalLM

>>> model = AriaTextForCausalLM.from_pretrained("meta-aria_text/AriaText-2-7b-hf")
>>> tokenizer = AutoTokenizer.from_pretrained("meta-aria_text/AriaText-2-7b-hf")

>>> prompt = "Hey, are you conscious? Can you talk to me?"
>>> inputs = tokenizer(prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(inputs.input_ids, max_length=30)
>>> tokenizer.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"Hey, are you conscious? Can you talk to me?\nI'm not conscious, but I can talk to you."
```

## AriaForConditionalGeneration[[transformers.AriaForConditionalGeneration]]

- **config** ([AriaConfig](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Aria model for conditional generation tasks.

This model combines a vision tower, a multi-modal projector, and a language model
to perform tasks that involve both image and text inputs.

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
  [AriaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaImageProcessor). See `AriaImageProcessor.__call__()` for details ([AriaProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaProcessor) uses
  [AriaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaImageProcessor) for processing images).
- **pixel_mask** (`torch.LongTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
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
  config.vocab_size]` or `model.image_token_id` (where `model` is your instance of `AriaForConditionalGeneration`).
  Tokens with indices set to `model.image_token_id` are ignored (masked), the loss is only
  computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **logits_to_keep** (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) --
  If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all
  `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that
  token can save memory, which becomes pretty significant for long sequences or large vocabulary size.
  If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension.
  This is useful when using packed tensor format (single dimension for batch and sequence length).`AriaCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `AriaCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AriaConfig](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaConfig)) and inputs.
The [AriaForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaForConditionalGeneration) forward method, overrides the `__call__` special method.

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
>>> import httpx
>>> from io import BytesIO
>>> import torch
>>> from PIL import Image
>>> from io import BytesIO

>>> from transformers import AutoProcessor, AutoModel
>>> from transformers.image_utils import load_image

>>> # Note that passing the image urls (instead of the actual pil images) to the processor is also possible
>>> image1 = load_image("https://cdn.britannica.com/61/93061-050-99147DCE/Statue-of-Liberty-Island-New-York-Bay.jpg")
>>> image2 = load_image("https://cdn.britannica.com/59/94459-050-DBA42467/Skyline-Chicago.jpg")
>>> image3 = load_image("https://cdn.britannica.com/68/170868-050-8DDE8263/Golden-Gate-Bridge-San-Francisco.jpg")

>>> processor = AutoProcessor.from_pretrained("Rhymes-AI/Aria")
>>> model = AutoModel.from_pretrained("Rhymes-AI/Aria", dtype=torch.bfloat16, device_map="auto")

>>> # Create inputs
>>> messages = [
...     {
...         "role": "user",
...         "content": [
...             {"type": "image"},
...             {"type": "text", "text": "In this image, we can see the city of New York, and more specifically the Statue of Liberty."},
...             {"type": "image"},
...             {"type": "text", "text": "What can we see in this image?"},
...         ]
...     },
...     {
...         "role": "user",
...         "content": [
...             {"type": "image"},
...             {"type": "text", "text": "In which city is that bridge located?"},
...         ]
...     }
... ]

>>> prompts = [processor.apply_chat_template([message], add_generation_prompt=True) for message in messages]
>>> images = [[image1, image2], [image3]]
>>> inputs = processor(text=prompts, images=images, padding=True, return_tensors="pt").to(model.device)

>>> # Generate
>>> generated_ids = model.generate(**inputs, max_new_tokens=256)
>>> generated_texts = processor.batch_decode(generated_ids, skip_special_tokens=True)

>>> print(generated_texts[0])
Assistant: There are buildings, trees, lights, and water visible in this image.

>>> print(generated_texts[1])
Assistant: The bridge is in San Francisco.
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [AriaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaImageProcessor). See `AriaImageProcessor.__call__()` for details ([AriaProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaProcessor) uses
  [AriaImageProcessor](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaImageProcessor) for processing images).
- **pixel_mask** (`torch.FloatTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*, defaults to `-1`) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AriaConfig](/docs/transformers/v5.14.0/en/model_doc/aria#transformers.AriaConfig)) and inputs.

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
>>> from transformers import AutoProcessor, AriaForConditionalGeneration

>>> model = AriaForConditionalGeneration.from_pretrained("rhymes-ai/Aria")
>>> processor = AutoProcessor.from_pretrained("rhymes-ai/Aria")

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
