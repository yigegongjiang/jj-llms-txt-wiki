# SAM3-LiteText

## Overview

SAM3-LiteText was proposed in [SAM3-LiteText: An Anatomical Study of the SAM3 Text Encoder for Efficient Vision-Language Segmentation](https://huggingface.co/papers/2602.12173) by Chengxi Zeng, Yuxuan Jiang, Ge Gao, Shuai Wang, Duolikun Danier, Bin Zhu, Stevan Rudinac, David Bull, and Fan Zhang.

SAM3-LiteText is a lightweight variant of [SAM3](sam3) that replaces the heavy SAM3 text encoder (353M parameters) with a compact MobileCLIP-based text encoder optimized through knowledge distillation. The SAM3 ViT-H image encoder is kept intact. This reduces text encoder parameters by up to 88% while maintaining segmentation performance comparable to the original model.

The abstract from the paper is the following:

*Vision-language segmentation models such as SAM3 enable flexible, prompt-driven visual grounding, but inherit large, general-purpose text encoders originally designed for open-ended language understanding. In practice, segmentation prompts are short, structured, and semantically constrained, leading to substantial over-provisioning in text encoder capacity and persistent computational and memory overhead. In this paper, we perform a large-scale anatomical analysis of text prompting in vision-language segmentation, covering 404,796 real prompts across multiple benchmarks. Our analysis reveals severe redundancy: most context windows are underutilized, vocabulary usage is highly sparse, and text embeddings lie on low-dimensional manifold despite high-dimensional representations. Motivated by these findings, we propose SAM3-LiteText, a lightweight text encoding framework that replaces the original SAM3 text encoder with a compact MobileCLIP student that is optimized by knowledge distillation. Extensive experiments on image and video segmentation benchmarks show that SAM3-LiteText reduces text encoder parameters by up to 88%, substantially reducing static memory footprint, while maintaining segmentation performance comparable to the original model.*

The text encoder architecture is based on [MobileCLIP](https://huggingface.co/papers/2311.17049) and comes in three variants:

| Variant | Text Encoder | Text Params | Reduction |
|---|---|---|---|
| SAM3-LiteText-S0-16 | MobileCLIP-S0 | 42.54M | ~88% |
| SAM3-LiteText-S1-16 | MobileCLIP-S1 | 63.53M | ~82% |
| SAM3-LiteText-L-16 | MobileCLIP2-L | 123.80M | ~65% |

This model was contributed by [nielsr](https://huggingface.co/nielsr) and [yonigozlan](https://huggingface.co/yonigozlan).
The original code can be found [here](https://github.com/SimonZeng7108/efficientsam3/tree/sam3_litetext).

## Usage

SAM3-LiteText is a drop-in replacement for SAM3 with a lightweight text encoder. It uses the same processor ([Sam3Processor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3Processor)) and supports the same prompting interface. Refer to the [SAM3 documentation](sam3) for detailed usage examples including text prompts, box prompts, batched inference, and more.

```python
from io import BytesIO

import httpx
from PIL import Image

from transformers import AutoModel, AutoProcessor

model = AutoModel.from_pretrained("yonigozlan/sam3-litetext-s0", device_map="auto")
processor = AutoProcessor.from_pretrained("yonigozlan/sam3-litetext-s0")

image_url = "http://images.cocodataset.org/val2017/000000077595.jpg"
image = Image.open(BytesIO(httpx.get(image_url).content)).convert("RGB")

inputs = processor(images=image, text="ear", return_tensors="pt").to(model.device)

outputs = model(**inputs)

results = processor.post_process_instance_segmentation(
    outputs,
    threshold=0.5,
    mask_threshold=0.5,
    target_sizes=inputs.get("original_sizes").tolist(),
)[0]

print(f"Found {len(results['masks'])} objects")
```

## Sam3LiteTextConfig[[transformers.Sam3LiteTextConfig]]

- **vision_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the vision backbone.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **geometry_encoder_config** (`dict` or `Sam3LiteTextGeometryEncoderConfig`, *optional*) --
  Configuration for the geometry encoder.
- **detr_encoder_config** (`dict` or `Sam3LiteTextDETREncoderConfig`, *optional*) --
  Configuration for the DETR encoder.
- **detr_decoder_config** (`dict` or `Sam3LiteTextDETRDecoderConfig`, *optional*) --
  Configuration for the DETR decoder.
- **mask_decoder_config** (`dict` or `Sam3LiteTextMaskDecoderConfig`, *optional*) --
  Configuration for the mask decoder.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3LiteTextModel. It is used to instantiate a Sam3 Lite Text
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/sam3-litetext-s0](https://huggingface.co/yonigozlan/sam3-litetext-s0)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:
```python
>>> from transformers import Sam3LiteTextConfig, Sam3LiteTextModel

>>> # Initializing a SAM3_LITE_TEXT configuration
>>> configuration = Sam3LiteTextConfig()

>>> # Initializing a model from the configuration
>>> model = Sam3LiteTextModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## Sam3LiteTextTextConfig[[transformers.Sam3LiteTextTextConfig]]

- **vocab_size** (`int`, *optional*, defaults to `49408`) --
  Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.
- **hidden_size** (`int`, *optional*, defaults to `512`) --
  Dimension of the hidden representations.
- **intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the MLP representations.
- **projection_dim** (`int`, *optional*, defaults to `512`) --
  Dimensionality of text and vision projection layers.
- **num_hidden_layers** (`int`, *optional*, defaults to `12`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **max_position_embeddings** (`int`, *optional*, defaults to `77`) --
  The maximum sequence length that this model might ever be used with.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **attention_dropout** (`float`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **use_repmixer_blocks** (`bool`, *optional*, defaults to `True`) --
  Whether to use RepMixer blocks (MobileCLIP-style) for the first and last encoder layers.
  When `False`, all layers are standard Transformer encoder layers.
- **layer_scale_init_value** (`float`, *optional*, defaults to `1e-5`) --
  Initial value for the learnable layer-scale parameters in RepMixer blocks (residual branches).
- **repmixer_kernel_size** (`int`, *optional*, defaults to `11`) --
  Kernel size for depthwise convolutions in RepMixer blocks (token mixer and convolutional feed-forward path).

This is the configuration class to store the configuration of a Sam3LiteTextModel. It is used to instantiate a Sam3 Lite Text
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/sam3-litetext-s0](https://huggingface.co/yonigozlan/sam3-litetext-s0)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3LiteTextGeometryEncoderConfig[[transformers.Sam3LiteTextGeometryEncoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **num_layers** (`int`, *optional*, defaults to `3`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the MLP representations.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **hidden_act** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **roi_size** (`int`, *optional*, defaults to 7) --
  ROI size for box pooling operations.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3LiteTextModel. It is used to instantiate a Sam3 Lite Text
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/sam3-litetext-s0](https://huggingface.co/yonigozlan/sam3-litetext-s0)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3LiteTextDETREncoderConfig[[transformers.Sam3LiteTextDETREncoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **num_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer decoder.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the MLP representations.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **hidden_act** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout** (`float`, *optional*, defaults to 0.0) --
  Dropout probability for hidden states.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3LiteTextModel. It is used to instantiate a Sam3 Lite Text
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/sam3-litetext-s0](https://huggingface.co/yonigozlan/sam3-litetext-s0)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3LiteTextDETRDecoderConfig[[transformers.Sam3LiteTextDETRDecoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **num_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer decoder.
- **num_queries** (`int`, *optional*, defaults to 200) --
  Number of object queries.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **intermediate_size** (`int`, *optional*, defaults to `2048`) --
  Dimension of the MLP representations.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **hidden_act** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **hidden_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3LiteTextModel. It is used to instantiate a Sam3 Lite Text
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/sam3-litetext-s0](https://huggingface.co/yonigozlan/sam3-litetext-s0)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3LiteTextMaskDecoderConfig[[transformers.Sam3LiteTextMaskDecoderConfig]]

- **hidden_size** (`int`, *optional*, defaults to `256`) --
  Dimension of the hidden representations.
- **num_upsampling_stages** (`int`, *optional*, defaults to 3) --
  Number of upsampling stages in the pixel decoder (FPN).
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The ratio for all dropout layers.
- **num_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

This is the configuration class to store the configuration of a Sam3LiteTextModel. It is used to instantiate a Sam3 Lite Text
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [yonigozlan/sam3-litetext-s0](https://huggingface.co/yonigozlan/sam3-litetext-s0)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

## Sam3LiteTextTextModel[[transformers.Sam3LiteTextTextModel]]

- **config** ([Sam3LiteTextTextConfig](/docs/transformers/v5.14.0/en/model_doc/sam3_lite_text#transformers.Sam3LiteTextTextConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

MobileCLIP MCT text encoder used in EfficientSAM3 LiteText.

When `config.use_repmixer_blocks` is `True`, the first and last layers are
`Sam3LiteTextRepMixerBlock` modules; the rest are standard `Sam3LiteTextTextEncoderLayer` layers.

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

  [What are attention masks?](../glossary#attention-mask)`Sam3LiteTextTextEncoderOutput` or `tuple(torch.FloatTensor)`A `Sam3LiteTextTextEncoderOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam3LiteTextConfig](/docs/transformers/v5.14.0/en/model_doc/sam3_lite_text#transformers.Sam3LiteTextConfig)) and inputs.
The [Sam3LiteTextTextModel](/docs/transformers/v5.14.0/en/model_doc/sam3_lite_text#transformers.Sam3LiteTextTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Full sequence of hidden states from the text encoder.
- **pooler_output** (`torch.FloatTensor` of shape `(batch_size, projection_dim)`) -- EOT-pooled output projected to `projection_dim` via the internal CLIP-style projection.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of hidden states at each layer, returned when `output_hidden_states=True`.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*) -- Tuple of attention weights at each transformer layer, returned when `output_attentions=True`.

## Sam3LiteTextModel[[transformers.Sam3LiteTextModel]]

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`, *optional*) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [Sam3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3ImageProcessor). See `Sam3ImageProcessor.__call__()` for details ([Sam3Processor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3Processor) uses
  [Sam3ImageProcessor](/docs/transformers/v5.14.0/en/model_doc/sam3#transformers.Sam3ImageProcessor) for processing images).
- **vision_embeds** (`Sam3LiteTextVisionEncoderOutput`, *optional*) --
  Pre-computed vision embeddings. Can be used to easily reuse vision embeddings. If provided, `pixel_values`
  should not be passed. Mutually exclusive with `pixel_values`.
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
- **text_embeds** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) --
  Pre-computed text embeddings. Can be used to easily reuse text embeddings. If provided, `input_ids`
  should not be passed. Mutually exclusive with `input_ids`.
- **input_boxes** (`torch.FloatTensor` of shape `(batch_size, num_boxes, 4)`, *optional*) --
  Normalized box coordinates in [0, 1] range, in (cx, cy, w, h) format.
- **input_boxes_labels** (`torch.LongTensor` of shape `(batch_size, num_boxes)`, *optional*) --
  Labels for boxes: 1 (positive), 0 (negative).`Sam3LiteTextImageSegmentationOutput` or `tuple(torch.FloatTensor)`A `Sam3LiteTextImageSegmentationOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([Sam3LiteTextConfig](/docs/transformers/v5.14.0/en/model_doc/sam3_lite_text#transformers.Sam3LiteTextConfig)) and inputs.
The [Sam3LiteTextModel](/docs/transformers/v5.14.0/en/model_doc/sam3_lite_text#transformers.Sam3LiteTextModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **pred_masks** (`torch.FloatTensor` of shape `(batch_size, num_queries, height, width)`) -- Predicted segmentation masks for each query.
- **pred_boxes** (`torch.FloatTensor` of shape `(batch_size, num_queries, 4)`) -- Predicted bounding boxes in (x1, y1, x2, y2) format.
- **pred_logits** (`torch.FloatTensor` of shape `(batch_size, num_queries)`, *optional*) -- Classification confidence scores for each query, computed via dot product between
  decoder query features and text features.
- **presence_logits** (`torch.FloatTensor` of shape `(batch_size, 1)`, *optional*) -- Presence logits from the DETR decoder presence token (last layer only). These indicate whether objects
  are present in the scene. Can be used to compute final scores by multiplying with pred_logits:
  `final_scores = pred_logits.sigmoid() * presence_logits.sigmoid()`.
- **semantic_seg** (`torch.FloatTensor` of shape `(batch_size, 1, height, width)`, *optional*) -- Semantic segmentation output.
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*) -- Tuple of hidden states from all DETR decoder layers. Each tensor has shape `(batch_size, num_queries, hidden_size)`.
- **decoder_reference_boxes** (`torch.FloatTensor` of shape `(num_layers, batch_size, num_queries, 4)`, *optional*) -- Reference boxes from all DETR decoder layers.
- **encoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*) -- Tuple of hidden states from all DETR encoder layers.
- **vision_hidden_states** (`tuple[torch.FloatTensor]`, *optional*) -- Tuple of hidden states from all vision encoder (ViT) layers.
- **vision_attentions** (`tuple[torch.FloatTensor]`, *optional*) -- Attention weights from vision encoder (ViT) layers.
- **detr_encoder_attentions** (`tuple[torch.FloatTensor]`, *optional*) -- Attention weights from DETR encoder layers.
- **detr_decoder_attentions** (`tuple[torch.FloatTensor]`, *optional*) -- Attention weights from DETR decoder layers (self-attention and cross-attention).
- **mask_decoder_attentions** (`tuple[torch.FloatTensor]`, *optional*) -- Attention weights from mask decoder layers.

Example:

```python
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO
>>> from transformers import AutoModel, AutoProcessor

>>> model = AutoModel.from_pretrained("facebook/sam3_lite_text")
>>> processor = AutoProcessor.from_pretrained("facebook/sam3_lite_text")

>>> url = "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/model_doc/sam-car.png"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read())).convert("RGB")
>>> text = "car"
>>> inputs = processor(images=image, text=text, return_tensors="pt")

>>> # Get segmentation output
>>> outputs = model(**inputs)
>>> pred_masks = outputs.pred_masks
>>> pred_boxes = outputs.pred_boxes
```

## Sam3LiteTextPreTrainedModel[[transformers.Sam3LiteTextPreTrainedModel]]

- **config** ([PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

A mock value for a dotted path (e.g. `torch.float32`): attribute access chains,
calls behave as pass-through decorators, `repr` is the dotted path, and using it
as a base class substitutes a plain-`type` base (PEP 560 `__mro_entries__`), so
real subclasses keep a normal metaclass and `inspect.signature` reads their real
`__init__` instead of a mock's.
