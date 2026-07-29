# Granite4Vision

[Granite Vision 4.1](https://huggingface.co/ibm-granite/granite-vision-4.1-4b) is a vision-language model from IBM Research designed for enterprise-grade document data extraction. It specializes in chart extraction (Chart2CSV, Chart2Summary, Chart2Code), table extraction (JSON, HTML, OTSL), and semantic key-value pair extraction.

The model builds on [LLaVA-NeXT](llava_next) with several architectural innovations:

1. **SigLIP2 Vision Encoder** ([`google/siglip2-so400m-patch16-384`](https://huggingface.co/google/siglip2-so400m-patch16-384)): images are tiled into 384x384 patches.
2. **Window Q-Former Projectors**: compress visual features 4x using windowed cross-attention over 4x4 patch windows into 2x2 tokens.
3. **DeepStack Feature Injection** with 8 vision-to-LLM injection points:
   - *LayerDeepstack*: features from 4 vision encoder depths are projected into different early LLM layers.
   - *SpatialDeepstack*: deepest vision features are split into 4 spatial groups and injected at later LLM layers.
4. **Language Model**: [Granite 4.1](https://huggingface.co/ibm-granite/granite-4.1-4b-base) (4B params) with LoRA adapters (rank 256) across all self-attention and MLP layers.

The model is delivered as a LoRA adapter on top of the base LLM, enabling single deployments to support both multimodal and text-only workloads. Total parameter count is ~4B.

> [!TIP]
> This model was contributed by the [IBM Granite Vision Team](https://github.com/ibm-granite).

## Usage Tips

- Set `padding_side="left"` during batched generation for more accurate results.

```py
processor.tokenizer.padding_side = "left"
```

- The model supports specialized task tags for document extraction: `<chart2csv>`, `<chart2summary>`, `<chart2code>`, `<tables_html>`, `<tables_otsl>`, `<tables_json>`. Pass these as the text prompt along with a document image.

- For key-value pair extraction, provide a JSON schema describing the fields to extract. The model returns structured JSON matching the schema.

The example below demonstrates how to generate text based on an image with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipe = pipeline(
    task="image-text-to-text",
    model="ibm-granite/granite-vision-4.1-4b",
)
messages = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
            {"type": "text", "text": "Describe this image."},
        ],
    }
]
pipe(text=messages, max_new_tokens=100, return_full_text=False)
```

```python
import torch
from transformers import AutoProcessor, AutoModelForImageTextToText

model_id = "ibm-granite/granite-vision-4.1-4b"

processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForImageTextToText.from_pretrained(model_id, device_map="auto")

conversation = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
            {"type": "text", "text": "Describe this image."},
        ],
    },
]
inputs = processor.apply_chat_template(
    conversation,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
).to(model.device)

output = model.generate(**inputs, max_new_tokens=100)
print(processor.decode(output[0], skip_special_tokens=True))
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to only quantize the weights to int4.

```python
import torch
from transformers import AutoProcessor, AutoModelForImageTextToText, BitsAndBytesConfig

quant_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_compute_dtype=torch.float16,
    bnb_4bit_quant_type="nf4",
)

model_id = "ibm-granite/granite-vision-4.1-4b"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForImageTextToText.from_pretrained(
    model_id, quantization_config=quant_config, device_map="auto"
)

conversation = [
    {
        "role": "user",
        "content": [
            {"type": "image", "url": "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"},
            {"type": "text", "text": "Describe this image."},
        ],
    },
]
inputs = processor.apply_chat_template(
    conversation,
    add_generation_prompt=True,
    tokenize=True,
    return_dict=True,
    return_tensors="pt",
).to(model.device)

output = model.generate(**inputs, max_new_tokens=100)
print(processor.decode(output[0], skip_special_tokens=True))
```

## Granite4VisionConfig[[transformers.Granite4VisionConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **image_token_index** (`int`, *optional*, defaults to `32000`) --
  The image token index used as a placeholder for input images.
