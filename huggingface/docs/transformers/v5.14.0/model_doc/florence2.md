# Florence-2

    
        
    

## Overview

[Florence-2](https://huggingface.co/papers/2311.06242) is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks. Florence-2 can interpret simple text prompts to perform tasks like captioning, object detection, and segmentation. It leverages the FLD-5B dataset, containing 5.4 billion annotations across 126 million images, to master multi-task learning. The model's sequence-to-sequence architecture enables it to excel in both zero-shot and fine-tuned settings, proving to be a competitive vision foundation model.

You can find all the original Florence-2 checkpoints under the [Florence-2](https://huggingface.co/models?other=florence-2) collection.

> [!TIP]
> This model was contributed by [ducviet00](https://huggingface.co/ducviet00).
> Click on the Florence-2 models in the right sidebar for more examples of how to apply Florence-2 to different vision and language tasks.

The example below demonstrates how to perform object detection with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

pipeline = pipeline(
    "image-text-to-text",
    model="florence-community/Florence-2-base",
    device=0,
)

pipeline(
    "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/tasks/car.jpg?download=true",
    text="<OD>"
)
```

```python
import requests
from PIL import Image

from transformers import AutoProcessor, Florence2ForConditionalGeneration

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/tasks/car.jpg?download=true"
image = Image.open(requests.get(url, stream=True).raw).convert("RGB")

model = Florence2ForConditionalGeneration.from_pretrained("florence-community/Florence-2-base", device_map="auto")
processor = AutoProcessor.from_pretrained("florence-community/Florence-2-base")

task_prompt = "<OD>"
inputs = processor(text=task_prompt, images=image, return_tensors="pt").to(model.device)

generated_ids = model.generate(
    **inputs,
    max_new_tokens=1024,
    num_beams=3,
)
generated_text = processor.batch_decode(generated_ids, skip_special_tokens=False)[0]

image_size = image.size
parsed_answer = processor.post_process_generation(generated_text, task=task_prompt, image_size=image_size)
print(parsed_answer)
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to quantize the model to 4-bit.

```python
# pip install bitsandbytes
import requests
import torch
from PIL import Image

from transformers import AutoProcessor, BitsAndBytesConfig, Florence2ForConditionalGeneration

quantization_config = BitsAndBytesConfig(load_in_4bit=True)

model = Florence2ForConditionalGeneration.from_pretrained(
    "florence-community/Florence-2-base",
    device_map="auto",
    quantization_config=quantization_config
)
processor = AutoProcessor.from_pretrained("florence-community/Florence-2-base")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/tasks/car.jpg?download=true"
image = Image.open(requests.get(url, stream=True).raw).convert("RGB")

task_prompt = "<OD>"
inputs = processor(text=task_prompt, images=image, return_tensors="pt").to(model.device, torch.bfloat16)

generated_ids = model.generate(
    **inputs,
    max_new_tokens=1024,
    num_beams=3,
)
generated_text = processor.batch_decode(generated_ids, skip_special_tokens=False)[0]

image_size = image.size
parsed_answer = processor.post_process_generation(generated_text, task=task_prompt, image_size=image_size)

print(parsed_answer)
```

    

## Notes

- Florence-2 is a prompt-based model. You need to provide a task prompt to tell the model what to do. Supported tasks are:
  - `<OCR>`
  - `<OCR_WITH_REGION>`
  - `<CAPTION>`
  - `<DETAILED_CAPTION>`
  - `<MORE_DETAILED_CAPTION>`
  - `<OD>`
  - `<DENSE_REGION_CAPTION>`
  - `<CAPTION_TO_PHRASE_GROUNDING>`
  - `<REFERRING_EXPRESSION_SEGMENTATION>`
  - `<REGION_TO_SEGMENTATION>`
  - `<OPEN_VOCABULARY_DETECTION>`
  - `<REGION_TO_CATEGORY>`
  - `<REGION_TO_DESCRIPTION>`
  - `<REGION_TO_OCR>`
  - `<REGION_PROPOSAL>`
- The raw output of the model is a string that needs to be parsed. The [Florence2Processor](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Processor) has a `post_process_generation()` method that can parse the string into a more usable format, like bounding boxes and labels for object detection.

## Resources

- [Florence-2 technical report](https://huggingface.co/papers/2311.06242)
- [Jupyter Notebook for inference and visualization of Florence-2-large model](https://huggingface.co/microsoft/Florence-2-large/blob/main/sample_inference.ipynb)

## Florence2VisionConfig[[transformers.Florence2VisionConfig]]

- **in_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **depths** (`Tuple[int]`, *optional*, defaults to `(1, 1, 9, 1)`) --
  The depth of the model.
- **patch_size** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(7, 3, 3, 3)`) --
  The size (resolution) of each patch.
- **patch_stride** (`Tuple[int]`, *optional*, defaults to `(4, 2, 2, 2)`) --
  The patch stride of the image.
- **patch_padding** (`Tuple[int]`, *optional*, defaults to `(3, 1, 1, 1)`) --
  The patch padding of the image.
- **patch_prenorm** (`Tuple[bool]`, *optional*, defaults to `(False, True, True, True)`) --
  Whether to apply layer normalization before the patch embedding layer.
- **embed_dim** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(128, 256, 512, 1024)`) --
  Dimensionality of the embeddings and hidden states.
- **num_heads** (`Union[list[int], tuple[int, ...]]`, *optional*, defaults to `(4, 8, 16, 32)`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_groups** (`Tuple[int]`, *optional*, defaults to `(4, 8, 16, 32)`) --
  The number of groups.
- **window_size** (`int`, *optional*, defaults to 12) --
  The window size of the model.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  Drop path rate for the patch fusion.
- **mlp_ratio** (`float`, *optional*, defaults to `4.0`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **activation_function** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **projection_dim** (`int`, *optional*, defaults to `1024`) --
  Dimensionality of text and vision projection layers.
- **max_temporal_embeddings** (`int`, *optional*, defaults to 100) --
  The configuration of the visual temporal embedding.
- **max_position_embeddings** (`int`, *optional*, defaults to `50`) --
  The maximum sequence length that this model might ever be used with.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Florence2Model. It is used to instantiate a Florence2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [florence-community/Florence-2-base](https://huggingface.co/florence-community/Florence-2-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Florence2VisionConfig, Florence2VisionModel

>>> # Initializing a Florence2 Vision style configuration
>>> configuration = Florence2VisionConfig()

>>> # Initializing a model (with random weights)
>>> model = Florence2VisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Florence2Config[[transformers.Florence2Config]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **image_token_id** (`int`, *optional*, defaults to `51289`) --
  The image token index used as a placeholder for input images.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Florence2Model. It is used to instantiate a Florence2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [florence-community/Florence-2-base](https://huggingface.co/florence-community/Florence-2-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Florence2ForConditionalGeneration, Florence2Config, CLIPVisionConfig, BartConfig

>>> # Initializing a clip-like vision config
>>> vision_config = CLIPVisionConfig()

>>> # Initializing a Bart config
>>> text_config = BartConfig()

>>> # Initializing a Florence-2 configuration
>>> configuration = Florence2Config(vision_config, text_config)

>>> # Initializing a model from the florence-2 configuration
>>> model = Florence2ForConditionalGeneration(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Florence2Processor[[transformers.Florence2Processor]]

- **image_processor** (`CLIPImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`BartTokenizer`) --
  The tokenizer is a required input.
- **num_additional_image_tokens** (`int`, *optional*, defaults to 0) --
  Number of additional tokens added to the image embeddings, such as CLS (+1). If the backbone has no CLS or other
  extra tokens appended, no need to set this arg.
- **post_processor_config** (`dict`,  *optional*, defaults to 0) --
  Task-specific parsing rules for `Florence2PostProcessor`, e.g. regex patterns,
  thresholds, or banned tokens.
Constructs a Florence2Processor which wraps a image processor and a tokenizer into a single processor.

[Florence2Processor](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Processor) offers all the functionalities of [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) and [BartTokenizer](/docs/transformers/v5.14.0/en/model_doc/roberta#transformers.RobertaTokenizer). See the
[~CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) and [~BartTokenizer](/docs/transformers/v5.14.0/en/model_doc/roberta#transformers.RobertaTokenizer) for more information.

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

## Florence2Model[[transformers.Florence2Model]]

- **config** ([Florence2Config](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Florence-2 is a vision model for captioning, detection, and segmentation.

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
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor). See `CLIPImageProcessor.__call__()` for details ([Florence2Processor](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Processor) uses
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Indices of decoder input sequence tokens in the vocabulary.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are decoder input IDs?](../glossary#decoder-input-ids)
- **decoder_attention_mask** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to
  make sure the model can only look at previous inputs in order to predict the future.
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded
  representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be
  input (see `past_key_values`). This is useful if you want more control over how to convert
  `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value
  of `inputs_embeds`.
- **encoder_outputs** (`list[torch.FloatTensor]`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
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
  `past_key_values`).`Florence2Seq2SeqModelOutput` or `tuple(torch.FloatTensor)`A `Florence2Seq2SeqModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Florence2Config](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Config)) and inputs.
The [Florence2Model](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the optional initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the optional initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_image_tokens, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

)>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`torch.FloatTensor]` of shape `(batch_size, channels, height, width)`) --
  The tensors corresponding to the input images.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Florence2Config](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Config)) and inputs.
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

## Florence2ForConditionalGeneration[[transformers.Florence2ForConditionalGeneration]]

- **config** ([Florence2Config](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Florence-2 is a vision model for captioning, detection, and segmentation.

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
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor). See `CLIPImageProcessor.__call__()` for details ([Florence2Processor](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Processor) uses
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **decoder_input_ids** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Indices of decoder input sequence tokens in the vocabulary.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are decoder input IDs?](../glossary#decoder-input-ids)
- **decoder_attention_mask** (`torch.LongTensor` of shape `(batch_size, target_sequence_length)`, *optional*) --
  Mask to avoid performing attention on certain token indices. By default, a causal mask will be used, to
  make sure the model can only look at previous inputs in order to predict the future.
- **encoder_outputs** (`list[torch.FloatTensor]`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
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
- **decoder_inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, target_sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `decoder_input_ids` you can choose to directly pass an embedded
  representation. If `past_key_values` is used, optionally only the last `decoder_inputs_embeds` have to be
  input (see `past_key_values`). This is useful if you want more control over how to convert
  `decoder_input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

  If `decoder_input_ids` and `decoder_inputs_embeds` are both unset, `decoder_inputs_embeds` takes the value
  of `inputs_embeds`.
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
  This is useful when using packed tensor format (single dimension for batch and sequence length).`Florence2Seq2SeqLMOutput` or `tuple(torch.FloatTensor)`A `Florence2Seq2SeqLMOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Florence2Config](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Config)) and inputs.
The [Florence2ForConditionalGeneration](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`EncoderDecoderCache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [EncoderDecoderCache](/docs/transformers/v5.14.0/en/internal/generation_utils#transformers.EncoderDecoderCache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention
  blocks) that can be used (see `past_key_values` input) to speed up sequential decoding.
- **decoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **cross_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder's cross-attention layer, after the attention softmax, used to compute the
  weighted average in the cross-attention heads.
- **encoder_last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the encoder at the output of each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the encoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size `(batch_size, num_image_tokens, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, Florence2ForConditionalGeneration

>>> model = Florence2ForConditionalGeneration.from_pretrained("florence-community/Florence-2-large")
>>> processor = AutoProcessor.from_pretrained("florence-community/Florence-2-large")

>>> prompt = "<CAPTION>"
>>> url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/tasks/car.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(text=prompt, images=image, return_tensors="pt")

>>> # Generate
>>> generate_ids = model.generate(**inputs, max_length=100)
>>> processor.batch_decode(generate_ids, skip_special_tokens=True, clean_up_tokenization_spaces=False)[0]
"A green car parked in front of a yellow building."
```

)>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor). See `CLIPImageProcessor.__call__()` for details ([Florence2Processor](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Processor) uses
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) for processing images).[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Florence2Config](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2Config)) and inputs.

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
>>> from transformers import AutoProcessor, Florence2ForConditionalGeneration

>>> model = Florence2ForConditionalGeneration.from_pretrained("florence-community/Florence-2-base")
>>> processor = AutoProcessor.from_pretrained("florence-community/Florence-2-base")

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

## Florence2VisionBackbone[[transformers.Florence2VisionBackbone]]

- **config** ([Florence2VisionConfig](/docs/transformers/v5.14.0/en/model_doc/florence2#transformers.Florence2VisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Florence2 backbone.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "**kwargs", "val": ": Unpack"}]}>
