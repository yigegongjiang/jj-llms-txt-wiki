# ColModernVBert

## Overview

ColModernVBert is a model for efficient visual document retrieval. It leverages [ModernVBert](modernvbert) to construct multi-vector embeddings directly from document images, following the ColPali approach.

The model was introduced in [ModernVBERT: Towards Smaller Visual Document Retrievers](https://huggingface.co/papers/2510.01149).

```python
import torch
from huggingface_hub import hf_hub_download
from PIL import Image

from transformers import ColModernVBertForRetrieval, ColModernVBertProcessor

processor = ColModernVBertProcessor.from_pretrained("ModernVBERT/colmodernvbert-hf")
model = ColModernVBertForRetrieval.from_pretrained("ModernVBERT/colmodernvbert-hf", device_map="auto")

# Load the test dataset
queries = [
    "A paint on the wall",
    "ColModernVBERT matches the performance of models nearly 10x larger on visual document benchmarks."
]

images = [
    Image.open(hf_hub_download("HuggingFaceTB/SmolVLM", "example_images/rococo.jpg", repo_type="space")),
    Image.open(hf_hub_download("ModernVBERT/colmodernvbert", "table.png", repo_type="model"))
]

# Preprocess the examples
batch_images = processor(images=images).to(model.device)
batch_queries = processor(text=queries).to(model.device)

# Run inference
with torch.inference_mode():
    image_embeddings = model(**batch_images).embeddings
    query_embeddings = model(**batch_queries).embeddings

# Compute retrieval scores
scores = processor.score_retrieval(
    query_embeddings=query_embeddings,
    passage_embeddings=image_embeddings,
)

scores = torch.softmax(scores, dim=-1)

print(scores)    # [[0.9350, 0.0650], [0.0015, 0.9985]]
```

## ColModernVBertConfig[[transformers.ColModernVBertConfig]]

- **vlm_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision-language backbone.
- **embedding_dim** (`int`, *optional*, defaults to `128`) --
  Dimensionality of the embeddings and hidden states.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a ColmodernvbertModel. It is used to instantiate a Colmodernvbert
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [ModernVBERT/colmodernvbert-merged](https://huggingface.co/ModernVBERT/colmodernvbert-merged)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
from transformers import ColModernVBertConfig, ColModernVBertForRetrieval

config = ColModernVBertConfig()
model = ColModernVBertForRetrieval(config)
```

## ColModernVBertProcessor[[transformers.ColModernVBertProcessor]]

- **image_processor** (`image_processor_class`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
- **chat_template** (`str`) --
  A Jinja template to convert lists of messages in a chat into a tokenizable string.
- **image_seq_len** (`int`, *optional*, defaults to 64) --
  The length of the image sequence i.e. the number of  tokens per image in the input.
- **visual_prompt_prefix** (`str`, *optional*) --
  A string that gets tokenized and prepended to the image tokens.
- **query_prefix** (`str`, *optional*) --
  A prefix to be used for the query.
Constructs a ColModernVBertProcessor which wraps a image processor and a tokenizer into a single processor.

[ColModernVBertProcessor](/docs/transformers/v5.14.0/en/model_doc/colmodernvbert#transformers.ColModernVBertProcessor) offers all the functionalities of `image_processor_class` and `tokenizer_class`. See the
`~image_processor_class` and `~tokenizer_class` for more information.

- **images** (`PIL.Image.Image`, `np.ndarray`, `torch.Tensor`, `list[PIL.Image.Image]`, `list[np.ndarray]`, `list[torch.Tensor]`) --
  The image or batch of images to be prepared. Each image can be a PIL image, NumPy array or PyTorch
  tensor. In case of a NumPy array/PyTorch tensor, each image should be of shape (C, H, W), where C is a
  number of channels, H and W are image height and width.
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:

- **input_ids** -- List of token ids to be fed to a model.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).
- **pixel_values** -- Pixel values to be fed to a model. Returned when `images` is not `None`.

Prepare for the model one or several image(s). Handles input validation, RGB conversion,
and prepends the `visual_prompt_prefix` to each image. Optionally computes labels from
`token_type_ids` when a `suffix` is provided in `text_kwargs`.

- **text** (`str`, `list[str]`, `list[list[str]]`) --
  The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings
  (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set
  `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).
- **return_tensors** (`str` or [TensorType](/docs/transformers/v5.14.0/en/internal/file_utils#transformers.TensorType), *optional*) --
  If set, will return tensors of a particular framework. Acceptable values are:

  - `'pt'`: Return PyTorch `torch.Tensor` objects.
  - `'np'`: Return NumPy `np.ndarray` objects.[BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature)A [BatchFeature](/docs/transformers/v5.14.0/en/main_classes/image_processor#transformers.BatchFeature) with the following fields:

- **input_ids** -- List of token ids to be fed to a model.
- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names` and if `text` is not
  `None`).

Prepare for the model one or several text queries. Handles input validation, prepends the
`query_prefix`, and appends query augmentation tokens (used to pad query embeddings for
better late-interaction retrieval performance).

- **query_embeddings** (`Union[torch.Tensor, list[torch.Tensor]`) -- Query embeddings.
- **passage_embeddings** (`Union[torch.Tensor, list[torch.Tensor]`) -- Passage embeddings.
- **batch_size** (`int`, *optional*, defaults to 128) -- Batch size for computing scores.
- **output_dtype** (`torch.dtype`, *optional*, defaults to `torch.float32`) -- The dtype of the output tensor.
  If `None`, the dtype of the input embeddings is used.
- **output_device** (`torch.device` or `str`, *optional*, defaults to "cpu") -- The device of the output tensor.`torch.Tensor`A tensor of shape `(n_queries, n_passages)` containing the scores. The score
tensor is saved on the "cpu" device.

Compute the late-interaction/MaxSim score (ColBERT-like) for the given multi-vector
query embeddings (`qs`) and passage embeddings (`ps`). For ColQwen2, a passage is the
image of a document page.

Because the embedding tensors are multi-vector and can thus have different shapes, they
should be fed as:
(1) a list of tensors, where the i-th tensor is of shape (sequence_length_i, embedding_dim)
(2) a single tensor of shape (n_passages, max_sequence_length, embedding_dim) -> usually
obtained by padding the list of tensors.

## ColModernVBertForRetrieval[[transformers.ColModernVBertForRetrieval]]

- **config** ([ColModernVBertConfig](/docs/transformers/v5.14.0/en/model_doc/colmodernvbert#transformers.ColModernVBertConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Following the ColPali approach, ColModernVBert leverages VLMs to construct efficient multi-vector embeddings directly
from document images (“screenshots”) for document retrieval. The model is trained to maximize the similarity
between these document embeddings and the corresponding query embeddings, using the late interaction method
introduced in ColBERT.

Using ColModernVBert removes the need for potentially complex and brittle layout recognition and OCR pipelines with
a single model that can take into account both the textual and visual content (layout, charts, ...) of a document.

ColModernVBert is trained on top of ModernVBert, and was introduced in the following paper:
[*ModernVBERT: Towards Smaller Visual Document Retrievers*](https://arxiv.org/abs/2510.01149).

ColModernVBert is part of the ColVision model family, which was introduced with ColPali in the following paper:
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
- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  `image_processor_class`. See `image_processor_class.__call__` for details ([ColModernVBertProcessor](/docs/transformers/v5.14.0/en/model_doc/colmodernvbert#transformers.ColModernVBertProcessor) uses
  `image_processor_class` for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)`ColModernVBertForRetrievalOutput` or `tuple(torch.FloatTensor)`A `ColModernVBertForRetrievalOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([ColModernVBertConfig](/docs/transformers/v5.14.0/en/model_doc/colmodernvbert#transformers.ColModernVBertConfig)) and inputs.
The [ColModernVBertForRetrieval](/docs/transformers/v5.14.0/en/model_doc/colmodernvbert#transformers.ColModernVBertForRetrieval) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **embeddings** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- The embeddings of the model.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **image_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True` and `pixel_values` are provided) -- Tuple of `torch.FloatTensor` (one for the output of the image modality projection + one for the output of each layer) of shape
  `(batch_size, num_channels, image_size, image_size)`.
  Hidden-states of the image encoder at the output of each layer plus the initial modality projection outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
```
