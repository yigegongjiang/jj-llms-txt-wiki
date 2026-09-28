# NeoMME

[![Hugging Face](https://img.shields.io/badge/Collection-FFD21E?style=for-the-badge&logo=huggingface&logoColor=000)](https://huggingface.co/collections/Hcompany/neomme)
[![arXiv](https://img.shields.io/badge/arXiv-coming_soon-b31b1b.svg?style=for-the-badge)](https://arxiv.org)

NeoMME is a family of efficient 260M and 800M parameter multimodal-native multilingual foundation encoders from H Company. It processes multilingual text tokens and raw image patches in a single bidirectional Transformer encoder, without a separately pretrained vision tower or causal language model.

NeoMME-Retriever is a model fine-tuned from the NeoMME backbone for visual document retrieval with joint late-interaction and dense objectives. It takes text queries and documents (text or page screenshots) and produces multi-vector embeddings for MeanMaxSim scoring (late-interaction) and mean-pooled embeddings for cosine similarity (dense).

The pretrained backbones and retrieval checkpoints are available under Apache 2.0 in the [NeoMME collection](https://huggingface.co/collections/Hcompany/neomme) and can be used with [Sentence Transformers](https://huggingface.co/sentence-transformers).

## Example usage

**Generate encoder hidden states**

```python
import requests
import torch
from PIL import Image

from transformers import AutoModel, AutoProcessor

def encode_document_text(processor, text: str) -> str:
    return f"{processor.tokenizer.document_token}{text}"

model_id = "Hcompany/NeoMME-260M"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModel.from_pretrained(model_id, device_map="auto")

text = "The cat sat on a mat."
image_url = "https://github.com/tonywu71/colpali-cookbooks/blob/main/examples/data/shift_kazakhstan.jpg?raw=true"
image = Image.open(requests.get(image_url, stream=True).raw)

inputs = processor(
    text=[
        encode_document_text(processor, text),
        encode_document_text(processor, processor.image_token),
    ],
    images=[image],
    padding=True,
    return_tensors="pt",
).to(model.device)

with torch.inference_mode():
    outputs = model(**inputs)

text_hidden_states, image_hidden_states = outputs.last_hidden_state
```

**Masked language modeling**

```python
import torch

from transformers import AutoModelForMaskedLM, AutoProcessor

model_id = "Hcompany/NeoMME-260M"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForMaskedLM.from_pretrained(model_id, device_map="auto")

# Equivalent: "<doc>The capital of <mask> is London."
text = f"{processor.tokenizer.document_token}The capital of {processor.tokenizer.mask_token} is London."
inputs = processor(text=[text], return_tensors="pt").to(model.device)

with torch.inference_mode():
    outputs = model(**inputs)

masked_index = (inputs.input_ids[0] == processor.tokenizer.mask_token_id).nonzero().item()
predicted_token_id = outputs.logits[0, masked_index].argmax(dim=-1)
print(processor.tokenizer.decode(predicted_token_id))
```

**Visual document retrieval**

> [!IMPORTANT]
> Install `sentence-transformers>=6.0.0` to use MeanMaxSim scoring in the retrieval example below. For the
> Sentence Transformers API, see the [Multi-Vector Encoder quickstart](https://sbert.net/docs/quickstart.html#multi-vector-encoder).

```python
from typing import Any, Literal

import requests
import torch
from PIL import Image
from sentence_transformers.util import cos_sim, mean_maxsim

from transformers import BatchFeature, NeoMMEForRetrieval, NeoMMEProcessor

def encode(
    messages: list[list[dict[str, Any]]],
    task: Literal["query", "document"],
) -> BatchFeature:
    return processor.apply_chat_template(
        messages,
        task=task,
        tokenize=True,
        return_dict=True,
        return_tensors="pt",
        processor_kwargs={"padding": "longest"},
    )

model_name = "Hcompany/NeoMME-260M-Retriever"
processor = NeoMMEProcessor.from_pretrained(model_name)
model = NeoMMEForRetrieval.from_pretrained(model_name)

# Document images (our corpus)
image_urls = [
    "https://github.com/tonywu71/colpali-cookbooks/blob/6ef1332da6bcb48c7ef1f19b25bfa555be7031a8/examples/data/shift_kazakhstan.jpg?raw=true",
    "https://github.com/tonywu71/colpali-cookbooks/blob/6ef1332da6bcb48c7ef1f19b25bfa555be7031a8/examples/data/energy_electricity_generation.jpg?raw=true",
]
documents = [Image.open(requests.get(url, stream=True).raw) for url in image_urls]

# Queries
queries = [
    "Quelle partie de la production pétrolière du Kazakhstan provient de champs en mer ?",
    "Which hour of the day had the highest overall electricity generation in 2019?",
]

document_messages = [
    [{"role": "user", "content": [{"type": "image", "image": document}]}] for document in documents
]
query_messages = [[{"role": "user", "content": query}] for query in queries]

inputs_documents = encode(document_messages, "document").to(model.device)
inputs_text = encode(query_messages, "query").to(model.device)

with torch.inference_mode():
    document_outputs = model(**inputs_documents)
    query_outputs = model(**inputs_text)

late_scores = mean_maxsim(
    query_outputs.embeddings,
    document_outputs.embeddings,
    a_mask=inputs_text["attention_mask"],
    b_mask=inputs_documents["attention_mask"],
)
dense_scores = cos_sim(query_outputs.dense_embeddings, document_outputs.dense_embeddings)

# Expected: late_scores[0, 0] > late_scores[0, 1] and late_scores[1, 1] > late_scores[1, 0].
print(late_scores, dense_scores)
```

## NeoMMEConfig[[transformers.NeoMMEConfig]]

#### transformers.NeoMMEConfig[[transformers.NeoMMEConfig]]

```python
transformers.NeoMMEConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 131072, embedding_rank: int = 256, hidden_size: int = 1024, intermediate_size: int = 3584, hidden_act: typing.Literal['relu2'] = 'relu2', mlp_bias: bool = False, num_hidden_layers: int = 17, num_attention_heads: int = 16, num_key_value_heads: int = 4, head_dim: int = 64, max_position_embeddings: int = 16384, norm_eps: float = 1e-06, initializer_range: float = 0.02, attention_dropout: float | int = 0.0, attention_bias: bool = False, layer_types: list[str] | None = None, rope_parameters: dict[typing.Literal['full_attention', 'sliding_attention'], dict] | None = None, sliding_window: int | None = 256, residual_multiplier: float | None = None, patch_size: int = 32, embedding_dim: int = 128, pad_token_id: int | None = 0, document_token_id: int | None = 5, image_token_id: int | None = 6, tie_word_embeddings: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/configuration_neomme.py#L33)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `131072`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

embedding_rank (`int`, *optional*, defaults to 256) : Width of the factorized token embedding table before projection to `hidden_size`.

hidden_size (`int`, *optional*, defaults to `1024`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `3584`) : Dimension of the MLP representations.

hidden_act (`Literal[relu2]`, *optional*, defaults to `relu2`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

num_hidden_layers (`int`, *optional*, defaults to `17`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `16`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `4`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

head_dim (`int`, *optional*, defaults to `64`) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

max_position_embeddings (`int`, *optional*, defaults to `16384`) : The maximum sequence length that this model might ever be used with.

norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

layer_types (`list[str]`, *optional*) : By default, every sixth layer and the final layer use full attention.

rope_parameters (`dict`, *optional*) : Rotary-position settings for `"full_attention"` and `"sliding_attention"` layers. The rotated dimensions, `head_dim * partial_rotary_factor`, must be a positive multiple of four.

sliding_window (`int`, *optional*, defaults to `256`) : Sliding window attention window size. If `None`, no sliding window is applied.

residual_multiplier (`float`, *optional*) : Scale applied to attention and MLP residual branches. Defaults to `1 / sqrt(2 * num_hidden_layers)`.

patch_size (`int`, *optional*, defaults to `32`) : The size (resolution) of each patch.

embedding_dim (`int`, *optional*, defaults to 128) : Width of the token-level embeddings returned by [NeoMMEForRetrieval](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEForRetrieval). This setting is unrelated to `embedding_rank`.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

document_token_id (`int`, *optional*, defaults to 5) : Token ID for the `<doc>` marker.

image_token_id (`int`, *optional*, defaults to `6`) : The image token index used as a placeholder for input images.

tie_word_embeddings (`bool`, *optional*, defaults to `True`) : Whether the masked token decoder reuses both factorized token embedding weights.

This is the configuration class to store the configuration of a NeoMMEModel. It is used to instantiate a Neomme
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [Hcompany/NeoMME-260M](https://huggingface.co/Hcompany/NeoMME-260M)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.17.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

```python
>>> from transformers import NeoMMEModel, NeoMMEConfig

>>> configuration = NeoMMEConfig()
>>> model = NeoMMEModel(configuration)
```

## NeoMMEImageProcessor[[transformers.NeoMMEImageProcessor]]

#### transformers.NeoMMEImageProcessor[[transformers.NeoMMEImageProcessor]]

```python
transformers.NeoMMEImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/image_processing_neomme.py#L107)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `None`) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*, defaults to `[0.5, 0.5, 0.5]`) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

patch_size (`int`, *kwargs*, *optional*, defaults to `self.patch_size`) : Side, in pixels, of one patch token. The image is padded to a whole multiple of it.

max_side (`int`, *kwargs*, *optional*) : Longest-side cap in pixels. Unset means no longest-side resize.

Constructs a NeoMMEImageProcessor image processor.

#### preprocess[[transformers.NeoMMEImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/image_processing_neomme.py#L136)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

do_convert_rgb (`bool`, *kwargs*, *optional*) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*) : Whether to normalize the image.

image_mean (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image mean to use for normalization. Only has an effect if `do_normalize` is set to `True`.

image_std (`Union[float, list[float], tuple[float, ...]]`, *kwargs*, *optional*) : Image standard deviation to use for normalization. Only has an effect if `do_normalize` is set to `True`.

do_pad (`bool`, *kwargs*, *optional*) : Whether to pad the image. Padding is done either to the largest size in the batch or to a fixed square size per image. The exact padding strategy depends on the model.

pad_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : The size in `{"height": int, "width" int}` to pad the images to. Must be larger than any image size provided for preprocessing. If `pad_size` is not provided, images will be padded to the largest height and width in the batch. Applied only when `do_pad=True.`

do_center_crop (`bool`, *kwargs*, *optional*) : Whether to center crop the image.

data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : Only `ChannelDimension.FIRST` is supported. Added for compatibility with slow processors.

input_data_format (`Union[str, ~image_utils.ChannelDimension]`, *kwargs*, *optional*) : The channel dimension format for the input image. If unset, the channel dimension format is inferred from the input image. Can be one of: - `"channels_first"` or `ChannelDimension.FIRST`: image in (num_channels, height, width) format. - `"channels_last"` or `ChannelDimension.LAST`: image in (height, width, num_channels) format. - `"none"` or `ChannelDimension.NONE`: image in (height, width) format.

device (`Annotated[Union[str, torch.device, NoneType], None]`, *kwargs*) : The device to process the videos on. If unset, the device is inferred from the input videos.

return_tensors (`Annotated[str | ~utils.generic.TensorType | None, None]`, *kwargs*) : Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

patch_size (`int`, *kwargs*, *optional*, defaults to `self.patch_size`) : Side, in pixels, of one patch token. The image is padded to a whole multiple of it.

max_side (`int`, *kwargs*, *optional*) : Longest-side cap in pixels. Unset means no longest-side resize.

**Returns:**

[BatchFeature](/docs/transformers/v5.17.0/en/main_classes/image_processor#transformers.BatchFeature) with `pixel_values` of shape `(total_patches, 3 * patch_size ** 2)` and
`image_grid_hw` of shape `(batch_size, 2)`. `pixel_values` concatenates patches from every image in the
batch.

## NeoMMEProcessor[[transformers.NeoMMEProcessor]]

#### transformers.NeoMMEProcessor[[transformers.NeoMMEProcessor]]

```python
transformers.NeoMMEProcessor(image_processor = None, tokenizer = None, chat_template = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/processing_neomme.py#L32)

**Parameters:**

image_processor (`NeoMMEImageProcessor`) : The image processor is a required input.

tokenizer (`tokenizer_class`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

Constructs a NeoMMEProcessor which wraps a image processor and a tokenizer into a single processor.

[NeoMMEProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEProcessor) offers all the functionalities of [NeoMMEImageProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEImageProcessor) and `tokenizer_class`. See the
[~NeoMMEImageProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEImageProcessor) and `~tokenizer_class` for more information.

#### __call__[[transformers.NeoMMEProcessor.__call__]]

```python
__call__(images: ImageInput | None = None, text: TextInput | list[TextInput] | None = None, **kwargs: Unpack[ProcessingKwargs])
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/processing_neomme.py#L53)

**Parameters:**

images (`ImageInput`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`TextInput | list[TextInput]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

return_tensors (`str` or [TensorType](/docs/transformers/v5.17.0/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

**Returns:**

A [BatchFeature](/docs/transformers/v5.17.0/en/main_classes/image_processor#transformers.BatchFeature) with `input_ids` and `attention_mask`. Image inputs also return `position_ids`,
and `pixel_values`.

#### apply_chat_template[[transformers.NeoMMEProcessor.apply_chat_template]]

```python
apply_chat_template(conversation: list[dict[str, str]] | list[list[dict[str, str]]], chat_template: str | None = None, task: Literal['query', 'document'] | None = None, processor_kwargs: dict | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/processing_neomme.py#L136)

Apply the configured retrieval template and optionally tokenize its output.

When `tokenize=True`, image content must include an image, URL, path, or base64 value. Pass processing
options such as `max_length` or `max_side` through `processor_kwargs`.

## NeoMMEModel[[transformers.NeoMMEModel]]

#### transformers.NeoMMEModel[[transformers.NeoMMEModel]]

```python
transformers.NeoMMEModel(config: NeoMMEConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/modeling_neomme.py#L498)

**Parameters:**

config ([NeoMMEConfig](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare NeoMME model. It encodes text tokens and image patches with one bidirectional Transformer encoder.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.NeoMMEModel.forward]]

```python
forward(input_ids: LongTensor, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/modeling_neomme.py#L541)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(2, batch_size, sequence_length)` or `(batch_size, sequence_length)`, *optional*) : Positions for the input tokens. [NeoMMEProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEProcessor) returns two-axis positions for document images. A one-axis position tensor is used for text inputs.

pixel_values (`torch.Tensor` of shape `(num_patches, 3 * patch_size ** 2)`, *optional*) : Flattened image patches returned by [NeoMMEProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEProcessor). The model places these patches at image placeholders in `input_ids`.

**Returns:** [BaseModelOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`

A [BaseModelOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([NeoMMEConfig](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEConfig)) and inputs.

The [NeoMMEModel](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## NeoMMEForMaskedLM[[transformers.NeoMMEForMaskedLM]]

#### transformers.NeoMMEForMaskedLM[[transformers.NeoMMEForMaskedLM]]

```python
transformers.NeoMMEForMaskedLM(config: NeoMMEConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/modeling_neomme.py#L618)

**Parameters:**

config ([NeoMMEConfig](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The NeoMME model with a factorized masked token decoder.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.NeoMMEForMaskedLM.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.LongTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/modeling_neomme.py#L631)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [NeoMMEImageProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEImageProcessor). See `NeoMMEImageProcessor.__call__()` for details ([NeoMMEProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEProcessor) uses [NeoMMEImageProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEImageProcessor) for processing images).

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for the masked-language-modeling loss. Indices should be in `[0, ..., config.vocab_size - 1]` or `-100`; only tokens with a label different from `-100` contribute.

**Returns:** [MaskedLMOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or `tuple(torch.FloatTensor)`

A [MaskedLMOutput](/docs/transformers/v5.17.0/en/main_classes/output#transformers.modeling_outputs.MaskedLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([NeoMMEConfig](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEConfig)) and inputs.

The [NeoMMEForMaskedLM](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEForMaskedLM) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Masked language modeling (MLM) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoTokenizer, NeoMMEForMaskedLM
>>> import torch

>>> tokenizer = AutoTokenizer.from_pretrained("Hcompany/NeoMME-260M")
>>> model = NeoMMEForMaskedLM.from_pretrained("Hcompany/NeoMME-260M")

>>> inputs = tokenizer("The capital of France is <mask>.", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> # retrieve index of <mask>
>>> mask_token_index = (inputs.input_ids == tokenizer.mask_token_id)[0].nonzero(as_tuple=True)[0]

>>> predicted_token_id = logits[0, mask_token_index].argmax(axis=-1)
>>> tokenizer.decode(predicted_token_id)
...

>>> labels = tokenizer("The capital of France is Paris.", return_tensors="pt")["input_ids"]
>>> # mask labels of non-<mask> tokens
>>> labels = torch.where(inputs.input_ids == tokenizer.mask_token_id, labels, -100)

>>> outputs = model(**inputs, labels=labels)
>>> round(outputs.loss.item(), 2)
...
```

## NeoMMEForRetrieval[[transformers.NeoMMEForRetrieval]]

#### transformers.NeoMMEForRetrieval[[transformers.NeoMMEForRetrieval]]

```python
transformers.NeoMMEForRetrieval(config: NeoMMEConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/modeling_neomme.py#L724)

**Parameters:**

config ([NeoMMEConfig](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The NeoMME model with multi-vector and dense retrieval heads. One forward pass can return token embeddings for
MaxSim scoring and mean-pooled embeddings for cosine similarity.

This model inherits from [PreTrainedModel](/docs/transformers/v5.17.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.NeoMMEForRetrieval.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.Tensor] = None, output_multivector: bool = True, output_dense: bool = True, dense_dim: int | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.17.0/src/transformers/models/neomme/modeling_neomme.py#L732)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.17.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.17.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [NeoMMEImageProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEImageProcessor). See `NeoMMEImageProcessor.__call__()` for details ([NeoMMEProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEProcessor) uses [NeoMMEImageProcessor](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEImageProcessor) for processing images).

output_multivector (`bool`, *optional*, defaults to `True`) : Whether to return token embeddings for late-interaction retrieval.

output_dense (`bool`, *optional*, defaults to `True`) : Whether to return one mean-pooled dense embedding per input.

dense_dim (`int`, *optional*) : Width of the Matryoshka prefix to return for dense embeddings. The model truncates the pooled vector before normalizing it.

**Returns:** `NeoMMEForRetrievalOutput` or `tuple(torch.FloatTensor)`

A `NeoMMEForRetrievalOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([NeoMMEConfig](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEConfig)) and inputs.

The [NeoMMEForRetrieval](/docs/transformers/v5.17.0/en/model_doc/neomme#transformers.NeoMMEForRetrieval) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*) -- Retrieval loss. This value is always `None`.
- **embeddings** (`torch.FloatTensor` of shape `(batch_size, sequence_length, embedding_dim)`, *optional*) -- Normalized token embeddings for late-interaction retrieval. Padding rows are zeroed. Score them with MeanMaxSim.
- **dense_embeddings** (`torch.FloatTensor` of shape `(batch_size, hidden_size)` or `(batch_size, dense_dim)`, *optional*) -- A normalized mean-pooled embedding for each input. When `dense_dim` is set, the last dimension is `dense_dim`.
  Score them with cosine similarity.
