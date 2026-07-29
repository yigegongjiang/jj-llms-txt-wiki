# V-JEPA 2

[V-JEPA 2](https://huggingface.co/papers/2506.09985) ([blog post](https://ai.meta.com/blog/v-jepa-2-world-model-benchmarks/)) is a self-supervised approach to training video encoders developed by FAIR, Meta. Using internet-scale video data, V-JEPA 2 attains state-of-the-art performance on motion understanding and human action anticipation tasks. V-JEPA 2-AC is a latent action-conditioned world model post-trained from V-JEPA 2 (using a small amount of robot trajectory interaction data) that solves robot manipulation tasks without environment-specific data collection or task-specific training or calibration.

    

You can find all original V-JEPA2 checkpoints under the [V-JEPA 2](https://huggingface.co/collections/facebook/v-jepa-2-6841bad8413014e185b497a6) collection.

This model was contributed by [koustuvs](https://huggingface.co/koustuvs), [yonigozlan](https://huggingface.co/yonigozlan) and [qubvel](https://huggingface.co/qubvel-hf). The original code can be found [here](https://github.com/facebookresearch/vjepa2).

## Usage example

The snippet below shows how to load the V-JEPA 2 model for feature extraction using the `AutoModel` class.

```python
import numpy as np
from torchcodec.decoders import VideoDecoder

processor = AutoVideoProcessor.from_pretrained("facebook/vjepa2-vitl-fpc64-256")
model = AutoModel.from_pretrained(
    "facebook/vjepa2-vitl-fpc64-256",
    device_map="auto",
    attn_implementation="sdpa"
)

video_url = "https://huggingface.co/datasets/nateraw/kinetics-mini/resolve/main/val/archery/-Qz25rXdMjE_000014_000024.mp4"

vr = VideoDecoder(video_url)
frame_idx = np.arange(0, 64) # choosing some frames. here, you can define more complex sampling strategy
video = vr.get_frames_at(indices=frame_idx).data  # T x C x H x W
video = processor(video, return_tensors="pt").to(model.device)
outputs = model(**video)

# V-JEPA 2 encoder outputs, same as calling `model.get_vision_features()`
encoder_outputs = outputs.last_hidden_state

# V-JEPA 2 predictor outputs
predictor_outputs = outputs.predictor_output.last_hidden_state
```

V-JEPA 2 can also be finetuned for video classification. In the following snippet, we show how use finetuned on Something-Something-V2 video classification model.

```python
import numpy as np
import torch
from torchcodec.decoders import VideoDecoder

from transformers import AutoModelForVideoClassification, AutoVideoProcessor

# Load model and video preprocessor
hf_repo = "facebook/vjepa2-vitl-fpc16-256-ssv2"

model = AutoModelForVideoClassification.from_pretrained(hf_repo, device_map="auto")
processor = AutoVideoProcessor.from_pretrained(hf_repo)

# To load a video, sample the number of frames according to the model.
video_url = "https://huggingface.co/datasets/nateraw/kinetics-mini/resolve/main/val/bowling/-WH-lxmGJVY_000005_000015.mp4"
vr = VideoDecoder(video_url)
frame_idx = np.arange(0, model.config.frames_per_clip, 8) # you can define more complex sampling strategy
video = vr.get_frames_at(indices=frame_idx).data  # frames x channels x height x width

# Preprocess and run inference
inputs = processor(video, return_tensors="pt").to(model.device)
with torch.no_grad():
    outputs = model(**inputs)
logits = outputs.logits

print("Top 5 predicted class names:")
top5_indices = logits.topk(5).indices[0]
top5_probs = torch.softmax(logits, dim=-1).topk(5).values[0]
for idx, prob in zip(top5_indices, top5_probs):
    text_label = model.config.id2label[idx.item()]
    print(f" - {text_label}: {prob:.2f}")
```

## VJEPA2Config[[transformers.VJEPA2Config]]

- **patch_size** (`Union[int, list[int], tuple[int, int]]`, *optional*, defaults to `16`) --
  The size (resolution) of each patch.
- **crop_size** (`int`, *optional*, defaults to 256) --
  Input resolution of the model
- **frames_per_clip** (`int`, *optional*, defaults to 64) --
  The number of frames the model has been pretrained with. Does not impact inference.
- **tubelet_size** (`int`, *optional*, defaults to 2) --
  The number of temporal frames used for a single rastor, check paper for more information.
- **hidden_size** (`int`, *optional*, defaults to `1024`) --
  Dimension of the hidden representations.
- **in_chans** (`int`, *optional*, defaults to `3`) --
  The number of input channels.
- **num_attention_heads** (`int`, *optional*, defaults to `16`) --
  Number of attention heads for each attention layer in the Transformer decoder.
- **num_hidden_layers** (`int`, *optional*, defaults to `24`) --
  Number of hidden layers in the Transformer decoder.
- **drop_path_rate** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  Drop path rate for the patch fusion.
- **mlp_ratio** (`Union[int, float]`, *optional*, defaults to `4.0`) --
  Ratio of the MLP hidden dim to the embedding dim.
- **layer_norm_eps** (`float`, *optional*, defaults to `1e-06`) --
  The epsilon used by the layer normalization layers.
- **qkv_bias** (`bool`, *optional*, defaults to `True`) --
  Whether to add a bias to the queries, keys and values.
- **attention_probs_dropout_prob** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **hidden_act** (`str`, *optional*, defaults to `gelu`) --
  The non-linear activation function (function or string) in the decoder. For example, `"gelu"`,
  `"relu"`, `"silu"`, etc.
- **initializer_range** (`float`, *optional*, defaults to `0.02`) --
  The standard deviation of the truncated_normal_initializer for initializing all weight matrices.
- **attention_dropout** (`Union[float, int]`, *optional*, defaults to `0.0`) --
  The dropout ratio for the attention probabilities.
- **num_pooler_layers** (`int`, *optional*, defaults to 3) --
  The number of self-attention layers in the pooler.
- **pred_hidden_size** (`int`, *optional*, defaults to 384) --
  Dimensionality of the predictor layers
- **pred_num_attention_heads** (`int`, *optional*, defaults to 12) --
  Number of attention heads for each attention layer in the Predictor
- **pred_num_hidden_layers** (`int`, *optional*, defaults to 12) --
  Number of hidden layers in the Predictor
- **pred_num_mask_tokens** (`int`, *optional*, defaults to 10) --
  Define the number of mask tokens to use in the Predictor
- **pred_zero_init_mask_tokens** (`bool`, *optional*, defaults to `True`) --
  Initialize the mask tokens in the predictor with 0.
- **pred_mlp_ratio** (`float`, *optional*, defaults to 4.0) --
  Ratio of the hidden size of the MLPs used in Predictor relative to the `pred_hidden_size`.

This is the configuration class to store the configuration of a VJEPA2Model. It is used to instantiate a Vjepa2
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [facebook/vjepa2-vitl-fpc64-256](https://huggingface.co/facebook/vjepa2-vitl-fpc64-256)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import VJEPA2Config, VJEPA2Model

>>> # Initializing a VJEPA2 vjepa2-vitl-fpc64-256 style configuration
>>> configuration = VJEPA2Config()

>>> # Initializing a model (with random weights) from the vjepa2-vitl-fpc64-256  style configuration
>>> model = VJEPA2Model(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## VJEPA2Model[[transformers.VJEPA2Model]]

- **config** ([VJEPA2Config](/docs/transformers/v5.14.0/en/model_doc/vjepa2#transformers.VJEPA2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Vjepa2 Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "context_mask", "val": ": list[)>] | None = None"}, {"name": "target_mask", "val": ": list[)>] | None = None"}, {"name": "skip_predictor", "val": ": bool = False"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values_videos** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [VJEPA2VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/vjepa2#transformers.VJEPA2VideoProcessor). See `VJEPA2VideoProcessor.__call__()` for details (`processor_class` uses
  [VJEPA2VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/vjepa2#transformers.VJEPA2VideoProcessor) for processing videos).
- **context_mask** (`torch.Tensor` with shape `[batch_size, patch_size, 1]`, *optional*) --
  The mask position ids indicating which encoder output patches are going to be exposed to the predictor.
  By default, this mask is created as torch.arange(N).unsqueeze(0).repeat(B,1), indicating full context
  available to the predictor.
- **target_mask** (`torch.Tensor` with shape `[batch_size, patch_size, 1]`, *optional*) --
  The mask position ids indicating which encoder output patches are going to be used as a prediction target
  for the predictor. By default, this mask is created as torch.arange(N).unsqueeze(0).repeat(B,1), indicating
  that the predictor should predict all encoder patches.
- **skip_predictor** (`bool`, *optional*, defaults to `False`) --
  flag to skip the predictor forward, useful if you just need the encoder outputs`VJEPA2WithMaskedInputModelOutput` or `tuple(torch.FloatTensor)`A `VJEPA2WithMaskedInputModelOutput` or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VJEPA2Config](/docs/transformers/v5.14.0/en/model_doc/vjepa2#transformers.VJEPA2Config)) and inputs.
The [VJEPA2Model](/docs/transformers/v5.14.0/en/model_doc/vjepa2#transformers.VJEPA2Model) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **masked_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`, *optional*, returned when `context_mask` is provided which is applied on VJEPA2Encoder outputs) -- The masked hidden state of the model.
- **hidden_states** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple[torch.FloatTensor, ...]`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.
- **predictor_output** (`VJEPA2WithMaskedInputPredictorOutput`, *optional*) -- The output from the Predictor module.

## VJEPA2ForVideoClassification[[transformers.VJEPA2ForVideoClassification]]

- **config** ([VJEPA2Config](/docs/transformers/v5.14.0/en/model_doc/vjepa2#transformers.VJEPA2Config)) --
  Model configuration class with all the parameters of the model. Initializing with a config file does not
  load the weights associated with the model, only the configuration. Check out the
  [from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

V-JEPA 2 Model transformer with a video classification head on top (a linear layer on top of the attentive pooler).

This model inherits from [PreTrainedModel](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

)>"}, {"name": "labels", "val": ": typing.Optional[torch.Tensor] = None"}, {"name": "**kwargs", "val": ": Unpack"}]}>
- **pixel_values_videos** (`doc_builder.mock_imports.torch.Tensor` of shape `(batch_size, num_frames, num_channels, frame_size, frame_size)`) --
  The tensors corresponding to the input video. Pixel values for videos can be obtained using
  [VJEPA2VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/vjepa2#transformers.VJEPA2VideoProcessor). See `VJEPA2VideoProcessor.__call__()` for details (`processor_class` uses
  [VJEPA2VideoProcessor](/docs/transformers/v5.14.0/en/model_doc/vjepa2#transformers.VJEPA2VideoProcessor) for processing videos).
- **labels** (`torch.LongTensor` of shape `(batch_size,)`, *optional*) --
  Labels for computing the image classification/regression loss. Indices should be in `[0, ...,
  config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If
  `config.num_labels > 1` a classification loss is computed (Cross-Entropy).[ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or `tuple(torch.FloatTensor)`A [ImageClassifierOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.modeling_outputs.ImageClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([VJEPA2Config](/docs/transformers/v5.14.0/en/model_doc/vjepa2#transformers.VJEPA2Config)) and inputs.
The [VJEPA2ForVideoClassification](/docs/transformers/v5.14.0/en/model_doc/vjepa2#transformers.VJEPA2ForVideoClassification) forward method, overrides the `__call__` special method.

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
>>> import torch
>>> import numpy as np
>>> from transformers import AutoVideoProcessor, VJEPA2ForVideoClassification

>>> device = "cuda"

>>> video_processor = AutoVideoProcessor.from_pretrained("facebook/vjepa2-vitl-fpc16-256-ssv2")
>>> model = VJEPA2ForVideoClassification.from_pretrained("facebook/vjepa2-vitl-fpc16-256-ssv2").to(device)

>>> video = np.ones((64, 256, 256, 3))  # 64 frames, 256x256 RGB
>>> inputs = video_processor(video, return_tensors="pt").to(device)

>>> # For inference
>>> with torch.no_grad():
...     outputs = model(**inputs)
>>> logits = outputs.logits

>>> predicted_label = logits.argmax(-1).item()
>>> print(model.config.id2label[predicted_label])

>>> # For training
>>> labels = torch.ones(1, dtype=torch.long, device=device)
>>> loss = model(**inputs, labels=labels).loss

```

## VJEPA2VideoProcessor[[transformers.VJEPA2VideoProcessor]]
