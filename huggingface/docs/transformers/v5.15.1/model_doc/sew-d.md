# SEW-D

## Overview

SEW-D (Squeezed and Efficient Wav2Vec with Disentangled attention) was proposed in [Performance-Efficiency Trade-offs
in Unsupervised Pre-training for Speech Recognition](https://huggingface.co/papers/2109.06870) by Felix Wu, Kwangyoun Kim,
Jing Pan, Kyu Han, Kilian Q. Weinberger, Yoav Artzi.

The abstract from the paper is the following:

*This paper is a study of performance-efficiency trade-offs in pre-trained models for automatic speech recognition
(ASR). We focus on wav2vec 2.0, and formalize several architecture designs that influence both the model performance
and its efficiency. Putting together all our observations, we introduce SEW (Squeezed and Efficient Wav2vec), a
pre-trained model architecture with significant improvements along both performance and efficiency dimensions across a
variety of training setups. For example, under the 100h-960h semi-supervised setup on LibriSpeech, SEW achieves a 1.9x
inference speedup compared to wav2vec 2.0, with a 13.5% relative reduction in word error rate. With a similar inference
time, SEW reduces word error rate by 25-50% across different model sizes.*

This model was contributed by [anton-l](https://huggingface.co/anton-l).

## Usage tips

- SEW-D is a speech model that accepts a float array corresponding to the raw waveform of the speech signal.
- SEWDForCTC is fine-tuned using connectionist temporal classification (CTC) so the model output has to be decoded
  using [Wav2Vec2CTCTokenizer](/docs/transformers/v5.15.1/en/model_doc/wav2vec2#transformers.Wav2Vec2CTCTokenizer).

## Resources

- [Audio classification task guide](../tasks/audio_classification)
- [Automatic speech recognition task guide](../tasks/asr)

## SEWDConfig[[transformers.SEWDConfig]]

#### transformers.SEWDConfig[[transformers.SEWDConfig]]

```python
transformers.SEWDConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 32, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, intermediate_size: int = 3072, squeeze_factor: int = 2, max_position_embeddings: int = 512, position_buckets: int = 256, share_att_key: bool = True, relative_attention: bool = True, pos_att_type: list[str] | tuple[str, ...] = ('p2c', 'c2p'), norm_rel_ebd: str = 'layer_norm', hidden_act: str = 'gelu_python', hidden_dropout: float | int = 0.1, activation_dropout: float | int = 0.1, attention_dropout: float | int = 0.1, feat_proj_dropout: float | int = 0.0, final_dropout: float | int = 0.1, initializer_range: float = 0.02, layer_norm_eps: float = 1e-07, feature_layer_norm_eps: float = 1e-05, feat_extract_norm: str = 'group', feat_extract_activation: str = 'gelu', conv_dim: list[int] | tuple[int, ...] = (64, 128, 128, 128, 128, 256, 256, 256, 256, 512, 512, 512, 512), conv_stride: list[int] | tuple[int, ...] = (5, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1), conv_kernel: list[int] | tuple[int, ...] = (10, 3, 1, 3, 1, 3, 1, 3, 1, 2, 1, 2, 1), conv_bias: bool = False, num_conv_pos_embeddings: int = 128, num_conv_pos_embedding_groups: int = 16, apply_spec_augment: bool = True, mask_time_prob: float | int = 0.05, mask_time_length: int = 10, mask_time_min_masks: int = 2, mask_feature_prob: float | int = 0.0, mask_feature_length: int = 10, mask_feature_min_masks: int = 0, ctc_loss_reduction: str = 'mean', ctc_zero_infinity: bool = False, use_weighted_layer_sum: bool = False, classifier_proj_size: int = 256, pad_token_id: int | None = 0, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 2)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sew_d/configuration_sew_d.py#L27)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `32`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

squeeze_factor (`int`, *optional*, defaults to 2) : Sequence length downsampling factor after the encoder and upsampling factor after the transformer.

max_position_embeddings (`int`, *optional*, defaults to `512`) : The maximum sequence length that this model might ever be used with.

position_buckets (`int`, *optional*, defaults to 256) : The maximum size of relative position embeddings.

share_att_key (`bool`, *optional*, defaults to `True`) : Whether to share attention key with c2p and p2c.

relative_attention (`bool`, *optional*, defaults to `True`) : Whether to use relative position encoding.

pos_att_type (`tuple[str]`, *optional*, defaults to `("p2c", "c2p")`) : The type of relative position attention, it can be a combination of `("p2c", "c2p")`, e.g. `("p2c")`, `("p2c", "c2p")`, `("p2c", "c2p")`.

norm_rel_ebd (`str`, *optional*, defaults to `"layer_norm"`) : Whether to use layer norm in relative embedding (`"layer_norm"` if yes)

hidden_act (`str`, *optional*, defaults to `gelu_python`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for activations inside the fully connected layer.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

feat_proj_dropout (`float`, *optional*, defaults to 0.0) : The dropout probability for output of the feature encoder.

final_dropout (`float`, *optional*, defaults to 0.1) : The dropout probability for the final projection layer of [SEWDForCTC](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDForCTC).

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-07`) : The epsilon used by the layer normalization layers.

feature_layer_norm_eps (`float`, *optional*, defaults to 1e-5) : The epsilon used by the layer normalization after the feature encoder.

feat_extract_norm (`str`, *optional*, defaults to `"group"`) : The norm to be applied to 1D convolutional layers in feature encoder. One of `"group"` for group normalization of only the first 1D convolutional layer or `"layer"` for layer normalization of all 1D convolutional layers.

feat_extract_activation (`str, `optional`, defaults to `"gelu"`) : The non-linear activation function (function or string) in the 1D convolutional layers of the feature extractor. If string, `"gelu"`, `"relu"`, `"selu"` and `"gelu_new"` are supported.

conv_dim (`tuple[int]` or `list[int]`, *optional*, defaults to `(64, 128, 128, 128, 128, 256, 256, 256, 256, 512, 512, 512, 512)`) : A tuple of integers defining the number of input and output channels of each 1D convolutional layer in the feature encoder. The length of *conv_dim* defines the number of 1D convolutional layers.

conv_stride (`tuple[int]` or `list[int]`, *optional*, defaults to `(5, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1)`) : A tuple of integers defining the stride of each 1D convolutional layer in the feature encoder. The length of *conv_stride* defines the number of convolutional layers and has to match the length of *conv_dim*.

conv_kernel (`tuple[int]` or `list[int]`, *optional*, defaults to `(10, 3, 1, 3, 1, 3, 1, 3, 1, 2, 1, 2, 1)`) : A tuple of integers defining the kernel size of each 1D convolutional layer in the feature encoder. The length of *conv_kernel* defines the number of convolutional layers and has to match the length of *conv_dim*.

conv_bias (`bool`, *optional*, defaults to `False`) : Whether the 1D convolutional layers have a bias.

num_conv_pos_embeddings (`int`, *optional*, defaults to 128) : Number of convolutional positional embeddings. Defines the kernel size of 1D convolutional positional embeddings layer.

num_conv_pos_embedding_groups (`int`, *optional*, defaults to 16) : Number of groups of 1D convolutional positional embeddings layer.

apply_spec_augment (`bool`, *optional*, defaults to `True`) : Whether to apply *SpecAugment* data augmentation to the outputs of the feature encoder. For reference see [SpecAugment: A Simple Data Augmentation Method for Automatic Speech Recognition](https://huggingface.co/papers/1904.08779).

mask_time_prob (`float`, *optional*, defaults to 0.05) : Percentage (between 0 and 1) of all feature vectors along the time axis which will be masked. The masking procedure generates ''mask_time_prob*len(time_axis)/mask_time_length'' independent masks over the axis. If reasoning from the probability of each feature vector to be chosen as the start of the vector span to be masked, *mask_time_prob* should be `prob_vector_start*mask_time_length`. Note that overlap may decrease the actual percentage of masked vectors. This is only relevant if `apply_spec_augment is True`.

mask_time_length (`int`, *optional*, defaults to 10) : Length of vector span along the time axis.

mask_time_min_masks (`int`, *optional*, defaults to 2), : The minimum number of masks of length `mask_feature_length` generated along the time axis, each time step, irrespectively of `mask_feature_prob`. Only relevant if ''mask_time_prob*len(time_axis)/mask_time_length < mask_time_min_masks''

mask_feature_prob (`float`, *optional*, defaults to 0.0) : Percentage (between 0 and 1) of all feature vectors along the feature axis which will be masked. The masking procedure generates ''mask_feature_prob*len(feature_axis)/mask_time_length'' independent masks over the axis. If reasoning from the probability of each feature vector to be chosen as the start of the vector span to be masked, *mask_feature_prob* should be `prob_vector_start*mask_feature_length`. Note that overlap may decrease the actual percentage of masked vectors. This is only relevant if `apply_spec_augment is True`.

mask_feature_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : Percentage (between 0 and 1) of all feature vectors along the feature axis which will be masked. The masking procedure generates `mask_feature_prob*len(feature_axis)/mask_time_length` independent masks over the axis. If reasoning from the probability of each feature vector to be chosen as the start of the vector span to be masked, *mask_feature_prob* should be `prob_vector_start*mask_feature_length`. Note that overlap may decrease the actual percentage of masked vectors. This is only relevant if `apply_spec_augment` is `True`.

mask_feature_length (`int`, *optional*, defaults to 10) : Length of vector span along the feature axis.

mask_feature_min_masks (`int`, *optional*, defaults to 0), : The minimum number of masks of length `mask_feature_length` generated along the feature axis, each time step, irrespectively of `mask_feature_prob`. Only relevant if ''mask_feature_prob*len(feature_axis)/mask_feature_length < mask_feature_min_masks''

diversity_loss_weight (`int`, *optional*, defaults to 0.1) : The weight of the codebook diversity loss component.

ctc_loss_reduction (`str`, *optional*, defaults to `mean`) : Specifies the reduction to apply to the output of `torch.nn.CTCLoss`. Only relevant when training.

ctc_zero_infinity (`bool`, *optional*, defaults to `False`) : Whether to zero infinite losses and the associated gradients of `torch.nn.CTCLoss`. Infinite losses mainly occur when the inputs are too short to be aligned to the targets. Only relevant when training an instance of [SEWDForCTC](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDForCTC).

use_weighted_layer_sum (`bool`, *optional*, defaults to `False`) : Whether to use a weighted average of layer outputs with learned weights. Only relevant when using an instance of [Wav2Vec2ForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/wav2vec2#transformers.Wav2Vec2ForSequenceClassification).

classifier_proj_size (`int`, *optional*, defaults to 256) : Dimensionality of the projection before token mean-pooling for classification.

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

This is the configuration class to store the configuration of a Sew DModel. It is used to instantiate a Sew D
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [BAAI/seggpt-vit-large](https://huggingface.co/BAAI/seggpt-vit-large)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python
>>> from transformers import SEWDConfig, SEWDModel

>>> # Initializing a SEW-D asapp/sew-d-tiny-100k style configuration
>>> configuration = SEWDConfig()

>>> # Initializing a model (with random weights) from the asapp/sew-d-tiny-100k style configuration
>>> model = SEWDModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## SEWDModel[[transformers.SEWDModel]]

#### transformers.SEWDModel[[transformers.SEWDModel]]

```python
transformers.SEWDModel(config: SEWDConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sew_d/modeling_sew_d.py#L1229)

**Parameters:**

config ([SEWDConfig](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Sew D Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SEWDModel.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, mask_time_indices: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sew_d/modeling_sew_d.py#L1296)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [Wav2Vec2Processor.__call__()](/docs/transformers/v5.15.1/en/model_doc/wav2vec2#transformers.Wav2Vec2Processor.__call__) for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

mask_time_indices (`torch.BoolTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices to mask extracted features for contrastive loss. When in training mode, model learns to predict masked extracted features in *config.proj_codevector_dim* space.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [BaseModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or `tuple(torch.FloatTensor)`

A [BaseModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SEWDConfig](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDConfig)) and inputs.

The [SEWDModel](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDModel) forward method, overrides the `__call__` special method.

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

## SEWDForCTC[[transformers.SEWDForCTC]]

#### transformers.SEWDForCTC[[transformers.SEWDForCTC]]

```python
transformers.SEWDForCTC(config, target_lang: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sew_d/modeling_sew_d.py#L1358)

**Parameters:**

config ([SEWDForCTC](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDForCTC)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

target_lang (`str`, *optional*) : Language id of adapter weights. Adapter weights are stored in the format adapter..safetensors or adapter..bin. Only relevant when using an instance of [SEWDForCTC](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDForCTC) with adapters. Uses 'eng' by default.

SEW-D Model with a `language modeling` head on top for Connectionist Temporal Classification (CTC).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SEWDForCTC.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, labels: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sew_d/modeling_sew_d.py#L1427)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [Wav2Vec2Processor.__call__()](/docs/transformers/v5.15.1/en/model_doc/wav2vec2#transformers.Wav2Vec2Processor.__call__) for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

labels (`torch.LongTensor` of shape `(batch_size, target_length)`, *optional*) : Labels for connectionist temporal classification. Note that `target_length` has to be smaller or equal to the sequence length of the output logits. Indices are selected in `[-100, 0, ..., config.vocab_size - 1]`. All labels set to `-100` are ignored (masked), the loss is only computed for labels in `[0, ..., config.vocab_size - 1]`.

**Returns:** [CausalLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutput) or `tuple(torch.FloatTensor)`

A [CausalLMOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.CausalLMOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SEWDConfig](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDConfig)) and inputs.

The [SEWDForCTC](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDForCTC) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Language modeling loss (for next-token prediction).
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
>>> from transformers import AutoProcessor, SEWDForCTC
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> processor = AutoProcessor.from_pretrained("BAAI/seggpt-vit-large")
>>> model = SEWDForCTC.from_pretrained("BAAI/seggpt-vit-large")

>>> # audio file is decoded on the fly
>>> inputs = processor(dataset[0]["audio"]["array"], sampling_rate=sampling_rate, return_tensors="pt")
>>> with torch.no_grad():
...     logits = model(**inputs).logits
>>> predicted_ids = torch.argmax(logits, dim=-1)

>>> # transcribe speech
>>> transcription = processor.batch_decode(predicted_ids)
>>> transcription[0]
...

>>> inputs["labels"] = processor(text=dataset[0]["text"], return_tensors="pt").input_ids

>>> # compute loss
>>> loss = model(**inputs).loss
>>> round(loss.item(), 2)
...
```

## SEWDForSequenceClassification[[transformers.SEWDForSequenceClassification]]

#### transformers.SEWDForSequenceClassification[[transformers.SEWDForSequenceClassification]]

```python
transformers.SEWDForSequenceClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sew_d/modeling_sew_d.py#L1507)

**Parameters:**

config ([SEWDForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDForSequenceClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

SEWD Model with a sequence classification head on top (a linear layer over the pooled output) for tasks like SUPERB
Keyword Spotting.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.SEWDForSequenceClassification.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, labels: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/sew_d/modeling_sew_d.py#L1540)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See `SEWDProcessor.__call__` for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

**Returns:** [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`

A [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([SEWDConfig](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDConfig)) and inputs.

The [SEWDForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/sew-d#transformers.SEWDForSequenceClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification (or regression if config.num_labels==1) loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.num_labels)`) -- Classification (or regression if config.num_labels==1) scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example of single-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, SEWDForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("BAAI/seggpt-vit-large")
>>> model = SEWDForSequenceClassification.from_pretrained("BAAI/seggpt-vit-large")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> model.config.id2label[predicted_class_id]
...

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = SEWDForSequenceClassification.from_pretrained("BAAI/seggpt-vit-large", num_labels=num_labels)

>>> labels = torch.tensor([1])
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

Example of multi-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, SEWDForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("BAAI/seggpt-vit-large")
>>> model = SEWDForSequenceClassification.from_pretrained("BAAI/seggpt-vit-large", problem_type="multi_label_classification")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_ids = torch.arange(0, logits.shape[-1])[torch.sigmoid(logits).squeeze(dim=0) > 0.5]

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = SEWDForSequenceClassification.from_pretrained(
...     "BAAI/seggpt-vit-large", num_labels=num_labels, problem_type="multi_label_classification"
... )

>>> labels = torch.sum(
...     torch.nn.functional.one_hot(predicted_class_ids[None, :].clone(), num_classes=num_labels), dim=1
... ).to(torch.float)
>>> loss = model(**inputs, labels=labels).loss
```
