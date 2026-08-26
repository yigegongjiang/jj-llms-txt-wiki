# TIPSv2

    
        
        
    

## Overview

TIPSv2 (Text-Image Pre-training with Spatial awareness) is a family of
contrastive vision-language encoders proposed in [TIPSv2: Advancing Vision-Language Pretraining with Enhanced
Patch-Text Alignment](https://huggingface.co/papers/2604.12012) by Bingyi Cao, Koert Chen, Kevis-Kokitsi Maninis, Kaifeng Chen, Arjun Karpur, Ye Xia, Sahil Dua, Tanmaya Dabral, Guangxing Han, Bohyung Han, Joshua Ainslie, Alex Bewley, Mithun Jacob, René Wagner, Washington Ramos, Krzysztof Choromanski, Mojtaba Seyedhosseini, Howard Zhou, André Araujo.

The abstract from the paper is the following:

*Recent progress in vision-language pretraining has enabled significant improvements to many downstream computer vision applications, such as classification, retrieval, segmentation and depth prediction. However, a fundamental capability that these models still struggle with is aligning dense patch representations with text embeddings of corresponding concepts. In this work, we investigate this critical issue and propose novel techniques to enhance this capability in foundational vision-language models. First, we reveal that a patch-level distillation procedure significantly boosts dense patch-text alignment – surprisingly, the patch-text alignment of the distilled student model strongly surpasses that of the teacher model. This observation inspires us to consider modifications to pretraining recipes, leading us to propose iBOT++, an upgrade to the commonly-used iBOT masked image objective, where unmasked tokens also contribute directly to the loss. This dramatically enhances patch-text alignment of pretrained models. Additionally, to improve vision-language pretraining efficiency and effectiveness, we modify the exponential moving average setup in the learning recipe, and introduce a caption sampling strategy to benefit from synthetic captions at different granularities. Combining these components, we develop TIPSv2, a new family of image-text encoder models suitable for a wide range of downstream applications. Through comprehensive experiments on 9 tasks and 20 datasets, we demonstrate strong performance, generally on par with or better than recent vision encoder models. Code and models are released via our project page at https://gdm-tipsv2.github.io/.*

<img src="https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/tipsv2_architecture.jpg"
alt="tipsv2 architecture overview" width="600"/>

This model was contributed by [Ternuraz](https://huggingface.co/Ternuraz) and [guarin](https://huggingface.co/guarin).
The original code can be found [here](https://github.com/google-deepmind/tips).

You can find all the original TIPSv2 checkpoints under the [TIPSv2](https://huggingface.co/collections/google/tipsv2) collection.

> [!TIP]
> See [TIPSv2 DPT](./tipsv2_dpt) for depth estimation, normal estimation, and semantic segmentation on top of the TIPSv2 vision backbone.

```python
from transformers import pipeline

image = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg"
candidate_labels = ["a photo of a cat", "a photo of a dog", "a photo of a car"]

classifier = pipeline(task="zero-shot-image-classification", model="google/tipsv2-b14", device_map="auto")
out = classifier(image, candidate_labels=candidate_labels)
print(out)
# [{'score': 0.999, 'label': 'a photo of a cat'}, {'score': 0.001, 'label': 'a photo of a dog'}, {'score': 0.0, 'label': 'a photo of a car'}]
```

```python
import torch

from transformers import AutoModel, AutoProcessor
from transformers.image_utils import load_image

model_id = "google/tipsv2-b14"
model = AutoModel.from_pretrained(model_id, device_map="auto")
processor = AutoProcessor.from_pretrained(model_id)

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
candidate_labels = ["a photo of a cat", "a photo of a dog", "a photo of a car"]

inputs = processor(text=candidate_labels, images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

probs = outputs.logits_per_image.softmax(dim=1)
most_likely_idx = probs.argmax(dim=1).item()
most_likely_label = candidate_labels[most_likely_idx]
print(f"Most likely label: '{most_likely_label}' with probability: {probs[0][most_likely_idx].item():.3f}")
# Most likely label: 'a photo of a cat' with probability: 0.975
```

Use [get_image_features()](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Model.get_image_features) and [get_text_features()](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Model.get_text_features) to encode images and texts separately, which is useful for retrieval workflows where embeddings are computed independently (e.g., pre-indexing a large image database). The returned embeddings are *not* normalized, so apply L2 normalization before computing similarity.

```python
import torch
import torch.nn.functional as F

from transformers import AutoModel, AutoProcessor
from transformers.image_utils import load_image

model_id = "google/tipsv2-b14"
model = AutoModel.from_pretrained(model_id, device_map="auto")
processor = AutoProcessor.from_pretrained(model_id)

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
candidate_labels = ["a photo of a cat", "a photo of a dog", "a photo of a car"]

image_inputs = processor(images=image, return_tensors="pt").to(model.device)
text_inputs = processor(text=candidate_labels, return_tensors="pt").to(model.device)

with torch.no_grad():
    image_outputs = model.get_image_features(**image_inputs)
    text_outputs = model.get_text_features(**text_inputs)

image_embeds = F.normalize(image_outputs.pooler_output, dim=-1)
text_embeds = F.normalize(text_outputs.pooler_output, dim=-1)

probs = (image_embeds @ text_embeds.T / model.temperature).softmax(dim=-1)
most_likely_idx = probs.argmax(dim=-1).item()
most_likely_label = candidate_labels[most_likely_idx]
print(f"Most likely label: '{most_likely_label}' with probability: {probs[0][most_likely_idx].item():.3f}")
# Most likely label: 'a photo of a cat' with probability: 0.975
```

Use [AutoBackbone](/docs/transformers/v5.15.1/en/main_classes/backbones#transformers.AutoBackbone) to load the vision backbone directly and get spatial feature maps, without the text model. 

```python
import torch
from transformers import AutoBackbone, AutoImageProcessor
from transformers.image_utils import load_image

model_id = "google/tipsv2-b14"
backbone = AutoBackbone.from_pretrained(model_id, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_id)

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
inputs = image_processor(images=image, return_tensors="pt").to(backbone.device)

with torch.no_grad():
    outputs = backbone(**inputs)

# feature_maps is a tuple of tensors, one per requested stage
patch_features = outputs.feature_maps[-1] # (batch_size, hidden_size, height, width) tensor
```

Use [Tipsv2VisionModel](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2VisionModel) if you only need access to the vision features. In particular, this allows you to access the two class tokens from Tipsv2 as shown below. Note that Tipsv2 repurposed the register token from [Dinov2WithRegistersModel](/docs/transformers/v5.15.1/en/model_doc/dinov2_with_registers#transformers.Dinov2WithRegistersModel) as a secondary class token. The two tokens differ in how they were trained:

- Class token 1: Supervised by web alt-text captions
- Class token 2: Supervised by PaliGemma synthetic captions

```python
import torch
from transformers import AutoConfig, AutoImageProcessor, AutoModel
from transformers.image_utils import load_image

model_id = "google/tipsv2-b14"
config = AutoConfig.from_pretrained(model_id)
model = AutoModel.from_pretrained(model_id, config=config.vision_config, device_map="auto")
image_processor = AutoImageProcessor.from_pretrained(model_id)

image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
inputs = image_processor(images=image, return_tensors="pt").to(model.device)

with torch.no_grad():
    outputs = model(**inputs)

sequence = outputs.last_hidden_state # (batch_size, 1 + config.num_register_tokens + num_patches, hidden_size)
cls_token_1 = sequence[:, 0] # (batch_size, hidden_size)
cls_token_2 = sequence[:, 1] # (batch_size, hidden_size)
```

## Notes

- [Tipsv2Model](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Model) returns normalized `image_embeds` and `text_embeds`. `logits_per_image` and `logits_per_text` are convenience outputs computed as cosine similarity divided by the temperature.
- Use [get_image_features()](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Model.get_image_features) and [get_text_features()](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Model.get_text_features) to retrieve image and text embeddings individually.
- Use [Tipsv2VisionBackbone](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2VisionBackbone) if you need access to feature maps from all layers.
- Use [Tipsv2VisionModel](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2VisionModel) if you need access to the two vision class tokens.

## Tipsv2Config[[transformers.Tipsv2Config]]

#### transformers.Tipsv2Config[[transformers.Tipsv2Config]]

```python
transformers.Tipsv2Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, text_config: dict | transformers.models.tipsv2.configuration_tipsv2.Tipsv2TextConfig | None = None, vision_config: dict | transformers.models.tipsv2.configuration_tipsv2.Tipsv2VisionConfig | None = None, temperature_init_value: float = 0.005065968260169029)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/configuration_tipsv2.py#L141)

**Parameters:**

text_config (`dict`, *optional*) : Dictionary of configuration options used to initialize [Tipsv2TextConfig](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2TextConfig).

vision_config (`dict`, *optional*) : Dictionary of configuration options used to initialize [Tipsv2VisionConfig](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2VisionConfig).

temperature_init_value (`float`, *optional*, defaults to `0.005065968260169029`) : Initial value for the learnable temperature parameter used to scale cosine-similarity logits in [Tipsv2Model](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Model).

This is the configuration class to store the configuration of a Tipsv2Model. It is used to instantiate a Tipsv2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/tipsv2-b14](https://huggingface.co/google/tipsv2-b14)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Tipsv2Config, Tipsv2Model

>>> configuration = Tipsv2Config()
>>> model = Tipsv2Model(configuration)
>>> configuration = model.config

>>> from transformers import Tipsv2TextConfig, Tipsv2VisionConfig

>>> text_config = Tipsv2TextConfig()
>>> vision_config = Tipsv2VisionConfig()
>>> config = Tipsv2Config(text_config=text_config, vision_config=vision_config)
```

## Tipsv2TextConfig[[transformers.Tipsv2TextConfig]]

#### transformers.Tipsv2TextConfig[[transformers.Tipsv2TextConfig]]

```python
transformers.Tipsv2TextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 32000, hidden_size: int = 768, intermediate_size: int = 3072, num_hidden_layers: int = 12, num_attention_heads: int = 12, max_position_embeddings: int = 64, hidden_act: str = 'relu', layer_norm_eps: float = 1e-05, attention_dropout: float | int = 0.0, pad_token_id: int | None = 0, bos_token_id: int | None = None, eos_token_id: int | list[int] | None = None, initializer_range: float = 0.02, scale_sqrt_depth: bool = True, pooling_epsilon: float = 1e-08)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/configuration_tipsv2.py#L92)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `32000`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

max_position_embeddings (`int`, *optional*, defaults to `64`) : The maximum sequence length that this model might ever be used with.

hidden_act (`str`, *optional*, defaults to `relu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*) : Token id used for end-of-stream in the vocabulary.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

scale_sqrt_depth (`bool`, *optional*, defaults to `True`) : Whether to scale token embeddings by `sqrt(hidden_size)` before adding sinusoidal position embeddings.

pooling_epsilon (`float`, *optional*, defaults to `1e-8`) : Epsilon added to the valid token count when computing masked mean pooling.

This is the configuration class to store the configuration of a Tipsv2Model. It is used to instantiate a Tipsv2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/tipsv2-b14](https://huggingface.co/google/tipsv2-b14)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Tipsv2TextConfig, Tipsv2TextModel

>>> configuration = Tipsv2TextConfig()
>>> model = Tipsv2TextModel(configuration)
>>> configuration = model.config
```

## Tipsv2VisionConfig[[transformers.Tipsv2VisionConfig]]

#### transformers.Tipsv2VisionConfig[[transformers.Tipsv2VisionConfig]]

```python
transformers.Tipsv2VisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, mlp_ratio: int | float = 4, hidden_act: str = 'gelu', hidden_dropout_prob: float | int = 0.0, attention_probs_dropout_prob: float | int = 0.0, initializer_range: float = 0.02, layer_norm_eps: float = 1e-06, image_size: int | list[int] | tuple[int, int] = 448, patch_size: int | list[int] | tuple[int, int] = 14, num_channels: int = 3, qkv_bias: bool = True, layerscale_value: float = 1.0, drop_path_rate: float | int = 0.0, use_swiglu_ffn: bool = False, num_register_tokens: int = 1, _out_features: list[str] | None = None, _out_indices: list[int] | None = None, apply_layernorm: bool = True, reshape_hidden_states: bool = True)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/configuration_tipsv2.py#L30)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

mlp_ratio (`Union[int, float]`, *optional*, defaults to `4`) : Ratio of the MLP hidden dim to the embedding dim.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

attention_probs_dropout_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `448`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `14`) : The size (resolution) of each patch.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

layerscale_value (`float`, *optional*, defaults to `1.0`) : Initial value to use for layer scale.

drop_path_rate (`Union[float, int]`, *optional*, defaults to `0.0`) : Drop path rate for the patch fusion.

use_swiglu_ffn (`bool`, *optional*, defaults to `False`) : Whether to use the SwiGLU feedforward neural network. Otherwise a standard MLP with `hidden_act` as activation function is used.

num_register_tokens (`int`, *optional*, defaults to `1`) : Number of register tokens to use.

apply_layernorm (`bool`, *optional*, defaults to `True`) : Whether to apply layer normalization to the feature maps in case the model is used as backbone.

reshape_hidden_states (`bool`, *optional*, defaults to `True`) : Whether to reshape the feature maps to 4D tensors of shape `(batch_size, hidden_size, height, width)` in case the model is used as backbone. If `False`, the feature maps will be 3D tensors of shape `(batch_size, seq_len, hidden_size)`.

This is the configuration class to store the configuration of a Tipsv2Model. It is used to instantiate a Tipsv2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [google/tipsv2-b14](https://huggingface.co/google/tipsv2-b14)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import Tipsv2VisionConfig, Tipsv2VisionModel

>>> configuration = Tipsv2VisionConfig()
>>> model = Tipsv2VisionModel(configuration)
>>> configuration = model.config
```

## Tipsv2Tokenizer[[transformers.Tipsv2Tokenizer]]

#### transformers.Tipsv2Tokenizer[[transformers.Tipsv2Tokenizer]]

```python
transformers.Tipsv2Tokenizer(vocab: dict[str, int] | None = None, merges: list[tuple[str, str]] | None = None, unk_token: str | None = '<unk>', pad_token: str | None = '<pad>', bos_token: str | None = None, eos_token: str | None = None, model_max_length: int = 64, do_lower_case: bool = True, token_type_ids_pattern: str = 'all_zeros', _spm_precompiled_charsmap = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/tokenization_tipsv2.py#L30)

Tipsv2 tokenizer backed by HuggingFace's *tokenizers* library, based on a BPE (SentencePiece) model.

#### __call__[[transformers.Tipsv2Tokenizer.__call__]]

```python
__call__(text: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, text_pair: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, text_target: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, text_pair_target: TextInput | PreTokenizedInput | list[TextInput] | list[PreTokenizedInput] | None = None, add_special_tokens: bool = True, padding: bool | str | PaddingStrategy = False, truncation: bool | str | TruncationStrategy | None = None, max_length: int | None = None, stride: int = 0, is_split_into_words: bool = False, pad_to_multiple_of: int | None = None, padding_side: str | None = None, return_tensors: str | TensorType | None = None, return_token_type_ids: bool | None = None, return_attention_mask: bool | None = None, return_overflowing_tokens: bool = False, return_special_tokens_mask: bool = False, return_offsets_mapping: bool = False, return_length: bool = False, verbose: bool = True, tokenizer_kwargs: dict[str, Any] | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/tokenization_utils_base.py#L2417)

**Parameters:**

text (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_pair (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_target (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

text_pair_target (`str`, `list[str]`, `list[list[str]]`, *optional*) : The sequence or batch of sequences to be encoded as target texts. Each sequence can be a string or a list of strings (pretokenized string). If the sequences are provided as list of strings (pretokenized), you must set `is_split_into_words=True` (to lift the ambiguity with a batch of sequences).

tokenizer_kwargs (`dict[str, Any]`, *optional*) : Additional kwargs to pass to the tokenizer. These will be merged with the explicit parameters and other kwargs, with explicit parameters taking precedence. 

add_special_tokens (`bool`, *optional*, defaults to `True`) : Whether or not to add special tokens when encoding the sequences. This will use the underlying `PretrainedTokenizerBase.build_inputs_with_special_tokens` function, which defines which tokens are automatically added to the input ids. This is useful if you want to add `bos` or `eos` tokens automatically.

padding (`bool`, `str` or [PaddingStrategy](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.utils.PaddingStrategy), *optional*, defaults to `False`) : Activates and controls padding. Accepts the following values:  - `True` or `'longest'`: Pad to the longest sequence in the batch (or no padding if only a single sequence is provided). - `'max_length'`: Pad to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. - `False` or `'do_not_pad'` (default): No padding (i.e., can output a batch with sequences of different lengths).

truncation (`bool`, `str` or [TruncationStrategy](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.tokenization_utils_base.TruncationStrategy), *optional*, defaults to `False`) : Activates and controls truncation. Accepts the following values:  - `True` or `'longest_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will truncate token by token, removing a token from the longest sequence in the pair if a pair of sequences (or a batch of pairs) is provided. - `'only_first'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the first sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `'only_second'`: Truncate to a maximum length specified with the argument `max_length` or to the maximum acceptable input length for the model if that argument is not provided. This will only truncate the second sequence of a pair if a pair of sequences (or a batch of pairs) is provided. - `False` or `'do_not_truncate'` (default): No truncation (i.e., can output batch with sequence lengths greater than the model maximum admissible input size).

max_length (`int`, *optional*) : Controls the maximum length to use by one of the truncation/padding parameters.  If left unset or set to `None`, this will use the predefined model maximum length if a maximum length is required by one of the truncation/padding parameters. If the model has no specific maximum input length (like XLNet) truncation/padding to a maximum length will be deactivated.

stride (`int`, *optional*, defaults to 0) : If set to a number along with `max_length`, the overflowing tokens returned when `return_overflowing_tokens=True` will contain some tokens from the end of the truncated sequence returned to provide some overlap between truncated and overflowing sequences. The value of this argument defines the number of overlapping tokens.

is_split_into_words (`bool`, *optional*, defaults to `False`) : Whether or not the input is already pre-tokenized (e.g., split into words). If set to `True`, the tokenizer assumes the input is already split into words (for instance, by splitting it on whitespace) which it will tokenize. This is useful for NER or token classification.

pad_to_multiple_of (`int`, *optional*) : If set will pad the sequence to a multiple of the provided value. Requires `padding` to be activated. This is especially useful to enable the use of Tensor Cores on NVIDIA hardware with compute capability `>= 7.5` (Volta).

padding_side (`str`, *optional*) : The side on which the model should have padding applied. Should be selected between ['right', 'left']. Default value is picked from the class attribute of the same name.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors instead of list of python integers. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return Numpy `np.ndarray` objects. 

return_token_type_ids (`bool`, *optional*) : Whether to return token type IDs. If left to the default, will return the token type IDs according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are token type IDs?](../glossary#token-type-ids)

return_attention_mask (`bool`, *optional*) : Whether to return the attention mask. If left to the default, will return the attention mask according to the specific tokenizer's default, defined by the `return_outputs` attribute.  [What are attention masks?](../glossary#attention-mask)

return_overflowing_tokens (`bool`, *optional*, defaults to `False`) : Whether or not to return overflowing token sequences. If a pair of sequences of input ids (or a batch of pairs) is provided with `truncation_strategy = longest_first` or `True`, an error is raised instead of returning overflowing tokens.

return_special_tokens_mask (`bool`, *optional*, defaults to `False`) : Whether or not to return special tokens mask information.

return_offsets_mapping (`bool`, *optional*, defaults to `False`) : Whether or not to return `(char_start, char_end)` for each token.  This is only available on fast tokenizers inheriting from [PreTrainedTokenizerFast](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend), if using Python's tokenizer, this method will raise `NotImplementedError`.

return_length  (`bool`, *optional*, defaults to `False`) : Whether or not to return the lengths of the encoded inputs.

verbose (`bool`, *optional*, defaults to `True`) : Whether or not to print more information and warnings.

- ****kwargs** : passed to the `self.tokenize()` method

**Returns:** [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding)

A [BatchEncoding](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.BatchEncoding) with the following fields:

- **input_ids** -- List of token ids to be fed to a model.

  [What are input IDs?](../glossary#input-ids)

- **token_type_ids** -- List of token type ids to be fed to a model (when `return_token_type_ids=True` or
  if *"token_type_ids"* is in `self.model_input_names`).

  [What are token type IDs?](../glossary#token-type-ids)

- **attention_mask** -- List of indices specifying which tokens should be attended to by the model (when
  `return_attention_mask=True` or if *"attention_mask"* is in `self.model_input_names`).

  [What are attention masks?](../glossary#attention-mask)

- **overflowing_tokens** -- List of overflowing tokens sequences (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **num_truncated_tokens** -- Number of tokens truncated (when a `max_length` is specified and
  `return_overflowing_tokens=True`).
- **special_tokens_mask** -- List of 0s and 1s, with 1 specifying added special tokens and 0 specifying
  regular sequence tokens (when `add_special_tokens=True` and `return_special_tokens_mask=True`).
- **length** -- The length of the inputs (when `return_length=True`)

Main method to tokenize and prepare for the model one or several sequence(s) or one or several pair(s) of
sequences.

## Tipsv2ImageProcessor[[transformers.Tipsv2ImageProcessor]]

#### transformers.Tipsv2ImageProcessor[[transformers.Tipsv2ImageProcessor]]

```python
transformers.Tipsv2ImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/image_processing_tipsv2.py#L27)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 448, 'width': 448}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BILINEAR`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

do_rescale (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to rescale the image.

rescale_factor (`float`, *kwargs*, *optional*, defaults to `0.00392156862745098`) : Rescale factor to rescale the image by if `do_rescale` is set to `True`.

do_normalize (`bool`, *kwargs*, *optional*, defaults to `False`) : Whether to normalize the image.

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

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models. Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors.

disable_grouping (`bool`, *kwargs*, *optional*) : Whether to disable grouping of images by size to process them individually and not in batches. If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157

image_seq_length (`int`, *kwargs*, *optional*) : The number of image tokens to be used for each image in the input. Added for backward compatibility but this should be set as a processor attribute in future models.

Returns stacked tensors if set to `'pt'`, otherwise returns a list of tensors. --

Constructs a Tipsv2ImageProcessor image processor.

disable_grouping (`bool`, *kwargs*, *optional*):
Whether to disable grouping of images by size to process them individually and not in batches.
If None, will be set to True if the images are on CPU, and False otherwise. This choice is based on
empirical observations, as detailed here: https://github.com/huggingface/transformers/pull/38157
image_seq_length (`int`, *kwargs*, *optional*):
The number of image tokens to be used for each image in the input.
Added for backward compatibility but this should be set as a processor attribute in future models.

#### preprocess[[transformers.Tipsv2ImageProcessor.preprocess]]

```python
preprocess(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor']], *args, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/image_processing_utils.py#L382)

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

**Returns:** `~image_processing_base.BatchFeature`

- **data** (`dict`) -- Dictionary of lists/arrays/tensors returned by the __call__ method ('pixel_values', etc.).
- **tensor_type** (`Union[None, str, TensorType]`, *optional*) -- You can give a tensor_type here to convert the lists of integers in PyTorch/Numpy Tensors at
  initialization.

## Tipsv2Processor[[transformers.Tipsv2Processor]]

#### transformers.Tipsv2Processor[[transformers.Tipsv2Processor]]

```python
transformers.Tipsv2Processor(image_processor = None, tokenizer = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/processing_tipsv2.py#L36)

**Parameters:**

image_processor (`Tipsv2ImageProcessor`) : The image processor is a required input.

tokenizer (`Tipsv2Tokenizer`) : The tokenizer is a required input.

Constructs a Tipsv2Processor which wraps a image processor and a tokenizer into a single processor.

[Tipsv2Processor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Processor) offers all the functionalities of [Tipsv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2ImageProcessor) and [Tipsv2Tokenizer](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Tokenizer). See the
[~Tipsv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2ImageProcessor) and [~Tipsv2Tokenizer](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Tokenizer) for more information.

#### __call__[[transformers.Tipsv2Processor.__call__]]

```python
__call__(images: typing.Union[ForwardRef('PIL.Image.Image'), numpy.ndarray, ForwardRef('torch.Tensor'), list['PIL.Image.Image'], list[numpy.ndarray], list['torch.Tensor'], NoneType] = None, text: str | list[str] | list[list[str]] | None = None, videos: typing.Union[list['PIL.Image.Image'], numpy.ndarray, ForwardRef('torch.Tensor'), list[numpy.ndarray], list['torch.Tensor'], list[list['PIL.Image.Image']], list[list[numpy.ndarray]], list[list['torch.Tensor']], transformers.video_utils.URL, list[transformers.video_utils.URL], list[list[transformers.video_utils.URL]], transformers.video_utils.Path, list[transformers.video_utils.Path], list[list[transformers.video_utils.Path]], NoneType] = None, audio: typing.Union[numpy.ndarray, ForwardRef('torch.Tensor'), collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence['torch.Tensor'], NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/processing_utils.py#L651)

**Parameters:**

images (`Union[PIL.Image.Image, numpy.ndarray, torch.Tensor, list[PIL.Image.Image], list[numpy.ndarray], list[torch.Tensor]]`, *optional*) : Image to preprocess. Expects a single or batch of images with pixel values ranging from 0 to 255. If passing in images with pixel values between 0 and 1, set `do_rescale=False`.

text (`Union[str, list[str], list[list[str]]]`, *optional*) : The sequence or batch of sequences to be encoded. Each sequence can be a string or a list of strings (pretokenized string). If you pass a pretokenized input, set `is_split_into_words=True` to avoid ambiguity with batched inputs.

videos (`Union[list[PIL.Image.Image], numpy.ndarray, torch.Tensor, list[numpy.ndarray], list[torch.Tensor], list[list[PIL.Image.Image]], list[list[numpy.ndarray]], list[list[torch.Tensor]], ~video_utils.URL, list[~video_utils.URL], list[list[~video_utils.URL]], ~video_utils.Path, list[~video_utils.Path], list[list[~video_utils.Path]]]`, *optional*) : Video to preprocess. Expects a single or batch of videos with pixel values ranging from 0 to 255. If passing in videos with pixel values between 0 and 1, set `do_rescale=False`.

audio (`Union[numpy.ndarray, torch.Tensor, collections.abc.Sequence[numpy.ndarray], collections.abc.Sequence[torch.Tensor]]`, *optional*) : The audio or batch of audios to be prepared. Each audio can be a NumPy array or PyTorch tensor. In case of a NumPy array/PyTorch tensor, each audio should be of shape (C, T), where C is a number of channels, and T is the sample length of the audio.

return_tensors (`str` or [TensorType](/docs/transformers/v5.15.1/en/internal/file_utils#transformers.TensorType), *optional*) : If set, will return tensors of a particular framework. Acceptable values are:  - `'pt'`: Return PyTorch `torch.Tensor` objects. - `'np'`: Return NumPy `np.ndarray` objects.

- ****kwargs** ([ProcessingKwargs](/docs/transformers/v5.15.1/en/main_classes/processors#transformers.ProcessingKwargs), *optional*) : Additional processing options for each modality (text, images, videos, audio). Model-specific parameters are listed above; see the TypedDict class for the complete list of supported arguments.

## Tipsv2Model[[transformers.Tipsv2Model]]

#### transformers.Tipsv2Model[[transformers.Tipsv2Model]]

```python
transformers.Tipsv2Model(config: Tipsv2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/modeling_tipsv2.py#L1040)

**Parameters:**

config ([Tipsv2Config](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Tipsv2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Tipsv2Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, bool_masked_pos: typing.Optional[torch.Tensor] = None, return_loss: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/modeling_tipsv2.py#L1093)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Tipsv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2ImageProcessor). See `Tipsv2ImageProcessor.__call__()` for details ([Tipsv2Processor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Processor) uses [Tipsv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2ImageProcessor) for processing images).

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, sequence_length)`, *optional*) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0). Only relevant for pre-training.

return_loss (`bool`, *optional*) : Whether or not to return the contrastive loss when both image and text inputs are provided.

**Returns:** `Tipsv2Output` or `tuple(torch.FloatTensor)`

A `Tipsv2Output` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Tipsv2Config](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Config)) and inputs.

The [Tipsv2Model](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `return_loss` is `True`) -- Contrastive loss for image-text similarity.
- **logits_per_image** (`torch.FloatTensor` of shape `(image_batch_size, text_batch_size)`) -- The scaled dot product scores between `image_embeds` and `text_embeds`. This represents the image-text
  similarity scores.
- **logits_per_text** (`torch.FloatTensor` of shape `(text_batch_size, image_batch_size)`) -- The scaled dot product scores between `text_embeds` and `image_embeds`. This represents the text-image
  similarity scores.
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The text embeddings obtained by applying the projection layer to the pooled output of [Tipsv2TextModel](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2TextModel).
- **image_embeds** (`torch.FloatTensor` of shape `(batch_size, output_dim`) -- The image embeddings obtained by applying the projection layer to the pooled output of [Tipsv2VisionModel](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2VisionModel).
- **text_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [Tipsv2TextModel](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2TextModel).
- **vision_model_output** (`~modeling_outputs.BaseModelOutputWithPooling`, *optional*) -- The output of the [Tipsv2VisionModel](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2VisionModel).

Example:

```python
>>> import torch
>>> from transformers import AutoModel, AutoProcessor
>>> from transformers.image_utils import load_image

>>> model_id = "google/tipsv2-b14"
>>> model = AutoModel.from_pretrained(model_id, device_map="auto")
>>> processor = AutoProcessor.from_pretrained(model_id)

>>> image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
>>> candidate_labels = ["a photo of a cat", "a photo of a dog", "a photo of a car"]

>>> inputs = processor(text=candidate_labels, images=image, return_tensors="pt").to(model.device)

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> probs = outputs.logits_per_image.softmax(dim=1)
>>> most_likely_idx = probs.argmax(dim=1).item()
>>> most_likely_label = candidate_labels[most_likely_idx]
>>> print(f"Most likely label: '{most_likely_label}' with probability: {probs[0][most_likely_idx].item():.3f}")
Most likely label: 'a photo of a cat' with probability: 0.975
```

#### get_text_features[[transformers.Tipsv2Model.get_text_features]]

```python
get_text_features(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/modeling_tipsv2.py#L1050)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Tipsv2Config](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Config)) and inputs.

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
```

#### get_image_features[[transformers.Tipsv2Model.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, bool_masked_pos: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/modeling_tipsv2.py#L1068)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [Tipsv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2ImageProcessor). See `Tipsv2ImageProcessor.__call__()` for details ([Tipsv2Processor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Processor) uses [Tipsv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2ImageProcessor) for processing images).

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, sequence_length)`, *optional*) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0). Only relevant for pre-training.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Tipsv2Config](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Config)) and inputs.

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
```

## Tipsv2TextModel[[transformers.Tipsv2TextModel]]

#### transformers.Tipsv2TextModel[[transformers.Tipsv2TextModel]]

```python
transformers.Tipsv2TextModel(config: Tipsv2TextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/modeling_tipsv2.py#L916)

**Parameters:**

config ([Tipsv2TextConfig](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2TextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The TIPSv2 text tower without any projection head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Tipsv2TextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/modeling_tipsv2.py#L927)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Tipsv2Config](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Config)) and inputs.

The [Tipsv2TextModel](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2TextModel) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> import torch
>>> from transformers import AutoConfig, Tipsv2TextModel, AutoProcessor

>>> model_id = "google/tipsv2-b14"
>>> config = AutoConfig.from_pretrained(model_id)
>>> model = Tipsv2TextModel.from_pretrained(model_id, config=config.text_config, device_map="auto")
>>> processor = AutoProcessor.from_pretrained(model_id)

>>> candidate_labels = ["a photo of a cat", "a photo of a dog", "a photo of a car"]
>>> inputs = processor(text=candidate_labels, return_tensors="pt").to(model.device)

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> text_embeds = outputs.pooler_output  # (batch_size, hidden_size)
```

## Tipsv2VisionModel[[transformers.Tipsv2VisionModel]]

#### transformers.Tipsv2VisionModel[[transformers.Tipsv2VisionModel]]

```python
transformers.Tipsv2VisionModel(config: Tipsv2VisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/modeling_tipsv2.py#L490)

**Parameters:**

config ([Tipsv2VisionConfig](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2VisionConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Tipsv2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Tipsv2VisionModel.forward]]

```python
forward(pixel_values: typing.Optional[torch.Tensor] = None, bool_masked_pos: typing.Optional[torch.Tensor] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/modeling_tipsv2.py#L506)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [Tipsv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2ImageProcessor). See `Tipsv2ImageProcessor.__call__()` for details ([Tipsv2Processor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Processor) uses [Tipsv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2ImageProcessor) for processing images).

bool_masked_pos (`torch.BoolTensor` of shape `(batch_size, sequence_length)`) : Boolean masked positions. Indicates which patches are masked (1) and which aren't (0). Only relevant for pre-training.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Tipsv2Config](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Config)) and inputs.

The [Tipsv2VisionModel](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2VisionModel) forward method, overrides the `__call__` special method.

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

Example:

```python
>>> import torch
>>> from transformers import AutoConfig, AutoImageProcessor, AutoModel
>>> from transformers.image_utils import load_image

>>> model_id = "google/tipsv2-b14"
>>> config = AutoConfig.from_pretrained(model_id)
>>> model = AutoModel.from_pretrained(model_id, config=config.vision_config, device_map="auto")
>>> image_processor = AutoImageProcessor.from_pretrained(model_id)

>>> image = load_image("https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/pipeline-cat-chonk.jpeg")
>>> inputs = image_processor(images=image, return_tensors="pt").to(model.device)

>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> # Tipsv2 repurposes the register token from DINOv2-with-registers as a secondary class token
>>> sequence = outputs.last_hidden_state  # (batch_size, 1 + num_register_tokens + num_patches, hidden_size)
>>> cls_token_1 = sequence[:, 0]  # supervised by web alt-text captions
>>> cls_token_2 = sequence[:, 1 : 1 + model.config.num_register_tokens]  # supervised by synthetic captions
```

## Tipsv2VisionBackbone[[transformers.Tipsv2VisionBackbone]]

#### transformers.Tipsv2VisionBackbone[[transformers.Tipsv2VisionBackbone]]

```python
transformers.Tipsv2VisionBackbone(config: Tipsv2Config | Tipsv2VisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/modeling_tipsv2.py#L565)

**Parameters:**

config (`Tipsv2Config | Tipsv2VisionConfig`) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Tipsv2Vision backbone, to be used with frameworks like DETR and MaskFormer.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.Tipsv2VisionBackbone.forward]]

```python
forward(pixel_values: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/tipsv2/modeling_tipsv2.py#L587)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [Tipsv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2ImageProcessor). See `Tipsv2ImageProcessor.__call__()` for details ([Tipsv2Processor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Processor) uses [Tipsv2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2ImageProcessor) for processing images).

**Returns:** `BackboneOutput` or `tuple(torch.FloatTensor)`

A `BackboneOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Tipsv2Config](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2Config)) and inputs.

The [Tipsv2VisionBackbone](/docs/transformers/v5.15.1/en/model_doc/tipsv2#transformers.Tipsv2VisionBackbone) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **feature_maps** (`tuple(torch.FloatTensor)` of shape `(batch_size, num_channels, height, width)`) -- Feature maps of the stages.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, sequence_length, hidden_size)` or `(batch_size, num_channels, height, width)`,
  depending on the backbone.

  Hidden-states of the model at the output of each stage plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`. Only applicable if the backbone uses attention.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Examples:

```python
>>> import torch
>>> from transformers import AutoBackbone, Tipsv2VisionConfig

>>> config = Tipsv2VisionConfig(out_features=["stage3", "stage6", "stage9", "stage12"])
>>> model = AutoBackbone.from_config(config)

>>> pixel_values = torch.randn(1, 3, 448, 448)

>>> outputs = model(pixel_values)
>>> feature_maps = outputs.feature_maps
>>> list(feature_maps[-1].shape)
[1, 768, 32, 32]
```
