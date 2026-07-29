# MM Grounding DINO

[MM Grounding DINO](https://huggingface.co/papers/2401.02361) model was proposed in [An Open and Comprehensive Pipeline for Unified Object Grounding and Detection](https://huggingface.co/papers/2401.02361) by Xiangyu Zhao, Yicheng Chen, Shilin Xu, Xiangtai Li, Xinjiang Wang, Yining Li, Haian Huang>.

MM Grounding DINO improves upon the [Grounding DINO](https://huggingface.co/docs/transformers/model_doc/grounding-dino) by improving the contrastive class head and removing the parameter sharing in the decoder, improving zero-shot detection performance on both COCO (50.6(+2.2) AP) and LVIS (31.9(+11.8) val AP and 41.4(+12.6) minival AP).

You can find all the original MM Grounding DINO checkpoints under the [MM Grounding DINO](https://huggingface.co/collections/openmmlab-community/mm-grounding-dino-688cbde05b814c4e2832f9df) collection. This model also supports LLMDet inference. You can find LLMDet checkpoints under the [LLMDet](https://huggingface.co/collections/iSEE-Laboratory/llmdet-688475906dc235d5f1dc678e) collection.

> [!TIP]
> Click on the MM Grounding DINO models in the right sidebar for more examples of how to apply MM Grounding DINO to different MM Grounding DINO tasks.

The example below demonstrates how to generate text based on an image with the [AutoModelForZeroShotObjectDetection](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoModelForZeroShotObjectDetection) class.

```python
import torch

from transformers import AutoModelForZeroShotObjectDetection, AutoProcessor
from transformers.image_utils import load_image

# Prepare processor and model
model_id = "openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg_v3det"
processor = AutoProcessor.from_pretrained(model_id)
model = AutoModelForZeroShotObjectDetection.from_pretrained(model_id, device_map="auto")

# Prepare inputs
image_url = "http://images.cocodataset.org/val2017/000000039769.jpg"
image = load_image(image_url)
text_labels = [["a cat", "a remote control"]]
inputs = processor(images=image, text=text_labels, return_tensors="pt").to(model.device)

# Run inference
with torch.no_grad():
    outputs = model(**inputs)

# Postprocess outputs
results = processor.post_process_grounded_object_detection(
    outputs,
    threshold=0.4,
    target_sizes=[(image.height, image.width)]
)

# Retrieve the first image result
result = results[0]
for box, score, labels in zip(result["boxes"], result["scores"], result["labels"]):
    box = [round(x, 2) for x in box.tolist()]
    print(f"Detected {labels} with confidence {round(score.item(), 3)} at location {box}")
```

## Notes

- Here's a table of models and their object detection performance results on COCO (results from [official repo](https://github.com/open-mmlab/mmdetection/blob/main/configs/mm_grounding_dino/README.md)):

    |                                                              Model                                                             | Backbone |      Pre-Train Data      |   Style   |  COCO mAP  |
    | ------------------------------------------------------------------------------------------------------------------------------ | -------- | ------------------------ | --------- | ---------- |
    |  [mm_grounding_dino_tiny_o365v1_goldg](https://huggingface.co/openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg)                       |  Swin-T  |        O365,GoldG        | Zero-shot | 50.4(+2.3) |
    |  [mm_grounding_dino_tiny_o365v1_goldg_grit](https://huggingface.co/openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg_grit)             |  Swin-T  |     O365,GoldG,GRIT      | Zero-shot | 50.5(+2.1) |
    |  [mm_grounding_dino_tiny_o365v1_goldg_v3det](https://huggingface.co/openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg_v3det)           |  Swin-T  |     O365,GoldG,V3Det     | Zero-shot | 50.6(+2.2) |
    |  [mm_grounding_dino_tiny_o365v1_goldg_grit_v3det](https://huggingface.co/openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg_grit_v3det) |  Swin-T  |  O365,GoldG,GRIT,V3Det   | Zero-shot | 50.4(+2.0) |
    |  [mm_grounding_dino_base_o365v1_goldg_v3det](https://huggingface.co/openmmlab-community/mm_grounding_dino_base_o365v1_goldg_v3det)           |  Swin-B  |     O365,GoldG,V3Det     | Zero-shot |    52.5    |
    |  [mm_grounding_dino_base_all](https://huggingface.co/openmmlab-community/mm_grounding_dino_base_all)                                         |  Swin-B  |         O365,ALL         |     -     |    59.5    |
    |  [mm_grounding_dino_large_o365v2_oiv6_goldg](https://huggingface.co/openmmlab-community/mm_grounding_dino_large_o365v2_oiv6_goldg)           |  Swin-L  | O365V2,OpenImageV6,GoldG | Zero-shot |    53.0    |
    |  [mm_grounding_dino_large_all](https://huggingface.co/openmmlab-community/mm_grounding_dino_large_all)                                       |  Swin-L  |  O365V2,OpenImageV6,ALL  |     -     |    60.3    |

- Here's a table of MM Grounding DINO tiny models and their object detection performance on LVIS (results from [official repo](https://github.com/open-mmlab/mmdetection/blob/main/configs/mm_grounding_dino/README.md)):

    |                                                              Model                                                             |    Pre-Train Data     | MiniVal APr | MiniVal APc | MiniVal APf | MiniVal AP  | Val1.0 APr | Val1.0 APc | Val1.0 APf |  Val1.0 AP  |
    | ------------------------------------------------------------------------------------------------------------------------------ | --------------------- | ----------- | ----------- | ----------- | ----------- | ---------- | ---------- | ---------- | ----------- |
    |  [mm_grounding_dino_tiny_o365v1_goldg](https://huggingface.co/openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg)                       |      O365,GoldG       |    28.1     |    30.2     |    42.0     | 35.7(+6.9)  |    17.1    |    22.4    |    36.5    | 27.0(+6.9)  |
    |  [mm_grounding_dino_tiny_o365v1_goldg_grit](https://huggingface.co/openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg_grit)             |    O365,GoldG,GRIT    |    26.6     |    32.4     |    41.8     | 36.5(+7.7)  |    17.3    |    22.6    |    36.4    | 27.1(+7.0)  |
    |  [mm_grounding_dino_tiny_o365v1_goldg_v3det](https://huggingface.co/openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg_v3det)           |   O365,GoldG,V3Det    |    33.0     |    36.0     |    45.9     | 40.5(+11.7) |    21.5    |    25.5    |    40.2    | 30.6(+10.5) |
    |  [mm_grounding_dino_tiny_o365v1_goldg_grit_v3det](https://huggingface.co/openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg_grit_v3det) | O365,GoldG,GRIT,V3Det |    34.2     |    37.4     |    46.2     | 41.4(+12.6) |    23.6    |    27.6    |    40.5    | 31.9(+11.8) |

- This implementation also supports inference for [LLMDet](https://github.com/iSEE-Laboratory/LLMDet). Here's a table of LLMDet models and their performance on LVIS (results from [official repo](https://github.com/iSEE-Laboratory/LLMDet)):

    |                             Model                         | Pre-Train Data            |  MiniVal APr | MiniVal APc | MiniVal APf | MiniVal AP  | Val1.0 APr | Val1.0 APc | Val1.0 APf |  Val1.0 AP  |
    | --------------------------------------------------------- | -------------------------------------------- | ------------ | ----------- | ----------- | ----------- | ---------- | ---------- | ---------- | ----------- |
    | [llmdet_tiny](https://huggingface.co/iSEE-Laboratory/llmdet_tiny)   | (O365,GoldG,GRIT,V3Det) + GroundingCap-1M    | 44.7         | 37.3        | 39.5        | 50.7        | 34.9       | 26.0       | 30.1       | 44.3        |
    | [llmdet_base](https://huggingface.co/iSEE-Laboratory/llmdet_base)   | (O365,GoldG,V3Det) + GroundingCap-1M         | 48.3         | 40.8        | 43.1        | 54.3        | 38.5       | 28.2       | 34.3       | 47.8        |
    | [llmdet_large](https://huggingface.co/iSEE-Laboratory/llmdet_large) | (O365V2,OpenImageV6,GoldG) + GroundingCap-1M | 51.1         | 45.1        | 46.1        | 56.6        | 42.0       | 31.6       | 38.8       | 50.2        |

## MMGroundingDinoConfig[[transformers.MMGroundingDinoConfig]]

- **is_encoder_decoder** (`bool`, *optional*, defaults to `True`) --
  Whether the model is used as an encoder/decoder or not.
- **backbone_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The configuration of the backbone model.
- **text_config** (`Union[dict, ~configuration_utils.PreTrainedConfig]`, *optional*) --
  The config object or dictionary of the text backbone.
- **num_queries** (`int`, *optional*, defaults to 900) --
  Number of object queries, i.e. detection slots. This is the maximal number of objects
  [MMGroundingDinoModel](/docs/transformers/v5.14.0/en/model_doc/mm-grounding-dino#transformers.MMGroundingDinoModel) can detect in a single image.
- **encoder_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer encoder. Will use the same value as `num_layers` if not set.
- **encoder_ffn_dim** (`int`, *optional*, defaults to `2048`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in encoder.
- **encoder_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer encoder.
- **decoder_layers** (`int`, *optional*, defaults to `6`) --
  Number of hidden layers in the Transformer decoder. Will use the same value as `num_layers` if not set.
- **decoder_ffn_dim** (`int`, *optional*, defaults to `2048`) --
  Dimensionality of the "intermediate" (often named feed-forward) layer in decoder.
- **decoder_attention_heads** (`int`, *optional*, defaults to `8`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **activation_function** (`str`, *optional*, defaults to `relu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **d_model** (`int`, *optional*, defaults to `256`) --
  Size of the encoder layers and the pooler layer.
- **dropout** (`Union[float, int]`, *optional*, defaults to `0.1`) --
  The ratio for all dropout layers.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **activation_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for activations inside the fully connected layer.
- **auxiliary_loss** (`bool`, *optional*, defaults to `False`) --
  Whether auxiliary decoding losses (loss at each decoder layer) are to be used.
- **position_embedding_type** (`str`, *optional*, defaults to `"sine"`) --
  Type of position embeddings to be used on top of the image features. One of `"sine"` or `"learned"`.
- **num_feature_levels** (`int`, *optional*, defaults to 4) --
  The number of input feature levels.
- **encoder_n_points** (`int`, *optional*, defaults to 4) --
  The number of sampled keys in each feature level for each attention head in the encoder.
- **decoder_n_points** (`int`, *optional*, defaults to 4) --
  The number of sampled keys in each feature level for each attention head in the decoder.
- **two_stage** (`bool`, *optional*, defaults to `True`) --
  Whether to apply a two-stage deformable DETR, where the region proposals are also generated by a variant of
  Grounding DINO, which are further fed into the decoder for iterative bounding box refinement.
- **class_cost** (`float`, *optional*, defaults to `1.0`) --
  Relative weight of the classification error in the Hungarian matching cost.
- **bbox_cost** (`float`, *optional*, defaults to `5.0`) --
  Relative weight of the L1 bounding box error in the Hungarian matching cost.
- **giou_cost** (`float`, *optional*, defaults to `2.0`) --
  Relative weight of the generalized IoU loss in the Hungarian matching cost.
- **bbox_loss_coefficient** (`float`, *optional*, defaults to `5.0`) --
  Relative weight of the L1 bounding box loss in the panoptic segmentation loss.
- **giou_loss_coefficient** (`float`, *optional*, defaults to `2.0`) --
  Relative weight of the generalized IoU loss in the panoptic segmentation loss.
- **focal_alpha** (`float`, *optional*, defaults to `0.25`) --
  Alpha parameter in the focal loss.
- **disable_custom_kernels** (`bool`, *optional*, defaults to `False`) --
  Disable the use of custom CUDA and CPU kernels. This option is necessary for the ONNX export, as custom
  kernels are not supported by PyTorch ONNX export.
- **max_text_len** (`int`, *optional*, defaults to 256) --
  The maximum length of the text input.
- **text_enhancer_dropout** (`float`, *optional*, defaults to 0.0) --
  The dropout ratio for the text enhancer.
- **fusion_droppath** (`float`, *optional*, defaults to 0.1) --
  The droppath ratio for the fusion module.
- **fusion_dropout** (`float`, *optional*, defaults to 0.0) --
  The dropout ratio for the fusion module.
- **embedding_init_target** (`bool`, *optional*, defaults to `True`) --
  Whether to initialize the target with Embedding weights.
- **query_dim** (`int`, *optional*, defaults to 4) --
  The dimension of the query vector.
- **positional_embedding_temperature** (`float`, *optional*, defaults to 20) --
  The temperature for Sine Positional Embedding that is used together with vision backbone.
- **init_std** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-05`) --
  The epsilon used by the layer normalization layers.
- **tie_word_embeddings** (`bool`, *optional*, defaults to `True`) --
  Whether to tie weight embeddings according to model's `tied_weights_keys` mapping.

This is the configuration class to store the configuration of a Mm Grounding DinoModel. It is used to instantiate a Mm Grounding Dino
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg_v3det](https://huggingface.co/openmmlab-community/mm_grounding_dino_tiny_o365v1_goldg_v3det)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Examples:

```python
>>> from transformers import MMGroundingDinoConfig, MMGroundingDinoModel

>>> # Initializing a MM Grounding DINO configuration
>>> configuration = MMGroundingDinoConfig()

>>> # Initializing a model (with random weights) from the configuration
>>> model = MMGroundingDinoModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## MMGroundingDinoModel[[transformers.MMGroundingDinoModel]]

- **config** ([MMGroundingDinoConfig](/docs/transformers/v5.14.0/en/model_doc/mm-grounding-dino#transformers.MMGroundingDinoConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Grounding DINO Model (consisting of a backbone and encoder-decoder Transformer) outputting raw
hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "input_ids", "val": ": )>"}, {"name": "token_type_ids", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "attention_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "pixel_mask", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "encoder_outputs", "val": " = None"}, {"name": "output_attentions", "val": " = None"}, {"name": "output_hidden_states", "val": " = None"}, {"name": "return_dict", "val": " = None"}, {"name": "**kwargs", "val": ""}]}>
- **pixel_values** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GroundingDinoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoImageProcessor). See `GroundingDinoImageProcessor.__call__()` for details ([GroundingDinoProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoProcessor) uses
  [GroundingDinoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoImageProcessor) for processing images).
- **input_ids** (`torch.LongTensor` of shape `(batch_size, text_sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you provide
  it.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [BertTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, text_sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0,
  1]`: 0 corresponds to a `sentence A` token, 1 corresponds to a `sentence B` token

  [What are token type IDs?](../glossary#token-type-ids)
- **attention_mask** (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **pixel_mask** (`torch.Tensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **encoder_outputs** (``) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **output_attentions** (``) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (``) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (``) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.`MMGroundingDinoModelOutput` or `tuple(torch.FloatTensor)`A `MMGroundingDinoModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([MMGroundingDinoConfig](/docs/transformers/v5.14.0/en/model_doc/mm-grounding-dino#transformers.MMGroundingDinoConfig)) and inputs.
The [MMGroundingDinoModel](/docs/transformers/v5.14.0/en/model_doc/mm-grounding-dino#transformers.MMGroundingDinoModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, num_queries, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the decoder of the model.
- **init_reference_points** (`torch.FloatTensor` of shape  `(batch_size, num_queries, 4)`) -- Initial reference points sent through the Transformer decoder.
- **intermediate_hidden_states** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, hidden_size)`) -- Stacked intermediate hidden states (output of each layer of the decoder).
- **intermediate_reference_points** (`torch.FloatTensor` of shape `(batch_size, config.decoder_layers, num_queries, 4)`) -- Stacked intermediate reference points (reference points of each layer of the decoder).
- **decoder_hidden_states** (`tuple[torch.FloatTensor]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the decoder at the output of each layer plus the initial embedding outputs.
- **decoder_attentions** (`tuple[tuple[torch.FloatTensor]]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights of the decoder, after the attention softmax, used to compute the weighted average in the
  self-attention heads.
- **encoder_last_hidden_state_vision** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_last_hidden_state_text** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) -- Sequence of hidden-states at the output of the last layer of the encoder of the model.
- **encoder_vision_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the vision embeddings + one for the output of each
  layer) of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states of the vision encoder at the
  output of each layer plus the initial embedding outputs.
- **encoder_text_hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the text embeddings + one for the output of each layer)
  of shape `(batch_size, sequence_length, hidden_size)`. Hidden-states of the text encoder at the output of
  each layer plus the initial embedding outputs.
- **encoder_attentions** (`tuple(tuple(torch.FloatTensor))`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of tuples of `torch.FloatTensor` (one for attention for each layer) of shape `(batch_size, num_heads,
  sequence_length, sequence_length)`. Attentions weights after the attention softmax, used to compute the
  weighted average in the text-vision attention, vision-text attention, text-enhancer (self-attention) and
  multi-scale deformable attention heads. attention softmax, used to compute the weighted average in the
  bi-attention heads.
- **enc_outputs_class** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.two_stage=True`) -- Predicted bounding boxes scores where the top `config.num_queries` scoring bounding boxes are picked as
  region proposals in the first stage. Output of bounding box binary classification (i.e. foreground and
  background).
- **enc_outputs_coord_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.two_stage=True`) -- Logits of predicted bounding boxes coordinates in the first stage.
- **encoder_logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`, *optional*, returned when `config.two_stage=True`) -- Logits of top `config.num_queries` scoring bounding boxes in the first stage.
- **encoder_pred_boxes** (`torch.FloatTensor` of shape `(batch_size, sequence_length, 4)`, *optional*, returned when `config.two_stage=True`) -- Coordinates of top `config.num_queries` scoring bounding boxes in the first stage.

Examples:

```python
>>> from transformers import AutoProcessor, AutoModel
>>> from PIL import Image
>>> import httpx
>>> from io import BytesIO

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> text = "a cat."

>>> processor = AutoProcessor.from_pretrained("IDEA-Research/grounding-dino-tiny")
>>> model = AutoModel.from_pretrained("IDEA-Research/grounding-dino-tiny")

>>> inputs = processor(images=image, text=text, return_tensors="pt")
>>> outputs = model(**inputs)

>>> last_hidden_states = outputs.last_hidden_state
>>> list(last_hidden_states.shape)
[1, 900, 256]
```

## MMGroundingDinoForObjectDetection[[transformers.MMGroundingDinoForObjectDetection]]

- **config** ([MMGroundingDinoConfig](/docs/transformers/v5.14.0/en/model_doc/mm-grounding-dino#transformers.MMGroundingDinoConfig)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

Grounding DINO Model (consisting of a backbone and encoder-decoder Transformer) with object detection heads on top,
for tasks such as COCO detection.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

- **pixel_values** (`torch.FloatTensor` of shape `(batch_size, num_channels, image_size, image_size)`) --
  The tensors corresponding to the input images. Pixel values can be obtained using
  [GroundingDinoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoImageProcessor). See `GroundingDinoImageProcessor.__call__()` for details ([GroundingDinoProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoProcessor) uses
  [GroundingDinoImageProcessor](/docs/transformers/v5.14.0/en/model_doc/grounding-dino#transformers.GroundingDinoImageProcessor) for processing images).
- **input_ids** (`torch.LongTensor` of shape `(batch_size, text_sequence_length)`) --
  Indices of input sequence tokens in the vocabulary. Padding will be ignored by default should you provide
  it.

  Indices can be obtained using [AutoTokenizer](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoTokenizer). See [BertTokenizer.__call__()](/docs/transformers/v5.14.0/en/internal/tokenization_utils#transformers.PreTrainedTokenizerBase.__call__) for details.
- **token_type_ids** (`torch.LongTensor` of shape `(batch_size, text_sequence_length)`, *optional*) --
  Segment token indices to indicate first and second portions of the inputs. Indices are selected in `[0,
  1]`: 0 corresponds to a `sentence A` token, 1 corresponds to a `sentence B` token

  [What are token type IDs?](../glossary#token-type-ids)
- **attention_mask** (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*) --
  Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:

  - 1 for tokens that are **not masked**,
  - 0 for tokens that are **masked**.

  [What are attention masks?](../glossary#attention-mask)
- **pixel_mask** (`torch.BoolTensor` of shape `(batch_size, height, width)`, *optional*) --
  Mask to avoid performing attention on padding pixel values. Mask values selected in `[0, 1]`:

  - 1 for pixels that are real (i.e. **not masked**),
  - 0 for pixels that are padding (i.e. **masked**).

  [What are attention masks?](../glossary#attention-mask)
- **encoder_outputs** (`Union[~models.mm_grounding_dino.modeling_mm_grounding_dino.MMGroundingDinoEncoderOutput, tuple]`, *optional*) --
  Tuple consists of (`last_hidden_state`, *optional*: `hidden_states`, *optional*: `attentions`)
  `last_hidden_state` of shape `(batch_size, sequence_length, hidden_size)`, *optional*) is a sequence of
  hidden-states at the output of the last layer of the encoder. Used in the cross-attention of the decoder.
- **output_attentions** (`bool`, *optional*) --
  Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned
  tensors for more detail.
- **output_hidden_states** (`bool`, *optional*) --
  Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for
  more detail.
- **return_dict** (`bool`, *optional*) --
  Whether or not to return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
- **labels** (`list[Dict]` of len `(batch_size,)`, *optional*) --
  Labels for computing the bipartite matching loss. List of dicts, each dictionary containing at least the
  following 2 keys: 'class_labels' and 'boxes' (the class labels and bounding boxes of an image in the batch
  respectively). The class labels themselves should be a `torch.LongTensor` of len `(number of bounding boxes
  in the image,)` and the boxes a `torch.FloatTensor` of shape `(number of bounding boxes in the image, 4)`.
The [MMGroundingDinoForObjectDetection](/docs/transformers/v5.14.0/en/model_doc/mm-grounding-dino#transformers.MMGroundingDinoForObjectDetection) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

Examples:

```python
>>> import httpx
>>> from io import BytesIO

>>> import torch
>>> from PIL import Image
>>> from transformers import AutoProcessor, AutoModelForZeroShotObjectDetection

>>> model_id = "IDEA-Research/grounding-dino-tiny"
>>> device = "cuda"

>>> processor = AutoProcessor.from_pretrained(model_id)
>>> model = AutoModelForZeroShotObjectDetection.from_pretrained(model_id).to(device)

>>> url = "http://images.cocodataset.org/val2017/000000039769.jpg"
>>> with httpx.stream("GET", url) as response:
...     image = Image.open(BytesIO(response.read()))
>>> # Check for cats and remote controls
>>> text_labels = [["a cat", "a remote control"]]

>>> inputs = processor(images=image, text=text_labels, return_tensors="pt").to(device)
>>> with torch.no_grad():
...     outputs = model(**inputs)

>>> results = processor.post_process_grounded_object_detection(
...     outputs,
...     threshold=0.4,
...     text_threshold=0.3,
...     target_sizes=[(image.height, image.width)]
... )
>>> # Retrieve the first image result
>>> result = results[0]
>>> for box, score, text_label in zip(result["boxes"], result["scores"], result["text_labels"]):
...     box = [round(x, 2) for x in box.tolist()]
...     print(f"Detected {text_label} with confidence {round(score.item(), 3)} at location {box}")
Detected a cat with confidence 0.479 at location [344.7, 23.11, 637.18, 374.28]
Detected a cat with confidence 0.438 at location [12.27, 51.91, 316.86, 472.44]
Detected a remote control with confidence 0.478 at location [38.57, 70.0, 176.78, 118.18]
```
