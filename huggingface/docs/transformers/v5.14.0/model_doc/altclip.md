# AltCLIP

[AltCLIP](https://huggingface.co/papers/2211.06679) replaces the [CLIP](./clip) text encoder with a multilingual XLM-R encoder and aligns image and text representations with teacher learning and contrastive learning.

You can find all the original AltCLIP checkpoints under the [AltClip](https://huggingface.co/collections/BAAI/alt-clip-diffusion-66987a97de8525205f1221bf) collection.

> [!TIP]
> Click on the AltCLIP models in the right sidebar for more examples of how to apply AltCLIP to different tasks.

The examples below demonstrates how to calculate similarity scores between an image and one or more captions with the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
import requests
from PIL import Image

from transformers import AltCLIPModel, AltCLIPProcessor

model = AltCLIPModel.from_pretrained("BAAI/AltCLIP", device_map="auto")
processor = AltCLIPProcessor.from_pretrained("BAAI/AltCLIP")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)

inputs = processor(text=["a photo of a cat", "a photo of a dog"], images=image, return_tensors="pt", padding=True).to(model.device)

outputs = model(**inputs)
logits_per_image = outputs.logits_per_image  # this is the image-text similarity score
probs = logits_per_image.softmax(dim=1)  # we can take the softmax to get the label probabilities

labels = ["a photo of a cat", "a photo of a dog"]
for label, prob in zip(labels, probs[0]):
    print(f"{label}: {prob.item():.4f}")
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [torchao](../quantization/torchao) to only quantize the weights to int4.

```python
# !pip install torchao
import requests
from PIL import Image

from transformers import AltCLIPModel, AltCLIPProcessor, TorchAoConfig

model = AltCLIPModel.from_pretrained(
    "BAAI/AltCLIP",
    quantization_config=TorchAoConfig("int4_weight_only", group_size=128),
    device_map="auto",
)

processor = AltCLIPProcessor.from_pretrained("BAAI/AltCLIP")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)

inputs = processor(text=["a photo of a cat", "a photo of a dog"], images=image, return_tensors="pt", padding=True).to(model.device)

outputs = model(**inputs)
logits_per_image = outputs.logits_per_image  # this is the image-text similarity score
probs = logits_per_image.softmax(dim=1)  # we can take the softmax to get the label probabilities

labels = ["a photo of a cat", "a photo of a dog"]
for label, prob in zip(labels, probs[0]):
    print(f"{label}: {prob.item():.4f}")
```

## Notes

- AltCLIP uses bidirectional attention instead of causal attention and it uses the `[CLS]` token in XLM-R to represent a text embedding.
- Use [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) to resize (or rescale) and normalize images for the model.
- [AltCLIPProcessor](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPProcessor) combines [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) and [XLMRobertaTokenizer](/docs/transformers/v5.14.0/en/model_doc/xlm-roberta#transformers.XLMRobertaTokenizer) into a single instance to encode text and prepare images.

## AltCLIPConfig[[transformers.AltCLIPConfig]]

- **text_config** (`Union[dict, ~models.altclip.configuration_altclip.AltCLIPTextConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~models.altclip.configuration_altclip.AltCLIPVisionConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **projection_dim** (`int`, *optional*, defaults to `768`) --
  Dimensionality of text and vision projection layers.
- **logit_scale_init_value** (`Union[float, int]`, *optional*, defaults to `2.6592`) --
  The initial value of the *logit_scale* parameter.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).

This is the configuration class to store the configuration of a AltCLIPModel. It is used to instantiate a Altclip
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [BAAI/AltCLIP](https://huggingface.co/BAAI/AltCLIP)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import AltCLIPConfig, AltCLIPModel

>>> # Initializing a AltCLIPConfig with BAAI/AltCLIP style configuration
>>> configuration = AltCLIPConfig()

>>> # Initializing a AltCLIPModel (with random weights) from the BAAI/AltCLIP style configuration
>>> model = AltCLIPModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a AltCLIPConfig from a AltCLIPTextConfig and a AltCLIPVisionConfig

>>> # Initializing a AltCLIPText and AltCLIPVision configuration
>>> config_text = AltCLIPTextConfig()
>>> config_vision = AltCLIPVisionConfig()

>>> config = AltCLIPConfig(text_config=config_text, vision_config=config_vision)
```

## AltCLIPTextConfig[[transformers.AltCLIPTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `250002`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `4096`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **max_position_embeddings** (`int`, *optional*, defaults to `514`) --
  The maximum sequence length that this model might ever be used with.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **initializer_factor** (`float`, *optional*, defaults to `0.02`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).
- **pad_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`int`, *optional*, defaults to `2`) --
  Token id used for end-of-stream in the vocabulary.
- **hidden_dropout_prob** (`Union[int, float]`, *optional*, defaults to `0.1`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[int, float]`, *optional*, defaults to `0`) --
  The dropout ratio for the attention probabilities.
- **type_vocab_size** (`int`, *optional*, defaults to `1`) --
  The vocabulary size of the `token_type_ids`.
- **project_dim** (`int`, *optional*, defaults to 768) --
  The dimensions of the teacher model before the mapping layer.

This is the configuration class to store the configuration of a AltCLIPModel. It is used to instantiate a Altclip
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [BAAI/AltCLIP](https://huggingface.co/BAAI/AltCLIP)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import AltCLIPTextModel, AltCLIPTextConfig

>>> # Initializing a AltCLIPTextConfig with BAAI/AltCLIP style configuration
>>> configuration = AltCLIPTextConfig()

>>> # Initializing a AltCLIPTextModel (with random weights) from the BAAI/AltCLIP style configuration
>>> model = AltCLIPTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## AltCLIPVisionConfig[[transformers.AltCLIPVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **projection_dim** (`int`, *optional*, defaults to `512`) --
  Dimensionality of text and vision projection layers.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `32`) --
  The size (resolution) of each patch.
- **hidden_act** (`str`, *optional*, defaults to `quick_gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[int, float]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).

This is the configuration class to store the configuration of a AltCLIPModel. It is used to instantiate a Altclip
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [BAAI/AltCLIP](https://huggingface.co/BAAI/AltCLIP)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import AltCLIPVisionConfig, AltCLIPVisionModel

>>> # Initializing a AltCLIPVisionConfig with BAAI/AltCLIP style configuration
>>> configuration = AltCLIPVisionConfig()

>>> # Initializing a AltCLIPVisionModel (with random weights) from the BAAI/AltCLIP style configuration
>>> model = AltCLIPVisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## AltCLIPModel[[transformers.AltCLIPModel]]

- **config** ([AltCLIPConfig](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Altclip Model outputting raw hidden-states without any specific head on top.

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
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor). See `CLIPImageProcessor.__call__()` for details ([AltCLIPProcessor](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPProcessor) uses
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **return_loss** (`bool`, *optional*) --
  Whether or not to return the contrastive loss.
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.`AltCLIPOutput` or `tuple(torch.FloatTensor)`A `AltCLIPOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AltCLIPConfig](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPConfig)) and inputs.
The [AltCLIPModel](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Contrastive loss for image-text similarity.
- **logits_per_image** (`torch.FloatTensor` of shape `(image_batch_size, text_batch_size)`) -- The scaled dot product scores between `image_embeds` and `text_embeds`. This represents the image-text
  similarity scores.
- **logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, image_batch_size)`) -- The scaled dot product scores between `text_embeds` and `image_embeds`. This represents the text-image
  similarity scores.
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The text embeddings obtained by applying the projection layer to the pooled output of [AltCLIPTextModel](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPTextModel).
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The image embeddings obtained by applying the projection layer to the pooled output of [AltCLIPVisionModel](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPVisionModel).
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [AltCLIPTextModel](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPTextModel).
- **vision_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [AltCLIPVisionModel](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPVisionModel).

Examples:

```python
>>> import torch
>>> from transformers import AutoProcessor, AltCLIPModel
>>> from transformers.image_utils import load_image

>>> model = AltCLIPModel.from_pretrained("OFA-Sys/chinese-clip-vit-base-patch16")
>>> processor = AutoProcessor.from_pretrained("OFA-Sys/chinese-clip-vit-base-patch16")

>>> url = "https://clip-cn-beijing.oss-cn-beijing.aliyuncs.com/pokemon.jpeg"
>>> image = load_image(url)

>>> inputs = processor(text=["杰尼龟", "妙蛙种子", "小火龙", "皮卡丘"], images=image, return_tensors="pt", padding=True)

>>> with torch.inference_mode():
...     outputs = model(**inputs)
>>> logits_per_image = outputs.logits_per_image  # this is the image-text similarity score
>>> probs = logits_per_image.softmax(dim=1)  # we can take the softmax to get the label probabilities
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor). See `CLIPImageProcessor.__call__()` for details ([AltCLIPProcessor](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPProcessor) uses
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) for processing images).
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AltCLIPConfig](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPConfig)) and inputs.

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

Examples:

```python
>>> import torch
>>> from transformers import AutoProcessor, AltCLIPModel
>>> from transformers.image_utils import load_image

>>> model = AltCLIPModel.from_pretrained("BAAI/AltCLIP")
>>> processor = AutoProcessor.from_pretrained("BAAI/AltCLIP")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = load_image(url)

>>> inputs = processor(images=image, return_tensors="pt")
>>> with torch.inference_mode():
...     image_features = model.get_image_features(**inputs)
```

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "token_type_ids", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "position_ids", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **input_ids** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AltCLIPConfig](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPConfig)) and inputs.

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

Examples:

```python
>>> import torch
>>> from transformers import AutoProcessor, AltCLIPModel

>>> model = AltCLIPModel.from_pretrained("BAAI/AltCLIP")
>>> processor = AutoProcessor.from_pretrained("BAAI/AltCLIP")

>>> inputs = processor(text=["a photo of a cat", "a photo of a dog"], padding=True, return_tensors="pt")
>>> with torch.inference_mode():
...     text_features = model.get_text_features(**inputs)
```

## AltCLIPTextModel[[transformers.AltCLIPTextModel]]

- **input_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and
  [PreTrainedTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.

  [What are input IDs?](../glossary#input-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **token_type_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0, 1]`:

  - 0 corresponds to a *sentence A* token,
  - 1 corresponds to a *sentence B* token.

  [What are token type IDs?](../glossary#token-type-ids)
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **inputs_embeds** (`torch.Tensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This
  is useful if you want more control over how to convert `input_ids` indices into associated vectors than the
  model's internal embedding lookup matrix.`BaseModelOutputWithPoolingAndProjection` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithPoolingAndProjection` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AltCLIPConfig](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPConfig)) and inputs.
The [AltCLIPTextModel](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

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
- **projection_state** (`tuple(torch.FloatTensor)`, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` of shape `(batch_size,config.project_dim)`.

  Text embeddings before the projection layer, used to mimic the last hidden state of the teacher encoder.

Examples:

```python
>>> from transformers import AutoProcessor, AltCLIPTextModel

>>> model = AltCLIPTextModel.from_pretrained("BAAI/AltCLIP")
>>> processor = AutoProcessor.from_pretrained("BAAI/AltCLIP")

>>> texts = ["it's a cat", "it's a dog"]

>>> inputs = processor(text=texts, padding=True, return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooled_output = outputs.pooler_output  # pooled CLS states
```

## AltCLIPVisionModel[[transformers.AltCLIPVisionModel]]

- **config** ([AltCLIPVisionConfig](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPVisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The vision model from ALTCLIP without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor). See `CLIPImageProcessor.__call__()` for details ([AltCLIPProcessor](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPProcessor) uses
  [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) for processing images).
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AltCLIPConfig](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPConfig)) and inputs.
The [AltCLIPVisionModel](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

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

Examples:

```python
>>> import httpx
>>> from io import BytesIO
>>> from PIL import Image
>>> from transformers import AutoProcessor, AltCLIPVisionModel

>>> model = AltCLIPVisionModel.from_pretrained("BAAI/AltCLIP")
>>> processor = AutoProcessor.from_pretrained("BAAI/AltCLIP")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooled_output = outputs.pooler_output  # pooled CLS states
```

## AltCLIPProcessor[[transformers.AltCLIPProcessor]]

- **image_processor** (`CLIPImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`tokenizer_class`) --
  The tokenizer is a required input.
Constructs a AltCLIPProcessor which wraps a image processor and a tokenizer into a single processor.

[AltCLIPProcessor](/docs/transformers/v5.14.0/en/model_doc/altclip#transformers.AltCLIPProcessor) offers all the functionalities of [CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) and `tokenizer_class`. See the
[~CLIPImageProcessor](/docs/transformers/v5.14.0/en/model_doc/clip#transformers.CLIPImageProcessor) and `~tokenizer_class` for more information.

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