- **vision_feature_select_strategy** (`Literal[default, full]`, *optional*, defaults to `default`) --
  The feature selection strategy used to select the vision feature from the vision backbone.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*, defaults to `-2`) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **image_grid_pinpoints** (`list`, *optional*) --
  A list of possible resolutions to use for processing high resolution images. Each item in the list should be a
  tuple or list of the form `(height, width)`.
- **image_seq_length** (`int`, *optional*, defaults to `576`) --
  Sequence length of one image embedding.
- **downsample_rate** (`str`, *optional*) --
  Fractional downsample rate for the Window Q-Former projector, e.g. `"1/4"` or `"3/8"`.
  The numerator is the query window side, the denominator is the key window side.
- **deepstack_layer_map** (`list`, *optional*) --
  List of `[vision_layer_idx, llm_layer_idx]` pairs. Features from each vision encoder layer
  are projected and injected at the corresponding LLM decoder layer during forward pass.
- **spatial_vision_layer** (`int`, *optional*, defaults to `-1`) --
  Index of the vision encoder layer used for spatial sampling.
- **spatial_target_layers** (`list`, *optional*, defaults to `[12, 15, 18, 21]`) --
  Target LLM layers for the 4 spatial offset groups.
- **projector_dropout** (`float`, *optional*, defaults to `0.1`) --
  Dropout probability in the Window Q-Former projector.
- **qformer_config** (`dict` or `Blip2QFormerConfig`, *optional*) --
  Configuration for the Window Q-Former projector. If `None`, defaults are derived from
  `vision_config.hidden_size`.

