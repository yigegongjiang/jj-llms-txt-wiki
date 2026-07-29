# SigLIP

[SigLIP](https://huggingface.co/papers/2303.15343) is a multimodal image-text model similar to [CLIP](clip). It uses separate image and text encoders to generate representations for both modalities.

Unlike CLIP, SigLIP employs a pairwise sigmoid loss on image-text pairs during training. This training loss eliminates the need for a global view of all pairwise similarities between images and texts within a batch. Consequently, it enables more efficient scaling to larger batch sizes while also delivering superior performance with smaller batch sizes.

You can find all the original SigLIP checkpoints under the [SigLIP](https://huggingface.co/collections/google/siglip-659d5e62f0ae1a57ae0e83ba) collection.

> [!TIP]
> Click on the SigLIP models in the right sidebar for more examples of how to apply SigLIP to different image and text tasks.

The example below demonstrates how to generate similarity scores between texts and image(s) with [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline) or the [AutoModel](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModel) class.

```python
from transformers import pipeline

image = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
candidate_labels = ["a Pallas cat", "a lion", "a Siberian tiger"]

pipeline = pipeline(task="zero-shot-image-classification", model="google/siglip-base-patch16-224", device=0)
pipeline(image, candidate_labels=candidate_labels)
```

```python
import requests
import torch
from PIL import Image

from transformers import AutoModel, AutoProcessor

model = AutoModel.from_pretrained("google/siglip-base-patch16-224", device_map="auto", attn_implementation="sdpa")
processor = AutoProcessor.from_pretrained("google/siglip-base-patch16-224")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
candidate_labels = ["a Pallas cat", "a lion", "a Siberian tiger"]
texts = [f'This is a photo of {label}.' for label in candidate_labels]
inputs = processor(text=texts, images=image, padding="max_length", return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

logits_per_image = outputs.logits_per_image
probs = torch.sigmoid(logits_per_image)
print(f"{probs[0][0]:.1%} that image 0 is '{candidate_labels[0]}'")
```

Quantization reduces the memory burden of large models by representing the weights in a lower precision. Refer to the [Quantization](../quantization/overview) overview for more available quantization backends.

The example below uses [bitsandbytes](../quantization/bitsandbytes) to only quantize the weights to int4.

```python
import requests
import torch
from PIL import Image

from transformers import AutoModel, AutoProcessor, BitsAndBytesConfig

bnb_config = BitsAndBytesConfig(load_in_4bit=True)
model = AutoModel.from_pretrained("google/siglip-base-patch16-224", quantization_config=bnb_config, device_map="auto", attn_implementation="sdpa")
processor = AutoProcessor.from_pretrained("google/siglip-base-patch16-224")

url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
image = Image.open(requests.get(url, stream=True).raw)
candidate_labels = ["a Pallas cat", "a lion", "a Siberian tiger"]
texts = [f'This is a photo of {label}.' for label in candidate_labels]
inputs = processor(text=texts, images=image, padding="max_length", return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

logits_per_image = outputs.logits_per_image
probs = torch.sigmoid(logits_per_image)
print(f"{probs[0][0]:.1%} that image 0 is '{candidate_labels[0]}'")
```

## Notes

- Training is supported for DDP and FSDP on single-node multi-GPU setups. However, it does not use [torch.distributed](https://pytorch.org/tutorials/beginner/dist_overview.html) utilities which may limit the scalability of batch size.
- When using the standalone [SiglipTokenizer](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipTokenizer) or [SiglipProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipProcessor), make sure to pass `padding="max_length"` because that is how the model was trained.
- To get the same results as the [Pipeline](/docs/transformers/v5.14.0/en/main_classes/pipelines#transformers.Pipeline), a prompt template of `"This is a photo of {label}."` should be passed to the processor.
- Toggle the `attn_implementation` parameter to either `"sdpa"` or `"flash_attention_2"` to use a more memory-efficient attention.

    ```py
    # pip install -U flash-attn --no-build-isolation

    from transformers import SiglipModel

    model = SiglipModel.from_pretrained(
        "google/siglip-so400m-patch14-384",
        attn_implementation="flash_attention_2",
        device_map="auto",
    )
    ```

## SiglipConfig[[transformers.SiglipConfig]]

- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **initializer_factor** (`float`, *optional*, defaults to `1.0`) --
  A factor for initializing all weight matrices (should be kept to 1, used internally for initialization
  testing).

This is the configuration class to store the configuration of a SiglipModel. It is used to instantiate a Siglip
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/siglip-base-patch16-224](https://huggingface.co/google/siglip-base-patch16-224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import SiglipConfig, SiglipModel

>>> # Initializing a SiglipConfig with google/siglip-base-patch16-224 style configuration
>>> configuration = SiglipConfig()

>>> # Initializing a SiglipModel (with random weights) from the google/siglip-base-patch16-224 style configuration
>>> model = SiglipModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config

>>> # We can also initialize a SiglipConfig from a SiglipTextConfig and a SiglipVisionConfig
>>> from transformers import SiglipTextConfig, SiglipVisionConfig

>>> # Initializing a SiglipText and SiglipVision configuration
>>> config_text = SiglipTextConfig()
>>> config_vision = SiglipVisionConfig()

>>> config = SiglipConfig(text_config=config_text, vision_config=config_vision)
```

## SiglipTextConfig[[transformers.SiglipTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `32000`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **max_position_embeddings** (`int`, *optional*, defaults to `64`) --
  The maximum sequence length that this model might ever be used with.
- **hidden_act** (`str`, *optional*, defaults to `gelu_pytorch_tanh`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **pad_token_id** (`int`, *optional*, defaults to `1`) --
  Token id used for padding in the vocabulary.
- **bos_token_id** (`int`, *optional*, defaults to `49406`) --
  Token id used for beginning-of-stream in the vocabulary.
- **eos_token_id** (`Union[int, list[int]]`, *optional*, defaults to `49407`) --
  Token id used for end-of-stream in the vocabulary.
- **projection_size** (`int`, *optional*) --
  Dimensionality of text and vision projection layers.

This is the configuration class to store the configuration of a SiglipModel. It is used to instantiate a Siglip
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/siglip-base-patch16-224](https://huggingface.co/google/siglip-base-patch16-224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import SiglipTextConfig, SiglipTextModel

>>> # Initializing a SiglipTextConfig with google/siglip-base-patch16-224 style configuration
>>> configuration = SiglipTextConfig()

>>> # Initializing a SiglipTextModel (with random weights) from the google/siglip-base-patch16-224 style configuration
>>> model = SiglipTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SiglipVisionConfig[[transformers.SiglipVisionConfig]]

- **hidden_size** (`int`, *optional*, defaults to `768`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `3072`) --
  Dimension of the MLP representations.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `12`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_channels** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **image_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `224`) --
  The size (resolution) of each image.
- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **hidden_act** (`str`, *optional*, defaults to `gelu_pytorch_tanh`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.

This is the configuration class to store the configuration of a SiglipModel. It is used to instantiate a Siglip
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/siglip-base-patch16-224](https://huggingface.co/google/siglip-base-patch16-224)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import SiglipVisionConfig, SiglipVisionModel

>>> # Initializing a SiglipVisionConfig with google/siglip-base-patch16-224 style configuration
>>> configuration = SiglipVisionConfig()

>>> # Initializing a SiglipVisionModel (with random weights) from the google/siglip-base-patch16-224 style configuration
>>> model = SiglipVisionModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SiglipTokenizer[[transformers.SiglipTokenizer]]

'"}, {"name": "unk_token", "val": " = ''"}, {"name": "pad_token", "val": " = ''"}, {"name": "additional_special_tokens", "val": " = None"}, {"name": "sp_model_kwargs", "val": ": dict[str, typing.Any] | None = None"}, {"name": "model_max_length", "val": " = 64"}, {"name": "do_lower_case", "val": " = True"}, {"name": "**kwargs", "val": ""}]}>
- **vocab_file** (`str`) --
  [SentencePiece](https://github.com/google/sentencepiece) file (generally has a *.spm* extension) that
  contains the vocabulary necessary to instantiate a tokenizer.
- **eos_token** (`str`, *optional*, defaults to `"</s>"`) --
  The end of sequence token.
- **unk_token** (`str`, *optional*, defaults to `"<unk>"`) --
  The unknown token. A token that is not in the vocabulary cannot be converted to an ID and is set to be this
  token instead.
- **pad_token** (`str`, *optional*, defaults to `"</s>"`) --
  The token used for padding, for example when batching sequences of different lengths.
- **additional_special_tokens** (`list[str]`, *optional*) --
  Additional special tokens used by the tokenizer.
- **sp_model_kwargs** (`dict`, *optional*) --
  Will be passed to the `SentencePieceProcessor.__init__()` method. The [Python wrapper for
  SentencePiece](https://github.com/google/sentencepiece/tree/master/python) can be used, among other things,
  to set:

  - `enable_sampling`: Enable subword regularization.
  - `nbest_size`: Sampling parameters for unigram. Invalid for BPE-Dropout.

    - `nbest_size = {0,1}`: No sampling is performed.
    - `nbest_size > 1`: samples from the nbest_size results.
    - `nbest_size < 0`: assuming that nbest_size is infinite and samples from the all hypothesis (lattice)
      using forward-filtering-and-backward-sampling algorithm.

  - `alpha`: Smoothing parameter for unigram sampling, and dropout probability of merge operations for
    BPE-dropout.
- **model_max_length** (`int`, *optional*, defaults to 64) --
  The maximum length (in number of tokens) for model inputs.
- **do_lower_case** (`bool`, *optional*, defaults to `True`) --
  Whether or not to lowercase the input when tokenizing.

Construct a Siglip tokenizer. Based on [SentencePiece](https://github.com/google/sentencepiece).

This tokenizer inherits from [PreTrainedTokenizer](/docs/transformers/v5.14.0/en/main_classes/tokenizer#transformers.PythonBackend) which contains most of the main methods. Users should refer to
this superclass for more information regarding those methods.

- **token_ids_0** (`list[int]`) --
  List of IDs to which the special tokens will be added.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.`list[int]`List of [input IDs](../glossary#input-ids) with the appropriate special tokens.

Build model inputs from a sequence or a pair of sequence for sequence classification tasks by concatenating and
adding special tokens. A sequence has the following format:

- single sequence: `X </s>`
- pair of sequences: `A </s> B </s>`

- **token_ids_0** (`list[int]`) --
  List of IDs.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.
- **already_has_special_tokens** (`bool`, *optional*, defaults to `False`) --
  Whether or not the token list is already formatted with special tokens for the model.`list[int]`A list of integers in the range [0, 1]: 1 for a special token, 0 for a sequence token.

Retrieve sequence ids from a token list that has no special tokens added. This method is called when adding
special tokens using the tokenizer `prepare_for_model` method.

- **token_ids_0** (`list[int]`) --
  List of IDs.
- **token_ids_1** (`list[int]`, *optional*) --
  Optional second list of IDs for sequence pairs.`list[int]`List of zeros.

Create a mask from the two sequences passed to be used in a sequence-pair classification task. T5 does not make
use of token type ids, therefore a list of zeros is returned.

## SiglipImageProcessor[[transformers.SiglipImageProcessor]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a SigLIP image processor.

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

## SiglipImageProcessorPil[[transformers.SiglipImageProcessorPil]]

- ****kwargs** ([ImagesKwargs](/docs/transformers/v5.14.0/en/main_classes/processors#transformers.ImagesKwargs), *optional*) --
  Additional image preprocessing options. Model-specific kwargs are listed above; see the TypedDict class
  for the complete list of supported arguments.
Constructs a SigLIP image processor.

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

## SiglipProcessor[[transformers.SiglipProcessor]]

- **image_processor** (`SiglipImageProcessor`) --
  The image processor is a required input.
- **tokenizer** (`SiglipTokenizer`) --
  The tokenizer is a required input.
Constructs a SiglipProcessor which wraps a image processor and a tokenizer into a single processor.

[SiglipProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipProcessor) offers all the functionalities of [SiglipImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipImageProcessor) and [SiglipTokenizer](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipTokenizer). See the
[~SiglipImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipImageProcessor) and [~SiglipTokenizer](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipTokenizer) for more information.

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

## SiglipModel[[transformers.SiglipModel]]

- **config** ([SiglipConfig](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Siglip Model outputting raw hidden-states without any specific head on top.

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
  [SiglipImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipImageProcessor). See `SiglipImageProcessor.__call__()` for details ([SiglipProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipProcessor) uses
  [SiglipImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipImageProcessor) for processing images).
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **position_ids** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)
- **return_loss** (`bool`, *optional*) --
  Whether or not to return the contrastive loss.
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.`SiglipOutput` or `tuple(torch.FloatTensor)`A `SiglipOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SiglipConfig](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipConfig)) and inputs.
The [SiglipModel](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Contrastive loss for image-text similarity.
- **logits_per_image** (`torch.FloatTensor` of shape `(image_batch_size, text_batch_size)`) -- The scaled dot product scores between `image_embeds` and `text_embeds`. This represents the image-text
  similarity scores.
- **logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, image_batch_size)`) -- The scaled dot product scores between `text_embeds` and `image_embeds`. This represents the text-image
  similarity scores.
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The text embeddings obtained by applying the projection layer to the pooled output of [SiglipTextModel](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipTextModel).
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The image embeddings obtained by applying the projection layer to the pooled output of [SiglipVisionModel](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipVisionModel).
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [SiglipTextModel](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipTextModel).
- **vision_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [SiglipVisionModel](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipVisionModel).

Examples:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoProcessor, AutoModel
>>> import torch

>>> model = AutoModel.from_pretrained("google/siglip-base-patch16-224")
>>> processor = AutoProcessor.from_pretrained("google/siglip-base-patch16-224")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> texts = ["a photo of 2 cats", "a photo of 2 dogs"]
>>> # important: we pass `padding=max_length` since the model was trained with this
>>> inputs = processor(text=texts, images=image, padding="max_length", return_tensors="pt")

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> logits_per_image = outputs.logits_per_image
>>> probs = torch.sigmoid(logits_per_image) # these are the probabilities
>>> print(f"{probs[0][0]:.1%} that image 0 is '{texts[0]}'")
31.9% that image 0 is 'a photo of 2 cats'
```

)>"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "position_ids", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
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
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SiglipConfig](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipConfig)) and inputs.

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
>>> from transformers import AutoTokenizer, AutoModel
>>> import torch

