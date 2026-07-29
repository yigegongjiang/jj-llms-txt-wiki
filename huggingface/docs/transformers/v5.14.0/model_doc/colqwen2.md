# ColQwen2

[ColQwen2](https://huggingface.co/papers/2407.01449) is a variant of the [ColPali](./colpali) model designed to retrieve documents by analyzing their visual features. Unlike traditional systems that rely heavily on text extraction and OCR, ColQwen2 treats each page as an image. It uses the [Qwen2-VL](./qwen2_vl) backbone to capture not only text, but also the layout, tables, charts, and other visual elements to create detailed multi-vector embeddings that can be used for retrieval by computing pairwise late interaction similarity scores. This offers a more comprehensive understanding of documents and enables more efficient and accurate retrieval.

This model was contributed by [@tonywu71](https://huggingface.co/tonywu71) (ILLUIN Technology) and [@yonigozlan](https://huggingface.co/yonigozlan) (HuggingFace).

You can find all the original ColPali checkpoints under Vidore's [Hf-native ColVision Models](https://huggingface.co/collections/vidore/hf-native-colvision-models-6755d68fc60a8553acaa96f7) collection.

> [!TIP]
> Click on the ColQwen2 models in the right sidebar for more examples of how to use ColQwen2 for image retrieval.

```python
import requests
import torch
from PIL import Image

from transformers import ColQwen2ForRetrieval, ColQwen2Processor
from transformers.utils.import_utils import is_flash_attn_2_available

# Load the model and the processor
model_name = "vidore/colqwen2-v1.0-hf"

model = ColQwen2ForRetrieval.from_pretrained(
    model_name,
    device_map="auto",  # "cpu", "cuda", "xpu" or "mps" for Apple Silicon
    attn_implementation="flash_attention_2" if is_flash_attn_2_available() else "sdpa",
)
processor = ColQwen2Processor.from_pretrained(model_name)

# The document page screenshots from your corpus
url1 = "https://upload.wikimedia.org/wikipedia/commons/8/89/US-original-Declaration-1776.jpg"
url2 = "https://upload.wikimedia.org/wikipedia/commons/thumb/4/4c/Romeoandjuliet1597.jpg/500px-Romeoandjuliet1597.jpg"

images = [
    Image.open(requests.get(url1, stream=True).raw),
    Image.open(requests.get(url2, stream=True).raw),
]

# The queries you want to retrieve documents for
queries = [
    "When was the United States Declaration of Independence proclaimed?",
    "Who printed the edition of Romeo and Juliet?",
]

# Process the inputs
inputs_images = processor(images=images).to(model.device)
inputs_text = processor(text=queries).to(model.device)

# Forward pass
with torch.no_grad():
    image_embeddings = model(**inputs_images).embeddings
    query_embeddings = model(**inputs_text).embeddings

# Score the queries against the images
scores = processor.score_retrieval(query_embeddings, image_embeddings)

print("Retrieval scores (query x image):")
print(scores)
```

If you have issue with loading the images with PIL, you can use the following code to create dummy images:

```python
images = [
    Image.new("RGB", (128, 128), color="white"),
    Image.new("RGB", (64, 32), color="black"),
]
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to quantize the weights to int4.

```python
import requests
import torch
from PIL import Image

from transformers import BitsAndBytesConfig, ColQwen2ForRetrieval, ColQwen2Processor

model_name = "vidore/colqwen2-v1.0-hf"

# 4-bit quantization configuration
bnb_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_use_double_quant=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_compute_dtype=torch.float16,
)

model = ColQwen2ForRetrieval.from_pretrained(
    model_name,
    quantization_config=bnb_config,
    device_map="auto",
).eval()

processor = ColQwen2Processor.from_pretrained(model_name)

url1 = "https://upload.wikimedia.org/wikipedia/commons/8/89/US-original-Declaration-1776.jpg"
url2 = "https://upload.wikimedia.org/wikipedia/commons/thumb/4/4c/Romeoandjuliet1597.jpg/500px-Romeoandjuliet1597.jpg"

images = [
    Image.open(requests.get(url1, stream=True).raw),
    Image.open(requests.get(url2, stream=True).raw),
]

queries = [
    "When was the United States Declaration of Independence proclaimed?",
    "Who printed the edition of Romeo and Juliet?",
]

# Process the inputs
inputs_images = processor(images=images, return_tensors="pt").to(model.device)
inputs_text = processor(text=queries, return_tensors="pt").to(model.device)

# Forward pass
with torch.no_grad():
    image_embeddings = model(**inputs_images).embeddings
    query_embeddings = model(**inputs_text).embeddings

# Score the queries against the images
scores = processor.score_retrieval(query_embeddings, image_embeddings)

print("Retrieval scores (query x image):")
print(scores)
```

You can also use checkpoints for `ColQwen2.5` that are **compatible with the ColQwen2 architecture**. This version of the model uses [Qwen2_5_VL](./qwen2_5_vl) as the backbone.

```python
from transformers import ColQwen2ForRetrieval, ColQwen2Processor
from transformers.utils.import_utils import is_flash_attn_2_available

model_name = "Sahil-Kabir/colqwen2.5-v0.2-hf" # An existing compatible checkpoint

model = ColQwen2ForRetrieval.from_pretrained(
    model_name,
    device_map="auto",
    attn_implementation="flash_attention_2" if is_flash_attn_2_available() else "sdpa"
)
processor = ColQwen2Processor.from_pretrained(model_name)
```

## Notes

- `score_retrieval()` returns a 2D tensor where the first dimension is the number of queries and the second dimension is the number of images. A higher score indicates more similarity between the query and image.
- Unlike ColPali, ColQwen2 supports arbitrary image resolutions and aspect ratios, which means images are not resized into fixed-size squares. This preserves more of the original input signal.
- Larger input images generate longer multi-vector embeddings, allowing users to adjust image resolution to balance performance and memory usage.

## ColQwen2Config[[transformers.ColQwen2Config]]

- **vlm_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision-language backbone.
- **embedding_dim** (`int`, *optional*, defaults to `128`) --
  Dimensionality of the embeddings and hidden states.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Colqwen2Model. It is used to instantiate a Colqwen2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [vidore/colqwen2-v1.0-hf](https://huggingface.co/vidore/colqwen2-v1.0-hf)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
from transformers.models.colqwen2 import ColQwen2Config, ColQwen2ForRetrieval

config = ColQwen2Config()
model = ColQwen2ForRetrieval(config)
```

## ColQwen2Processor[[transformers.ColQwen2Processor]]

- **image_processor** (`Qwen2VLImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`Qwen2Tokenizer`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **visual_prompt_prefix** (`str`, *optional*, defaults to `"<|im_start|>user\n<|vision_start|><|image_pad|><|vision_end|>Describe the image.<|im_end|><|endoftext|>"`) --
  A string that gets tokenized and prepended to the image tokens.
- **query_prefix** (`str`, *optional*, defaults to `"Query -- "`):
  A prefix to be used for the query.
Constructs a ColQwen2Processor which wraps a image processor and a tokenizer into a single processor.

[ColQwen2Processor](/docs/transformers/v5.14.0/en/model_doc/colqwen2#transformers.ColQwen2Processor) offers all the functionalities of [Qwen2VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor) and [Qwen2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer). See the
[~Qwen2VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor) and [~Qwen2Tokenizer](/docs/transformers/v5.14.0/en/model_doc/qwen2#transformers.Qwen2Tokenizer) for more information.

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

- **input_ids** -- List of token ids to be fed to a model.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).
- **pixel_values** -- Pixel values to be fed to a model. Returned when `images` is not `None`.

## ColQwen2ForRetrieval[[transformers.ColQwen2ForRetrieval]]

- **config** ([ColQwen2Config](/docs/transformers/v5.14.0/en/model_doc/colqwen2#transformers.ColQwen2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Following the ColPali approach, ColQwen2 leverages VLMs to construct efficient multi-vector embeddings directly
from document images (“screenshots”) for document retrieval. The model is trained to maximize the similarity
between these document embeddings and the corresponding query embeddings, using the late interaction method
introduced in ColBERT.

Using ColQwen2 removes the need for potentially complex and brittle layout recognition and OCR pipelines with
a single model that can take into account both the textual and visual content (layout, charts, ...) of a document.

ColQwen2 is part of the ColVision model family, which was introduced with ColPali in the following paper:
[*ColPali: Efficient Document Retrieval with Vision Language Models*](https://huggingface.co/papers/2407.01449).

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
- **labels** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Labels for computing the masked language modeling loss. Indices should either be in `[0, ...,
  config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored
  (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.
- **inputs_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.
- **use_cache** (`bool`, *optional*) --
  If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see
  `past_key_values`).
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Qwen2VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor). See `Qwen2VLImageProcessor.__call__()` for details ([ColQwen2Processor](/docs/transformers/v5.14.0/en/model_doc/colqwen2#transformers.ColQwen2Processor) uses
  [Qwen2VLImageProcessor](/docs/transformers/v5.14.0/en/model_doc/qwen2_vl#transformers.Qwen2VLImageProcessor) for processing images).
- **image_grid_thw** (`torch.LongTensor` of shape `(num_images, 3)`, *optional*) --
  The temporal, height and width of feature shape of each image in LLM.`ColQwen2ForRetrievalOutput` or `tuple(torch.FloatTensor)`A `ColQwen2ForRetrievalOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ColQwen2Config](/docs/transformers/v5.14.0/en/model_doc/colqwen2#transformers.ColQwen2Config)) and inputs.
The [ColQwen2ForRetrieval](/docs/transformers/v5.14.0/en/model_doc/colqwen2#transformers.ColQwen2ForRetrieval) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **embeddings** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- The embeddings of the model.
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

Example:

```python
```