This is the configuration class to store the configuration of a Granite4VisionModel. It is used to instantiate a Granite4 Vision
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [llava-hf/llava-v1.6-mistral-7b-hf](https://huggingface.co/llava-hf/llava-v1.6-mistral-7b-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Granite4VisionTextConfig[[transformers.Granite4VisionTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `32000`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `11008`) --
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
- **pad_token_id** (`int`, *optional*) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `False`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.
- **rope_parameters** (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) --
  Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain
  a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE
  with longer `max_position_embeddings`.
- **attention_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in the query, key, value and output projection layers during self-attention.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **mlp_bias** (`bool`, *optional*, defaults to `False`) --
  Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.
- **embedding_multiplier** (`Union[float, int]`, *optional*, defaults to `1.0`) --
  Scaling factor applied to the word embeddings. Used to scale the embeddings relative to the hidden size.
- **logits_scaling** (`Union[float, int]`, *optional*, defaults to `1.0`) --
  Scaling factor applied to the output logits before computing the probability distribution.
- **residual_multiplier** (`Union[float, int]`, *optional*, defaults to `1.0`) --
  Scaling factor applied to the residual connections.
- **attention_multiplier** (`Union[float, int]`, *optional*, defaults to `1.0`) --
  Scaling factor applied to the attention weights.

This is the configuration class to store the configuration of a Granite4VisionModel. It is used to instantiate a Granite4 Vision
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ibm-granite4_vision_text/granite4_vision_text-3.0-8b-base](https://huggingface.co/ibm-granite4_vision_text/granite4_vision_text-3.0-8b-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import Granite4VisionTextModel, Granite4VisionTextConfig

>>> # Initializing a Granite4VisionText granite4_vision_text-3b style configuration
>>> configuration = Granite4VisionTextConfig()

>>> # Initializing a model from the granite4_vision_text-7b style configuration
>>> model = Granite4VisionTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Granite4VisionProcessor[[transformers.Granite4VisionProcessor]]

'"}, {"name": "num_additional_image_tokens", "val": " = 0"}, {"name": "downsample_rate", "val": " = None"}, {"name": "**kwargs", "val": ""}]}>
- **image_processor** (`LlavaNextImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **patch_size** (`int`, *optional*) --
  Patch size from the vision tower.
- **vision_feature_select_strategy** (`str`, *optional*) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Should be same as in model's config.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **image_token** (`str`, *optional*, defaults to `"<image>"`) --
  Special token used to denote image location.
- **num_additional_image_tokens** (`int`, *optional*, defaults to `0`) --
  Number of additional tokens added to the image embeddings, such as CLS (+1).
- **downsample_rate** (`str`, *optional*) --
  Fractional downsample rate (e.g. `"1/4"`), used to adjust the number of image tokens
  when computing token counts for padding/truncation.
Constructs a Granite4VisionProcessor which wraps a image processor and a tokenizer into a single processor.

[Granite4VisionProcessor](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionProcessor) offers all the functionalities of [LlavaNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextImageProcessor) and `tokenizer_class`. See the
[~LlavaNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextImageProcessor) and `~tokenizer_class` for more information.

- **images** (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) --
  Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If
  passing in images with pixel values between 0 and 1, set `do_rescale=False`.
- **text** (`Union[str, list[str], list[list[str]]]`, *optional*) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.
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

## Granite4VisionModel[[transformers.Granite4VisionModel]]

- **config** ([Granite4VisionConfig](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Llava-Next model which consists of a vision backbone and a language model without language modeling head.

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
  [LlavaNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextImageProcessor). See `LlavaNextImageProcessor.__call__()` for details ([Granite4VisionProcessor](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionProcessor) uses
  [LlavaNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextImageProcessor) for processing images).
- **image_sizes** (`torch.LongTensor` of shape `(batch_size, 2)`, *optional*) --
  The sizes of the images in the batch, being (height, width) for each image.
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
- **vision_feature_select_strategy** (`str`, *optional*, defaults to `"default"`) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`. If `"default"`, the CLS token is removed from the vision features.
  If `"full"`, the full vision features are used.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).`Granite4VisionModelOutputWithPast` or `tuple(torch.FloatTensor)`A `Granite4VisionModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Granite4VisionConfig](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionConfig)) and inputs.
The [Granite4VisionModel](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionModel) forward method, overrides the `__call__` special method.

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
- **image_hidden_states** (`torch.FloatTensor` of shape `(batch_size, num_images, sequence_length, hidden_size)`, *optional*) -- Image hidden states of the model produced by the vision encoder and after projecting the last hidden state.
- **deepstack_features** (`list[tuple[int, list[torch.Tensor]]]`, *optional*) -- List of `(llm_layer_idx, packed_features)` pairs produced by the deepstack
  and spatial projectors. Each entry targets one LLM decoder layer; `packed_features`
  is a per-image list of tensors of shape `(num_image_tokens, hidden_size)`.

)>"}, {"name": "vision_feature_layer", "val": ": int | list[int] | None = None"}, {"name": "vision_feature_select_strategy", "val": ": str | None = None"}, {"name": "output_hidden_states", "val": ": bool | None = None"}, {"name": "**kwargs", "val": ""}]}>
- **pixel_values** (`torch.FloatTensor]` of shape `(batch_size, num_patches, channels, height, width)`) --
  The tensors corresponding to the input images.
- **image_sizes** (`torch.Tensor` of shape `(num_images, 2)`) --
  Actual image size of each images (H, W).
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **image_sizes** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, 2)`) --
  The sizes of the images in the batch, being (height, width) for each image.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **vision_feature_select_strategy** (`str`, *optional*) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.`Granite4VisionImageFeaturesOutput` or `tuple(torch.FloatTensor)`A `Granite4VisionImageFeaturesOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Granite4VisionConfig](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionConfig)) and inputs.
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
- **deepstack_features** (`list[tuple[int, list[torch.Tensor]]]`, *optional*) -- List of `(llm_layer_idx, packed_features)` pairs produced by the deepstack
  and spatial projectors. Each entry targets one LLM decoder layer; `packed_features`
  is a per-image list of tensors of shape `(num_image_tokens, hidden_size)`.

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

Reshape, unpad and then pack each image_feature into a single image_features tensor containing all visual vectors.

Overrides the parent to apply downsample_rate to height/width calculations.

## Granite4VisionTextModel[[transformers.Granite4VisionTextModel]]

- **config** ([Granite4VisionTextConfig](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Granite4 Vision Text Model outputting raw hidden-states without any specific head on to.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>] | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
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
- **vision_mask** (`torch.BoolTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Boolean mask marking image token positions. Required when `deepstack_features` is provided.
- **deepstack_features** (`dict[int, torch.Tensor]`, *optional*) --
  Mapping from LLM layer index to projected vision features of shape `(num_image_tokens, hidden_size)`.
  Features are added into image-token positions of hidden states before the corresponding decoder layer.[BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPast](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Granite4VisionConfig](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionConfig)) and inputs.