>>> model = AutoModel.from_pretrained("google/siglip-base-patch16-224")
>>> tokenizer = AutoTokenizer.from_pretrained("google/siglip-base-patch16-224")

>>> # important: make sure to set padding="max_length" as that's how the model was trained
>>> inputs = tokenizer(["a photo of a cat", "a photo of a dog"], padding="max_length", return_tensors="pt")
>>> with torch.no_grad():
...     text_features = model.get_text_features(**inputs)
```

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [SiglipImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipImageProcessor). See `SiglipImageProcessor.__call__()` for details ([SiglipProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipProcessor) uses
  [SiglipImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipImageProcessor) for processing images).
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SiglipConfig](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipConfig)) and inputs.

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
>>> from transformers import AutoProcessor, AutoModel
>>> from transformers.image_utils import load_image

>>> model = AutoModel.from_pretrained("google/siglip-base-patch16-224")
>>> processor = AutoProcessor.from_pretrained("google/siglip-base-patch16-224")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> image = load_image(url)

>>> inputs = processor(images=image, return_tensors="pt")

>>> with torch.no_grad():
...     image_features = model.get_image_features(**inputs)
```

## SiglipTextModel[[transformers.SiglipTextModel]]

- **config** ([SiglipTextConfig](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The text model from SigLIP without any head or projection on top.

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
- **position_ids** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.

  [What are position IDs?](../glossary#position-ids)[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SiglipConfig](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipConfig)) and inputs.
The [SiglipTextModel](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipTextModel) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, SiglipTextModel

>>> model = SiglipTextModel.from_pretrained("google/siglip-base-patch16-224")
>>> tokenizer = AutoTokenizer.from_pretrained("google/siglip-base-patch16-224")

>>> # important: make sure to set padding="max_length" as that's how the model was trained
>>> inputs = tokenizer(["a photo of a cat", "a photo of a dog"], padding="max_length", return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooled_output = outputs.pooler_output  # pooled (EOS token) states
```

## SiglipVisionModel[[transformers.SiglipVisionModel]]

- **config** ([SiglipVisionConfig](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipVisionConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The vision model from SigLIP without any head or projection on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [SiglipImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipImageProcessor). See `SiglipImageProcessor.__call__()` for details ([SiglipProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipProcessor) uses
  [SiglipImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipImageProcessor) for processing images).
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.[BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`A [BaseModelOutputWithPooling](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SiglipConfig](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipConfig)) and inputs.
The [SiglipVisionModel](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipVisionModel) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoProcessor, SiglipVisionModel

>>> model = SiglipVisionModel.from_pretrained("google/siglip-base-patch16-224")
>>> processor = AutoProcessor.from_pretrained("google/siglip-base-patch16-224")

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> inputs = processor(images=image, return_tensors="pt")

>>> outputs = model(**inputs)
>>> last_hidden_state = outputs.last_hidden_state
>>> pooled_output = outputs.pooler_output  # pooled features
```

## SiglipForImageClassification[[transformers.SiglipForImageClassification]]

- **config** ([SiglipConfig](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SigLIP vision encoder with an image classification head on top (a linear layer on top of the pooled final hidden states of
the patch tokens) e.g. for ImageNet.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [SiglipImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipImageProcessor). See `SiglipImageProcessor.__call__()` for details ([SiglipProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipProcessor) uses
  [SiglipImageProcessor](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipImageProcessor) for processing images).
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the image classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).
- **interpolate_pos_encoding** (`bool`, *optional*, defaults to `False`) --
  Whether to interpolate the pre-trained position encodings.[ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or `tuple(torch.FloatTensor)`A [ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SiglipConfig](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipConfig)) and inputs.
The [SiglipForImageClassification](/docs/transformers/v5.14.0/en/model_doc/siglip#transformers.SiglipForImageClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each stage) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states
  (also called feature maps) of the model at the output of each stage.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, patch_size,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> from transformers import AutoImageProcessor, SiglipForImageClassification
>>> import torch
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> torch.manual_seed(3)
>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))

>>> # note: we are loading a `SiglipModel` from the hub here,
>>> # so the head will be randomly initialized, hence the predictions will be random if seed is not set above.
>>> image_processor = AutoImageProcessor.from_pretrained("google/siglip-base-patch16-224")
>>> model = SiglipForImageClassification.from_pretrained("google/siglip-base-patch16-224")

>>> inputs = image_processor(images=image, return_tensors="pt")
>>> outputs = model(**inputs)
>>> logits = outputs.logits
>>> # model predicts one of the two classes
>>> predicted_class_idx = logits.argmax(-1).item()
>>> print("Predicted class:", model.config.id2label[predicted_class_idx])
Predicted class: LABEL_1
```
