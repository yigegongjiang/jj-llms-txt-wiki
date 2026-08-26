# WavLM

## Overview

The WavLM model was proposed in [WavLM: Large-Scale Self-Supervised Pre-Training for Full Stack Speech Processing](https://huggingface.co/papers/2110.13900) by Sanyuan Chen, Chengyi Wang, Zhengyang Chen, Yu Wu, Shujie Liu, Zhuo Chen,
Jinyu Li, Naoyuki Kanda, Takuya Yoshioka, Xiong Xiao, Jian Wu, Long Zhou, Shuo Ren, Yanmin Qian, Yao Qian, Jian Wu,
Michael Zeng, Furu Wei.

The abstract from the paper is the following:

*Self-supervised learning (SSL) achieves great success in speech recognition, while limited exploration has been
attempted for other speech processing tasks. As speech signal contains multi-faceted information including speaker
identity, paralinguistics, spoken content, etc., learning universal representations for all speech tasks is
challenging. In this paper, we propose a new pre-trained model, WavLM, to solve full-stack downstream speech tasks.
WavLM is built based on the HuBERT framework, with an emphasis on both spoken content modeling and speaker identity
preservation. We first equip the Transformer structure with gated relative position bias to improve its capability on
recognition tasks. For better speaker discrimination, we propose an utterance mixing training strategy, where
additional overlapped utterances are created unsupervisedly and incorporated during model training. Lastly, we scale up
the training dataset from 60k hours to 94k hours. WavLM Large achieves state-of-the-art performance on the SUPERB
benchmark, and brings significant improvements for various speech processing tasks on their representative benchmarks.*

Relevant checkpoints can be found under https://huggingface.co/models?other=wavlm.

This model was contributed by [patrickvonplaten](https://huggingface.co/patrickvonplaten). The Authors' code can be
found [here](https://github.com/microsoft/unilm/tree/master/wavlm).

## Usage tips

- WavLM is a speech model that accepts a float array corresponding to the raw waveform of the speech signal. Please use
  [Wav2Vec2Processor](/docs/transformers/v5.15.1/en/model_doc/wav2vec2#transformers.Wav2Vec2Processor) for the feature extraction.
- WavLM model can be fine-tuned using connectionist temporal classification (CTC) so the model output has to be decoded
  using [Wav2Vec2CTCTokenizer](/docs/transformers/v5.15.1/en/model_doc/wav2vec2#transformers.Wav2Vec2CTCTokenizer).
- WavLM performs especially well on speaker verification, speaker identification, and speaker diarization tasks.

## Resources

- [Audio classification task guide](../tasks/audio_classification)
- [Automatic speech recognition task guide](../tasks/asr)

## WavLMConfig[[transformers.WavLMConfig]]

#### transformers.WavLMConfig[[transformers.WavLMConfig]]

```python
transformers.WavLMConfig(transformers_version: str | None = None, architectures: list[str] | None = None, output_hidden_states: bool | None = False, return_dict: bool | None = True, dtype: typing.Union[str, ForwardRef('torch.dtype'), NoneType] = None, chunk_size_feed_forward: int = 0, is_encoder_decoder: bool = False, id2label: dict[int, str] | dict[str, str] | None = None, label2id: dict[str, int] | dict[str, str] | None = None, problem_type: typing.Optional[typing.Literal['regression', 'single_label_classification', 'multi_label_classification']] = None, vocab_size: int = 32, hidden_size: int = 768, num_hidden_layers: int = 12, num_attention_heads: int = 12, intermediate_size: int = 3072, hidden_act: str = 'gelu', hidden_dropout: float | int = 0.1, activation_dropout: float | int = 0.1, attention_dropout: float | int = 0.1, feat_proj_dropout: float | int = 0.0, final_dropout: float | int = 0.1, layerdrop: float | int = 0.1, initializer_range: float = 0.02, layer_norm_eps: float = 1e-05, feat_extract_norm: str = 'group', feat_extract_activation: str = 'gelu', conv_dim: list[int] | tuple[int, ...] = (512, 512, 512, 512, 512, 512, 512), conv_stride: list[int] | tuple[int, ...] = (5, 2, 2, 2, 2, 2, 2), conv_kernel: list[int] | tuple[int, ...] = (10, 3, 3, 3, 3, 2, 2), conv_bias: bool = False, num_conv_pos_embeddings: int = 128, num_conv_pos_embedding_groups: int = 16, num_buckets: int = 320, max_bucket_distance: int = 800, do_stable_layer_norm: bool = False, apply_spec_augment: bool = True, mask_time_prob: float | int = 0.05, mask_time_length: int = 10, mask_time_min_masks: int = 2, mask_feature_prob: float | int = 0.0, mask_feature_length: int = 10, num_codevectors_per_group: int = 320, num_codevector_groups: int = 2, contrastive_logits_temperature: float = 0.1, num_negatives: int = 100, codevector_dim: int = 256, proj_codevector_dim: int = 256, diversity_loss_weight: float = 0.1, ctc_loss_reduction: str = 'mean', ctc_zero_infinity: bool = False, use_weighted_layer_sum: bool = False, classifier_proj_size: int = 256, tdnn_dim: list[int] | tuple[int, ...] = (512, 512, 512, 512, 1500), tdnn_kernel: list[int] | tuple[int, ...] = (5, 3, 3, 1, 1), tdnn_dilation: list[int] | tuple[int, ...] = (1, 2, 3, 1, 1), xvector_output_dim: int = 512, num_ctc_classes: int = 80, pad_token_id: int | None = 0, bos_token_id: int | None = 1, eos_token_id: int | list[int] | None = 2, add_adapter: bool = False, adapter_kernel_size: int = 3, adapter_stride: int = 2, num_adapter_layers: int = 3, output_hidden_size: int | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/configuration_wavlm.py#L27)

**Parameters:**

vocab_size (`int`, *optional*, defaults to `32`) : Vocabulary size of the model. Defines the number of different tokens that can be represented by the `input_ids`.

hidden_size (`int`, *optional*, defaults to `768`) : Dimension of the hidden representations.

num_hidden_layers (`int`, *optional*, defaults to `12`) : Number of hidden layers in the Transformer decoder.

num_attention_heads (`int`, *optional*, defaults to `12`) : Number of attention heads for each attention layer in the Transformer decoder.

intermediate_size (`int`, *optional*, defaults to `3072`) : Dimension of the MLP representations.

hidden_act (`str`, *optional*, defaults to `gelu`) : The non-linear activation function (function or string) in the decoder. For example, `"gelu"`, `"relu"`, `"silu"`, etc.

hidden_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout probability for all fully connected layers in the embeddings, encoder, and pooler.

activation_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for activations inside the fully connected layer.

attention_dropout (`Union[float, int]`, *optional*, defaults to `0.1`) : The dropout ratio for the attention probabilities.

feat_proj_dropout (`float`, *optional*, defaults to 0.0) : The dropout probability for output of the feature encoder.

final_dropout (`float`, *optional*, defaults to 0.1) : The dropout probability for the final projection layer of [WavLMForCTC](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForCTC).

layerdrop (`Union[float, int]`, *optional*, defaults to `0.1`) : The LayerDrop probability. See the [LayerDrop paper](see https://huggingface.co/papers/1909.11556) for more details.

initializer_range (`float`, *optional*, defaults to `0.02`) : The standard deviation of the truncated_normal_initializer for initializing all weight matrices.

layer_norm_eps (`float`, *optional*, defaults to `1e-05`) : The epsilon used by the layer normalization layers.

feat_extract_norm (`str`, *optional*, defaults to `"group"`) : The norm to be applied to 1D convolutional layers in feature encoder. One of `"group"` for group normalization of only the first 1D convolutional layer or `"layer"` for layer normalization of all 1D convolutional layers.

feat_extract_activation (`str, `optional`, defaults to `"gelu"`) : The non-linear activation function (function or string) in the 1D convolutional layers of the feature extractor. If string, `"gelu"`, `"relu"`, `"selu"` and `"gelu_new"` are supported.

conv_dim (`tuple[int]` or `list[int]`, *optional*, defaults to `(512, 512, 512, 512, 512, 512, 512)`) : A tuple of integers defining the number of input and output channels of each 1D convolutional layer in the feature encoder. The length of *conv_dim* defines the number of 1D convolutional layers.

conv_stride (`tuple[int]` or `list[int]`, *optional*, defaults to `(5, 2, 2, 2, 2, 2, 2)`) : A tuple of integers defining the stride of each 1D convolutional layer in the feature encoder. The length of *conv_stride* defines the number of convolutional layers and has to match the length of *conv_dim*.

conv_kernel (`tuple[int]` or `list[int]`, *optional*, defaults to `(10, 3, 3, 3, 3, 3, 3)`) : A tuple of integers defining the kernel size of each 1D convolutional layer in the feature encoder. The length of *conv_kernel* defines the number of convolutional layers and has to match the length of *conv_dim*.

conv_bias (`bool`, *optional*, defaults to `False`) : Whether the 1D convolutional layers have a bias.

num_conv_pos_embeddings (`int`, *optional*, defaults to 128) : Number of convolutional positional embeddings. Defines the kernel size of 1D convolutional positional embeddings layer.

num_conv_pos_embedding_groups (`int`, *optional*, defaults to 16) : Number of groups of 1D convolutional positional embeddings layer.

num_buckets (`int`, *optional*, defaults to 320) : The number of buckets to use for each attention layer

max_bucket_distance (`int`, *optional*, defaults to 800) : Maximum bucket distance

do_stable_layer_norm (`bool`, *optional*, defaults to `False`) : Whether to apply *stable* layer norm architecture of the Transformer encoder. `do_stable_layer_norm is True` corresponds to applying layer norm before the attention layer, whereas `do_stable_layer_norm is False` corresponds to applying layer norm after the attention layer.

apply_spec_augment (`bool`, *optional*, defaults to `True`) : Whether to apply *SpecAugment* data augmentation to the outputs of the feature encoder. For reference see [SpecAugment: A Simple Data Augmentation Method for Automatic Speech Recognition](https://huggingface.co/papers/1904.08779).

mask_time_prob (`float`, *optional*, defaults to 0.05) : Probability of each feature vector along the time axis to be chosen as the start of the vector span to be masked. Approximately `mask_time_prob * sequence_length // mask_time_length` feature vectors will be masked along the time axis. This is only relevant if `apply_spec_augment is True`.

mask_time_length (`int`, *optional*, defaults to 10) : Length of vector span along the time axis.

mask_time_min_masks (`int`, *optional*, defaults to 2), : The minimum number of masks of length `mask_feature_length` generated along the time axis, each time step, irrespectively of `mask_feature_prob`. Only relevant if ''mask_time_prob*len(time_axis)/mask_time_length < mask_time_min_masks''

mask_feature_prob (`float`, *optional*, defaults to 0.0) : Probability of each feature vector along the feature axis to be chosen as the start of the vector span to be masked. Approximately `mask_time_prob * hidden_size // mask_time_length` feature vectors will be masked along the time axis. This is only relevant if `apply_spec_augment is True`.

mask_feature_prob (`Union[float, int]`, *optional*, defaults to `0.0`) : Percentage (between 0 and 1) of all feature vectors along the feature axis which will be masked. The masking procedure generates `mask_feature_prob*len(feature_axis)/mask_time_length` independent masks over the axis. If reasoning from the probability of each feature vector to be chosen as the start of the vector span to be masked, *mask_feature_prob* should be `prob_vector_start*mask_feature_length`. Note that overlap may decrease the actual percentage of masked vectors. This is only relevant if `apply_spec_augment` is `True`.

mask_feature_length (`int`, *optional*, defaults to 10) : Length of vector span along the feature axis.

num_codevectors_per_group (`int`, *optional*, defaults to 320) : Number of entries in each quantization codebook (group).

num_codevector_groups (`int`, *optional*, defaults to 2) : Number of codevector groups for product codevector quantization.

contrastive_logits_temperature (`float`, *optional*, defaults to 0.1) : The temperature *kappa* in the contrastive loss.

num_negatives (`int`, *optional*, defaults to 100) : Number of negative samples for the contrastive loss.

codevector_dim (`int`, *optional*, defaults to 256) : Dimensionality of the quantized feature vectors.

proj_codevector_dim (`int`, *optional*, defaults to 256) : Dimensionality of the final projection of both the quantized and the transformer features.

diversity_loss_weight (`int`, *optional*, defaults to 0.1) : The weight of the codebook diversity loss component.

ctc_loss_reduction (`str`, *optional*, defaults to `mean`) : Specifies the reduction to apply to the output of `torch.nn.CTCLoss`. Only relevant when training.

ctc_zero_infinity (`bool`, *optional*, defaults to `False`) : Whether to zero infinite losses and the associated gradients of `torch.nn.CTCLoss`. Infinite losses mainly occur when the inputs are too short to be aligned to the targets. Only relevant when training an instance of [WavLMForCTC](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForCTC).

use_weighted_layer_sum (`bool`, *optional*, defaults to `False`) : Whether to use a weighted average of layer outputs with learned weights. Only relevant when using an instance of [WavLMForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForSequenceClassification).

classifier_proj_size (`int`, *optional*, defaults to 256) : Dimensionality of the projection before token mean-pooling for classification.

tdnn_dim (`tuple[int]` or `list[int]`, *optional*, defaults to `(512, 512, 512, 512, 1500)`) : A tuple of integers defining the number of output channels of each 1D convolutional layer in the *TDNN* module of the *XVector* model. The length of *tdnn_dim* defines the number of *TDNN* layers.

tdnn_kernel (`tuple[int]` or `list[int]`, *optional*, defaults to `(5, 3, 3, 1, 1)`) : A tuple of integers defining the kernel size of each 1D convolutional layer in the *TDNN* module of the *XVector* model. The length of *tdnn_kernel* has to match the length of *tdnn_dim*.

tdnn_dilation (`tuple[int]` or `list[int]`, *optional*, defaults to `(1, 2, 3, 1, 1)`) : A tuple of integers defining the dilation factor of each 1D convolutional layer in *TDNN* module of the *XVector* model. The length of *tdnn_dilation* has to match the length of *tdnn_dim*.

xvector_output_dim (`int`, *optional*, defaults to 512) : Dimensionality of the *XVector* embedding vectors.

num_ctc_classes (`int`, *optional*, defaults to 80) : Specifies the number of classes (phoneme tokens and blank token) for phoneme-level CTC loss. Only relevant when using an instance of [UniSpeechForPreTraining](/docs/transformers/v5.15.1/en/model_doc/unispeech#transformers.UniSpeechForPreTraining).

pad_token_id (`int`, *optional*, defaults to `0`) : Token id used for padding in the vocabulary.

bos_token_id (`int`, *optional*, defaults to `1`) : Token id used for beginning-of-stream in the vocabulary.

eos_token_id (`Union[int, list[int]]`, *optional*, defaults to `2`) : Token id used for end-of-stream in the vocabulary.

add_adapter (`bool`, *optional*, defaults to `False`) : Whether a convolutional network should be stacked on top of the Wav2Vec2 Encoder. Can be very useful for warm-starting Wav2Vec2 for SpeechEncoderDecoder models.

adapter_kernel_size (`int`, *optional*, defaults to 3) : Kernel size of the convolutional layers in the adapter network. Only relevant if `add_adapter is True`.

adapter_stride (`int`, *optional*, defaults to 2) : Stride of the convolutional layers in the adapter network. Only relevant if `add_adapter is True`.

num_adapter_layers (`int`, *optional*, defaults to 3) : Number of convolutional layers that should be used in the adapter network. Only relevant if `add_adapter is True`.

output_hidden_size (`int`, *optional*) : Dimensionality of the encoder output layer. If not defined, this defaults to *hidden-size*. Only relevant if `add_adapter is True`.

This is the configuration class to store the configuration of a WavLMModel. It is used to instantiate a Wavlm
model according to the specified arguments, defining the model architecture. Instantiating a configuration with the
defaults will yield a similar configuration to that of the [microsoft/wavlm-base](https://huggingface.co/microsoft/wavlm-base)

Configuration objects inherit from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) and can be used to control the model outputs. Read the
documentation from [PreTrainedConfig](/docs/transformers/v5.15.1/en/main_classes/configuration#transformers.PreTrainedConfig) for more information.

Example:

```python

```

Example:

```python
>>> from transformers import WavLMConfig, WavLMModel

>>> # Initializing a WavLM facebook/wavlm-base-960h style configuration
>>> configuration = WavLMConfig()

>>> # Initializing a model (with random weights) from the facebook/wavlm-base-960h style configuration
>>> model = WavLMModel(configuration)

>>> # Accessing the model configuration
>>> configuration = model.config
```

## WavLMModel[[transformers.WavLMModel]]

#### transformers.WavLMModel[[transformers.WavLMModel]]

```python
transformers.WavLMModel(config: WavLMConfig)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/modeling_wavlm.py#L957)

**Parameters:**

config ([WavLMConfig](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMConfig)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The bare Wavlm Model outputting raw hidden-states without any specific head on top.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.WavLMModel.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, mask_time_indices: typing.Optional[torch.FloatTensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/modeling_wavlm.py#L1031)

**Parameters:**

input_values (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See [Wav2Vec2Processor.__call__()](/docs/transformers/v5.15.1/en/model_doc/wav2vec2#transformers.Wav2Vec2Processor.__call__) for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

mask_time_indices (`torch.BoolTensor` of shape `(batch_size, sequence_length)`, *optional*) : Indices to mask extracted features for contrastive loss. When in training mode, model learns to predict masked extracted features in *config.proj_codevector_dim* space.

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [Wav2Vec2BaseModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Wav2Vec2BaseModelOutput) or `tuple(torch.FloatTensor)`

A [Wav2Vec2BaseModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.Wav2Vec2BaseModelOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([WavLMConfig](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMConfig)) and inputs.

The [WavLMModel](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMModel) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **last_hidden_state** (`torch.FloatTensor` of shape `(batch_size, sequence_length, hidden_size)`) -- Sequence of hidden-states at the output of the last layer of the model.
- **extract_features** (`torch.FloatTensor` of shape `(batch_size, sequence_length, conv_dim[-1])`) -- Sequence of extracted feature vectors of the last convolutional layer of the model.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

## WavLMForCTC[[transformers.WavLMForCTC]]

#### transformers.WavLMForCTC[[transformers.WavLMForCTC]]

```python
transformers.WavLMForCTC(config, target_lang: str | None = None)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/modeling_wavlm.py#L1099)

**Parameters:**

config ([WavLMForCTC](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForCTC)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

target_lang (`str`, *optional*) : Language id of adapter weights. Adapter weights are stored in the format adapter..safetensors or adapter..bin. Only relevant when using an instance of [WavLMForCTC](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForCTC) with adapters. Uses 'eng' by default.

WavLM Model with a `language modeling` head on top for Connectionist Temporal Classification (CTC).

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.WavLMForCTC.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, labels: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/modeling_wavlm.py#L1168)

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
elements depending on the configuration ([WavLMConfig](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMConfig)) and inputs.

The [WavLMForCTC](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForCTC) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoProcessor, WavLMForCTC
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> processor = AutoProcessor.from_pretrained("microsoft/wavlm-base")
>>> model = WavLMForCTC.from_pretrained("microsoft/wavlm-base")

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

## WavLMForSequenceClassification[[transformers.WavLMForSequenceClassification]]

#### transformers.WavLMForSequenceClassification[[transformers.WavLMForSequenceClassification]]

```python
transformers.WavLMForSequenceClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/modeling_wavlm.py#L1247)

**Parameters:**

config ([WavLMForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForSequenceClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

WavLM Model with a sequence classification head on top (a linear layer over the pooled output) for tasks like
SUPERB Keyword Spotting.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.WavLMForSequenceClassification.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, labels: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/modeling_wavlm.py#L1280)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See `WavLMProcessor.__call__` for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

**Returns:** [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or `tuple(torch.FloatTensor)`

A [SequenceClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.SequenceClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([WavLMConfig](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMConfig)) and inputs.

The [WavLMForSequenceClassification](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForSequenceClassification) forward method, overrides the `__call__` special method.

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
>>> from transformers import AutoTokenizer, WavLMForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("microsoft/wavlm-base")
>>> model = WavLMForSequenceClassification.from_pretrained("microsoft/wavlm-base")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_id = logits.argmax().item()
>>> model.config.id2label[predicted_class_id]
...

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = WavLMForSequenceClassification.from_pretrained("microsoft/wavlm-base", num_labels=num_labels)

>>> labels = torch.tensor([1])
>>> loss = model(**inputs, labels=labels).loss
>>> round(loss.item(), 2)
...
```

Example of multi-label classification:

```python
>>> import torch
>>> from transformers import AutoTokenizer, WavLMForSequenceClassification

>>> tokenizer = AutoTokenizer.from_pretrained("microsoft/wavlm-base")
>>> model = WavLMForSequenceClassification.from_pretrained("microsoft/wavlm-base", problem_type="multi_label_classification")

>>> inputs = tokenizer("Hello, my dog is cute", return_tensors="pt")

>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> predicted_class_ids = torch.arange(0, logits.shape[-1])[torch.sigmoid(logits).squeeze(dim=0) > 0.5]

>>> # To train a model on `num_labels` classes, you can pass `num_labels=num_labels` to `.from_pretrained(...)`
>>> num_labels = len(model.config.id2label)
>>> model = WavLMForSequenceClassification.from_pretrained(
...     "microsoft/wavlm-base", num_labels=num_labels, problem_type="multi_label_classification"
... )

>>> labels = torch.sum(
...     torch.nn.functional.one_hot(predicted_class_ids[None, :].clone(), num_classes=num_labels), dim=1
... ).to(torch.float)
>>> loss = model(**inputs, labels=labels).loss
```

## WavLMForAudioFrameClassification[[transformers.WavLMForAudioFrameClassification]]

#### transformers.WavLMForAudioFrameClassification[[transformers.WavLMForAudioFrameClassification]]

```python
transformers.WavLMForAudioFrameClassification(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/modeling_wavlm.py#L1352)

**Parameters:**

config ([WavLMForAudioFrameClassification](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForAudioFrameClassification)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

The Wavlm Model with a frame classification head on top for tasks like Speaker Diarization.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.WavLMForAudioFrameClassification.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, labels: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/modeling_wavlm.py#L1384)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See `WavLMProcessor.__call__` for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

**Returns:** [TokenClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or `tuple(torch.FloatTensor)`

A [TokenClassifierOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.TokenClassifierOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([WavLMConfig](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMConfig)) and inputs.

The [WavLMForAudioFrameClassification](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForAudioFrameClassification) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, sequence_length, config.num_labels)`) -- Classification scores (before SoftMax).
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings, if the model has an embedding layer, +
  one for the output of each layer) of shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the optional initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoFeatureExtractor, WavLMForAudioFrameClassification
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> feature_extractor = AutoFeatureExtractor.from_pretrained("microsoft/wavlm-base")
>>> model = WavLMForAudioFrameClassification.from_pretrained("microsoft/wavlm-base")

>>> # audio file is decoded on the fly
>>> inputs = feature_extractor(dataset[0]["audio"]["array"], return_tensors="pt", sampling_rate=sampling_rate)
>>> with torch.no_grad():
...     logits = model(**inputs).logits

>>> probabilities = torch.sigmoid(logits[0])
>>> # labels is a one-hot array of shape (num_frames, num_speakers)
>>> labels = (probabilities > 0.5).long()
>>> labels[0].tolist()
...
```

## WavLMForXVector[[transformers.WavLMForXVector]]

#### transformers.WavLMForXVector[[transformers.WavLMForXVector]]

```python
transformers.WavLMForXVector(config)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/modeling_wavlm.py#L1506)

**Parameters:**

config ([WavLMForXVector](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForXVector)) : Model configuration class with all the parameters of the model. Initializing with a config file does not load the weights associated with the model, only the configuration. Check out the [from_pretrained()](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) method to load the model weights.

WavLM Model with an XVector feature extraction head on top for tasks like Speaker Verification.

This model inherits from [PreTrainedModel](/docs/transformers/v5.15.1/en/main_classes/model#transformers.PreTrainedModel). Check the superclass documentation for the generic methods the
library implements for all its model (such as downloading or saving, resizing the input embeddings, pruning heads
etc.)

This model is also a PyTorch [torch.nn.Module](https://pytorch.org/docs/stable/nn.html#torch.nn.Module) subclass.
Use it as a regular PyTorch Module and refer to the PyTorch documentation for all matter related to general usage
and behavior.

#### forward[[transformers.WavLMForXVector.forward]]

```python
forward(input_values: typing.Optional[torch.Tensor], attention_mask: typing.Optional[torch.Tensor] = None, output_attentions: bool | None = None, output_hidden_states: bool | None = None, return_dict: bool | None = None, labels: typing.Optional[torch.Tensor] = None, **kwargs)
```

[Source](https://github.com/huggingface/transformers/blob/v5.15.1/src/transformers/models/wavlm/modeling_wavlm.py#L1556)

**Parameters:**

input_values (`torch.FloatTensor` of shape `(batch_size, sequence_length)`) : Float values of input raw speech waveform. Values can be obtained by loading a `.flac` or `.wav` audio file into an array of type `list[float]`, a `numpy.ndarray` or a `torch.Tensor`, *e.g.* via the torchcodec library (`pip install torchcodec`) or the soundfile library (`pip install soundfile`). To prepare the array into `input_values`, the [AutoProcessor](/docs/transformers/v5.15.1/en/model_doc/auto#transformers.AutoProcessor) should be used for padding and conversion into a tensor of type `torch.FloatTensor`. See `WavLMProcessor.__call__` for details.

attention_mask (`torch.Tensor` of shape `(batch_size, sequence_length)`, *optional*) : Mask to avoid performing attention on padding token indices. Mask values selected in `[0, 1]`:  - 1 for tokens that are **not masked**, - 0 for tokens that are **masked**.  [What are attention masks?](../glossary#attention-mask)

output_attentions (`bool`, *optional*) : Whether or not to return the attentions tensors of all attention layers. See `attentions` under returned tensors for more detail.

output_hidden_states (`bool`, *optional*) : Whether or not to return the hidden states of all layers. See `hidden_states` under returned tensors for more detail.

return_dict (`bool`, *optional*) : Whether or not to return a [ModelOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.

labels (`torch.LongTensor` of shape `(batch_size,)`, *optional*) : Labels for computing the sequence classification/regression loss. Indices should be in `[0, ..., config.num_labels - 1]`. If `config.num_labels == 1` a regression loss is computed (Mean-Square loss), If `config.num_labels > 1` a classification loss is computed (Cross-Entropy).

**Returns:** [XVectorOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.XVectorOutput) or `tuple(torch.FloatTensor)`

A [XVectorOutput](/docs/transformers/v5.15.1/en/main_classes/output#transformers.modeling_outputs.XVectorOutput) or a tuple of
`torch.FloatTensor` (if `return_dict=False` is passed or when `config.return_dict=False`) comprising various
elements depending on the configuration ([WavLMConfig](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMConfig)) and inputs.

The [WavLMForXVector](/docs/transformers/v5.15.1/en/model_doc/wavlm#transformers.WavLMForXVector) forward method, overrides the `__call__` special method.

Although the recipe for forward pass needs to be defined within this function, one should call the `Module`
instance afterwards instead of this since the former takes care of running the pre and post processing steps while
the latter silently ignores them.

- **loss** (`torch.FloatTensor` of shape `(1,)`, *optional*, returned when `labels` is provided) -- Classification loss.
- **logits** (`torch.FloatTensor` of shape `(batch_size, config.xvector_output_dim)`) -- Classification hidden states before AMSoftmax.
- **embeddings** (`torch.FloatTensor` of shape `(batch_size, config.xvector_output_dim)`) -- Utterance embeddings used for vector similarity-based retrieval.
- **hidden_states** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_hidden_states=True` is passed or when `config.output_hidden_states=True`) -- Tuple of `torch.FloatTensor` (one for the output of the embeddings + one for the output of each layer) of
  shape `(batch_size, sequence_length, hidden_size)`.

  Hidden-states of the model at the output of each layer plus the initial embedding outputs.
- **attentions** (`tuple(torch.FloatTensor)`, *optional*, returned when `output_attentions=True` is passed or when `config.output_attentions=True`) -- Tuple of `torch.FloatTensor` (one for each layer) of shape `(batch_size, num_heads, sequence_length,
  sequence_length)`.

  Attentions weights after the attention softmax, used to compute the weighted average in the self-attention
  heads.

Example:

```python
>>> from transformers import AutoFeatureExtractor, WavLMForXVector
>>> from datasets import load_dataset
>>> import torch

>>> dataset = load_dataset("hf-internal-testing/librispeech_asr_demo", "clean", split="validation")
>>> dataset = dataset.sort("id")
>>> sampling_rate = dataset.features["audio"].sampling_rate

>>> feature_extractor = AutoFeatureExtractor.from_pretrained("microsoft/wavlm-base")
>>> model = WavLMForXVector.from_pretrained("microsoft/wavlm-base")

>>> # audio file is decoded on the fly
>>> inputs = feature_extractor(
...     [d["array"] for d in dataset[:2]["audio"]], sampling_rate=sampling_rate, return_tensors="pt", padding=True
... )
>>> with torch.no_grad():
...     embeddings = model(**inputs).embeddings

>>> embeddings = torch.nn.functional.normalize(embeddings, dim=-1).cpu()

>>> # the resulting embeddings can be used for cosine similarity-based retrieval
>>> cosine_sim = torch.nn.CosineSimilarity(dim=-1)
>>> similarity = cosine_sim(embeddings[0], embeddings[1])
>>> threshold = 0.7  # the optimal threshold is dataset-dependent
>>> if similarity < threshold:
...     print("Speakers are not the same!")
>>> round(similarity.item(), 2)
...
```
