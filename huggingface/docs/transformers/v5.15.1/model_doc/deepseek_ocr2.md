# DeepSeek-OCR-2

## Overview

The DeepSeek-OCR-2 model was proposed in [Visual Causal Flow: A Novel Approach to OCR-Specialized Vision-Language Models](https://huggingface.co/papers/2601.20552) by the DeepSeek team.

DeepSeek-OCR-2 is an OCR-specialized vision-language model built on a distinctive architecture: a SAM ViT-B vision encoder feeds into a Qwen2 hybrid attention encoder, which is connected through an MLP projector to a DeepSeek-V2 Mixture-of-Experts (MoE) language model. A key feature of the model is its hybrid attention mechanism, which applies bidirectional attention over image tokens and causal attention over query tokens, enabling efficient and accurate document understanding.

 DeepSeek-OCR 2: Visual Causal Flow.

This model was contributed by [thisisiron](https://huggingface.co/thisisiron).

## Usage example

### Plain OCR

```python
from transformers import AutoProcessor, AutoModelForImageTextToText

model = AutoModelForImageTextToText.from_pretrained(
    "deepseek-community/DeepSeek-OCR-2", device_map="auto"
)
processor = AutoProcessor.from_pretrained("deepseek-community/DeepSeek-OCR-2")

image = "https://huggingface.co/datasets/hf-internal-testing/fixtures_got_ocr/resolve/main/image_ocr.jpg"
inputs = processor(images=image, text="<image>\nFree OCR.", return_tensors="pt").to(model.device)

generate_ids = model.generate(**inputs, do_sample=False, max_new_tokens=256)
processor.decode(generate_ids[0, inputs["input_ids"].shape[1] :], skip_special_tokens=True)
# "R&D QUALITY IMPROVEMENT\nSUGGESTION/SOLUTION FORM\nName/Phone Ext. : (...)"
```

### Grounding with markdown conversion

The `<|grounding|>` token enables coordinate-aware output with `<|ref|>` and `<|det|>` tags.

```python
inputs = processor(
    images=image,
    text="<image>\n<|grounding|>Convert the document to markdown.",
    return_tensors="pt",
).to(model.device)

generate_ids = model.generate(**inputs, do_sample=False, max_new_tokens=256)
processor.decode(generate_ids[0, inputs["input_ids"].shape[1] :], skip_special_tokens=False)
# "<|ref|>title<|/ref|><|det|>[[330, 198, 558, 230]]<|/det|>\n# R&D QUALITY (...)"
```

## DeepseekOcr2Config[[transformers.DeepseekOcr2Config]]

#### transformers.DeepseekOcr2Config[[transformers.DeepseekOcr2Config]]

```python
transformers.DeepseekOcr2Config(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vision_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, text_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, image_token_id: int = 128815, tie_word_embeddings: bool = False)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/configuration_deepseek_ocr2.py#L278)

**Parameters:**

vision_config (`dict` or `DeepseekOcr2VisionConfig`, *optional*) : Configuration for the vision encoders. Defaults to `DeepseekOcr2VisionConfig()`.

text_config (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) : The config object or dictionary of the text backbone.

image_token_id (`int`, *optional*, defaults to `128815`) : The image token index used as a placeholder for input images.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a DeepseekOcr2Model. It is used to instantiate a Deepseek Ocr2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-community/DeepSeek-OCR-2](https://huggingface.co/deepseek-community/DeepSeek-OCR-2)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## DeepseekOcr2VisionConfig[[transformers.DeepseekOcr2VisionConfig]]

#### transformers.DeepseekOcr2VisionConfig[[transformers.DeepseekOcr2VisionConfig]]

```python
transformers.DeepseekOcr2VisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, sam_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None, encoder_config: dict | transformers.configuration_utils.PreTrainedConfig | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/configuration_deepseek_ocr2.py#L150)

**Parameters:**

sam_config (`dict` or `DeepseekOcr2SamVisionConfig`, *optional*) : Configuration for the SAM vision encoder. Defaults to `DeepseekOcr2SamVisionConfig()`.

encoder_config (`dict` or `DeepseekOcr2VisionEncoderConfig`, *optional*) : Configuration for the DeepSeek-OCR-2 vision encoder. Defaults to `DeepseekOcr2VisionEncoderConfig()`.

This is the configuration class to store the configuration of a DeepseekOcr2Model. It is used to instantiate a Deepseek Ocr2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-community/DeepSeek-OCR-2](https://huggingface.co/deepseek-community/DeepSeek-OCR-2)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## DeepseekOcr2SamVisionConfig[[transformers.DeepseekOcr2SamVisionConfig]]

#### transformers.DeepseekOcr2SamVisionConfig[[transformers.DeepseekOcr2SamVisionConfig]]

```python
transformers.DeepseekOcr2SamVisionConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, hidden_size: int = 768, output_channels: int = 256, num_hidden_layers: int = 12, num_attention_heads: int = 12, num_channels: int = 3, image_size: int | list[int] | tuple[int, int] = 1024, patch_size: int | list[int] | tuple[int, int] = 16, hidden_act: str = 'gelu', layer_norm_eps: float = 1e-06, attention_dropout: float | int = 0.0, initializer_range: float = 1e-10, qkv_bias: bool = True, mlp_ratio: float = 4.0, use_abs_pos: bool = True, use_rel_pos: bool = True, window_size: int = 14, global_attn_indexes: list[int] | tuple[int, ...] = (2, 5, 8, 11), mlp_dim: int | None = None, downsample_channels: list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/configuration_deepseek_ocr2.py#L30)

**Parameters:**

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

output_channels (`int`, *optional*, defaults to 256) : The number of output channels in the SAM neck.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

num_channels (`int`, *optional*, defaults to `3`) : The number of input channels.

image_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `1024`) : The size (resolution) of each image.

patch_size (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) : The size (resolution) of each patch.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

layer_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the layer normalization layers.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

initializer_range (`float`, *optional*, defaults to `1e-10`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

qkv_bias (`bool`, *optional*, defaults to `True`) : Whether to add a bias to the queries, keys and values.

mlp_ratio (`float`, *optional*, defaults to `4.0`) : Ratio of the MLP hidden dim to the embedding dim.

use_abs_pos (`bool`, *optional*, defaults to `True`) : Whether to use absolute position embeddings.

use_rel_pos (`bool`, *optional*, defaults to `True`) : Whether to use relative position bias in the self-attention layers.

window_size (`int`, *optional*, defaults to 14) : Window size for windowed attention layers.

global_attn_indexes (`list[int]`, *optional*, defaults to `[2, 5, 8, 11]`) : Indices of encoder layers that use global (non-windowed) attention.

mlp_dim (`int`, *optional*) : Dimensionality of the MLP layer in each vision encoder block. Defaults to `hidden_size * mlp_ratio`.

downsample_channels (`list[int]`, *optional*) : The channel dimensions for the multi-scale downsampling neck layers. Defaults to `[512, 896]`.

This is the configuration class to store the configuration of a DeepseekOcr2Model. It is used to instantiate a Deepseek Ocr2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-community/DeepSeek-OCR-2](https://huggingface.co/deepseek-community/DeepSeek-OCR-2)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## DeepseekOcr2VisionEncoderConfig[[transformers.DeepseekOcr2VisionEncoderConfig]]

#### transformers.DeepseekOcr2VisionEncoderConfig[[transformers.DeepseekOcr2VisionEncoderConfig]]

```python
transformers.DeepseekOcr2VisionEncoderConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 151936, hidden_size: int = 4096, intermediate_size: int = 22016, num_hidden_layers: int = 32, num_attention_heads: int = 32, num_key_value_heads: int | None = 32, hidden_act: str = 'silu', max_position_embeddings: int = 32768, initializer_range: float = 0.02, rms_norm_eps: float = 1e-06, use_cache: bool = True, tie_word_embeddings: bool = False, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, use_sliding_window: bool = False, sliding_window: int | None = 4096, max_window_layers: int = 28, layer_types: list[str] | None = None, attention_dropout: float | int = 0.0, pad_token_id: int | None = None, bos_token_id: int | None = None, eos_token_id: int | list[int] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/configuration_deepseek_ocr2.py#L78)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `151936`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `4096`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `22016`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*, defaults to `32`) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `32768`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

use_sliding_window (`bool`, *optional*, defaults to `False`) : Whether to use sliding window attention.

sliding_window (`int`, *optional*, defaults to `4096`) : Sliding window attention window size. If `None`, no sliding window is applied.

max_window_layers (`int`, *optional*, defaults to `28`) : The number of layers using full attention. The first `max_window_layers` layers will use full attention, while any additional layer afterwards will use SWA (Sliding Window Attention).

layer_types (`list[str]`, *optional*) : A list that explicitly maps each layer index with its layer type. If not provided, it will be automatically generated based on config values.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*) : Token id used for end-of-stream in the vocabulary.

This is the configuration class to store the configuration of a DeepseekOcr2Model. It is used to instantiate a Deepseek Ocr2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-community/DeepSeek-OCR-2](https://huggingface.co/deepseek-community/DeepSeek-OCR-2)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import DeepseekOcr2Config

>>> config = DeepseekOcr2Config()
>>> encoder_config = config.vision_config.encoder_config
```

## DeepseekOcr2TextConfig[[transformers.DeepseekOcr2TextConfig]]

#### transformers.DeepseekOcr2TextConfig[[transformers.DeepseekOcr2TextConfig]]

```python
transformers.DeepseekOcr2TextConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 32000, hidden_size: int = 4096, intermediate_size: int = 11008, num_hidden_layers: int = 32, num_attention_heads: int = 32, num_key_value_heads: int | None = None, hidden_act: str = 'silu', max_position_embeddings: int = 2048, initializer_range: float = 0.02, rms_norm_eps: float = 1e-06, use_cache: bool = True, pad_token_id: int | None = None, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 2, pretraining_tp: int | None = 1, tie_word_embeddings: bool = False, rope_parameters: transformers.modeling_rope_utils.RopeParameters | dict | None = None, attention_bias: bool = False, attention_dropout: float | None = 0.0, mlp_bias: bool = False, head_dim: int | None = None, n_group: int | None = None, n_routed_experts: int = 64, n_shared_experts: int = 2, routed_scaling_factor: float = 1.0, topk_group: int | None = None, topk_method: str | None = 'greedy', num_experts_per_tok: int | None = None, moe_intermediate_size: int = 1407, mlp_layer_types: list[str] | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/configuration_deepseek_ocr2.py#L184)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `32000`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `4096`) : Dimension of the hidden representations.

intermediate_size (`int`, *optional*, defaults to `11008`) : Dimension of the MLP representations.

num_hidden_layers (`int`, *optional*, defaults to `32`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `32`) : Number of attention heads for each attention layer in the Transformer decoder.

num_key_value_heads (`int`, *optional*) : This is the number of key_value heads that should be used to implement Grouped Query Attention. If `num_key_value_heads=num_attention_heads`, the model will use Multi Head Attention (MHA), if `num_key_value_heads=1` the model will use Multi Query Attention (MQA) otherwise GQA is used. When converting a multi-head checkpoint to a GQA checkpoint, each group key and value head should be constructed by meanpooling all the original heads within that group. For more details, check out [this paper](https://huggingface.co/papers/2305.13245). If it is not specified, will default to `num_attention_heads`.

hidden_act (`str`, *optional*, defaults to `silu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

max_position_embeddings (`int`, *optional*, defaults to `2048`) : The maximum sequence length that this model might ever be used with.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

rms_norm_eps (`float`, *optional*, defaults to `1e-06`) : The epsilon used by the rms normalization layers.

use_cache (`bool`, *optional*, defaults to `True`) : Whether or not the model should return the last key/values attentions (not used by all models). Only relevant if `config.is_decoder=True` or when the model is a decoder-only generative model.

pad_token_id (`int`, *optional*) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

pretraining_tp (`int`, *optional*, defaults to `1`) : Experimental feature. Tensor parallelism rank used during pretraining. Please refer to [this document](https://huggingface.co/docs/transformers/main/perf_train_gpu_many#tensor-parallelism) to understand more about it. This value is necessary to ensure exact reproducibility of the pretraining results. Please refer to [this issue](https://github.com/pytorch/pytorch/issues/76232).

tie_word_embeddings (`bool`, *optional*, defaults to `False`) : Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

rope_parameters (`Union[~modeling_rope_utils.RopeParameters, dict]`, *optional*) : Dictionary containing the configuration parameters for the RoPE embeddings. The dictionary should contain a value for `rope_theta` and optionally parameters used for scaling in case you want to use RoPE with longer `max_position_embeddings`.

attention_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in the query, key, value and output projection layers during self-attention.

attention_dropout (`float`, *optional*, defaults to `0.0`) : The dropout ratio for the attention probabilities.

mlp_bias (`bool`, *optional*, defaults to `False`) : Whether to use a bias in up_proj, down_proj and gate_proj layers in the MLP layers.

head_dim (`int`, *optional*) : The attention head dimension. If None, it will default to hidden_size // num_attention_heads

n_group (`int`, *optional*) : Number of groups for grouped top-k expert routing.

n_routed_experts (`int`, *optional*, defaults to `64`) : Number of routed experts.

n_shared_experts (`int`, *optional*, defaults to `2`) : Number of shared experts.

routed_scaling_factor (`float`, *optional*, defaults to `1.0`) : Scaling factor or routed experts.

topk_group (`int`, *optional*) : Number of selected groups for each token (for each token, ensuring the selected experts is only within `topk_group` groups).

topk_method (`str`, *optional*, defaults to `"greedy"`) : Method for selecting top-k experts in MoE layers.

num_experts_per_tok (`int`, *optional*) : Number of experts to route each token to. This is the top-k value for the token-choice routing.

moe_intermediate_size (`int`, *optional*, defaults to `1407`) : Intermediate size of the routed expert MLPs.

mlp_layer_types (`list[str]`, *optional*) : MLP type (`"dense"` or `"sparse"`) for each decoder layer, e.g. `["dense", "sparse", "sparse", ...]`.

This is the configuration class to store the configuration of a DeepseekOcr2Model. It is used to instantiate a Deepseek Ocr2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [deepseek-community/DeepSeek-OCR-2](https://huggingface.co/deepseek-community/DeepSeek-OCR-2)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## DeepseekOcr2ImageProcessor[[transformers.DeepseekOcr2ImageProcessor]]

#### transformers.DeepseekOcr2ImageProcessor[[transformers.DeepseekOcr2ImageProcessor]]

```python
transformers.DeepseekOcr2ImageProcessor(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/image_processing_deepseek_ocr2.py#L134)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 1024, 'width': 1024}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

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

crop_to_patches (`bool`, *kwargs*, *optional*, defaults to `self.crop_to_patches`) : Whether to crop the image to patches. Can be overridden by the `crop_to_patches` parameter in the `preprocess` method.

min_patches (`int`, *kwargs*, *optional*, defaults to `self.min_patches`) : The minimum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is set to `True`. Can be overridden by the `min_patches` parameter in the `preprocess` method.

max_patches (`int`, *kwargs*, *optional*, defaults to `self.max_patches`) : The maximum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is set to `True`. Can be overridden by the `max_patches` parameter in the `preprocess` method.

tile_size (`int`, *kwargs*, *optional*, defaults to `768`) : The size of each local tile. Must match the model's query embedding size.

background_color (`list[int]`, *kwargs*, *optional*, defaults to `[127, 127, 127]`) : The background color for padding.

Constructs a DeepseekOcr2ImageProcessor image processor.

#### crop_image_to_patches[[transformers.DeepseekOcr2ImageProcessor.crop_image_to_patches]]

```python
crop_image_to_patches(images: torch.Tensor, min_patches: int, max_patches: int, tile_size: int, resample: PIL.Image.Resampling | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/image_processing_deepseek_ocr2.py#L154)

**Parameters:**

images (`torch.Tensor`) : The images to crop, shape `(batch, channels, height, width)`.

min_patches (`int`) : Minimum number of patches.

max_patches (`int`) : Maximum number of patches.

tile_size (`int`) : The size of each tile.

resample (`PILImageResampling`, *optional*) : Resampling filter for resizing.

**Returns:** `tuple[torch.Tensor, int]`

Stacked patches `(batch, num_patches, channels, tile_size, tile_size)`
and number of patches per image.

Crop batched images to patches based on optimal tiling.

#### get_number_of_image_patches[[transformers.DeepseekOcr2ImageProcessor.get_number_of_image_patches]]

```python
get_number_of_image_patches(height: int, width: int, images_kwargs: dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/image_processing_deepseek_ocr2.py#L288)

Returns the number of image patches for a given image size (1 global + local patches).

#### pad_to_square[[transformers.DeepseekOcr2ImageProcessor.pad_to_square]]

```python
pad_to_square(images: torch.Tensor, background_color: int | tuple[int, int, int] = 0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/image_processing_deepseek_ocr2.py#L308)

**Parameters:**

images (`torch.Tensor`) : The images to pad. Shape: (batch_size, num_channels, height, width) or (num_channels, height, width).

background_color (`int` or `tuple[int, int, int]`, *optional*, defaults to 0) : The color to use for the padding. Can be an integer for single channel or a tuple of integers representing for multi-channel images. If passed as integer in multi-channel mode, it will default to `0` in subsequent channels.

**Returns:** `torch.Tensor`

The padded images.

Pads an image to a square based on the longest edge.

## DeepseekOcr2ImageProcessorPil[[transformers.DeepseekOcr2ImageProcessorPil]]

#### transformers.DeepseekOcr2ImageProcessorPil[[transformers.DeepseekOcr2ImageProcessorPil]]

```python
transformers.DeepseekOcr2ImageProcessorPil(**kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/image_processing_pil_deepseek_ocr2.py#L144)

**Parameters:**

do_convert_rgb (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to convert the image to RGB.

do_resize (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to resize the image.

size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*, defaults to `{'height' : 1024, 'width': 1024}`): Describes the maximum input dimensions to the model.

default_to_square (`bool`, *kwargs*, *optional*, defaults to `True`) : Whether to default to a square image when resizing, if size is an int.

crop_size (`Annotated[int | list[int] | tuple[int, ...] | dict[str, int] | None, None]`, *kwargs*) : Size of the output image after applying `center_crop`.

resample (`Annotated[Union[int, PILImageResampling, NoneType], None]`, *kwargs*, defaults to `Resampling.BICUBIC`) : Resampling filter to use if resizing the image. This can be one of the enum `PILImageResampling`. Only has an effect if `do_resize` is set to `True`.

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

crop_to_patches (`bool`, *kwargs*, *optional*, defaults to `self.crop_to_patches`) : Whether to crop the image to patches. Can be overridden by the `crop_to_patches` parameter in the `preprocess` method.

min_patches (`int`, *kwargs*, *optional*, defaults to `self.min_patches`) : The minimum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is set to `True`. Can be overridden by the `min_patches` parameter in the `preprocess` method.

max_patches (`int`, *kwargs*, *optional*, defaults to `self.max_patches`) : The maximum number of patches to be extracted from the image. Only has an effect if `crop_to_patches` is set to `True`. Can be overridden by the `max_patches` parameter in the `preprocess` method.

tile_size (`int`, *kwargs*, *optional*, defaults to `768`) : The size of each local tile. Must match the model's query embedding size.

background_color (`list[int]`, *kwargs*, *optional*, defaults to `[127, 127, 127]`) : The background color for padding.

Constructs a DeepseekOcr2ImageProcessor image processor.

#### crop_image_to_patches[[transformers.DeepseekOcr2ImageProcessorPil.crop_image_to_patches]]

```python
crop_image_to_patches(image: ndarray, min_patches: int, max_patches: int, tile_size: int, resample: PILImageResampling | int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/image_processing_pil_deepseek_ocr2.py#L164)

Crop the image to patches and return a list of cropped images.

#### get_number_of_image_patches[[transformers.DeepseekOcr2ImageProcessorPil.get_number_of_image_patches]]

```python
get_number_of_image_patches(height: int, width: int, images_kwargs: dict | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/image_processing_pil_deepseek_ocr2.py#L276)

**Parameters:**

height (`int`) : Height of the input image.

width (`int`) : Width of the input image.

images_kwargs (`dict`, *optional*) : Any kwargs to override defaults of the image processor.

**Returns:** `int`

Number of patches per image.

A utility that returns number patches for a given image size.

#### pad_to_square[[transformers.DeepseekOcr2ImageProcessorPil.pad_to_square]]

```python
pad_to_square(image: ndarray, background_color: int | tuple[int, int, int] = 0)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/image_processing_pil_deepseek_ocr2.py#L311)

**Parameters:**

image (`np.ndarray`) : The image to pad. Shape: (num_channels, height, width) - always channels_first in backend.

background_color (`int` or `tuple[int, int, int]`, *optional*, defaults to 0) : The color to use for the padding.

**Returns:** `np.ndarray`

The padded image.

Pads an image to a square based on the longest edge.

## DeepseekOcr2Processor[[transformers.DeepseekOcr2Processor]]

#### transformers.DeepseekOcr2Processor[[transformers.DeepseekOcr2Processor]]

```python
transformers.DeepseekOcr2Processor(image_processor = None, tokenizer = None, chat_template = None, patch_size = 16, downsample_ratio = 4, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/processing_deepseek_ocr2.py#L44)

**Parameters:**

image_processor (`DeepseekOcr2ImageProcessor`) : The image processor is a required input.

tokenizer (`TokenizersBackend`) : The tokenizer is a required input.

chat_template (`str`) : A Jinja template to convert lists of messages in a chat into a tokenizable string.

patch_size (`int`, *optional*, defaults to `16`) : The patch size used by the vision encoder (SAM ViT patch embedding size).

downsample_ratio (`int`, *optional*, defaults to `4`) : The downsampling ratio applied after the vision encoder.

Constructs a DeepseekOcr2Processor which wraps a image processor and a tokenizer into a single processor.

[DeepseekOcr2Processor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Processor) offers all the functionalities of [DeepseekOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ImageProcessor) and [TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend). See the
[~DeepseekOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ImageProcessor) and [~TokenizersBackend](/docs/transformers/v5.15.1/en/main_classes/tokenizer#transformers.TokenizersBackend) for more information.

## DeepseekOcr2TextModel[[transformers.DeepseekOcr2TextModel]]

#### transformers.DeepseekOcr2TextModel[[transformers.DeepseekOcr2TextModel]]

```python
transformers.DeepseekOcr2TextModel(config: DeepseekOcr2TextConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L1327)

**Parameters:**

config ([DeepseekOcr2TextConfig](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2TextConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Deepseek Ocr2 Text Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DeepseekOcr2TextModel.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L1345)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPast](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPast) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekOcr2Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Config)) and inputs.

The [DeepseekOcr2TextModel](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2TextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

## DeepseekOcr2VisionModel[[transformers.DeepseekOcr2VisionModel]]

#### transformers.DeepseekOcr2VisionModel[[transformers.DeepseekOcr2VisionModel]]

```python
transformers.DeepseekOcr2VisionModel(config: DeepseekOcr2VisionConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L955)

Vision pipeline: SAM ViT-B (with neck)

#### forward[[transformers.DeepseekOcr2VisionModel.forward]]

```python
forward(pixel_values: Tensor, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L968)

**Parameters:**

pixel_values (`torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [DeepseekOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ImageProcessor). See `DeepseekOcr2ImageProcessor.__call__()` for details ([DeepseekOcr2Processor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Processor) uses [DeepseekOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ImageProcessor) for processing images).

**Returns:** [BaseModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`

A [BaseModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekOcr2Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Config)) and inputs.

The [DeepseekOcr2VisionModel](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2VisionModel) forward method, overrides the `__call__` special method.

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

## DeepseekOcr2Model[[transformers.DeepseekOcr2Model]]

#### transformers.DeepseekOcr2Model[[transformers.DeepseekOcr2Model]]

```python
transformers.DeepseekOcr2Model(config: DeepseekOcr2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L1406)

**Parameters:**

config ([DeepseekOcr2Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Llava-Next model which consists of a vision backbone and a language model without language modeling head.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DeepseekOcr2Model.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_values_local: typing.Optional[torch.FloatTensor] = None, num_local_patches: typing.Union[list[int], torch.Tensor, NoneType] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, use_cache: bool | None = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L1504)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [DeepseekOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ImageProcessor). See `DeepseekOcr2ImageProcessor.__call__()` for details ([DeepseekOcr2Processor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Processor) uses [DeepseekOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ImageProcessor) for processing images).

pixel_values_local (`torch.FloatTensor`, *optional*) : Local patch pixel values of shape `(total_patches, 3, H, W)`.

num_local_patches (`list[int]` or `torch.Tensor`, *optional*) : Number of local patches per image in the batch.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

**Returns:** `DeepseekOcr2ModelOutputWithPast` or `tuple(torch.FloatTensor)`

A `DeepseekOcr2ModelOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekOcr2Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Config)) and inputs.

The [DeepseekOcr2Model](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.

  If `past_key_values` is used only the last hidden-state of the sequences of shape `(batch_size, 1,
  hidden_size)` is output.
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

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

#### get_image_features[[transformers.DeepseekOcr2Model.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, pixel_values_local: typing.Optional[torch.FloatTensor] = None, num_local_patches: typing.Union[list[int], torch.Tensor, NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L1425)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) : The tensors corresponding to the input images. Pixel values can be obtained using [DeepseekOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ImageProcessor). See `DeepseekOcr2ImageProcessor.__call__()` for details ([DeepseekOcr2Processor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Processor) uses [DeepseekOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ImageProcessor) for processing images).

pixel_values_local (`torch.FloatTensor` of shape `(total_patches, 3, height, width)`, *optional*) : All local patches flattened across the batch, or `None` if no local views.

num_local_patches (`list[int]` or `torch.Tensor`, *optional*) : Number of local patches per image, e.g. `[6, 0, 4]`.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekOcr2Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Config)) and inputs.

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

#### get_placeholder_mask[[transformers.DeepseekOcr2Model.get_placeholder_mask]]

```python
get_placeholder_mask(input_ids: LongTensor, inputs_embeds: FloatTensor, image_features: FloatTensor)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L1481)

Obtains multimodal placeholder mask from `input_ids` or `inputs_embeds`, and checks that the placeholder token count is
equal to the length of multimodal features. If the lengths are different, an error is raised.

## DeepseekOcr2ForConditionalGeneration[[transformers.DeepseekOcr2ForConditionalGeneration]]

#### transformers.DeepseekOcr2ForConditionalGeneration[[transformers.DeepseekOcr2ForConditionalGeneration]]

```python
transformers.DeepseekOcr2ForConditionalGeneration(config: DeepseekOcr2Config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L1558)

**Parameters:**

config ([DeepseekOcr2Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Config)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Deepseek Ocr2 Model for token generation conditioned on other modalities (e.g. image-text-to-text generation).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.DeepseekOcr2ForConditionalGeneration.forward]]

```python
forward(input_ids: typing.Optional[torch.LongTensor] = None, pixel_values: typing.Optional[torch.FloatTensor] = None, pixel_values_local: typing.Optional[torch.FloatTensor] = None, num_local_patches: typing.Union[list[int], torch.Tensor, NoneType] = None, attention_mask: typing.Optional[torch.Tensor] = None, position_ids: typing.Optional[torch.LongTensor] = None, past_key_values: transformers.cache_utils.Cache | None = None, inputs_embeds: typing.Optional[torch.FloatTensor] = None, labels: typing.Optional[torch.LongTensor] = None, use_cache: bool | None = None, logits_to_keep: typing.Union[int, torch.Tensor] = 0, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L1597)

**Parameters:**

input_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of input sequence tokens in the vocabulary. Padding will be ignored by default.  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoTokenizer). See [PreTrainedTokenizer.encode()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.encode) and [PreTrainedTokenizer.__call__()](/docs/transformers/v5.15.1/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.  [What are input IDs?](../glossary#input-ids)

pixel_values (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) : The tensors corresponding to the input images. Pixel values can be obtained using [DeepseekOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ImageProcessor). See `DeepseekOcr2ImageProcessor.__call__()` for details ([DeepseekOcr2Processor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Processor) uses [DeepseekOcr2ImageProcessor](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ImageProcessor) for processing images).

pixel_values_local (`torch.FloatTensor`, *optional*) : Local patch pixel values of shape `(total_patches, 3, H, W)`.

num_local_patches (`list[int]` or `torch.Tensor`, *optional*) : Number of local patches per image in the batch.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

position_ids (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices of positions of each input sequence tokens in the position embeddings. Selected in the range `[0, config.n_positions - 1]`.  [What are position IDs?](../glossary#position-ids)

past_key_values (`~cache_utils.Cache`, *optional*) : Pre-computed hidden-states (key and values in the self-attention blocks and in the cross-attention blocks) that can be used to speed up sequential decoding. This typically consists in the `past_key_values` returned by the model at a previous stage of decoding, when `use_cache=True` or `config.use_cache=True`.  Only [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance is allowed as input, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache). If no `past_key_values` are passed, [DynamicCache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.DynamicCache) will be initialized by default.  The model will output the same cache format that is fed as input.  If `past_key_values` are used, the user is expected to input only unprocessed `input_ids` (those that don't have their past key value states given to this model) of shape `(batch_size, unprocessed_length)` instead of all `input_ids` of shape `(batch_size, sequence_length)`.

inputs_embeds (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) : Optionally, instead of passing `input_ids` you can choose to directly pass an embedded representation. This is useful if you want more control over how to convert `input_ids` indices into associated vectors than the model's internal embedding lookup matrix.

labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) : Labels for computing the masked language modeling loss. Indices should either be in `[0, ..., config.vocab_size]` or -100 (see `input_ids` docstring). Tokens with indices set to `-100` are ignored (masked), the loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`.

use_cache (`bool`, *optional*) : If set to `True`, `past_key_values` key value states are returned and can be used to speed up decoding (see `past_key_values`).

logits_to_keep (`Union[int, torch.Tensor]`, *optional*, defaults to `0`) : If an `int`, compute logits for the last `logits_to_keep` tokens. If `0`, calculate logits for all `input_ids` (special case). Only last token logits are needed for generation, and calculating them only for that token can save memory, which becomes pretty significant for long sequences or large vocabulary size. If a `torch.Tensor`, must be 1D corresponding to the indices to keep in the sequence length dimension. This is useful when using packed tensor format (single dimension for batch and sequence length).

**Returns:** `DeepseekOcr2CausalLMOutputWithPast` or `tuple(torch.FloatTensor)`

A `DeepseekOcr2CausalLMOutputWithPast` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekOcr2Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Config)) and inputs.

The [DeepseekOcr2ForConditionalGeneration](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2ForConditionalGeneration) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.vocab_size)`) -- Prediction scores of the language modeling head (scores for each vocabulary token before SoftMax).
- **past_key_values** (`Cache`, *optional*, returned when `use_cache=True` is passed or when `config.use_cache=True`) -- It is a [Cache](/docs/transformers/v5.15.1/en/internal/generation_utils#transformers.Cache) instance. For more details, see our [kv cache guide](https://huggingface.co/docs/transformers/en/kv_cache).

  Contains pre-computed hidden-states (key and values in the self-attention blocks) that can be used (see
  `past_key_values` input) to speed up sequential decoding.
- **hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **image_hidden_states** (`torch.FloatTensor`, *optional*) -- A `torch.FloatTensor` of size (batch_size * num_patches, num_images, sequence_length, hidden_size)`.
  image_hidden_states of the model produced by the vision encoder and after projecting the last hidden state.

Example:

```python
>>> from PIL import Image
>>> from transformers import AutoProcessor, DeepseekOcr2ForConditionalGeneration

>>> model = DeepseekOcr2ForConditionalGeneration.from_pretrained("deepseek-community/DeepSeek-OCR-2")
>>> processor = AutoProcessor.from_pretrained("deepseek-community/DeepSeek-OCR-2")

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

#### get_image_features[[transformers.DeepseekOcr2ForConditionalGeneration.get_image_features]]

```python
get_image_features(pixel_values: FloatTensor, pixel_values_local: typing.Optional[torch.FloatTensor] = None, num_local_patches: typing.Union[list[int], torch.Tensor, NoneType] = None, **kwargs: Unpack)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/deepseek_ocr2/modeling_deepseek_ocr2.py#L1573)

**Parameters:**

pixel_values (`torch.FloatTensor` of shape `(batch_size, 3, height, width)`) : The tensors corresponding to the global view input images.

pixel_values_local (`torch.FloatTensor` of shape `(total_patches, 3, height, width)`, *optional*) : All local patches flattened across the batch, or `None` if no local views.

num_local_patches (`list[int]` or `torch.Tensor`, *optional*) : Number of local patches per image, e.g. `[6, 0, 4]`.

**Returns:** [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or `tuple(torch.FloatTensor)`

A [BaseModelOutputWithPooling](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutputWithPooling) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([DeepseekOcr2Config](/docs/transformers/v5.15.1/en/model_doc/deepseek_ocr2#transformers.DeepseekOcr2Config)) and inputs.

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
>>> from transformers import AutoProcessor, DeepseekOcr2ForConditionalGeneration

>>> model = DeepseekOcr2ForConditionalGeneration.from_pretrained("deepseek-community/DeepSeek-OCR-2")
>>> processor = AutoProcessor.from_pretrained("deepseek-community/DeepSeek-OCR-2")

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
