# ALIGN

[ALIGN](https://huggingface.co/papers/2102.05918) is pretrained on a noisy 1.8 billion alt‑text and image pair dataset to show that scale can make up for the noise. It uses a dual‑encoder architecture, [EfficientNet](./efficientnet) for images and [BERT](./bert) for text, and a contrastive loss to align similar image–text embeddings together while pushing different embeddings apart. Once trained, ALIGN can encode any image and candidate captions into a shared vector space for zero‑shot retrieval or classification without requiring extra labels. This scale‑first approach reduces dataset curation costs and powers state‑of‑the‑art image–text retrieval and zero‑shot ImageNet classification.

You can find all the original ALIGN checkpoints under the [Kakao Brain](https://huggingface.co/kakaobrain?search_models=align) organization.

> [!TIP]
> Click on the ALIGN models in the right sidebar for more examples of how to apply ALIGN to different vision and text related tasks.

The example below demonstrates zero-shot image classification with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

  

```python
from transformers import pipeline

pipeline = pipeline(
    task="zero-shot-image-classification",
    model="kakaobrain/align-base",
    device=0,
)

candidate_labels = [
    "a photo of a dog",
    "a photo of a cat",
    "a photo of a person"
]

pipeline("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg", candidate_labels=candidate_labels)
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoModelForZeroShotImageClassification, AutoProcessor

processor = AutoProcessor.from_pretrained("kakaobrain/align-base")
model = AutoModelForZeroShotImageClassification.from_pretrained("kakaobrain/align-base", device_map="auto")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = requests.get(url, stream=True)
inputs = Image.open(image.raw).convert("RGB")

image_inputs = processor(images=inputs, return_tensors="pt").to(model.device)
with torch.no_grad():
    image_embeds = model.get_image_features(**image_inputs)

candidate_labels = ["a photo of a dog", "a photo of a cat", "a photo of a person"]
text_inputs = processor(text=candidate_labels, padding=True, return_tensors="pt").to(model.device)
with torch.no_grad():
    text_embeds = model.get_text_features(**text_inputs)

image_embeds = image_embeds / image_embeds.norm(p=2, dim=-1, keepdim=True)
text_embeds  = text_embeds  / text_embeds.norm(p=2, dim=-1, keepdim=True)

logits = (image_embeds @ text_embeds.T) * 100.0
probs  = logits.softmax(dim=-1).cpu().squeeze()

for label, score in zip(candidate_labels, probs):
    print(f"{label:20s} → {score.item():.4f}")
```

## Notes

- ALIGN projects the text and visual features into latent space and the dot product between the projected image and text features is used as the similarity score. The example below demonstrates how to calculate the image-text similarity score with [AlignProcessor](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignProcessor) and [AlignModel](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignModel).

  ```py
  # Example of using ALIGN for image-text similarity
  from transformers import AlignProcessor, AlignModel
  import torch
  from PIL import Image
  import requests
  from io import BytesIO
  
  # Load processor and model
  processor = AlignProcessor.from_pretrained("kakaobrain/align-base")
  model = AlignModel.from_pretrained("kakaobrain/align-base", device_map="auto")
  
  # Download image from URL
  url = "https://huggingface.co/roschmid/dog-races/resolve/main/images/Golden_Retriever.jpg"
  response = requests.get(url)
  image = Image.open(BytesIO(response.content))  # Convert the downloaded bytes to a PIL Image
  
  texts = ["a photo of a cat", "a photo of a dog"]
  
  # Process image and text inputs
  inputs = processor(images=image, text=texts, return_tensors="pt").to(model.device)
  
  # Get the embeddings
  with torch.no_grad():
      outputs = model(**inputs)
  
  image_embeds = outputs.image_embeds
  text_embeds = outputs.text_embeds
  
  # Normalize embeddings for cosine similarity
  image_embeds = image_embeds / image_embeds.norm(dim=1, keepdim=True)
  text_embeds = text_embeds / text_embeds.norm(dim=1, keepdim=True)
  
  # Calculate similarity scores
  similarity_scores = torch.matmul(text_embeds, image_embeds.T)
  
  # Print raw scores
  print("Similarity scores:", similarity_scores)
  
  # Convert to probabilities
  probs = torch.nn.functional.softmax(similarity_scores, dim=0)
  print("Probabilities:", probs)
  
  # Get the most similar text
  most_similar_idx = similarity_scores.argmax().item()
  print(f"Most similar text: '{texts[most_similar_idx]}'")
  ```

## Resources

- Refer to the [Kakao Brain’s Open Source ViT, ALIGN, and the New COYO Text-Image Dataset](https://huggingface.co/blog/vit-align) blog post for more details.

## AlignConfig[[transformers.AlignConfig]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **projection_dim** (`int`, *optional*, defaults to `640`) --
  Dimensionality of text and vision projection layers.
- **temperature_init_value** (`float`, *optional*, defaults to 1.0) --
  The initial value of the *temperature* parameter. Default is used as per the original ALIGN implementation.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a AlignModel. It is used to instantiate a Align
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [kakaobrain/align-base](https://huggingface.co/kakaobrain/align-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import AlignConfig, AlignModel

>>> # Initializing a AlignConfig with kakaobrain/align-base style configuration
>>> configuration = AlignConfig()

>>> # Initializing a AlignModel (with random weights) from the kakaobrain/align-base style configuration
>>> model = AlignModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a AlignConfig from a AlignTextConfig and a AlignVisionConfig
>>> from transformers import AlignTextConfig, AlignVisionConfig

>>> # Initializing ALIGN Text and Vision configurations
>>> config_text = AlignTextConfig()
>>> config_vision = AlignVisionConfig()

>>> config = AlignConfig(text_config=config_text, vision_config=config_vision)
```

## AlignTextConfig[[transformers.AlignTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `30522`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The dropout ratio for the attention probabilities.
- **max_position_embeddings** (`int`, *optional*, defaults to `512`) --
  The maximum sequence length that this model might ever be used with.
- **type_vocab_size** (`int`, *optional*, defaults to `2`) --
  The vocabulary size of the `token_type_ids`.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-12`) --
  The epsilon used by the layer normalization layers.
- **pad_token_id** (`int`, *optional*, defaults to `0`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*) --
  Token id used for end-of-stream in the vocabulary.

This is the configuration class to store the configuration of a AlignModel. It is used to instantiate a Align
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [kakaobrain/align-base](https://huggingface.co/kakaobrain/align-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import AlignTextConfig, AlignTextModel

>>> # Initializing a AlignTextConfig with kakaobrain/align-base style configuration
>>> configuration = AlignTextConfig()

>>> # Initializing a AlignTextModel (with random weights) from the kakaobrain/align-base style configuration
>>> model = AlignTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## AlignVisionConfig[[transformers.AlignVisionConfig]]

- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `600`) --
  The size (resolution) of each image.
- **width_coefficient** (`float`, *optional*, defaults to 2.0) --
  Scaling coefficient for network width at each stage.
- **depth_coefficient** (`float`, *optional*, defaults to 3.1) --
  Scaling coefficient for network depth at each stage.
- **depth_divisor** (`int`, *optional*, defaults to 8) --
  A unit of network width.
- **kernel_sizes** (`list[int]`, *optional*, defaults to `[3, 3, 5, 3, 5, 5, 3]`) --
  List of kernel sizes to be used in each block.
- **in_channels** (`list[int]`, *optional*, defaults to `[32, 16, 24, 40, 80, 112, 192]`) --
  List of input channel sizes to be used in each block for convolutional layers.
- **out_channels** (`list[int]`, *optional*, defaults to `[16, 24, 40, 80, 112, 192, 320]`) --
  List of output channel sizes to be used in each block for convolutional layers.
- **depthwise_padding** (`list[int]`, *optional*, defaults to `[]`) --
  List of block indices with square padding.
- **strides** (`list[int]`, *optional*, defaults to `[1, 2, 2, 2, 1, 2, 1]`) --
  List of stride sizes to be used in each block for convolutional layers.
- **num_block_repeats** (`list[int]`, *optional*, defaults to `[1, 2, 2, 3, 3, 4, 1]`) --
  List of the number of times each block is to repeated.
- **expand_ratios** (`list[int]`, *optional*, defaults to `[1, 6, 6, 6, 6, 6, 6]`) --
  List of scaling coefficient of each block.
- **squeeze_expansion_ratio** (`float`, *optional*, defaults to 0.25) --
  Squeeze expansion ratio.
- **hidden_act** (`str`, *optional*, defaults to `swish`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dim** (`int`, *optional*, defaults to 1280) --
  The hidden dimension of the layer before the classification head.
- **pooling_type** (`str` or `function`, *optional*, defaults to `"mean"`) --
  Type of final pooling to be applied before the dense classification head. Available options are [`"mean"`,
  `"max"`]
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **batch_norm_eps** (`float`, *optional*, defaults to `0.001`) --
  The epsilon used by the batch normalization layers.
- **batch_norm_momentum** (`float`, *optional*, defaults to 0.99) --
  The momentum used by the batch normalization layers.
- **drop_connect_rate** (`float`, *optional*, defaults to 0.2) --
  The drop rate for skip connections.

This is the configuration class to store the configuration of a AlignModel. It is used to instantiate a Align
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [kakaobrain/align-base](https://huggingface.co/kakaobrain/align-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import AlignVisionConfig, AlignVisionModel

>>> # Initializing a AlignVisionConfig with kakaobrain/align-base style configuration
>>> configuration = AlignVisionConfig()

>>> # Initializing a AlignVisionModel (with random weights) from the kakaobrain/align-base style configuration
>>> model = AlignVisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## AlignProcessor[[transformers.AlignProcessor]]

- **image_processor** (`EfficientNetImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`BertTokenizer`) --
  The tokenizer is a required input.
Constructs a AlignProcessor which wraps a image processor and a tokenizer into a single processor.

[AlignProcessor](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignProcessor) offers all the functionalities of [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor) and [BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer). See the
[~EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor) and [~BertTokenizer](/docs/transformers/v5.14.0/en/model_doc/electra#transformers.BertTokenizer) for more information.

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

## AlignModel[[transformers.AlignModel]]

- **config** ([AlignConfig](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Align Model outputting raw hidden-states without any specific head on top.

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
  [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor). See `EfficientNetImageProcessor.__call__()` for details ([AlignProcessor](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignProcessor) uses
  [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor) for processing images).
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
  model's internal embedding lookup matrix.
- **return_loss** (`bool`, *optional*) --
  Whether or not to return the contrastive loss.`AlignOutput` or `tuple(torch.FloatTensor)`A `AlignOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AlignConfig](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignConfig)) and inputs.
The [AlignModel](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Contrastive loss for image-text similarity.
- **logits_per_image** (`torch.FloatTensor` of shape `(image_batch_size, text_batch_size)`) -- The scaled dot product scores between `image_embeds` and `text_embeds`. This represents the image-text
  similarity scores.
- **logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, image_batch_size)`) -- The scaled dot product scores between `text_embeds` and `image_embeds`. This represents the text-image
  similarity scores.
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The text embeddings obtained by applying the projection layer to the pooled output of [AlignTextModel](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignTextModel).
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The output of [AlignVisionModel](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignVisionModel).
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [AlignTextModel](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignTextModel).
- **vision_model_output** (`~modeling_outputs.BaseModelOutputWithPoolingAndNoAttention`, *optional*) -- The output of the [AlignVisionModel](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignVisionModel).

Examples:

```python
>>> import torch
>>> from transformers import AutoProcessor, AlignModel
>>> from transformers.image_utils import load_image

>>> model = AlignModel.from_pretrained("kakaobrain/align-base")
>>> processor = AutoProcessor.from_pretrained("kakaobrain/align-base")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = load_image(url)

>>> inputs = processor(
...     images=image, text=["a photo of a cat", "a photo of a dog"], return_tensors="pt", padding=True
... )

>>> with torch.inference_mode():
...     outputs = model(**inputs)
>>> logits_per_image = outputs.logits_per_image  # this is the image-text similarity score
>>> probs = logits_per_image.softmax(dim=1)  # we can take the softmax to get the label probabilities
```

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
  model's internal embedding lookup matrix.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AlignConfig](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignConfig)) and inputs.

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
>>> from transformers import AutoTokenizer, AlignModel

>>> model = AlignModel.from_pretrained("kakaobrain/align-base")
>>> tokenizer = AutoTokenizer.from_pretrained("kakaobrain/align-base")

>>> inputs = tokenizer(["a photo of a cat", "a photo of a dog"], padding=True, return_tensors="pt")
>>> with torch.inference_mode():
...     text_features = model.get_text_features(**inputs)
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor). See `EfficientNetImageProcessor.__call__()` for details ([AlignProcessor](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignProcessor) uses
  [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor) for processing images).[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AlignConfig](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignConfig)) and inputs.

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
>>> from transformers import AutoProcessor, AlignModel
>>> from transformers.image_utils import load_image

>>> model = AlignModel.from_pretrained("kakaobrain/align-base")
>>> processor = AutoProcessor.from_pretrained("kakaobrain/align-base")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = load_image(url)

>>> inputs = processor(images=image, return_tensors="pt")
>>> with torch.inference_mode():
...     image_features = model.get_image_features(**inputs)
```

## AlignTextModel[[transformers.AlignTextModel]]

- **config** ([AlignTextConfig](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.
- **add_pooling_layer** (`bool`, *optional*, defaults to `True`) --
  Whether to add a pooling layer

The text model from ALIGN without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

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
  model's internal embedding lookup matrix.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AlignConfig](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignConfig)) and inputs.
The [AlignTextModel](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignTextModel) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, AlignTextModel

>>> model = AlignTextModel.from_pretrained("kakaobrain/align-base")
>>> tokenizer = AutoTokenizer.from_pretrained("kakaobrain/align-base")

>>> inputs = tokenizer(["a photo of a cat", "a photo of a dog"], padding=True, return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooled_output = outputs.pooler_output  # pooled (EOS token) states
```

## AlignVisionModel[[transformers.AlignVisionModel]]

- **config** ([AlignVisionConfig](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignVisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The vision model from ALIGN without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor). See `EfficientNetImageProcessor.__call__()` for details ([AlignProcessor](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignProcessor) uses
  [EfficientNetImageProcessor](/docs/transformers/v5.14.0/en/model_doc/efficientnet#transformers.EfficientNetImageProcessor) for processing images).`BaseModelOutputWithPoolingAndNoAttention` or `tuple(torch.FloatTensor)`A `BaseModelOutputWithPoolingAndNoAttention` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([AlignConfig](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignConfig)) and inputs.
The [AlignVisionModel](/docs/transformers/v5.14.0/en/model_doc/align#transformers.AlignVisionModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_channels, height, width)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, hidden_size)`) -- Last layer hidden-state after a pooling operation on the spatial dimensions.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, num_channels, height, width)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.

Examples:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, AlignVisionModel

>>> model = AlignVisionModel.from_pretrained("kakaobrain/align-base")
>>> processor = AutoProcessor.from_pretrained("kakaobrain/align-base")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooled_output = outputs.pooler_output  # pooled CLS states
```