The [Granite4VisionTextModel](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionTextModel) forward method, overrides the `__call__` special method.

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

## Granite4VisionForConditionalGeneration[[transformers.Granite4VisionForConditionalGeneration]]

- **config** ([Granite4VisionConfig](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The LLAVA-NeXT model which consists of a vision backbone and a language model.

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
  [LlavaNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextImageProcessor). See `LlavaNextImageProcessor.__call__()` for details ([Granite4VisionProcessor](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionProcessor) uses
  [LlavaNextImageProcessor](/docs/transformers/v5.14.0/en/model_doc/llava_next#transformers.LlavaNextImageProcessor) for processing images).
- **image_sizes** (`torch.LongTensor` of shape `(batch_size, 2)`, *optional*) --
  The sizes of the images in the batch, being (height, width) for each image.
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
- **vision_feature_select_strategy** (`str`, *optional*, defaults to `"default"`) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`. If `"default"`, the CLS token is removed from the vision features.
  If `"full"`, the full vision features are used.
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).`Granite4VisionCausalLMOutputWithPast` or `tuple(torch.FloatTensor)`A `Granite4VisionCausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Granite4VisionConfig](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionConfig)) and inputs.
The [Granite4VisionForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
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
- **image_hidden_states** (`torch.FloatTensor` of shape `(batch_size, num_images, sequence_length, hidden_size)`, *optional*) -- Image hidden states of the model produced by the vision encoder and after projecting the last hidden state.
- **deepstack_features** (`list[tuple[int, list[torch.Tensor]]]`, *optional*) -- List of `(llm_layer_idx, packed_features)` pairs produced by the deepstack
  and spatial projectors. Each entry targets one LLM decoder layer; `packed_features`
  is a per-image list of tensors of shape `(num_image_tokens, hidden_size)`.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, Granite4VisionForConditionalGeneration

>>> model = Granite4VisionForConditionalGeneration.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")
>>> processor = AutoProcessor.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")

>>> prompt = "[INST] <image>\nWhat is shown in this image? [/INST]"
>>> url = "https://www.ilankelman.org/stopsigns/australia.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, text=prompt, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(**inputs, max_length=30)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"[INST]  \nWhat is shown in this image? [/INST] The image appears to be a radar chart, which is a type of multi-dimensional plot (...)"
```

)>"}, {"name": "vision_feature_layer", "val": ": int | list[int] | None = None"}, {"name": "vision_feature_select_strategy", "val": ": str | None = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`torch.FloatTensor]` of shape `(batch_size, num_patches, channels, height, width)`) --
  The tensors corresponding to the input images.
- **image_sizes** (`torch.Tensor` of shape `(num_images, 2)`) --
  Actual image size of each images (H, W).
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **image_sizes** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, 2)`) --
  The sizes of the images in the batch, being (height, width) for each image.
- **vision_feature_layer** (`Union[int, list[int]]`, *optional*) --
  The index of the layer to select the vision feature. If multiple indices are provided,
  the vision feature of the corresponding indices will be concatenated to form the
  vision features.
- **vision_feature_select_strategy** (`str`, *optional*) --
  The feature selection strategy used to select the vision feature from the vision backbone.
  Can be one of `"default"` or `"full"`[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Granite4VisionConfig](/docs/transformers/v5.14.0/en/model_doc/granite4_vision#transformers.Granite4VisionConfig)) and inputs.

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
>>> from transformers import AutoProcessor, Granite4VisionForConditionalGeneration

>>> model = Granite4VisionForConditionalGeneration.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")
>>> processor = AutoProcessor.from_pretrained("llava-hf/llava-v1.6-mistral-7b-hf")

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
